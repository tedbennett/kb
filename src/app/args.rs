use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub filename: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds files to myapp
    New(NewBoardArgs),
}

#[derive(Args)]
pub struct NewBoardArgs {
    pub filename: Option<String>,
}
