pub use crate::rows::{
	CellFormatter,
	TableBuilder,
	Alignment,
//	Frame,
};
pub use prettytable::Cell;
pub use prettytable::format;
pub use prettytable::Row;
pub use prettytable::Table;

impl TableBuilder {
	pub fn table(&mut self) -> &Table {
		// retun cached table if available
		if let Some(ref table) = self.table {
			return table;
		}

		// Clone necessary components
		let column_widths = self.column_widths().clone();
		let headers = self.headers().clone();
		let data = self.data().clone();
		let numeric_columns = self.numeric_columns().clone();

		// Initialize table with custom format
		let mut table = Table::new();
		table.set_format(format::FormatBuilder::new()
			.padding(0, 0) // 0 spaces horizontal and vertical
			.separator(format::LinePosition::Top,    format::LineSeparator::new(' ', ' ', ' ', ' ')) // No top    border
			.separator(format::LinePosition::Title,  format::LineSeparator::new(' ', ' ', ' ', ' ')) // No title  border
			.separator(format::LinePosition::Bottom, format::LineSeparator::new(' ', ' ', ' ', ' ')) // No bottom border
			.build());

		// Handle headers
		if !headers.is_empty() {
			for header_line in headers {

				let mut row = Row::empty();

				// Iterate over each header cell and add to the row
				for (i, header_cell) in header_line.iter().enumerate() {

					let (pt_alignment, alignment) = if numeric_columns[i] {
						(format::Alignment::RIGHT, Alignment::RIGHT)
					} else {
						(format::Alignment::LEFT, Alignment::LEFT)
					};

					let mut formatter = CellFormatter::new(header_cell)
						.set_width(column_widths[i])
						.set_alignment(alignment)
						.set_frame(self.frame)
						.set_no_ellipsis(self.no_ellipsis)
						.clone();

					// Prepend self.ofs to the cell, except for the first cell
					let cell_content = if i > 0 {
						format!("{}{}", self.ofs, formatter.formatted())
					} else {
						formatter.formatted()
					};

					let formatted_cell = Cell::new_align(&cell_content, pt_alignment);

					// Add cell to the row
					row.add_cell(formatted_cell);

				}

				// Add the row to the table
				table.add_row(row);

			}
		}

		if !self.no_divider {
			let mut divider_row = Row::empty();

			// Iterate over each column width
			for (i, &width) in column_widths.iter().enumerate() {
				// Repeat the divider character `width` times
				let divider_cell = self.divider_char.to_string().repeat(width);

				// Prepend self.ofs to the cell, except for the first cell
				let cell_content = if i > 0 {
					format!("{}{}", self.ofs, divider_cell)
				} else {
					divider_cell
				};

				// Create a cell and add it to the row
				divider_row.add_cell(Cell::new(&cell_content));
			}

			// Add the divider row to the table
			table.add_row(divider_row);
		}


		// Handle data (you can implement data row filling in a similar way)
		if !data.is_empty() {
			for data_row in data {
				let mut row = prettytable::Row::empty();

				// Fill each cell in the data row
				for (i, data_cell) in data_row.iter().enumerate() {

					let (pt_alignment, alignment) = if numeric_columns[i] {
						(format::Alignment::RIGHT, Alignment::RIGHT)
					} else {
						(format::Alignment::LEFT, Alignment::LEFT)
					};

					let mut formatter = CellFormatter::new(data_cell)
						.set_width(column_widths[i])
						.set_alignment(alignment)
						.set_frame(self.frame)
						.set_no_ellipsis(self.no_ellipsis)
						.set_pad_decimal_digits(self.pad_decimal_digits)
						.set_max_decimal_digits(self.max_decimal_digits)
						.set_decimal_separator(self.decimal_separator)
						.set_use_thousand_separator(self.use_thousand_separator)
						.set_thousand_separator(self.thousand_separator)
						.clone();

					// Prepend self.ofs to the cell, except for the first cell
					let cell_content = if i > 0 {
						format!("{}{}", self.ofs, formatter.formatted())
					} else {
						formatter.formatted()
					};

					let formatted_cell = Cell::new_align(&cell_content, pt_alignment);

					// Add cell to the row
					row.add_cell(formatted_cell);
				}

				// Add the data row to the table
				table.add_row(row);
			}
		}




        // Cache the table if required
        self.table = Some(table.clone());

        // Return a reference to the table
        self.table.as_ref().unwrap()






	}
}
