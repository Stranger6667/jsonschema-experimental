use jsonlike::Json;
use crate::drafts::{Draft, IntoDraft};
use crate::vocabulary::Keyword;

#[derive(Debug, Default)]
pub struct Draft04;

impl IntoDraft for Draft04 {
    fn get_draft() -> Draft {
        Draft::Draft04
    }
}

impl  Draft04 {
    pub(crate) fn get_keyword<J: Json>(&self, key: &str, value: &J) -> Option<Keyword> {
        None
    }
}
