use std::{collections::HashMap, sync::Arc};

use jsonlike::Json;

use crate::{
    compiler,
    drafts::{draft_from_schema, Draft},
    format::FormatFactory,
    resolver::DefaultResolver,
    vocabulary::KeywordFactory,
    BuildResult, ReferenceResolver, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> BuildResult<Validator<J>> {
    let draft = draft_from_schema(schema);
    ValidatorBuilder::default().draft(draft).build(schema).await
}

pub struct ValidatorBuilder<'a, J: Json> {
    pub(crate) draft: Draft,
    pub(crate) resolver: Arc<dyn ReferenceResolver>,
    pub(crate) formats: HashMap<String, Arc<dyn FormatFactory<'a, J>>>,
    pub(crate) keywords: HashMap<String, Arc<dyn KeywordFactory<'a, J>>>,
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
    pub async fn build(&self, schema: &J) -> BuildResult<Validator<J>> {
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
    pub fn format<F>(&mut self, name: impl Into<String>, factory: F) -> &mut Self
    where
        F: FormatFactory<'a, J>,
    {
        self.formats.insert(name.into(), Arc::new(factory));
        self
    }
    pub fn keyword<F>(&mut self, name: impl Into<String>, factory: F) -> &mut Self
    where
        F: KeywordFactory<'a, J>,
    {
        self.keywords.insert(name.into(), Arc::new(factory));
        self
    }
}
