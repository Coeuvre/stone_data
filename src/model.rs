use std::collections::HashMap;

pub type Attributes<T> = HashMap<String, T>;

pub trait Model<T> {
    fn new() -> Self;

    fn ty() -> &'static str;
    fn id(&self) -> Option<&T>;
    fn attributes(&self) -> &Attributes<T>;
    fn attributes_mut(&mut self) -> &mut Attributes<T>;

    fn get_attribute(&self, name: &str) -> Option<&T> {
        self.attributes().get(name)
    }

    fn set_attribute(&mut self, name: &str, attribute: T) {
        match self.attributes_mut().get_mut(name) {
            Some(a) => *a = attribute,
            None => {
                panic!(format!("attribute `{}` not exist", name));
            },
        };
    }

    // fn save();
    // fn create();
    // fn delete();
}
