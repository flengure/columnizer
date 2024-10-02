use eyre::{Result, eyre, Context};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

/// Reads a binary file from the specified path and returns its contents
/// as a `Vec<u8>`.
///
/// # Arguments
///
/// * `path` - The path to the file.
///
/// # Errors
///
/// Returns an error if the file could not be opened or read.
fn read_binary(path: &Path) -> Result<Vec<u8>> {
	let mut contents = Vec::new();
	
	File::open(path)
		.with_context(|| format!("Error opening file '{}'", path.display()))?
		.read_to_end(&mut contents)
		.with_context(|| format!("Error reading file '{}'", path.display()))?;
	
	Ok(contents)
}

/// Reads a text file from the specified path and returns its contents
/// as a `String`.
///
/// # Arguments
///
/// * `path` - The path to the file.
///
/// # Errors
///
/// Returns an error if the file could not be opened or read.
fn read_text(path: &Path) -> Result<String> {
	let mut contents = String::new();

	File::open(path)
		.with_context(|| format!("Error opening file '{}'", path.display()))?
		.read_to_string(&mut contents)
		.with_context(|| format!("Error reading file '{}'", path.display()))?;
	
	Ok(contents)
}

/// Represents input data that can be either text (UTF-8) or binary.
pub enum InputData {
	Text(String),
	#[allow(dead_code)]
	Binary(Vec<u8>),
}

/// Processes input data from a file, a UTF-8 string, or binary input.
///
/// This function handles input in the following ways:
/// 1. If a file path is provided, it attempts to read the file as either
///	binary or text (UTF-8).
/// 2. If the input is not a valid file path, it checks if the input is a
///	valid UTF-8 string or treats it as binary data.
/// 3. If no input is provided, it reads from stdin and determines if the
///	data is text or binary.
///
/// # Arguments
///
/// * `input` - Optional file path, UTF-8 string, or binary input.
/// * `max_attempts` - The number of retry attempts for reading stdin.
/// * `delay` - The delay (in milliseconds) between retry attempts.
///
/// # Errors
///
/// Returns an error if the input could not be read after the specified
/// number of attempts or if no valid input was found.
///
/// # Examples
///
/// To process input from a file path:
///
/// ```rust
/// let input = process_input(Some("file.txt"), 3, 500);
/// match input {
///	 Ok(InputData::Text(data)) => println!("Text data: {}", data),
///	 Ok(InputData::Binary(data)) => println!("Binary data: {:?}", data),
///	 Err(e) => println!("Error: {}", e),
/// }
/// ```
///
/// To process input from a UTF-8 string or binary data:
///
/// ```rust
/// let input = process_input(Some("Some UTF-8 text"), 3, 500);
/// match input {
///	 Ok(InputData::Text(data)) => println!("Text data: {}", data),
///	 Ok(InputData::Binary(data)) => println!("Binary data: {:?}", data),
///	 Err(e) => println!("Error: {}", e),
/// }
/// ```
///
/// To process input from stdin (e.g., when no file or input is provided):
///
/// ```rust
/// let input = process_input(None, 3, 500);
/// match input {
///	 Ok(InputData::Text(data)) => println!("Text data: {}", data),
///	 Ok(InputData::Binary(data)) => println!("Binary data: {:?}", data),
///	 Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn process_input<T: AsRef<str>>(
	input: Option<T>, max_attempts: usize, delay: u64
) -> Result<InputData> {

	if let Some(input_str) = input {
		let path = Path::new(input_str.as_ref());

		// Attempt to read as binary from the path
		if let Ok(binary_data) = read_binary(&path) {
			return Ok(InputData::Binary(binary_data));
		}
		
		// If binary read fails, attempt to read as text
		if let Ok(text_data) = read_text(&path) {
			return Ok(InputData::Text(text_data));
		} 
		
		// If both read attempts fail, treat as invalid input
		// In case the input is a string, check if it's valid UTF-8 or binary
		let buf = input_str.as_ref().as_bytes().to_vec();
		if let Ok(text_data) = String::from_utf8(buf.clone()) {
			return Ok(InputData::Text(text_data)); // Valid UTF-8, treat as text
		} else {
			return Ok(InputData::Binary(buf)); // Treat as binary
		}
	}

	// If input is None, read from stdin
	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	let mut buf = Vec::new();

	for attempt in 0..max_attempts {
		match handle.read_to_end(&mut buf) {
			Ok(0) => {
				// No data read, retry after a delay
				if attempt == max_attempts - 1 {
					return Err(eyre!(
						"Failed to read input from stdin after {} attempts",
						max_attempts
					));
				}
				sleep(Duration::from_millis(delay));
			}
			Ok(_) => {
				// Successfully read input, determine if it's binary or text
				if let Ok(text_data) = String::from_utf8(buf.clone()) {
					return Ok(InputData::Text(text_data)); // Valid UTF-8, treat as text
				} else {
					return Ok(InputData::Binary(buf)); // Treat as binary
				}
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
