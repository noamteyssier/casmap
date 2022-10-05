use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;

mod cli;
mod sgrna_table;
use cli::Cli;
use sgrna_table::VariableTable;

struct KmerIter<'a> {
    sequence: &'a str,
    kmer: usize,
    position: usize,
    len: usize,
}
impl<'a> KmerIter<'a> {
    pub fn new(sequence: &'a str, kmer: usize) -> Self {
        Self {
            sequence,
            kmer,
            position: 0,
            len: sequence.len(),
        }
    }
}
impl<'a> Iterator for KmerIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.kmer + self.position <= self.len {
            let substr = &self.sequence[self.position..self.position+self.kmer];
            self.position += 1;
            Some(substr)
        } else {
            None
        }
    }
}

struct SequenceResults<'a, 'b, 'c> {
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
        r1_variables
            .iter()
            .for_each(|x| {
                *self.variables.entry(x).or_default() += 1;
            });
        r2_variables
            .iter()
            .for_each(|x| {
                *self.variables.entry(x).or_default() += 1;
            });

    }
    fn kmer_search(&self, table: &'c VariableTable, sequence: &'c str) -> Vec<&'c str> {
        KmerIter::new(sequence, table.variable_length())
            .filter(|x| table.contains(x))
            .collect()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let table = VariableTable::from_file(&cli.sgrna_table)?;
    let r1_reader = fxread::initialize_reader(&cli.r1)?;
    let r2_reader = fxread::initialize_reader(&cli.r2)?;
    
    for (r1_bytes, r2_bytes) in r1_reader.zip(r2_reader) {
        let r1 = std::str::from_utf8(r1_bytes.seq())?;
        let r2 = r2_bytes.seq_rev_comp();
        let r2 = std::str::from_utf8(&r2)?;
        let mut results = SequenceResults::new(r1, r2);
        results.match_into(&table);
        println!("{:#?}", results.variables);
    }
    Ok(())
}
