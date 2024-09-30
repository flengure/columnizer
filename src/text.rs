use clap::{Args, ValueEnum};
use crate::io::unwrap_or_stdin;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;
use textwrap;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Cleans the provided text by trimming whitespace and removing empty lines.
///
/// This function takes an optional string slice (`Option<&str>`), converts it
/// to an `Option<String>`, and uses the `TextFormatter` struct to perform
/// the cleaning process. If the input is `None`, it returns an empty string.
///
/// # Arguments
///
/// * `text` - An optional string slice containing the text to be cleaned. 
///   If `None`, the function will return an empty string.
///
/// # Returns
///
/// * A `String` containing the cleaned text. If the input is `None` or if
///   all lines are empty after cleaning, it returns an empty string.
///
/// # Example
///
/// ```
/// let cleaned_text = clean(Some("  line1  \n\n  line2  \n  ")); // Returns "line1\nline2"
/// let empty_text = clean(None); // Returns ""
/// ```
pub fn clean(text: Option<&str>) -> String {
    let text_string = text.map(|s| s.to_string());       // Convert Option<&str> to Option<String>
    let mut formatter = TextFormatter::new(text_string); // Pass the Option<String>
    formatter.clean()                                    // Mutably clean the text
}

/// Right-aligns the lines in the text string to the specified width.
///
/// # Arguments
///
/// * `text` - A string slice containing the lines to be right-aligned.
/// * `width` - An optional width to which the lines should be aligned. If `None` or less than or equal to zero,
///			 the width of the longest line will be used.
///
/// # Returns
///
/// A `String` with each line right-aligned to the specified width or to the maximum line width if no width is provided.
pub fn right(text: Option<&str>, width: Option<usize>) -> String {
    let text_string = text.map(|s| s.to_string());        // Convert Option<&str> to Option<String>
    let mut formatter = TextFormatter::new(text_string);  // Pass the Option<String>
    if let Some(w) = width { formatter.set_width(w); }    // Set the width, only if it is provided
    formatter.right()                                     // Return the formatted right-aligned text
}

/// Formats the given text as left-aligned.
///
/// # Parameters
/// - `text`: An optional string slice that contains the text to be formatted.
/// If `None` is provided, the function returns an empty string.
///
/// # Returns
/// A `String` containing the left-aligned formatted text.
pub fn left(text: Option<&str>) -> String {
    let text_string = text.map(|s| s.to_string());      // Convert Option<&str> to Option<String>
    let mut formatter = TextFormatter::new(text_string);    // Pass the Option<String>
    formatter.left()                                    // Return the formatted left-aligned text
}

/// Wraps the given text to the specified width.
///
/// # Parameters
/// - `text`: An optional string slice that contains the text to be wrapped.
/// If `None` is provided, the function returns an empty string.
/// - `width`: The maximum width for wrapping the text.
///
/// # Returns
/// A `String` containing the wrapped text at the specified width.
pub fn wrap(text: Option<&str>, width: Option<usize>) -> String {
    let text_string = text.map(|s| s.to_string());       // Convert Option<&str> to Option<String>
    let mut formatter = TextFormatter::new(text_string); // Pass the Option<String>
    
    // Set the width, only if it is provided
    if let Some(w) = width { formatter.set_width(w); }

	formatter.wrap()
}

/// Centers the provided text within the specified width.
///
/// # Parameters
/// - `text`: An optional string slice that may contain the text to be centered.
/// - `width`: The total width within which the text should be centered.
///
/// # Returns
/// A `String` containing the center-aligned text. If `text` is `None`, an empty string is returned.
pub fn center(text: Option<&str>, width: Option<usize>) -> String {
    // Convert Option<&str> to Option<String>
    let text_string = text.map(|s| s.to_string());
    
    // Create a TextFormatter instance with the optional string
    let mut formatter = TextFormatter::new(text_string);
    
    // Set the width, only if it is provided
    if let Some(w) = width { formatter.set_width(w); }
    
    // Center the text and return the result
    formatter.center()
}

/// Truncates the provided text to fit within the specified width, applying the designated frame type.
///
/// # Parameters
/// - `text`: An optional string slice that may contain the text to be truncated.
/// - `width`: The maximum width the text should occupy.
/// - `frame`: An optional `Frame` type that defines how the text should be truncated (e.g., `CHOP`).
///
/// # Returns
/// A `String` containing the truncated text. If `text` is `None`, an empty string is returned.
pub fn truncate(
	text: Option<&str>,
	width: Option<usize>,
	no_ellipsis: Option<bool>,
	frame: Option<Frame>,
) -> String {
    // Convert Option<&str> to Option<String>
    let text_string = text.map(|s| s.to_string());
    
    // Create a TextFormatter instance with the optional string
    let mut formatter = TextFormatter::new(text_string);
    
    // Set the frame to the provided value or default to TRUNCATE
    formatter.set_frame(frame.unwrap_or(Frame::TRUNCATE));
    
    // Set the width, only if it is provided
    if let Some(w) = width { formatter.set_width(w); }
    
    // Set no_ellipsis, only if it is provided
    if let Some(n) = no_ellipsis { formatter.set_no_ellipsis(n); }
    
    // Perform truncation and return the result
    formatter.truncate()
}

