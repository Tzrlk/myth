//!

use std::fmt::{ Display, Formatter, Error };

pub enum Chaos {
	High    = 6,
	MedHigh = 5,
	MedLow  = 4,
	Low     = 3
}

impl Chaos {

	pub fn to_mod(&self) -> i32 {
		use self::Chaos::*;
		match *self {
			High => -2,
			Low  =>  2,
			_    =>  0
		}
	}

}

impl From<i32> for Chaos {
	fn from(index: i32) -> Self {
		use self::Chaos::*;
		return match index {
			3 => Low,
			4 => MedLow,
			5 => MedHigh,
			6 => High,
			_ => panic!()
		}
	}
}

impl Display for Chaos {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		use self::Chaos::*;
		return write!(f, "{}", match *self {
			Low =>     "Low",
			MedLow =>  "Medium-low",
			MedHigh => "Medium-high",
			High    => "High",
		});
	}
}
