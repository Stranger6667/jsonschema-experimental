use std::collections::BTreeMap;

use crate::{maybe_owned::MaybeOwned, Validator};
use jpointer::JsonPointer;
use jsonlike::Json;

pub struct Output<'v, 'i, J: Json> {
    validator: MaybeOwned<'v, Validator<J>>,
    instance: &'i J,
}

impl<'v, 'i, J: Json> Output<'v, 'i, J> {
    pub(crate) fn new(
        validator: MaybeOwned<'v, Validator<J>>,
        instance: &'i J,
    ) -> Output<'v, 'i, J> {
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
    pub fn list(&self) -> List<J> {
        todo!()
    }
    pub fn hierarchical(&self) -> Hierarchical<J> {
        todo!()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Flag {
    pub valid: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct List<J: Json> {
    pub valid: bool,
    pub nested: Vec<OutputUnit<J>>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct OutputUnit<J: Json> {
    pub valid: bool,
    pub evaluation_path: JsonPointer,
    pub schema_location: String,
    pub instance_location: JsonPointer,
    pub nested: Option<Vec<OutputUnit<J>>>,
    pub annotations: Option<BTreeMap<String, J>>,
    pub dropped_annotations: Option<BTreeMap<String, J>>,
    pub errors: Option<BTreeMap<String, String>>,
}

pub type Hierarchical<J> = OutputUnit<J>;
