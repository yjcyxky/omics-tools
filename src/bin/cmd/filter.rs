extern crate exitcode;
extern crate omics_tools;

use clap::{App, Arg, ArgMatches, SubCommand};
use log::*;
use omics_tools::bam::cigar as bam_cigar;
use rust_htslib::bam::{header, Format, Read, Reader, Writer};
use std::path::Path;

pub static COMMAND_NAME: &str = "filter";

pub fn subcommand<'b>() -> App<'static, 'b> {
  return SubCommand::with_name(COMMAND_NAME)
    .about("Filter Bam file by some flags or indicators")
    .author("Jingcheng Yang <yjcyxky@163.com>")
    .arg(
      Arg::with_name("INPUT")
        .help("A bam file...")
        .required(true)
        .index(1),
    )
    .arg(
      Arg::with_name("format")
        .short("O")
        .long("format")
        .takes_value(true)
        .possible_values(&["BAM", "SAM"])
        .default_value("BAM")
        .help("A format for output file.")
    )
    .arg(
      Arg::with_name("cigar")
        .long("cigar")
        .short("c")
        .value_name("Cigar Expression")
        .takes_value(true)
        .help("A filtered expression for cigar. e.g. each(S) > 100"),
    );
}

pub fn run(matches: &ArgMatches) {
  let inputpath = matches.value_of("INPUT").unwrap();
  let cigar_exp = matches.value_of("cigar").unwrap();
  let format = matches.value_of("format").unwrap();

  info!("{} - Cigar Expression: {:?}", module_path!(), cigar_exp);

  if Path::new(inputpath).exists() {
    filter(inputpath, cigar_exp, format);
  } else {
    error!("{} - Not Found: {:?}", module_path!(), inputpath);
  }
}

pub fn filter(inputpath: &str, cigar_exp: &str, format: &str) {
  let mut reader = Reader::from_path(inputpath).unwrap();
  let header = header::Header::from_template(reader.header());
  let format = if format == "BAM" { Format::BAM } else { Format::SAM };
  let mut writer = Writer::from_stdout(&header, format).unwrap();
  writer.set_threads(10).unwrap();

  for record in reader.records() {
    let record = record.unwrap();
    let cigar = record.cigar();
    bam_cigar::check_expr(cigar_exp);
    let results = bam_cigar::exec(&cigar, cigar_exp);

    debug!(
      "{} - Cigar Expression Results: {:?} {:?}",
      module_path!(),
      std::str::from_utf8(record.qname()).unwrap(),
      results,
    );

    if results {
      writer.write(&record).unwrap();
    }
  }
}
