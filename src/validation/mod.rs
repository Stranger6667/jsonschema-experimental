use std::marker::PhantomData;

use jsonlike::Json;
mod iter;
pub use iter::ValidationErrorIter;
mod validator;
use crate::{
    compiler,
    drafts::Autodetect,
    error::{Error, SchemaError},
    format::OutputFormatter,
    Draft,
};
pub use validator::JsonSchemaValidator;

pub async fn is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, Error> {
    Ok(validator_for(schema).await?.is_valid(instance))
}

// TODO: Check the most popular order of arguments.
pub async fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
    validator_for(schema).await?.validate(instance)
}

pub async fn iter_errors<'schema, 'instance, J: Json>(
    schema: &'schema J,
    instance: &'instance J,
) -> Result<ValidationErrorIter<'static, 'instance, J>, Error> {
    let validator = validator_for(schema).await?;
    Ok(validator.iter_errors_once(instance))
}

pub async fn validator_for<J: Json>(schema: &J) -> Result<JsonSchemaValidator, Error> {
    todo!()
}

pub async fn collect_output<F: OutputFormatter, J: Json>(
    instance: &J,
    schema: &J,
    formatter: F,
) -> Result<F::Output, Error> {
    let validator = validator_for(schema).await?;
    formatter.format(&validator, instance)
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
    use crate::{
        compiler, draft04::Draft04, drafts::Autodetect, error::SchemaError,
        format::OutputFormatter, validation::ValidationErrorIter, Draft, Error,
        JsonSchemaValidator,
    };
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

    pub type Draft4Validator = Validator<Draft04>;

    pub fn is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, Error> {
        let validator = validator_for(schema)?;
        todo!()
    }

    pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
        let validator = validator_for(schema)?;
        todo!()
    }

    pub fn iter_errors<'instance, J: Json>(
        schema: &J,
        instance: &'instance J,
    ) -> Result<ValidationErrorIter<'static, 'instance, J>, Error> {
        let validator = validator_for(schema)?;
        Ok(validator.iter_errors_once(instance))
    }

    pub fn validator_for<J: Json>(schema: &J) -> Result<JsonSchemaValidator, Error> {
        todo!()
    }

    pub fn collect_output<F: OutputFormatter, J: Json>(
        instance: &J,
        schema: &J,
        formatter: F,
    ) -> Result<F::Output, Error> {
        let validator = validator_for(schema)?;
        formatter.format(&validator, instance)
    }
}
