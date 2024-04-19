use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Validation(ValidationError),
    Schema(SchemaError),
}

impl From<SchemaError> for Error {
    fn from(value: SchemaError) -> Self {
        Error::Schema(value)
    }
}

impl From<ValidationError> for Error {
    fn from(value: ValidationError) -> Self {
        Error::Validation(value)
    }
}

// TODO: Bound to instance? and maybe validator? Not clear if it is convenient for the end user,
// check existing library and see what is used there
#[derive(Debug)]
pub enum ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug)]
pub enum SchemaError {}

impl Display for SchemaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for SchemaError {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Validation(error) => Some(error),
            Error::Schema(error) => Some(error),
        }
    }
}
