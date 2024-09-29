use clap::{Args, ValueEnum};
use crate::io::input_or_stdin;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;
use textwrap;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Cleans the input by trimming whitespace and removing empty lines.
///
/// This function reads input data either from the provided option or from stdin.
/// After reading, it trims whitespace from each line and filters out any empty lines.
///
/// # Parameters
/// - `input`: An optional string input. If `None`, it reads from stdin.
///
/// # Returns
/// A `String` containing the cleaned input data, with leading/trailing whitespace 
/// removed from each line and empty lines filtered out.
pub fn clean(input: Option<&str>) -> String {

	// Read data from stdin if input is None
	let input_data = input_or_stdin(input, 5, 500);

	// Clean the input by trimming lines and removing empty lines
	let cleaned_lines: Vec<String> = input_data
		.lines()
		.map(|line| line.trim().to_string()) // Trim each line
		.filter(|line| !line.is_empty())	 // Filter out empty lines
		.collect();

	// If there are no cleaned lines, return an empty string
	if cleaned_lines.is_empty() {
		return String::new();
	}

	// Join the cleaned lines with newline characters
	cleaned_lines.join("\n")
}

/// Right-aligns the lines in the input string to the specified width.
///
/// # Arguments
///
/// * `input` - A string slice containing the lines to be right-aligned.
/// * `width` - An optional width to which the lines should be aligned. If `None` or less than or equal to zero,
///			 the width of the longest line will be used.
///
/// # Returns
///
/// A `String` with each line right-aligned to the specified width or to the maximum line width if no width is provided.
#[allow(dead_code)]
pub fn right(input: Option<&str>, width: Option<usize>) -> String {

	let input_data = input_or_stdin(input, 5, 500);

	let cleaned = clean(Some(&input_data));

	// Split the input into lines
	let lines: Vec<&str> = cleaned.lines().collect();
	
	// Calculate the maximum width of the lines
	let max_line_width = lines.iter()
		.map(|line| line.width())
		.max()
		.unwrap_or(0); // Fallback to 0 if there are no lines

	// Determine the effective width to use
	let effective_width = match width {
		Some(w) if w > 0 => std::cmp::max(w, max_line_width),
		_ => max_line_width,
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
	let joined_lines = aligned_lines.join("\n");
	
	joined_lines
}

#[allow(dead_code)]
pub fn left(input: Option<&str>) -> String {

	let input_data = input_or_stdin(input, 5, 500);

	let cleaned = clean(Some(&input_data));

	let aligned_lines: Vec<String> = cleaned
		.lines()
		.map(String::from)
		.collect();

	aligned_lines.join("\n")
}
#[allow(dead_code)]
pub fn wrap(input: Option<&str>, width: usize) -> String {

	let input_data = input_or_stdin(input, 5, 500);

	let cleaned = clean(Some(&input_data));

	let wrapped = textwrap::wrap(&cleaned, width);
    // Convert Vec<Cow<'_, str>> to String by joining the wrapped lines with newlines
    wrapped
        .into_iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n")

}

#[allow(dead_code)]
pub fn center(input: Option<&str>, width: Option<usize>) -> String {

	let input_data = input_or_stdin(input, 5, 500);

	let cleaned = clean(Some(&input_data));

	// Split the input into lines
	let lines: Vec<&str> = cleaned.lines().collect();

	// Calculate the maximum width of the lines
	let max_line_width = lines.iter()
		.map(|line| line.width())
		.max()
		.unwrap_or(0); // Fallback to 0 if there are no lines

	// Determine the effective width to use
	let effective_width = match width {
		Some(w) if w > 0 => std::cmp::max(w, max_line_width),
		_ => max_line_width,
	};

    // Center each line based on the effective width
    let centered_lines: Vec<String> = lines.iter().map(|line| {
        // Calculate the total padding needed for the current line
        let total_padding = if effective_width > line.width() {
            effective_width - line.width()
        } else {
            0 // No padding needed if the line is wider than or equal to the width
        };

        // Calculate left and right padding
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;

        // Create the centered line with the appropriate padding
        format!("{}{}{}", " ".repeat(left_padding), line, " ".repeat(right_padding))
    }).collect();

    // Join the centered lines into a single output string
    centered_lines.join("\n")

}

/// Truncates each line in the input string to the specified width and optionally adds ellipses.
///
/// # Arguments
///
/// * `input` - A string slice containing the lines to be truncated.
/// * `width` - The maximum width to which each line should be truncated.
/// * `no_ellipsis` - An optional boolean indicating whether to add ellipses (`...`) to truncated lines. If `None` or `false`, ellipses will be added.
///
/// # Returns
///
/// A `String` with each line truncated to the specified width, optionally followed by ellipses if the line was truncated.
#[allow(dead_code)]
pub fn truncate(input: Option<&str>, width: Option<usize>, no_ellipsis: Option<bool>) -> String {

	let input_data = input_or_stdin(input, 5, 500);

	let cleaned = clean(Some(&input_data));

	let width = width.unwrap_or(0);

	// Determine if ellipsis should be used
	let use_ellipsis = no_ellipsis.unwrap_or(false) == false;

	// Split input into lines and process each line
	let truncated_lines: Vec<String> = cleaned
		.lines()
		.map(|line| {
			let text_width = line.width();
			if text_width > width {
				let mut current_width = 0;
				let mut truncated = String::new();
				let ellipsis_len = if use_ellipsis { 3 } else { 0 };
				let max_width = width.saturating_sub(ellipsis_len);

				for c in line.chars() {
					let char_width = c.width().unwrap_or(0);
					if current_width + char_width > max_width {
						break;
					}
					current_width += char_width;
					truncated.push(c);
				}

				// Add ellipsis if applicable
				if use_ellipsis && width > 3 {
					format!("{}...", truncated.trim())
				} else {
					truncated.trim().to_string() // Convert to String for uniform return type
				}
			} else {
				line.to_string()                 // If not truncated, return the original line as a String
			}
		})
		.collect();

	// Join the truncated lines into a single output string
	truncated_lines.join("\n")
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Frame {
	/// Shorten the text to fit the width
	TRUNCATE,
	/// Wrap the text to fit the width
	WRAP,
	/// Leave the text unchanged
	NONE,
}

impl FromStr for Frame {
	type Err = String;

	fn from_str(input: &str) -> Result<Frame, Self::Err> {
		match input.to_uppercase().as_str() {
			"TRUNCATE" => Ok(Frame::TRUNCATE),
			"WRAP" => Ok(Frame::WRAP),
			"NONE" => Ok(Frame::NONE),
			_ => Err(format!("Invalid frame type: {}", input)),
		}
	}
}

impl fmt::Display for Frame {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Frame::TRUNCATE => write!(f, "TRUNCATE"),
			Frame::WRAP => write!(f, "WRAP"),
			Frame::NONE => write!(f, "NONE"),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Alignment {
	/// Align to the right if numeric
	AUTO,
	CENTER,
	LEFT,
	RIGHT,
}

impl FromStr for Alignment {
	type Err = String;

	fn from_str(input: &str) -> Result<Alignment, Self::Err> {
		match input.to_uppercase().as_str() {
			"AUTO"   => Ok(Alignment::AUTO),
			"CENTER" => Ok(Alignment::CENTER),
			"LEFT"   => Ok(Alignment::LEFT),
			"RIGHT"  => Ok(Alignment::RIGHT),
			_ => Err(format!("Invalid frame type: {}", input)),
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
		}
	}
}

#[derive(Args, Clone)]
pub struct TextFormatter {
	/// Text be formatted according to the specified options
	pub input: Option<String>,

	// The maximum width (in characters) allocated for the formatted field.
	// This width determines how the input text will be displayed in the output.
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

#[allow(dead_code)]
impl TextFormatter {
	/// Creates a new `Formatter` instance with default settings.
	///
	/// # Arguments
	///
	/// * `input` - The content to be formatted.
	/// * `width` - The width of the cell.
	///
	/// # Returns
	///
	/// A new `Formatter` with default values for formatting options.
	pub fn new(input: Option<String>) -> Self {

		let input_data = input_or_stdin(input.as_deref(), 5, 500);
		let cleaned = clean(Some(&input_data)).clone();
		
		Self {
			input:          Some(cleaned), // Cleaned input
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

#[allow(dead_code)]
impl TextFormatter {
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

#[allow(dead_code)]
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

		// Ensure that input is present (unwrap Option to get &str)
		let normalized_content = if let Some(input_str) = &self.input {
			input_str
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

	/// Formats the input text based on the specified frame and width.
	///
	/// This method processes the input text, cleans it, and applies formatting 
	/// based on the frame settings (TRUNCATE, WRAP, or no frame). Depending on 
	/// the frame, it will either truncate, wrap, or return the cleaned input directly.
	///
	/// # Returns
	///
	/// A `String` containing the formatted text.
	///
	/// # Frame behavior:
	///
	/// - **TRUNCATE**: If the text width exceeds the specified width, the text is truncated
	///   with an optional ellipsis.
	/// - **WRAP**: Wraps the text to the specified width.
	/// - **None**: Returns the cleaned input text without any formatting.
	pub fn format_text(&self) -> String {
		// Clean the input

		// Determine the formatted text based on the frame setting
		let formatted_text = match self.frame {
			Frame::TRUNCATE => {
				// Call the truncate function directly
				truncate(self.input.as_deref(), Some(self.width), Some(self.no_ellipsis))
			}
			Frame::WRAP => {
				// Wrap the text to the specified width
				wrap(self.input.as_deref(), self.width)
			}
			_ => {
				// If no frame is specified, just return the cleaned text
				self.input.as_ref().expect("REASON").clone()
			}
		};

		// Apply alignment based on the settings
		let width = self.width;
		let aligned_result = match self.alignment {
			Alignment::AUTO   => formatted_text,                       // No alignment
			Alignment::CENTER => center(Some(&formatted_text), Some(width)),
			Alignment::LEFT   => left(Some(&formatted_text)),
			Alignment::RIGHT  => right(Some(&formatted_text),  Some(width)),
		};

		aligned_result
	}

	/// Formats the input as a numeric value with custom formatting options.
	///
	/// This method takes the input string, cleans and normalizes it, and attempts
	/// to parse it as a floating-point number. If parsing is successful, it formats
	/// the number according to the specified settings, such as applying thousands 
	/// separators, padding decimal digits, and custom alignment. If parsing fails, 
	/// an error message is printed, and an empty string is returned.
	///
	/// # Returns
	///
	/// A `String` containing the formatted numeric value. If the input cannot be parsed
	/// as a number, an empty string is returned and an error message is printed.
	///
	/// # Behavior:
	///
	/// - **Thousand Separators**: Adds thousands separators based on the specified custom separator.
	/// - **Decimal Formatting**: Pads decimal places if specified, up to the maximum number of decimal digits.
	/// - **Alignment**: Aligns the formatted number to the left, right, or center based on the `alignment` setting.
	/// - **Error Handling**: If the input is not a valid numeric value, it returns an empty string and prints an error.
	///
	/// # Example:
	///
	/// ```
	/// let formatter = NumericFormatter {
	///     input: "1.234567",
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

		// Normalize input by replacing custom separators

		// Ensure that input is present (unwrap Option to get &str)
		let normalized = if let Some(input_str) = &self.input {
			input_str
				.replace(self.thousand_separator, "")  // Using characters directly
				.replace(self.decimal_separator, ".")
		} else {
			String::new()
		};

		// Attempt to parse the normalized input as a number
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
					Alignment::AUTO   => right(Some(&final_formatted_number),  Some(width)),
					Alignment::CENTER => center(Some(&final_formatted_number), Some(width)),
					Alignment::LEFT   => left(Some(&final_formatted_number)),
					Alignment::RIGHT  => right(Some(&final_formatted_number),  Some(width)),
				};

				aligned_result
			}

			Err(_) => {
				eprintln!("Error: Not a number"); // Print error message
				String::new() // Return an empty string or an appropriate message
			}
		}
	}

	/// Formats the input content based on its type (numeric or text).
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
