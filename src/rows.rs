use crate::cell::{ CellFormatter, TextAlignment, TextFormat };
use crate::table::TableBuilder;


impl TableBuilder {

	pub fn column_count(&mut self) -> usize {
		if let Some(column_count) = self.column_count {
			return column_count;
		}

		let normalized_content = self.formatted_text
			.replace(self.thousand_separator, "")
			.replace(self.decimal_separator, ".");

		self.is_numeric = Some(normalized_content.parse::<f64>().is_ok());
		self.column_count.unwrap()
	}



}
