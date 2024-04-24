use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    format::FormatFactory,
    output::Output,
    validation::builder::ValidatorBuilder as AsyncValidatorBuilder,
    vocabulary::KeywordFactory,
    BuildResult, ReferenceResolver, ValidationError, ValidationErrorIter, Validator,
};
use jsonlike::Json;

pub fn is_valid<J: Json>(schema: &J, instance: &J) -> bool {
    try_is_valid(schema, instance).expect("Invalid schema")
}

pub fn try_is_valid<J: Json>(schema: &J, instance: &J) -> BuildResult<bool> {
    Ok(validator_for(schema)?.is_valid(instance))
}

pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), ValidationError> {
    try_validate(schema, instance).expect("Invalid schema")
}

pub fn try_validate<J: Json>(schema: &J, instance: &J) -> BuildResult<Result<(), ValidationError>> {
    Ok(validator_for(schema)?.validate(instance))
}

pub fn iter_errors<'i, J: Json>(
    schema: &J,
    instance: &'i J,
) -> ValidationErrorIter<'static, 'i, J> {
    try_iter_errors(schema, instance).expect("Invalid schema")
}

pub fn try_iter_errors<'i, J: Json>(
    schema: &J,
    instance: &'i J,
) -> BuildResult<ValidationErrorIter<'static, 'i, J>> {
    let validator = validator_for(schema)?;
    Ok(validator.iter_errors_once(instance))
}

pub fn evaluate<'i, J: Json>(instance: &'i J, schema: &J) -> Output<'static, 'i, J> {
    try_evaluate(instance, schema).expect("Invalid schema")
}

pub fn try_evaluate<'i, J: Json>(
    instance: &'i J,
    schema: &J,
) -> BuildResult<Output<'static, 'i, J>> {
    Ok(validator_for(schema)?.evaluate_once(instance))
}

pub fn validator_for<J: Json>(schema: &J) -> BuildResult<Validator<J>> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::default().draft(draft).build(schema)
}

pub struct ValidatorBuilder<'a, J: Json> {
    inner: AsyncValidatorBuilder<'a, J>,
}

impl<'a, J: Json> Default for ValidatorBuilder<'a, J> {
    fn default() -> Self {
        ValidatorBuilder {
            inner: AsyncValidatorBuilder::default(),
        }
    }
}

impl<'a, J: Json> ValidatorBuilder<'a, J> {
    pub fn build(&self, schema: &J) -> BuildResult<Validator<J>> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.inner.draft)
    }
    pub fn draft(&mut self, draft: Draft) -> &mut Self {
        self.inner.draft(draft);
        self
    }
    pub fn resolver(&mut self, resolver: impl ReferenceResolver + 'static) -> &mut Self {
        self.inner.resolver(resolver);
        self
    }
    pub fn format<F>(&mut self, name: impl Into<String>, factory: F) -> &mut Self
    where
        F: FormatFactory<'a, J>,
    {
        self.inner.format(name, factory);
        self
    }
    pub fn keyword<F>(&mut self, name: impl Into<String>, factory: F) -> &mut Self
    where
        F: KeywordFactory<'a, J>,
    {
        self.inner.keyword(name, factory);
        self
    }
}

#[cfg(all(test, feature = "serde_json"))]
mod tests {
    use serde_json::json;

    #[test]
    fn test_validator_for_blocking() {
        let schema = json!({"type": "integer"});
        let validator = crate::blocking::validator_for(&schema).expect("Invalid schema");
    }

    #[test]
    fn test_options_blocking() {
        let schema = json!({"type": "integer"});
        let validator = crate::blocking::ValidatorBuilder::default()
            .build(&schema)
            .expect("Invalid schema");
    }
}
