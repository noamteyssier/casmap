use std::{collections::HashSet, hash::Hash};

use anyhow::{Result, bail};
use serde::{Serialize, Deserialize};


#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct sgRNA {
    
    // DNA sequence of the variable region
    sequence: String,

    // Construct identifier
    cid: usize,

}

impl sgRNA {
    pub fn sequence(&self) -> &str {
        &self.sequence
    }
    pub fn cid(&self) -> usize {
        self.cid
    }
}

#[derive(Debug)]
pub struct VariableTable {
    records: HashSet<sgRNA>,
    variable_length: usize,
}
impl VariableTable {
    pub fn from_file(filepath: &str) -> Result<Self> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(false)
            .from_path(filepath)?;

        let records = reader
            .into_deserialize()
            .filter_map(|x| x.ok())
            .fold(HashSet::new(), |mut set, x| {
                set.insert(x);
                set
            });
        let variable_length = Self::calculate_variable_length(&records)?;
        Ok(Self{records, variable_length})
    }

    fn calculate_variable_length(seqset: &HashSet<sgRNA>) -> Result<usize> {
        let len_set = seqset
            .iter()
            .map(|x| x.sequence())
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
}
