use std::collections::HashMap;

pub type Subscriptions = HashMap<String, HashMap<usize, HashMap<Vec<u8>, Vec<Vec<u8>>>>>;

pub enum Selection {
    Equal(Vec<u8>)
}