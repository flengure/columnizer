use clap::ArgMatches;
use crate::formatter::{Alignment, Frame};
use std::io;
use std::io::Read;
use crate::builder::TableBuilder;

pub fn handle_cli(matches: ArgMatches) {

    let ifs = matches.get_one::<String>("ifs").expect("default value is set").to_string();
    let ofs = matches.get_one::<String>("ofs").expect("default value is set").to_string();
    let header_index = *matches.get_one::<usize>("header_index").expect("default value is set");
    let header_count = *matches.get_one::<usize>("header_count").expect("default value is set");
    let column_width_limits_index = *matches.get_one::<usize>("column_width_limits_index").expect("default value is set");
    let no_divider = *matches.get_one::<bool>("no_divider").expect("default value is set");
    let divider_char = *matches.get_one::<char>("divider_char").expect("default value is set");
    let max_cell_width = *matches.get_one::<usize>("max_cell_width").expect("default value is set");
    let frame: Frame = *matches.get_one::<Frame>("frame").expect("default value is set");
    let no_ellipsis = *matches.get_one::<bool>("no_ellipsis").expect("default value is set");
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
		if let Err(e) = io::stdin().read_to_string(&mut buf) {
			eprintln!("Failed to read from stdin: {}", e);
			std::process::exit(1);
		}
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
		.set_no_ellipsis(no_ellipsis)
		.set_alignment(alignment)
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

