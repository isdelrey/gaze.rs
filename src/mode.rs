use crate::errors::ParseError;

#[derive(PartialEq, Debug)]
pub enum Mode {
    Readable,
    Machine
}

impl Mode {
    pub fn parse(value: String) -> Result<Mode, ParseError> {
        match value.to_lowercase().as_str() {
            "b" => Ok(Mode::Machine),
            _ => Ok(Mode::Readable)
        }
    }
}