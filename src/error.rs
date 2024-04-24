/// An error that occured during the building of a validator.
#[derive(Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}

#[derive(Debug)]
enum BuildErrorKind {
    Json(jsonlike::JsonError),
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            BuildErrorKind::Json(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            BuildErrorKind::Json(error) => Some(error),
        }
    }
}

impl From<jsonlike::JsonError> for BuildError {
    fn from(err: jsonlike::JsonError) -> Self {
        Self {
            kind: BuildErrorKind::Json(err),
        }
    }
}

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
