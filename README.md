# jsonschema

Experimental JSON Schema validator for Rust.

> **⚠️ THIS LIBRARY IS WORK-IN-PROGRESS ⚠️**

This crate is a from-scratch rewrite of [jsonschema-rs](https://github.com/Stranger6667/jsonschema-rs) that aims to address some of the design flaws.
It started as a separate private repo, but I plan to move the development back to that one.
For an in-depth roadmap, please take a look [here](https://github.com/Stranger6667/jsonschema/issues/1)

This is how library API may look like:

```rust
use jsonschema::format;

async fn test() -> Result<(), jsonschema::Error> {
    let schema = serde_json::json!({"type": "integer"});
    let instance = serde_json::json!("a");

    // One-off validation with a boolean result
    jsonschema::is_valid(&instance, &schema).await?;
    jsonschema::blocking::is_valid(&instance, &schema)?;
    // One-off with the first error as `Result<(), jsonschema::Error>`
    jsonschema::validate(&instance, &schema).await?;
    jsonschema::blocking::validate(&instance, &schema)?;
    // One-off iteration over errors
    for error in jsonschema::iter_errors(&instance, &schema).await? {
        println!("{}", error);
    }
    for error in jsonschema::blocking::iter_errors(&instance, &schema)? {
        println!("{}", error);
    }
    // One-off collecting validation results into a struct conforming to the JSON Schema "Verbose" output format 
    let verbose = jsonschema::collect_output(&instance, &schema, format::Verbose);
    // Serialize validation output to JSON (requires the `serde` feature)
    let serialized = serde_json::to_string(&verbose)?;
    // One-off iteration over validation results
    for unit in jsonschema::iter_output_units(&instance, &schema, format::Verbose) {
        println!("{:?}", unit);
    }

    // Async by default, autodetect draft based on the `$schema` property
    let validator = jsonschema::validator_for(&schema).await?;
    let validator = jsonschema::blocking::validator_for(&schema)?;
    // Specific draft
    let validator = jsonschema::Draft4Validator::from_schema(&schema).await?;
    let validator = jsonschema::blocking::Draft4Validator::from_schema(&schema)?;

    // Boolean result
    assert!(!validator.is_valid(&instance));
    // First error as `Result<(), jsonschema::Error>`
    assert!(validator.validate(&instance).is_err());

    // Iterate over errors
    for error in validator.iter_errors(&instance) {
        println!("{}", error);
    }

    // Collecting validation results into a struct conforming to the JSON Schema "Verbose" output format
    let verbose = validator.collect_output(&instance, format::Verbose);
    // Serialize validation output to JSON according to the verbose output format
    let serialized = serde_json::to_string(&verbose)?;
    // Iteration over validation results
    for unit in validator.iter_output_units(&instance, format::Verbose) {
        println!("{:?}", unit);
    }

    // Configuration
    let validator = jsonschema::Validator::options()
        // I.e. a resolver that forbids references
        .with_resolver(MyResolver::new())
        // Custom validator for the "format" keyword
        .with_format("card_number", CardNumberFormat::new())
        // Completely custom behavior for the `my-keyword` keyword
        .with_keyword("my-keyword", CustomKeywordValidator::new(42))
        .build(&schema)
        // .build_blocking(&schema)
        .await?;
}
```
