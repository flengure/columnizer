use crate::text;
use std::num::ParseFloatError;
use textwrap;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextFormat {
	/// Shorten the text to fit the width
	Truncate,
	/// Wrap the text to fit the width
	Wrap,
	/// Leave the text unchanged
	NoFormat,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextAlignment {
	/// Align to the right if numeric
	Auto,
	/// Align to the right, text or numeric
	Right,
	/// No alignment
	NoAlignment,
}

#[derive(Clone, Debug)]
pub struct CellFormatter {
    /// The input string to be formatted.
    pub input_text: String,
    pub formatted_text: String,
    /// The width of the field to be formatted.
    pub width: usize,

    /// Whether to truncate the text within the field width.
    /// If false, text will be wrapped to fit within the field width.
    pub text_format: TextFormat,
    /// Whether to truncate the text with an ellipsis if it exceeds the width.
    /// Applies only if `truncate` is true.
    pub ellipsis: bool,
    /// Whether to pad decimal digits with trailing zeros if necessary.
    pub pad_decimal_digits: bool,
    /// The maximum number of decimal digits to display.
    pub max_decimal_digits: usize,
    /// The character to use as the decimal separator.
    pub decimal_separator: char,
    /// Whether to use a thousand separator in numeric values.
    pub use_thousand_separator: bool,
    /// The character to use as the thousand separator.
    pub thousand_separator: char,

	/// alignment
	/// if set to Auto, will align numeric text to the right,
	/// if set to Right will force alignment to the right text or numeric
	/// if set to None will leave the text unchanged
    pub alignment: TextAlignment,

    /// A flag indicating whether the content is numeric.
    pub is_numeric: Option<bool>,
}

impl CellFormatter {
	/// Creates a new `CellFormatter` instance with default settings.
	///
	/// # Arguments
	///
	/// * `input` - The content to be formatted.
	/// * `width` - The width of the cell.
	///
	/// # Returns
	///
	/// A new `CellFormatter` with default values for formatting options.
	pub fn new(input: String, width: usize) -> Self {
		let trimmed_input = text::trim_and_strip_blanks(&input);
		Self {
			input_text: input,
			width,
			formatted_text: trimmed_input.clone(),
			text_format:     TextFormat::Truncate,
			alignment:        TextAlignment::Auto,
			ellipsis:                        true,
			pad_decimal_digits:             false,
			max_decimal_digits:                 2, // Default to 2 decimal places
			decimal_separator:                '.', // Default decimal separator
			use_thousand_separator:         false,
			thousand_separator:               ',',
			is_numeric:                      None,
		}
	}
}

#[allow(dead_code)]
impl CellFormatter {
    pub fn set_width(&mut self, width: usize) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_text_format(&mut self, text_format: TextFormat) -> &mut Self {
        self.text_format = text_format;
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

    pub fn set_alignment(&mut self, alignment: TextAlignment) -> &mut Self {
        self.alignment = alignment;
        self
    }
}

impl CellFormatter {

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

impl CellFormatter {

    // Method to wrap or truncate text based on the `truncate` flag
    pub fn wrap_or_truncate(&mut self) -> &mut Self {
		if self.width == 0 {
			return self;
		}
        let text_width = self.formatted_text.width();
        let formatted_text = if let TextFormat::Truncate = self.text_format {
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
        } else if let TextFormat::Wrap = self.text_format {
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
		if self.alignment == TextAlignment::Right 
			|| (self.alignment == TextAlignment::Auto && self.is_numeric()) {

			// Right-align numeric data
			self.formatted_text = format!("{:>width$}", self.formatted_text, width = self.width);
		}

		self
	}
}
