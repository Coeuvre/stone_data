use std::collections::HashMap;

use attribute::{Attribute, AttributeType, Attributes, AttributeTypes};
use relationship::{Relationship, RelationshipType, Relationships, RelationshipTypes};

#[derive(Debug)]
pub struct Model {
    pub ty: &'static str,
    pub primary_key: &'static str,
    pub attributes: HashMap<&'static str, AttributeType>,
    pub relationships: HashMap<&'static str, RelationshipType>,
}

pub fn model(ty: &'static str, primary_key: &'static str) -> Model {
    Model {
        ty: ty,
        primary_key: primary_key,
        attributes: AttributeTypes::new(),
        relationships: RelationshipTypes::new(),
    }
}

impl Model {
    pub fn create<'a, 'b>(&'a self) -> Record<'b> {
        Record {
            id: Attribute::String(None),
            ty: self.ty,
            attributes: self.attributes.iter().map(|(name, ty)| (name.to_string(), ty.to_attribute())).collect(),
            relationships: self.relationships.iter().map(|(name, ty)| (name.to_string(), ty.to_relationship())).collect(),
        }
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

    pub fn get_one(&self, name: &str) -> Option<Option<&'a Record>> {
        if let Some(relationship) = self.relationships.get(name) {
            match *relationship {
                Relationship::BelongsTo(r) => Some(r),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_many(&self, name: &str) -> Option<&Vec<&Record>> {
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

#[cfg(test)]
mod tests {
    model! {
        User {
            type: user,

            attributes: {
                first_name: String,
                last_name: String,
            },

            relationships: {
                orders: HasMany<Order>,
            },
        },

        Order {
            type: order,

            attributes: {
                price: Float,
            },

            relationships: {
                customer: BelongsTo<User>,
            },
        }
    }

    #[test]
    fn test() {
        let user = User.create();
        assert!(user.get("first_name").is_some());
        assert!(user.get("other").is_none());
    }
}
