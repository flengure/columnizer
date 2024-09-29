mod builder;
mod format;
mod formatter;
mod io;
mod rows;
mod table;
use clap::{ Args, Parser, Subcommand };
use crate::format::{ center, clean, left, right, truncate, wrap };
use crate::formatter::{ Formatter };
use crate::builder::{ TableBuilder };

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Formats,
}

#[derive(Subcommand)]
enum Formats {
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
	/// Formats text based on cetain parameters
	Format(Formatter),
	/// Formats table based on cetain parameters
	Table(TableBuilder),
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
		Formats::Center(input) => {
			println!("{}", center(input.input.as_deref(), input.width));
		},
		Formats::Clean(input) => {
			println!("{}", clean(input.input.as_deref()));
		},
		Formats::Left(input) => {
			println!("{}", left(input.input.as_deref()));
		},
		Formats::Right(input) => {
			println!("{}", right(input.input.as_deref(), input.width));
		},
		Formats::Truncate(input) => {
			println!("{}", truncate(input.input.as_deref(), input.width, Some(input.no_ellipsis)));
		},
		Formats::Wrap(input) => {
			println!("{}", wrap(input.input.as_deref(), input.width));
		},
		Formats::Format(input) => {
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
		Formats::Table(input) => {
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

			let formatted_table = table.build();
			
			println!("{}", formatted_table);
		},
	}
}
