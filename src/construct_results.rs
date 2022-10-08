use hashbrown::HashSet;

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
        let cid_set_r1 = self.kmer_search_r1(table, self.r1);
        let cid_set_r2 = self.kmer_search_r2(table, self.r2);

        self.cid = if cid_set_r1.is_none() || cid_set_r2.is_none() {
            None
        } else {
            let mut ix = cid_set_r1.unwrap().intersection(cid_set_r2.unwrap());
            if let Some(cid) = ix.next() {
                if let Some(_) = ix.next() {
                    panic!("Ambiguous R1/R2 Intersection discovered");
                }
                Some(*cid)
            } else {
                None
            }
        };
    }
    fn kmer_search_r1(&self, table: &'a ConstructTable, sequence: &str) -> Option<&HashSet<usize>> {
        for kmer in KmerIter::new(sequence, table.k()) {
            if let Some(cid_set) = table.r1_contains(kmer) {
                return Some(cid_set);
            }
        }
        None
    }
    fn kmer_search_r2(&self, table: &'a ConstructTable, sequence: &str) -> Option<&HashSet<usize>> {
        for kmer in KmerIter::new(sequence, table.k()) {
            if let Some(cid_set) = table.r2_contains(kmer) {
                return Some(cid_set);
            }
        }
        None
    }
}
