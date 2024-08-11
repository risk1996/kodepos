mod cmd_populate;
mod data_structure;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
  #[clap(name = "populate")]
  Populate {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
  },
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    | Command::Populate { input, output } => cmd_populate::populate(input, output)?,
  };

  Ok(())
}
