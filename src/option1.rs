use super::{IterableEnum, NextPrevEnum, Pages};

// non-generic iterator based

impl NextPrevEnum<Pages> for Pages {
    fn next(current: Pages) -> Pages {
        let mut it = Pages::iterator();

        let _ = it.find(|page| current == *page);
        let next = it.next();

        match next {
            Some(page) => page,
            None => {
                // iteration stopped at last element
                Pages::Home
            }
        }
    }

    fn previous(current: Pages) -> Pages {
        let it = Pages::iterator();
        let scan_it = it.scan(None, |_: &mut Option<Pages>, item| {
            if current == item {
                return None;
            }
            Some(item)
        });

        let last_value = scan_it.last();
        match last_value {
            Some(previous_page) => {
                return previous_page;
            }
            None => {
                // iteration stopped at first element
                return Pages::Glossary;
            }
        }
    }
}
