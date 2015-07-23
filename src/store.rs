use adapter::Adapter;
use attribute::Attribute;
use model::{Model, Record};
use query::Query;
use serializer::Serializer;

pub struct Store;

impl Store {
    pub fn new() -> Store {
        Store
    }

    pub fn query<'a, A, S>(&'a mut self, model: &Model, adapter: &A, serializer: &'a S, query: &Query) -> Option<Vec<Record<'a>>>
        where A: Adapter, S: Serializer
    {
        let attributes = match adapter.query(&query) {
            Some(a) => a,
            None => return None,
        };
        Some(attributes.into_iter().map(|attributes| serializer.extract(model, attributes)).collect())
    }

    pub fn find<'a, A, S>(&'a mut self, model: &Model, adapter: &A, serializer: &'a S, id: &Attribute) -> Option<Record<'a>>
        where A: Adapter, S: Serializer
    {
        let query = Query::table(model.ty).filter(model.primary_key).eq(id).limit(1);
        let attributes = match adapter.query(&query) {
            Some(a) => a,
            None => return None,
        };

        let attributes = attributes.into_iter().next().unwrap();
        Some(serializer.extract(model, attributes))
    }

    pub fn find_by<'a, A, S>(&'a mut self, model: &Model, adapter: &A, serializer: &'a S, name: &str, filter: &Attribute) -> Option<Vec<Record<'a>>>
        where A: Adapter, S: Serializer
    {
        let query = Query::table(model.ty).filter(name).eq(filter);
        let attributes = match adapter.query(&query) {
            Some(a) => a,
            None => return None,
        };
        Some(attributes.into_iter().map(|attributes| serializer.extract(model, attributes)).collect())
    }
}
