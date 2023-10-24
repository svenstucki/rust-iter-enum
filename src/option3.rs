use super::{IterableEnum, NextPrevEnum};

// generic array-of-items based

impl<T: IterableEnum<T> + PartialEq + Copy> NextPrevEnum<T> for T {
    fn next(current: T) -> T {
        let elems: Vec<T> = T::iterator().collect();
        let idx = elems.iter().position(|item| current == *item).unwrap();

        if idx + 1 >= elems.len() {
            elems[0]
        } else {
            elems[idx + 1]
        }
    }

    fn previous(current: T) -> T {
        let elems: Vec<T> = T::iterator().collect();
        let idx = elems.iter().position(|item| current == *item).unwrap();

        if idx == 0 {
            elems[elems.len() - 1]
        } else {
            elems[idx - 1]
        }
    }
}
