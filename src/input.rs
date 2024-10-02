use eyre::{Result, eyre, Context};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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
pub fn binary_file(path: &Path) -> Result<Vec<u8>> {
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
pub fn text_file(path: &Path) -> Result<String> {
	let mut contents = String::new();

	File::open(path)
		.with_context(|| format!("Error opening file '{}'", path.display()))?
		.read_to_string(&mut contents)
		.with_context(|| format!("Error reading file '{}'", path.display()))?;
	
	Ok(contents)
}

/// Represents input data that can be either text (UTF-8) or binary.
pub enum Data {
	Text(String),
	#[allow(dead_code)]
	Binary(Vec<u8>),
}

/// Attempts to read a file as either binary or text data.
pub fn binary_or_text_file(path: &Path) -> Result<Data> {
    // Attempt to read as binary from the path
    if let Ok(binary_data) = binary_file(path) {
        return Ok(Data::Binary(binary_data));
    }

    // If binary read fails, attempt to read as text
    if let Ok(text_data) = text_file(path) {
        return Ok(Data::Text(text_data));
    }

    // If both read attempts fail, return an error
    Err(eyre::eyre!("Failed to read the file '{}' as binary or text", path.display()))
}

/// Attempts to determine if the input data is valid UTF-8 text or binary data.
pub fn binary_or_text<T: AsRef<[u8]>>(data: T) -> Result<Data> {
    // Convert the input into bytes
    let buf = data.as_ref().to_vec();

    // Attempt to interpret the bytes as UTF-8
    if let Ok(text_data) = String::from_utf8(buf.clone()) {
        return Ok(Data::Text(text_data)); // Valid UTF-8, return the String
    } else {
        return Ok(Data::Binary(buf)); // Treat as binary
    }
}

//#[allow(dead_code)]
pub fn read_stdin(max_attempts: usize, timeout: Duration) -> Result<Data> {
    let stdin = Arc::new(Mutex::new(std::io::stdin()));
    let buf = Arc::new(Mutex::new(Vec::new()));
    let (sender, receiver) = std::sync::mpsc::channel();

    for attempt in 0..max_attempts {
        let stdin_clone = Arc::clone(&stdin);
        let buf_clone = Arc::clone(&buf);
        let sender_clone = sender.clone();

        // Spawn a new thread to read from stdin
        thread::spawn(move || {
            let mut handle = stdin_clone.lock().unwrap();
            let mut local_buf = Vec::new();

            // Attempt to read from stdin
            if handle.read_to_end(&mut local_buf).is_ok() {
                sender_clone.send(local_buf).unwrap();
            }
        });

        // Wait for data or timeout
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if let Ok(data) = receiver.recv_timeout(timeout) {
                let mut buffer = buf_clone.lock().unwrap();
                *buffer = data; // Store the received data
                break; // Exit the waiting loop
            }
        }

        // Check if we successfully read data
        let data = buf.lock().unwrap();
        if !data.is_empty() {
            if let Ok(text_data) = String::from_utf8(data.clone()) {
                return Ok(Data::Text(text_data)); // Valid UTF-8, treat as text
            } else {
                return Ok(Data::Binary(data.clone())); // Treat as binary
            }
        }

        // If this was the last attempt, return an error
        if attempt == max_attempts - 1 {
            return Err(eyre!(
                "Timeout: Failed to read input from stdin after {} attempts",
                max_attempts
            ));
        }

        // Optionally sleep before the next attempt
        thread::sleep(Duration::from_millis(100));
    }

    Err(eyre!("Timeout: Unexpected error in read_stdin"))
}

/// Processes input from a string or stdin.
pub fn data_or_stdin<T: AsRef<[u8]>>(input: Option<T>, max_attempts: usize, delay: u64) -> Result<Data> {
    // If input is provided, process it
    if let Some(input_data) = input {
        return binary_or_text(input_data); // Directly pass the input as bytes
    }

    // If input is None, read from stdin
    // Convert delay from u64 milliseconds to Duration
    let timeout = Duration::from_millis(delay);
    read_stdin(max_attempts, timeout)
}

/// Processes input from a file or stdin.
#[allow(dead_code)]
pub fn file_or_stdin(path: Option<&Path>, max_attempts: usize, delay: u64) -> Result<Data> {

    // If path is provided, process it
    if let Some(input_path) = path {
        return binary_or_text_file(input_path); // Directly pass the input as bytes
    }

    // If input is None, read from stdin
    // Convert delay from u64 milliseconds to Duration
    let timeout = Duration::from_millis(delay);
    read_stdin(max_attempts, timeout)
}



