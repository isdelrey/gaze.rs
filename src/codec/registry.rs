use avro_rs::Schema;
use std::collections::HashMap;

pub struct Registry {
    schemas: HashMap<Vec<u8>, (Schema, String)>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            schemas: HashMap::new(),
        }
    }
    pub fn get(&mut self, message_type: &[u8]) -> Option<&Schema> {
        match self.schemas.get(message_type) {
            Some((schema, _)) => Some(schema),
            None => None,
        }
    }
    pub fn add(&mut self, message_type: Vec<u8>, raw_schema: String) {
        let schema = Schema::parse_str(&raw_schema).expect("Cannot parse schema");

        self.schemas.insert(message_type, (schema, raw_schema));
    }
    pub fn get_raw(&self, message_type: Vec<u8>) -> Result<&str, ()> {
        let raw_schema: &str = match self.schemas.get(&message_type) {
            Some(value) => &value.1,
            None => return Err(()),
        };

        Ok(raw_schema)
    }
}
