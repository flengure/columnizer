use clap::ArgMatches;
use crate::format::clean;

pub fn handle_cli(matches: &ArgMatches) -> Result<String, Box<dyn std::error::Error>> {
    // Get the "text" argument
    let text = matches.get_one::<String>("text").expect("text is required").clone();

    // Clean the text
    let cleaned_text = clean(&text);

    Ok(cleaned_text)
}
