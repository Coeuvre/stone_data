use attribute::Attributes;
use model::{Model, Record};

pub trait Serializer {
    fn extract(&self, model: &Model, attributes: Attributes) -> Record;
}

pub struct SimpleSerializer;

impl Serializer for SimpleSerializer {
    fn extract(&self, model: &Model, attributes: Attributes) -> Record {
        let mut record = model.create();
        for (name, attribute) in attributes {
            if name == model.primary_key {
                record.id = attribute;
            } else {
                record.set(&name, attribute);
            }
        }
        record
    }
}