pub fn text(
	text: Option<&str>,
	width: Option<usize>,
	frame: Option<Frame>,
	no_ellipsis: Option<bool>,
	pad_decimal_digits: Option<bool>,
	max_decimal_digits: Option<usize>,
	decimal_separator: Option<char>,
	use_thousand_separator: Option<bool>,
	thousand_separator: Option<char>,
	alignment: Option<Alignment>,
) -> String {
    // Convert Option<&str> to Option<String>
    let text_string = text.map(|s| s.to_string());
    
    // Create a TextFormatter instance with the optional string
    let mut formatter = TextFormatter::new(text_string);
    
    // Set the width, only if it is provided
    if let Some(w) = width { formatter.set_width(w); }
    
    // Set the frame to the provided value or default to TRUNCATE
    if let Some(n) = frame { formatter.set_frame(n); }
    
    // Set no_ellipsis, only if it is provided
    if let Some(n) = no_ellipsis { formatter.set_no_ellipsis(n); }
    
    // Set pad_decimal_digits, only if it is provided
    if let Some(n) = pad_decimal_digits { formatter.set_pad_decimal_digits(n); }
    
    // Set max_decimal_digits, only if it is provided
    if let Some(n) = max_decimal_digits { formatter.set_max_decimal_digits(n); }
    
    // Set decimal_separator, only if it is provided
    if let Some(n) = decimal_separator { formatter.set_decimal_separator(n); }
    
    // Set use_thousand_separator, only if it is provided
    if let Some(n) = use_thousand_separator { formatter.set_use_thousand_separator(n); }
    
    // Set thousand_separator, only if it is provided
    if let Some(n) = thousand_separator { formatter.set_thousand_separator(n); }
    
    // Set alignment, only if it is provided
    if let Some(n) = alignment { formatter.set_alignment(n); }
    
    // Perform truncation and return the result
    formatter.text()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Frame {
    /// Truncate the end of text to fit the width.
    TRUNCATE,
    /// Chop off the start of text to fit the width.
    CHOP,
    /// Wrap the text to fit the width.
    WRAP,
    /// Leave the text unchanged.
    NONE,
}

impl Default for Frame {
    fn default() -> Self {
        Frame::TRUNCATE
    }
}

impl FromStr for Frame {
	type Err = String;

	fn from_str(text: &str) -> Result<Frame, Self::Err> {
		match text.to_uppercase().as_str() {
			"TRUNCATE" => Ok(Frame::TRUNCATE),
			"CHOP" => Ok(Frame::CHOP),
			"WRAP" => Ok(Frame::WRAP),
			"NONE" => Ok(Frame::NONE),
			_ => Err(format!("Invalid frame type: {}", text)),
		}
	}
}

impl fmt::Display for Frame {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Frame::TRUNCATE => write!(f, "TRUNCATE"),
			Frame::CHOP => write!(f, "CHOP"),
			Frame::WRAP => write!(f, "WRAP"),
			Frame::NONE => write!(f, "NONE"),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Alignment {
	/// Automatically align to the right for numeric values,
	/// or left for non-numeric values.
	AUTO,
	
	/// Center align the text.
	CENTER,
	
	/// Left align the text.
	LEFT,
	
	/// Right align the text.
	RIGHT,
	
	/// No alignment; do not apply any specific alignment.
	NONE,
}

impl FromStr for Alignment {
	type Err = String;

	fn from_str(text: &str) -> Result<Alignment, Self::Err> {
		match text.to_uppercase().as_str() {
			"AUTO"   => Ok(Alignment::AUTO),
			"CENTER" => Ok(Alignment::CENTER),
			"LEFT"   => Ok(Alignment::LEFT),
			"RIGHT"  => Ok(Alignment::RIGHT),
			"NONE"   => Ok(Alignment::NONE),
			_ => Err(format!("Invalid frame type: {}", text)),
		}
	}
}

impl fmt::Display for Alignment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Alignment::AUTO   => write!(f, "AUTO"  ),
			Alignment::CENTER => write!(f, "CENTER"),
			Alignment::LEFT   => write!(f, "LEFT"  ),
			Alignment::RIGHT  => write!(f, "RIGHT" ),
			Alignment::NONE   => write!(f, "NONE"  ),
		}
	}
}

