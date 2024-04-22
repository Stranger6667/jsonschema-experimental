pub trait ReferenceResolver: Send + Sync {}

pub(crate) struct DefaultResolver;

impl ReferenceResolver for DefaultResolver {}
