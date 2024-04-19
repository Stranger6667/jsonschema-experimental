use super::Draft;
use crate::vocabulary::Keyword;

#[derive(Debug, Default)]
pub struct Draft07;

impl Draft for Draft07 {
    fn new_boxed() -> Box<dyn Draft>
    where
        Self: Sized,
    {
        Box::new(Draft07)
    }

    fn get_keyword(&self, key: &str) -> Option<Keyword> {
        None
    }
}
