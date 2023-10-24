use super::{IterableEnum, NextPrevEnum};

// generic iterator based

impl<T: IterableEnum<T> + PartialEq> NextPrevEnum<T> for T {
    fn next(current: T) -> T {
        let mut it = T::iterator();
        let _ = it.find(|item| current == *item);

        let next = it.next();
        match next {
            Some(next_item) => next_item,
            None => {
                // iteration stopped at last element
                T::first()
            }
        }
    }

    fn previous(current: T) -> T {
        let it = T::iterator();
        let scan_it = it.scan(None, |_: &mut Option<T>, item| {
            if current == item {
                return None;
            }
            Some(item)
        });

        let last_value = scan_it.last();
        match last_value {
            Some(previous_item) => previous_item,
            None => {
                // iteration stopped at first element
                T::last()
            }
        }
    }
}
