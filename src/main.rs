mod builder;
mod cli;
mod command;
mod format;
mod formatter;
mod rows;
mod table;

use clap::ArgMatches;

fn main() {

    let matches: ArgMatches = match cli::build().try_get_matches() {
        Ok(matches) => matches,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    command::handle_cli(matches);

}
