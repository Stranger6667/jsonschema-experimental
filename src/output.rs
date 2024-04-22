use std::borrow::Cow;

use crate::Validator;
use jsonlike::Json;

pub struct Output<'v, 'i, J: Json> {
    validator: Cow<'v, Validator>,
    instance: &'i J,
}

impl<'v, 'i, J: Json> Output<'v, 'i, J> {
    pub(crate) fn new(validator: Cow<'v, Validator>, instance: &'i J) -> Output<'v, 'i, J> {
        Output {
            validator,
            instance,
        }
    }
    pub fn flag(&self) -> Flag {
        Flag {
            valid: self.validator.is_valid(self.instance),
        }
    }
    pub fn basic(&self) -> OutputUnit {
        todo!()
    }
    pub fn detailed(&self) -> OutputUnit {
        todo!()
    }
    pub fn verbose(&self) -> OutputUnit {
        todo!()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Flag {
    pub valid: bool,
}

// TODO: custom `Serialize` to match the spec
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub enum OutputUnit {
    Valid {
        keyword_location: String,
        absolute_keyword_location: Option<String>,
        instance_location: String,
        annotations: Vec<OutputUnit>,
    },
    SingleError {
        keyword_location: String,
        absolute_keyword_location: Option<String>,
        instance_location: String,
        error: String,
        annotations: Vec<OutputUnit>,
    },
    MultipleErrors {
        keyword_location: String,
        absolute_keyword_location: Option<String>,
        instance_location: String,
        errors: Vec<OutputUnit>,
        annotations: Vec<OutputUnit>,
    },
}
