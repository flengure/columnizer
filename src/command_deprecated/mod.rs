mod table;
mod clean;
mod format;

use clap::ArgMatches;

pub fn handle_cli(matches: ArgMatches) {
    // Check for the "clean" subcommand
    if let Some(clean_matches) = matches.subcommand_matches("clean") {
        if let Err(e) = clean::handle_cli(clean_matches) {
            eprintln!("Error cleaning text: {}", e);
            std::process::exit(1); // Exit with an error code
        }
    }
    // Check for the "format" subcommand
    else if let Some(format_matches) = matches.subcommand_matches("format") {
        format::handle_cli(format_matches.clone());
    }
    // If no subcommand is provided, handle the table command as the default
    else {
        // Create matches for the table command without any arguments
        let table_matches = crate::cli::table::command().get_matches();
        table::handle_cli(table_matches);
    }
}

