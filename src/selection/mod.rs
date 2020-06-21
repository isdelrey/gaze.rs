use std::collections::HashMap;

pub type Subscriptions = HashMap<String, HashMap<usize, HashMap<Vec<u8>, Vec<Vec<u8>>>>>;

pub enum Constraint {
    Equal(Vec<u8>)
}

pub struct Selection {
    reading_from_storage: bool,
    constraints: Vec<Constraint>
}