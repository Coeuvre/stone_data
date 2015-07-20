use model::{Attributes, Model};

pub trait Adapter<T> {
    fn find<M>(&self, id: &T) -> Option<Attributes<T>> where M: Model<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use model::{Model, Attributes};
    use serializer::Serializer;

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
            let ty = M::ty();
            if ty != "user" {
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
    }

    pub struct SimpleSerializer;

    impl Serializer<i32> for SimpleSerializer {
        fn extract<M>(&self, attributes: Attributes<i32>) -> M where M: Model<i32> {
            let mut model = M::new();
            for (ref name, attribute) in attributes {
                model.set_attribute(name, attribute);
            }
            model
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
    }
}
