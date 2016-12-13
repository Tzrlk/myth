//!

use std::error::Error as StdError;
use std::convert::From;

use rusqlite::Error as RusqlError;

use super::error_value_required::ErrorValueRequired;

pub enum Error {
	ValueRequired(ErrorValueRequired),
	Rusql(RusqlError)
}

impl <T: StdError> From<T> for Error {
	fn from(error: T) -> Error {
		return match error {
			ErrorValueRequired => Error::ValueRequired(error as ErrorValueRequired),
			RusqlError         => Error::Rusql(error as RusqlError),
			_                  => panic!("{}", error)
		};
	}
}
