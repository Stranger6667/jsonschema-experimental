use core::fmt;
use url::Url;

#[derive(Debug)]
pub enum ReferencingError {
    /// A reference was unresolvable.
    Unresolvable { reference: String },
    /// A JSON Pointer leads to a part of a document that does not exist.
    PointerToNowhere { reference: String },
    /// An anchor does not exist within a particular resource.
    NoSuchAnchor { reference: Url, anchor: String },
    /// An anchor which could never exist in a resource was dereferenced.
    InvalidAnchor { reference: Url, anchor: String },
}

impl ReferencingError {
    pub(crate) fn unresolvable(reference: impl Into<String>) -> ReferencingError {
        ReferencingError::Unresolvable {
            reference: reference.into(),
        }
    }
    pub(crate) fn pointer_to_nowhere(reference: impl Into<String>) -> ReferencingError {
        ReferencingError::PointerToNowhere {
            reference: reference.into(),
        }
    }
}

impl fmt::Display for ReferencingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferencingError::Unresolvable { reference } => {
                f.write_fmt(format_args!("'{reference}' does not exist"))
            }
            ReferencingError::PointerToNowhere { reference } => {
                f.write_fmt(format_args!("'{reference}' does not exist"))
            }
            ReferencingError::NoSuchAnchor { reference, anchor } => {
                f.write_fmt(format_args!("'{anchor}' does not exist"))
            }
            ReferencingError::InvalidAnchor { reference, anchor } => {
                f.write_fmt(format_args!("'{anchor}' is not a valid anchor"))
            }
        }
    }
}

impl std::error::Error for ReferencingError {}
