use clap::{App, Arg, ArgMatches, SubCommand};

pub static COMMAND_NAME: &str = "filter";

pub fn subcommand<'b>() -> App<'static, 'b> {
  return SubCommand::with_name(COMMAND_NAME)
    .about("Filter Bam file by some flags or indicators")
    .author("Jingcheng Yang <yjcyxky@163.com>")
    .arg(
      Arg::with_name("debug")
        .short("d")
        .help("print debug information verbosely"),
    );
}

pub fn run(matches: &ArgMatches) {
  if matches.is_present("debug") {
    println!("Printing debug info...");
  } else {
    println!("Printing normally...");
  }
}
