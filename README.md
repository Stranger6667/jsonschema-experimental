# jsonschema: JSON Schema Validation for Rust

> **⚠️ THIS LIBRARY IS WORK-IN-PROGRESS ⚠️**

> This crate is a from-scratch rewrite of [jsonschema-rs](https://github.com/Stranger6667/jsonschema-rs) that aims to address some of the design flaws.
> It started as a separate private repo, but I plan to move the development back to that one.
> For an in-depth roadmap, please take a look [here](https://github.com/Stranger6667/jsonschema/issues/1)
> This README represent the end goal and serves as the reference for the ongoing development.

The `jsonschema` crate offers performant and flexible JSON Schema validation for Rust.
It provides both async and blocking reference resolving and is designed to be easy to use.
The following JSON Schema drafts are supported:

- Draft 4
- Draft 6
- Draft 7
- Draft 2019-09
- Draft 2020-12

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
jsonschema = "0.18.0"
```

## Quick Start

One-off validation:

```rust
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = json!({"type": "integer"});
    let instance = json!("a");
    jsonschema::validate(&instance, &schema).await;
    Ok(())
}
```

## Usage

`jsonschema` provides an async API by default:

```rust
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = json!({"type": "integer"});
    let instance = json!("a");
    // Boolean result
    assert!(!jsonschema::is_valid(&instance, &schema).await);
    // Only first error as `Result<(), jsonschema::ValidationError>`
    jsonschema::validate(&instance, &schema).await?;
    // Iterate over all errors
    for error in jsonschema::iter_errors(&instance, &schema).await {
        println!("{}", error);
    }
    Ok(())
}
```

This method is preferred if your schema includes external references, requiring non-blocking IO operations.
The blocking API is available inside the `blocking` module. Use it if your schema does not contain any external references.

```rust
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = json!({"type": "integer"});
    let instance = json!("a");
    // Boolean result
    assert!(!jsonschema::blocking::is_valid(&instance, &schema));
    // Only first error as `Result<(), jsonschema::ValidationError>`
    jsonschema::blocking::validate(&instance, &schema)?;
    // Iterate over all errors
    for error in jsonschema::blocking::iter_errors(&instance, &schema) {
        println!("{}", error);
    }
    Ok(())
}
```

If you need to validate multiple instances against the same schema, build a validator upfront:

```rust
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = json!({"type": "integer"});
    // Build once, reuse many times
    let validator = jsonschema::validator_for(&schema).await?;
    let instances = vec![json!(1), json!(2), json!("a"), json!(3)];
    for instance in instances {
        assert!(validator.is_valid(&instance));
        validator.validate(&instance)?;
        for error in validator.iter_errors(&instance) {
            println!("{}", error);
        }
    }
    Ok(())
}
```

## Advanced Usage

### Output formatting

`jsonschema` supports multiple output formats for validation results in accordance with the JSON Schema specification:

- `Flag`
- `Basic`
- `Detailed`
- `Verbose`

```rust
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... omitted for brevity
    let verbose = jsonschema::evaluate(&instance, &schema).await.verbose();
    // Serialize validation output to JSON
    let serialized = serde_json::to_string(&verbose)?;
    Ok(())
}
```

### Customization

```rust
use jsonschema::{Json, Draft, BuildResult, BoxedFormat, BoxedKeyword};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... omitted for brevity
    struct Resolver;

    impl jsonschema::ReferenceResolver for Resolver {
        fn resolve_external(&self, url: &str) -> impl core::future::Future<Output = ()> {
            async {}
        }
    };

    struct FixedSize {
        size: usize,
    }

    impl jsonschema::Format for FixedSize {
        fn is_valid(&self, value: &str) -> bool {
            value.len() == self.size
        }
    }

    fn fixed_size_factory<J: Json>(schema: &J) -> BuildResult<BoxedFormat> {
        Ok(Box::new(FixedSize { size: 43 }))
    }

    #[derive(Debug)]
    struct AsciiKeyword {
        size: usize
    }

    impl<J: Json> jsonschema::Keyword<J> for AsciiKeyword {
        fn is_valid(&self, instance: &J) -> bool {
            if let Some(string) = instance.as_string().map(AsRef::as_ref) {
                string.len() == self.size && string.chars().all(|c| c.is_ascii())
            } else {
                true
            }
        }
    }

    fn ascii_keyword_factory<J: Json>(schema: &J) -> BuildResult<BoxedKeyword<J>> {
        Ok(Box::new(AsciiKeyword { size: 42 }))
    }

    let validator = jsonschema::ValidatorBuilder::default()
        .draft(Draft::Draft07)
        .resolver(Resolver)
        .format(
            "fixed-size-1",
            |schema| -> BuildResult<BoxedFormat> {
                Ok(Box::new(FixedSize { size: 5 }))
            }
        )
        .format("fixed-size-2", fixed_size_factory)
        .keyword(
            "ascii",
            |schema| -> BuildResult<BoxedKeyword<_>> {
                Ok(Box::new(AsciiKeyword { size: 42 }))
            }
        )
        .keyword("also-ascii", ascii_keyword_factory)
        .build(&schema)
        .await?;

    Ok(())
}
```
