use std::borrow::Borrow;

use jsonlike::prelude::*;

use crate::{
    compiler,
    drafts::{self, Draft},
    SchemaError, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
    let draft = draft_from_schema(schema);
    ValidatorBuilderOptions::new(draft).build(schema).await
}

fn draft_from_schema(schema: &impl Json) -> Draft {
    if let Some(object) = schema.as_object() {
        if let Some(url) = object.get("$schema").and_then(Json::as_string) {
            drafts::from_url(url.borrow()).unwrap_or(drafts::LATEST)
        } else {
            drafts::LATEST
        }
    } else {
        drafts::LATEST
    }
}

macro_rules! define_validator {
    ($name:ident, $draft:expr) => {
        pub struct $name;
        impl $name {
            pub async fn from_schema<J: jsonlike::Json>(
                schema: &J,
            ) -> Result<$crate::Validator, $crate::error::SchemaError> {
                Self::options().build(schema).await
            }

            pub fn options() -> $crate::validation::builder::ValidatorBuilderOptions {
                $crate::validation::builder::ValidatorBuilderOptions::new($draft)
            }
        }
    };
}

pub(crate) use define_validator;

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
    use super::draft_from_schema;
    use crate::{compiler, drafts, drafts::Draft, SchemaError, Validator};
    use jsonlike::Json;

    pub fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
        let draft = draft_from_schema(schema);
        ValidatorBuilderOptions::new(draft).build(schema)
    }

    macro_rules! define_validator {
        ($name:ident, $draft:expr) => {
            pub struct $name;
            impl $name {
                pub fn from_schema<J: jsonlike::Json>(
                    schema: &J,
                ) -> Result<$crate::Validator, $crate::error::SchemaError> {
                    Self::options().build(schema)
                }

                pub fn options() -> $crate::validation::builder::blocking::ValidatorBuilderOptions {
                    $crate::validation::builder::blocking::ValidatorBuilderOptions::new($draft)
                }
            }
        };
    }

    pub(crate) use define_validator;

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
}
