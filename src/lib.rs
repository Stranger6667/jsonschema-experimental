//!
//! ```rust
//! use jsonschema::{Json, Draft, BuildResult, BoxedFormat, BoxedKeyword};
//!
//! #[cfg(feature = "serde_json")]
//! async fn test() -> Result<(), Box<dyn std::error::Error>> {
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
//!     let verbose = jsonschema::evaluate(&instance, &schema).await.verbose();
//!     let verbose = jsonschema::blocking::evaluate(&instance, &schema).verbose();
//!     // Serialize validation output to JSON (requires the `serde` feature)
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&verbose)?;
//!     }
//!
//!     // Async by default, autodetect draft based on the `$schema` property
//!     let validator = jsonschema::validator_for(&schema).await?;
//!     let validator = jsonschema::blocking::validator_for(&schema)?;
//!     // Specific draft
//!     let validator = jsonschema::ValidatorBuilder::default()
//!         .draft(Draft::Draft04)
//!         .build(&schema)
//!         .await?;
//!     let validator = jsonschema::blocking::ValidatorBuilder::default()
//!         .draft(Draft::Draft04)
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
//!     let verbose = validator.evaluate(&instance).verbose();
//!     // Serialize validation output to JSON according to the verbose output format
//!     #[cfg(feature = "serde")]
//!     {
//!         let serialized = serde_json::to_string(&verbose)?;
//!     }
//!
//!     struct Resolver;
//!
//!     impl jsonschema::ReferenceResolver for Resolver {};
//!
//!     struct FixedSize {
//!         size: usize,
//!     }
//!
//!     impl jsonschema::Format for FixedSize {
//!         fn is_valid(&self, value: &str) -> bool {
//!             value.len() == self.size
//!         }
//!     }
//!
//!     fn fixed_size_factory<J: Json>(schema: &J) -> BuildResult<BoxedFormat> {
//!         Ok(Box::new(FixedSize { size: 43 }))
//!     }
//!
//!     #[derive(Debug)]
//!     struct AsciiKeyword {
//!         size: usize
//!     }
//!
//!     impl<J: Json> jsonschema::Keyword<J> for AsciiKeyword {
//!         fn is_valid(&self, instance: &J) -> bool {
//!             if let Some(string) = instance.as_string().map(AsRef::as_ref) {
//!                 string.len() == self.size && string.chars().all(|c| c.is_ascii())
//!             } else {
//!                 true
//!             }
//!         }
//!     }
//!
//!     fn ascii_keyword_factory<J: Json>(schema: &J) -> BuildResult<BoxedKeyword<J>> {
//!         Ok(Box::new(AsciiKeyword { size: 42 }))
//!     }
//!
//!     let validator = jsonschema::ValidatorBuilder::default()
//!         .resolver(Resolver)
//!         .format(
//!             "fixed-size-1",
//!             |schema| -> BuildResult<BoxedFormat> {
//!                 Ok(Box::new(FixedSize { size: 5 }))
//!             }
//!         )
//!         .format("fixed-size-2", fixed_size_factory)
//!         .keyword(
//!             "ascii",
//!             |schema| -> BuildResult<BoxedKeyword<_>> {
//!                 Ok(Box::new(AsciiKeyword { size: 42 }))
//!             }
//!         )
//!         .keyword("also-ascii", ascii_keyword_factory)
//!         .build(&schema)
//!         .await?;
//!     let validator = jsonschema::blocking::ValidatorBuilder::default()
//!         .resolver(Resolver)
//!         .format("custom", fixed_size_factory)
//!         .keyword("ascii", ascii_keyword_factory)
//!         .build(&schema)?;
//!
//!     Ok(())
//! }
//! ```
pub mod blocking;
mod compiler;
mod drafts;
mod error;
mod format;
mod graph;
mod maybe_owned;
pub mod output;
mod resolver;
mod validation;
mod vocabulary;

pub use crate::{
    drafts::Draft,
    error::{BuildError, ValidationError},
    format::Format,
    output::Output,
    resolver::ReferenceResolver,
    validation::{
        builder::{validator_for, ValidatorBuilder},
        evaluate, is_valid,
        iter::ValidationErrorIter,
        iter_errors, try_evaluate, try_is_valid, try_iter_errors, validate, Validator,
    },
    vocabulary::Keyword,
};
pub use jsonlike::Json;

pub type BoxedFormat = Box<dyn Format>;
pub type BoxedKeyword<J> = Box<dyn Keyword<J>>;
pub type BuildResult<T> = Result<T, BuildError>;

#[cfg(test)]
mod tests {
    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_send_sync() {
        #[cfg(feature = "serde_json")]
        assert_send_sync::<crate::Validator<serde_json::Value>>();
        assert_send_sync::<crate::ValidationError>();
    }
}
