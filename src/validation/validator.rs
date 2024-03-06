use std::borrow::Cow;

use crate::validation::ValidationState;
use jsonlike::Json;

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
    pub fn validate<'v, 'i, J: Json>(&'v self, instance: &'i J) -> ValidationState<'v, 'i, J> {
        ValidationState::new(Cow::Borrowed(self), instance)
    }
    pub(crate) fn validate_once<'v, J: Json>(self, instance: &J) -> ValidationState<'v, '_, J> {
        ValidationState::new(Cow::Owned(self), instance)
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
