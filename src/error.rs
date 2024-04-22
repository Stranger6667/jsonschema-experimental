/// An error that occured during the building of a validator.
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}

#[derive(Clone, Debug)]
enum BuildErrorKind {}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl std::error::Error for BuildError {}

/// An error that occured during JSON Schema validation.
#[derive(Clone, Debug)]
pub struct ValidationError(Box<ValidationErrorKind>);

#[derive(Clone, Debug)]
pub enum ValidationErrorKind {}

impl core::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl std::error::Error for ValidationError {}
