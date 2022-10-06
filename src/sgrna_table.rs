use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::utils::reverse_complement;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct sgRNA {
    // DNA sequence of the variable region
    sequence: String,

    // Construct identifier
    cid: usize,
}

impl Hash for sgRNA {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.sequence.as_bytes())
    }
}

impl sgRNA {
    pub fn sequence(&self) -> &str {
        &self.sequence
    }
}

#[derive(Debug)]
pub struct VariableTable {
    records: HashMap<String, String>,
    variable_length: usize,
}
impl VariableTable {
    pub fn from_file(filepath: &str) -> Result<Self> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(false)
            .from_path(filepath)?;

        let records = reader.into_deserialize().filter_map(|x| x.ok()).fold(
            HashMap::new(),
            |mut map, x: sgRNA| {
                let seq = x.sequence().to_owned();
                let revcomp = reverse_complement(&seq);
                map.insert(seq.to_owned(), seq.to_owned());
                map.insert(revcomp, seq);
                map
            },
        );
        let variable_length = Self::calculate_variable_length(&records)?;
        Ok(Self {
            records,
            variable_length,
        })
    }

    fn calculate_variable_length(seqmap: &HashMap<String, String>) -> Result<usize> {
        let len_set = seqmap
            .keys()
            .map(|x| x.len())
            .fold(HashSet::new(), |mut set, x| {
                set.insert(x);
                set
            });
        if len_set.len() == 1 {
            Ok(*len_set.iter().next().unwrap())
        } else if len_set.len() == 0 {
            bail!("No records found in sgRNA table")
        } else {
            bail!("Multiple sequence lengths found in sgRNA table")
        }
    }
    pub fn contains(&self, seq: &str) -> Option<&String> {
        self.records.get(seq)
    }
    pub fn variable_length(&self) -> usize {
        self.variable_length
    }
}
