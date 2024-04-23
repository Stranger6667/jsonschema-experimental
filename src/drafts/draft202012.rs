use crate::vocabulary::KeywordValue;
use jsonlike::Json;

pub(crate) fn get_keyword<J: Json>(key: &str, value: &J) -> Option<KeywordValue<J>> {
    None
}
