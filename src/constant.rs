use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Constant {
    sequence: String,
    id: usize,
}
impl Constant {
    pub fn sequence(&self) -> &str {
        &self.sequence
    }
    pub fn ordering(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
