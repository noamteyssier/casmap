use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Spacers {
        /// Read 1
        #[clap(short = 'i', long)]
        r1: String,

        /// Read 2
        #[clap(short = 'I', long)]
        r2: String,

        /// sgRNA table
        #[clap(short = 's', long)]
        sgrna_table: String,
    },

    Constructs {
        /// Read 1
        #[clap(short = 'i', long)]
        r1: String,

        /// Read 2
        #[clap(short = 'I', long)]
        r2: String,

        /// sgRNA table
        #[clap(short = 's', long)]
        sgrna_table: String,

        /// Constant Repeats (DR)
        #[clap(short = 'd', long)]
        dr_table: String,
    },
}
