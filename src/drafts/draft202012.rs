use crate::vocabulary::Keyword;
use jsonlike::Json;

pub(crate) fn get_keyword<J: Json>(key: &str, value: &J) -> Option<Keyword<J>> {
    None
}
