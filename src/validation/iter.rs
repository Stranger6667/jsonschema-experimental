use jsonlike::Json;

use crate::ValidationError;

use super::ValidationState;

pub struct ValidationErrorIter<'v, 'i, 's, J: Json> {
    state: &'s ValidationState<'v, 'i, J>,
}

impl<'v, 'i, 's, J: Json> ValidationErrorIter<'v, 'i, 's, J> {
    pub(crate) fn new(state: &'s ValidationState<'v, 'i, J>) -> ValidationErrorIter<'v, 'i, 's, J> {
        ValidationErrorIter { state }
    }
}

impl<'v, 'i, 's, J: Json> Iterator for ValidationErrorIter<'v, 'i, 's, J> {
    type Item = ValidationError;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
