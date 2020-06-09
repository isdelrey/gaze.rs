use std::collections::HashMap;

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
