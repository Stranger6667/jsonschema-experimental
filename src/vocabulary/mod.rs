use std::sync::Arc;

use jsonlike::Json;

#[derive(Debug, Clone)]
pub enum Keyword {
    Type(Type),
    Custom(Arc<dyn CustomKeyword>),
}

#[derive(Debug, Clone)]
pub struct Type {}

pub trait CustomKeyword: Send + Sync + core::fmt::Debug + 'static {
    fn is_valid<J: Json>(&self, instance: &J) -> bool
    where
        Self: Sized;
}

mod sealed {

    pub trait Sealed<J> {}
}

pub trait CustomKeywordConstructor<'a, J: Json>: Send + Sync + sealed::Sealed<J> + 'static {
    fn init(&self, schema: &'a J) -> Arc<dyn CustomKeyword>
    where
        Self: Sized;
}

impl<'a, F, J: Json + 'a> sealed::Sealed<J> for F where
    F: Fn(&'a J) -> Arc<dyn CustomKeyword> + Send + Sync + 'static
{
}

impl<'a, F, J: Json + 'a> CustomKeywordConstructor<'a, J> for F
where
    F: Fn(&'a J) -> Arc<dyn CustomKeyword> + Send + Sync + 'static,
{
    fn init(&self, schema: &'a J) -> Arc<dyn CustomKeyword>
    where
        Self: Sized,
    {
        self(schema)
    }
}
