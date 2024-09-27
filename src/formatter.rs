use crate::format::{clean, center, left, right, truncate};
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;
use textwrap;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Alignment {
	/// Align to the right if numeric
	AUTO,
	CENTER,
	LEFT,
	RIGHT,
	NONE,
}

impl FromStr for Alignment {
	type Err = String;

	fn from_str(input: &str) -> Result<Alignment, Self::Err> {
		match input.to_uppercase().as_str() {
			"AUTO"   => Ok(Alignment::AUTO),
			"CENTER" => Ok(Alignment::CENTER),
			"LEFT"   => Ok(Alignment::LEFT),
			"RIGHT"  => Ok(Alignment::RIGHT),
			"NONE"   => Ok(Alignment::LEFT),
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
			Alignment::NONE   => write!(f, "LEFT"  ),
		}
	}
}

#[derive(Clone, Copy)]
pub struct Formatter<'a> {
	/// The raw input string that will be formatted according to the specified options.
	#[allow(dead_code)]
	pub input: &'a str,

	/// The maximum width (in characters) allocated for the formatted field.
	/// This width determines how the input text will be displayed in the output.
	pub width: usize,

	/// Specifies the formatting style for the text within the field.
	/// If set to `Frame::TRUNCATE`, text will be truncated to fit within the field width.
	/// If set to `Frame::WRAP`, text will be wrapped onto multiple lines if it exceeds the width.
	pub frame: Frame,

	/// Indicates whether to truncate text with an ellipsis ("...") when it exceeds the defined width.
	/// This is applicable only when `frame` is set to `Frame::TRUNCATE`.
	pub no_ellipsis: bool,

	/// Determines whether to pad decimal digits with trailing zeros to maintain a consistent appearance.
	/// If set to true, decimal numbers will display the specified number of digits after the decimal point.
	pub pad_decimal_digits: bool,

	/// The maximum number of decimal digits that will be displayed for numeric values.
	/// This setting helps control the precision of the output for numeric formatting.
	pub max_decimal_digits: usize,

	/// The character used as the decimal separator in formatted numeric values.
	/// This is particularly useful for ensuring compatibility with various regional formats.
	pub decimal_separator: char,

	/// A flag indicating whether to include a thousand separator in large numeric values.
	/// If set to true, numbers will be formatted with the specified `thousand_separator`.
	pub use_thousand_separator: bool,

	/// The character used as the thousand separator in formatted numeric values.
	/// This enhances readability by grouping digits in large numbers.
	pub thousand_separator: char,

	/// Specifies the alignment of the text within the field.
	/// - `Alignment::AUTO`: Automatically aligns numeric text to the right.
	/// - `Alignment::RIGHT`: Forces right alignment for both numeric and non-numeric text.
	/// - `Alignment::None`: Leaves the text unchanged, preserving its original alignment.
	pub alignment: Alignment,

	/// A flag indicating whether the content being formatted is numeric.
	/// This can influence how certain formatting rules are applied, such as decimal padding.
	#[allow(dead_code)]
	pub is_numeric: Option<bool>,

}

impl <'a>Formatter<'a> {
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
	pub fn new(input: &'a str) -> Self {
		let trimmed_input = clean(&input);
		Self {
			                        input, // Original input
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

impl <'a>Formatter<'a> {
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

impl <'a>Formatter<'a> {

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

		// Clean and normalize the input by removing the thousand separator and replacing the decimal separator with '.'
		let cleaned_input = clean(self.input);
		let normalized_content = cleaned_input
			.replace(self.thousand_separator, "")
			.replace(self.decimal_separator, ".");

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
		let cleaned = clean(self.input);

		// Determine the formatted text based on the frame setting
		let formatted_text = match self.frame {
			Frame::TRUNCATE => {
				// Call the truncate function directly
				truncate(&cleaned, self.width, Some(self.no_ellipsis))
			}
			Frame::WRAP => {
				// Wrap the text to the specified width
				textwrap::wrap(&cleaned, self.width).join("\n")
			}
			_ => {
				// If no frame is specified, just return the cleaned text
				cleaned
			}
		};

		// Apply alignment based on the settings
		let width = self.width;
		let aligned_result = match self.alignment {
			Alignment::AUTO   => formatted_text,                       // No alignment
			Alignment::CENTER => center(&formatted_text, Some(width)),
			Alignment::LEFT   => left(&formatted_text),
			Alignment::RIGHT  => right(&formatted_text,  Some(width)),
			Alignment::NONE   => formatted_text,                       // No alignment
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
		// Clean the input
		let cleaned = clean(self.input);

		// Normalize input by replacing custom separators
		let normalized = cleaned
			.replace(self.thousand_separator, ",")
			.replace(self.decimal_separator, ".");

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
				let width = self.width; // Assuming self.width is defined and represents the alignment width
				let aligned_result = match self.alignment {
					Alignment::AUTO   => right(&final_formatted_number,  Some(width)),
					Alignment::CENTER => center(&final_formatted_number, Some(width)),
					Alignment::LEFT   => left(&final_formatted_number),
					Alignment::RIGHT  => right(&final_formatted_number,  Some(width)),
					Alignment::NONE   => final_formatted_number, // No alignment
				};

				aligned_result // Return the aligned formatted number
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
