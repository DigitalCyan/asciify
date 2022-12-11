mod asciify;

use asciify::{args::CLIArgs, process_args};
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CLIArgs::parse();

    process_args(args);

    Ok(())
}
