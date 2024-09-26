pub use crate::cell::{ CellFormatter, TextAlignment, TextFormat };
pub use crate::builder::TableBuilder;

impl TableBuilder {

	/// Determines and returns the number of columns in the input data.
	///
	/// If the column count has already been calculated, this method returns the cached value.
	/// Otherwise, it calculates the number of columns by iterating over all lines in the input,
	/// splitting each line by the input field separator (`ifs`), and finding the maximum number of fields (columns)
	/// across all rows. This ensures that the widest row determines the total column count.
	///
	/// If the input contains no lines, the column count defaults to `0`.
	///
	/// The calculated column count is cached in `self.column_count` for future use to avoid redundant calculations.
	///
	/// # Returns
	/// 
	/// * The total number of columns in the input, as a `usize`.
	///
	/// # Side Effects
	/// 
	/// * Caches the computed column count in `self.column_count`.
	///
	/// # Example
	///
	/// ```
	/// let column_count = your_instance.column_count();
	/// println!("The table has {} columns.", column_count);
	/// ```
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

	/// Returns the maximum allowed width for each column, initializing if necessary.
	///
	/// If the column width limits have already been calculated, this method returns the stored values.
	/// Otherwise, it calculates the limits based on the `column_width_limits_index`, which points to a 
	/// specific row used for determining the column width constraints. If the index is valid (greater than 0),
	/// it uses that row to compute the width for each column by taking the parsed numeric value from the row,
	/// with `max_cell_width` as the fallback for any invalid or zero values.
	///
	/// If the `column_width_limits_index` is not set, the method initializes the column width limits with 
	/// `max_cell_width` for each column.
	///
	/// This method also ensures `self.column_count()` and `self.column_width_limits()` are calculated 
	/// as part of the initialization process.
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

	/// Returns a reference to the `header_column_widths`, initializing if necessary.
	/// 
	/// If `header_column_widths` is already calculated, this function returns a reference to it.
	/// Otherwise, it initializes a vector of zeros with a length equal to `self.column_count()`.
	/// 
	/// This method ensures `self.column_count()` is calculated as part of the initialization process.
	/// `self.header_column_widths` is set during the `self.headers()` method execution.
	pub fn header_column_widths(&mut self) -> &Vec<usize> {

		if let Some(ref header_column_widths) = self.header_column_widths {
			return header_column_widths;
		}

		self.header_column_widths = Some(vec![0; self.column_count()]);

		self.header_column_widths.as_ref().unwrap()
	}

	/// Returns a reference to the `data_column_widths`, initializing if necessary.
	/// 
	/// If `data_column_widths` is already calculated, this function returns a reference to it.
	/// Otherwise, it initializes a vector of zeros with a length equal to `self.column_count()`.
	/// 
	/// This method ensures `self.column_count()` is calculated as part of the initialization process.
	/// `self.data_column_widths` is set during the `self.data()` method execution.
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

	/// Returns a reference to the `numeric_columns` vector, initializing it if necessary.
	///
	/// This vector tracks whether each column contains only numeric values. Initially, all columns are
	/// assumed to be numeric (set to `true`). During data processing, if any value in a column is found to
	/// be non-numeric, the entire column is marked as non-numeric (`false`).
	///
	/// If the `numeric_columns` vector has already been calculated, this method simply returns a reference
	/// to the existing data. If not, it initializes the vector with a length equal to `self.column_count()`,
	/// setting each value to `true` (assuming all columns are numeric until proven otherwise).
	///
	/// This method also ensures that `self.column_count()` is calculated as part of the initialization process.
	/// The actual determination of numeric status for each column happens during the execution of the `self.data()` method.
	///
	/// # Returns
	/// 
	/// * A reference to a `Vec<bool>` indicating whether each column is numeric (`true`) or not (`false`).
	///
	/// # Example
	///
	/// ```
	/// let numeric_columns = your_instance.numeric_columns();
	/// println!("Are columns numeric? {:?}", numeric_columns);
	/// ```
	pub fn numeric_columns(&mut self) -> &Vec<bool> {

		if let Some(ref numeric_columns) = self.numeric_columns {
			return numeric_columns;
		}

		self.numeric_columns = Some(vec![true; self.column_count()]);

		self.numeric_columns.as_ref().unwrap()

	}