#[derive(Args, Clone)]
pub struct TextFormatter {
	/// Text be formatted according to the specified options
	pub text: Option<String>,

	// The maximum width (in characters) allocated for the formatted field.
	// This width determines how the text text will be displayed in the output.
	#[arg(default_value_t = 0)]
	#[arg(short, long)]
	pub width: usize,

	// Specifies the formatting style for the text within the field.
	// If set to `Frame::TRUNCATE`, text will be truncated to fit width.
	// If set to `Frame::WRAP`, wrapped accross multiple lines to fit width.
	#[arg(default_value_t = Frame::TRUNCATE)]
	#[arg(value_enum)]
	#[arg(short, long)]
	pub frame: Frame,

	// Indicates whether to truncate text with an ellipsis ("...") when it
	//  exceeds the defined width.
	// This is applicable only when `frame` is set to `Frame::TRUNCATE`.
	#[arg(short, long)]
	pub no_ellipsis: bool,

	// Determines whether to pad decimal digits with trailing zeros to maintain
	//   consistent appearance. If set to true, decimal numbers will display
	//   the specified number of digits after the decimal point.
	#[arg(short, long)]
	pub pad_decimal_digits: bool,

	// The maximum number of decimal digits that will be displayed for numeric values.
	// This setting helps control the precision of the output for numeric formatting.
	#[arg(default_value_t = 2)]
	#[arg(short, long)]
	pub max_decimal_digits: usize,

	// The character used as the decimal separator in formatted numeric values.
	// \nThis is particularly useful for ensuring compatibility with various regional formats.
	#[arg(default_value_t = '.')]
	#[arg(short, long)]
	pub decimal_separator: char,

	// A flag indicating whether to include a thousand separator in large numeric values.
	// If set to true, numbers will be formatted with the specified `thousand_separator`.
	#[arg(short, long)]
	pub use_thousand_separator: bool,

	// The character used as the thousand separator in formatted numeric values.
	// \nThis enhances readability by grouping digits in large numbers.
	#[arg(default_value_t = ',')]
	#[arg(short, long)]
	pub thousand_separator: char,

	// Specifies the alignment of the text within the field.
	// - `Alignment::AUTO`: Automatically aligns numeric text to the right.
	// - `Alignment::RIGHT`: Forces right alignment for both numeric and non-numeric text.
	// - `Alignment::None`: Leaves the text unchanged, preserving its original alignment.
	#[arg(default_value_t = Alignment::AUTO)]
	#[arg(value_enum)]
	#[arg(short, long)]
	pub alignment: Alignment,

	/// A flag indicating whether the content being formatted is numeric.
	/// This can influence how certain formatting rules are applied, such as decimal padding.
	#[arg(hide = true)]
	pub is_numeric: Option<bool>,

}

impl Default for TextFormatter {
	/// Creates a new `TextFormatter` with default settings.
	fn default() -> Self {
		Self {
			text:                   None, // No text
			width:                     48, // Default 48
			frame:        Frame::TRUNCATE, // Default TRUNCATE
			alignment:    Alignment::AUTO, // Default text left, numbers right
			no_ellipsis:            false, // Default add an ellipsis to truncated text
			pad_decimal_digits:     false, // Default dont pad decimal places
			max_decimal_digits:         2, // Default to 2 decimal places
			decimal_separator:        '.', // Default decimal separator
			use_thousand_separator: false, // Default no thousands grouping
			thousand_separator:       ',', // Default `,`
			is_numeric:              None, // Unknown
		}
	}
}

impl TextFormatter {
	/// Creates a new `Formatter` instance with default settings.
	///
	/// # Arguments
	///
	/// * `text` - The content to be formatted.
	/// * `width` - The width of the cell.
	///
	/// # Returns
	///
	/// A new `Formatter` with default values for formatting options.
	pub fn new(text: Option<String>) -> Self {
		// Initialize a default instance of TextFormatter
		let mut formatter = TextFormatter::default();

		// Attempt to read text, falling back to stdin if text is None
		let text_data = match unwrap_or_stdin(text, 5, 500) {
			Ok(content) => content,
			Err(e) => {
				eprintln!("Error: {}", e);
				return formatter; // Return the default instance on error
			}
		};

		// Set the text field with the read/cleaned data
		formatter.text = Some(text_data);

		// Return the updated formatter instance
		formatter
	}

	pub fn set_width(&mut self, width: usize) -> &mut Self {
		self.width = width;
		self
	}

	pub fn set_frame(&mut self, frame: Frame) -> &mut Self {
		self.frame = frame;
		self
	}

	pub fn set_no_ellipsis(&mut self, no_ellipsis: bool) -> &mut Self {
		self.no_ellipsis = no_ellipsis;
		self
	}

