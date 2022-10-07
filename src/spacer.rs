use serde::{Deserialize, Serialize};
use std::{hash::Hash, cmp::Ordering};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Spacer {
    // DNA sequence of the variable region
    sequence: String,

    // Construct identifier
    cid: usize,

    // Location identifer
    vid: usize,
}

impl Hash for Spacer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.sequence.as_bytes())
    }
}

impl Spacer {
    pub fn sequence(&self) -> &str {
        &self.sequence
    }
    pub fn ordering(&self, other: &Self) -> Ordering {
        match self.cid.cmp(&other.cid) {
            Ordering::Equal => {
                self.vid.cmp(&other.vid)
            },
            order => {
                order
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use std::cmp::Ordering;
    use super::Spacer;

    #[test]
    fn ordering_a() {
        let a = Spacer {
            sequence: String::new(),
            cid: 0,
            vid: 0,
        };
        let b = Spacer {
            sequence: String::new(),
            cid: 1,
            vid: 0,
        };
        assert_eq!(a.ordering(&b), Ordering::Less);
        assert_eq!(b.ordering(&a), Ordering::Greater);
    }

    #[test]
    fn ordering_b() {
        let a = Spacer {
            sequence: String::new(),
            cid: 0,
            vid: 0,
        };
        let b = Spacer {
            sequence: String::new(),
            cid: 0,
            vid: 0,
        };
        assert_eq!(a.ordering(&b), Ordering::Equal);
        assert_eq!(b.ordering(&a), Ordering::Equal);
    }

    #[test]
    fn ordering_c() {
        let a = Spacer {
            sequence: String::new(),
            cid: 0,
            vid: 1,
        };
        let b = Spacer {
            sequence: String::new(),
            cid: 0,
            vid: 0,
        };
        assert_eq!(a.ordering(&b), Ordering::Greater);
        assert_eq!(b.ordering(&a), Ordering::Less);
    }
}
