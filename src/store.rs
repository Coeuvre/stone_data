use attribute::Attribute;
use model::{Model, Record};
use adapter::Adapter;
use serializer::Serializer;

pub struct Store;

impl Store {
    pub fn new() -> Store {
        Store
    }

    pub fn find<'a, A, S>(&'a mut self, model: &Model, adapter: &A, serializer: &'a S, id: &Attribute) -> Option<Record<'a>>
        where A: Adapter, S: Serializer
    {
        let attributes = match adapter.find(model, id) {
            Some(a) => a,
            None => return None,
        };

        Some(serializer.extract(model, attributes))
    }
    //
    // pub fn find_all<A, S, M>(&self, adapter: &A, serializer: &S) -> Vec<M>
    //     where A: Adapter<T>, S: Serializer<T>, M: Model<T> + Any
    // {
    //     let many_attributes = adapter.find_all::<M>();
    //     many_attributes.into_iter().map(|attributes| serializer.extract::<M>(attributes)).collect()
    // }
    //
    // pub fn find_many<A, S, M>(&self, adapter: &A, serializer: &S, ids: &[&T]) -> Vec<M>
    //     where A: Adapter<T>, S: Serializer<T>, M: Model<T> + Any
    // {
    //     let many_attributes = adapter.find_many::<M>(ids);
    //     many_attributes.into_iter().map(|attributes| serializer.extract::<M>(attributes)).collect()
    // }

    // pub fn push<M: Any>(&mut self, model: M) where M: Model<T> {
    //     let mut models = match self.cache.remove::<Vec<M>>() {
    //         None => Vec::new(),
    //         Some(models) => models,
    //     };
    //     models.push(model);
    //     self.cache.insert::<Vec<M>>(models);
    // }
    //
    // fn find_in_cache<M: Any>(&self, id: &T) -> Option<&M> where M: Model<T> {
    //     if let Some(ref models) = self.cache.get::<Vec<M>>() {
    //         let model = models.iter().find(|model| model.id() == Some(id));
    //         if model.is_some() {
    //             return model;
    //         }
    //     }
    //
    //     None
    // }
}
