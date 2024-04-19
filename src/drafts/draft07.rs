use jsonlike::Json;
use crate::drafts::{Draft, IntoDraft};
use crate::vocabulary::Keyword;

#[derive(Debug, Default)]
pub struct Draft07;


impl IntoDraft for Draft07 {
    fn get_draft() -> Draft {
        Draft::Draft07
    }
}

impl Draft07 {
    pub(crate) fn get_keyword<J: Json>(&self, key: &str, value: &J) -> Option<Keyword> {
        None
    }
}
