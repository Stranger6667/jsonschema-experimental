use std::borrow::Cow;

use crate::{
    output::{OutputFormatter, OutputFormatterState},
    validation::{JsonSchemaValidator, ValidationErrorIter},
};
use jsonlike::Json;

pub struct ValidationState<'v, 'i, J> {
    validator: Cow<'v, JsonSchemaValidator>,
    instance: &'i J,
}

impl<'v, 'i, J: Json> ValidationState<'v, 'i, J> {
    pub(crate) fn new(
        validator: Cow<'v, JsonSchemaValidator>,
        instance: &'i J,
    ) -> ValidationState<'v, 'i, J> {
        ValidationState {
            validator,
            instance,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.validator.is_valid(self.instance)
    }
    pub fn iter_errors<'s>(&'s self) -> ValidationErrorIter<'v, 'i, 's, J> {
        ValidationErrorIter::new(self)
    }
    pub fn format_with<'s, F: OutputFormatter>(
        &'s self,
        formatter: F,
    ) -> OutputFormatterState<'s, 'v, 'i, F, J> {
        OutputFormatterState::new(self, formatter)
    }
}
