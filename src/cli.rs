use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Reports the spacers and counts for each read
    Spacers {
        /// Read 1
        #[clap(short = 'i', long)]
        r1: String,

        /// Read 2
        #[clap(short = 'I', long)]
        r2: String,

        /// Spacer table
        #[clap(short = 's', long)]
        spacer_table: String,

        /// Output file to write to
        #[clap(short, long, default_value = "spacer_counts.tsv")]
        output: String,
    },

    /// Counts the number of perfect constructs
    Constructs {
        /// Read 1
        #[clap(short = 'i', long)]
        r1: String,

        /// Read 2
        #[clap(short = 'I', long)]
        r2: String,

        /// Spacer table
        #[clap(short = 's', long)]
        spacer_table: String,

        /// Constant Repeats
        #[clap(short = 'c', long)]
        constant_table: String,

        /// Output file to write to
        #[clap(short, long, default_value = "construct_counts.tsv")]
        output: String,
    },
}
