use crate::{maybe_owned::MaybeOwned, ValidationError, Validator};
use jsonlike::Json;

pub struct ValidationErrorIter<'v, 'i, J: Json> {
    validator: MaybeOwned<'v, Validator<J>>,
    instance: &'i J,
}

impl<'v, 'i, J: Json> ValidationErrorIter<'v, 'i, J> {
    pub(crate) fn new(
        validator: MaybeOwned<'v, Validator<J>>,
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
