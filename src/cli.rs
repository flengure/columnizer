use clap::{Args, Parser, Subcommand};
use crate::text::{center, clean, Frame, left, right, truncate, wrap, text, TextFormatter};
use crate::table::TableBuilder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
	#[command(subcommand)]
	pub command: FmtCommands,
}

#[derive(Subcommand)]
pub enum FmtCommands {
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
	/// Formats text based on certain parameters
	Text(TextFormatter),
	/// Formats table based on certain parameters
	Table(TableBuilder),
	/// Checks if the format is numeric or hex
	#[command(subcommand)]
	Is(IsSubcommand),
}

#[derive(Subcommand)]
pub enum IsSubcommand {
	/// Checks if the format is hex
	Hex(HexCli),
	/// Checks if the format is numeric
	Numeric(NumericCli),
}

#[derive(Args)]
pub struct HexCli {
	pub text: Option<String>,
}

#[derive(Args)]
pub struct NumericCli {
	pub text: Option<String>,
}

#[derive(Args)]
pub struct CleanCli {
	pub text: Option<String>,
}

#[derive(Args)]
pub struct LeftCli {
	pub text: Option<String>,
}

#[derive(Args)]
pub struct RightCli {
	pub text: Option<String>,

	#[arg(short, long)]
	pub width: Option<usize>,
}

#[derive(Args)]
pub struct WrapCli {
	pub text: Option<String>,

	#[arg(short, long)]
	pub width: Option<usize>,
}

#[derive(Args)]
pub struct CenterCli {
	pub text: Option<String>,

	#[arg(short, long)]
	pub width: Option<usize>,
}

#[derive(Args)]
pub struct TruncateCli {
	pub text: Option<String>,

	#[arg(short, long)]
	pub width: Option<usize>,

	#[arg(short, long)]
	pub no_ellipsis: Option<bool>,

	#[arg(short, long)]
	pub frame: Option<Frame>,
}

pub fn run_cli(cli: &Cli) {

	match &cli.command {
		FmtCommands::Is(is_cmd) => {
			// Handle the subcommands under `fmt is`
			match is_cmd {
				IsSubcommand::Hex(input) => {
					let formatter = TextFormatter::new(input.text.clone());
					println!("{}", formatter.is_hex());
				},
				IsSubcommand::Numeric(input) => {
					let mut formatter = TextFormatter::new(input.text.clone());
					println!("{}", formatter.is_numeric());
				},
			}
		},
		FmtCommands::Center(input) => { println!("{}", center(
			input.text.as_deref(),
			input.width,
		)); },
		FmtCommands::Clean(input) => { println!("{}", clean(
			input.text.as_deref()
		)); },
		FmtCommands::Left(input) => { println!("{}", left(
			input.text.as_deref()
		)); },
		FmtCommands::Right(input) => { println!("{}", right(
			input.text.as_deref(),
			input.width
		)); },
		FmtCommands::Truncate(input) => { println!("{}", truncate(
			input.text.as_deref(), 
			input.width, 
			input.no_ellipsis,
			input.frame,
		)); },
		FmtCommands::Wrap(input) => { println!("{}", wrap(
			input.text.as_deref(),
			input.width
		)); },
		FmtCommands::Text(input) => { println!("{}", text(
			input.text.as_deref(), 
			Some(input.width),
			Some(input.frame),
			Some(input.no_ellipsis),
			Some(input.pad_decimal_digits),
			Some(input.max_decimal_digits),
			Some(input.decimal_separator),
			Some(input.use_thousand_separator),
			Some(input.thousand_separator),
			Some(input.alignment),
		)); }
		FmtCommands::Table(input) => {
			let mut table = TableBuilder::new(input.input.clone())
				.set_ifs(input.ifs.clone())
				.set_ofs(input.ofs.clone())
				.set_header_index(input.header_index)
				.set_header_count(input.header_count)
				.set_column_width_limits_index(input.column_width_limits_index)
				.set_no_divider(input.no_divider)
				.set_divider_char(input.divider_char)
				.set_max_cell_width(input.max_cell_width)
				.set_frame(input.frame)
				.set_no_ellipsis(input.no_ellipsis)
				.set_alignment(input.alignment)
				.set_pad_decimal_digits(input.pad_decimal_digits)
				.set_max_decimal_digits(input.max_decimal_digits)
				.set_decimal_separator(input.decimal_separator)
				.set_use_thousand_separator(input.use_thousand_separator)
				.set_thousand_separator(input.thousand_separator)
				.clone();

			let built_table = table.build();
			built_table.printstd();
		},
	}
}
