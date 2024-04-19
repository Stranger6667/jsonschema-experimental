use super::Draft;
use crate::vocabulary::Keyword;

#[derive(Debug, Default)]
pub struct Draft04;

impl Draft for Draft04 {
    fn new_boxed() -> Box<dyn Draft>
    where
        Self: Sized,
    {
        Box::new(Draft04)
    }
    fn get_keyword(&self, key: &str) -> Option<Keyword> {
        None
    }
}
