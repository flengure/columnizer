use std::io::{self, Cursor, Read};
use std::thread::sleep;
use std::time::Duration;

/// Reads input from a provided source or falls back to reading from stdin.
///
/// If `input` is provided as an `Option<R>` where `R` implements `Read`, the function
/// will read from this input. If `input` is `None`, the function will attempt to read
/// from stdin. It retries reading from stdin up to `max_attempts` times, with a delay
/// of `delay` milliseconds between each attempt.
///
/// # Arguments
///
/// * `input` - An optional input stream that implements `Read`. If `None`, stdin is used.
/// * `max_attempts` - The number of times to retry reading from stdin if no input is provided.
/// * `delay` - The number of milliseconds to wait between retry attempts.
///
/// # Returns
///
/// A `String` containing the trimmed lines of input. If no input is provided and reading
/// from stdin fails after `max_attempts`, an empty string is returned.
///
/// # Example
///
/// ```
/// use std::io::Cursor;
/// let input = Some(Cursor::new("example input\n"));
/// let result = input_or_stdin(input, 3, 500);
/// assert_eq!(result, "example input");
/// ```
pub fn input_or_stdin<R: Read>(input: Option<R>, max_attempts: usize, delay: u64) -> String {
    // If input is provided, read from it
    if let Some(mut reader) = input {
        let mut buf = String::new();
        match reader.read_to_string(&mut buf) {
            Ok(_) => {
                return buf.lines()
                    .map(|line| line.trim())
                    .collect::<Vec<_>>()
                    .join("\n");
            }
            Err(_) => return String::new(), // Handle any error during reading
        }
    }

    // Otherwise, read from stdin
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();

    for attempt in 0..max_attempts {
        match handle.read_to_string(&mut buf) {
            Ok(0) => {
                // No data read, retry after a delay
                if attempt == max_attempts - 1 {
                    return String::new();  // Give up after max_attempts
                }
                sleep(Duration::from_millis(delay));
            }
            Ok(_) => {
                // Successfully read input, trim and return it
                let result = buf
                    .lines()                    // Split into lines
                    .map(|line| line.trim())    // Trim each line
                    .map(String::from)          // Convert &str to String
                    .collect::<Vec<_>>()        // Collect into a Vec<String>
                    .join("\n");

                return result.to_string();
            }
            Err(_) => {
                // Error occurred, handle appropriately
                buf.clear();
            }
        }
    }

    String::new() // Return empty string after all attempts
}

/// Reads a string from a provided input or stdin, with retries.
///
/// This function wraps `input_or_stdin` to accept an optional string slice (`&str`) as input.
/// If the string slice is provided, it converts it into an input stream and reads from it.
/// Otherwise, it falls back to reading from stdin with retries.
///
/// # Arguments
///
/// * `input` - An optional string slice. If `None`, stdin is used as the input source.
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
/// let input = Some("test input");
/// let result = str_or_stdin(input, 3, 500);
/// assert_eq!(result, "test input");
/// ```
pub fn str_or_stdin(input: Option<&str>, max_attempts: usize, delay: u64) -> String {
    input_or_stdin(input.map(|s| Cursor::new(s.as_bytes())), max_attempts, delay)
}

