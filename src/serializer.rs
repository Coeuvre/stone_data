use model::{Attributes, Model};

pub trait Serializer<T> {
    fn extract<M>(&self, attributes: Attributes<T>) -> M where M: Model<T>;
}
