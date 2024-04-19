pub mod draft04;

pub trait Draft {}

pub struct Autodetect;

impl Draft for Autodetect {}
