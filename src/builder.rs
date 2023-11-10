use std::marker::PhantomData;

use crate::{compiler, drafts::Autodetect, error::BuildError, Draft, JsonSchema};
use jsonlike::Json;

pub(crate) async fn build<'a, 'b: 'a, D: Draft>(schema: &'b impl Json<'a>) -> JsonSchema<D> {
    // TODO: resolve
    compiler::compile::<D>(schema)
}

pub struct Builder<D: Draft = Autodetect> {
    _phantom: PhantomData<D>,
}

impl<D: Draft> Builder<D> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }

    pub async fn build<'a, 'b: 'a>(
        self,
        schema: &'b impl Json<'a>,
    ) -> Result<JsonSchema<D>, BuildError> {
        todo!()
    }
}

pub mod blocking {
    use std::marker::PhantomData;

    use jsonlike::Json;

    use crate::{drafts::Autodetect, error::BuildError, Draft, JsonSchema};

    pub struct Builder<D: Draft = Autodetect> {
        _phantom: PhantomData<D>,
    }

    impl<D: Draft> Builder<D> {
        pub fn new() -> Self {
            Self {
                _phantom: PhantomData::default(),
            }
        }

        pub fn build<'a, 'b: 'a>(
            self,
            schema: &'b impl Json<'a>,
        ) -> Result<JsonSchema<D>, BuildError> {
            todo!()
        }
    }
}
