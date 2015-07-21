use std::collections::HashMap;

pub type AttributeTypes = HashMap<&'static str, AttributeType>;
pub type Attributes = HashMap<String, Attribute>;

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

        #[derive(Clone, Debug, PartialEq)]
        pub enum Attribute {
            $($attribute_type(Option<$attribute>),)*
        }
    }
}

attribute! {
    Bool<bool>,
    I8<i8>,
    I16<i16>,
    I32<i32>,
    I64<i64>,
    F32<f32>,
    F64<f64>,
    String<String>,
}
