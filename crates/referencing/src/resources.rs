use aho_corasick::AhoCorasick;
use std::{borrow::Borrow, collections::HashMap};
use url::Url;

use jsonlike::prelude::*;

use crate::{
    anchors::Resolvable,
    error::ReferencingError,
    jsonschema::Draft4,
    path::{JsonPath, Segment},
    Resolved, Resolver, Specification,
};

#[derive(Debug)]
pub enum Res<'a, D: Json> {
    Owned(&'a Resource<D>),
    Referenced(SubResource<'a, D>),
}

impl<'a, D: Json> Res<'a, D> {
    pub(crate) fn contents(&self) -> &'a D {
        match self {
            Res::Owned(resource) => &resource.contents,
            Res::Referenced(subresource) => subresource.contents,
        }
    }
}

/// A document with a concrete interpretation under a specification.
#[derive(Debug)]
pub struct Resource<D: Json> {
    pub contents: D,
    // TODO: Properly explain why dynamic dispatch
    pub specification: Box<dyn Specification<D>>,
}

impl<D: Json> PartialEq for Resource<D> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: It should not be by name
        self.specification.name() == other.specification.name()
            && self.contents.equal(&other.contents)
    }
}

impl<D: Json> Resource<D> {
    /// Create a new resource with the given specification.
    pub fn new(contents: D, specification: Box<dyn Specification<D>>) -> Self {
        Self {
            contents,
            specification,
        }
    }
    pub fn from_contents(
        contents: D,
        default_specification: Option<Box<dyn Specification<D>>>,
    ) -> Resource<D> {
        // TODO: Properly handle no spec
        let specification = if let Some(object) = contents.as_object() {
            if let Some(dialect_id) = object.get("$schema").and_then(Json::as_string) {
                match dialect_id.borrow() {
                    "http://json-schema.org/draft-04/schema" => Draft4.boxed(),
                    _ => default_specification.unwrap(),
                }
            } else {
                default_specification.unwrap()
            }
        } else {
            default_specification.unwrap()
        };
        Resource {
            contents,
            specification,
        }
    }
    /// Retrieve resource's identifier.
    pub fn id(&self) -> Option<&str> {
        self.specification
            .id_of(&self.contents)
            .map(|id| id.trim_end_matches('#'))
    }
    /// Retrieve this resource's subresources.
    pub fn subresources<'a>(&'a self) -> Box<dyn Iterator<Item = SubResource<'a, D>> + 'a> {
        Box::new(
            self.specification
                .subresources_of(&self.contents)
                .map(|(path, subresource)| {
                    SubResource::from_contents(
                        path.clone(),
                        subresource,
                        Some(self.specification.box_clone()),
                    )
                }),
        )
    }
    /// Retrieve this resource's (specification-specific) identifier.
    pub fn anchors<'a>(&'a self) -> Box<dyn Iterator<Item = Box<dyn Resolvable<D>>> + 'a> {
        self.specification.anchors_in(&self.contents)
    }

    /// Resolve the given JSON pointer.
    pub(crate) fn pointer<'a>(
        &self,
        pointer: &str,
        resolver: Resolver<'a, D>,
    ) -> Result<Resolved<'a, D>, ReferencingError> {
        let mut contents = &self.contents;
        let mut segments = vec![];
        let mut pointer_chars = pointer.chars();
        pointer_chars.next();
        let trimmed_pointer = pointer_chars.as_str();
        let automaton =
            AhoCorasick::new(["~1", "~0"]).expect("Invalid patterns for Aho-Corasick automaton");
        for segment in percent_encoding::percent_decode_str(trimmed_pointer)
            .decode_utf8_lossy()
            .split('/')
        {
            let parsed_segment;
            (contents, parsed_segment) = if let Some(array) = contents.as_array() {
                let idx = segment.parse::<usize>().expect("Properly return an error");
                (
                    array
                        .get(idx)
                        .ok_or_else(|| ReferencingError::pointer_to_nowhere(pointer))?,
                    Segment::from(idx),
                )
            } else if let Some(object) = contents.as_object() {
                if automaton.is_match(segment) {
                    let key = automaton.replace_all(segment, &["/", "~"]);
                    (
                        object
                            .get(&key)
                            .ok_or_else(|| ReferencingError::pointer_to_nowhere(pointer))?,
                        Segment::from(key),
                    )
                } else {
                    (
                        object
                            .get(segment)
                            .ok_or_else(|| ReferencingError::pointer_to_nowhere(pointer))?,
                        Segment::from(segment.to_owned()),
                    )
                }
            } else {
                return Err(ReferencingError::unresolvable(pointer));
            };
            segments.push(parsed_segment);
            // last_resolver = &resolver;
            let _resolver = self.specification.maybe_in_subresource(
                &segments,
                &resolver,
                SubResource {
                    contents,
                    path: JsonPath::new(),
                    specification: self.specification.box_clone(),
                },
            );
            // if resolver is not last:
            //     segments = []
        }
        Ok(Resolved {
            contents: todo!(),
            resolver,
        })
    }

    fn get_subresource(&self, path: &JsonPath) -> Option<SubResource<D>> {
        let mut contents = &self.contents;
        for segment in path.iter() {
            match segment {
                Segment::Key(key) => {
                    contents = contents.as_object().and_then(|object| object.get(key))?;
                }
                Segment::Index(index) => {
                    contents = contents.as_array().and_then(|array| array.get(*index))?;
                }
            }
        }
        Some(SubResource {
            contents,
            path: path.clone(),
            specification: self.specification.box_clone(),
        })
    }
}

