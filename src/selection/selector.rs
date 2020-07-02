use crate::codec::numbers::*;
use crate::selection::subscription::Subscription;
use crate::protocol::command::Command;
use crate::protocol::writer::*;
use async_trait::async_trait;
use avro_rs::{schema, Schema};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

pub const ROOT_FIELD_PATH: &str = "";
pub const FIELD_SEPARATOR: &str = ".";

pub type SelectorSubscription = Arc<Subscription>;
pub type SelectorSubscriptions = Vec<SelectorSubscription>;

#[derive(std::cmp::PartialEq, std::fmt::Debug)]
pub enum FieldNonEqualityCheck {
    GreaterThan(Vec<u8>),
    LowerThan(Vec<u8>),
    StartsWith(Vec<u8>),
    EndsWith(Vec<u8>),
}

/* FOR ALL MESSAGES: */
pub type FieldNonEqualityChecks = Vec<(FieldNonEqualityCheck, SelectorSubscriptions)>;
pub type FieldEqualityCheck = HashMap<Vec<u8>, SelectorSubscriptions>;
pub type SubscriptionsByField = HashMap<String, (FieldEqualityCheck, FieldNonEqualityChecks)>;
pub type SubscriptionsByType = HashMap<Vec<u8>, SubscriptionsByField>;
pub type Selector = SubscriptionsByType;

/* FOR EACH MESSAGE: */
/* Contains all the subscriptions interested in a message and how many of the constraints have been fulfilled: */
type ChecksPerMessage = HashMap<Vec<u8>, usize>;
type Subscriptions = HashMap<Vec<u8>, Arc<Subscription>>;
type Checker = (ChecksPerMessage, Subscriptions);

pub trait CheckerOperations {
    fn pass(&mut self, subscriptions: SelectorSubscriptions);
}

impl CheckerOperations for Checker {
    fn pass(&mut self, subscriptions: SelectorSubscriptions) {
        let subscriptions_ids: Vec<Vec<u8>> = subscriptions.iter().map(|v| v.id.clone()).collect();
        //println!("Passed checks for {:?}", subscriptions_ids);
        for subscription in subscriptions {
            let checks_counter = self.0.entry(subscription.id.to_vec()).or_insert(0);
            self.1.insert(subscription.id.to_vec(), subscription);
            *checks_counter += 1;
        }
    }
}

pub trait ConditionalInsertion {
    fn insert_or_add_to_existing(&mut self, check: FieldNonEqualityCheck, subscription: SelectorSubscription);
}

impl ConditionalInsertion for FieldNonEqualityChecks {
     fn insert_or_add_to_existing(&mut self, check: FieldNonEqualityCheck, subscription: SelectorSubscription) {
        if let Some((_, ref mut checks)) = &mut self.iter_mut().find(|(n, _)| *n == check) {
            checks.push(subscription);
        }
        else {
            self.push((check, vec![subscription]));
        }
    }
}



