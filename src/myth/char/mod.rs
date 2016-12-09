//!

use clap::{ App, ArgMatches, SubCommand, Arg };

use ::util::cli_node::CliNode;

pub struct Cmd;

impl CliNode for Cmd {

	fn build_args<'a, 'b>() -> App<'a, 'b> {

		return SubCommand::with_name("char")
			.about("Manipulates characters both in and out of scene")

			.arg(Arg::with_name("selector"))

		;

	}

	fn execute(args: &ArgMatches) {
		unimplemented!()
	}

}
