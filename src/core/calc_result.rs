//!

use std::fmt::{ Display, Formatter, Error };

pub enum CalcResult {
	HellYes =  2,
	Yes     =  1,
	No      =  0,
	HellNo  = -1
}

impl Display for CalcResult {

	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		use self::CalcResult::*;
		return write!(f, "{}", match *self {
			HellYes => "Hell yes!",
			Yes     => "Yes",
			No      => "No",
			HellNo  => "Hell no!"
		});
	}

}
