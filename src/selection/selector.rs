use super::filter::{Constraint, Filter};
use crate::client::Client;
use crate::codec::numbers::*;
use crate::router::Router;
use crate::selection::subscription::Subscription;
use async_trait::async_trait;
use avro_rs::Schema;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

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
pub type SubscriptionsByField = HashMap<usize, (FieldEqualityCheck, FieldNonEqualityChecks)>;
pub type SubscriptionsByType = HashMap<Vec<u8>, SubscriptionsByField>;
pub type Selector = SubscriptionsByType;

/* FOR EACH MESSAGE: */
/* Contains all the subscriptions interested in a message and how many of the constraints have been fulfilled: */
type ChecksPerMessage = HashMap<Vec<u8>, usize>;
type Subscriptions = HashMap<Vec<u8>, Arc<Subscription>>;

#[async_trait]
pub trait Selection {
    fn get_recipients(
        subscriptions: Subscriptions,
        message_type: &[u8],
        and_checks: ChecksPerMessage,
    ) -> Vec<Arc<Subscription>>;
    async fn relay(router: Arc<Router>, recipients: Vec<Arc<Subscription>>, message: &[u8]);
    async fn select(
        &self,
        router: Arc<Router>,
        message_type: &[u8],
        schema: &Schema,
        message: &[u8],
    );
}

#[async_trait]
impl Selection for Selector {
    fn get_recipients(
        subscriptions: Subscriptions,
        message_type: &[u8],
        checks: ChecksPerMessage,
    ) -> Vec<Arc<Subscription>> {
        let mut recipients: Vec<Arc<Subscription>> = Vec::new();

        for (subscription_id, checks_passed) in checks {
            let subscription = subscriptions
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

    async fn relay(router: Arc<Router>, recipients: Vec<Arc<Subscription>>, message: &[u8]) {
        for recipient in recipients {
            let mut writer = recipient.client.writer.lock().await;

            writer.write(message).await;
        }
    }

    async fn select(
        &self,
        router: Arc<Router>,
        message_type: &[u8],
        schema: &Schema,
        message: &[u8],
    ) {
        /* Check message type in selector for possible subscribers: */
        let subscriptions_by_field = match self.get(message_type) {
            Some(value) => value,
            None => return,
        };

        let mut checks = HashMap::new();
        let mut subscriptions: Subscriptions = HashMap::new();
        let mut position: usize = 0;
        let mut start: usize = 0;

        match schema {
            Schema::String | Schema::Bytes => {
                position = position + 1;
                let (size, bytes_read) =
                    message.read_varint_size().expect("Cannot read string size");
                start = start + bytes_read;

                let (equality_field_check, non_equality_field_checks) =
                    match subscriptions_by_field.get(&position) {
                        Some(value) => value,
                        None => return,
                    };

                let end = start + size;
                match equality_field_check.get(&message[start..end]) {
                    Some(subscription) => {
                        subscriptions.insert(subscription.id.to_vec(), subscription.clone());
                        let checks_counter = checks.entry(subscription.id.to_vec()).or_insert(0);
                        *checks_counter += 1;
                    }
                    None => {}
                };
            }
            _ => {}
        }

        /* Get recipients that fulfill all checks: */
        let recipients = Self::get_recipients(subscriptions, message_type, checks);

        /* Relay message: */
        Self::relay(router, recipients, message).await;
    }
}
