use crate::{spacer::Spacer, utils::reverse_complement};
use anyhow::{bail, Result};
use hashbrown::{HashMap, HashSet};

#[derive(Debug)]
pub struct SpacerTable {
    records: HashMap<String, String>,
    spacer_length: usize,
}
impl SpacerTable {
    pub fn from_file(filepath: &str) -> Result<Self> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(false)
            .from_path(filepath)?;

        let records = reader.into_deserialize().filter_map(|x| x.ok()).fold(
            HashMap::new(),
            |mut map, x: Spacer| {
                let seq = x.sequence().to_owned();
                let revcomp = reverse_complement(&seq);
                map.insert(seq.to_owned(), seq.to_owned());
                map.insert(revcomp, seq);
                map
            },
        );
        let spacer_length = Self::calculate_spacer_length(&records)?;
        Ok(Self {
            records,
            spacer_length,
        })
    }

    fn calculate_spacer_length(seqmap: &HashMap<String, String>) -> Result<usize> {
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
    pub fn spacer_length(&self) -> usize {
        self.spacer_length
    }
}
