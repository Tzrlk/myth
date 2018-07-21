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

impl Estimate {
}

impl From<i32> for Estimate {
	fn from(index: i32) -> Self {
		return match index {
			-8 => Estimate::Impossible,
			-6 => Estimate::NoWay,
			-4 => Estimate::VeryUnlikely,
			-2 => Estimate::Unlikely,
			 0 => Estimate::FiftyFifty,
			 2 => Estimate::Likely,
			 4 => Estimate::VeryLikely,
			 6 => Estimate::SureThing,
			 8 => Estimate::HasToBe,
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
