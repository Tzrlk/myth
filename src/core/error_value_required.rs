//! Defines errors for sql stuff.

use std::error::Error as StdError;
use std::fmt::{ Display, Formatter };
use std::fmt::Error as DisplayError;

#[derive(Debug)]
pub struct ErrorValueRequired {
	name: String
}

impl ErrorValueRequired {

	fn new(name: &str) -> ErrorValueRequired {
		return ErrorValueRequired { name: name.to_string() };
	}

}

impl StdError for ErrorValueRequired {
	fn description(&self) -> &str {
		return "Unable to proceed with operation without required value";
	}
}

impl Display for ErrorValueRequired {
	fn fmt(&self, f: &mut Formatter) -> Result<(), DisplayError> {
		return write!(f, "{}: {}", self.description(), self.name);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	describe! error_value_required {

		it "formats its description along with the value name" {
			let msg = format!("{}", ErrorValueRequired::new("my_value"));
			assert_eq!(msg, "Unable to proceed with operation without required value: my_value");
		}

	}

}
