mod cmd_populate;
mod data_structure;

use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenvy::var;
use sea_orm::{Database, DatabaseConnection};

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
  },
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();
  let db_url = var("DATABASE_URL")?;
  let db: DatabaseConnection = Database::connect(db_url).await?;

  match cli.command {
    | Command::Populate { input } => cmd_populate::populate(input, &db).await?,
  };

  Ok(())
}
