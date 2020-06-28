use std::collections::HashMap;
use fasthash::xx;

#[derive(std::fmt::Debug)]
pub enum Constraint {
    Equal(Vec<u8>),
    GreaterThan(Vec<u8>),
    LowerThan(Vec<u8>),
    StartsWith(Vec<u8>),
    EndsWith(Vec<u8>),
}

pub type FieldConstraint = (String, Constraint);
pub type TypeConstraints = (Vec<u8>, Vec<FieldConstraint>);
pub type Filter = Vec<TypeConstraints>;

pub type ChecksRequiredPerType = HashMap<Vec<u8>, usize>;

pub trait FilterBuilder {
    fn build_checks_per_type(&self) -> ChecksRequiredPerType;
    fn parse_constraint(input: serde_json::Value) -> Result<Constraint, &'static str>;
    fn hash_message_type(message_type: String) -> Vec<u8>;
    fn parse(input: serde_json::Value) -> Result<Filter, ()>;
}

impl FilterBuilder for Filter {
    fn build_checks_per_type(&self) -> ChecksRequiredPerType {
        let mut checks = ChecksRequiredPerType::new();
        for (message_type, constraints) in self {
            checks.insert(message_type.clone(), constraints.len());
        }

        checks
    }
    fn parse_constraint(input: serde_json::Value) -> Result<Constraint, &'static str> {
        match input {
            serde_json::Value::Object(object_values) => {
                if object_values.len() > 1 {
                    return Err("Filter contains more than 1 operator");
                }
                let (operator, value) = object_values
                    .iter()
                    .next()
                    .expect("No non-equality constraint found but one expected");

                let parsed_value = match Self::parse_constraint(value.clone()).unwrap() {
                    Constraint::Equal(parsed_value) => parsed_value,
                    _ => return Err("Filter contains an operator that cannot be parsed"),
                };
                match &operator[..] {
                    "$lt" => Ok(Constraint::LowerThan(parsed_value)),
                    "$gt" => Ok(Constraint::GreaterThan(parsed_value)),
                    _ => return Err("Filter operator not known"),
                }
            }
            serde_json::Value::Number(number) if number.is_u64() => {
                let value = number.as_u64().unwrap().to_le_bytes().to_vec();
                Ok(Constraint::Equal(value))
            }
            serde_json::Value::Number(number) if number.is_i64() => {
                let value = number.as_i64().unwrap().to_le_bytes().to_vec();
                Ok(Constraint::Equal(value))
            }
            serde_json::Value::Number(number) if number.is_f64() => {
                let value = number.as_f64().unwrap().to_le_bytes().to_vec();
                Ok(Constraint::Equal(value))
            }
            serde_json::Value::String(string) => {
                let value = string.as_bytes().to_vec();
                Ok(Constraint::Equal(value))
            }
            v => return Err("Filter contains field not allowed"),
        }
    }
    fn hash_message_type(message_type: String) -> Vec<u8> {
        Vec::from(&xx::hash32(message_type.as_bytes()).to_le_bytes()[..])
    }
    fn parse(input: serde_json::Value) -> Result<Filter, ()> {
        let mut filter = Vec::new();
        /* Check original filter: */
        let raw_contraints_per_type = match input {
            serde_json::Value::Array(values) => values,
            _ => return Err(()),
        };
        /* Build filter: */
        for raw_type_constraints in raw_contraints_per_type {
            let (message_type, fields) = match raw_type_constraints {
                serde_json::Value::Object(mut values) => {
                    let message_type = match values.remove("$type") {
                        Some(serde_json::Value::String(message_type)) => message_type,
                        _ => return Err(()),
                    };

                    (message_type, values)
                }
                _ => return Err(()),
            };

            /* Hash message type: */
            let message_type = Self::hash_message_type(message_type);

            let mut type_constraints = Vec::new();
            /* For each remaining field: */
            for (field_name, value) in fields.iter() {
                type_constraints.push((field_name.clone(), Self::parse_constraint(value.clone()).unwrap()));
            }

            let type_constraints = (
                message_type,
                type_constraints
            );

            filter.push(type_constraints);
        }

        println!("Filter build: {:?}", filter);

        Ok(filter)
    }
}
