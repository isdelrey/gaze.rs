use super::filter::{Constraint, Filter};
use crate::client::Client;
use crate::codec::numbers::*;
use crate::router::Router;
use crate::selection::subscription::Subscription;
use async_trait::async_trait;
use avro_rs::{schema, Schema};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

const ROOT_FIELD_PATH: &str = "";
const FIELD_SEPARATOR: &str = ".";

pub type SelectorSubscription = Arc<Subscription>;

pub enum FieldNonEqualityCheck {
    GreaterThan(Vec<u8>),
    LowerThan(Vec<u8>),
    StartsWith(Vec<u8>),
    EndsWith(Vec<u8>),
}

/* FOR ALL MESSAGES: */
pub type FieldNonEqualityChecks = Vec<(FieldNonEqualityCheck, SelectorSubscription)>;
pub type FieldEqualityCheck = HashMap<Vec<u8>, SelectorSubscription>;
pub type SubscriptionsByField = HashMap<String, (FieldEqualityCheck, FieldNonEqualityChecks)>;
pub type SubscriptionsByType = HashMap<Vec<u8>, SubscriptionsByField>;
pub type Selector = SubscriptionsByType;

/* FOR EACH MESSAGE: */
/* Contains all the subscriptions interested in a message and how many of the constraints have been fulfilled: */
type ChecksPerMessage = HashMap<Vec<u8>, usize>;
type Subscriptions = HashMap<Vec<u8>, Arc<Subscription>>;
type Checker = (ChecksPerMessage, Subscriptions);

pub trait CheckerOperations {
    fn pass(&mut self, subscription: SelectorSubscription);
}

impl CheckerOperations for Checker {
    fn pass(&mut self, subscription: SelectorSubscription) {
        let checks_counter = self.0.entry(subscription.id.to_vec()).or_insert(0);
        self.1.insert(subscription.id.to_vec(), subscription);
        *checks_counter += 1;
    }
}

#[async_trait]
pub trait Selection {
    fn get_recipients(checker: Checker, message_type: &[u8]) -> Vec<Arc<Subscription>>;
    async fn relay(recipients: Vec<Arc<Subscription>>, message: &[u8]);
    fn select(
        &self,
        checker: &mut Checker,
        subscriptions_by_field: &SubscriptionsByField,
        schema: &Schema,
        field_name: &str,
        message: &[u8],
    );
    async fn distribute(
        &self,
        router: Arc<Router>,
        message_type: &[u8],
        schema: &Schema,
        message: &[u8],
    );
}

#[async_trait]
impl Selection for Selector {
    fn get_recipients(checker: Checker, message_type: &[u8]) -> Vec<Arc<Subscription>> {
        let mut recipients: Vec<Arc<Subscription>> = Vec::new();

        for (subscription_id, checks_passed) in checker.0 {
            let subscription = checker
                .1
                .get(&subscription_id)
                .expect("Cannot find subscription with subscription id");
            let expected_passed_checks = subscription
                .checks_per_type
                .get(message_type)
                .expect("Cannot get checks per type");
            if *expected_passed_checks == checks_passed {
                recipients.push(subscription.clone());
            }
        }

        recipients
    }

    async fn relay(recipients: Vec<Arc<Subscription>>, message: &[u8]) {
        for recipient in recipients {
            let mut writer = recipient.client.writer.lock().await;

            writer.write(message).await.expect("Cannot relay message");
        }
    }

    async fn distribute(
        &self,
        router: Arc<Router>,
        message_type: &[u8],
        schema: &Schema,
        message: &[u8],
    ) {
        let mut checker = (HashMap::new(), HashMap::new());

        /* Check message type in selector for possible subscribers: */
        let subscriptions_by_field = match self.get(message_type) {
            Some(value) => value,
            None => return,
        };

        /* Select: */
        self.select(
            &mut checker,
            subscriptions_by_field,
            schema,
            ROOT_FIELD_PATH,
            message,
        );

        /* Get recipients that fulfill all checks: */
        let recipients = Self::get_recipients(checker, message_type);

        /* Relay message: */
        Self::relay(recipients, message).await;
    }

    fn select(
        &self,
        checker: &mut Checker,
        subscriptions_by_field: &SubscriptionsByField,
        schema: &Schema,
        field_name: &str,
        message: &[u8],
    ) {
        let mut start = 0;
        match schema {
            Schema::String | Schema::Bytes => {
                let (size, field_length) =
                    message.read_varint_size().expect("Cannot read string size");
                start = start + field_length;

                let (equality_field_check, non_equality_field_checks) =
                    match subscriptions_by_field.get(field_name) {
                        Some(value) => value,
                        None => return,
                    };

                let end = start + size;
                let field = &message[start..end];
                match equality_field_check.get(field) {
                    Some(subscription) => {
                        checker.pass(subscription.clone());
                    }
                    None => {}
                };

                for (non_equality_type, subscription) in non_equality_field_checks {
                    match non_equality_type {
                        FieldNonEqualityCheck::StartsWith(value) => {
                            if &value[..] == &field[0..value.len()] {
                                checker.pass(subscription.clone());
                            }
                        }
                        FieldNonEqualityCheck::EndsWith(value) => {
                            if &value[..] == &field[field.len() - value.len()..field.len()] {
                                checker.pass(subscription.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
            Schema::Int => {
                let field_length = message
                    .read_varint_length()
                    .expect("Cannot read string size");

                let (equality_field_check, non_equality_field_checks) =
                    match subscriptions_by_field.get(field_name) {
                        Some(value) => value,
                        None => return,
                    };

                let end = start + field_length;
                let field = &message[start..end];
                match equality_field_check.get(field) {
                    Some(subscription) => {
                        checker.pass(subscription.clone());
                    }
                    None => {}
                };

                for (non_equality_type, subscription) in non_equality_field_checks {
                    match non_equality_type {
                        FieldNonEqualityCheck::GreaterThan(value) => {
                            if &value[..] == &field[0..value.len()] {
                                checker.pass(subscription.clone());
                            }
                        }
                        FieldNonEqualityCheck::LowerThan(value) => {
                            if &value[..] == &field[field.len() - value.len()..field.len()] {
                                checker.pass(subscription.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
            Schema::Record {
                ref fields,
                ref name,
                ..
            } => {
                let map_name = &name.name[..];

                for schema::RecordField {
                    ref name,
                    ref schema,
                    ..
                } in fields
                {
                    let subfield_name = &name[..];
                    /* Select: */
                    self.select(
                        checker,
                        subscriptions_by_field,
                        schema,
                        &[ROOT_FIELD_PATH, FIELD_SEPARATOR, map_name, subfield_name].concat(),
                        &message[start..message.len()],
                    );
                }
            }
            _ => {}
        }
    }
}
