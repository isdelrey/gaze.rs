use super::filter::{ChecksRequiredPerType, Filter, FilterBuilder};
use super::selector::{Selector, FIELD_SEPARATOR, FieldNonEqualityCheck, ConditionalInsertion};
use super::filter::Constraint;
use crate::client::Client;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Subscription {
    pub id: Vec<u8>,
    pub filter: Filter,
    pub checks_per_type: ChecksRequiredPerType,
    pub client: Arc<Client>
}

impl Subscription {
    pub fn new(
        id: Vec<u8>,
        client: Arc<Client>,
        filter: Filter
    ) -> Arc<Subscription> {
        let checks_per_type = filter.build_checks_per_type();

        let myself = Subscription {
            id,
            filter,
            checks_per_type,
            client
        };

        Arc::new(myself)
    }
    pub fn integrate(self: &Arc<Self>, selector: &mut Selector) {
        for (message_type, type_constraints) in self.filter.iter() {
            let subscriptions_by_field = selector.entry(message_type.clone()).or_insert(HashMap::new());
            for (field, field_constraint) in type_constraints {
                let field_with_leading_dot = [FIELD_SEPARATOR, &field].concat();
                let (equality_check, non_equality_checks) = subscriptions_by_field.entry(field_with_leading_dot).or_insert((HashMap::new(), Vec::new()));

                match field_constraint {
                        Constraint::Equal(value) => {
                            let field_equality_check = equality_check.entry(value.clone()).or_insert(Vec::new());
                            field_equality_check.push(self.clone());
                        },
                        Constraint::StartsWith(value) => {
                            let check = FieldNonEqualityCheck::StartsWith(value.clone());
                            non_equality_checks.insert_or_add_to_existing(check, self.clone());
                        },
                        Constraint::EndsWith(value) => {
                            let check = FieldNonEqualityCheck::EndsWith(value.clone());
                            non_equality_checks.insert_or_add_to_existing(check, self.clone());
                        },
                        Constraint::GreaterThan(value) => {
                            let check = FieldNonEqualityCheck::GreaterThan(value.clone());
                            non_equality_checks.insert_or_add_to_existing(check, self.clone());
                        },
                        Constraint::LowerThan(value) => {
                            let check = FieldNonEqualityCheck::LowerThan(value.clone());
                            non_equality_checks.insert_or_add_to_existing(check, self.clone());
                        },
                    }
            }
        }
    }
    pub fn disgregate(self: &Arc<Self>, selector: &mut Selector) {}
}
