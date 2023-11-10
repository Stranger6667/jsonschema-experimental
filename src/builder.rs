use std::marker::PhantomData;

use crate::{compiler, drafts::Autodetect, error::BuildError, Draft, JsonSchemaValidator};
use jsonlike::Json;

pub(crate) async fn build<'a, 'b: 'a, D: Draft>(
    schema: &'b impl Json<'a>,
) -> JsonSchemaValidator<D> {
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
    ) -> Result<JsonSchemaValidator<D>, BuildError> {
        todo!()
    }
}

pub mod blocking {
    use std::marker::PhantomData;

    use jsonlike::Json;

    use crate::{drafts::Autodetect, error::BuildError, Draft, JsonSchemaValidator};

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
        ) -> Result<JsonSchemaValidator<D>, BuildError> {
            todo!()
        }
    }
}
