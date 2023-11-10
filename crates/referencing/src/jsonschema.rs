use std::borrow::Borrow;

use jsonlike::prelude::*;

use crate::{
    anchors::{Anchor, Resolvable},
    error::ReferencingError,
    path::{JsonPath, Segment},
    resources::SubResource,
    specification::AnchorIter,
    Resolved, Resolver, Specification,
};

fn dollar_id<J: Json>(contents: &J) -> Option<String> {
    contents
        .as_object()
        .and_then(|obj| obj.get("$id"))
        .and_then(|id| id.as_string())
        .map(|id| id.borrow().to_owned())
}

fn legacy_id<J: Json>(contents: &J) -> Option<&str> {
    let Some(object) = contents.as_object() else {
        return None;
    };
    if object.contains_key("$ref") {
        return None;
    }
    if let Some(id) = object.get("id").and_then(Json::as_string) {
        // TODO: Borrow is not really convenient - Deref would be nice
        let id = id.borrow();
        if !id.starts_with('#') {
            return Some(id);
        }
    }
    None
}

#[derive(Debug)]
struct DynamicAnchor {
    name: String,
    path: JsonPath,
}

impl DynamicAnchor {
    fn new(name: String, path: JsonPath) -> Self {
        Self { name, path }
    }
}
impl<D: Json> Resolvable<D> for DynamicAnchor {
    fn name(&self) -> &str {
        &self.name
    }

    /// Return the resource for this anchor.
    fn resolve<'a>(&self, resolver: Resolver<'a, D>) -> Result<Resolved<'a, D>, ReferencingError> {
        todo!()
    }
}

fn anchor<J: Json>(contents: &J) -> AnchorIter<'_, J> {
    let Some(object) = contents.as_object() else {
        return Box::new(std::iter::empty());
    };
    if let Some(anchor) = object.get("$anchor").and_then(Json::as_string) {
        let anchor: Box<dyn Resolvable<J>> =
            Box::new(Anchor::new(anchor.borrow().to_owned(), JsonPath::new()));
        return Box::new(std::iter::once(anchor));
    }
    if let Some(dynamic_anchor) = object.get("$dynamicAnchor").and_then(Json::as_string) {
        let anchor: Box<dyn Resolvable<J>> = Box::new(DynamicAnchor::new(
            dynamic_anchor.borrow().to_owned(),
            JsonPath::new(),
        ));
        return Box::new(std::iter::once(anchor));
    }
    Box::new(std::iter::empty())
}

fn anchor_2019<J: Json>(contents: &J) -> AnchorIter<'_, J> {
    let Some(object) = contents.as_object() else {
        return Box::new(std::iter::empty());
    };
    if let Some(anchor) = object.get("$anchor").and_then(Json::as_string) {
        let anchor: Box<dyn Resolvable<J>> =
            Box::new(Anchor::new(anchor.borrow().to_owned(), JsonPath::new()));
        return Box::new(std::iter::once(anchor));
    }
    Box::new(std::iter::empty())
}

fn legacy_anchor_in_id<J: Json>(contents: &J) -> AnchorIter<'_, J> {
    let Some(object) = contents.as_object() else {
        return Box::new(std::iter::empty());
    };
    if let Some(id) = object.get("$id").and_then(Json::as_string) {
        if let Some(id) = id.borrow().strip_prefix('#') {
            // TODO: build proper json path
            let anchor: Box<dyn Resolvable<J>> =
                Box::new(Anchor::new(id.to_owned(), JsonPath::new()));
            return Box::new(std::iter::once(anchor));
        }
    }
    Box::new(std::iter::empty())
}

macro_rules! iter_values {
    ($object:expr, $keys:expr) => {
        $keys.into_iter().filter_map(|key| $object.get(key))
    };
}

macro_rules! iter_subarrays {
    ($object:expr, $keys:expr) => {
        $keys
            .into_iter()
            .filter_map(|key| $object.get(key))
            .filter_map(Json::as_array)
            .flat_map(|value| value.iter())
            .filter_map(|item| item.ok())
    };
}

macro_rules! iter_subvalues {
    ($object:expr, $keys:expr) => {
        $keys
            .into_iter()
            .filter_map(|key| $object.get(key))
            .filter_map(Json::as_object)
            .flat_map(|value| value.iter())
            .map(|(_, subvalue)| subvalue)
    };
}

enum ArrayOrSingle<'a, J: Json> {
    ArrayIter(<J::Array as JsonArray>::Iter<'a>),
    SingleIter(std::iter::Once<&'a J>),
}

impl<'a, J: Json> Iterator for ArrayOrSingle<'a, J> {
    type Item = Result<&'a J, JsonError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ArrayOrSingle::ArrayIter(iter) => iter.next(),
            ArrayOrSingle::SingleIter(iter) => iter.next().map(Ok),
        }
    }
}

fn iter_items<'a, O, J>(object: &'a O) -> impl Iterator<Item = &J>
where
    J: Json + 'a,
    O: JsonObject<Value = J>,
{
    object
        .get("items")
        .map(|items| {
            if let Some(array) = items.as_array() {
                ArrayOrSingle::ArrayIter(array.iter())
            } else {
                ArrayOrSingle::SingleIter(std::iter::once(items))
            }
        })
        .into_iter()
        .flatten()
        .filter_map(|r| r.ok())
}

fn subresources_of_with_ap_items_dependencies<'a, J: Json>(
    contents: &'a J,
    in_value: &'static [&'static str],
    in_subarray: &'static [&'static str],
    in_subvalues: &'static [&'static str],
) -> Box<dyn Iterator<Item = &'a J> + 'a> {
    let Some(object) = contents.as_object() else {
        return Box::new(std::iter::empty());
    };
    let values = iter_values!(object, in_value);
    let subarrays = iter_subarrays!(object, in_subarray);
    let subvalues = iter_subvalues!(object, in_subvalues);
    let items = iter_items(object);
    let dependencies = iter_subvalues!(object, &["dependencies"]).filter(|dep| dep.is_object());
    let additional = iter_subvalues!(object, &["additionalItems", "additionalProperties"])
        .filter(|items| items.is_object());
    Box::new(
        values
            .chain(subarrays)
            .chain(subvalues)
            .chain(items)
            .chain(dependencies)
            .chain(additional),
    )
}

#[derive(Copy, Clone)]
pub struct Draft4;

impl<J: Json> Specification<J> for Draft4 {
    fn name(&self) -> &'static str {
        "draft-04"
    }

    fn id_of<'a>(&self, document: &'a J) -> Option<&'a str> {
        legacy_id(document)
    }

    fn subresources_of<'a>(
        &self,
        document: &'a J,
    ) -> Box<dyn Iterator<Item = (JsonPath, &'a J)> + 'a> {
        todo!()
        //  subresources_of_with_ap_items_dependencies(
        //      document,
        //      &["not"],
        //      &["allOf", "anyOf", "oneOf"],
        //      &["definitions", "patternProperties", "properties"],
        //  )
    }

    fn anchors_in<'a>(&self, document: &'a J) -> AnchorIter<'a, J> {
        legacy_anchor_in_id(document)
    }

    fn maybe_in_subresource<'a>(
        &self,
        segments: &[Segment],
        resolver: &'a Resolver<'a, J>,
        subresource: SubResource<'a, J>,
    ) -> Resolver<'a, J> {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn Specification<J>> {
        Box::new(*self)
    }
}
