use std::ops::Deref;

#[derive(Debug)]
pub(crate) enum MaybeOwned<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for MaybeOwned<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeOwned::Borrowed(b) => b,
            MaybeOwned::Owned(o) => o,
        }
    }
}
