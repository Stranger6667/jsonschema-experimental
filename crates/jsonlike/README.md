# jsonlike: Unified interface for JSON-like data in Rust

`jsonlike` allows for generic access to JSON-like data without unnecessary serialization.

## Why jsonlike?

While many libraries are using a single JSON representation (like `serde_json`), `jsonlike` aims to offer a consistent, abstracted interface. 
This enables you to write Rust code that's not tied to a single JSON implementation.

> This is an experimental crate. More detailed documentation and examples are coming soon.

## Features

- Zero-dependency core crate.
- Optional feature flags for `serde_json` and `pyo3` integration.
- A set of traits for easy implementation to make your types behave like JSON.

## Quick Start

Here's an example that shows how you can interact with Python dictionaries using `jsonlike`:

```rust
use pyo3::prelude::*;
use jsonlike::prelude::*;

fn main() {
    Python::with_gil(|py| {
        let value = py.eval("{\"a\": 1}", None, None).expect("Invalid Python code");
        let object = value.as_object().expect("Should be an object");
        assert!(object.get("a").is_some());
    }).unwrap();
}
```

Inspiration
- https://github.com/timothee-haudebourg/generic-json/
- https://github.com/macisamuele/json-trait-rs/

Inspiration
- https://github.com/timothee-haudebourg/generic-json/
- https://github.com/macisamuele/json-trait-rs/

## License

This project is licensed under the MIT License.
