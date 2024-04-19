use crate::{drafts::Draft, graph, Error, Validator};
use jsonlike::{Json, JsonObject};

pub(crate) fn compile<J: Json>(schema: &J, draft: Box<dyn Draft>) -> Result<Validator, Error> {
    let mut graph = graph::Graph::new();
    if let Some(obj) = schema.as_object() {
        for (key, value) in obj.iter() {
            if let Some(keyword) = draft.get_keyword(key.unwrap().as_ref()) {
                graph.push_node(keyword);
            }
        }
    } else if let Some(b) = schema.as_boolean() {
    } else {
        return Err(Error::Schema(todo!()));
    };
    Ok(Validator::new(graph))
}
