/// Custom parser for validating a hex string
pub fn hex(s: &str) -> Result<String, String> {
    if s.chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(s.to_string())
    } else {
        Err(format!("'{}' is not a valid hex string", s))
    }
}

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Determins if the input is a hex strin

	Clean(CleanCli),
	/// Aligns to the right, padding with spaces up to width
	Right(RightCli),
	/// Aligns to the left
	Left(LeftCli),
	/// Centers according to width
	Center(CenterCli),
	/// Wraps to width
	Wrap(WrapCli),
	/// Truncates to width
	Truncate(TruncateCli),
	/// Formats text based on certain parameters
	Format(TextFormatter),
	/// Formats table based on certain parameters
	Table(TableBuilder),
}
