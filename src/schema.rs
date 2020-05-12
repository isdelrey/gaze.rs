use tokio::io::AsyncRead;
use avro_rs::{Schema};
use failure::Error;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Literal(usize),
    Record(HashMap<&'a str, Type<'a>>)
}



/* We want to give the following function a record encoded with a schema
and we want it to return a map with the position of the fields */
pub fn map_fields<'a>(schema: &Schema, record: &[u8], offset: usize) -> Result<Type<'a>, Error> {
    match *schema {
        Schema::Null => {
            Ok(Type::Literal(offset))
        },
        Schema::Boolean => {
            Ok(Type::Literal(offset))
        },
        Schema::Int => {
            Ok(Type::Literal(offset))
        },
        Schema::Long => {
            Ok(Type::Literal(offset))
        },
        Schema::Float => {
            Ok(Type::Literal(offset))
        },
        Schema::Double => {
            Ok(Type::Literal(offset))
        },
        Schema::Bytes => {
            Ok(Type::Literal(offset))
        },
        Schema::String => {
            Ok(Type::Literal(offset))
        },
        _ => Ok(Type::Literal(offset))
    }
}

#[cfg(test)]
mod tests {
    use avro_rs::{Schema};
    use crate::schema::*;

    #[test]
    fn test_map_fields_with_null() {
        let schema = Schema::parse_str(r#"
            {
                "type": "null",
                "name": "test"
            }
        "#).unwrap();
        let mut input: &[u8] = &[0x00];
        let map = map_fields(&schema, input, 0).unwrap();
        
        assert_eq!(map, Type::Literal(0))
    }

    #[test]
    fn test_map_fields_with_record() {
        let schema = Schema::parse_str(r#"
            {
                "type": "record",
                "name": "test",
                "fields": [
                    {"name": "a", "type": "long", "default": 42},
                    {"name": "b", "type": "string"}
                ]
            }
        "#).unwrap();
        let mut input: &[u8] = &[0x02, 0x08, 0x74, 0x65, 0x73, 0x74, 0x02, 0x00];
        let map = map_fields(&schema, input, 0).unwrap();
        
        assert_eq!(map, Type::Literal(0))
    }
}