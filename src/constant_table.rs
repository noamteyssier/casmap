use anyhow::{Result, bail};
use hashbrown::HashMap;
use crate::{constant::Constant, utils::reverse_complement};

#[derive(Debug)]
pub struct ConstantTable {
    records: HashMap<String, String>,
    k: usize,
}
impl ConstantTable {
    pub fn from_file(filepath: &str) -> Result<Self> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(false)
            .from_path(filepath)?;

        let records = reader.into_deserialize().filter_map(|x| x.ok()).fold(
            HashMap::new(),
            |mut map, x: Constant| {
                let seq = x.sequence().to_owned();
                let revcomp = reverse_complement(&seq);
                map.insert(seq.to_owned(), seq.to_owned());
                map.insert(revcomp, seq);
                map
            },
        );
        let k = match records.keys().next() {
            Some(x) => x.len(),
            None => bail!("No constants found in table")
        };

        Ok(Self { records, k})
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn get_constant(&self, sequence: &str) -> Option<&String> {
        self.records.get(sequence)
    }
}
