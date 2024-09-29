use clap::ArgMatches;
use crate::formatter::{Alignment, Frame, Formatter};
use std::io;
use std::io::Read;

pub fn handle_cli(matches: ArgMatches) {

    let width = *matches.get_one::<usize>("width").expect("default value is set");
    let frame: Frame = *matches.get_one::<Frame>("frame").expect("default value is set");
    let no_ellipsis = *matches.get_one::<bool>("no_ellipsis").expect("default value is set");
    let alignment: Alignment = *matches.get_one::<Alignment>("alignment").expect("default value is set");
    let pad_decimal_digits = *matches.get_one::<bool>("pad_decimal_digits").expect("default value is set");
    let max_decimal_digits = *matches.get_one::<usize>("max_decimal_digits").expect("default value is set");
    let decimal_separator = *matches.get_one::<char>("decimal_separator").expect("default value is set");
    let use_thousand_separator = *matches.get_one::<bool>("use_thousand_separator").expect("default value is set");
    let thousand_separator = *matches.get_one::<char>("thousand_separator").expect("default value is set");

    // Read quote flags
    let quote = matches.get_one::<bool>("quote").unwrap_or(&false);
    let single_quote = matches.get_one::<bool>("single-quote").unwrap_or(&false);

    // Ensure that only one of quote or single_quote is used
    if *quote && *single_quote {
        eprintln!("Error: --quote and --single-quote cannot be used together.");
        std::process::exit(1);
    }

    // Read input text or stdin
	let input = if let Some(input_text) = matches.get_one::<String>("input") {
		input_text.to_string()
	} else {
		let mut buf = String::new();
		if let Err(e) = io::stdin().read_to_string(&mut buf) {
			eprintln!("Failed to read from stdin: {}", e);
			std::process::exit(1);
		}
		buf
	};

	let mut formatter = Formatter::new(&input)
		.set_width(width)
		.set_frame(frame)
		.set_no_ellipsis(no_ellipsis)
		.set_alignment(alignment)
		.set_pad_decimal_digits(pad_decimal_digits)
		.set_max_decimal_digits(max_decimal_digits)
		.set_decimal_separator(decimal_separator)
		.set_use_thousand_separator(use_thousand_separator)
		.set_thousand_separator(thousand_separator)
		.clone();

	let formatted_text = formatter.formatted();

	if *quote {
		println!("\"{}\"", formatted_text);
	} else if *single_quote {
		println!("'{}'", formatted_text);
	} else {
		println!("{}", formatted_text);
	}
}

