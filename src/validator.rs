use std::marker::PhantomData;

use jsonlike::Json;

use crate::{compiler, drafts::Autodetect, error::SchemaError, Draft, JsonSchemaValidator};

pub struct Validator<D: Draft = Autodetect> {
    _phantom: PhantomData<D>,
}

impl<D: Draft> Validator<D> {
    pub async fn from_schema<'a, 'b: 'a>(
        schema: &'b impl Json<'a>,
    ) -> Result<JsonSchemaValidator, SchemaError> {
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

    pub async fn build<'a, 'b: 'a>(
        self,
        schema: &'b impl Json<'a>,
    ) -> Result<JsonSchemaValidator, SchemaError> {
        // TODO: resolve
        compiler::compile::<D>(schema)
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
        pub fn from_schema<'a, 'b: 'a>(
            schema: &'b impl Json<'a>,
        ) -> Result<JsonSchemaValidator, SchemaError> {
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

        pub fn build<'a, 'b: 'a>(
            self,
            schema: &'b impl Json<'a>,
        ) -> Result<JsonSchemaValidator, SchemaError> {
            // TODO: resolve
            compiler::compile::<D>(schema)
        }
    }
}
