use std::{collections::HashMap, sync::Arc};

use jsonlike::Json;

use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    resolver::DefaultResolver,
    vocabulary::CustomKeywordFactory,
    BuildError, Format, ReferenceResolver, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator<J>, BuildError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::default().draft(draft).build(schema).await
}

pub struct ValidatorBuilder<'a, J: Json> {
    pub(crate) draft: Draft,
    pub(crate) resolver: Arc<dyn ReferenceResolver>,
    pub(crate) formats: HashMap<String, Arc<dyn Format>>,
    pub(crate) keywords: HashMap<String, Arc<dyn CustomKeywordFactory<'a, J>>>,
}

impl<'a, J: Json> Default for ValidatorBuilder<'a, J> {
    fn default() -> Self {
        ValidatorBuilder {
            draft: Draft::latest(),
            resolver: Arc::new(DefaultResolver),
            formats: HashMap::default(),
            keywords: HashMap::default(),
        }
    }
}

impl<'a, J: Json> ValidatorBuilder<'a, J> {
    pub async fn build(&self, schema: &J) -> Result<Validator<J>, BuildError> {
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
    pub fn keyword<F>(&mut self, name: impl Into<String>, factory: F) -> &mut Self
    where
        F: CustomKeywordFactory<'a, J>,
    {
        self.keywords.insert(name.into(), Arc::new(factory));
        self
    }
}
