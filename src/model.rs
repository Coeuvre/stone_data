use std::collections::HashMap;

use attribute::{Attribute, AttributeType, Attributes, AttributeTypes};
use relationship::{Relationship, RelationshipType, Relationships, RelationshipTypes};
use query::Query;

pub type Model = &'static ModelDef;

#[derive(Debug)]
pub struct ModelDef {
    pub ty: &'static str,
    pub primary_key: &'static str,
    pub attributes: HashMap<&'static str, AttributeType>,
    pub relationships: HashMap<&'static str, RelationshipType>,
}

pub fn model(ty: &'static str, primary_key: &'static str) -> ModelDef {
    ModelDef {
        ty: ty,
        primary_key: primary_key,
        attributes: AttributeTypes::new(),
        relationships: RelationshipTypes::new(),
    }
}

impl ModelDef {
    pub fn create(&'static self) -> Record {
        Record {
            id: Attribute::String(None),
            ty: self.ty,
            attributes: self.attributes.iter().map(|(name, ty)| (name.to_string(), ty.to_attribute())).collect(),
            relationships: self.relationships.iter().map(|(name, ty)| (name.to_string(), ty.to_relationship())).collect(),
        }
    }

    pub fn find<'a>(&'static self, id: &'a Attribute) -> Query<'a> {
        Query::new(self).where_(self.primary_key).eq(id).limit(1)
    }

    pub fn find_by<'a>(&'static self, name: &'a str, filter: &'a Attribute) -> Query<'a> {
        Query::new(self).where_(name).eq(filter)
    }

    pub fn find_in<'a>(&'static self, name: &'a str, filters: Vec<&'a Attribute>) -> Query<'a> {
        Query::new(self).where_(name).in_(filters)
    }
}

#[derive(Debug)]
pub struct Record<'a> {
    pub id: Attribute,
    pub ty: &'static str,
    pub attributes: Attributes,
    pub relationships: Relationships<'a>,
}

impl<'a> Record<'a> {
    pub fn get(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    pub fn set(&mut self, name: &str, attribute: Attribute) {
        if let Some(a) = self.attributes.get_mut(name) {
            *a = attribute;
        }
    }

    pub fn get_one(&self, name: &str) -> Option<&Option<Record>> {
        if let Some(relationship) = self.relationships.get(name) {
            match *relationship {
                Relationship::HasOne(ref r) => Some(r),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_many(&self, name: &str) -> Option<&Vec<Record>> {
        if let Some(relationship) = self.relationships.get(name) {
            match *relationship {
                Relationship::HasMany(ref r) => Some(r),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RecordSet<'a> {
    records: Vec<Record<'a>>
}

impl<'a> RecordSet<'a> {
    pub fn new(records: Vec<Record<'a>>) -> RecordSet<'a> {
        RecordSet {
            records: records,
        }
    }

    pub fn first(&self) -> Option<&Record> {
        self.records.iter().next()
    }
}

#[cfg(test)]
mod tests {
    model! {
        User {
            type: "user",

            attributes: {
                "first_name": String,
                "last_name": String,
            },

            relationships: {
                "orders": HasMany<Order>,
            },
        },

        Order {
            type: "order",

            attributes: {
                "price": F32,
            },

            relationships: {
                "customer": BelongsTo<User>,
            },
        }
    }

    #[test]
    fn test() {
        let user_id = 1.into();
        User.find(&user_id);
    }
}
