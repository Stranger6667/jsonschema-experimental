//! A lightweight library for working with JSON Pointers (RFC 6901).
//!
//! This crate provides a simple and efficient way to represent and build JSON Pointers.
//!
//! Note: This crate focuses on the representation and manipulation of JSON Pointers and does not
//! provide functionality for resolving JSON Pointers against JSON documents.
use core::{fmt, fmt::Write};

/// Owned JSON Pointer.
/// TODO: Maybe cache the string representation to avoid doing it during serde serialization?
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct JsonPointer(Vec<Segment>);

impl fmt::Display for JsonPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.0 {
            f.write_char('/')?;
            segment.fmt(f)?;
        }
        Ok(())
    }
}

/// A segment within a JSON pointer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Segment {
    /// Key within a JSON object.
    Key(Box<str>),
    /// Index within a JSON array.
    Index(usize),
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Key(value) => {
                for ch in value.chars() {
                    match ch {
                        '/' => f.write_str("~1")?,
                        '~' => f.write_str("~0")?,
                        _ => f.write_char(ch)?,
                    }
                }
                Ok(())
            }
            Segment::Index(idx) => f.write_fmt(format_args!("{}", idx)),
        }
    }
}

impl From<String> for Segment {
    #[inline]
    fn from(value: String) -> Self {
        Segment::Key(value.into_boxed_str())
    }
}

impl From<usize> for Segment {
    #[inline]
    fn from(value: usize) -> Self {
        Segment::Index(value)
    }
}

/// A node in a linked list representing a JSON pointer.
///
/// `JsonPointerNode` is used to build a JSON pointer incrementally during the JSON Schema validation process.
/// Each node contains a segment of the JSON pointer and a reference to its parent node, forming
/// a linked list.
///
/// The linked list representation allows for efficient traversal and manipulation of the JSON pointer
/// without the need for memory allocation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JsonPointerNode<'a> {
    pub(crate) segment: Segment,
    pub(crate) parent: Option<&'a JsonPointerNode<'a>>,
}

impl<'a> JsonPointerNode<'a> {
    #[inline]
    pub const fn new() -> Self {
        JsonPointerNode {
            // The value does not matter, it will never be used
            segment: Segment::Index(0),
            parent: None,
        }
    }

    #[inline]
    pub fn push(&'a self, segment: impl Into<Segment>) -> Self {
        JsonPointerNode {
            segment: segment.into(),
            parent: Some(self),
        }
    }

    pub fn to_vec(&'a self) -> Vec<Segment> {
        let mut buffer = Vec::new();
        let mut head = self;
        if head.parent.is_some() {
            buffer.push(head.segment.clone())
        }
        while let Some(next) = head.parent {
            head = next;
            if head.parent.is_some() {
                buffer.push(head.segment.clone());
            }
        }
        buffer.reverse();
        buffer
    }
}

impl From<JsonPointerNode<'_>> for JsonPointer {
    #[inline]
    fn from(node: JsonPointerNode<'_>) -> Self {
        JsonPointer(node.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_pointer_display() {
        let pointer = JsonPointer(vec![
            Segment::Key("foo".into()),
            Segment::Index(42),
            Segment::Key("bar".into()),
        ]);
        assert_eq!(pointer.to_string(), "/foo/42/bar");
    }

    #[test]
    fn test_segment_display() {
        let key_segment = Segment::Key("foo/bar~baz".into());
        assert_eq!(key_segment.to_string(), "foo~1bar~0baz");

        let index_segment = Segment::Index(42);
        assert_eq!(index_segment.to_string(), "42");
    }

    #[test]
    fn test_segment_from_string() {
        let segment = Segment::from("foo".to_string());
        assert_eq!(segment, Segment::Key("foo".into()));
    }

    #[test]
    fn test_segment_from_usize() {
        let segment = Segment::from(42_usize);
        assert_eq!(segment, Segment::Index(42));
    }

    #[test]
    fn test_json_pointer_node_push() {
        let root = JsonPointerNode::new();
        let node1 = root.push("foo".to_string());
        let node2 = node1.push(42);

        assert_eq!(node1.segment, Segment::Key("foo".into()));
        assert_eq!(node1.parent, Some(&root));

        assert_eq!(node2.segment, Segment::Index(42));
        assert_eq!(node2.parent, Some(&node1));
    }

    #[test]
    fn test_json_pointer_node_to_vec_empty() {
        assert_eq!(JsonPointerNode::new().to_vec(), vec![]);
    }

    #[test]
    fn test_json_pointer_node_to_vec() {
        let root = JsonPointerNode::new();
        let node1 = root.push("foo".to_string());
        let node2 = node1.push(42);

        let segments = node2.to_vec();
        assert_eq!(
            segments,
            vec![Segment::Key("foo".into()), Segment::Index(42)]
        );
    }

    #[test]
    fn test_json_pointer_from_json_pointer_node() {
        let root = JsonPointerNode::new();
        let node1 = root.push("foo".to_string());
        let node2 = node1.push(42);

        let json_pointer: JsonPointer = node2.into();
        assert_eq!(
            json_pointer,
            JsonPointer(vec![Segment::Key("foo".into()), Segment::Index(42)])
        );
    }
}
