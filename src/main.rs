mod builder;
mod cell;
mod rows;
mod table;
mod text;
use clap::{Arg, ArgAction, Command};
use crate::table::{Alignment, Frame, TableBuilder};
use std::io::{self, Read};

fn main() {
    let matches = Command::new("columnizer")
        .version("1.0")
        .about("Formats text into columns with customizable options.")
        .long_about(
            "The `columnizer` tool takes input text and formats it into a neatly aligned columnar view. \
            You can specify the number of header rows, a divider line, and separators for fields in both \
            the input and output.",
        )
        .arg(
            Arg::new("input")
                .help("Input text to be formatted. If not provided, reads from stdin.")
                .index(1)
                .value_parser(clap::value_parser!(String))
                .long_help(
                    "Provide input text directly to be formatted. \
                    If not provided, the tool will read input from standard input (stdin).",
                ),
        )
        .arg(
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
        )
        .arg(
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
        )
        .arg(
            Arg::new("header_index")
                .long("header-row")
                .visible_alias("header_index")
                .value_parser(clap::value_parser!(usize))
                .default_value("1")
                .help("Specify the row number to treat as the header (default: 1). Set to 0 for no header.")
                .long_help(
                    "Specify the row number that should be treated as the header.\n\
                    If set to 0, it indicates that the data contains no headers or that you wish to format\n\
                    headers using prettytable."
                ),
        )
        .arg(
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
        )
        .arg(
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
        )
        .arg(
            Arg::new("no_divider")
                .long("no-divider")
                .action(ArgAction::SetTrue)
                .help("Add a divider line after the header row.")
                .long_help(
                    "If set, adds a divider line after the header row and before any other lines.",
                ),
        )
        .arg(
            Arg::new("divider_char")
                .long("divider-char")
                .value_parser(clap::value_parser!(char))
                .default_value("-")
                .help("Character used for the divider line.")
                .long_help(
                    "Set the character that will be used to draw the divider line between columns. \
                    The default character is a dash ('-').",
                ),
        )
        .arg(
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
        )
        .arg(
            Arg::new("frame")
                .long("frame")
                .value_parser(clap::value_parser!(Frame))
                .default_value("TRUNCATE")
                .help("Specifies the frame style for table cells.")
                .long_help(
                    "Set the frame style for table cells. Options include: TRUNCATE, WRAP, or NONE. \
                    The default value is TRUNCATE, which limits text in a cell to the width of the column.",
                ),
        )
        .arg(
            Arg::new("ellipsis")
                .long("ellipsis")
                .action(ArgAction::SetTrue)
                .help("Use ellipsis when truncating long text.")
                .long_help(
                    "When enabled, ellipses ('...') will be used to indicate truncated text. \
                    This is useful when the content is cut off at a specified width.",
                ),
        )
        .arg(
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
        )
        .arg(
            Arg::new("pad_decimal_digits")
                .short('p')
                .long("pad-decimal-digits")
                .action(ArgAction::SetTrue)
                .help("Pad decimal digits in numeric columns.")
                .long_help(
                    "Set this flag to true if you want to pad the decimals for numeric columns. \
                    By default, this is set to false.",
                ),
        )
        .arg(
            Arg::new("max_decimal_digits")
                .short('d')
                .long("max-decimal-digits")
                .value_parser(clap::value_parser!(usize))
                .default_value("2")
                .help("Maximum number of decimal places for numeric columns.")
                .long_help(
                    "Specify the maximum number of decimal places to display for numeric columns. \
                    The default value is 2.",
                ),
        )
        .arg(
            Arg::new("decimal_separator")
                .short('e')
                .long("decimal-separator")
                .value_parser(clap::value_parser!(char))
                .default_value(".")
                .help("Character used as a decimal separator.")
                .long_help(
                    "Specify the character used as a decimal separator. \
                    The default value is '.'.",
                ),
        )
        .arg(
            Arg::new("use_thousand_separator")
                .long("use-thousand-separator")
                .action(ArgAction::SetTrue)
                .help("Add a thousands separator to numbers.")
                .long_help(
                    "If set, adds a thousands separator to numbers.",
                ),
        )
        .arg(
            Arg::new("thousand_separator")
                .long("thousand-separator")
                .value_parser(clap::value_parser!(char))
                .default_value(",")
                .help("Character used as a thousands separator.")
                .long_help(
                    "Specify the character used as a thousands separator. \
                    The default value is ','.",
                ),
        )
        .get_matches();

    // Argument retrievals are directly used since default values are already set by Clap.
    let ifs = matches.get_one::<String>("ifs").expect("default value is set").to_string();
    let ofs = matches.get_one::<String>("ofs").expect("default value is set").to_string();
    let header_index = *matches.get_one::<usize>("header_index").expect("default value is set");
    let header_count = *matches.get_one::<usize>("header_count").expect("default value is set");
    let column_width_limits_index = *matches.get_one::<usize>("column_width_limits_index").expect("default value is set");
    let no_divider = *matches.get_one::<bool>("no_divider").expect("default value is set");
    let divider_char = *matches.get_one::<char>("divider_char").expect("default value is set");
    let max_cell_width = *matches.get_one::<usize>("max_cell_width").expect("default value is set");
    let frame: Frame = *matches.get_one::<Frame>("frame").expect("default value is set");
    let ellipsis = *matches.get_one::<bool>("ellipsis").expect("default value is set");
    let alignment: Alignment = *matches.get_one::<Alignment>("alignment").expect("default value is set");
    let pad_decimal_digits = *matches.get_one::<bool>("pad_decimal_digits").expect("default value is set");
    let max_decimal_digits = *matches.get_one::<usize>("max_decimal_digits").expect("default value is set");
    let decimal_separator = *matches.get_one::<char>("decimal_separator").expect("default value is set");
    let use_thousand_separator = *matches.get_one::<bool>("use_thousand_separator").expect("default value is set");
    let thousand_separator = *matches.get_one::<char>("thousand_separator").expect("default value is set");

    // Read input text or stdin
    let input = if let Some(input_text) = matches.get_one::<String>("input") {
        input_text.to_string()
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("Failed to read from stdin");
        buf
    };

	let mut table_builder = TableBuilder::new(input)
		.set_ifs(ifs)
		.set_ofs(ofs)
		.set_header_index(header_index)
		.set_header_count(header_count)
		.set_column_width_limits_index(column_width_limits_index)
		.set_no_divider(no_divider)
		.set_divider_char(divider_char)
		.set_max_cell_width(max_cell_width)
		.set_frame(frame)
		.set_ellipsis(ellipsis)
		.set_pad_decimal_digits(pad_decimal_digits)
		.set_max_decimal_digits(max_decimal_digits)
		.set_decimal_separator(decimal_separator)
		.set_use_thousand_separator(use_thousand_separator)
		.set_thousand_separator(thousand_separator)
		.clone();

	let table = table_builder.table();

    // Print the table
    table.printstd();

}



