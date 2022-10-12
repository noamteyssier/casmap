use std::{rc::Rc, borrow::Borrow};
use anyhow::Result;
use disambiseq::{Disambiseq, SeqWrapper};
use hashbrown::HashMap;
use crate::spacer::Spacer;

/// Convenience alias for SeqWrapper 6-Mer Tuples
pub type SeqTuple = (
    SeqWrapper,
    SeqWrapper,
    SeqWrapper,
    SeqWrapper,
    SeqWrapper,
    SeqWrapper,
);

#[derive(Debug)]
pub struct TupleTable {
    sequences: HashMap<SeqWrapper, SeqWrapper>,
    tuple_map: HashMap<SeqTuple, usize>,
    k: usize,
}
impl TupleTable {
    pub fn from_file(filepath: &str) -> Result<Self> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(false)
            .from_path(filepath)?;

        let mut records = reader.deserialize().filter_map(|x| x.ok()).collect::<Vec<Spacer>>();
        records.sort_unstable_by(|a, b| a.ordering(&b));

        let mut sequences = HashMap::with_capacity(records.len());
        let mut tuple_map = HashMap::with_capacity(records.len());
        let mut dsq = Disambiseq::new();
        for chunks in records.chunks(6) {
            let cid = chunks[0].cid();
            let record_sequences = chunks
                .iter()
                .map(|x| Rc::new(x.sequence().to_string()))
                .map(|x| SeqWrapper(x))
                .collect::<Vec<SeqWrapper>>();

            let tuple = (
                record_sequences[0].clone(),
                record_sequences[1].clone(),
                record_sequences[2].clone(),
                record_sequences[3].clone(),
                record_sequences[4].clone(),
                record_sequences[5].clone()
                );
            tuple_map.insert(tuple, cid);

            record_sequences
                .iter()
                .for_each(|x| {
                    dsq.insert_with_reverse_complement(x.borrow());
                });


            for (k, v) in dsq.unambiguous().into_iter() {
                sequences.insert(k.clone(), v.clone());
            }
            for k in dsq.parents().into_iter() {
                sequences.insert(k.clone(), k.clone());
            }
        }
        let k = records[0].sequence().len();
        Ok(Self{sequences, tuple_map, k})
    }
    pub fn k(&self) -> usize {
        self.k
    }
    pub fn len(&self) -> usize {
        self.tuple_map.len()
    }
    pub fn get_spacer(&self, seq: &str) -> Option<SeqWrapper> {
        self.sequences.get(seq).map(|x| x.clone())
    }
    pub fn get_tuple(&self, tuple: &SeqTuple) -> Option<usize> {
        self.tuple_map.get(tuple).map(|x| *x)
    }
}
