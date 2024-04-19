use super::Draft;

#[derive(Debug, Default)]
pub struct Draft07;

impl Draft for Draft07 {
    fn new_boxed() -> Box<dyn Draft>
    where
        Self: Sized,
    {
        Box::new(Draft07)
    }
}
