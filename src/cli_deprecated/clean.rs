use clap::{Arg, Command};

pub fn command() -> Command {
	Command::new("clean")
		.about("Clean text by removing unnecessary characters")
		.arg(
			Arg::new("text")
				.help("Text to be cleaned")
				.required(true)
				.index(1),
		)
}