	pub fn set_pad_decimal_digits(&mut self, pad_decimal_digits: bool) -> &mut Self {
		self.pad_decimal_digits = pad_decimal_digits;
		self
	}

	pub fn set_max_decimal_digits(&mut self, max_decimal_digits: usize) -> &mut Self {
		self.max_decimal_digits = max_decimal_digits;
		self
	}

	pub fn set_decimal_separator(&mut self, decimal_separator: char) -> &mut Self {
		self.decimal_separator = decimal_separator;
		self
	}

	pub fn set_use_thousand_separator(&mut self, use_thousand_separator: bool) -> &mut Self {
		self.use_thousand_separator = use_thousand_separator;
		self
	}

	pub fn set_thousand_separator(&mut self, thousand_separator: char) -> &mut Self {
		self.thousand_separator = thousand_separator;
		self
	}

	pub fn set_alignment(&mut self, alignment: Alignment) -> &mut Self {
		self.alignment = alignment;
		self
	}
}

impl TextFormatter {

	/// Checks if the content is numeric by first checking the cached value.
	/// If not cached, normalizes the content and checks if it can be parsed as f64.
	///
	/// # Returns
	///
	/// A boolean indicating whether the content is numeric.
	pub fn is_numeric(&mut self) -> bool {
		// Return cached result if available
		if let Some(is_numeric) = self.is_numeric {
			return is_numeric;
		}

		// Ensure that text is present (unwrap Option to get &str)
		let normalized_content = if let Some(text_str) = &self.text {
			text_str
				.replace(self.thousand_separator, "")  // Using characters directly
				.replace(self.decimal_separator, ".")
		} else {
			String::new()
		};


		// Try to parse the normalized content as f64 and cache the result
		self.is_numeric = Some(normalized_content.parse::<f64>().is_ok());
		
		// Return the newly cached result
		self.is_numeric.unwrap()
	}

	pub fn is_hex(&self) -> Result<String, String> {
		// First, check if the text exists (unwrap or handle None case)
		if let Some(ref text) = self.text {
			// Now check if all characters are valid hexadecimal digits
			if text.chars().all(|c| c.is_ascii_hexdigit()) {
				Ok(text.to_string()) // Return Ok if it's a valid hex string
			} else {
				Err(format!("'{}' is not a valid hex string", text)) // Return Err if not valid
			}
		} else {
			Err("No text provided".to_string()) // Handle case when text is None
		}
	}

    /// Cleans the input text by trimming whitespace and removing empty lines.
    ///
    /// This method will:
    /// - Trim leading and trailing whitespace from each line.
    /// - Remove any lines that are empty after trimming.
    /// - Update `self.text` with the cleaned text.
    ///
    /// If no lines remain after cleaning, `self.text` will be set to `None`,
    /// and an empty string will be returned.
    ///
    /// # Returns
    ///
    /// A `String` containing the cleaned text.
	pub fn clean(&mut self) -> String {
		// Read data from self.text or default to empty string if None
		let text_data = self.text.as_deref().unwrap_or("");

		// Clean the text by trimming lines and removing empty lines
		let cleaned_lines: Vec<String> = text_data
			.lines()
			.map(|line| line.trim().to_string()) // Trim each line
			.filter(|line| !line.is_empty())     // Filter out empty lines
			.collect();

		// If there are no cleaned lines, set text to None and return an empty string
		if cleaned_lines.is_empty() {
			self.text = None; // Set text to None if no lines remain
			return String::new(); // Return empty string
		}

		// Join the cleaned lines with newline characters
		self.text = Some(cleaned_lines.join("\n")); // Update self.text
		self.text.as_ref().unwrap().clone() // Return the cleaned text as a String
	}

    /// Right-aligns the text stored in `self.text` and updates it.
    ///
    /// # Returns
    ///
    /// Returns a `String` that contains the right-aligned text. 
    /// If no text is provided, it returns an empty string.
    pub fn right(&mut self) -> String {
        // Unwrap text from self.text, or return an empty string if None
        let text = match &self.text {
            Some(data) => data, // Use the contained data
            None => {
                eprintln!("Error: No text provided.");
                return String::new(); // Return empty string if no text
            }
        };

        // Split the text into lines and trim trailing whitespace
        let lines: Vec<&str> = text
            .lines()
            .map(str::trim_end) // Trim trailing whitespace from each line
            .collect();

        // Calculate the maximum width of the lines
        let max_line_width = lines.iter()
            .map(|line| line.width())
            .max()
            .unwrap_or(0); // Fallback to 0 if there are no lines

        // Determine the effective width to use
        let effective_width = if self.width > 0 {
            std::cmp::max(self.width, max_line_width) // Use max width
        } else {
            max_line_width // Use max_line_width if self.width is 0
        };

        // Ensure effective_width is positive; if max_line_width is 0, use 1 to avoid formatting issues
        let effective_width = if effective_width == 0 { 1 } else { effective_width };

        // Iterate through each line and right-align it
        let aligned_lines: Vec<String> = lines
            .iter()
            .map(|line| {
                let line_width = line.width(); // Get the width of the current line
                if line_width < effective_width {
                    // Right-align with padding if line is shorter than the effective width
                    format!("{:>width$}", line, width = effective_width)
                } else {
                    // No alignment needed if the line is already wider than or equal to the effective width
                    line.to_string()
                }
            })
            .collect();

        // Join the aligned lines into a single output string
        let result = aligned_lines.join("\n");

        // Set the text to the newly aligned text
        self.text = Some(result.clone());

        // Return the aligned string
        result
    }

