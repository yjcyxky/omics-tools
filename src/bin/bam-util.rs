#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;

mod cmd;

use structopt::StructOpt;
use cmd::filter;

/// A suite of programs for interacting with bam file
#[derive(StructOpt, Debug)]
#[structopt(setting=structopt::clap::AppSettings::ColoredHelp, name = "Omics Tool Suite - Bam Utility", author="Jingcheng Yang <yjcyxky@163.com>")]
struct Opt {
  /// A flag which control whether show more messages, true if used in the command line
  #[structopt(short="q", long="quiet")]
  quiet: bool,

  /// The number of occurrences of the `v/verbose` flag
  /// Verbose mode (-v, -vv, -vvv, etc.)
  #[structopt(short="v", long="verbose", parse(from_occurrences))]
  verbose: usize,

  /// Timestamp(sec, ms, ns, none)
  #[structopt(short="t", long="timestamp")]
  ts: Option<stderrlog::Timestamp>,

  #[structopt(subcommand)]
  cmd: SubCommands
}

#[derive(Debug, PartialEq, StructOpt)]
enum SubCommands {
  #[structopt(name="filter")]
  Filter(filter::Arguments)
}

fn main() {
  let opt = Opt::from_args();

  stderrlog::new()
    .module(module_path!())
    .modules(vec!["omics_tools"])
    .quiet(opt.quiet)
    .verbosity(opt.verbose)
    .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
    .init()
    .unwrap();

  match opt.cmd {
    SubCommands::Filter(filter) => {
      filter::run(&filter);
    }
  }
}
