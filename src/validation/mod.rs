use std::marker::PhantomData;

use jsonlike::Json;
pub mod formats;
mod iter;
mod state;
pub use formats::OutputFormatState;
pub use iter::ValidationErrorIter;
mod validator;
use crate::{
    compiler,
    drafts::Autodetect,
    error::{Error, SchemaError},
    Draft,
};
pub use state::ValidationState;
pub use validator::JsonSchemaValidator;

pub async fn is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, Error> {
    Ok(Validator::<Autodetect>::from_schema(schema)
        .await?
        .is_valid(instance))
}

// TODO: Check the most popular order of arguments.
pub async fn validate<'s, 'i, J: Json>(
    schema: &'s J,
    instance: &'i J,
) -> Result<ValidationState<'static, 'i, J>, Error> {
    Ok(Validator::<Autodetect>::from_schema(schema)
        .await?
        .validate_once(instance))
}

pub struct Validator<D: Draft = Autodetect> {
    _phantom: PhantomData<D>,
}

impl<D: Draft> Validator<D> {
    pub async fn from_schema<J: Json>(schema: &J) -> Result<JsonSchemaValidator, SchemaError> {
        Self::options().build(schema).await
    }

    pub fn options() -> ValidatorBuilder<D> {
        ValidatorBuilder::<D>::new()
    }
}

pub struct ValidatorBuilder<D: Draft = Autodetect> {
    _phantom: PhantomData<D>,
}

impl<D: Draft> Default for ValidatorBuilder<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Draft> ValidatorBuilder<D> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub async fn build<J: Json>(self, schema: &J) -> Result<JsonSchemaValidator, SchemaError> {
        // TODO: resolve
        compiler::compile::<J, D>(schema)
    }
}

pub mod blocking {
    use crate::{compiler, drafts::Autodetect, error::SchemaError, Draft, JsonSchemaValidator};
    use jsonlike::Json;
    use std::marker::PhantomData;

    pub struct Validator<D: Draft = Autodetect> {
        _phantom: PhantomData<D>,
    }

    impl<D: Draft> Validator<D> {
        pub fn from_schema<J: Json>(schema: &J) -> Result<JsonSchemaValidator, SchemaError> {
            ValidatorBuilder::<D>::new().build(schema)
        }

        pub fn options() -> ValidatorBuilder<D> {
            ValidatorBuilder::<D>::new()
        }
    }

    pub struct ValidatorBuilder<D: Draft = Autodetect> {
        _phantom: PhantomData<D>,
    }

    impl<D: Draft> Default for ValidatorBuilder<D> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<D: Draft> ValidatorBuilder<D> {
        pub fn new() -> Self {
            Self {
                _phantom: PhantomData,
            }
        }

        pub fn build<J: Json>(self, schema: &J) -> Result<JsonSchemaValidator, SchemaError> {
            // TODO: resolve
            compiler::compile::<J, D>(schema)
        }
    }
}
