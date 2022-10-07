use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Deserialize, Serialize, Clone, Debug)]
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
