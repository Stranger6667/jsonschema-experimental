use std::borrow::Cow;

use jsonlike::Json;

use super::{JsonSchemaValidator, OutputFormatState, ValidationErrorIter};

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
    pub fn errors<'s>(&'s self) -> ValidationErrorIter<'v, 'i, 's, J> {
        ValidationErrorIter::new(self)
    }
    pub fn format<'s>(&'s self) -> OutputFormatState<'v, 'i, 's, J> {
        OutputFormatState::new(self)
    }
}
