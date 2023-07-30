use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::run::{run, RunArgs};

#[derive(Debug, Parser)]
#[command(name = "lucy")]
#[command(about = "lucy is a DWN-based runtime in Wasm")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<LucyCommands>,
}

#[derive(Debug, Subcommand)]
pub enum LucyCommands {
    #[command(about = "Run a DWN program")]
    Run(RunArgs),
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self.command {
            Some(LucyCommands::Run(config)) => run(config),
            None => {
                println!("No command specified");
                Ok(())
            }
        }
    }
}
