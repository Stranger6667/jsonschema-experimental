use crate::{Error, JsonSchemaValidator};
use jsonlike::Json;
use std::borrow::Cow;

pub trait OutputFormatter: Clone {
    type Output;

    fn format<J: Json>(
        &self,
        validator: &JsonSchemaValidator,
        instance: &J,
    ) -> Result<Self::Output, Error>;

    fn iter<'v, 'i, J: Json>(
        self,
        validator: &'v JsonSchemaValidator,
        instance: &'i J,
    ) -> OutputUnitIter<'v, 'static, 'i, Self, J> {
        OutputUnitIter::new(Cow::Borrowed(validator), Cow::Owned(self), instance)
    }

    fn into_iter<J: Json>(
        self,
        validator: JsonSchemaValidator,
        instance: &J,
    ) -> OutputUnitIter<'static, 'static, '_, Self, J> {
        OutputUnitIter::new(Cow::Owned(validator), Cow::Owned(self), instance)
    }
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

pub struct OutputUnitIter<'v, 'f, 'i, F: OutputFormatter, J: Json> {
    validator: Cow<'v, JsonSchemaValidator>,
    formatter: Cow<'f, F>,
    instance: &'i J,
}

impl<'v, 'f, 'i, F: OutputFormatter, J: Json> OutputUnitIter<'v, 'f, 'i, F, J> {
    pub(crate) fn new(
        validator: Cow<'v, JsonSchemaValidator>,
        formatter: Cow<'f, F>,
        instance: &'i J,
    ) -> OutputUnitIter<'v, 'f, 'i, F, J> {
        OutputUnitIter {
            validator,
            formatter,
            instance,
        }
    }
}

impl<'v, 'f, 'i, F: OutputFormatter, J: Json> Iterator for OutputUnitIter<'v, 'f, 'i, F, J> {
    type Item = OutputUnit;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug)]
pub struct OutputUnit {}
