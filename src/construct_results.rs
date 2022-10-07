use crate::{construct_table::ConstructTable, kmer::KmerIter};

#[derive(Debug)]
pub struct ConstructResults<'a> {
    r1: &'a str,
    r2: &'a str,
    cid: Option<usize>,
}
impl<'a> ConstructResults<'a> {
    pub fn new(r1: &'a str, r2: &'a str) -> Self {
        Self { r1, r2, cid: None }
    }
    pub fn cid(&self) -> Option<usize> {
        self.cid
    }
    pub fn match_into(&mut self, table: &ConstructTable) {
        self.cid = match self.kmer_search_r1(table, self.r1) {
            Some(cid_r1) => match self.kmer_search_r2(table, self.r2) {
                Some(cid_r2) => {
                    if cid_r1 == cid_r2 {
                        Some(cid_r1)
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        };
    }
    fn kmer_search_r1(&self, table: &ConstructTable, sequence: &str) -> Option<usize> {
        for kmer in KmerIter::new(sequence, table.k()) {
            if let Some(cid) = table.r1_contains(kmer) {
                return Some(*cid);
            }
        }
        None
    }
    fn kmer_search_r2(&self, table: &ConstructTable, sequence: &str) -> Option<usize> {
        for kmer in KmerIter::new(sequence, table.k()) {
            if let Some(cid) = table.r2_contains(kmer) {
                return Some(*cid);
            }
        }
        None
    }
}
