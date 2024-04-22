use std::{collections::HashMap, sync::Arc};

use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    output::Output,
    resolver::DefaultResolver,
    vocabulary::{CustomKeyword, CustomKeywordConstructor},
    BuildError, Format, ReferenceResolver, ValidationError, ValidationErrorIter, Validator,
};
use jsonlike::Json;

pub fn is_valid<J: Json>(schema: &J, instance: &J) -> bool {
    try_is_valid(schema, instance).expect("Invalid schema")
}

pub fn try_is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, BuildError> {
    Ok(validator_for(schema)?.is_valid(instance))
}

pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), ValidationError> {
    try_validate(schema, instance).expect("Invalid schema")
}

pub fn try_validate<J: Json>(
    schema: &J,
    instance: &J,
) -> Result<Result<(), ValidationError>, BuildError> {
    Ok(validator_for(schema)?.validate(instance))
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
) -> Result<ValidationErrorIter<'static, 'instance, J>, BuildError> {
    let validator = validator_for(schema)?;
    Ok(validator.iter_errors_once(instance))
}

pub fn evaluate<'i, J: Json>(instance: &'i J, schema: &J) -> Output<'static, 'i, J> {
    try_evaluate(instance, schema).expect("Invalid schema")
}

pub fn try_evaluate<'i, J: Json>(
    instance: &'i J,
    schema: &J,
) -> Result<Output<'static, 'i, J>, BuildError> {
    Ok(validator_for(schema)?.evaluate_once(instance))
}

pub fn validator_for<J: Json>(schema: &J) -> Result<Validator, BuildError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::default().draft(draft).build(schema)
}

pub struct ValidatorBuilder<'a, J: Json> {
    draft: Draft,
    resolver: Arc<dyn ReferenceResolver>,
    formats: HashMap<String, Arc<dyn Format>>,
    keyword: HashMap<String, Arc<dyn CustomKeywordConstructor<'a, J>>>,
}

impl<'a, J: Json> Default for ValidatorBuilder<'a, J> {
    fn default() -> Self {
        ValidatorBuilder {
            draft: Draft::latest(),
            resolver: Arc::new(DefaultResolver),
            formats: HashMap::default(),
            keyword: HashMap::default(),
        }
    }
}

impl<'a, J: Json> ValidatorBuilder<'a, J> {
    pub fn build(&self, schema: &J) -> Result<Validator, BuildError> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.draft)
    }
    pub fn draft(&mut self, draft: Draft) -> &mut ValidatorBuilder<'a, J> {
        self.draft = draft;
        self
    }
    pub fn resolver(&mut self, resolver: impl ReferenceResolver + 'static) -> &mut Self {
        self.resolver = Arc::new(resolver);
        self
    }
    pub fn format(&mut self, name: impl Into<String>, format: impl Format) -> &mut Self {
        self.formats.insert(name.into(), Arc::new(format));
        self
    }
    pub fn keyword(
        &mut self,
        name: impl Into<String>,
        function: impl CustomKeywordConstructor<'a, J>,
    ) -> &mut Self {
        self.keyword.insert(name.into(), Arc::new(function));
        self
    }
}
