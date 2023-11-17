use std::marker::PhantomData;

use jsonlike::Json;

use crate::{compiler, drafts::Autodetect, error::SchemaError, Draft, JsonSchemaValidator};

pub struct Validator<D: Draft = Autodetect> {
    _phantom: PhantomData<D>,
}

impl<D: Draft> Validator<D> {
    pub async fn from_schema<J: Json>(schema: &J) -> Result<JsonSchemaValidator, SchemaError> {
        ValidatorOptions::<D>::new().build(schema).await
    }

    pub fn options() -> ValidatorOptions<D> {
        ValidatorOptions::<D>::new()
    }
}

pub struct ValidatorOptions<D: Draft = Autodetect> {
    _phantom: PhantomData<D>,
}

impl<D: Draft> ValidatorOptions<D> {
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
            ValidatorOptions::<D>::new().build(schema)
        }

        pub fn options() -> ValidatorOptions<D> {
            ValidatorOptions::<D>::new()
        }
    }
    pub struct ValidatorOptions<D: Draft = Autodetect> {
        _phantom: PhantomData<D>,
    }

    impl<D: Draft> ValidatorOptions<D> {
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
