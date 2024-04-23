use std::ops::Deref;

#[derive(Debug)]
pub(crate) enum LeanCow<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for LeanCow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            LeanCow::Borrowed(b) => b,
            LeanCow::Owned(o) => o,
        }
    }
}
