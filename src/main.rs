mod format;
mod io;
use clap::{Args, Parser, Subcommand};
use crate::format::clean;
use crate::format::right;
use crate::format::left;
use crate::format::center;
use crate::format::truncate;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sanitizes the input text by removing leading and trailing blank lines and whitespace
    Clean(CleanCli),
	Left(LeftCli),
	Right(RightCli),
	Truncate(TruncateCli),
	Center(CenterCli),
}

#[derive(Args)]
struct CleanCli {
    input: Option<String>,
}

#[derive(Args)]
struct LeftCli {
	input: Option<String>,
}

#[derive(Args)]
struct RightCli {
	input: Option<String>,

    #[arg(short, long)]
	width: Option<usize>,
}

#[derive(Args)]
struct CenterCli {
	input: Option<String>,

    #[arg(short, long)]
	width: Option<usize>,
}

#[derive(Args)]
struct TruncateCli {
	input: Option<String>,

    #[arg(short, long)]
	width: Option<usize>,

    #[arg(short, long)]
	no_ellipsis: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Clean(input) => {
			println!("{}", clean(input.input.as_deref()));
		},
        Commands::Left(input) => {
			println!("{}", left(input.input.as_deref()));
		},
        Commands::Right(input) => {
			println!("{}", right(input.input.as_deref(), input.width));
		},
        Commands::Truncate(input) => {
			println!("{}", truncate(input.input.as_deref(), input.width, Some(input.no_ellipsis)));
		},
        Commands::Center(input) => {
			println!("{}", center(input.input.as_deref(), input.width));
		},
    }
}
