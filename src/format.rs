use jsonlike::Json;

pub trait Format: Send + Sync + 'static {
    fn is_valid(&self, value: &str) -> bool;
}

mod sealed {
    pub trait Sealed<J> {}
}

pub trait FormatFactory<'a, J: Json>: Send + Sync + sealed::Sealed<J> + 'a {
    fn init(&self, schema: &'a J) -> Box<dyn Format>;
}

impl<'a, F, J: Json + 'a> sealed::Sealed<J> for F where
    F: Fn(&'a J) -> Box<dyn Format> + Send + Sync + 'a
{
}

impl<'a, F, J: Json + 'a> FormatFactory<'a, J> for F
where
    F: Fn(&'a J) -> Box<dyn Format> + Send + Sync + 'a,
{
    fn init(&self, schema: &'a J) -> Box<dyn Format> {
        self(schema)
    }
}
