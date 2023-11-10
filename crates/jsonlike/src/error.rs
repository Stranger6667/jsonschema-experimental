use core::fmt;

#[derive(Debug)]
pub struct JsonError(Box<dyn std::error::Error>);

impl JsonError {
    pub fn new(error: Box<dyn std::error::Error>) -> JsonError {
        JsonError(error)
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.0)
    }
}
