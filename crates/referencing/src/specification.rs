use core::fmt;

use crate::{
    anchors::Resolvable,
    path::{JsonPath, Segment},
    resources::SubResource,
    Resolver, Resource,
};
use jsonlike::prelude::*;

pub(crate) type AnchorIter<'a, J> = Box<dyn Iterator<Item = Box<dyn Resolvable<J>>> + 'a>;

pub trait Specification<D: Json> {
    fn name(&self) -> &'static str;
    fn id_of<'a>(&self, document: &'a D) -> Option<&'a str>;
    fn subresources_of<'a>(
        &self,
        document: &'a D,
    ) -> Box<dyn Iterator<Item = (JsonPath, &'a D)> + 'a>;
    fn anchors_in<'a>(&self, document: &'a D) -> AnchorIter<'a, D>;
    fn maybe_in_subresource<'a>(
        &self,
        segments: &[Segment],
        resolver: &'a Resolver<'a, D>,
        subresource: SubResource<'a, D>,
    ) -> Resolver<'a, D>;
    fn create_resource(&self, contents: D) -> Resource<D>
    where
        Self: Copy + 'static,
    {
        Resource::new(contents, self.boxed())
    }

    fn boxed(&self) -> Box<dyn Specification<D>>
    where
        Self: Copy + 'static,
    {
        Box::new(*self)
    }
    fn box_clone(&self) -> Box<dyn Specification<D>>;
}

impl<D: Json> fmt::Debug for dyn Specification<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Specification{{{}}}", self.name())
    }
}
