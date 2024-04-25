use core::future::Future;

use jsonlike::Json;

use crate::BuildResult;

pub trait ReferenceResolver<J: Json>: Send + Sync {
    fn resolve_external(&self, url: &str) -> impl Future<Output = BuildResult<J>>
    where
        Self: Sized;
}

pub(crate) struct DefaultResolver;

impl<J: Json> ReferenceResolver<J> for DefaultResolver {
    async fn resolve_external(&self, url: &str) -> BuildResult<J> {
        Ok(J::from_str("{}")?)
    }
}
