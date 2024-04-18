use std::borrow::Cow;

use crate::validation::{JsonSchemaValidator, ValidationErrorIter};
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
    pub fn iter_errors<'s>(&'s self) -> ValidationErrorIter<'v, 'i, 's, J> {
        ValidationErrorIter::new(self)
    }
}
