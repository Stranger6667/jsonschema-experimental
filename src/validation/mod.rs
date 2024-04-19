use std::marker::PhantomData;

use jsonlike::Json;
mod iter;
pub use iter::ValidationErrorIter;
mod validator;
use crate::{compiler, drafts, drafts::Draft, error::Error, output::OutputFormatter};
pub use validator::JsonSchemaValidator;

pub async fn is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, Error> {
    Ok(validator_for(schema).await?.is_valid(instance))
}

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
    let draft = drafts::from_url("TODO").unwrap_or_else(drafts::Latest::new_boxed);
    ValidatorBuilderOptions::new(draft).build(schema).await
}

pub async fn collect_output<F: OutputFormatter, J: Json>(
    instance: &J,
    schema: &J,
    formatter: F,
) -> Result<F::Output, Error> {
    let validator = validator_for(schema).await?;
    formatter.format(&validator, instance)
}

pub struct ValidatorBuilder<D: Draft> {
    _phantom: PhantomData<D>,
}

impl<D: Draft> ValidatorBuilder<D> {
    pub async fn from_schema<J: Json>(schema: &J) -> Result<JsonSchemaValidator, Error> {
        Self::options().build(schema).await
    }

    pub fn options() -> ValidatorBuilderOptions {
        ValidatorBuilderOptions::new(D::new_boxed())
    }
}

pub struct ValidatorBuilderOptions {
    draft: Box<dyn Draft>,
}

impl Default for ValidatorBuilderOptions {
    fn default() -> Self {
        Self::new(Box::<drafts::Latest>::default())
    }
}

impl ValidatorBuilderOptions {
    fn new(draft: Box<dyn Draft>) -> Self {
        Self { draft }
    }

    pub async fn build<J: Json>(self, schema: &J) -> Result<JsonSchemaValidator, Error> {
        // TODO: Resolve references
        compiler::compile::<J>(schema, self.draft)
    }
}

pub mod blocking {
    use crate::{
        compiler, drafts, drafts::Draft, output::OutputFormatter, validation::ValidationErrorIter,
        Error, JsonSchemaValidator,
    };
    use jsonlike::Json;
    use std::marker::PhantomData;

    pub struct ValidatorBuilder<D: Draft> {
        _phantom: PhantomData<D>,
    }

    impl<D: Draft> ValidatorBuilder<D> {
        pub fn from_schema<J: Json>(schema: &J) -> Result<JsonSchemaValidator, Error> {
            Self::options().build(schema)
        }

        pub fn options() -> ValidatorBuilderOptions {
            ValidatorBuilderOptions::new(D::new_boxed())
        }
    }

    pub struct ValidatorBuilderOptions {
        draft: Box<dyn Draft>,
    }

    impl Default for ValidatorBuilderOptions {
        fn default() -> Self {
            Self::new(Box::<drafts::Latest>::default())
        }
    }

    impl ValidatorBuilderOptions {
        pub(crate) fn new(draft: Box<dyn Draft>) -> Self {
            Self { draft }
        }

        pub fn build<J: Json>(self, schema: &J) -> Result<JsonSchemaValidator, Error> {
            // TODO: Resolve references
            compiler::compile::<J>(schema, self.draft)
        }
    }

    pub fn is_valid<J: Json>(schema: &J, instance: &J) -> Result<bool, Error> {
        Ok(validator_for(schema)?.is_valid(instance))
    }

    pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
        validator_for(schema)?.validate(instance)
    }

    pub fn iter_errors<'instance, J: Json>(
        schema: &J,
        instance: &'instance J,
    ) -> Result<ValidationErrorIter<'static, 'instance, J>, Error> {
        let validator = validator_for(schema)?;
        Ok(validator.iter_errors_once(instance))
    }

    pub fn validator_for<J: Json>(schema: &J) -> Result<JsonSchemaValidator, Error> {
        let draft = drafts::from_url("TODO").unwrap_or_else(drafts::Latest::new_boxed);
        ValidatorBuilderOptions::new(draft).build(schema)
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
