/// Main entry point for the `columnizer` tool.
///
/// This tool formats input text into columns with customizable options. You can specify various arguments
/// to control the formatting, including field separators, header rows, divider lines, and text width.
///
/// # Arguments
///
/// * `input` - Optional. The input text to be formatted. If not provided, the tool reads from standard input (stdin).
/// * `--IFS` - Separator for fields in the input text (default is a space).
/// * `--OFS` - Separator for fields in the output text (default is a space).
/// * `--header-row` - Row number to be treated as the header or 0 for no header (default is 0).
/// * `--max-width-row` - Row number containing the maximum width for each column (default is 0).
/// * `--format-string-row` - Row number containing the format string for each column (default is 0).
/// * `--add-divider` - Add a divider line after the header row (boolean flag).
/// * `--divider-char` - Character used for the divider line (default is '-').
/// * `--max-text-width` - Maximum length of text fields (default is 40).
/// * `--pad-decimal-digits` - Pad decimal digits in numeric columns (boolean flag).
/// * `--max-decimal-digits` - Maximum number of decimal places for numeric columns (default is 2).
/// * `--decimal-separator` - Character used as a decimal separator (default is '.').
/// * `--add-thousand-separator` - Add a thousands separator to numbers (boolean flag).
/// * `--thousand-separator` - Character used as a thousands separator (default is ',').
///
/// # Example
///
/// ```
/// $ columnizer --header-row 1 --add-divider --divider-char '=' --max-text-width 20 --pad-decimal-digits
/// ```
mod columnizer;
use clap::{Arg, ArgAction, Command};
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
                .long("IFS")
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
                .long("OFS")
                .value_parser(clap::value_parser!(String))
                .default_value(" ")
                .help("Separator for fields in the output text.")
                .long_help(
                    "Specify the character or string used to separate fields in the output text. \
                    The default separator is a space.",
                ),
        )
        .arg(
            Arg::new("header_row")
                .short('r')
                .long("header-row")
                .value_parser(clap::value_parser!(usize))
                .default_value("0")
                .help("Row number of the header or 0 for no header.")
                .long_help(
                    "Specify the row number that should be treated as the header. \
                    If you don't want a header, set this to 0. \
                    The header row will not be formatted with column alignment.",
                ),
        )
        .arg(
            Arg::new("max_width_row")
                .short('w')
                .long("max-width-row")
                .value_parser(clap::value_parser!(usize))
                .default_value("0")
                .help("Row number containing maximum width for each column.")
                .long_help(
                    "Specify the row number where each column's maximum width is defined. \
                    This row will be used to determine the column widths for formatting.",
                ),
        )
        .arg(
            Arg::new("format_string_row")
                .short('f')
                .long("format-string-row")
                .value_parser(clap::value_parser!(usize))
                .default_value("0")
                .help("Row number containing a Rust format string for each column.")
                .long_help(
                    "Specify the row number where each column's format string is defined. \
                    This row will be used to determine the columns' format string for formatting.",
                ),
        )
        .arg(
            Arg::new("no_divider")
                .short('n')
                .long("no-divider")
                .action(ArgAction::SetTrue)
                .help("Add a divider line after the header row.")
                .long_help(
                    "If set, adds a divider line after the header row and before any other lines.",
                ),
        )
        .arg(
            Arg::new("divider_char")
                .short('c')
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
            Arg::new("max_text_width")
                .short('m')
                .long("max-text-width")
                .value_parser(clap::value_parser!(usize))
                .default_value("40")
                .help("Maximum length of text fields.")
                .long_help(
                    "Specify the maximum length of text fields. \
                    Text fields will be trimmed to fit this length.",
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
            Arg::new("add_thousand_separator")
                .short('t')
                .long("add-thousand-separator")
                .action(ArgAction::SetTrue)
                .help("Add a thousands separator to numbers.")
                .long_help(
                    "If set, adds a thousands separator to numbers.",
                ),
        )
        .arg(
            Arg::new("thousand_separator")
                .short('s')
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
    let header_row = *matches.get_one::<usize>("header_row").expect("default value is set");
    let max_width_row = *matches.get_one::<usize>("max_width_row").expect("default value is set");
    let format_string_row = *matches.get_one::<usize>("format_string_row").expect("default value is set");
    let no_divider = *matches.get_one::<bool>("no_divider").expect("default value is set");
    let divider_char = *matches.get_one::<char>("divider_char").expect("default value is set");
    let max_text_width = *matches.get_one::<usize>("max_text_width").expect("default value is set");
    let pad_decimal_digits = *matches.get_one::<bool>("pad_decimal_digits").expect("default value is set");
    let max_decimal_digits = *matches.get_one::<usize>("max_decimal_digits").expect("default value is set");
    let decimal_separator = *matches.get_one::<char>("decimal_separator").expect("default value is set");
    let add_thousand_separator = *matches.get_one::<bool>("add_thousand_separator").expect("default value is set");
    let thousand_separator = *matches.get_one::<char>("thousand_separator").expect("default value is set");

    // Read input text or stdin
    let input = if let Some(input_text) = matches.get_one::<String>("input") {
        input_text.to_string()
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("Failed to read from stdin");
        buf
    };

    let formatted_output = columnizer::run(
        &input,
        &ifs,
        &ofs,
        header_row,
        max_width_row,
        format_string_row,
        no_divider,
        divider_char,
        max_text_width,
        pad_decimal_digits,
        max_decimal_digits,
        decimal_separator,
        add_thousand_separator,
        thousand_separator,
    );

    // Handle the formatted output, e.g., print it
    println!("{}", formatted_output);
}
