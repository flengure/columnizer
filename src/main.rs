mod cli;
mod io;
mod table;
mod text;

use clap::Parser;

fn main() {
	cli::run_cli(&cli::Cli::parse());
}
