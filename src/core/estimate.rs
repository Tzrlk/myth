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
	HasToBe        = 10
}

impl Estimate {

	fn from_i32(index: i32) -> Option<Estimate> {
		return match index {
			0  => Some(Estimate::Impossible),
			1  => Some(Estimate::NoWay),
			2  => Some(Estimate::VeryUnlikely),
			3  => Some(Estimate::Unlikely),
			4  => Some(Estimate::FiftyFifty),
			5  => Some(Estimate::SomewhatLikely),
			6  => Some(Estimate::Likely),
			7  => Some(Estimate::VeryLikely),
			8  => Some(Estimate::NearSureThing),
			9  => Some(Estimate::SureThing),
			10 => Some(Estimate::HasToBe),
			_  => None
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
