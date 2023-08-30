use args::{Command, LuminexCLIArgs};
use clap::Parser;
use std::{fs, path::Path, time::Instant};
use tokenizer::Tokenizer;

mod args;
mod ast;
mod tokenizer;
mod utils;

fn main() -> Result<(), String> {
    let args = LuminexCLIArgs::parse();

    match args.cmd {
        Command::Build => {}

        Command::Fix => {}

        Command::Analyze => {}

        Command::Init => {}

        Command::Test { test_file } => {}
    }

    Ok(())
}
