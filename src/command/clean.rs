use clap::ArgMatches;
use crate::format::clean;
use std::io;
use std::io::Read;

pub fn handle_cli(matches: &ArgMatches) -> Result<String, Box<dyn std::error::Error>> {
    // Get the "text" argument
//    let text = matches.get_one::<String>("text").expect("text is required").clone();

    // Read input text or stdin
	let input = if let Some(input_text) = matches.get_one::<String>("text") {
		input_text.to_string()
	} else {
		let mut buf = String::new();
		if let Err(e) = io::stdin().read_to_string(&mut buf) {
			eprintln!("Failed to read from stdin: {}", e);
			std::process::exit(1);
		}
		buf
	};

	println!("hi ther: {}", &input);

    // Clean the text
    let cleaned_text = clean(&input);

    Ok(cleaned_text)
}
