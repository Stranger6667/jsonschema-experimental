use jsonlike::Json;

use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    SchemaError, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::new(draft).build(schema).await
}

pub struct ValidatorBuilder {
    draft: Draft,
}

impl Default for ValidatorBuilder {
    fn default() -> Self {
        Self::new(Draft::latest())
    }
}

impl ValidatorBuilder {
    pub(crate) fn new(draft: Draft) -> Self {
        Self { draft }
    }

    pub async fn build<J: Json>(self, schema: &J) -> Result<Validator, SchemaError> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.draft)
    }
    pub fn with_draft(mut self, draft: Draft) -> Self {
        self.draft = draft;
        self
    }
}
