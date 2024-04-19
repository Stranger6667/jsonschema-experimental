use crate::{JsonSchemaValidator, ValidationError};
use jsonlike::Json;
use std::borrow::Cow;

pub struct ValidationErrorIter<'validator, 'instance, J: Json> {
    validator: Cow<'validator, JsonSchemaValidator>,
    instance: &'instance J,
}

impl<'validator, 'instance, J: Json> ValidationErrorIter<'validator, 'instance, J> {
    pub(crate) fn new(
        validator: Cow<'validator, JsonSchemaValidator>,
        instance: &'instance J,
    ) -> ValidationErrorIter<'validator, 'instance, J> {
        ValidationErrorIter {
            validator,
            instance,
        }
    }
}

impl<'validator, 'instance, J: Json> Iterator for ValidationErrorIter<'validator, 'instance, J> {
    type Item = ValidationError;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
