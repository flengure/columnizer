/// Cleans the input string by removing blank lines and trimming whitespace.
///
/// # Arguments
///
/// * `input` - A string slice that needs to be cleaned.
///
/// # Returns
///
/// A `String` with blank lines removed and trimmed content.
pub fn clean(input: &str) -> String {
	input
		.lines()                                 // Split input into lines
		.filter(|line| !line.trim().is_empty())  // Remove empty lines
		.map(str::trim)                          // Trim whitespace from each line
		.collect::<Vec<_>>()                     // Collect lines into a Vec
		.join("\n")                              // Join them back into a single String
}
