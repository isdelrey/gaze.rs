use tokio::io::{AsyncReadExt, AsyncWriteExt};
use failure::Error;
use std::collections::HashMap;
use crate::client::Client;

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Literal(usize),
    Record(HashMap<&'a str, Type<'a>>)
}

pub enum Model {
    Null,
    Boolean,
    Int,
    Long,
    Float,
    Double,
    Bytes,
    String,
    Record
}

pub struct Schema {
    type: Type
}

/* We want to give the following function a record encoded with a schema
and we want it to return a map with the position of the fields */
pub fn read<'a, R: AsyncReadExt>(model: &Model, reader: R, offset: usize) -> Result<Type<'a>, Error> {
    match *model {
        Model::Null => {
            Ok(Type::Literal(offset))
        },
        Model::Boolean => {
            Ok(Type::Literal(offset))
        },
        Model::Int => {
            Ok(Type::Literal(offset))
        },
        Model::Long => {
            Ok(Type::Literal(offset))
        },
        Model::Float => {
            Ok(Type::Literal(offset))
        },
        Model::Double => {
            Ok(Type::Literal(offset))
        },
        Model::Bytes => {
            Ok(Type::Literal(offset))
        },
        Model::String => {
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