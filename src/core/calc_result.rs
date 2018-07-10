//!

use std::fmt::{ Display, Formatter, Error };

pub struct CalcResult {
	pub yes_result:   bool,
	pub exceptional:  bool,
	pub random_event: bool
}

impl Display for CalcResult {

	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

		let message = &[
			if self.exceptional  { "Exceptional " } else { "" },
			if self.yes_result   { "Yes" }          else { "No" },
			if self.random_event { " with Event" }  else { "" }
		].concat();

		return write!(f, "{}", message);

	}

}

#[cfg(test)]
mod tests {
	use super::*;

	fn format_result(yes_result: bool, exceptional: bool, random_event: bool) -> String {
		return format!("{}", CalcResult { yes_result, exceptional, random_event });
	}

	describe! calc_result {

		it "constructs display sentences based on contained data" {
			assert_eq!(format_result(false, false, false), "No");
			assert_eq!(format_result(true,  false, false), "Yes");
			assert_eq!(format_result(false, true,  false), "Exceptional No");
			assert_eq!(format_result(false, false, true ), "No with Event");
			assert_eq!(format_result(true,  true,  false), "Exceptional Yes");
			assert_eq!(format_result(true,  false, true ), "Yes with Event");
			assert_eq!(format_result(false, true,  true ), "Exceptional No with Event");
			assert_eq!(format_result(true,  true,  true ), "Exceptional Yes with Event");
		}

	}

}
