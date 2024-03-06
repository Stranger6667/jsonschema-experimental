//!  One-off validation:
//!
//! ```rust
//! use serde_json::json;
//!
//! async fn test() -> Result<(), jsonschema::Error> {
//!     let schema = json!({"type": "integer"});
//!     let instance = json!("a");
//!     jsonschema::is_valid(&schema, &instance).await?;
//!     Ok(())
//! }
//! ```
//!
//! ```rust
//! use serde_json::json;
//! use jsonlike::Json;
//! use serde::Deserialize;
//! use jsonschema::formats::{OutputFormatter, OutputFormatState};
//!
//! struct MyFormatter;
//!
//! #[derive(Debug)]
//! enum MyError {}
//!
//! impl std::fmt::Display for MyError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         Ok(())
//!     }
//! }
//!
//! impl std::error::Error for MyError {}
//!
//! impl OutputFormatter for MyFormatter {
//!     type Error = MyError;
//!
//!    fn try_format<'v, 'i, 's, 'f, 'de, J: Json, T: Deserialize<'de>>(
//!        &self,
//!        state: &'f OutputFormatState<'v, 'i, 's, J>,
//!    ) -> Result<T, Self::Error> {
//!        todo!("Not clear how to do this - should user create their own Deserializer??? or there
//!        should be some trait implemented?")
//!    }
//! }
//!
//! async fn test() -> Result<(), jsonschema::Error> {
//!     let schema = json!({"type": "integer"});
//!     let instance = json!("a");
//!     let validator = jsonschema::Validator::from_schema(&schema).await?;
//!     let x = validator.is_valid(&instance);
//!     let state = validator.validate(&instance);
//!     let y = state.is_valid();
//!     for error in state.errors() {
//!     }
//!     let verbose: serde_json::Value = state.format().verbose();
//!     for error in jsonschema::validate(&instance, &schema).await?.errors() {
//!
//!     }
//!     Ok(())
//! }
//! ```
mod compiler;
mod drafts;
mod error;
mod validation;

pub use crate::{
    drafts::{draft04, Draft},
    error::{Error, SchemaError, ValidationError},
    validation::{formats, is_valid, validate, JsonSchemaValidator},
};
use drafts::{draft04::Draft04, Autodetect};

pub type Draft4Validator = validation::Validator<Draft04>;
pub type Validator = validation::Validator<Autodetect>;

pub mod blocking {
    use jsonlike::Json;

    use crate::{drafts::Draft04, validation, Error};

    pub type Draft4Validator = validation::blocking::Validator<Draft04>;
    pub type Validator = validation::blocking::Validator;

    pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), Error> {
        let validator = Validator::from_schema(schema)?;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_from_schema() {
        let schema = json!({"type": "integer"});
        let validator = Draft4Validator::from_schema(&schema)
            .await
            .expect("Invalid schema");
    }

    #[test]
    fn test_from_schema_blocking() {
        let schema = json!({"type": "integer"});
        let validator = blocking::Draft4Validator::from_schema(&schema).expect("Invalid schema");
    }

    #[tokio::test]
    async fn test_options() {
        let schema = json!({"type": "integer"});
        let validator = Draft4Validator::options()
            .build(&schema)
            .await
            .expect("Invalid schema");
    }

    #[test]
    fn test_options_blocking() {
        let schema = json!({"type": "integer"});
        let validator = blocking::Draft4Validator::options()
            .build(&schema)
            .expect("Invalid schema");
    }
}
