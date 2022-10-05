use anyhow::Result;
use clap::Parser;

mod cli;
mod sgrna_table;
use cli::Cli;
use sgrna_table::VariableTable;


fn main() -> Result<()> {
    let cli = Cli::parse();
    let table = VariableTable::from_file(&cli.sgrna_table)?;
    println!("{:#?}", table);
    Ok(())
}
