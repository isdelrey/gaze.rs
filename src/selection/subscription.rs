use super::filter::Filter;
use std::collections::HashMap;

pub type Subscriptions = HashMap<String, HashMap<usize, HashMap<Vec<u8>, Vec<Vec<u8>>>>>;

pub struct Subscription {
    reading_from_store: bool,
    pub filter: Filter,
}

impl Subscription {
    pub fn new(reading_from_store: bool, filter: Filter) -> Subscription {
        Subscription {
            reading_from_store,
            filter,
        }
    }
}
