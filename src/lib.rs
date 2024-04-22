//!
//! ```rust
//! use jsonschema::{output, Draft};
//!
//! #[cfg(feature = "serde_json")]
//! async fn test() -> Result<(), jsonschema::Error> {
//!     let schema = serde_json::json!({"type": "integer"});
//!     let instance = serde_json::json!("a");
//!
//!     // One-off validation with a boolean result
//!     jsonschema::is_valid(&instance, &schema).await;
//!     jsonschema::blocking::is_valid(&instance, &schema);
//!     // One-off with the first error as `Result<(), jsonschema::Error>`
//!     jsonschema::validate(&instance, &schema).await?;
//!     jsonschema::blocking::validate(&instance, &schema)?;
//!     // One-off iteration over errors
//!     for error in jsonschema::iter_errors(&instance, &schema).await {
//!         println!("{}", error);
//!     }
//!     for error in jsonschema::blocking::iter_errors(&instance, &schema) {
//!         println!("{}", error);
//!     }
//!     // One-off collecting validation results into a struct conforming to the JSON Schema "Verbose" output format
//!     let verbose = jsonschema::evaluate(&instance, &schema, output::Verbose).await;
//!     let verbose = jsonschema::blocking::evaluate(&instance, &schema, output::Verbose);
//!     // Serialize validation output to JSON (requires the `serde` feature)
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&verbose).unwrap();
//!     }
//!
//!     // Async by default, autodetect draft based on the `$schema` property
//!     let validator = jsonschema::validator_for(&schema).await?;
//!     let validator = jsonschema::blocking::validator_for(&schema)?;
//!     // Specific draft
//!     let validator = jsonschema::Validator::options()
//!         .with_draft(Draft::Draft04)
//!         .build(&schema)
//!         .await?;
//!     let validator = jsonschema::blocking::Validator::options()
//!         .with_draft(Draft::Draft04)
//!         .build(&schema)?;
//!
//!     // Boolean result
//!     assert!(!validator.is_valid(&instance));
//!     // First error as `Result<(), jsonschema::Error>`
//!     assert!(validator.validate(&instance).is_err());
//!
//!     // Iterate over errors
//!     for error in validator.iter_errors(&instance) {
//!         println!("{}", error);
//!     }
//!
//!     // Collecting validation results into a struct conforming to the JSON Schema "Verbose" output format
//!     let verbose = validator.evaluate(&instance, output::Verbose);
//!     // Serialize validation output to JSON according to the verbose output format
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&verbose).unwrap();
//!     }
//!     Ok(())
//! }
//! ```
mod compiler;
mod drafts;
mod error;
mod graph;
pub mod output;
mod validation;
mod vocabulary;

pub use crate::{
    drafts::Draft,
    error::{Error, SchemaError, ValidationError},
    validation::{
        builder::{validator_for, ValidatorBuilder},
        evaluate, is_valid,
        iter::ValidationErrorIter,
        iter_errors, try_evaluate, try_is_valid, try_iter_errors, validate, Validator,
    },
};

pub mod blocking {
    pub use crate::validation::{
        blocking::{
            evaluate, is_valid, iter_errors, try_evaluate, try_is_valid, try_iter_errors, validate,
            Validator,
        },
        builder::blocking::{validator_for, ValidatorBuilder},
    };
}

#[cfg(all(test, feature = "serde_json"))]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_validator_for() {
        let schema = json!({"type": "integer"});
        let validator = crate::validator_for(&schema).await.expect("Invalid schema");
    }

    #[test]
    fn test_validator_for_blocking() {
        let schema = json!({"type": "integer"});
        let validator = crate::blocking::validator_for(&schema).expect("Invalid schema");
    }

    #[tokio::test]
    async fn test_options() {
        let schema = json!({"type": "integer"});
        let validator = crate::Validator::options()
            .build(&schema)
            .await
            .expect("Invalid schema");
    }

    #[test]
    fn test_options_blocking() {
        let schema = json!({"type": "integer"});
        let validator = crate::blocking::ValidatorBuilder::default()
            .build(&schema)
            .expect("Invalid schema");
    }
}
