#[derive(Debug)]
pub enum JsonSchemaError {
    Validation(ValidationError),
    Schema(SchemaError),
}

impl From<SchemaError> for JsonSchemaError {
    fn from(value: SchemaError) -> Self {
        JsonSchemaError::Schema(value)
    }
}

impl From<ValidationError> for JsonSchemaError {
    fn from(value: ValidationError) -> Self {
        JsonSchemaError::Validation(value)
    }
}

#[derive(Debug)]
pub enum ValidationError {}

#[derive(Debug)]
pub enum SchemaError {}
