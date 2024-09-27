use clap::{Arg, ArgAction, Command};
use crate::formatter::Alignment;
use crate::formatter::Frame;

pub fn command() -> Command {
    Command::new("table")
        .about("Formats input text as a table")
		.args(args()) 
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
		Arg::new("ifs")
			.short('i')
			.long("ifs")
			.value_parser(clap::value_parser!(String))
			.default_value(" ")
			.help("Separator for fields in the input text.")
			.long_help(
				"Specify the character or string used to separate fields in the input text. \
				The default separator is a space.",
			),
		Arg::new("ofs")
			.short('o')
			.long("ofs")
			.value_parser(clap::value_parser!(String))
			.default_value(" ")
			.help("Separator for fields in the output text.")
			.long_help(
				"Specify the character or string used to separate fields in the output text. \
				The default separator is a space.",
			),
		Arg::new("header_index")
			.long("header-row")
			.visible_alias("header-index")
			.value_parser(clap::value_parser!(usize))
			.default_value("1")
			.help("Specify the row number to treat as the header (default: 1). Set to 0 for no header.")
			.long_help(
				"Specify the row number that should be treated as the header.\n\
				If set to 0, it indicates that the data contains no headers or that you wish to format\n\
				headers using prettytable."
			),
		Arg::new("header_count")
			.long("header-count")
			.alias("header-rows")
			.value_parser(clap::value_parser!(usize))
			.default_value("1")
			.help("Specify the number of rows that make up the header (default: 1).")
			.long_help(
				"Specify the number of rows that make up the header.\n\
				Use this if your header spans multiple rows."
			),
		Arg::new("column_width_limits_index")
			.long("max-width-row")
			.visible_alias("column_width_limits_index")
			.value_parser(clap::value_parser!(usize))
			.default_value("0")
			.help("Row containing maximum width for each column.")
			.long_help(
				"Specify the row number where each column's maximum widths are defined. \
				This row will be used to determine the column widths for formatting.",
			),
		Arg::new("no_divider")
			.long("no-divider")
			.action(ArgAction::SetTrue)
			.help("Add a divider line after the header row.")
			.long_help(
				"If set, adds a divider line after the header row and before any other lines.",
			),
		Arg::new("divider_char")
			.long("divider-char")
			.value_parser(clap::value_parser!(char))
			.default_value("-")
			.help("Character used for the divider line.")
			.long_help(
				"Set the character that will be used to draw the divider line between columns. \
				The default character is a dash ('-').",
			),
		Arg::new("max_cell_width")
			.long("max-cell-width")
			.alias("max-text-width")
			.value_parser(clap::value_parser!(usize))
			.default_value("40")
			.help("Maximum length of text fields.")
			.long_help(
				"Specify the maximum length of text fields. \
				Text fields will be trimmed to fit this length.",
			),
		Arg::new("frame")
			.long("frame")
			.value_parser(clap::value_parser!(Frame))
			.default_value("TRUNCATE")
			.help("Specifies the frame style for table cells.")
			.long_help(
				"Set the frame style for table cells. Options include: TRUNCATE, WRAP, or NONE. \
				The default value is TRUNCATE, which limits text in a cell to the width of the column.",
			),
		Arg::new("no_ellipsis")
			.long("no-ellipsis")
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
    ]
}
