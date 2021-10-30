use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Serialize, GraphQLObject, Debug)]
pub struct ValidationError {
    pub key: String,
    pub msg: String,
}

pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn valid(&self) -> bool {
        self.errors.len() == 0
    }

    pub fn add_error(&mut self, key: &str, msg: &str) {
        self.errors.push(ValidationError {
            key: String::from(key),
            msg: String::from(msg),
        })
    }
}
