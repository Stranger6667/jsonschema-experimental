use std::error::Error;

use jsonlike::Json;

use super::ValidationState;

pub struct OutputFormatState<'v, 'i, 's, J: Json> {
    state: &'s ValidationState<'v, 'i, J>,
}

impl<'v, 'i, 's, J: Json> OutputFormatState<'v, 'i, 's, J> {
    pub(crate) fn new(state: &'s ValidationState<'v, 'i, J>) -> OutputFormatState<'v, 'i, 's, J> {
        OutputFormatState { state }
    }

    pub fn verbose(&self) -> VerboseOutput {
        todo!()
    }
    pub fn with<F: OutputFormatter>(&self, formatter: F) -> F::Output {
        formatter.format::<J>(self)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VerboseOutput {}

pub trait OutputFormatter {
    type Error: Error;
    type Output;

    fn try_format<J: Json>(
        &self,
        state: &OutputFormatState<J>,
    ) -> Result<Self::Output, Self::Error>;

    fn format<J: Json>(&self, state: &OutputFormatState<J>) -> Self::Output {
        self.try_format::<J>(state).expect("Failed to format")
    }
}
