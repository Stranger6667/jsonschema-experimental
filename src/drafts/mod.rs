mod draft04;
mod draft07;
use crate::vocabulary::Keyword;
pub use draft04::Draft04;
pub use draft07::Draft07;
use jsonlike::Json;

pub const LATEST: Draft = Draft::Draft04;

pub enum Draft {
    Draft04,
    Draft07,
}

pub trait IntoDraft {
    fn get_draft() -> Draft;
}

impl Draft {
    pub(crate) fn get_keyword<J: Json>(&self, key: &str, value: &J) -> Option<Keyword> {
        match self {
            Draft::Draft04 => Draft04.get_keyword(key, value),
            Draft::Draft07 => Draft07.get_keyword(key, value),
        }
    }
}

pub(crate) fn from_url(url: &str) -> Option<Draft> {
    todo!()
}
