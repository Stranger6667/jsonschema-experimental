pub type Uri = String;

pub type BoxedIter<'a, D> = Box<dyn Iterator<Item = &'a D> + 'a>;
