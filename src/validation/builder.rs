use std::sync::Arc;

use jsonlike::Json;

use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    BuildError, ReferenceResolver, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator, BuildError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::default().draft(draft).build(schema).await
}

pub struct ValidatorBuilder {
    draft: Draft,
    resolver: Arc<dyn ReferenceResolver>,
}

impl Default for ValidatorBuilder {
    fn default() -> Self {
        ValidatorBuilder {
            draft: Draft::latest(),
            resolver: Arc::new(crate::resolver::DefaultResolver),
        }
    }
}

impl ValidatorBuilder {
    pub async fn build<J: Json>(&self, schema: &J) -> Result<Validator, BuildError> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.draft)
    }
    pub fn draft(&mut self, draft: Draft) -> &mut ValidatorBuilder {
        self.draft = draft;
        self
    }
    pub fn resolver(&mut self, resolver: impl ReferenceResolver + 'static) -> &mut Self {
        self.resolver = Arc::new(resolver);
        self
    }
}
