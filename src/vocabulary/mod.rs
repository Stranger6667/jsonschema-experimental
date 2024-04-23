use std::sync::Arc;

use jsonlike::Json;

#[derive(Debug, Clone)]
pub enum Keyword {
    Type(Type),
    Custom(Arc<dyn CustomKeyword>),
}

#[derive(Debug, Clone)]
pub struct Type {}

pub trait CustomKeyword: Send + Sync + core::fmt::Debug {
    fn is_valid<J: Json>(&self, instance: &J) -> bool
    where
        Self: Sized;
}

mod sealed {

    pub trait Sealed<J> {}
}

pub trait CustomKeywordFactory<'a, J: Json>: Send + Sync + sealed::Sealed<J> + 'a {
    fn init(&self, schema: &'a J) -> Box<dyn CustomKeyword>;
}

impl<'a, F, J: Json + 'a> sealed::Sealed<J> for F where
    F: Fn(&'a J) -> Box<dyn CustomKeyword> + Send + Sync + 'a
{
}

impl<'a, F, J: Json + 'a> CustomKeywordFactory<'a, J> for F
where
    F: Fn(&'a J) -> Box<dyn CustomKeyword> + Send + Sync + 'a,
{
    fn init(&self, schema: &'a J) -> Box<dyn CustomKeyword> {
        self(schema)
    }
}
