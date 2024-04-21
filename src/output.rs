use crate::Validator;
use jsonlike::Json;

pub trait OutputFormat {
    type Output;

    fn evaluate<J: Json>(&self, validator: &Validator, instance: &J) -> Self::Output;
}

pub struct Flag;
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FlagOutput {
    pub valid: bool,
}

impl OutputFormat for Flag {
    type Output = FlagOutput;

    fn evaluate<J: Json>(&self, validator: &Validator, instance: &J) -> Self::Output {
        FlagOutput {
            valid: validator.is_valid(instance),
        }
    }
}

pub struct Basic;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BasicOutput(OutputUnit);

impl OutputFormat for Basic {
    type Output = BasicOutput;

    fn evaluate<J: Json>(&self, validator: &Validator, instance: &J) -> Self::Output {
        todo!()
    }
}

pub struct Detailed;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DetailedOutput(OutputUnit);

impl OutputFormat for Detailed {
    type Output = DetailedOutput;

    fn evaluate<J: Json>(&self, validator: &Validator, instance: &J) -> Self::Output {
        todo!()
    }
}

pub struct Verbose;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VerboseOutput(OutputUnit);

impl OutputFormat for Verbose {
    type Output = VerboseOutput;

    fn evaluate<J: Json>(&self, validator: &Validator, instance: &J) -> Self::Output {
        todo!()
    }
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
