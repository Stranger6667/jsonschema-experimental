use std::error::Error;

use jsonlike::Json;
use serde::Deserialize;

use super::ValidationState;

pub struct OutputFormatState<'v, 'i, 's, J: Json> {
    state: &'s ValidationState<'v, 'i, J>,
}

impl<'v, 'i, 's, J: Json> OutputFormatState<'v, 'i, 's, J> {
    pub(crate) fn new(state: &'s ValidationState<'v, 'i, J>) -> OutputFormatState<'v, 'i, 's, J> {
        OutputFormatState { state }
    }

    pub fn verbose<'de, D: Deserialize<'de>>(&self) -> D {
        todo!()
    }
    pub fn with<'de, D: Deserialize<'de>, F: OutputFormatter>(&self, formatter: F) -> D {
        formatter.format::<J, D>(self)
    }
}

pub trait OutputFormatter {
    type Error: Error;

    fn try_format<'v, 'i, 's, 'f, 'de, J: Json, D: Deserialize<'de>>(
        &self,
        state: &'f OutputFormatState<'v, 'i, 's, J>,
    ) -> Result<D, Self::Error>;

    fn format<'v, 'i, 's, 'f, 'de, J: Json, D: Deserialize<'de>>(
        &self,
        state: &'f OutputFormatState<'v, 'i, 's, J>,
    ) -> D {
        self.try_format::<J, D>(state).expect("Failed to format")
    }
}
