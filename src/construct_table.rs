use std::collections::{HashMap, HashSet};

use anyhow::{Result, bail};
use crate::{spacer::Spacer, constant::Constant, construct::Construct};

#[derive(Debug)]
#[allow(unused)]
pub struct ConstructTable {
    spacers: Vec<Spacer>,
    constants: Vec<Constant>,
    constructs: Vec<Construct>,
    r1_table: HashMap<String, usize>,
    r2_table: HashMap<String, usize>,
    k: usize,
}
impl ConstructTable {
    pub fn new(sgrna_table: &str, dr_table: &str) -> Result<ConstructTable> {
        let spacers = Self::parse_spacers(sgrna_table)?;
        let constants = Self::parse_constants(dr_table)?;
        let constructs = Self::build_constructs(&spacers, &constants)?;
        let (r1_table, r2_table) = Self::build_hashtables(&constructs);
        let k = Self::half_construct_size(&r1_table, &r2_table)?;
        Ok(Self {spacers, constants, constructs, r1_table, r2_table, k})
    }
    fn parse_spacers(sgrna_table: &str) -> Result<Vec<Spacer>> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(false)
            .from_path(sgrna_table)?;

        let mut records = reader
            .into_deserialize()
            .filter_map(|x| x.ok())
            .collect::<Vec<Spacer>>();
        records.sort_unstable_by(|a, b| a.ordering(&b));
        Ok(records)
    }

    fn parse_constants(dr_table: &str) -> Result<Vec<Constant>> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(false)
            .from_path(dr_table)?;

        let mut records = reader
            .into_deserialize()
            .filter_map(|x| x.ok())
            .collect::<Vec<Constant>>();
        records.sort_unstable_by(|a, b| a.ordering(&b));
        Ok(records)
    }
    fn build_constructs(spacers: &[Spacer], constants: &[Constant]) -> Result<Vec<Construct>> {
        let mut constructs = Vec::new();
        for (cid, chunk) in spacers.chunks(6).enumerate() {
            constructs.push(Construct::new(chunk, constants, cid)?);
        }
        Ok(constructs)
    }
    fn build_hashtables(constructs: &[Construct]) -> (HashMap<String, usize>, HashMap<String, usize>) {
        constructs
            .iter()
            .fold(
                (HashMap::new(), HashMap::new()), |(mut r1_map, mut r2_map), c| {
                    r1_map.insert(c.r1(), c.cid());
                    r2_map.insert(c.r2(), c.cid());
                    (r1_map, r2_map)
                })
    }
    fn half_construct_size(r1_map: &HashMap<String, usize>, r2_map: &HashMap<String, usize>) -> Result<usize> {
        let r1_size_vec = r1_map.keys().map(|x| x.len()).collect::<HashSet<usize>>();
        let r2_size_vec = r2_map.keys().map(|x| x.len()).collect::<HashSet<usize>>();

        if r1_size_vec.len() != r2_size_vec.len() {
            bail!("Unequal sized R1 and R2 found")
        } else if r1_size_vec.len() != 1 {
            bail!("Unequal sized spacers or constructs found")
        } else {
            Ok(*r1_size_vec.iter().next().unwrap())
        }
    }
    pub fn k(&self) -> usize {
        self.k
    }
    pub fn r1_contains(&self, seq: &str) -> Option<&usize> {
        self.r1_table.get(seq)
    }
    pub fn r2_contains(&self, seq: &str) -> Option<&usize> {
        self.r2_table.get(seq)
    }
}
