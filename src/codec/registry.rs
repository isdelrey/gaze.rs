use std::collections::HashMap;
use avro_rs::Schema;



pub struct Registry {
    models: HashMap<Vec<u8>, (Schema, String)>
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            models: HashMap::new()
        }
    }
    pub fn add(&mut self, message_type: Vec<u8>, raw_schema: String)  {
        let schema = Schema::parse_str(&raw_schema).expect("Cannot parse schema");

        self.models.insert(message_type, (schema, raw_schema));
    }
    pub fn get_raw(&self, message_type: Vec<u8>) -> Result<&str, ()> {
         let raw_schema: &str = match self.models.get(&message_type) {
             Some(value) => &value.1,
             None => {return Err(())}
         };

         Ok(raw_schema)
    }
}