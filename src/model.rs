use std::any::Any;
use std::collections::HashMap;

use attr::Attr;

pub type Attrs = HashMap<String, Box<Any>>;

pub trait Model {
    fn attrs(&self) -> &Attrs;
    fn attrs_mut(&mut self) -> &mut Attrs;

    fn set<T: Any>(&mut self, name: &str, attr: Attr<T>) {
        let attrs = self.attrs_mut();

        if let Some(a) = attrs.get_mut(name) {
            if let Some(a) = a.downcast_mut::<Attr<T>>() {
                *a = attr;
            } else {
                // ERROR: not correct attribute type
            }
        }
    }

    fn get<T: Any>(&self, name: &str) -> Option<&Attr<T>> {
        let attrs = self.attrs();

        if let Some(attr) = attrs.get(name) {
            if let Some(attr) = attr.downcast_ref::<Attr<T>>() {
                return Some(attr)
            }
        }

        None
    }

    // fn save();
    // fn create();
    // fn delete();
}

#[cfg(test)]
mod tests {
    use super::*;

    use attr::{attr, Attr};

    pub struct User {
        attrs: Attrs,
    }

    impl User {
        pub fn new() -> User {
            let mut attrs = Attrs::new();

            attrs.insert("id".to_string(), attr::<i32>());
            attrs.insert("name".to_string(), attr::<String>());

            User {
                attrs: attrs,
            }
        }
    }

    impl Model for User {
        fn attrs(&self) -> &Attrs {
            &self.attrs
        }

        fn attrs_mut(&mut self) -> &mut Attrs {
            &mut self.attrs
        }
    }

    #[test]
    fn test() {
        let mut user = User::new();

        user.set("id", Attr::<i32>::new("1".to_string()));
        assert_eq!(user.get("id").unwrap().get(), Some(&1));

        user.set("name", Attr::<String>::new("coeuvre".to_string()));
        assert_eq!(user.get("name").unwrap().get(), Some(&"coeuvre".to_string()));
    }
}
