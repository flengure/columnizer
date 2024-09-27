use clap::Command;

pub mod table;
mod clean;
mod format;

pub fn build() -> Command {
	Command::new("table")
		.version("0.1.0")
		.author("flengure <flengure@gmail.com>")
		.about("format text as table")
		.subcommand(table::command())
		.subcommand(clean::command())
		.subcommand(format::command())
}
