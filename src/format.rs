use jsonlike::Json;

pub trait CustomFormat: Send + Sync + 'static {
    fn is_valid(&self, value: &str) -> bool;
}

mod sealed {
    pub trait Sealed<J> {}
}

pub trait CustomFormatFactory<'a, J: Json>: Send + Sync + sealed::Sealed<J> + 'a {
    fn init(&self, schema: &'a J) -> Box<dyn CustomFormat>;
}

impl<'a, F, J: Json + 'a> sealed::Sealed<J> for F where
    F: Fn(&'a J) -> Box<dyn CustomFormat> + Send + Sync + 'a
{
}

impl<'a, F, J: Json + 'a> CustomFormatFactory<'a, J> for F
where
    F: Fn(&'a J) -> Box<dyn CustomFormat> + Send + Sync + 'a,
{
    fn init(&self, schema: &'a J) -> Box<dyn CustomFormat> {
        self(schema)
    }
}
