//!

use std::fmt::{ Display, Formatter, Error };

pub enum CheckResult {
	Anger,
	Sadness,
	Fear,
	DisfavoursThread,
	DisfavoursPc,
	FocusNpc,
	FavoursNpc,
	FocusPc,
	DisfavoursNpc,
	FocusThread,
	FavoursPc,
	FavoursThread,
	Courage,
	Happiness,
	Calm
}

impl From<i32> for CheckResult {
	fn from(index: i32) -> Self {
		use self::CheckResult::*;
		return match index {
			0...4   => Anger,
			5       => Sadness,
			6       => Fear,
			7       => DisfavoursThread,
			8       => DisfavoursPc,
			9       => FocusNpc,
			10      => FavoursNpc,
			11      => FocusPc,
			12      => DisfavoursNpc,
			13      => FocusThread,
			14      => FavoursPc,
			15      => FavoursThread,
			16      => Courage,
			17      => Happiness,
			18...22 => Calm,
			_       => panic!()
		}
	}
}

impl Display for CheckResult {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		use self::CheckResult::*;
		return write!(f, "{}", match *self {
			Anger            => "Anger",
			Sadness          => "Sadness",
			Fear             => "Fear",
			DisfavoursThread => "Disfavours Thread",
			DisfavoursPc     => "Disfavours PC",
			FocusNpc         => "Focus NPC",
			FavoursNpc       => "Favours NPC",
			FocusPc          => "Focus PC",
			DisfavoursNpc    => "Disfavours NPC",
			FocusThread      => "Focus Thread",
			FavoursPc        => "Favours PC",
			FavoursThread    => "Favours Thread",
			Courage          => "Courage",
			Happiness        => "Happiness",
			Calm             => "Calm",
		});
	}
}
