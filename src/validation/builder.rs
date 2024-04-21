use std::marker::PhantomData;

use jsonlike::Json;

use crate::{
    compiler,
    drafts::{self, Draft, IntoDraft},
    SchemaError, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
    let draft = drafts::from_url("TODO").unwrap_or(drafts::LATEST);
    ValidatorBuilderOptions::new(draft).build(schema).await
}

pub struct ValidatorBuilder<D: IntoDraft> {
    _phantom: PhantomData<D>,
}

impl<D: IntoDraft> ValidatorBuilder<D> {
    pub async fn from_schema<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
        Self::options().build(schema).await
    }

    pub fn options() -> ValidatorBuilderOptions {
        ValidatorBuilderOptions::new(D::get_draft())
    }
}

pub struct ValidatorBuilderOptions {
    draft: Draft,
}

impl Default for ValidatorBuilderOptions {
    fn default() -> Self {
        Self::new(drafts::LATEST)
    }
}

impl ValidatorBuilderOptions {
    pub(crate) fn new(draft: Draft) -> Self {
        Self { draft }
    }

    pub async fn build<J: Json>(self, schema: &J) -> Result<Validator, SchemaError> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.draft)
    }
}

pub mod blocking {
    use crate::{
        compiler, drafts,
        drafts::{Draft, IntoDraft},
        SchemaError, Validator,
    };
    use jsonlike::Json;
    use std::marker::PhantomData;

    pub struct ValidatorBuilder<D: IntoDraft> {
        _phantom: PhantomData<D>,
    }

    impl<D: IntoDraft> ValidatorBuilder<D> {
        pub fn from_schema<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
            Self::options().build(schema)
        }

        pub fn options() -> ValidatorBuilderOptions {
            ValidatorBuilderOptions::new(D::get_draft())
        }
    }

    pub struct ValidatorBuilderOptions {
        draft: Draft,
    }

    impl Default for ValidatorBuilderOptions {
        fn default() -> Self {
            Self::new(drafts::LATEST)
        }
    }

    impl ValidatorBuilderOptions {
        pub(crate) fn new(draft: Draft) -> Self {
            Self { draft }
        }

        pub fn build<J: Json>(self, schema: &J) -> Result<Validator, SchemaError> {
            // TODO: Resolve references
            compiler::compile::<J>(schema, self.draft)
        }
    }

    pub fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
        let draft = drafts::from_url("TODO").unwrap_or(drafts::LATEST);
        ValidatorBuilderOptions::new(draft).build(schema)
    }
}
