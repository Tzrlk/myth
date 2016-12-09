//!

use clap::{ App, ArgMatches, SubCommand };

use ::util::cli_node::CliNode;

pub struct Cmd;

impl CliNode for Cmd {

	fn build_args<'a, 'b>() -> App<'a, 'b> {

		return SubCommand::with_name("status")
			.about("Displays the status of the current story (if any)")

		;

	}

	fn execute(args: &ArgMatches) {
		unimplemented!()
	}

}