#[derive(Debug)]
pub struct SubResource<'a, D: Json> {
    pub contents: &'a D,
    // TODO: Maybe remove it? to avoid cloning `JsonPath` into it
    pub path: JsonPath,
    pub specification: Box<dyn Specification<D>>,
}
impl<'a, D: Json> PartialEq for SubResource<'a, D> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: It should not be by name
        self.specification.name() == other.specification.name()
            && self.path == other.path
            && self.contents.equal(other.contents)
    }
}

impl<'a, D: Json> SubResource<'a, D> {
    pub fn from_contents(
        path: JsonPath,
        contents: &D,
        default_specification: Option<Box<dyn Specification<D>>>,
    ) -> SubResource<D> {
        // TODO: Properly handle no spec
        let specification = if let Some(object) = contents.as_object() {
            if let Some(dialect_id) = object.get("$schema").and_then(Json::as_string) {
                match dialect_id.borrow() {
                    "http://json-schema.org/draft-04/schema" => Draft4.boxed(),
                    _ => default_specification.unwrap(),
                }
            } else {
                default_specification.unwrap()
            }
        } else {
            default_specification.unwrap()
        };
        SubResource {
            contents,
            path,
            specification,
        }
    }
    /// Retrieve resource's identifier.
    pub fn id(&self) -> Option<&str> {
        self.specification
            .id_of(self.contents)
            .map(|id| id.trim_end_matches('#'))
    }
    pub fn subresources(&'a self) -> Box<dyn Iterator<Item = SubResource<'a, D>> + 'a> {
        Box::new(
            self.specification
                .subresources_of(self.contents)
                .map(|(path, subresource)| {
                    SubResource::from_contents(
                        path.clone(),
                        subresource,
                        Some(self.specification.box_clone()),
                    )
                }),
        )
    }
    pub fn into_reference_in(self, uri: Url) -> ResourceReference {
        ResourceReference::new(uri, self.path)
    }
}

/// A map of resources identified by URI.
#[derive(Debug, Default)]
pub struct ResourceMap<D: Json> {
    owned: HashMap<Url, Resource<D>>,
    referenced: HashMap<Url, ResourceReference>,
}

impl<J: Json> ResourceMap<J> {
    pub(crate) fn new() -> ResourceMap<J> {
        ResourceMap {
            owned: HashMap::default(),
            referenced: HashMap::default(),
        }
    }
    /// Return the number of resources in this map.
    pub fn len(&self) -> usize {
        self.owned.len() + self.referenced.len()
    }

    pub fn is_empty(&self) -> bool {
        self.owned.is_empty() && self.referenced.is_empty()
    }

    /// Return the `Resource` identified by the given URI.
    pub fn get(&self, key: &Url) -> Option<Res<J>> {
        // TODO: Avoid key cloning
        if let Some(owned) = self.get_resource(key) {
            Some(Res::Owned(owned))
        } else if let Some(reference) = self.referenced.get(key) {
            self.get_subresource(reference).map(Res::Referenced)
        } else {
            None
        }
    }
    pub fn get_resource(&self, key: &Url) -> Option<&Resource<J>> {
        let mut key = key.clone();
        key.set_fragment(None);
        self.owned.get(&key)
    }

    pub fn get_subresource(&self, reference: &ResourceReference) -> Option<SubResource<J>> {
        let mut uri = reference.parent_uri.clone();
        uri.set_fragment(None);
        self.owned.get(&uri)?.get_subresource(&reference.path)
    }

    pub fn insert(&mut self, key: Url, resource: Resource<J>) {
        self.owned.insert(key, resource);
    }

    pub fn insert_referenced(&mut self, key: Url, resource: ResourceReference) {
        self.referenced.insert(key, resource);
    }
}

#[derive(Debug)]
pub struct ResourceReference {
    pub parent_uri: Url,
    pub path: JsonPath,
}

impl ResourceReference {
    pub fn new(parent_uri: Url, path: JsonPath) -> Self {
        Self { parent_uri, path }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use test_case::test_case;

    use crate::{jsonschema::Draft4, Specification};

    #[test_case(json!({ "id": "foo" }), Some("foo"))]
    #[test_case(json!({}), None)]
    fn test_id(contents: Value, expected: Option<&str>) {
        let resource = Draft4.create_resource(contents);
        assert_eq!(resource.id(), expected);
    }

    #[test]
    fn test_subresource_valid() {
        let resource = Draft4.create_resource(json!({"not": {"type": "integer"}}));
        let subresources = resource.subresources().collect::<Vec<_>>();
        assert_eq!(subresources.len(), 1);
    }
}
