use crate::{
    graph, output::OutputFormat, validation::ValidationErrorIter, vocabulary::Keyword, Error,
};
use jsonlike::Json;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Validator {
    graph: graph::Graph<Keyword>,
}

impl Validator {
    pub(crate) fn new(graph: graph::Graph<Keyword>) -> Self {
        Self { graph }
    }

    pub fn is_valid<J: Json>(&self, instance: &J) -> bool {
        true
    }
    pub fn validate<J: Json>(&self, instance: &J) -> Result<(), Error> {
        match self.iter_errors(instance).next() {
            None => Ok(()),
            Some(error) => Err(error.into()),
        }
    }
    pub fn iter_errors<'v, 'i, J: Json>(
        &'v self,
        instance: &'i J,
    ) -> ValidationErrorIter<'v, 'i, J> {
        ValidationErrorIter::new(Cow::Borrowed(self), instance)
    }
    pub(crate) fn iter_errors_once<J: Json>(
        self,
        instance: &J,
    ) -> ValidationErrorIter<'static, '_, J> {
        ValidationErrorIter::new(Cow::Owned(self), instance)
    }
    pub fn evaluate<F: OutputFormat, J: Json>(&self, instance: &J, format: F) -> F::Output {
        format.evaluate(self, instance)
    }
}
