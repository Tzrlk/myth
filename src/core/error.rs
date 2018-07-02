//!

use std::error::Error as StdError;
use std::convert::From;

//use rusqlite::Error as RusqlError;

use super::error_value_required::ErrorValueRequired;

pub enum Error {
	ValueRequired(ErrorValueRequired)//,
//	Rusql(RusqlError)
}

impl From<ErrorValueRequired> for Error {
	fn from(error: ErrorValueRequired) -> Error {
		return Error::ValueRequired(error);
	}
}

//impl From<RusqlError> for Error {
//	fn from(error: RusqlError) -> Error {
//		return Error::Rusql(error);
//	}
//}
