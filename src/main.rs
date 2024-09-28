mod format;
use clap::{Args, Parser, Subcommand};
use crate::format::clean;
use crate::format::right;
use crate::format::read_from_stdin_with_timeout;

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
//    Right(RightCli),
}

#[derive(Args)]
struct CleanCli {
    input: Option<String>,
}

//#[derive(Args)]
//struct RightCli {
//    input: Option<String>,
//    width: Option<usize>,
//}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Clean(input) => {
			clean(input.input.as_deref());
		},
//        Commands::Right(input) => {
//			let input_data = match input.input.as_deref() {
//				Some(input_str) => {
//					let result = right(input_str, input.width);
//				},
//				None => {
//					match read_from_stdin_with_timeout(5, 500) {
//						Ok(data) => {
//							println!("{}", right(&data, input.width));
//						},
//						Err(_) => return ,
//					}
//				}
//			};
//
//		},
    }
}
