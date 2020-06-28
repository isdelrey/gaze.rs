use super::filter::{ChecksRequiredPerType, Filter, FilterBuilder};
use super::selector::Selector;
use crate::client::Client;
use std::collections::HashMap;
use std::sync::Arc;

pub type Subscriptions = HashMap<String, HashMap<usize, HashMap<Vec<u8>, Vec<Vec<u8>>>>>;

pub struct Subscription {
    pub id: Vec<u8>,
    pub reading_from_store: bool,
    pub filter: Filter,
    pub checks_per_type: ChecksRequiredPerType,
    pub client: Arc<Client>,
}

impl Subscription {
    pub fn new(
        id: Vec<u8>,
        client: Arc<Client>,
        reading_from_store: bool,
        filter: Filter,
    ) -> Subscription {
        let checks_per_type = filter.build_checks_per_type();
        Subscription {
            id,
            reading_from_store,
            filter,
            checks_per_type,
            client,
        }
    }
    pub fn integrate(&self, selector: &mut Selector) {}
    pub fn disgregate(&self, selector: &mut Selector) {}
}
