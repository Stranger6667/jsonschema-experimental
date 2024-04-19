use crate::{drafts::Draft, Error, JsonSchemaValidator};
use jsonlike::{Json, JsonObject};

pub(crate) fn compile<J: Json>(
    schema: &J,
    draft: Box<dyn Draft>,
) -> Result<JsonSchemaValidator, Error> {
    let mut nodes = Vec::new();
    if let Some(obj) = schema.as_object() {
        for (key, value) in obj.iter() {
            //if let Some(k) = Draft04::make_keyword(key, value) {
            //    nodes.push(k);
            //}
        }
    } else {
        panic!()
    };
    Ok(JsonSchemaValidator::new(nodes))
}
