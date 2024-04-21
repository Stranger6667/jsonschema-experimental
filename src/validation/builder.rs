use jsonlike::Json;

use crate::{
    compiler,
    drafts::{self, Draft},
    SchemaError, Validator,
};

pub async fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
    let draft = drafts::from_url("TODO").unwrap_or(drafts::LATEST);
    ValidatorBuilderOptions::new(draft).build(schema).await
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
    use crate::{compiler, drafts, drafts::Draft, SchemaError, Validator};
    use jsonlike::Json;

    pub fn validator_for<J: Json>(schema: &J) -> Result<Validator, SchemaError> {
        let draft = drafts::from_url("TODO").unwrap_or(drafts::LATEST);
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
