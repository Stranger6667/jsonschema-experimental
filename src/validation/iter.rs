use crate::{ValidationError, Validator};
use jsonlike::Json;
use std::borrow::Cow;

pub struct ValidationErrorIter<'v, 'i, J: Json> {
    validator: Cow<'v, Validator>,
    instance: &'i J,
}

impl<'v, 'i, J: Json> ValidationErrorIter<'v, 'i, J> {
    pub(crate) fn new(
        validator: Cow<'v, Validator>,
        instance: &'i J,
    ) -> ValidationErrorIter<'v, 'i, J> {
        ValidationErrorIter {
            validator,
            instance,
        }
    }
}

impl<'v, 'i, J: Json> Iterator for ValidationErrorIter<'v, 'i, J> {
    type Item = ValidationError;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
