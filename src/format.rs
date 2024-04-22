pub trait Format: Send + Sync + 'static {
    fn is_valid(&self, value: &str) -> bool;
}

impl<F> Format for F
where
    F: Fn(&str) -> bool + Send + Sync + 'static,
{
    fn is_valid(&self, value: &str) -> bool {
        self(value)
    }
}
