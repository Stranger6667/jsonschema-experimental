use crate::{Error, JsonSchemaValidator};
use jsonlike::Json;

pub trait OutputFormat {
    type Output;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error>;
}

pub struct Flag;
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FlagOutput {
    pub valid: bool,
}

impl OutputFormat for Flag {
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

pub struct Basic;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BasicOutput {}

impl OutputFormat for Basic {
    type Output = BasicOutput;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error> {
        todo!()
    }
}

pub struct Detailed;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DetailedOutput {}

impl OutputFormat for Detailed {
    type Output = DetailedOutput;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error> {
        todo!()
    }
}

pub struct Verbose;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VerboseOutput {}

impl OutputFormat for Verbose {
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
