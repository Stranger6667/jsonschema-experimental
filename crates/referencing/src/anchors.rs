use core::fmt;
use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::{Hash, Hasher},
};

use jsonlike::Json;
use url::Url;

use crate::{error::ReferencingError, path::JsonPath, Resolved, Resolver};

pub trait Resolvable<D: Json> {
    fn name(&self) -> &str;
    fn resolve<'a>(&self, resolver: Resolver<'a, D>) -> Result<Resolved<'a, D>, ReferencingError>;
}

impl<D: Json> fmt::Debug for dyn Resolvable<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Resolvable{{{}}}", self.name())
    }
}

#[derive(Debug)]
pub struct Anchor {
    name: String,
    path: JsonPath,
}

impl Anchor {
    pub(crate) fn new(name: String, path: JsonPath) -> Anchor {
        Anchor { name, path }
    }
}

impl<D: Json> Resolvable<D> for Anchor {
    fn name(&self) -> &str {
        &self.name
    }

    /// Return the resource for this anchor.
    fn resolve<'a>(&self, resolver: Resolver<'a, D>) -> Result<Resolved<'a, D>, ReferencingError> {
        // TODO: This should receive a resource here
        Ok(Resolved {
            contents: todo!("Return a reference here"),
            resolver,
        })
    }
}

trait AnchorKey {
    fn uri(&self) -> &Url;
    fn name(&self) -> &str;
}

impl<'a> Borrow<dyn AnchorKey + 'a> for (Url, String) {
    fn borrow(&self) -> &(dyn AnchorKey + 'a) {
        self
    }
}

impl Hash for dyn AnchorKey + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uri().hash(state);
        self.name().hash(state);
    }
}

impl PartialEq for dyn AnchorKey + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.uri() == other.uri() && self.name() == other.name()
    }
}

impl Eq for dyn AnchorKey + '_ {}

impl AnchorKey for (Url, String) {
    fn uri(&self) -> &Url {
        &self.0
    }
    fn name(&self) -> &str {
        &self.1
    }
}

impl AnchorKey for (&Url, &str) {
    fn uri(&self) -> &Url {
        self.0
    }
    fn name(&self) -> &str {
        self.1
    }
}

#[derive(Debug)]
pub(crate) struct AnchorMap<D: Json> {
    anchors: HashMap<(Url, String), Box<dyn Resolvable<D>>>,
}

impl<D: Json> Default for AnchorMap<D> {
    fn default() -> Self {
        Self {
            anchors: HashMap::default(),
        }
    }
}

impl<D: Json> AnchorMap<D> {
    pub(crate) fn insert(&mut self, uri: Url, name: String, anchor: Box<dyn Resolvable<D>>) {
        self.anchors.insert((uri, name), anchor);
    }
    pub(crate) fn get<'a>(&'a self, uri: &Url, name: &str) -> Option<Box<dyn Resolvable<D> + 'a>> {
        self.anchors
            .get(&(uri, name) as &dyn AnchorKey)
            .map(|boxed| todo!())
    }
}
