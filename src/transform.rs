pub trait Transform<T> {
    fn transform(self) -> Option<T>;
}

impl<T> Transform<T> for T {
    fn transform(self) -> Option<T> {
        Some(self)
    }
}

impl Transform<i32> for String {
    fn transform(self) -> Option<i32> {
        self.parse().ok()
    }
}

impl Transform<i64> for String {
    fn transform(self) -> Option<i64> {
        self.parse().ok()
    }
}

impl Transform<String> for i32 {
    fn transform(self) -> Option<String> {
        Some(format!("{}", self))
    }
}
