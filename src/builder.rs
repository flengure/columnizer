use crate::cell::{ TextAlignment, TextFormat };
use crate::text::trim_and_strip_blanks;
use prettytable::format;
use prettytable::Table;

/// Builder for configuring and formatting text into columns.
///
/// This struct allows setting various options to control the formatting of text into columns,
/// including field separators, header rows, divider lines, and text width.
#[allow(dead_code)]
pub struct TableBuilder {

	/// The text input to be formatted.
	pub input: String,

	/// The string used to separate input fields.
	///
	/// This is the delimiter used to split fields in the input data.
	/// For example, a comma (`,`) might be used to separate CSV fields.
	pub ifs: String,

	/// The string used to separate output fields.
	///
	/// This character is inserted between fields in the output data.
	/// It is useful for formatting the output in a desired way.
	pub ofs: String,

	/// The index of the row used for headers.
	///
	/// This specifies which row in the data is used as the header row.
	/// Indexing is 1-based, so the first row is `1`.
	pub header_index: usize,

	/// The number of header rows.
	///
	/// This specifies how many rows at the top of the data are considered as headers.
	/// Useful when multiple rows are used to describe columns.
	pub header_count: usize,

    /// The row in the data that specifies column widths.
    ///
    /// This specifies which row contains the column widths. The width values in this
    /// row can override the global width settings.
    pub column_width_limits_index: usize,

	/// Whether to exclude a divider line between the header and the data.
	///
	/// If `true`, no divider line will be drawn between the header and the rest of the data.
	/// If `false`, a divider line will be added to visually separate the header from the data.
    pub no_divider: bool,

    /// The character used to create a divider line between the header and the data.
    ///
    /// This character is repeated to form a divider line separating the header from the data rows.
    pub divider_char: char,

    /// The global maximum allowed width for any cell.
    ///
    /// This is the maximum width that any cell can have. If a cell's content
    /// exceeds this width, it will be truncated or wrapped depending on other settings.
	pub max_cell_width: usize,

    /// Whether to wrap text or truncate it, leave it alone.
    ///
	/// Enum TextFormat::{Truncate, Wrap, NoFormat}
	pub text_format: TextFormat,

    /// Whether to use ellipsis when truncating text.
    ///
    /// If `true`, an ellipsis (`...`) will be added to the end of truncated text.
    /// If `false`, text will be cut off without ellipsis.
    pub ellipsis: bool,

	pub pad_decimal_digits:       bool, // Do we align the decimals padding with 0 at the end if necessary
	pub max_decimal_digits:      usize, // Limit the number of decimal places
	pub decimal_separator:        char, // Character to display decimals 0.0, 0,0
	pub use_thousand_separator:   bool, // Do we add thousands separator in output

    /// The separator character for thousands in numeric fields.
    ///
    /// This is used by `numfmt` to format numeric values. Common separators are `','` or `'.'`.
	pub thousand_separator: char,

    /// Whether align right to width,
	/// align right based on wether the content is text or numeric, or
	/// dont align.
    ///
	/// Enum TextAlignment::{Auto, Right, NoAlignment}
	pub alignment: TextAlignment,

	/// These field are computed and cached f
    /// An instance of `Table` from the `prettytable` crate.
	/// to collect and apply final formatting
	pub table: Table,

	/// Column width limits specified in the data row.
	///
	/// This contains the column widths as specified in a special row in the data. These widths
	/// are clamped by global maximum column width col_width_max.
	pub column_width_limits: Option<Vec<usize>>,

    /// Column widths specified in the data row.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
    pub header_column_widths: Option<Vec<usize>>,

    /// Column widths specified in the data row.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
    pub data_column_widths: Option<Vec<usize>>,

    /// Column widths specified for both header abd data rows.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
    pub column_widths: Option<Vec<usize>>,

    /// The header rows cached from the input data.
    ///
    /// extracted from the input, by considering, header_index and header_count
    /// this will store the header rows.
    pub headers: Option<Vec<Vec<String>>>,

    /// Cached data rows from the input.
    ///
    /// This stores the rows parsed from the input data after excluding the headers and column width limits.
	pub data: Option<Vec<Vec<String>>>,

    /// Cached status indicating whether each column is numeric.
    ///
    /// This indicates whether each column in the data is numeric (`true`) or text (`false`).
    /// This helps in formatting and alignment.
    pub numeric_columns: Option<Vec<bool>>,
	pub column_count:    Option<usize>, // number of columns after parsing data
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
            column_width_limits_index:      0, // Default no column width row
			no_divider:                 false, // Default add a divider between header & data
			divider_char:                 '-', // Default divider mad of -
			max_cell_width:                80, // Default maximum cell width
			text_format: TextFormat::Truncate, // Default truncate text
			ellipsis:                   false, // Default no ellipsis on truncate
			pad_decimal_digits:         false, // Default dont pad decimal digits
			max_decimal_digits:             2, // Default maximum decimal digits
			decimal_separator:            '.', // Default decimal separator
			use_thousand_separator:     false, // Default don't add thousand separator
			thousand_separator:           ',', // Default thousand seperator char ,
			alignment:    TextAlignment::Auto, // Default align numeric columns to the right
            column_width_limits:         None, // Unknown column width limits
			header_column_widths:        None, // Unknown header column widths
			data_column_widths:          None, // Unknown data column widths
			column_widths:               None, // Unknown column widths
			headers:                     None, // Unknown header rows
			data:                        None, // Unknown data rows
			numeric_columns:             None, // Unknown numeric columns
			column_count:                None, // Unknown column count
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

	pub fn set_use_thousand_separator(mut self, use_thousand_separator: bool) -> Self {
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
