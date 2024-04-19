use crate::{Error, JsonSchemaValidator};
use jsonlike::Json;

pub trait OutputFormatter: Clone {
    type Output;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error>;
}

#[derive(Clone)]
pub struct Flag;
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FlagOutput {
    valid: bool,
}

impl OutputFormatter for Flag {
    type Output = FlagOutput;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error> {
        Ok(FlagOutput {
            valid: validator.is_valid(instance),
        })
    }
}

#[derive(Clone)]
pub struct Basic;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BasicOutput {}

impl OutputFormatter for Basic {
    type Output = BasicOutput;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct Detailed;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DetailedOutput {}

impl OutputFormatter for Detailed {
    type Output = DetailedOutput;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct Verbose;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VerboseOutput {}

impl OutputFormatter for Verbose {
    type Output = VerboseOutput;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error> {
        todo!()
    }
}

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
