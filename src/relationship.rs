use std::collections::HashMap;

use model::{Model, Record};

pub type RelationshipTypes = HashMap<&'static str, RelationshipType>;
pub type Relationships<'a> = HashMap<&'static str, Relationship<'a>>;

#[derive(Debug)]
pub enum RelationshipType {
    BelongsTo(&'static Model),
    HasMany(&'static Model),
}

impl RelationshipType {
    pub fn to_relationship<'a, 'b>(&'a self) -> Relationship<'b> {
        match *self {
            RelationshipType::BelongsTo(_) => Relationship::BelongsTo(None),
            RelationshipType::HasMany(_) => Relationship::HasMany(vec![]),
        }
    }
}

#[derive(Debug)]
pub enum Relationship<'a> {
    BelongsTo(Option<&'a Record<'a>>),
    HasMany(Vec<&'a Record<'a>>),
}
