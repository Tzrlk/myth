//! Defines errors for sql stuff.

use std::error::Error as StdError;
use std::fmt::{ Display, Formatter };
use std::fmt::Error as DisplayError;

#[derive(Debug)]
pub struct ErrorValueRequired {
	name: String
}

impl ErrorValueRequired {
	fn new(name: String) -> ErrorValueRequired {
		return ErrorValueRequired {
			name: name
		};
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
