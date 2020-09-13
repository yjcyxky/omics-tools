#[macro_use]
extern crate log;
extern crate clap;
extern crate stderrlog;

mod cmd;

use clap::{App, AppSettings, Arg};
use cmd::filter;
use std::str::FromStr;

/// A StructOpt example
#[derive(Debug)]
struct Opt {
  quiet: bool,
  verbose: usize,
  ts: Option<stderrlog::Timestamp>,
}

fn main() {
  let app = App::new("Omics Tool Suite - Bam Utility")
    .version("1.0")
    .setting(AppSettings::GlobalVersion)
    .author("Jingcheng Yang <yjcyxky@163.com>")
    .arg(
      Arg::with_name("quiet")
        .long("quiet")
        .short("q")
        .required(false)
        .takes_value(false)
        .help("Quiet mode"),
    )
    .arg(
      Arg::with_name("timestamp")
        .short("t")
        .help("prepend log lines with a timestamp")
        .takes_value(true)
        .possible_values(&["none", "sec", "ms", "ns"]),
    )
    .arg(
      Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .multiple(true)
        .help("Sets the level of verbosity"),
    )
    .about("A suite of programs for interacting with bam file.");

  // You can add more subcommands on it.
  let subcommand = app.subcommand(filter::subcommand());

  let matches = subcommand.get_matches();

  let verbose = matches.occurrences_of("verbose") as usize;
  let quiet = matches.is_present("quiet");
  let ts = matches
    .value_of("timestamp")
    .map(|v| {
      stderrlog::Timestamp::from_str(v).unwrap_or_else(|_| {
        clap::Error {
          message: "invalid value for 'timestamp'".into(),
          kind: clap::ErrorKind::InvalidValue,
          info: None,
        }
        .exit()
      })
    })
    .unwrap_or(stderrlog::Timestamp::Off);

  stderrlog::new()
    .module(module_path!())
    .modules(vec!["crate::cmd"])
    .quiet(quiet)
    .verbosity(verbose)
    .timestamp(ts)
    .init()
    .unwrap();

  if let Some(matches) = matches.subcommand_matches(filter::COMMAND_NAME) {
    filter::run(matches);
  }
}
