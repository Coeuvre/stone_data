use std::collections::HashMap;

pub type AttributeTypes = HashMap<&'static str, AttributeType>;
pub type Attributes = HashMap<&'static str, Attribute>;

macro_rules! attribute {
    (
        $($attribute_type:ident<$attribute:ty>,)*
    ) => {
        #[derive(Debug)]
        pub enum AttributeType {
            $($attribute_type,)*
        }

        impl AttributeType {
            pub fn to_attribute(&self) -> Attribute {
                match *self {
                    $(
                        AttributeType::$attribute_type => Attribute::$attribute_type(None),
                    )*
                }
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum Attribute {
            $($attribute_type(Option<$attribute>),)*
        }
    }
}

attribute! {
    Integer<i32>,
    Bool<bool>,
    Float<f32>,
    String<String>,
}
