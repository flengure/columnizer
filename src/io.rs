use eyre::{Result, eyre, Context};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

/// Reads input from a provided source (file or string) or falls back to reading from stdin.
///
/// If `input` is provided as `Some`, the function will read from this input. 
/// If `None`, the function will attempt to read from stdin. 
/// It retries reading from stdin up to `max_attempts` times, 
/// with a delay of `delay` milliseconds between each attempt.
///
/// # Arguments
///
/// * `input` - An optional value that implements `AsRef<str>`. 
///			  If it corresponds to a valid file path, it will read from that file.
///			  If `None`, stdin is used as the input source.
/// * `max_attempts` - The number of times to retry reading from stdin if no input is provided.
/// * `delay` - The number of milliseconds to wait between retry attempts.
///
/// # Returns
///
/// A `String` containing the trimmed lines of input. If reading fails after `max_attempts`,
/// an empty string is returned.
///
/// # Example
///
/// ```
/// let result = unwrap_or_stdin(Some("example.txt"), 3, 500);
/// assert_eq!(result, "expected content");
/// ```
pub fn unwrap_or_stdin<T: AsRef<str>>(
	input: Option<T>, max_attempts: usize, delay: u64
) -> Result<String> {

	// If an input is provided, check if it's a valid file path or a string.
	if let Some(input_data) = input {

		let input_str = input_data.as_ref();
		let path = Path::new(input_str);

		// Check if it's a valid file path
		if path.exists() && path.is_file() {

			// Create a buffer to store the read data
			let mut buf = String::new();

			// Attempt to open the file; if it fails, return the error with context
			let file = File::open(path)
				.wrap_err_with(|| format!("Failed to open file: {}", input_str))?;
			let mut reader = BufReader::new(file);
			reader.read_to_string(&mut buf)
				.wrap_err_with(|| format!("Failed to read from file: {}", input_str))?;

			return Ok(buf.lines()
				.map(|line| line.trim())
				.collect::<Vec<_>>() // Collect trimmed lines into a Vec<&str>
				.join("\n"));        // Join them into a single String

		} else {
			// If not a valid file, treat it as a string input
			return Ok(input_str.lines()
				.map(|line| line.trim())
				.collect::<Vec<_>>() // Collect trimmed lines into a Vec<&str>
				.join("\n"));        // Join them into a single String
		}
	}

	// If input is None, read from stdin
	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	let mut buf = String::new();

	for attempt in 0..max_attempts {
		match handle.read_to_string(&mut buf) {
			Ok(0) => {
				// No data read, retry after a delay
				if attempt == max_attempts - 1 {
					return Err(eyre!("Failed to read input from stdin after {} attempts", max_attempts));
				}
				sleep(Duration::from_millis(delay));
			}
			Ok(_) => {
				// Successfully read input, trim and return it
				return Ok(buf.lines()
					.map(|line| line.trim())
					.collect::<Vec<_>>() // Collect trimmed lines into a Vec<&str>
					.join("\n"));        // Join them into a single String
			}
			Err(e) => {
				// Error occurred, return with context
				return Err(eyre!("Error reading from stdin: {}", e));
			}
		}
	}

	// If nothing was read after all attempts, return an error
	Err(eyre!("Failed to read input after {} attempts", max_attempts))
}