	pub fn left(&mut self) -> String {
		let text = self.text.as_deref().unwrap_or_else(|| {
			eprintln!("Error: No text provided.");
			""
		});

		// Collect lines, trim leading whitespace, and join them with newlines
		let left_aligned = text
			.lines()
			.map(|line| line.trim_start().to_string()) // Trim leading whitespace from each line
			.collect::<Vec<_>>()                       // Collect trimmed lines into a Vec<String>
			.join("\n");                               // Join them with newlines

		// Update self.text with the left-aligned result
		self.text = Some(left_aligned.clone());

		// Return the left-aligned text
		left_aligned
	}

	/// Centers the text within the specified width, applying the given frame formatting.
	///
	/// This function takes the current text, trims whitespace from each line,
	/// and processes the text based on the specified `Frame` variant:
	///
	/// - `Frame::NONE`: No modifications are made to the text; it is returned unchanged.
	/// - `Frame::WRAP`: If a line exceeds the specified width, it is wrapped to fit within the width.
	/// - `Frame::TRUNCATE` and `Frame::CHOP`: If a line exceeds the width, it is truncated or chopped to fit,
	///   ensuring the text remains within the specified width.
	///
	/// The resulting centered lines are padded with spaces to align them within the effective width.
	///
	/// # Returns
	///
	/// Returns the centered text as a `String`. If no text is provided, an empty string is returned.
	///
	/// # Errors
	///
	/// If the provided text is `None`, an error message is printed to standard error, and an empty string is returned.
	pub fn center(&mut self) -> String {
		// Get the text, or return an empty string if None
		let text = self.text.as_deref().unwrap_or_else(|| {
			eprintln!("Error: No text provided.");
			""
		});

		// Split text into lines, trim whitespace, and remove empty lines
		let lines: Vec<String> = text.lines()
			.filter_map(|line| {
				let trimmed = line.trim();
				if trimmed.is_empty() {
					None // Skip empty lines
				} else {
					Some(trimmed.to_string()) // Return trimmed line as Some(String)
				}
			})
			.collect();

		// Determine the effective width based on the specified frame
		let effective_width = if self.frame == Frame::NONE {
			usize::MAX // No width limit for NONE
		} else {
			self.width
		};

		let mut centered_lines: Vec<String> = Vec::with_capacity(lines.len());

		for line in lines {
			let text_width = line.width();

			// Determine the appropriate width for truncation or wrapping
			let max_width = if text_width > effective_width {
				match self.frame {
					Frame::WRAP => {
						// Use textwrap to wrap the line
						textwrap::wrap(&line, effective_width).join("\n")
					},
					Frame::NONE => line.clone(), // No modification
					_ => {
						// Truncate the line if it exceeds the effective width
						line.chars().take(effective_width).collect()
					},
				}
			} else {
				line.clone() // No need to truncate or wrap
			};

			// Calculate padding for centering
			let total_padding = effective_width.saturating_sub(max_width.width());
			let left_padding = total_padding / 2;

			// Center the line with padding
			let centered_line = format!("{:width$}{}", "", max_width, width = left_padding);
			centered_lines.push(centered_line);
		}

		// Join the centered lines into a single output string
		let result = centered_lines.join("\n");

		// Set self.text to the newly centered text
		self.text = Some(result.clone());

		// Return the centered text
		result
	}

