use crate::io::input_or_stdin;
use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;

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
