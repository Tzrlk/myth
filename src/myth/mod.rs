//! The main entry point for the myth cli.

mod char;
mod init;
mod scene;
mod status;

use clap::{ App, Arg, ArgMatches };
use log4rs::config::{ Config, Appender, Root };
use log4rs::append::console::ConsoleAppender;
use log::LogLevelFilter;

pub struct Myth;

impl CliNode for Myth {

	fn build_args<'a, 'b>() -> App<'a, 'b> {

		return App::new(env!("CARGO_PKG_NAME"))
			.version(crate_version!())
			.author(crate_author!())
			.about(env!("CARGO_PKG_DESCRIPTION"))

			.arg(Arg::with_name("verbose")
				.help("Sets the level of logging.")
				.long("verbose")
				.short("v")
				.multiple(true))

			.arg(Arg::with_name("quiet")
				.help("Decreases the level of logging.")
				.long("quiet")
				.short("q")
				.multiple(true));

	}

	fn execute(args: &ArgMatches) {

		/// Determine the level of logging from the verbose and quiet flags
		let quietness = args.occurrences_of("q");
		let verbosity = args.occurrences_of("v");
		let log_level = ( verbosity - quietness ) as i32;
		build_logging(match log_level {
			-3 => LogLevelFilter::Off,
			-2 => LogLevelFilter::Error,
			-1 => LogLevelFilter::Warn,
			0 => LogLevelFilter::Info,
			1 => LogLevelFilter::Debug,
			_ => LogLevelFilter::Trace
		});

		/// Figure out what subcommand (if any) has been called.
		match args.subcommand() {
			(&_, _) => println!("{}", args.usage())
		}

	}

}

/// Sets up the logging with the provided level.
fn build_logging(level: LogLevelFilter) -> Config {

	/// This needs to be like this for the Box::new invocation.
	let stdout = ConsoleAppender::builder().build();

	return Config::builder()

		.appender(Appender::builder()
			.build("stdout", Box::new(stdout)))

		.build(Root::builder()
			.appender("stdout")
			.build(level))

		.unwrap();

}
