use std::marker::PhantomData;

use model::Model;
use adapter::Adapter;
use serializer::Serializer;

pub struct Store<T> {
    phantom: PhantomData<T>,
}

impl<T> Store<T> {
    pub fn new() -> Store<T> {
        Store {
            phantom: PhantomData,
        }
    }

    pub fn find<A, S, M>(&self, adapter: &A, serializer: &S, id: &T) -> Option<M>
        where A: Adapter<T>, S: Serializer<T>, M: Model<T>
    {
        let attributes = match adapter.find::<M>(id) {
            Some(a) => a,
            None => return None,
        };

        Some(serializer.extract::<M>(attributes))
    }
}
