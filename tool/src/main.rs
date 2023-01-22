#![allow(unused)]

use clap::Parser;
use log::{info, warn};
use std::io::{self, Write};
use anyhow::{Context, Result};


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle_lock = stdout.lock();


    // TODO optimize with BefReader    
    let content = std::fs::read_to_string(&args.path).unwrap();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    writeln!(handle_lock, "file content: {}", content);
    Ok(())
}


