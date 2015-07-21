use attribute::{Attribute, Attributes};
use model::Model;

#[cfg(feature="postgres-adapter")]
pub mod postgres_adapter;

pub trait Adapter {
    fn find(&self, model: &Model, id: &Attribute) -> Option<Attributes>;
    fn find_all(&self, model: &Model) -> Vec<Attributes>;
    fn find_many(&self, model: &Model, ids: &[&Attribute]) -> Vec<Attributes>;
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use model::{Model, Attributes};
    use serializer::{Serializer, SimpleSerializer};

    pub struct User {
        attributes: Attributes<i32>,
    }

    impl Model<i32> for User {
        fn new() -> User {
            let mut attributes = Attributes::new();
            attributes.insert("name".to_string(), 1);
            User {
                attributes: attributes,
            }
        }

        fn ty() -> &'static str {
            "user"
        }

        fn id(&self) -> Option<&i32> {
            None
        }

        fn attributes(&self) -> &Attributes<i32> {
            &self.attributes
        }

        fn attributes_mut(&mut self) -> &mut Attributes<i32> {
            &mut self.attributes
        }
    }

    pub struct MemoryAdapter {
        usernames: HashMap<i32, i32>,
    }

    impl MemoryAdapter {
        pub fn new() -> MemoryAdapter {
            let mut usernames = HashMap::new();
            usernames.insert(1, 1);
            usernames.insert(2, 2);
            usernames.insert(3, 3);
            MemoryAdapter {
                usernames: usernames,
            }
        }
    }

    impl Adapter<i32> for MemoryAdapter {
        fn find<M>(&self, id: &i32) -> Option<Attributes<i32>> where M: Model<i32> {
            if M::ty() != "user" {
                return None;
            }

            let username = match self.usernames.get(id) {
                Some(username) => username,
                None => return None,
            };

            let mut attributes = Attributes::new();
            attributes.insert("name".to_string(), username.clone());
            Some(attributes)
        }

        fn find_all<M>(&self) -> Vec<Attributes<i32>> where M: Model<i32> {
            let many_attributes = vec![];
            if M::ty() != "user" {
                return many_attributes;
            }

            self.usernames.iter().map(|(_, username)| {
                let mut attributes = Attributes::new();
                attributes.insert("name".to_string(), username.clone());
                attributes
            }).collect()
        }

        fn find_many<M>(&self, ids: &[&i32]) -> Vec<Attributes<i32>> where M: Model<i32> {
            let many_attributes = vec![];
            if M::ty() != "user" {
                return many_attributes;
            }

            self.usernames.iter().filter(|&(&id, _)| {
                ids.iter().find(|id_to_find| (***id_to_find) == id).is_some()
            }).map(|(_, username)| {
                let mut attributes = Attributes::new();
                attributes.insert("name".to_string(), username.clone());
                attributes
            }).collect()
        }
    }

    #[test]
    fn test() {
        let adapter = MemoryAdapter::new();
        let serializer = SimpleSerializer;

        let attributes = adapter.find::<User>(&1).unwrap();
        assert_eq!(attributes.get("name"), Some(&1));
        let user = serializer.extract::<User>(attributes);
        assert_eq!(user.get_attribute("name"), Some(&1));

        let attributes = adapter.find::<User>(&10);
        assert!(attributes.is_none());

        assert_eq!(adapter.find_all::<User>().len(), 3);
        assert_eq!(adapter.find_many::<User>(&[&1, &2]).len(), 2);
        assert_eq!(adapter.find_many::<User>(&[&1, &5]).len(), 1);
        assert_eq!(adapter.find_many::<User>(&[&5]).len(), 0);
    }
}

*/
