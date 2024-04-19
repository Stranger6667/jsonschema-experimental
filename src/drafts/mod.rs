mod draft04;
mod draft07;
pub use draft04::Draft04;
pub use draft07::Draft07;

pub type Latest = Draft04;

pub trait Draft {
    fn new_boxed() -> Box<dyn Draft>
    where
        Self: Sized;
}

pub(crate) fn from_url(url: &str) -> Option<Box<dyn Draft>> {
    todo!()
}
