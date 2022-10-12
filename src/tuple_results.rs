use crate::{tuple_table::{TupleTable, SeqWrapper, SeqTuple}, kmer::KmerIter};

#[derive(Debug)]
pub struct TupleResults<'a> {
    r1: &'a str,
    r2: &'a str,
    cid: Option<usize>,
}
impl<'a> TupleResults<'a> {
    pub fn new(r1: &'a str, r2: &'a str) -> Self {
        Self { r1, r2, cid: None }
    }
    pub fn cid(&self) -> Option<usize> {
        self.cid
    }
    pub fn match_into(&mut self, table: &'a TupleTable) {
        let mut tuple = Vec::with_capacity(6);
        let mut tuple_r1 = Self::kmer_search(table, self.r1, false);
        let mut tuple_r2 = Self::kmer_search(table, self.r2, true);
        tuple.append(&mut tuple_r1);
        tuple.append(&mut tuple_r2);
        if tuple.len() == 6 {
            let tuple = Self::build_tuple(&tuple);
            self.cid = table.get_tuple(&tuple);
       }
    }
    fn build_tuple(tuple: &[SeqWrapper]) -> SeqTuple {
        (
            tuple[0].clone(),
            tuple[1].clone(),
            tuple[2].clone(),
            tuple[3].clone(),
            tuple[4].clone(),
            tuple[5].clone(),
        )
    }
    fn kmer_search(table: &'a TupleTable, sequence: &'a str, reverse: bool) -> Vec<SeqWrapper> {
        let mut spacers = KmerIter::new(sequence, table.k())
            .filter_map(|x| table.get_spacer(&x))
            .take(3)
            .collect::<Vec<SeqWrapper>>();
        if reverse { spacers.reverse(); }
        spacers
    }
}
