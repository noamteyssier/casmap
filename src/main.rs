use anyhow::Result;
use clap::Parser;

mod cli;
mod constant;
mod construct;
mod construct_counts;
mod construct_results;
mod construct_table;
mod kmer;
mod sequence;
mod spacer;
mod spacer_table;
mod utils;
use cli::{Cli, Commands};
use construct_counts::ConstructCounts;
use construct_table::ConstructTable;
use sequence::SequenceResults;
use spacer_table::SpacerTable;
use spinoff::{Color, Spinner, Spinners, Streams};

use crate::construct_results::ConstructResults;

fn collect_spacers(r1: &str, r2: &str, spacer_table: &str, _output: &str) -> Result<()> {
    let sp = Spinner::new_with_stream(
        Spinners::Dots12,
        "Building Spacer Hashmap",
        Color::Green,
        Streams::Stderr,
    );
    let table = SpacerTable::from_file(spacer_table)?;
    sp.stop_and_persist("✔️", "Finished Spacer Table");

    let sp = Spinner::new_with_stream(
        Spinners::Dots12,
        "Matching Reads",
        Color::Green,
        Streams::Stderr,
    );
    let r1_reader = fxread::initialize_reader(r1)?;
    let r2_reader = fxread::initialize_reader(r2)?;

    for (r1_bytes, r2_bytes) in r1_reader.zip(r2_reader) {
        let r1 = std::str::from_utf8(r1_bytes.seq())?;
        let r2 = std::str::from_utf8(r2_bytes.seq())?;
        let mut results = SequenceResults::new(r1, r2);
        results.match_into(&table);
        println!("{:#?}", results.spacers());
    }
    sp.stop_and_persist("✔️", "Finished Mapping Reads");

    Ok(())
}

fn collect_constructs(
    r1: &str,
    r2: &str,
    spacer_table: &str,
    constant_table: &str,
    output: &str,
) -> Result<()> {
    let sp = Spinner::new_with_stream(
        Spinners::Dots12,
        "Building Construct Hashmap",
        Color::Green,
        Streams::Stderr,
    );
    let table = ConstructTable::new(spacer_table, constant_table)?;
    let mut counts = ConstructCounts::new(table.len());
    sp.stop_and_persist("✔️", "Finished Construct Table");

    let sp = Spinner::new_with_stream(
        Spinners::Dots12,
        "Matching Reads",
        Color::Green,
        Streams::Stderr,
    );
    let r1_reader = fxread::initialize_reader(r1)?;
    let r2_reader = fxread::initialize_reader(r2)?;

    for (r1_bytes, r2_bytes) in r1_reader.zip(r2_reader) {
        let r1 = std::str::from_utf8(r1_bytes.seq())?;
        let r2 = std::str::from_utf8(r2_bytes.seq())?;
        let mut results = ConstructResults::new(r1, r2);
        results.match_into(&table);
        counts.count(&results);
    }
    sp.stop_and_persist("✔️", "Finished Mapping Reads");
    counts.pprint(output)?;
    counts.statistics();
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Spacers {
            r1,
            r2,
            spacer_table,
            output,
        } => {
            collect_spacers(&r1, &r2, &spacer_table, &output)?;
        }
        Commands::Constructs {
            r1,
            r2,
            spacer_table,
            constant_table,
            output,
        } => {
            collect_constructs(&r1, &r2, &spacer_table, &constant_table, &output)?;
        }
    }
    Ok(())
}
