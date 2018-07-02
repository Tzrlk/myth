//!

//#![feature(plugin)]
//#![feature(test)]
//#![cfg_attr(test, plugin(stainless))]

#![allow(unused_variables)]

#[macro_use]
mod util;

mod myth;
mod core;

#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate clap;

extern crate num;
extern crate phf;
extern crate rand;
extern crate time;
//extern crate rusqlite;

use ::util::cli_node::CliNode;
type MythCmd = self::myth::Cmd;

fn main() {

	// Parse the console arguments and attempt to make use of them.
	let args = MythCmd::build_args()
		.get_matches();

	MythCmd::execute(&args);

}