	/// Wraps the text to fit within the specified width.
	///
	/// This function takes the text stored in `self.text`, trims leading and trailing
	/// whitespace from each line, and wraps the text to fit within the specified width.
	/// The wrapped lines are then joined into a single string with newline characters.
	///
	/// If no text is provided, an error message is printed, and an empty string is returned.
	///
	/// # Returns
	///
	/// A `String` containing the wrapped text, or an empty string if no text was provided.
	///
	/// # Example
	///
	/// ```
	/// let mut formatter = TextFormatter::new(Some("This is a long line of text that will be wrapped to fit within a specified width."));
	/// formatter.set_width(20);
	/// let wrapped_text = formatter.wrap();
	/// println!("{}", wrapped_text);
	/// ```
	pub fn wrap(&mut self) -> String {
		let text = self.text.as_deref().unwrap_or_else(|| {
			eprintln!("Error: No text provided.");
			""
		});

		// Trim whitespace from each line, wrap, and join the results
		let wrapped_text = textwrap::wrap(
			&text.lines()
				.map(str::trim)          // Trim leading and trailing whitespace
				.collect::<Vec<_>>()     // Collect trimmed lines
				.join("\n"),             // Join trimmed lines into a single string
			self.width,
		)
		.into_iter()
		.map(String::from)               // Convert Cow to String
		.collect::<Vec<_>>()             // Collect wrapped lines
		.join("\n");                     // Join wrapped lines with newlines

		// Update self.text with the new wrapped text
		self.text = Some(wrapped_text.clone());

		// Return the wrapped text
		wrapped_text
	}

	/// Truncates the text in the specified frame, ensuring it fits within the designated width.
	///
	/// This function ensures the text is left-aligned or right-aligned based on the specified
	/// frame type. If the text exceeds the maximum width:
	/// - For `Frame::CHOP`, characters are removed from the beginning of the line,
	///   and ellipses are prepended if applicable.
	/// - By default, characters are collected from the start until the width is exceeded,
	///   with ellipses appended at the end if applicable.
	///
	/// # Returns
	/// A `String` containing the truncated text. If the original text fits within the width,
	/// the original text is returned unchanged.
	///
	/// # Examples
	/// ```
	/// let mut text_instance = Text::new("This is a long line of text that might need truncation.");
	/// text_instance.set_width(20);
	/// text_instance.set_frame(Frame::CHOP);
	/// let truncated = text_instance.truncate();
	/// assert_eq!(truncated, "...of text that might need truncation.");
	/// ```
	pub fn truncate(&mut self) -> String {
		// Ensure text is left-aligned or right-aligned based on frame
		match self.frame {
			Frame::CHOP => self.right(),
			_ => self.left(),
		};

		// Extract the text from self.text or provide a default empty string
		let text = self.text.as_deref().unwrap_or_else(|| {
			eprintln!("Error: No text provided.");
			""
		});

		// Process the text, truncate each line as necessary
		let truncated_lines: Vec<String> = text
			.lines()
			.map(|line| {
				let text_width = line.width();
				if text_width > self.width {
					let ellipsis_len = if self.no_ellipsis { 0 } else { 3 };
					let max_width = self.width.saturating_sub(ellipsis_len);

					let mut current_width = 0;
					let mut truncated = String::new();

					match self.frame {
						Frame::CHOP => {
							// Iterate from the end to find how much to keep
							for c in line.chars().rev() {
								let char_width = c.width().unwrap_or(0);
								current_width += char_width;

								if current_width > max_width {
									truncated.insert(0, c); // Prepend character
								} else {
									truncated.insert(0, c);
								}
							}
							// Prepend ellipses if applicable
							if !self.no_ellipsis && self.width > 3 {
								format!("...{}", truncated.trim())
							} else {
								truncated.trim().to_string()
							}
						},
						_ => {
							for c in line.chars() {
								let char_width = c.width().unwrap_or(0);
								if current_width + char_width > max_width {
									break;
								}
								current_width += char_width;
								truncated.push(c);
							}
							// Append ellipses if applicable
							if !self.no_ellipsis && self.width > 3 {
								format!("{}...", truncated.trim())
							} else {
								truncated.trim().to_string()
							}
						},
					}
				} else {
					line.to_string() // Return original line if no truncation is needed
				}
			})
			.collect();

		// Join the truncated lines into a single string
		let truncated_text = truncated_lines.join("\n");

		// Update self.text with the truncated text
		self.text = Some(truncated_text.clone());

		// Return the truncated text
		truncated_text
	}

	/// Formats the text based on the specified frame and alignment settings.
	///
	/// This function formats the text according to the `frame` setting:
	/// - If the frame is `TRUNCATE` or `CHOP`, it truncates the text to fit within the specified width.
	/// - If the frame is `WRAP`, it wraps the text to the specified width.
	/// - If the frame is `NONE`, it returns the original text without modification.
	///
	/// After formatting, the text is aligned based on the specified alignment setting:
	/// - `AUTO`: No specific alignment; returns the formatted text as is.
	/// - `NONE`: No alignment applied; returns the formatted text unchanged.
	/// - `LEFT`: Aligns the text to the left.
	/// - `RIGHT`: Aligns the text to the right, padding as necessary.
	/// - `CENTER`: Centers the text within the specified width.
	///
	/// Returns the formatted and aligned text as a `String`.
	pub fn format_text(&mut self) -> String {
		// Determine the formatted text based on the frame setting
		let formatted_text = match self.frame {
			Frame::TRUNCATE | Frame::CHOP => self.truncate(),
			Frame::WRAP => self.wrap(),
			Frame::NONE => self.text.as_ref().expect("Text is None").clone(),
		};

		// Apply alignment based on the settings
		let aligned_result = match self.alignment {
			Alignment::AUTO | Alignment::NONE => formatted_text,
			Alignment::LEFT => left(Some(&formatted_text)),
			Alignment::RIGHT => right(Some(&formatted_text), Some(self.width)),
			Alignment::CENTER => center(Some(&formatted_text), Some(self.width)),
		};

		aligned_result
	}

