//!

#![feature(plugin)]
#![feature(test)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use]
mod util;

mod myth;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate clap;

extern crate phf;
extern crate rand;
extern crate sqlite3;

use myth::Myth;

fn main() {

	/// Parse the console arguments and attempt to make use of them.
	let args = Myth::build_args()
		.get_matches();

	Myth::execute(&args);

}
