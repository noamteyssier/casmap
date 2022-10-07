use crate::{kmer::KmerIter, spacer_table::SpacerTable};
use std::collections::HashMap;

pub struct SequenceResults<'a, 'b, 'c> {
    r1: &'a str,
    r2: &'b str,
    spacers: HashMap<&'c str, usize>,
}
impl<'a, 'b, 'c> SequenceResults<'a, 'b, 'c>
where
    'a: 'c,
    'b: 'c,
{
    pub fn new(r1: &'a str, r2: &'b str) -> Self {
        Self {
            r1,
            r2,
            spacers: HashMap::new(),
        }
    }
    pub fn match_into(&mut self, table: &'c SpacerTable) {
        let r1_spacers = self.kmer_search(table, self.r1);
        let r2_spacers = self.kmer_search(table, self.r2);
        r1_spacers.iter().for_each(|x| {
            *self.spacers.entry(x).or_default() += 1;
        });
        r2_spacers.iter().for_each(|x| {
            *self.spacers.entry(x).or_default() += 1;
        });
    }
    fn kmer_search(&self, table: &'c SpacerTable, sequence: &'c str) -> Vec<&'c str> {
        KmerIter::new(sequence, table.spacer_length())
            .filter_map(|x| table.contains(x))
            .map(|x| x.as_str())
            .collect()
    }
    pub fn spacers(&self) -> &HashMap<&'c str, usize> {
        &self.spacers
    }
}
