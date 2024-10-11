mod cli;
//mod io;
mod input;
mod table;
mod text;
use eyre::Result;

use clap::Parser;

fn main() -> Result<()>{
    crate::cli::Cli::parse().run()
}
