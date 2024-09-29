use std::io::{self, Cursor, Read};
use std::thread::sleep;
use std::time::Duration;

/// Reads input from a provided string or `stdin` with a configurable retry mechanism.
///
/// # Parameters:
/// - `input`: An optional string slice. If `Some(input_str)` is provided, it will be returned
///   directly. If `None`, the function will attempt to read from `stdin`.
/// - `max_attempts`: The maximum number of retry attempts when reading from `stdin` if no data
///   is initially available. A value of `5` will try up to 5 times.
/// - `delay`: The amount of time (in milliseconds) to wait between attempts when no data is
///   available from `stdin`. This is useful for situations where input may take time to appear.
///
/// # Returns:
/// - A `String` containing the input from either the provided `input` or `stdin`.
/// - If input is provided, it returns that string immediately.
/// - If reading from `stdin`, it returns the trimmed string once data is successfully read.
/// - If the maximum number of attempts is exhausted, it returns an empty string.
///
/// # Behavior:
/// - On failed attempts to read from `stdin`, the buffer is cleared and sleeps for the specified
///   `delay` before retrying, up to `max_attempts`.
/// - If an error occurs during the read, the function retries unless the error persists after
///   `max_attempts`, returning an empty string.
/// - The result of a successful `stdin` read is printed to `stderr` using `eprintln!`.
///
/// # Example:
/// ```
/// let input = input_or_stdin(None, 3, 500); // Attempts to read from stdin 3 times with 500ms delay.
/// println!("Input received: {}", input);
///
/// let input_from_arg = input_or_stdin(Some("Hello"), 3, 500); // Directly uses the provided input.
/// println!("Input from arg: {}", input_from_arg);
/// ```
///
/// # Panics:
/// - This function does not panic under normal circumstances. It returns an empty string if
///   `stdin` is unavailable or an error occurs after the allowed number of attempts.
///
/// # Notes:
/// - Useful for cases where input may be delayed, such as reading from a file or pipe.
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

pub fn str_or_stdin(input: Option<&str>, max_attempts: usize, delay: u64) -> String {
    input_or_stdin(input.map(|s| Cursor::new(s.as_bytes())), max_attempts, delay)
}

//pub fn input_or_stdin(input: Option<&str>, max_attempts: usize, delay: u64) -> String {
//	// If input is provided, use it directly
//	if let Some(input_str) = input {
//		return input_str.to_string();
//	}
//
//	// Otherwise, attempt to read from stdin
//	let stdin = io::stdin();
//	let mut handle = stdin.lock();
//	let mut buf = String::new();
//
//	for attempt in 0..max_attempts {
//		match handle.read_to_string(&mut buf) {
//			Ok(0) => {
//				// No data was read, retry after a delay
//				if attempt == max_attempts - 1 {
//					return String::new();  // Give up after max_attempts
//				}
//				sleep(Duration::from_millis(delay));
//			}
//			Ok(_) => {
//				// Successfully read input, trim and return it
//				let result = buf
//					.lines()                    // Split into lines
//					.map(|line| line.trim())    // Trim each line
//					.map(String::from)          // Convert &str to String
//					.collect::<Vec<_>>()        // Collect into a Vec<String>
//					.join("\n");
//
//				return result.to_string();
//			}
//			Err(e) => {
//				// Handle error, retry if it's a WouldBlock error and we have retries left
//				if e.kind() == ErrorKind::WouldBlock && attempt < max_attempts - 1 {
//					sleep(Duration::from_millis(delay));
//				} else {
//					// If not WouldBlock or out of attempts, return empty string
//					return String::new();
//				}
//			}
//		}
//		buf.clear();  // Clear buffer after failed attempt before next retry
//	}
//	
//	// If we exhaust attempts and still have no input, return an empty string
//	String::new()
//}

