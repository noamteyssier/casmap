use std::{fs::File, io::Write};

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
mod tuple_table;
mod tuple_results;
mod utils;
use cli::{Cli, Commands};
use construct_counts::ConstructCounts;
use construct_table::ConstructTable;
use sequence::SequenceResults;
use spacer_table::SpacerTable;
use spinoff::{Color, Spinner, Spinners, Streams};
use tuple_results::TupleResults;

use crate::{construct_results::ConstructResults, tuple_table::TupleTable};

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

fn collect_tuples(
    r1: &str,
    r2: &str,
    spacer_table: &str,
    output: &str,
) -> Result<()> {

    let sp = Spinner::new_with_stream(
        Spinners::Dots12,
        "Building Construct Hashmap",
        Color::Green,
        Streams::Stderr,
    );
    let table = TupleTable::from_file(spacer_table)?;
    let mut counts = ConstructCounts::new(table.len());
    sp.stop_and_persist("✔️", "Finished Construct Table");

    let r1_reader = fxread::initialize_reader(r1)?;
    let r2_reader = fxread::initialize_reader(r2)?;


    let sp = Spinner::new_with_stream(
        Spinners::Dots12,
        "Matching Reads",
        Color::Green,
        Streams::Stderr,
    );
    for (r1_bytes, r2_bytes) in r1_reader.zip(r2_reader) {
        let r1 = std::str::from_utf8(r1_bytes.seq())?;
        let r2 = std::str::from_utf8(r2_bytes.seq())?;
        let mut results = TupleResults::new(r1, r2);
        results.match_into(&table);
        counts.count_tuple(&results);
    }
    sp.stop_and_persist("✔️", "Finished Mapping Reads");
    counts.pprint(output)?;
    counts.statistics();
    Ok(())
}

fn build_constructs(spacer_table: &str, constant_table: &str, output: &str) -> Result<()> {
    let table = ConstructTable::new(spacer_table, constant_table)?;
    let mut file = File::create(output)?;
    for c in table.constructs() {
        let rep = format!(">cid_{}\n{}\n", c.cid(), c.sequence());
        write!(file, "{}", rep)?;
    }
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
        },
        Commands::Constructs {
            r1,
            r2,
            spacer_table,
            constant_table,
            output,
        } => {
            collect_constructs(&r1, &r2, &spacer_table, &constant_table, &output)?;
        },
        Commands::Build { spacer_table, constant_table, output } => {
            build_constructs(&spacer_table, &constant_table, &output)?;
        },
        Commands::Tuples { r1, r2, spacer_table, output } => {
            collect_tuples(&r1, &r2, &spacer_table, &output)?;
        }
    }
    Ok(())
}
