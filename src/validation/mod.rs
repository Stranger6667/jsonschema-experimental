use std::borrow::Cow;

use jsonlike::Json;
pub(crate) mod builder;
pub(crate) mod iter;
use crate::{graph, output::Output, vocabulary::Keyword, BuildError, ValidationError};
use builder::validator_for;
use iter::ValidationErrorIter;

pub async fn is_valid<J: Json>(schema: &J, instance: &J) -> bool {
    try_is_valid(schema, instance)
        .await
        .expect("Invalid schema")
}

pub async fn try_is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, BuildError> {
    Ok(validator_for(schema).await?.is_valid(instance))
}

pub async fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), ValidationError> {
    try_validate(schema, instance)
        .await
        .expect("Invalid schema")
}

pub async fn try_validate<J: Json>(
    schema: &J,
    instance: &J,
) -> Result<Result<(), ValidationError>, BuildError> {
    Ok(validator_for(schema).await?.validate(instance))
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
) -> Result<ValidationErrorIter<'static, 'instance, J>, BuildError> {
    let validator = validator_for(schema).await?;
    Ok(validator.iter_errors_once(instance))
}

pub async fn evaluate<'i, J: Json>(instance: &'i J, schema: &J) -> Output<'static, 'i, J> {
    try_evaluate(instance, schema)
        .await
        .expect("Invalid schema")
}

pub async fn try_evaluate<'i, J: Json>(
    instance: &'i J,
    schema: &J,
) -> Result<Output<'static, 'i, J>, BuildError> {
    Ok(validator_for(schema).await?.evaluate_once(instance))
}

#[derive(Debug, Clone)]
pub struct Validator {
    graph: graph::Graph<Keyword>,
}

impl Validator {
    pub(crate) fn new(graph: graph::Graph<Keyword>) -> Self {
        Self { graph }
    }

    pub fn is_valid<J: Json>(&self, instance: &J) -> bool {
        true
    }
    pub fn validate<J: Json>(&self, instance: &J) -> Result<(), ValidationError> {
        match self.iter_errors(instance).next() {
            None => Ok(()),
            Some(error) => Err(error),
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
    pub fn evaluate<'v, 'i, J: Json>(&'v self, instance: &'i J) -> Output<'v, 'i, J> {
        Output::new(Cow::Borrowed(self), instance)
    }
    pub(crate) fn evaluate_once<J: Json>(self, instance: &J) -> Output<'static, '_, J> {
        Output::new(Cow::Owned(self), instance)
    }
}

#[cfg(all(test, feature = "serde_json"))]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_validator_for() {
        let schema = json!({"type": "integer"});
        let validator = crate::validator_for(&schema).await.expect("Invalid schema");
    }

    #[tokio::test]
    async fn test_builder() {
        let schema = json!({"type": "integer"});
        let validator = crate::ValidatorBuilder::default()
            .build(&schema)
            .await
            .expect("Invalid schema");
    }
}
