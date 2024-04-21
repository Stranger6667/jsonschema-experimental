use crate::{
    drafts::{Draft, IntoDraft},
    vocabulary::Keyword,
};
use jsonlike::Json;

#[derive(Debug, Default)]
pub struct Draft04;

impl IntoDraft for Draft04 {
    fn get_draft() -> Draft {
        Draft::Draft04
    }
}

impl Draft04 {
    pub(crate) fn get_keyword<J: Json>(&self, key: &str, value: &J) -> Option<Keyword> {
        None
    }
}
