//!

use std::fmt::{ Display, Formatter, Error };

pub enum Estimate {
	Impossible     = 0,
	NoWay          = 1,
	VeryUnlikely   = 2,
	Unlikely       = 3,
	FiftyFifty     = 4,
	SomewhatLikely = 5,
	Likely         = 6,
	VeryLikely     = 7,
	NearSureThing  = 8,
	SureThing      = 9,
	HasToBe        = 10,
}

impl Estimate {
}

impl From<i32> for Estimate {
	fn from(index: i32) -> Self {
		return match index {
			0  => Estimate::Impossible,
			1  => Estimate::NoWay,
			2  => Estimate::VeryUnlikely,
			3  => Estimate::Unlikely,
			4  => Estimate::FiftyFifty,
			5  => Estimate::SomewhatLikely,
			6  => Estimate::Likely,
			7  => Estimate::VeryLikely,
			8  => Estimate::NearSureThing,
			9  => Estimate::SureThing,
			10 => Estimate::HasToBe,
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
			SomewhatLikely => "Somewhat Likely",
			Likely         => "Likely",
			VeryLikely     => "Very likely",
			NearSureThing  => "Near Sure Thing",
			SureThing      => "Sure Thing",
			HasToBe        => "Has To Be",
		});
	}
}
