extern crate exitcode;
extern crate omics_tools;

use log::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use omics_tools::utils::bam_cigar;
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

  info!("{} - Cigar Expression: {:?}", module_path!(), cigar_exp);

  if Path::new(inputpath).exists() {
    filter(inputpath, cigar_exp);
  } else {
    error!("{} - Not Found: {:?}", module_path!(), inputpath);
  }
}

pub fn filter(inputpath: &str, cigar_exp: &str) {
  let mut reader = Reader::from_path(inputpath).unwrap();
  let header = header::Header::from_template(reader.header());
  let mut writer = Writer::from_stdout(&header, Format::SAM).unwrap();
  writer.set_threads(10).unwrap();

  for record in reader.records() {
    let record = record.unwrap();
    let cigar = record.cigar();

    debug!(
      "{} - Cigar Expression Results: {:?} {:?}",
      module_path!(),
      std::str::from_utf8(record.qname()).unwrap(),
      bam_cigar::exec(&cigar, cigar_exp)
    );

    if bam_cigar::exec(&cigar, cigar_exp) {
      writer.write(&record).unwrap();
    }
  }
}
