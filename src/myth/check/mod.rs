//!

use clap::{ App, ArgMatches, SubCommand, Arg };

use ::util::cli_node::CliNode;
use ::core::calc;

pub struct Cmd;

impl CliNode for Cmd {

	fn build_args<'a, 'b>() -> App<'a, 'b> {

		return SubCommand::with_name("check")
			.about("Performs a table check")

			.arg(Arg::with_name("estimate")
				.help("The base likelihood of a yes response")
				.long("estimate")
				.short("e")
				.possible_values(&[ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10" ])
				.default_value("5")
				.takes_value(true))

			.arg(Arg::with_name("chaos")
				.help("How crazy things have gotten")
				.long("chaos")
				.short("c")
				.possible_values(&[ "0", "1", "2", "3", "4", "5", "6", "7", "8", "9" ])
				.default_value("0"))

			.arg(Arg::with_name("save")
				.help("Save the result to the database")
				.long("save")
				.short("s")
				.takes_value(false))

			.arg(Arg::with_name("question")
				.help("The question to get the answer for"))

		;

	}

	fn execute(args: &ArgMatches) {

		let estimate = args.value_of("estimate").unwrap().parse::<i32>().unwrap();
		let chaos = args.value_of("chaos").unwrap().parse::<i32>().unwrap();

		let result = calc::calc(estimate, chaos).unwrap();
		print!("{}", result);

	}

}
