use core::fmt;

// TODO: small data optimization - maybe smallvec
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct JsonPath(Vec<Segment>);

impl fmt::Display for JsonPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter();
        if let Some(segment) = iter.next() {
            f.write_fmt(format_args!("{}", segment))?;
        } else {
            return f.write_str("<empty>");
        }
        for segment in iter {
            f.write_fmt(format_args!(" -> {}", segment))?;
        }
        Ok(())
    }
}

impl JsonPath {
    pub const fn new() -> JsonPath {
        JsonPath(Vec::new())
    }

    pub fn push(&mut self, segment: impl Into<Segment>) {
        self.0.push(segment.into());
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Segment> {
        self.0.iter()
    }
}

macro_rules! jsonpath {
    ($($element:expr),*) => {{
        // TODO: reserve enough space
        let mut path = crate::path::JsonPath::new();
        $(
            path.push($element);
        )*
        path
    }};
}

pub(crate) use jsonpath;
// TODO: Have a borrowed version of it

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Segment {
    Key(Box<str>),
    Index(usize),
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Key(key) => f.write_str(key),
            Segment::Index(idx) => f.write_fmt(format_args!("{}", idx)),
        }
    }
}

impl From<usize> for Segment {
    fn from(value: usize) -> Self {
        Segment::Index(value)
    }
}
impl From<String> for Segment {
    fn from(value: String) -> Self {
        Segment::Key(value.into_boxed_str())
    }
}
impl<'a> From<&'a str> for Segment {
    fn from(value: &'a str) -> Self {
        Segment::Key(value.to_owned().into_boxed_str())
    }
}
