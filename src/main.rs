mod format;
mod formatter;
mod io;
use clap::{Args, Parser, Subcommand};
use crate::format::{ center, clean, left, right, truncate, wrap };
use crate::formatter::{ Alignment, Frame, Formatter };

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Formats text based on cetain parameters
	Format(FormatCli),
	/// Sanitizes the input text by removing leading and trailing blank lines and whitespace
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
}

#[derive(Args)]
struct FormatCli {
	input: Option<String>,

	#[arg(default_value_t = 48)]
	#[arg(short, long)]
	width: usize,

	#[arg(default_value_t = Frame::TRUNCATE)]
	#[arg(value_enum)]
	#[arg(short, long)]
	frame: Frame,

	#[arg(short, long)]
	no_ellipsis: bool,

	#[arg(default_value_t = Alignment::AUTO)]
	#[arg(value_enum)]
	#[arg(short, long)]
	alignment: Alignment,

	#[arg(short, long)]
	pad_decimal_digits: bool,

	#[arg(default_value_t = 2)]
	#[arg(short, long)]
	max_decimal_digits: usize,

	#[arg(default_value_t = '.')]
	#[arg(short, long)]
	decimal_separator: char,

	#[arg(short, long)]
	use_thousand_separator: bool,

	#[arg(default_value_t = ',')]
	#[arg(short, long)]
	thousand_separator: char,
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
struct WrapCli {
	input: Option<String>,

	#[arg(short, long)]
	width: usize,
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
		Commands::Center(input) => {
			println!("{}", center(input.input.as_deref(), input.width));
		},
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
		Commands::Wrap(input) => {
			println!("{}", wrap(input.input.as_deref(), input.width));
		},
		Commands::Format(input) => {
			let mut formatter = Formatter::new(input.input.clone())
				.set_width(input.width)
				.set_frame(input.frame)
				.set_no_ellipsis(input.no_ellipsis)
				.set_alignment(input.alignment)
				.set_pad_decimal_digits(input.pad_decimal_digits)
				.set_max_decimal_digits(input.max_decimal_digits)
				.set_decimal_separator(input.decimal_separator)
				.set_use_thousand_separator(input.use_thousand_separator)
				.set_thousand_separator(input.thousand_separator)
				.clone();

			let formatted_text = formatter.formatted();
			
			println!("{}", formatted_text);
		},
	}
}
