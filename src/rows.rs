use crate::cell::{ CellFormatter, TextAlignment, TextFormat };
use crate::table::TableBuilder;


impl TableBuilder {

	pub fn column_count(&mut self) -> usize {
		if let Some(column_count) = self.column_count {
			return column_count;
		}

		self.column_count = Some(
			self.input.lines()
				.map(|line| line.split(&self.ifs).count())
				.max()
				.unwrap_or(0)
		);

		self.column_count.unwrap()

	}

    pub fn column_width_limits(&mut self) -> &Vec<usize> {

		if let Some(ref column_width_limits) = self.column_width_limits {
			return column_width_limits;
		}

		self.column_width_limits = Some(if self.column_width_limits_index > 0 {
			self.input.lines()
				.nth(self.column_width_limits_index)
				.map(|line| {
					line.split(&self.ifs)
						.map(|s| s.trim().parse::<usize>().unwrap_or(self.max_cell_width))
						.map(|width| if width == 0 { self.max_cell_width } else { width })
						.collect::<Vec<usize>>()
				})
				.unwrap_or_else(|| vec![self.max_cell_width; self.column_count()])
		} else {
			vec![self.max_cell_width; self.column_count()]
		});

		self.column_width_limits.as_ref().unwrap()

    }

    pub fn header_column_widths(&mut self) -> &Vec<usize> {

		if let Some(ref header_column_widths) = self.header_column_widths {
			return header_column_widths;
		}

		self.header_column_widths = Some(vec![0; self.column_count()]);

		self.header_column_widths.as_ref().unwrap()

    }

	pub fn data_column_widths(&mut self) -> &Vec<usize> {
		// If widths are already calculated, return a reference to them
		if let Some(ref data_column_widths) = self.data_column_widths {
			return data_column_widths;
		}

		// Initialize with zeros if not calculated
		self.data_column_widths = Some(vec![0; self.column_count()]);

		// Return a reference to the newly created vector
		self.data_column_widths.as_ref().unwrap()
	}

    pub fn numeric_columns(&mut self) -> &Vec<bool> {

		if let Some(ref numeric_columns) = self.numeric_columns {
			return numeric_columns;
		}

		self.numeric_columns = Some(vec![true; self.column_count()]);

		self.numeric_columns.as_ref().unwrap()

    }

	pub fn clear_headers(&mut self) -> &mut Self {
		self.headers = None;
		self
	}

	pub fn headers(&mut self) -> &Vec<Vec<String>> {
		// Check if headers have already been processed and stored
		if let Some(ref headers) = self.headers {
			return headers;
		}

		// Initialize headers if header_index and header_count are set
		self.headers = Some(if self.header_index > 0 && self.header_count > 0 {
			// Initialize the column widths vector
			let mut column_widths = self.header_column_widths();

			// Iterate over the specified header lines
			self.input.lines()
				.skip(self.header_index - 1)
				.take(self.header_count)
				.map(|line| {
					// Split each line by the input field separator (IFS)
					line.split(&self.ifs)
						.enumerate()
						.map(|(i, s)| {
							let header_cell = s.to_string();
							let width = header_cell.len(); // Get length of the header cell

							// Update the column width for the current column (if within bounds)
							if i < column_widths.len() {
								column_widths[i] = column_widths[i]
									.max(width)
									.min(self.column_width_limits().get(i).copied().unwrap_or(usize::MAX));
							}

							header_cell // Return the header cell as a String
						})
						.collect::<Vec<String>>()
				})
				.collect::<Vec<Vec<String>>>()
		} else {
			vec![] // Return an empty vector if no headers are defined
		});

		// After processing, update the column widths stored in self
		self.header_column_widths = Some(column_widths);

		// Return the headers, or an empty vector if not present
		self.headers.as_ref().unwrap_or_default()
	}

	pub fn data(&mut self) -> &Vec<Vec<String>> {
		// Check if data have already been processed and stored
		if let Some(ref data) = self.data {
			return data;
		}

		// Adjust for 1-indexed header and column width limits index
		let header_start = self.header_index.saturating_sub(1);
		let header_end = header_start + self.header_count;
		let column_width_limits_index = self.column_width_limits_index.saturating_sub(1);

		let column_width_limits = self.column_width_limits().clone(); // Make a copy of the width limits
		let mut column_widths = self.data_column_widths().clone();	// Mutable copy to allow updates
		let mut numeric_columns = self.numeric_columns().clone();	 // Mutable copy to allow updates

		// Collect all rows that are not headers and not the column_width_limits_index row
		self.data = Some(self.input.lines()
			.enumerate() // Get the line number along with the content
			.filter_map(|(i, line)| {
				// Skip header rows and the column width limits row
				if (header_start <= i && i < header_end) || i == column_width_limits_index {
					None // Skip this row
				} else {
					// Split the line by the input field separator and collect into Vec<String>
					let mut row: Vec<String> = line.split(&self.ifs).map(String::from).collect();

					// Check each column for numeric values and format the cell
					for (j, cell) in row.iter_mut().enumerate() {
						let formatted_cell = CellFormatter::new(cell)
							.set_text_format(TextFormat::NoFormat)
							.set_alignment(TextAlignment::NoAlignment)
							.set_decimal_separator(self.decimal_separator)
							.set_pad_decimal_digits(self.pad_decimal_digits)
							.set_max_decimal_digits(self.max_decimal_digits)
							.set_thousand_separator(self.thousand_separator)
							.set_use_thousand_separator(self.use_thousand_separator)
							.format();

						// Update column width for this cell
						let width = formatted_cell.get_width();
						if j < column_widths.len() {
							column_widths[j] = column_widths[j]
								.max(width)
								.min(column_width_limits.get(j).copied().unwrap_or(usize::MAX));
						}

						// Update numeric_columns
						if j < numeric_columns.len() {
							numeric_columns[j] = formatted_cell.is_numeric();
						}

						// Replace the original cell with the formatted one
						*cell = formatted_cell.formatted_text;
					}
					Some(row) // Include this row in the result
				}
			})
			.collect());

		// Update self with the new column widths and numeric columns
		self.data_column_widths = Some(column_widths);
		self.numeric_columns = Some(numeric_columns);

		// Return a reference to the processed data
		self.data.as_ref().unwrap()
	}




}
