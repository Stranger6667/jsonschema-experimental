use jsonlike::Json;

use crate::{BoxedFormat, BuildResult};

pub trait Format: Send + Sync + 'static {
    fn is_valid(&self, value: &str) -> bool;
}

mod sealed {
    pub trait Sealed<J> {}
}

pub trait FormatFactory<'a, J: Json>: Send + Sync + sealed::Sealed<J> + 'a {
    fn init(&self, schema: &'a J) -> BuildResult<BoxedFormat>;
}

impl<'a, F, J: Json + 'a> sealed::Sealed<J> for F where
    F: Fn(&'a J) -> BuildResult<BoxedFormat> + Send + Sync + 'a
{
}

impl<'a, F, J: Json + 'a> FormatFactory<'a, J> for F
where
    F: Fn(&'a J) -> BuildResult<BoxedFormat> + Send + Sync + 'a,
{
    fn init(&self, schema: &'a J) -> BuildResult<BoxedFormat> {
        self(schema)
    }
}
