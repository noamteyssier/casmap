use serde::{Deserialize, Serialize};
use std::hash::Hash;

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
}

