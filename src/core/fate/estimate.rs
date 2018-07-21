//!

use std::fmt::{ Display, Formatter, Error };

pub enum Estimate {
	Impossible     = -8,
	NoWay          = -6,
	VeryUnlikely   = -4,
	Unlikely       = -2,
	FiftyFifty     =  0,
	Likely         =  2,
	VeryLikely     =  4,
	SureThing      =  6,
	HasToBe        =  8,
}

impl From<i32> for Estimate {
	fn from(index: i32) -> Self {
		use self::Estimate::*;
		return match index {
			-8 => Impossible,
			-6 => NoWay,
			-4 => VeryUnlikely,
			-2 => Unlikely,
			 0 => FiftyFifty,
			 2 => Likely,
			 4 => VeryLikely,
			 6 => SureThing,
			 8 => HasToBe,
			_  => panic!(),
		}
	}
}

impl Display for Estimate {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		use self::Estimate::*;
		return write!(f, "{}", match *self {
			Impossible     => "Impossible",
			NoWay          => "No Way",
			VeryUnlikely   => "Very Unlikely",
			Unlikely       => "Unlikely",
			FiftyFifty     => "Fifty-Fifty",
			Likely         => "Likely",
			VeryLikely     => "Very likely",
			SureThing      => "Sure Thing",
			HasToBe        => "Has To Be",
		});
	}
}
