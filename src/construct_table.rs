use anyhow::Result;
use crate::{spacer::Spacer, constant::Constant, construct::Construct};

#[derive(Debug)]
pub struct ConstructTable {
    spacers: Vec<Spacer>,
    constants: Vec<Constant>,
    constructs: Vec<Construct>,
}
impl ConstructTable {
    pub fn new(sgrna_table: &str, dr_table: &str) -> Result<ConstructTable> {
        let spacers = Self::parse_spacers(sgrna_table)?;
        let constants = Self::parse_constants(dr_table)?;
        let constructs = Self::build_constructs(&spacers, &constants)?;
        Ok(Self {spacers, constants, constructs})
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
}
