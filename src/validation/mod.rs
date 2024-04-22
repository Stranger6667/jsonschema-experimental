use std::borrow::Cow;

use jsonlike::Json;
pub(crate) mod builder;
pub(crate) mod iter;
use crate::{error::Error, graph, output::OutputFormat, vocabulary::Keyword, SchemaError};
use builder::validator_for;
use iter::ValidationErrorIter;

pub async fn is_valid<J: Json>(schema: &J, instance: &J) -> bool {
    try_is_valid(schema, instance)
        .await
        .expect("Invalid schema")
}

pub async fn try_is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, SchemaError> {
    Ok(validator_for(schema).await?.is_valid(instance))
}

pub async fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
    validator_for(schema).await?.validate(instance)
}

pub async fn iter_errors<'schema, 'instance, J: Json>(
    schema: &'schema J,
    instance: &'instance J,
) -> ValidationErrorIter<'static, 'instance, J> {
    try_iter_errors(schema, instance)
        .await
        .expect("Invalid schema")
}

pub async fn try_iter_errors<'schema, 'instance, J: Json>(
    schema: &'schema J,
    instance: &'instance J,
) -> Result<ValidationErrorIter<'static, 'instance, J>, SchemaError> {
    let validator = validator_for(schema).await?;
    Ok(validator.iter_errors_once(instance))
}

pub async fn evaluate<F: OutputFormat, J: Json>(instance: &J, schema: &J, format: F) -> F::Output {
    try_evaluate(instance, schema, format)
        .await
        .expect("Invalid schema")
}

pub async fn try_evaluate<F: OutputFormat, J: Json>(
    instance: &J,
    schema: &J,
    format: F,
) -> Result<F::Output, SchemaError> {
    let validator = validator_for(schema).await?;
    Ok(format.evaluate(&validator, instance))
}

#[derive(Debug, Clone)]
pub struct Validator {
    graph: graph::Graph<Keyword>,
}

impl Validator {
    pub(crate) fn new(graph: graph::Graph<Keyword>) -> Self {
        Self { graph }
    }

    pub fn options() -> builder::ValidatorBuilder {
        builder::ValidatorBuilder::default()
    }

    pub fn is_valid<J: Json>(&self, instance: &J) -> bool {
        true
    }
    pub fn validate<J: Json>(&self, instance: &J) -> Result<(), Error> {
        match self.iter_errors(instance).next() {
            None => Ok(()),
            Some(error) => Err(error.into()),
        }
    }
    pub fn iter_errors<'v, 'i, J: Json>(
        &'v self,
        instance: &'i J,
    ) -> ValidationErrorIter<'v, 'i, J> {
        ValidationErrorIter::new(Cow::Borrowed(self), instance)
    }
    pub(crate) fn iter_errors_once<J: Json>(
        self,
        instance: &J,
    ) -> ValidationErrorIter<'static, '_, J> {
        ValidationErrorIter::new(Cow::Owned(self), instance)
    }
    pub fn evaluate<F: OutputFormat, J: Json>(&self, instance: &J, format: F) -> F::Output {
        format.evaluate(self, instance)
    }
}

pub mod blocking {
    use crate::{
        output::OutputFormat,
        validation::{builder::blocking::validator_for, ValidationErrorIter},
        Error, SchemaError,
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
}
