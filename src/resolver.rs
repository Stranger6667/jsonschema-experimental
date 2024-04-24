use core::future::Future;

pub trait ReferenceResolver: Send + Sync {
    fn resolve_external(&self, url: &str) -> impl Future<Output = ()>
    where
        Self: Sized;
}

pub(crate) struct DefaultResolver;

impl ReferenceResolver for DefaultResolver {
    fn resolve_external(&self, url: &str) -> impl Future<Output = ()> {
        async {}
    }
}
