use std::sync::Arc;

use jsonlike::Json;

#[derive(Debug, Clone)]
pub enum Keyword<J: Json> {
    Type(Type),
    Custom(Arc<dyn CustomKeyword<J>>),
}

#[derive(Debug, Clone)]
pub struct Type {}

pub trait CustomKeyword<J: Json>: Send + Sync + core::fmt::Debug {
    fn is_valid(&self, instance: &J) -> bool;
}

mod sealed {
    pub trait Sealed<J> {}
}

pub trait CustomKeywordFactory<'a, J: Json>: Send + Sync + sealed::Sealed<J> + 'a {
    fn init(&self, schema: &'a J) -> Box<dyn CustomKeyword<J>>;
}

impl<'a, F, J: Json + 'a> sealed::Sealed<J> for F where
    F: Fn(&'a J) -> Box<dyn CustomKeyword<J>> + Send + Sync + 'a
{
}

impl<'a, F, J: Json + 'a> CustomKeywordFactory<'a, J> for F
where
    F: Fn(&'a J) -> Box<dyn CustomKeyword<J>> + Send + Sync + 'a,
{
    fn init(&self, schema: &'a J) -> Box<dyn CustomKeyword<J>> {
        self(schema)
    }
}