#[async_trait]
pub trait Selection {
    fn get_recipients(checker: Checker, message_type: &[u8]) -> Vec<Arc<Subscription>>;
    async fn relay(recipients: Vec<Arc<Subscription>>, message_type: &[u8], message: &[u8]);
    fn select(
        &self,
        checker: &mut Checker,
        subscriptions_by_field: &SubscriptionsByField,
        schema: &Schema,
        field_name: &str,
        message: &[u8],
    ) -> usize;
    async fn distribute(
        &self,
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

    async fn relay(recipients: Vec<Arc<Subscription>>, message_type: &[u8], message: &[u8]) {
        for recipient in recipients {
            let mut writer = recipient.client.writer.lock().await;

            /* Write command: */
            writer.write_command(Command::SubscriptionMessage).await;

            /* Write message id: */
            let id = writer.write_id().await;
            //println!("Message id: {:?}", id);

            /* Write message type: */
            writer.write(&message_type).await.expect("Cannot write message type");
            //println!("Message type: {:?}", &message_type);

            /* Write subscription id: */
            writer.write(&recipient.id).await.expect("Cannot write subscription id");
            //println!("Subscriber id: {:?}", &recipient.id);

            /* Write message: */
            writer.write_size(message.len()).await;
            //println!("Message size: {}", message.len());
            
            /* Write message: */
            writer.write(message).await.expect("Cannot write message");
        }
    }

    async fn distribute(
        &self,
        message_type: &[u8],
        schema: &Schema,
        message: &[u8],
    ) {
        //println!("Distribution started");
        let mut checker = (HashMap::new(), HashMap::new());

        /* Check message type in selector for possible subscribers: */
        let subscriptions_by_field = match self.get(message_type) {
            Some(value) => value,
            None => {
                //println!("No subscriptions for message type {:?}, message types: {:?}", message_type, self.keys());
                return
            },
        };

        //println!("Subscriptions for message type {:?} found, message types: {:?}", message_type, self.keys());

        let start = std::time::Instant::now();
        /* Select: */
        self.select(
            &mut checker,
            subscriptions_by_field,
            schema,
            ROOT_FIELD_PATH,
            message,
        );
        
        //println!("Selection for {:?} performed in {}µs", message_type, start.elapsed().as_micros());

        /* Get recipients that fulfill all checks: */
        let recipients = Self::get_recipients(checker, message_type);

        for recipient in recipients.clone() {
            //println!("Recipient is {:?}", recipient.id);
        }

        /* Relay message: */
        Self::relay(recipients, &message_type, message).await;
    }

    fn select(
        &self,
        checker: &mut Checker,
        subscriptions_by_field: &SubscriptionsByField,
        schema: &Schema,
        field_name: &str,
        message: &[u8],
    ) -> usize {
        let mut start = 0;
        match schema {
            Schema::String | Schema::Bytes => {
                //println!("Field is {}", field_name);
                //println!("Message: {:?}", message);
                //println!("Selecting from field of type string or bytes");
                let (size, size_length) =
                    message.read_varint().expect("Cannot read string size");
                //println!("Start is {}, size {} and size size {}", start, size, size_length);
                start = start + size_length;

                let subscription_fields: Vec<String> = subscriptions_by_field.iter().map(|(k, _)| k.clone()).collect();
                //println!("Subscriptions by field {:?}", subscription_fields);
                
                let end = start + size;
                let (equality_field_check, non_equality_field_checks) =
                    match subscriptions_by_field.get(field_name) {
                        Some(value) => value,
                        None => {
                            //println!("No subscriptions for this field");
                            return end
                        },
                    };

                //println!("End is {}", end);
                let field = &message[start..end];


                //println!("Field value is {:?} -> {}", field, std::str::from_utf8(field.clone()).unwrap());

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

                end
            }
            Schema::Int => {
                //println!("Field is {}", field_name);
                //println!("Message: {:?}", message);
                //println!("Selecting from field of type int");

                let field_length = message
                    .get_varint_size()
                    .expect("Cannot read string size");
                //println!("Field length is {}", field_length);

                let end = start + field_length;
                let (equality_field_check, non_equality_field_checks) =
                    match subscriptions_by_field.get(field_name) {
                        Some(value) => value,
                        None => {
                            //println!("No subscriptions for this field");
                            return end
                        },
                    };

                let field = &message[start..end];
                //println!("Field value is {:?}", field);
                match equality_field_check.get(field) {
                    Some(subscription) => {
                        checker.pass(subscription.clone());
                    }
                    None => {}
                };

                for (non_equality_type, subscription) in non_equality_field_checks {
                    match non_equality_type {
                        FieldNonEqualityCheck::GreaterThan(value) => {
                            let value_is_negative = (value[0] & 0b0000_0001) != 0;
                            let field_is_negative = (field[0] & 0b0000_0001) != 0;

                            if (value_is_negative && field_is_negative) || (!value_is_negative && !field_is_negative) {
                                if value.len() > field.len() { continue }
                                if value.len() == field.len() { if value[value.len() - 1] > field[field.len() - 1] { continue } }
                                if value[value.len() - 1] == field[field.len() - 1] { continue }
                            }
                            else if field_is_negative { continue }

                            checker.pass(subscription.clone());
                        }
                        FieldNonEqualityCheck::LowerThan(value) => {
                            let value_is_negative = (value[0] & 0b0000_0001) != 0;
                            let field_is_negative = (field[0] & 0b0000_0001) != 0;

                            if (value_is_negative && field_is_negative) || (!value_is_negative && !field_is_negative) {
                                if value.len() < field.len() { continue }
                                if value.len() == field.len() { if value[value.len() - 1] < field[field.len() - 1] { continue } }
                                if value[value.len() - 1] == field[field.len() - 1] { continue }
                            }
                            else if value_is_negative { continue }

                            checker.pass(subscription.clone());
                        }
                        _ => {}
                    }
                }

                end
            }
            Schema::Record {
                ref fields,
                ref name,
                ..
            } => {
                let map_name = &name.name[..];
                //println!("Record: {}", map_name);

                let mut start = start;
                for schema::RecordField {
                    ref name,
                    ref schema,
                    ..
                } in fields
                {
                    let subfield_name = &name[..];
                    //println!("Record field: {}", subfield_name);
                    /* Select: */
                    start = self.select(
                        checker,
                        subscriptions_by_field,
                        schema,
                        &[field_name, FIELD_SEPARATOR, subfield_name].concat(),
                        &message[start..message.len()],
                    );
                }

                start
            }
            _ => {
                0
            }
        }
    }
}