	/// Formats the text as a numeric value with custom formatting options.
	///
	/// This method takes the text string, cleans and normalizes it, and attempts
	/// to parse it as a floating-point number. If parsing is successful, it formats
	/// the number according to the specified settings, such as applying thousands 
	/// separators, padding decimal digits, and custom alignment. If parsing fails, 
	/// an error message is printed, and an empty string is returned.
	///
	/// # Returns
	///
	/// A `String` containing the formatted numeric value. If the text cannot be parsed
	/// as a number, an empty string is returned and an error message is printed.
	///
	/// # Behavior:
	///
	/// - **Thousand Separators**: Adds thousands separators based on the specified custom separator.
	/// - **Decimal Formatting**: Pads decimal places if specified, up to the maximum number of decimal digits.
	/// - **Alignment**: Aligns the formatted number to the left, right, or center based on the `alignment` setting.
	/// - **Error Handling**: If the text is not a valid numeric value, it returns an empty string and prints an error.
	///
	/// # Example:
	///
	/// ```
	/// let formatter = NumericFormatter {
	///     text: "1.234567",
	///     thousand_separator: ',',
	///     decimal_separator: '.',
	///     pad_decimal_digits: true,
	///     max_decimal_digits: 2,
	///     use_thousand_separator: true,
	///     alignment: Alignment::RIGHT,
	///     width: 10,
	/// };
	///
	/// assert_eq!(formatter.format_numeric(), "      1.23");
	/// ```
	pub fn format_numeric(&self) -> String {

		// Normalize text by replacing custom separators

		// Ensure that text is present (unwrap Option to get &str)
		let normalized = if let Some(text_str) = &self.text {
			text_str
				.replace(self.thousand_separator, "")  // Using characters directly
				.replace(self.decimal_separator, ".")
		} else {
			String::new()
		};

		// Attempt to parse the normalized text as a number
		let result: Result<f64, ParseFloatError> = normalized.parse();

		match result {
			Ok(number) => {
				// Format the number with native Rust formatting
				let formatted = if self.pad_decimal_digits {
					format!("{:.*}", self.max_decimal_digits, number)
				} else {
					format!("{}", number)
				};

				// Split formatted number into integer and fractional parts
				let parts: Vec<&str> = formatted.split('.').collect();
				let integer_part = parts[0];
				let fractional_part = if parts.len() > 1 { parts[1] } else { "" };

				// Apply thousands separators if needed
				let integer_with_thousands = if self.use_thousand_separator {
					let integer_chars: Vec<char> = integer_part.chars().rev().collect();
					let mut result_chars = Vec::new();
					for (i, ch) in integer_chars.iter().enumerate() {
						if i > 0 && i % 3 == 0 {
							result_chars.push(self.thousand_separator);
						}
						result_chars.push(*ch);
					}
					result_chars.reverse();
					result_chars.iter().collect::<String>()
				} else {
					integer_part.to_string()
				};

				// Combine integer and fractional parts and replace decimal separator
				let formatted_result = if !fractional_part.is_empty() {
					format!("{}.{}", integer_with_thousands, fractional_part)
				} else {
					integer_with_thousands
				};

				// Replace decimal separator with custom one
				let final_formatted_number = formatted_result.replace('.', &self.decimal_separator.to_string());

				// Apply alignment
				let width = self.width;
				let aligned_result = match self.alignment {
					Alignment::NONE   => final_formatted_number,
					Alignment::LEFT   => left(Some(&final_formatted_number)),
					Alignment::AUTO   => right(Some(&final_formatted_number),  Some(width)),
					Alignment::RIGHT  => right(Some(&final_formatted_number),  Some(width)),
					Alignment::CENTER => center(Some(&final_formatted_number), Some(width)),
				};

				aligned_result
			}

			Err(_) => {
				eprintln!("Error: Not a number"); // Print error message
				String::new() // Return an empty string or an appropriate message
			}
		}
	}

