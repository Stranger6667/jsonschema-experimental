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

#[derive(Debug)]
pub enum SchemaError {}
