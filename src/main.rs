use anyhow::Result;
use clap::Parser;

mod cli;
mod kmer;
mod sgrna_table;
mod sequence;
mod utils;
use cli::Cli;
use sgrna_table::VariableTable;
use sequence::SequenceResults;



fn main() -> Result<()> {
    let cli = Cli::parse();
    let table = VariableTable::from_file(&cli.sgrna_table)?;
    let r1_reader = fxread::initialize_reader(&cli.r1)?;
    let r2_reader = fxread::initialize_reader(&cli.r2)?;
    
    for (r1_bytes, r2_bytes) in r1_reader.zip(r2_reader) {
        let r1 = std::str::from_utf8(r1_bytes.seq())?;
        let r2 = std::str::from_utf8(r2_bytes.seq())?;
        let mut results = SequenceResults::new(r1, r2);
        results.match_into(&table);
        println!("{:#?}", results.variables());
    }
    Ok(())
}
