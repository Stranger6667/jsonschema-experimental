/*!

  One-off validation:

  ```rust
  use serde_json::json;

  let schema = json!({"type": "integer"});
  let instance = json!("a");
  jsonschema::validate(&schema, &instance);
  ````

  New validator (auto-detection)

  ```rust
  use serde_json::json;

  let schema = json!({"type": "integer"});
  let validator = jsonschema::Validator::from_schema(&schema)
      .await
      .expect("Invalid schema");

  assert!(schema.is_valid(&json!(1)));
  assert!(!schema.is_valid(&json!("a")));
  ```

  Specific draft

  ```rust
  use serde_json::json;

  let schema = json!({"type": "integer"});
  let validator = jsonschema::Draft4Validator::from_schema(&schema)
      .await
      .expect("Invalid schema");
  ```

  Configure validator:

  ```rust
  use serde_json::json;

  let schema = json!({"type": "integer"});
  let validator = jsonschema::Validator::options()
      .with_resolver(MyResolver)
      .build(&schema)
      .await
      .expect("Invalid schema");
  ```

*/
mod compiler;
mod drafts;
mod error;
mod schema;
mod validator;

pub use drafts::{draft04, Draft};
use drafts::{draft04::Draft04, Autodetect};
pub use error::{JsonSchemaError, SchemaError, ValidationError};
use jsonlike::Json;
pub use schema::JsonSchemaValidator;

pub type Draft4Validator = validator::Validator<Draft04>;
pub type Validator = validator::Validator<Autodetect>;

// # Validation - reuse
//
// validator.is_valid(...)
// for error in validator.validate(...) {
//     ...
// }

// TODO: Check the most popular order of arguments.
pub async fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), JsonSchemaError> {
    todo!()
}

pub mod blocking {
    use jsonlike::Json;

    use crate::{drafts::Draft04, validator, JsonSchemaError};

    pub type Draft4Validator = validator::blocking::Validator<Draft04>;
    pub type Validator = validator::blocking::Validator;

    pub fn validate<J: Json>(schema: &J, instance: &J) -> Result<(), JsonSchemaError> {
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
