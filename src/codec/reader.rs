// use tokio::io::{AsyncReadExt};
// use failure::Error;
// use std::collections::HashMap;

// #[derive(Debug, PartialEq)]
// pub enum Model<'a> {
//     Literal(Type),
//     Record(HashMap<&'a str, Type>)
// }

// pub enum Type {
//     Null,
//     Boolean,
//     Int,
//     Long,
//     Float,
//     Double,
//     Bytes,
//     String
// }


// /* We want to give the following function a record encoded with a schema
// and we want it to return a map with the position of the fields */
// pub fn read<'a, R: AsyncReadExt>(model: &Model, reader: R, offset: usize) -> Result<Type<'a>, Error> {
//     match *model {
//         Model::Literal(Type::Null) => {
//             Ok(Type::Literal(offset))
//         },
//         Model::Literal(Type::Boolean) => {
//             Ok(Type::Literal(offset))
//         },
//         Model::Literal(Type::Int) => {
//             Ok(Type::Literal(offset))
//         },
//         Model::Literal(Type::Long) => {
//             Ok(Type::Literal(offset))
//         },
//         Model::Literal(Type::Float) => {
//             Ok(Type::Literal(offset))
//         },
//         Model::Literal(Type::Double) => {
//             Ok(Type::Literal(offset))
//         },
//         Model::Literal(Type::Bytes) => {
//             Ok(Type::Literal(offset))
//         },
//         Model::Literal(Type::String) => {
//             Ok(Type::Literal(offset))
//         },
//         _ => Ok(Type::Literal(offset))
//     }
// }
/*
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
}*/