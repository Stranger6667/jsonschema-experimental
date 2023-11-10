use std::marker::PhantomData;

use crate::{drafts::Autodetect, Draft};

pub struct JsonSchemaValidator<D: Draft = Autodetect> {
    nodes: Vec<JsonSchemaValidatorNode<D>>,
}

impl<D: Draft> JsonSchemaValidator<D> {
    pub(crate) fn new(nodes: Vec<JsonSchemaValidatorNode<D>>) -> Self {
        Self { nodes }
    }
}

pub(crate) struct JsonSchemaValidatorNode<D: Draft> {
    keyword: Keyword<D>,
}

enum Keyword<D: Draft> {
    Type(Type<D>),
}

struct Type<D: Draft> {
    _phantom: PhantomData<D>,
}
