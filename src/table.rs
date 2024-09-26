use crate::cell::{ CellFormatter, TextAlignment, TextFormat };
use crate::text::trim_and_strip_blanks;
use prettytable::format;
use prettytable::Table;

/// Builder for configuring and formatting text into columns.
///
/// This struct allows setting various options to control the formatting of text into columns,
/// including field separators, header rows, divider lines, and text width.
#[allow(dead_code)]
pub struct TableBuilder {
	input:                  String, // The text to be formatted
	ifs:                    String, // Input Field Separator
	ofs:                    String, // Output Field Separator
	header_index:            usize, // Which row is the header or 0 for no header
	header_count:            usize, // Which row is the header or 0 for no header

    /// The row in the data that specifies column widths.
    ///
    /// This specifies which row contains the column widths. The width values in this
    /// row can override the global width settings.
    column_width_limits_index: usize,

    /// Column width limits specified in the data row.
    ///
    /// This contains the column widths as specified in a special row in the data. These widths
    /// are clamped by global maximum column width col_width_max.
    column_width_limits: Option<Vec<usize>>,

	no_divider:               bool, // Whether to include a divider line ----
	divider_char:             char, // Divider Character ----, ====, ####
	max_cell_width:          usize, // Maximum width of a cell
	text_format:        TextFormat, // truncate or wrap text to width of cell
	pad_decimal_digits:       bool, // Do we align the decimals padding with 0 at the end if necessary
	max_decimal_digits:      usize, // Limit the number of decimal places
	decimal_separator:        char, // Character to display decimals 0.0, 0,0
	use_thousand_separator:   bool, // Do we add thousands separator in output
	thousand_separator:       char, // Separator for thousands, 0,000, 0.000
	alignment:       TextAlignment, // Do we align numeric columns to the right
	table:                   Table, // prettytable instance
	max_column_widths: Vec<String>, // maximum width for each column

    /// Column widths specified in the data row.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
    header_column_widths: Option<Vec<usize>>,

    /// Column widths specified in the data row.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
    data_column_widths: Option<Vec<usize>>,

    /// The header rows cached from the input data.
    ///
    /// extracted from the input, by considering, header_index and header_count
    /// this will store the header rows.
    headers: Option<Vec<Vec<String>>>,
	data:         Vec<Vec<String>>, // data rows

    /// Cached status indicating whether each column is numeric.
    ///
    /// This indicates whether each column in the data is numeric (`true`) or text (`false`).
    /// This helps in formatting and alignment.
    pub numeric_columns: Option<Vec<bool>>,
	column_count:    Option<usize>, // number of columns after parsing data
}

#[allow(dead_code)]
impl TableBuilder  {
	/// Creates a new `Builder` with default settings.
	pub fn new(input: String) -> Self {

		let trimmed_input = trim_and_strip_blanks(&input);

		let mut table = Table::new();
		table.set_format(format::FormatBuilder::new()
			.padding(0, 0) // 0 spaces horizontal and vertical
			.separator(format::LinePosition::Top,    format::LineSeparator::new(' ', ' ', ' ', ' ')) // No top    border
			.separator(format::LinePosition::Title,  format::LineSeparator::new(' ', ' ', ' ', ' ')) // No title  border
			.separator(format::LinePosition::Bottom, format::LineSeparator::new(' ', ' ', ' ', ' ')) // No bottom border
			.build());

		Self {
			table,
			input:      trimmed_input.clone(),
			ifs:              " ".to_string(), // Default input field separator
			ofs:              " ".to_string(), // Default output field separator
			header_index:                   1, // Default header at row 1
			header_count:                   1, // Default 1 header row
            column_width_limits_index:      0, // Default: no column width row
            column_width_limits:         None, // Default: no column width limits
			no_divider:                 false, // Default add a divider between header & data
			divider_char:                 '-', // Default divider mad of -
			max_cell_width:                80, // Default maximum cell width
			text_format: TextFormat::Truncate, // Default truncate text
			pad_decimal_digits:         false, // Default dont pad decimal digits
			max_decimal_digits:             2, // Default maximum decimal digits
			decimal_separator:            '.', // Default decimal separator
			use_thousand_separator:     false, // Default don't add thousand separator
			thousand_separator:           ',', // Default thousand seperator char ,
			alignment:    TextAlignment::Auto, // Default align numeric columns to the right
			max_column_widths:     Vec::new(), // unclaculated maximum column widths
			header_column_widths:        None, // uncalculated column widths
			data_column_widths:          None, // uncalculated column widths
			headers:                     None, // unextracted header rows
			data:                  Vec::new(), // unextracted data rows
			numeric_columns:             None, // uncalculated numeric columns
			column_count:                None, // uncalculated column count
		}
	}
}

#[allow(dead_code)]
impl TableBuilder  {

	pub fn set_ifs(&mut self, ifs: String) -> &mut Self {
		self.ifs = ifs;
		self
	}

	pub fn set_ofs(&mut self, ofs: String) -> &mut Self {
		self.ofs = ofs;
		self
	}

	/// Sets the index of the header row in the input data.
	///
	/// # Arguments
	///
	/// * `index` - The index of the row in the data to be treated as the header row.
	pub fn set_header_index(&mut self, index: usize) -> &mut Self {
		self.header_index = std::cmp::max(0, index);
		if self.header_index > 0 {
			self.header_count = std::cmp::max(self.header_count, 1);
		}
		self
	}

	/// Sets the number of header rows to process.
	///
	/// # Arguments
	///
	/// * `count` - The number of rows at the top of the data to treat as header rows.
	#[allow(dead_code)]
	pub fn set_header_count(&mut self, count: usize) -> &mut Self {
		if self.header_index > 0 {
			self.header_count = std::cmp::max(count, 1);
		}
		self
	}

	/// Sets the index of the row in data that specifies column widths.
	///
	/// # Arguments
	///
	/// * `index` - The index of the row that provides the maximum width for each column.
	pub fn set_column_width_limits_index(&mut self, index: usize) -> &mut Self {
		self.column_width_limits_index = index;
		self
	}


	pub fn set_no_divider(&mut self, no_divider: bool) -> &mut Self {
		self.no_divider = no_divider;
		self
	}

	pub fn set_divider_char(&mut self, divider_char: char) -> &mut Self {
		self.divider_char = divider_char;
		self
	}

	pub fn set_max_cell_width(&mut self, max_cell_width: usize) -> &mut Self {
		self.max_cell_width = max_cell_width;
		self
	}

	pub fn set_text_format(&mut self, text_format: TextFormat) -> &mut Self {
		self.text_format = text_format;
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
