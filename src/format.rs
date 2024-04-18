use crate::ValidationState;
use jsonlike::Json;

pub trait OutputFormatter {}

pub struct Flag;
pub struct Basic;
pub struct Detailed;
pub struct Verbose;

impl OutputFormatter for Flag {}

impl OutputFormatter for Basic {}

impl OutputFormatter for Detailed {}

impl OutputFormatter for Verbose {}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VerboseOutput {}

// #[cfg_attr(feature = "serde", derive(serde::Serialize))]
// struct Flag {
//     valid: bool
// }

pub struct OutputFormatterState<'s, 'v, 'i, F: OutputFormatter, J: Json> {
    state: &'s ValidationState<'v, 'i, J>,
    formatter: F,
}

#[cfg(feature = "serde")]
impl<'s, 'v, 'i, F: OutputFormatter, J: Json> serde::Serialize
    for OutputFormatterState<'s, 'v, 'i, F, J>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'s, 'v, 'i, F: OutputFormatter, J: Json> OutputFormatterState<'s, 'v, 'i, F, J> {
    pub(crate) fn new(
        state: &'s ValidationState<'v, 'i, J>,
        formatter: F,
    ) -> OutputFormatterState<'s, 'v, 'i, F, J> {
        OutputFormatterState { state, formatter }
    }
    pub fn iter_units(&self) -> OutputUnitIter<F> {
        OutputUnitIter {
            formatter: &self.formatter,
        }
    }
}

pub struct OutputUnitIter<'a, F: OutputFormatter> {
    formatter: &'a F,
}

impl<'a, F: OutputFormatter> Iterator for OutputUnitIter<'a, F> {
    type Item = OutputUnit;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct OutputUnit {}
