//! The main entry point for the myth cli.

mod char;
mod check;
mod init;
mod scene;
mod status;

use clap::{ App, Arg, ArgMatches };
use log4rs::config::{ Config, Appender, Root };
use log4rs::append::console::ConsoleAppender;
use log::LogLevelFilter;

use ::util::cli_node::CliNode;
type CharCmd = self::char::Cmd;
type CheckCmd = self::check::Cmd;
type InitCmd = self::init::Cmd;
type SceneCmd = self::scene::Cmd;
type StatusCmd = self::status::Cmd;

pub struct Cmd;

impl CliNode for Cmd {

	fn build_args<'a, 'b>() -> App<'a, 'b> {

		return App::new(env!("CARGO_PKG_NAME"))
			.version(crate_version!())
			.author(crate_authors!())
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
				.multiple(true))

			.subcommand(CharCmd::build_args())
			.subcommand(CheckCmd::build_args())
			.subcommand(InitCmd::build_args())
			.subcommand(SceneCmd::build_args())
			.subcommand(StatusCmd::build_args())

		;

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
			("char",   Some(sub)) => CharCmd::execute(sub),
			("check",  Some(sub)) => CheckCmd::execute(sub),
			("init",   Some(sub)) => InitCmd::execute(sub),
			("scene",  Some(sub)) => SceneCmd::execute(sub),
			("status", Some(sub)) => StatusCmd::execute(sub),
			(&_, _)               => println!("{}", args.usage())
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
