pub mod commands;

use anyhow::{Context, Result};
use clap::Parser;

use commands::Cli;

fn main() -> Result<()> {
    Cli::parse().execute().context("Failed to execute command")
}
