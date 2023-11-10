use std::{borrow::Cow, collections::VecDeque};

use error::ReferencingError;
use jsonlike::prelude::*;
use url::Url;

mod anchors;
mod error;
pub mod jsonschema;
mod path;
mod registry;
mod resources;
mod specification;
pub use registry::Registry;
pub use resources::Resource;
use resources::SubResource;
pub use specification::Specification;

// TODO:
//   - Idea: type parameter for crawled / uncrawled
//   - Boxed `Json` trait? to support different reprs, e.g. some resource is YAML, another is JSON
//   - Invalidate StoredResource::Referenced when an existing value is replaced
//   - Better type & variant names
//   - Rethink the whole structure and properly set pub/private types
//   - maybe in subresource
//   - Use `Cow` in retrieved? or just a reference
//   - Issue - no dyn dispatch for `Json` without specifying all assoc types

#[derive(Debug)]
pub struct Retrieved<'a, V, D: Json> {
    value: V,
    registry: &'a Registry<D>,
}

impl<'a, V, D: Json> Retrieved<'a, V, D> {
    pub fn new(value: V, registry: &'a Registry<D>) -> Self {
        Self { value, registry }
    }
}

#[derive(Debug)]
pub struct Resolved<'a, D: Json> {
    pub contents: &'a D,
    pub resolver: Resolver<'a, D>,
}

impl<'a, D: Json> Resolved<'a, D> {
    pub fn new(contents: &'a D, resolver: Resolver<'a, D>) -> Self {
        Self { contents, resolver }
    }
}

#[derive(Debug)]
pub struct Resolver<'a, D: Json> {
    base_uri: Url,
    registry: &'a Registry<D>,
    previous: VecDeque<Url>,
}

impl<'a, D: Json> Clone for Resolver<'a, D> {
    fn clone(&self) -> Self {
        Self {
            base_uri: self.base_uri.clone(),
            registry: self.registry,
            previous: self.previous.clone(),
        }
    }
}

impl<'a, D: Json> Resolver<'a, D> {
    pub fn new(base_uri: Url, registry: &'a Registry<D>) -> Self {
        Self {
            base_uri,
            registry,
            previous: VecDeque::new(),
        }
    }

    pub fn lookup(&self, reference: &str) -> Result<Resolved<'a, D>, ReferencingError> {
        let (uri, fragment) = if let Some(reference) = reference.strip_prefix('#') {
            (Cow::Borrowed(&self.base_uri), Cow::Borrowed(reference))
        } else {
            let mut full = self.base_uri.join(reference).expect("TODO");
            let fragment = full.fragment().unwrap_or_default().to_owned();
            full.set_fragment(None);
            (Cow::Owned(full), Cow::Owned(fragment))
        };
        let retrieved = self.registry.get_or_retrieve(&uri);
        if fragment.starts_with('/') {
            let resolver = self.evolve(retrieved.registry, &uri);
            todo!("Pointer should be implemented for both, resource & subresource")
            //  retrieved.value.pointer(&fragment, resolver)
        } else if !fragment.is_empty() {
            let retrieved = retrieved.registry.anchor(&uri, &fragment)?;
            let resolver = self.evolve(retrieved.registry, &uri);
            retrieved.value.resolve(resolver)
        } else {
            let resolver = self.evolve(retrieved.registry, &uri);
            Ok(Resolved::new(retrieved.value.contents(), resolver))
        }
    }

    /// Evolve, appending to the dynamic scope.
    fn evolve(&self, registry: &'a Registry<D>, base_uri: &Url) -> Self {
        let mut previous = self.previous.clone();
        // TODO: Re-check if it could be empty
        if !self.base_uri.as_str().is_empty()
            && (!previous.is_empty() || base_uri != &self.base_uri)
        {
            previous.push_front(self.base_uri.clone());
        }
        Resolver {
            base_uri: base_uri.to_owned(),
            registry,
            previous,
        }
    }

    /// Create a resolver for a subresource (which may have a new base URI).
    fn in_subresource(&self, subresource: SubResource<'a, D>) -> Resolver<D> {
        if let Some(id) = subresource.id() {
            Resolver {
                base_uri: self.base_uri.join(id).expect("Is it possible?"),
                registry: self.registry,
                previous: self.previous.clone(),
            }
        } else {
            self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use jsonlike::prelude::*;

    use crate::{
        anchors::{Anchor, Resolvable},
        path::{jsonpath, JsonPath, Segment},
        resources::SubResource,
        specification::AnchorIter,
        Resolver, Specification,
    };

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct IdAndChildren;

    impl<D: Json> Specification<D> for IdAndChildren {
        fn name(&self) -> &'static str {
            "id-and-children"
        }

        fn id_of<'a>(&self, document: &'a D) -> Option<&'a str> {
            document
                .as_object()
                .and_then(|obj| obj.get("ID"))
                .and_then(Json::as_string)
                .map(Borrow::borrow)
        }

        fn subresources_of<'a>(
            &self,
            document: &'a D,
        ) -> Box<dyn Iterator<Item = (JsonPath, &'a D)> + 'a> {
            let mut subresources_of = vec![];
            if let Some(object) = document.as_object() {
                if let Some(children) = object.get("children").and_then(Json::as_array) {
                    for (idx, element) in children.iter().enumerate() {
                        subresources_of.push((
                            jsonpath!("children", idx),
                            element.expect("Failed to get the element"),
                        ))
                    }
                }
            }
            Box::new(subresources_of.into_iter())
        }

        fn anchors_in<'a>(&self, document: &'a D) -> AnchorIter<'a, D> {
            let mut anchors_in = vec![];
            if let Some(object) = document.as_object() {
                if let Some(anchors) = object.get("anchors").and_then(Json::as_object) {
                    for (key, _) in anchors.iter() {
                        if let Ok(key) = key {
                            let anchor = Anchor::new(key.as_ref().to_owned(), jsonpath!("anchors"));
                            let boxed: Box<dyn Resolvable<D>> = Box::new(anchor);
                            anchors_in.push(boxed);
                        }
                    }
                }
            }
            Box::new(anchors_in.into_iter())
        }

        fn maybe_in_subresource<'a>(
            &self,
            segments: &[Segment],
            resolver: &'a Resolver<'a, D>,
            subresource: SubResource<'a, D>,
        ) -> Resolver<'a, D> {
            if segments.len() % 2 == 0
                && segments
                    .iter()
                    .step_by(2)
                    .all(|each| each == &Segment::from("children".to_owned()))
            {
                resolver.in_subresource(subresource)
            } else {
                resolver.clone()
            }
        }

        fn box_clone(&self) -> Box<dyn Specification<D>> {
            Box::new(*self)
        }
    }
}
