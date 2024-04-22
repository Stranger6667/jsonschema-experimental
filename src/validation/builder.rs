use std::{collections::HashMap, sync::Arc};

use jsonlike::Json;

use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    resolver::DefaultResolver,
    vocabulary::CustomKeywordFactory,
    BuildError, Format, ReferenceResolver, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator, BuildError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::default().draft(draft).build(schema).await
}

pub struct ValidatorBuilder<'a, J: Json> {
    pub(crate) draft: Draft,
    pub(crate) resolver: Arc<dyn ReferenceResolver>,
    pub(crate) formats: HashMap<String, Arc<dyn Format>>,
    pub(crate) keyword: HashMap<String, Arc<dyn CustomKeywordFactory<'a, J>>>,
}

impl<'a, J: Json> Default for ValidatorBuilder<'a, J> {
    fn default() -> Self {
        ValidatorBuilder {
            draft: Draft::latest(),
            resolver: Arc::new(DefaultResolver),
            formats: HashMap::default(),
            keyword: HashMap::default(),
        }
    }
}

impl<'a, J: Json> ValidatorBuilder<'a, J> {
    pub async fn build(&self, schema: &J) -> Result<Validator, BuildError> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.draft)
    }
    pub fn draft(&mut self, draft: Draft) -> &mut ValidatorBuilder<'a, J> {
        self.draft = draft;
        self
    }
    pub fn resolver(&mut self, resolver: impl ReferenceResolver + 'static) -> &mut Self {
        self.resolver = Arc::new(resolver);
        self
    }
    pub fn format(&mut self, name: impl Into<String>, format: impl Format) -> &mut Self {
        self.formats.insert(name.into(), Arc::new(format));
        self
    }
    pub fn keyword(
        &mut self,
        name: impl Into<String>,
        function: impl CustomKeywordFactory<'a, J>,
    ) -> &mut Self {
        self.keyword.insert(name.into(), Arc::new(function));
        self
    }
}
