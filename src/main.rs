use anyhow::Result;
use clap::Parser;

mod cli;
mod kmer;
mod sequence;
mod sgrna_table;
mod utils;
use cli::{Cli, Commands};
use sequence::SequenceResults;
use sgrna_table::VariableTable;

fn collect_spacers(r1: &str, r2: &str, sgrna_table: &str) -> Result<()> {
    let table = VariableTable::from_file(sgrna_table)?;
    let r1_reader = fxread::initialize_reader(r1)?;
    let r2_reader = fxread::initialize_reader(r2)?;

    for (r1_bytes, r2_bytes) in r1_reader.zip(r2_reader) {
        let r1 = std::str::from_utf8(r1_bytes.seq())?;
        let r2 = std::str::from_utf8(r2_bytes.seq())?;
        let mut results = SequenceResults::new(r1, r2);
        results.match_into(&table);
        println!("{:#?}", results.variables());
    }

    Ok(())
}

fn collect_constructs(r1: &str, r2: &str, sgrna_table: &str, dr_table: &str) -> Result<()> {
    let table = VariableTable::from_file(sgrna_table)?;
    println!("{:#?}", table);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Spacers { r1, r2, sgrna_table } => {
            collect_spacers(&r1, &r2, &sgrna_table)?;
        },
        Commands::Constructs { r1, r2, sgrna_table, dr_table } => {
            collect_constructs(&r1, &r2, &sgrna_table, &dr_table)?;
        }
    }
    Ok(())
}
