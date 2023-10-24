#![feature(return_position_impl_trait_in_trait)]

// mod option1;
// mod option2;
mod option3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pages {
    Home,
    Introduction,
    Glossary,
}

pub trait IterableEnum<T> {
    fn iterator() -> impl Iterator<Item = T>;
    fn first() -> T;
    fn last() -> T;
}

impl IterableEnum<Pages> for Pages {
    fn iterator() -> impl Iterator<Item = Pages> {
        use Pages::*;
        [Home, Introduction, Glossary].iter().copied()
    }
    fn first() -> Pages {
        Pages::Home
    }
    fn last() -> Pages {
        Pages::Glossary
    }
}

pub trait NextPrevEnum<T: IterableEnum<T>> {
    fn next(current: T) -> T;
    fn previous(current: T) -> T;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_works() {
        let page = Pages::Home;
        assert_eq!(Pages::next(page), Pages::Introduction);
        assert_eq!(Pages::previous(page), Pages::Glossary);
    }

    #[test]
    fn intro_works() {
        let page = Pages::Introduction;
        assert_eq!(Pages::next(page), Pages::Glossary);
        assert_eq!(Pages::previous(page), Pages::Home);
    }

    #[test]
    fn glossary_works() {
        let page = Pages::Glossary;
        assert_eq!(Pages::next(page), Pages::Home);
        assert_eq!(Pages::previous(page), Pages::Introduction);
    }
}
