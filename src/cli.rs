use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {

    /// Read 1
    #[clap(short='i', long)]
    pub r1: String,

    /// Read 2
    #[clap(short='I', long)]
    pub r2: String,

    /// sgRNA table
    #[clap(short='s', long)]
    pub sgrna_table: String,

}

