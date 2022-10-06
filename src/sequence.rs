use crate::{kmer::KmerIter, sgrna_table::VariableTable};
use std::collections::HashMap;

pub struct SequenceResults<'a, 'b, 'c> {
    r1: &'a str,
    r2: &'b str,
    variables: HashMap<&'c str, usize>,
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
            variables: HashMap::new(),
        }
    }
    pub fn match_into(&mut self, table: &'c VariableTable) {
        let r1_variables = self.kmer_search(table, self.r1);
        let r2_variables = self.kmer_search(table, self.r2);
        r1_variables.iter().for_each(|x| {
            *self.variables.entry(x).or_default() += 1;
        });
        r2_variables.iter().for_each(|x| {
            *self.variables.entry(x).or_default() += 1;
        });
    }
    fn kmer_search(&self, table: &'c VariableTable, sequence: &'c str) -> Vec<&'c str> {
        KmerIter::new(sequence, table.variable_length())
            .filter_map(|x| table.contains(x))
            .map(|x| x.as_str())
            .collect()
    }
    pub fn variables(&self) -> &HashMap<&'c str, usize> {
        &self.variables
    }
}
