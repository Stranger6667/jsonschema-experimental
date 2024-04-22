use std::borrow::Borrow;

use jsonlike::prelude::*;

use crate::{
    compiler,
    drafts::{self, Draft},
    SchemaError, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::new(draft).build(schema).await
}

fn draft_from_schema(schema: &impl Json) -> Draft {
    if let Some(object) = schema.as_object() {
        if let Some(url) = object.get("$schema").and_then(Json::as_string) {
            drafts::from_url(url.borrow()).unwrap_or_else(Draft::latest)
        } else {
            Draft::latest()
        }
    } else {
        Draft::latest()
    }
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

pub mod blocking {
    use super::draft_from_schema;
    use crate::{compiler, drafts::Draft, SchemaError, Validator};
    use jsonlike::Json;

    pub fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
        let draft = draft_from_schema(schema);
        ValidatorBuilder::new(draft).build(schema)
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

        pub fn build<J: Json>(self, schema: &J) -> Result<Validator, SchemaError> {
            // TODO: Resolve references
            compiler::compile::<J>(schema, self.draft)
        }
        pub fn with_draft(mut self, draft: Draft) -> Self {
            self.draft = draft;
            self
        }
    }
}
