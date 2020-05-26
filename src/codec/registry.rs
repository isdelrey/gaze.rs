use std::collections::HashMap;
use failure::Error;



pub struct Registry {
    models: HashMap<String, Model>
}

impl Registry {
    pub fn add(&self, input: &str) -> Result<(), Error> {
        let model: Model = ();

        Ok(())
    }
}