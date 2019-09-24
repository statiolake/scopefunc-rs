pub trait ScopeFunc: Sized {
    fn transform<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    fn modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
}

impl<T> ScopeFunc for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn get_string() -> String {
            "hello".to_string()
        }

        fn to_iter(s: String) -> impl Iterator<Item = char> {
            s.chars().collect::<Vec<_>>().into_iter()
        }

        assert_eq!(
            "hello",
            get_string().transform(|s| to_iter(s)).collect::<String>()
        );

        assert_eq!("HELLO", get_string().modify(|s| s.make_ascii_uppercase()));
    }
}
