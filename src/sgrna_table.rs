use anyhow::Result;
use serde::{Serialize, Deserialize};


#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct sgRNA {
    
    // DNA sequence of the variable region
    sequence: String,

    // Construct identifier
    cid: usize,

}

#[derive(Debug)]
pub struct VariableTable {
    records: Vec<sgRNA>,
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
            .collect();

        Ok(Self{records})

    }
}
