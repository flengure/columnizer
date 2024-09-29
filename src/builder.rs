use clap::{ Args };
use crate::text::{ clean, Alignment, Frame };
use crate::io::{ input_or_stdin };
use prettytable::{ Table };

/// Builder for configuring and formatting text into columns.
///
/// This struct allows setting various options to control the formatting of text into columns,
/// including field separators, header rows, divider lines, and text width.
#[derive(Args, Clone)]
pub struct TableBuilder {

	/// The text input to be formatted.
	pub input: Option<String>,

	/// Input Field Separator
	#[arg(default_value = " ", long, short)]
	pub ifs: String,

	/// Output Field Separator
	#[arg(default_value = " ", long, short)]
	pub ofs: String,

	/// Header rows start on this row
	#[arg(default_value_t = 1, long, short = 'r')]
	pub header_index: usize,

	/// Number of header rows.
	#[arg(default_value_t = 1, long, short = 'c')]
	pub header_count: usize,

	// will not be included in output, unless as part of headers
	/// A row specifying maximum column widths
	#[arg(default_value_t = 0, long, short = 'w')]
    pub column_width_limits_index: usize,

	/// Disable divider line between headers and data
	#[arg(short, long)]
    pub no_divider: bool,

	/// Divider line made of this character
	#[arg(default_value_t = '-', long, short)]
    pub divider_char: char,

	/// Maximimu width (display characters) of any cell
	#[arg(default_value_t = 48)]
	#[arg(short, long)]
	pub max_cell_width: usize,

	/// How we frame text 
	#[arg(default_value_t = Frame::TRUNCATE)]
	#[arg(value_enum)]
	#[arg(short, long)]
	pub frame: Frame,

	/// Disable ellipsis for truncated text
	#[arg(short, long)]
    pub no_ellipsis: bool,

	/// Use decimal precision for numbers
	#[arg(short, long)]
	pub pad_decimal_digits: bool,

	/// Limit decimal precision for numbers
	#[arg(default_value_t = 2)]
	#[arg(short, long)]
	pub max_decimal_digits: usize,

	/// Decimal point character
	#[arg(default_value_t = '.')]
	#[arg(short, long)]
	pub decimal_separator: char,

	/// Use thousands grouping for numbers
	#[arg(short, long)]
	pub use_thousand_separator: bool,

	/// thousands grouping character
	#[arg(default_value_t = ',')]
	#[arg(short, long)]
	pub thousand_separator: char,

	#[arg(default_value_t = Alignment::AUTO)]
	#[arg(value_enum)]
	#[arg(short, long)]
	pub alignment: Alignment,

	/// These field are computed and cached
	/// An instance of `Table` from the `prettytable` crate.
	/// to collect and apply final formatting
	#[clap(skip)]
	pub table: Option<Table>,

	/// Column width limits specified in the data row.
	///
	/// This contains the column widths as specified in a special row in the data. These widths
	/// are clamped by global maximum column width col_width_max.
	#[clap(skip)]
	pub column_width_limits: Option<Vec<usize>>,

    /// Column widths specified in the data row.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
	#[clap(skip)]
    pub header_column_widths: Option<Vec<usize>>,

    /// Column widths specified in the data row.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
	#[clap(skip)]
    pub data_column_widths: Option<Vec<usize>>,

    /// Column widths specified for both header abd data rows.
    ///
    /// These are final column widths for each column, taking the maximum width for all rows in the column
    /// then limiting it by col_width_limits
	#[clap(skip)]
    pub column_widths: Option<Vec<usize>>,

    /// The header rows cached from the input data.
    ///
    /// extracted from the input, by considering, header_index and header_count
    /// this will store the header rows.
	#[clap(skip)]
    pub headers: Option<Vec<Vec<String>>>,

    /// Cached data rows from the input.
    ///
    /// This stores the rows parsed from the input data after excluding the headers and column width limits.
	#[clap(skip)]
	pub data: Option<Vec<Vec<String>>>,

    /// Cached status indicating whether each column is numeric.
    ///
    /// This indicates whether each column in the data is numeric (`true`) or text (`false`).
    /// This helps in formatting and alignment.
	#[clap(skip)]
    pub numeric_columns: Option<Vec<bool>>,

	/// number of columns after parsing data
	#[clap(skip)]
	pub column_count:    Option<usize>,
}

#[allow(dead_code)]
impl TableBuilder  {
	/// Creates a new `Builder` with default settings.
	pub fn new(input: Option<String>) -> Self {

		let input_data = input_or_stdin(input.as_deref(), 5, 500);
		let cleaned = clean(Some(&input_data)).clone();

		Self {
			input:              Some(cleaned), // Sanitized input trim_and_strip_blank_lines
			ifs:              " ".to_string(), // Default input field separator
			ofs:              " ".to_string(), // Default output field separator
			header_index:                   1, // Default header at row 1
			header_count:                   1, // Default 1 header row
            column_width_limits_index:      0, // Default no column width row
			no_divider:                 false, // Default add a divider between header & data
			divider_char:                 '-', // Default divider mad of -
			max_cell_width:                80, // Default maximum cell width
			frame:            Frame::TRUNCATE, // Default truncate text
			no_ellipsis:                false, // Default no ellipsis on truncate
			pad_decimal_digits:         false, // Default dont pad decimal digits
			max_decimal_digits:             2, // Default maximum decimal digits
			decimal_separator:            '.', // Default decimal separator
			use_thousand_separator:     false, // Default don't add thousand separator
			thousand_separator:           ',', // Default thousand seperator char ,
			alignment:        Alignment::AUTO, // Default align numeric columns to the right
			table:                       None, // Unknown prettytable
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
