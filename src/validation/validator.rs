use crate::{output::OutputFormat, validation::ValidationErrorIter, Error};
use jsonlike::Json;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct JsonSchemaValidator {
    nodes: Vec<JsonSchemaValidatorNode>,
}

impl JsonSchemaValidator {
    pub(crate) fn new(nodes: Vec<JsonSchemaValidatorNode>) -> Self {
        Self { nodes }
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
    pub fn validate_with_output_format<F: OutputFormat, J: Json>(
        &self,
        instance: &J,
        format: F,
    ) -> Result<F::Output, Error> {
        format.validate_with_output_format(self, instance)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JsonSchemaValidatorNode {
    keyword: Keyword,
}

#[derive(Debug, Clone)]
enum Keyword {
    Type(Type),
}

#[derive(Debug, Clone)]
struct Type {}