	/// Clears the cached headers to allow for recalculation during the next headers retrieval.
	///
	/// This method sets the `headers` field to `None`, indicating that the headers need to be recalculated 
	/// the next time they are accessed. It can be particularly useful when the underlying data has changed 
	/// and the headers must reflect these changes.
	///
	/// # Example
	///
	/// ```
	/// let exa = CellFormatter::new()
	///  ...
	///	 .clear_headers()
	///	 .headers(); // Forces a recalculation of headers
	/// ```
	///
	/// # Returns
	/// 
	/// * A mutable reference to the current instance of `Self`, allowing for method chaining.
	pub fn clear_headers(&mut self) -> &mut Self {
		self.headers = None; // Clear the cached headers
		self
	}

	/// Retrieves the headers from the input data, processing and storing them if not already done.
	///
	/// This method checks whether the headers have been previously processed and stored. If they are already
	/// available, it returns a reference to the stored headers. If not, it initializes the headers by 
	/// reading the specified number of header lines from the input, starting from the specified header index. 
	/// The headers are split by the input field separator (IFS), and the width of each header cell is recorded 
	/// to update the corresponding column widths specifically in `header_column_widths`.
	///
	/// The method ensures that the column widths are updated according to the length of each header cell,
	/// while respecting any specified column width limits.
	///
	/// # Returns
	///
	/// * A reference to a vector of vectors containing the header strings. If no headers are defined, 
	///   an empty vector is returned.
	///
	/// # Example
	///
	/// ```
	/// let headers = formatter.headers();
	/// ```
	pub fn headers(&mut self) -> &Vec<Vec<String>> {
		// Check if headers have already been processed and stored
		if let Some(ref headers) = self.headers {
			return headers;
		}

		let mut column_widths = self.header_column_widths().clone();
		let column_width_limits = self.column_width_limits().clone();

		// Initialize headers if header_index and header_count are set
		self.headers = Some(if self.header_index > 0 && self.header_count > 0 {
			// Initialize the column widths vector

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
									.min(column_width_limits.get(i).copied().unwrap_or(usize::MAX));
							}

							header_cell // Return the header cell as a String
						})
						.collect::<Vec<String>>()
				})
				.collect::<Vec<Vec<String>>>()
		} else {
			vec![] // Return an empty vector if no headers are defined
		});

		// After processing, specifically update the header_column_widths stored in self
		self.header_column_widths = Some(column_widths.to_vec());

		// Return the headers, or an empty vector if not present
		self.headers.as_ref().unwrap()
	}

	/// Processes the input data and updates relevant attributes.
	///
	/// This method performs the following operations:
	/// - Updates `self.data`, `self.numeric_columns`, and `self.data_column_widths`.
	/// - Formats numeric values according to specified formatting options, while leaving text cells unchanged for now.
	/// - The widths in `self.data_column_widths` will reflect the widths of both formatted numbers and unchanged text.
	///
	/// If `self.data` is already set, the method returns a reference to the corresponding stored field.
	/// Otherwise, it processes the input lines, excluding header rows and the column width limits row, 
	/// and formats each cell based on its type, while also setting `self.numeric_columns`.
	///
	/// # Returns
	///
	/// * A reference to a vector of vectors containing the processed data. The structure reflects the rows and formatted cells.
	///
	/// # Example
	///
	/// ```
	/// let processed_data = formatter.data();
	/// ```
	pub fn data(&mut self) -> &Vec<Vec<String>> {
		// Check if data have already been processed and stored
		if let Some(ref data) = self.data {
			return data;
		}

		// Adjust for 1-indexed header and column width limits index
		let header_start = self.header_index.saturating_sub(1);
		let header_end   = header_start + self.header_count;
		let column_width_limits_index = self.column_width_limits_index.saturating_sub(1);

		let column_width_limits = self.column_width_limits().clone();
		let mut column_widths   = self.data_column_widths().clone();
		let mut numeric_columns = self.numeric_columns().clone();

		// Collect all rows that are not headers and not the column_width_limits_index row
		self.data = Some(self.input.lines()
			.enumerate()
			.filter_map(|(i, line)| {
				// Skip header rows and the column width limits row
				if (header_start <= i && i < header_end) || i == column_width_limits_index {
					None // Skip this row
				} else {
				// Split the line by the input field separator and collect into Vec<String>
				let mut row: Vec<String> = line.split(&self.ifs).map(String::from).collect();

				// Check each column for numeric values and format the cell
				for (j, cell) in row.iter_mut().enumerate() {
					let cell_value = cell.clone();
			   
						let mut formatter = CellFormatter::new(cell_value)
							.set_text_format(TextFormat::NoFormat)
							.set_alignment(TextAlignment::NoAlignment)
							.set_decimal_separator(self.decimal_separator)
							.set_pad_decimal_digits(self.pad_decimal_digits)
							.set_max_decimal_digits(self.max_decimal_digits)
							.set_thousand_separator(self.thousand_separator)
							.set_use_thousand_separator(self.use_thousand_separator)
							.format()
							.clone();

						// Update column width for this cell
						let width = formatter.trimmed_width();
						if j < column_widths.len() {
							column_widths[j] = column_widths[j]
								.max(width)
								.min(column_width_limits.get(j).copied().unwrap_or(usize::MAX));
						}

						// Update numeric_columns
						if j < numeric_columns.len() {
							numeric_columns[j] = formatter.is_numeric();
						}

						// Replace the original cell with the formatted one
						*cell = formatter.formatted_text;
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

	/// Calculates and returns the column widths used for formatting table output.
	///
	/// This method determines the optimal column widths based on the provided text format:
	///
	/// - If the `TextFormat` is set to `Truncate` or `NoFormat`, the column width for each column
	///   is set to the maximum value between the corresponding header and data widths. This ensures
	///   that the columns are wide enough to fit both header and data without truncation.
	/// - If the `TextFormat` is set to `Wrap`, the column width for each column is set to the data width,
	///   allowing text to wrap within the defined column width.
	///
	/// The method ensures that headers, data, and their respective column widths (`header_column_widths`
	/// and `data_column_widths`) are initialized before computing the final column widths.
	///
	/// Once computed, the result is cached in `self.column_widths` to avoid redundant calculations
	/// on subsequent calls.
	///
	/// # Returns
	/// 
	/// A reference to the computed column widths, which is a `Vec<usize>` where each entry corresponds
	/// to the width of the respective column.
	///
	/// # Panics
	///
	/// This method may panic if the internal column count is inconsistent with the lengths of
	/// `header_column_widths` or `data_column_widths`.
	///
	/// # Example
	///
	/// ```rust
	/// let column_widths = table.column_widths();
	/// // column_widths now contains the calculated widths for each column.
	/// ```
	///
	/// # Notes
	///
	/// - The method assumes that `self.column_count()`, `self.headers()`, and `self.data()`
	///   are properly populated before determining column widths.
	/// - The column widths are only recalculated if they have not been cached previously.
	pub fn column_widths(&mut self) -> &Vec<usize> {
		// Return cached column widths if they are already computed
		if let Some(ref column_widths) = self.column_widths {
			return column_widths;
		}

		// Initialize column_widths as a vector of zeroes with the length equal to column_count
		let mut column_widths = vec![0; self.column_count()];

		// Ensure that headers and data are populated
		let _headers = self.headers(); // populate self.headers and self.header_column_widths
		let _data = self.data(); // populate self.data, self.numeric_columns, and self.data_column_widths

		// Unwrap header_column_widths and data_column_widths
		let header_column_widths = self.header_column_widths().clone();
		let data_column_widths = self.data_column_widths().clone();

		// Determine the column widths based on the format_text option
		match self.text_format {
			TextFormat::Truncate | TextFormat::NoFormat => {
				// If Truncate or NoFormat, set column_widths to the max of header and data widths
				for i in 0..column_widths.len() {
					column_widths[i] = header_column_widths[i].max(data_column_widths[i]);
				}
			}
			TextFormat::Wrap => {
				// If Wrap, use data_column_widths
				column_widths.copy_from_slice(&data_column_widths);
			}
		}

		// Cache the computed column widths in self.column_widths
		self.column_widths = Some(column_widths);

		// Return the cached column widths
		self.column_widths.as_ref().unwrap()
	}

}
