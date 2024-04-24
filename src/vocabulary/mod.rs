use std::sync::Arc;

use jsonlike::Json;

use crate::{BoxedKeyword, BuildResult};

#[derive(Debug, Clone)]
pub(crate) enum KeywordValue<J: Json> {
    Type(Type),
    Custom(Arc<dyn Keyword<J>>),
}

#[derive(Debug, Clone)]
pub struct Type {}

pub trait Keyword<J: Json>: Send + Sync + core::fmt::Debug {
    fn is_valid(&self, instance: &J) -> bool;
}

mod sealed {
    pub trait Sealed<J> {}
}

pub trait KeywordFactory<'a, J: Json>: Send + Sync + sealed::Sealed<J> + 'a {
    fn init(&self, schema: &'a J) -> BuildResult<BoxedKeyword<J>>;
}

impl<'a, F, J: Json + 'a> sealed::Sealed<J> for F where
    F: Fn(&'a J) -> BuildResult<BoxedKeyword<J>> + Send + Sync + 'a
{
}

impl<'a, F, J: Json + 'a> KeywordFactory<'a, J> for F
where
    F: Fn(&'a J) -> BuildResult<BoxedKeyword<J>> + Send + Sync + 'a,
{
    fn init(&self, schema: &'a J) -> BuildResult<BoxedKeyword<J>> {
        self(schema)
    }
}
