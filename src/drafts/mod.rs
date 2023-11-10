pub mod draft04;
pub use draft04::Draft04;

pub trait Draft {}

pub struct Autodetect;

impl Draft for Autodetect {}
