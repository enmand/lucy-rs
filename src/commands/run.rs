use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;

use std::fs::File;

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(short, long, value_name = "FILE", required = true)]
    config: PathBuf,
}

pub fn run(args: RunArgs) -> Result<()> {
    println!("Running program: {:?}", args);
    let f = File::open(args.config).context("unable to open file");
    _ = f;
    Ok(())
}
