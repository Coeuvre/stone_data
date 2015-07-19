use transform::Transform;

pub fn attr<T>() -> Box<Attr<T>> {
    Box::new(Attr {
        val: None
    })
}

pub struct Attr<T> {
    val: Option<T>,
}

impl<T> Attr<T> {
    pub fn new<F>(val: F) -> Attr<T> where F: Transform<T> {
        Attr {
            val: val.transform(),
        }
    }

    pub fn set<F>(&mut self, val: F) where F: Transform<T> {
        self.val = val.transform();
    }

    pub fn get(&self) -> Option<&T> {
        self.val.as_ref()
    }
}

#[test]
fn test() {
    // String -> String
    let attr = Attr::<String>::new("coeuvre".to_string());
    assert_eq!(attr.get(), Some(&"coeuvre".to_string()));

    // String -> i32
    let attr = Attr::<i32>::new("1".to_string());
    assert_eq!(attr.get(), Some(&1));
}
