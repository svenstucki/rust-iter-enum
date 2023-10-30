use super::{IterableEnum, NextPrevEnum};

// generic double-ended iterator based

impl<T: IterableEnum<T> + PartialEq> NextPrevEnum<T> for T {
    fn next(current: T) -> T {
        let mut it = T::iterator();
        let _ = it.find(|item| current == *item);

        let next = it.next();
        match next {
            Some(next_item) => next_item,
            None => T::first(),
        }
    }

    fn previous(current: T) -> T {
        let mut it = T::iterator().rev();
        let _ = it.find(|item| current == *item);

        let previous = it.next();
        match previous {
            Some(previous_item) => previous_item,
            None => T::last(),
        }
    }
}
