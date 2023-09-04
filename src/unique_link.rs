use std::sync::atomic::{AtomicUsize, Ordering};

fn to_lowercase_string(mut n: usize) -> String {
    let mut result = String::new();

    const BASE: usize = 'a' as usize;

    loop {
        let remainder = n % 26;
        result.push((BASE + remainder) as u8 as char);
        n /= 26;
        if n == 0 {
            break;
        }
        n -= 1;
    }

    result.chars().rev().collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniqueLink {
    value: usize,
}

impl UniqueLink {
    pub fn new() -> UniqueLink {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        UniqueLink {
            value: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn get(&self) -> String {
        to_lowercase_string(self.get_raw())
    }

    pub(crate) fn get_raw(&self) -> usize {
        self.value
    }
}

#[cfg(test)]
mod tests {
    mod unique_link {
        use crate::unique_link::UniqueLink;

        #[test]
        fn never_the_same_id() {
            let l1 = UniqueLink::new();
            let l2 = UniqueLink::new();

            assert_ne!(l1.get_raw(), l2.get_raw());
            assert_ne!(l1.get(), l2.get());
        }
    }

    mod to_lowercase_string {
        use proptest::prelude::*;

        use crate::unique_link::to_lowercase_string;

        proptest! {
            #[test]
            fn never_empty(value: usize) {
                let s = to_lowercase_string(value);
                assert!(!s.is_empty());
            }

            #[test]
            fn always_lowercase(value: usize) {
                let s = to_lowercase_string(value);

                for c in s.as_bytes() {
                    assert!(c.is_ascii_lowercase());
                }
            }
        }
    }
}
