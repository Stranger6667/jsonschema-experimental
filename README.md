# jsonschema

Experimental JSON Schema validator for Rust.

> **⚠️ THIS LIBRARY IS WORK-IN-PROGRESS ⚠️**

This crate is a from-scratch rewrite of [jsonschema-rs](https://github.com/Stranger6667/jsonschema-rs) that aims to address some of the design flaws.
It started as a separate private repo, but I plan to move the development back to that one.
For an in-depth roadmap, please take a look [here](https://github.com/Stranger6667/jsonschema/issues/1)

This is how library API may look like:

```rust
use jsonschema::{validator, Validator, draft4, blocking};
use serde_json::json;

// One-off validation with a boolean result
jsonschema::is_valid(&json!({"type": "integer"}), &json!(5)).await?;

// Macro for validator (async by default, autodetect draft, defaults to latest)
let validator = validator!({"type": "integer"}).await?;

// Boolean result
validator.is_valid(&json!(5));

// Lazy result
let instance = json!("abc");
let result = validator.validate(&instance);

// Boolean result from the lazy one
result.is_valid();

// Error iterator
for error in result.errors() {
    println!("{}", error);
}

// Result formatting with different styles (`serde` feature)
let verbose: serde_json::Value = result.format().verbose();
let basic: serde_yaml::Value = result.format().basic();
let custom: serde_json::Value = result.format().with(MyCustomFormatter);

// Validator for a specific draft (`draft4` feature)
let validator = draft4::validator!({"type": "integer"}).await?;

// Non-macro
let schema = json!({"type": "integer"});
let validator = Validator::from_schema(&schema).await?;
let validator = draft4::Validator::from_schema(&schema).await?;

// Blocking ref resolving
blocking::is_valid(&json!({"type": "integer"}), &json!(5))?;
let validator = blocking::Validator::from_schema(&schema)?;
let validator = blocking::draft4::Validator::from_schema(&schema)?;

// Configuration
let validator = Validator::options()
    // I.e. a resolver that forbids references
    .with_resolver(MyResolver::new())
    // Custom validator for the "format" keyword
    .with_format("card_number", CardNumberFormat::new())
    // Completely custom behavior for the `my-keyword` keyword
    .with_keyword("my-keyword", CustomKeywordValidator::new(42))
    .build(&schema)
    .await?;
```
