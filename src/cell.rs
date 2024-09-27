use crate::text::clean;
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
}

impl FromStr for Alignment {
	type Err = String;

	fn from_str(input: &str) -> Result<Alignment, Self::Err> {
		match input.to_uppercase().as_str() {
			"AUTO" => Ok(Alignment::AUTO),
			"CENTER" => Ok(Alignment::CENTER),
			"LEFT" => Ok(Alignment::LEFT),
			"RIGHT" => Ok(Alignment::RIGHT),
			_ => Err(format!("Invalid frame type: {}", input)),
		}
	}
}

impl fmt::Display for Alignment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Alignment::AUTO => write!(f, "AUTO"),
			Alignment::CENTER => write!(f, "CENTER"),
			Alignment::LEFT => write!(f, "LEFT"),
			Alignment::RIGHT => write!(f, "RIGHT"),
		}
	}
}

#[derive(Clone)]
pub struct Formatter {
	/// The raw input string that will be formatted according to the specified options.
	#[allow(dead_code)]
	pub input_text: String,

	/// The resulting formatted string after applying the specified formatting rules.
	#[allow(dead_code)]
	pub formatted_text: String,

	/// The maximum width (in characters) allocated for the formatted field.
	/// This width determines how the input text will be displayed in the output.
	pub width: usize,

	/// Specifies the formatting style for the text within the field.
	/// If set to `Frame::TRUNCATE`, text will be truncated to fit within the field width.
	/// If set to `Frame::WRAP`, text will be wrapped onto multiple lines if it exceeds the width.
	pub frame: Frame,

	/// Indicates whether to truncate text with an ellipsis ("...") when it exceeds the defined width.
	/// This is applicable only when `frame` is set to `Frame::TRUNCATE`.
	pub ellipsis: bool,

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

impl Formatter {
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
	pub fn new(input: String) -> Self {
		let trimmed_input = clean(&input);
		Self {
			input_text:                     input, // Original input
			width:                             48, // Default 48
			formatted_text: trimmed_input.clone(), // Default cleaned input
			frame:                Frame::TRUNCATE, // Default TRUNCATE
			alignment:            Alignment::AUTO, // Default text left, numbers right
			ellipsis:                        true, // Default add an ellipsis to truncated text
			pad_decimal_digits:             false, // Default dont pad decimal places
			max_decimal_digits:                 2, // Default to 2 decimal places
			decimal_separator:                '.', // Default decimal separator
			use_thousand_separator:         false, // Default no thousands grouping
			thousand_separator:               ',', // Default `,`
			is_numeric:                      None, // Unknown
		}
	}
}

//#[allow(dead_code)]
impl Formatter {
	pub fn set_width(&mut self, width: usize) -> &mut Self {
		self.width = width;
		self
	}

	pub fn set_frame(&mut self, frame: Frame) -> &mut Self {
		self.frame = frame;
		self
	}

	pub fn set_ellipsis(&mut self, ellipsis: bool) -> &mut Self {
		self.ellipsis = ellipsis;
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

impl Formatter {

	/// Checks if the content is numeric by first checking the cached value.
	/// If not cached, normalizes the content and checks if it can be parsed as f64.
	///
	/// # Returns
	///
	/// A boolean indicating whether the content is numeric.
	pub fn is_numeric(&mut self) -> bool {
		if let Some(is_numeric) = self.is_numeric {
			return is_numeric;
		}

		let normalized_content = self.formatted_text
			.replace(self.thousand_separator, "")
			.replace(self.decimal_separator, ".");

		self.is_numeric = Some(normalized_content.parse::<f64>().is_ok());
		self.is_numeric.unwrap()
	}

	// Method to get the width of the formatted text after trimming
	pub fn trimmed_width(&self) -> usize {
		// Trim the formatted text and calculate its width
		self.formatted_text.trim().width()
	}
}

#[allow(dead_code)]
impl Formatter {

	// Method to wrap or truncate text based on the `truncate` flag
	pub fn wrap_or_truncate(&mut self) -> &mut Self {
		if self.width == 0 {
			return self;
		}
		let text_width = self.formatted_text.width();
		let formatted_text = if let Frame::TRUNCATE = self.frame {
			if text_width <= self.width {
				&self.formatted_text
			} else {
				let mut width = 0;
				let mut truncated = String::new();
				let ellipsis_len = if self.ellipsis { 3 } else { 0 };
				let max_width = self.width.saturating_sub(ellipsis_len);
				for c in self.formatted_text.chars() {
					let char_width = c.width().unwrap_or(0);
					if width + char_width > max_width {
						break;
					}
					width += char_width;
					truncated.push(c);
				}
				&if self.ellipsis && self.width > 3 {
					format!("{}...", truncated.trim())
				} else {
					truncated.trim().to_string()
				}
			}
		} else if let Frame::WRAP = self.frame {
			&textwrap::wrap(&self.formatted_text, self.width).join("\n")
		} else {
			&self.formatted_text.clone()
		};
		self.formatted_text = formatted_text.to_string();
		self
	}

	pub fn format(&mut self) -> &mut Self {

		// Normalize input by replacing custom separators
		let normalized_text = self.formatted_text
			.replace(self.thousand_separator, ",")
			.replace(self.decimal_separator, ".");

		// Attempt to parse the normalized input as a number
		let result: Result<f64, ParseFloatError> = normalized_text.parse();
		
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
				let formatted_number = if !fractional_part.is_empty() {
					format!("{}.{}", integer_with_thousands, fractional_part)
				} else {
					integer_with_thousands
				};

				// Replace decimal separator with custom one
				let final_formatted_number = formatted_number.replace('.', &self.decimal_separator.to_string());

				self.is_numeric = Some(true);
				self.formatted_text = final_formatted_number;
			}

			Err(_) => {

				self.is_numeric = Some(false);
				self.wrap_or_truncate();

			}
		}

		self
	}

	// Method to align text
	pub fn align(&mut self) -> &mut Self {
		if self.alignment == Alignment::RIGHT 
			|| (self.alignment == Alignment::AUTO && self.is_numeric()) {

			// Right-align numeric data
			self.formatted_text = format!("{:>width$}", self.formatted_text, width = self.width);
		}

		self
	}
}
