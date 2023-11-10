use std::collections::HashSet;

use jsonlike::Json;
use url::Url;

use crate::{
    anchors::{AnchorMap, Resolvable},
    error::ReferencingError,
    resources::{self, Res},
    Resolver, Resource, Retrieved,
};

#[derive(Default, Debug)]
pub struct Registry<D: Json> {
    resources: resources::ResourceMap<D>,
    anchors: AnchorMap<D>,
    uncrawled: HashSet<Url>,
}

impl<D: Json> Registry<D> {
    pub fn new() -> Registry<D> {
        Registry {
            resources: resources::ResourceMap::new(),
            anchors: AnchorMap::default(),
            uncrawled: HashSet::default(),
        }
    }
    /// Return the `Resource` identified by the given URI.
    pub fn get<'a>(&'a self, uri: &Url) -> Option<Res<'a, D>> {
        self.resources.get(uri)
    }
    /// Count the total number of resources in this registry.
    pub fn len(&self) -> usize {
        self.resources.len()
    }
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
    /// Crawl all added resources, discovering subresources.
    pub fn crawl(&mut self) {
        let mut subresources = Vec::new();
        // Inspect the known uncrawled resources first
        while let Some(uri) = self.uncrawled.drain().next() {
            // INVARIANT: These resources always exist in both, `self.resources` &
            // `self.uncrawled`, therefore this should never panic.
            let resource = self
                .resources
                .get_resource(&uri)
                .unwrap_or_else(|| panic!("Resource does not exist: {uri}"));
            // Push all subresources for further traversal.
            // NOTE: Subresources are represesented as their parent ID + a path to this subresource
            for subresource in resource.subresources() {
                subresources.push(subresource.into_reference_in(uri.clone()))
            }
            for anchor in resource.anchors() {
                // TODO: Implement AnchorKey, so it is easier to implement Borrow for it
                let name = anchor.name().to_owned();
                self.anchors.insert(uri.clone(), name, anchor);
            }
        }
        // Then traverse all reachable subresources
        while let Some(reference) = subresources.pop() {
            // INVARIANT: Every subresource is built from a path within a valid parent resource
            // and is created within this function, therefore this should never panic.
            let subresource = self
                .resources
                .get_subresource(&reference)
                .unwrap_or_else(|| {
                    panic!(
                        "Subresource does not exist: {} in {}",
                        reference.path, reference.parent_uri
                    )
                });
            for subresource in subresource.subresources() {
                subresources.push(subresource.into_reference_in(reference.parent_uri.clone()))
            }
            if let Some(id) = subresource.id() {
                let uri = reference.parent_uri.join(id).expect("Can it fail?");
                self.resources
                    .insert_referenced(uri, subresource.into_reference_in(reference.parent_uri));
            }
        }
    }

    pub fn with_resource(self, uri: Url, resource: Resource<D>) -> Registry<D> {
        self.with_resources([(uri, resource)].into_iter())
    }

    pub fn with_resources(self, pairs: impl Iterator<Item = (Url, Resource<D>)>) -> Registry<D> {
        let mut resources = self.resources;
        let mut uncrawled = self.uncrawled;
        for (mut uri, resource) in pairs {
            uri.set_fragment(None);
            resources.insert(uri.clone(), resource);
            uncrawled.insert(uri);
        }
        Registry {
            resources,
            anchors: AnchorMap::default(),
            uncrawled,
        }
    }
    /// Return a `Resolver` which resolves references against this registry.
    pub fn resolver(&self, base_uri: Url) -> Resolver<'_, D> {
        Resolver::new(base_uri, self)
    }

    pub fn get_or_retrieve(&self, uri: &Url) -> Retrieved<Res<D>, D> {
        if let Some(resource) = self.resources.get(uri) {
            todo!()
            //  return Retrieved {
            //      value: AnchorOrResource::Resource(resource),
            //      registry: self,
            //  };
        }

        //  registry = self.crawl()
        //  resource = registry._resources.get(uri)
        //  if resource is not None:
        //      return Retrieved(registry=registry, value=resource)

        //  try:
        //      resource = registry._retrieve(uri)
        //  except (
        //      exceptions.CannotDetermineSpecification,
        //      exceptions.NoSuchResource,
        //  ):
        //      raise
        //  except Exception:
        //      raise exceptions.Unretrievable(ref=uri)
        //  else:
        //      registry = registry.with_resource(uri, resource)
        //      return Retrieved(registry=registry, value=resource)
        todo!()
    }

    /// Retrieve a given anchor from a resource which must already be crawled.
    pub fn anchor(
        &self,
        uri: &Url,
        name: &str,
    ) -> Result<Retrieved<Box<dyn Resolvable<D> + '_>, D>, ReferencingError> {
        if let Some(value) = self.anchors.get(uri, name) {
            return Ok(Retrieved::new(value, self));
        }
        // TODO: Maybe interior mutability?

        //        registry = self.crawl()
        //        value = registry._anchors.get((uri, name))
        //        if value is not None:
        //            return Retrieved(value=value, registry=registry)
        //
        //        resource = self[uri]
        //        canonical_uri = resource.id()
        //        if canonical_uri is not None:
        //            value = registry._anchors.get((canonical_uri, name))
        //            if value is not None:
        //                return Retrieved(value=value, registry=registry)
        //
        //        if "/" in name:
        //            raise exceptions.InvalidAnchor(
        //                ref=uri,
        //                resource=resource,
        //                anchor=name,
        //            )
        Err(ReferencingError::NoSuchAnchor {
            reference: uri.to_owned(),
            anchor: name.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;

    use crate::{tests::IdAndChildren, Registry, Specification};

    #[test]
    fn test_crawl_still_has_top_level_resource() {
        let contents = json!({"foo": "bar"});
        let resource = IdAndChildren.create_resource(contents.clone());
        let uri = Url::parse("urn:example").expect("Invalid URL");
        let mut registry = Registry::new().with_resource(uri.clone(), resource);
        registry.crawl();
        let resource = registry.get(&uri).expect("Resource not found");
        assert_eq!(resource.contents(), &contents);
    }

    #[test]
    fn test_crawl_finds_a_subresource() {
        let child_id = Url::parse("urn:child").expect("Invalid URL");
        let contents =
            json!({"ID": "urn:root", "children": [{"ID": child_id.as_str(), "foo": 12}]});
        let root = IdAndChildren.create_resource(contents);
        let root_id = Url::parse(root.id().expect("Missing ID")).expect("Invalid URL");
        let mut registry = Registry::new().with_resource(root_id, root);
        assert!(registry.get(&child_id).is_none());
        registry.crawl();
        let resource = registry.get(&child_id).expect("Resource not found");
        assert_eq!(
            resource.contents(),
            &json!({"ID": child_id.as_str(), "foo": 12})
        );
    }

    #[test]
    fn test_crawl_finds_anchors_with_id() {
        let root = IdAndChildren.create_resource(json!({"ID": "urn:bar", "anchors": {"foo": 12}}));
        let root_id = Url::parse(root.id().expect("Missing ID")).expect("Invalid URL");
        let mut registry = Registry::new().with_resource(root_id.clone(), root);
        registry.crawl();
        let anchor = registry.anchor(&root_id, "foo").expect("Anchor not found");
        //   assert_eq!(anchor.value, 12);
    }
}
