
use clap::{Arg, ArgAction, ArgGroup, Command};
use crate::formatter::{Alignment, Frame};

pub fn command() -> Command {
    Command::new("format")
        .about("Formats input text, wrapping/runcating text and formatting/aligning numbers")
		.args(args()) 
        .group(
            ArgGroup::new("quote_group")
                .args(&["quote", "single_quote"])
                .required(false), // Neither should be required
        )
}

fn args() -> Vec<Arg> {
    vec![
		Arg::new("input")
			.help("Input text to be formatted. If not provided, reads from stdin.")
			.short('t')
			.long("text")
			.visible_alias("input")
			.value_parser(clap::value_parser!(String))
			.long_help(
				"Provide input text directly to be formatted. \
				If not provided, the tool will read input from standard input (stdin).",
			),
		Arg::new("width")
			.short('w')
			.long("width")
			.value_parser(clap::value_parser!(usize))
			.default_value("20")
			.help("Width to be used for wrapping, truncating or alignment"),
		Arg::new("frame")
			.long("frame")
			.value_parser(clap::value_parser!(Frame))
			.default_value("TRUNCATE")
			.help("Specifies the frame style for table cells.")
			.long_help(
				"Set the frame style for table cells. Options include: TRUNCATE, WRAP, or NONE. \
				The default value is TRUNCATE, which limits text in a cell to the width of the column.",
			),
		Arg::new("ellipsis")
			.long("ellipsis")
			.action(ArgAction::SetTrue)
			.help("Use ellipsis when truncating long text.")
			.long_help(
				"When enabled, ellipses ('...') will be used to indicate truncated text. \
				This is useful when the content is cut off at a specified width.",
			),
		Arg::new("alignment")
			.long("alignment")
			.value_parser(clap::value_parser!(Alignment))
			.default_value("AUTO")
			.help("Sets the alignment of text in table cells.")
			.long_help(
				"Set the alignment for table cells. The available options are: LEFT, RIGHT, CENTER, or AUTO. \
				AUTO aligns numeric columns to the right and other columns to the left by default. \
				The default value is AUTO.",
			),
		Arg::new("pad_decimal_digits")
			.long("pad-decimal-digits")
			.action(ArgAction::SetTrue)
			.help("Pad decimal digits in numeric columns.")
			.long_help(
				"Set this flag to true if you want to pad the decimals for numeric columns. \
				By default, this is set to false.",
			),
		Arg::new("max_decimal_digits")
			.long("max-decimal-digits")
			.value_parser(clap::value_parser!(usize))
			.default_value("2")
			.help("Maximum number of decimal places for numeric columns.")
			.long_help(
				"Specify the maximum number of decimal places to display for numeric columns. \
				The default value is 2.",
			),
		Arg::new("decimal_separator")
			.long("decimal-separator")
			.value_parser(clap::value_parser!(char))
			.default_value(".")
			.help("Character used as a decimal separator.")
			.long_help(
				"Specify the character used as a decimal separator. \
				The default value is '.'.",
			),
		Arg::new("use_thousand_separator")
			.long("use-thousand-separator")
			.action(ArgAction::SetTrue)
			.help("Add a thousands separator to numbers.")
			.long_help(
				"If set, adds a thousands separator to numbers.",
			),
		Arg::new("thousand_separator")
			.long("thousand-separator")
			.value_parser(clap::value_parser!(char))
			.default_value(",")
			.help("Character used as a thousands separator.")
			.long_help(
				"Specify the character used as a thousands separator. \
				The default value is ','.",
			),
        Arg::new("quote")
            .long("quote")
            .help("Use double quotes for formatting.")
			.action(ArgAction::SetTrue),
        Arg::new("single_quote")
            .long("single-quote")
            .help("Use single quotes for formatting.")
			.action(ArgAction::SetTrue)
    ]
}
