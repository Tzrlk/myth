//!

use clap::{ App, ArgMatches };

pub trait CliNode {

	fn build_args<'a, 'b>() -> App<'a, 'b>;

	fn execute(args: &ArgMatches);

}
