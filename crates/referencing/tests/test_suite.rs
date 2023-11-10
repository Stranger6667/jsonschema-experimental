use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use once_cell::sync::Lazy;
use referencing::{jsonschema, Registry, Resource, Specification};
use serde_json::Value;
use url::Url;

static DIALECT_IDS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let file = File::open("suite/tests/specifications.json").expect("Failed to open file");
    serde_json::from_reader(file).expect("Invalid specifications file")
});

fn get_dialect_id(case: &Path) -> &str {
    let dialect = case
        .parent()
        .expect("Missing parent")
        .file_stem()
        .expect("Missing file stem")
        .to_string_lossy();
    &DIALECT_IDS[dialect.as_ref()]
}

#[derive(Debug, serde::Deserialize)]
struct TestGroup {
    #[serde(rename = "$schema")]
    schema: String,
    registry: HashMap<String, Value>,
    tests: Vec<TestCase>,
}

impl TestGroup {
    fn from_path(path: &Path) -> Self {
        let file = File::open(
            path.strip_prefix("crates/referencing/")
                .expect("Invalid prefix"),
        )
        .unwrap_or_else(|e| panic!("Failed to open file: {}\n{e}", path.display()));
        serde_json::from_reader(file).expect("Failed to parse JSON")
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum TestCase {
    Success(Success),
    Error {
        base_uri: Option<String>,
        #[serde(rename = "ref")]
        reference: String,
        error: bool,
    },
}

#[derive(Debug, serde::Deserialize)]
struct Success {
    base_uri: Option<String>,
    #[serde(rename = "ref")]
    reference: String,
    target: Value,
    then: Option<Box<Success>>,
}

impl TestCase {
    fn base_uri(&self) -> Url {
        let base_uri = match self {
            TestCase::Success(Success { base_uri, .. }) => base_uri,
            TestCase::Error { base_uri, .. } => base_uri,
        };
        if let Some(base_uri) = base_uri {
            Url::parse(base_uri).expect("Invalid URL")
        } else {
            todo!()
        }
    }
}

#[suite::test("crates/referencing/suite/tests")]
fn test_suite(path: PathBuf) {
    let dialect_id = get_dialect_id(&path);
    let group = TestGroup::from_path(&path);
    let specification = match dialect_id.trim_matches('#') {
        //   "https://json-schema.org/draft/2020-12/schema" => {},
        //   "https://json-schema.org/draft/2019-09/schema" => DRAFT201909,
        //   "http://json-schema.org/draft-07/schema" => DRAFT7,
        //   "http://json-schema.org/draft-06/schema" => DRAFT6,
        "http://json-schema.org/draft-04/schema" => jsonschema::Draft4.boxed(),
        //   "http://json-schema.org/draft-03/schema" => DRAFT3,
        _ => todo!(),
    };
    let registry =
        Registry::new().with_resources(group.registry.into_iter().map(|(key, value)| {
            (
                Url::parse(&key).expect("Invalid URL"),
                Resource::new(value, specification.box_clone()),
            )
        }));
    for test in group.tests {
        let base_uri = test.base_uri();
        let resolver = registry.resolver(base_uri);
        match test {
            TestCase::Success(Success {
                reference,
                target,
                then,
                ..
            }) => {
                let mut resolved = resolver
                    .lookup(&reference)
                    .expect("Failed to resolve reference");
                assert_eq!(resolved.contents, &target);
                let mut then = then;
                while let Some(next) = then {
                    resolved = resolved
                        .resolver
                        .lookup(&next.reference)
                        .expect("Failed to resolve reference");
                    assert_eq!(resolved.contents, &next.target);
                    then = next.then;
                }
            }
            TestCase::Error { reference, .. } => {
                let resolved = resolver.lookup(&reference);
                assert!(resolved.is_err());
                let error = resolved.expect_err("Should be an error");
                assert_eq!(error.to_string(), "BLA");
            }
        }
    }
}
