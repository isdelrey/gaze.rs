use avro_rs::{Schema};
use std::collections::HashMap;
use failure::Error;


pub struct Registry {
    schemas: HashMap<String, Schema>
}

impl Registry {
    pub fn add(&self, input: &str) -> Result<(), Error> {
        let schema: Schema = Schema::parse_str(input)?;

        Ok(())
    }
}