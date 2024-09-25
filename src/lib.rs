use prettytable::Table;

/// Builder for configuring and formatting text into columns.
///
/// This struct allows setting various options to control the formatting of text into columns,
/// including field separators, header rows, divider lines, and text width.
#[allow(dead_code)]
pub struct TableBuilder<'a> {
	input: &'a str,                        // The text to be formatted
	ifs: &'a str,                          // Input Field Separator
	ofs: &'a str,                          // Output Field Separator
	header_index: usize,                   // Which row is the header or 0 for no header
	header_count: usize,                   // Which row is the header or 0 for no header
	max_column_widths_index: usize,        // A row containing max widths of each column or 0 not to bother
	no_divider: bool,                      // Whether to include a divider line ----
	divider_char: char,                    // Divider Character ----, ====, ####
	max_cell_width: usize,                 // Maximum width of a cell
	truncate: bool,                        // truncate or wrap text to width of cell
	pad_decimal_digits: bool,              // Do we align the decimals padding with 0 at the end if necessary
	max_decimal_digits: usize,             // Limit the number of decimal places
	decimal_separator: char,               // Character to display decimals 0.0, 0,0
	use_thousand_separator: bool,          // Do we add thousands separator in output
	thousand_separator: char,              // Separator for thousands, 0,000, 0.000
	table: Table                           // prettytable instance
	max_column_widths: Vec<String>         // maximum width for each column
	column_widths: Vec<String>             // calculated width of each column
	headers: Vec<Vec<String>>              // header rows
	data: Vec<Vec<String>>                 // data rows
}

impl<'a> TableBuilder<'a> {
    /// Creates a new `Builder` with default settings.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            ifs: " ",                      // Default input field separator
            ofs: " ",                      // Default output field separator
            header_index: 1,               // Default header at row 1
            header_count: 1,               // Default 1 header row
            max_column_widths_index: 0,    // Default no max_column_widths_row
            no_divider: false,             // Default add a divider between header & data
            divider_char: '-',             // Default divider mad of -
            max_cell_width: 80,            // Default maximum cell width
			truncate: true,                // Default truncate text
            pad_decimal_digits: false,     // Default dont pad decimal digits
            max_decimal_digits: 2,         // Default maximum decimal digits
            decimal_separator: '.',        // Default decimal separator
            use_thousand_separator: false, // Default don't add thousand separator
            thousand_separator: ',',       // Default thousand seperator char ,
            table: Table::new(),           // New prettytable
            max_column_widths: Vec::new(), // unclaculated maximum column widths
            column_widths:     Vec::new(), // uncalculated column widths
            headers:           Vec::new(), // unextracted header rows
            data:              Vec::new(), // unextracted data rows
        }
    }

	pub fn set_ifs(&mut self, ifs: &'a str) -> &mut Self {
		self.ifs = ifs;
		self
	}

	pub fn set_ofs(&mut self, ofs: &'a str) -> &mut Self {
		self.ofs = ofs;
		self
	}

	pub fn set_header_index(&mut self, index: usize) -> &mut Self {
		self.header_index = index;
		self
	}

	pub fn set_header_count(&mut self, count: usize) -> &mut Self {
		self.header_count = count;
		self
	}

	pub fn set_max_column_widths_index(&mut self, index: usize) -> &mut Self {
		self.max_column_widths_index = index;
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

	pub fn set_truncate(&mut self, truncate: bool) -> &mut Self {
		self.truncate = truncate;
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

}