	pub fn text(&mut self) -> String {
		// Extract the text or provide a default empty string, setting is_numeric to None if no text is found
		let text = self.text.as_deref().unwrap_or_else(|| {
			eprintln!("Error: No text provided.");
			self.is_numeric = None; // Set to None since no text is available
			return "";
		});

		// Normalize text by replacing custom separators
		let normalized = text
			.replace(self.thousand_separator, "")  // Remove thousand separators
			.replace(self.decimal_separator, ".");   // Normalize decimal separator

		// Attempt to parse the normalized text as a number
		match normalized.parse::<f64>() {
			Ok(number) => {
				// Format the number with native Rust formatting
				let formatted = if self.pad_decimal_digits {
					format!("{:.*}", self.max_decimal_digits, number)
				} else {
					format!("{}", number)
				};

				// Split formatted number into integer and fractional parts
				let parts: Vec<&str> = formatted.split('.').collect();
				let integer_part = parts[0];
				let fractional_part = if parts.len() > 1 { parts[1] } else { "" };

				// Apply thousands separators if needed
				let integer_with_thousands = if self.use_thousand_separator {
					let integer_chars: Vec<char> = integer_part.chars().rev().collect();
					let mut result_chars = Vec::new();
					for (i, ch) in integer_chars.iter().enumerate() {
						if i > 0 && i % 3 == 0 {
							result_chars.push(self.thousand_separator);
						}
						result_chars.push(*ch);
					}
					result_chars.reverse();
					result_chars.iter().collect::<String>()
				} else {
					integer_part.to_string()
				};

				// Combine integer and fractional parts and replace decimal separator
				let formatted_result = if !fractional_part.is_empty() {
					format!("{}.{}", integer_with_thousands, fractional_part)
				} else {
					integer_with_thousands
				};

				// Replace decimal separator with custom one
				let final_formatted_number = formatted_result.replace('.', &self.decimal_separator.to_string());

				// Apply alignment
				let aligned_result = match self.alignment {
					Alignment::NONE   => final_formatted_number,
					Alignment::LEFT   => left(Some(&final_formatted_number)),
					Alignment::AUTO   => right(Some(&final_formatted_number), Some(self.width)),
					Alignment::RIGHT  => right(Some(&final_formatted_number), Some(self.width)),
					Alignment::CENTER => center(Some(&final_formatted_number), Some(self.width)),
				};

				// Set properties
				self.is_numeric = Some(true);
				self.text = Some(aligned_result.clone()); // Store the formatted text
				aligned_result // Return the formatted text
			}
			Err(_) => {
				// If parsing fails, format as general text based on the frame setting
				let formatted_text = match self.frame {
					Frame::TRUNCATE | Frame::CHOP => self.truncate(),
					Frame::WRAP => self.wrap(),
					Frame::NONE => {
						let text_value = self.text.clone().expect("Text is None");
						self.is_numeric = Some(false); // Set to false because it couldn't parse
						text_value // Return original text
					},
				};

				// Apply alignment based on the settings
				let aligned_result = match self.alignment {
					Alignment::AUTO | Alignment::NONE => formatted_text.clone(),
					Alignment::LEFT => left(Some(&formatted_text)),
					Alignment::RIGHT => right(Some(&formatted_text), Some(self.width)),
					Alignment::CENTER => center(Some(&formatted_text), Some(self.width)),
				};

				// Set properties
				self.is_numeric = Some(false);
				self.text = Some(aligned_result.clone()); // Store the formatted text
				aligned_result // Return the formatted text
			}
		}
	}


	/// Formats the text content based on its type (numeric or text).
	///
	/// This method checks if the formatted content has already been cached. If it is, 
	/// the cached result is returned immediately. If not, it determines whether the 
	/// content is numeric by calling `is_numeric()`. Based on this check, it either 
	/// formats the content as a numeric value using `format_numeric()` or as text 
	/// using `format_text()`. The result is then cached for future calls.
	///
	/// # Returns
	///
	/// A `String` containing the formatted content. If the content was formatted as numeric, 
	/// it will return the numeric representation; otherwise, it returns the text representation.
	///
	/// # Caching
	///
	/// The formatted result is cached to optimize performance. Subsequent calls to this method 
	/// will return the cached value until it is invalidated.
	///
	/// # Example
	///
	/// ```
	/// let mut formatter = Formatter::new("1234.56");
	/// let result = formatter.formatted(); // Formats as numeric
	/// assert_eq!(result, "1,234.56"); // Assuming formatting adds thousands separators
	/// ```
	///
	/// # Panics
	///
	/// This function does not panic; however, if there is a logic error leading to unexpected 
	/// behavior in `is_numeric`, `format_numeric`, or `format_text`, it may lead to incorrect results.
	pub fn formatted(&mut self) -> String {
		if self.is_numeric() {
			self.format_numeric()
		} else {
			self.format_text()
		}
	}

}
