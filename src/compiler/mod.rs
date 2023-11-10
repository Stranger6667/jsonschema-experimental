use crate::{Draft, JsonSchemaValidator, SchemaError};
use jsonlike::{Json, JsonObject};

pub fn compile<J: Json, D: Draft>(schema: &J) -> Result<JsonSchemaValidator, SchemaError> {
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
