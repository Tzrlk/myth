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
			if self.random_event { "with Event" }   else { "" }
		].concat();

		return write!(f, "{}", message);

	}

}
