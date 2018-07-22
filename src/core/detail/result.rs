//!

use std::fmt::{ Display, Formatter, Error };
use super::super::{ Thread, Player, NonPlayer };

pub enum CheckResult {
	Anger,
	Sadness,
	Fear,
	DisfavoursThread(Option<Thread>),
	DisfavoursPc(Option<Player>),
	FocusNpc(Option<NonPlayer>),
	FavoursNpc(Option<NonPlayer>),
	FocusPc(Option<Player>),
	DisfavoursNpc(Option<NonPlayer>),
	FocusThread(Option<Thread>),
	FavoursPc(Option<Player>),
	FavoursThread(Option<Thread>),
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
			7       => DisfavoursThread(None),
			8       => DisfavoursPc(None),
			9       => FocusNpc(None),
			10      => FavoursNpc(None),
			11      => FocusPc(None),
			12      => DisfavoursNpc(None),
			13      => FocusThread(None),
			14      => FavoursPc(None),
			15      => FavoursThread(None),
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
			Anger                    => "Anger",
			Sadness                  => "Sadness",
			Fear                     => "Fear",
			DisfavoursThread(thread) => format!("Disfavours Thread ({})", thread.map_or_else(|| "?".to_string(), |it| it.name)),
			DisfavoursPc(pc)         => format!("Disfavours PC ({})", pc.map_or_else(|| "?".to_string(), |it| it.name)),
			FocusNpc(npc)            => format!("Focus NPC ({})", npc.map_or_else(|| "?".to_string(), |it| it.name)),
			FavoursNpc(npc)          => format!("Favours NPC ({})", npc.map_or_else(|| "?".to_string(), |it| it.name)),
			FocusPc(pc)              => format!("Focus PC ({})", pc.map_or_else(|| "?".to_string(), |it| it.name)),
			DisfavoursNpc(npc)       => format!("Disfavours NPC ({})", npc.map_or_else(|| "?".to_string(), |it| it.name)),
			FocusThread(thread)      => format!("Focus Thread ({})", thread.map_or_else(|| "?".to_string(), |it| it.name)),
			FavoursPc(pc)            => format!("Favours PC ({})", pc.map_or_else(|| "?".to_string(), |it| it.name)),
			FavoursThread(thread)    => format!("Favours Thread ({})", thread.map_or_else(|| "?".to_string(), |it| it.name)),
			Courage                  => "Courage",
			Happiness                => "Happiness",
			Calm                     => "Calm",
		});
	}
}
