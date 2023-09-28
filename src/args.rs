use clap::{
	Parser,
	Subcommand,
};

#[derive(Parser, Debug)]
pub struct LuminexCLIArgs {
	/*
	The function to run.
	 */
	 #[clap(subcommand)]
	pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command { Build, Analyze, Fix, Init, Test { test_file: String } }