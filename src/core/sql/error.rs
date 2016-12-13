//! Defines errors for sql stuff.

use std::error::{ Error };
use std::fmt::{ Display, Formatter };
use std::fmt::Error as DisplayError;

#[derive(Debug)]
pub struct RequiredValueError {
	name: &'static str
}

impl Error for RequiredValueError {
	fn description(&self) -> &str {
		return "Unable to proceed with operation without required value";
	}
}

impl Display for RequiredValueError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), DisplayError> {
		return write!(f, "{}: {}", self.description(), self.name);
	}
}
