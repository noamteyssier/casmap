use crate::{constant_table::ConstantTable, tuple_table::TupleTable, kmer::KmerIter};


#[derive(Debug)]
pub struct DescribeResult<'a> {
    r1: &'a str,
    r2: &'a str,
    r1_dr: Vec<String>,
    r1_spacers: Vec<String>,
    r2_dr: Vec<String>,
    r2_spacers: Vec<String>,
}
impl<'a> DescribeResult<'a> {
    pub fn new(r1: &'a str, r2: &'a str) -> Self {
        Self { r1, r2, r1_dr: Vec::new(), r1_spacers: Vec::new(), r2_dr: Vec::new(), r2_spacers: Vec::new() }
    }
    pub fn match_into(&mut self, spacer_table: &'a TupleTable, constant_table: &'a ConstantTable) {
        self.kmer_search(spacer_table, constant_table, self.r1, false);
        self.kmer_search(spacer_table, constant_table, self.r2, true);
    }
    fn kmer_search(&mut self, spacer_table: &'a TupleTable, constant_table: &'a ConstantTable, sequence: &'a str, reverse: bool) {
        let mut constants = KmerIter::new(sequence, constant_table.k())
            .filter_map(|x| constant_table.get_constant(x))
            .take(3)
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut spacers = KmerIter::new(sequence, spacer_table.k())
            .filter_map(|x| spacer_table.get_spacer(x))
            .take(3)
            .map(|x| x.sequence().to_string())
            .collect::<Vec<String>>();
        if reverse { 
            constants.reverse();
            spacers.reverse(); 
            self.r2_dr.append(&mut constants);
            self.r2_spacers.append(&mut spacers);
        } else {
            self.r1_dr.append(&mut constants);
            self.r1_spacers.append(&mut spacers);
        }
    }
    pub fn pprint(&self, idx: usize) -> String {
        let mut s = String::with_capacity(100);
        s.push_str(&format!("{}", idx));
        for idx in 0..3 {
            s.push('\t');
            if idx < self.r1_dr.len() {
                s.push_str(&self.r1_dr[idx]);
            }
        }
        for idx in 0..3 {
            s.push('\t');
            if idx < self.r1_spacers.len() {
                s.push_str(&self.r1_spacers[idx]);
            }
        }
        for idx in 0..3 {
            s.push('\t');
            if idx < self.r2_dr.len() {
                s.push_str(&self.r2_dr[idx]);
            }
        }
        for idx in 0..3 {
            s.push('\t');
            if idx < self.r2_spacers.len() {
                s.push_str(&self.r2_spacers[idx]);
            }
        }
        s
    }
}
