use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    output::OutputFormat,
    Error, SchemaError, ValidationErrorIter, Validator,
};
use jsonlike::Json;

pub fn is_valid<J: Json>(schema: &J, instance: &J) -> bool {
    try_is_valid(schema, instance).expect("Invalid schema")
}

pub fn try_is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, SchemaError> {
    Ok(validator_for(schema)?.is_valid(instance))
}

pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
    validator_for(schema)?.validate(instance)
}

pub fn iter_errors<'instance, J: Json>(
    schema: &J,
    instance: &'instance J,
) -> ValidationErrorIter<'static, 'instance, J> {
    try_iter_errors(schema, instance).expect("Invalid schema")
}

pub fn try_iter_errors<'instance, J: Json>(
    schema: &J,
    instance: &'instance J,
) -> Result<ValidationErrorIter<'static, 'instance, J>, SchemaError> {
    let validator = validator_for(schema)?;
    Ok(validator.iter_errors_once(instance))
}

pub fn evaluate<F: OutputFormat, J: Json>(instance: &J, schema: &J, format: F) -> F::Output {
    try_evaluate(instance, schema, format).expect("Invalid schema")
}

pub fn try_evaluate<F: OutputFormat, J: Json>(
    instance: &J,
    schema: &J,
    format: F,
) -> Result<F::Output, SchemaError> {
    let validator = validator_for(schema)?;
    Ok(format.evaluate(&validator, instance))
}

pub fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::new(draft).build(schema)
}

pub struct ValidatorBuilder {
    draft: Draft,
}

impl Default for ValidatorBuilder {
    fn default() -> Self {
        Self::new(Draft::latest())
    }
}

impl ValidatorBuilder {
    pub(crate) fn new(draft: Draft) -> Self {
        Self { draft }
    }

    pub fn build<J: Json>(self, schema: &J) -> Result<Validator, SchemaError> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.draft)
    }
    pub fn with_draft(mut self, draft: Draft) -> Self {
        self.draft = draft;
        self
    }
}
