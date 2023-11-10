pub struct JsonSchemaValidator {
    nodes: Vec<JsonSchemaValidatorNode>,
}

impl JsonSchemaValidator {
    pub(crate) fn new(nodes: Vec<JsonSchemaValidatorNode>) -> Self {
        Self { nodes }
    }
}

pub(crate) struct JsonSchemaValidatorNode {
    keyword: Keyword,
}

enum Keyword {
    Type(Type),
}

struct Type {}
