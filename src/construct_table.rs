use anyhow::Result;
use crate::{spacer::Spacer, constant::Constant};
pub struct ConstructTable {

}
impl ConstructTable {
    pub fn new(sgrna_table: &str, dr_table: &str) -> Result<()> {
        let spacers = Self::parse_spacers(sgrna_table)?;
        let constants = Self::parse_constants(dr_table)?;
        println!("{:#?}", spacers);
        println!("{:#?}", constants);
        Ok(())
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
}
