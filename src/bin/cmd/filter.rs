extern crate exitcode;
extern crate omics_tools;

use log::*;
use omics_tools::bam::cigar as bam_cigar;
use rust_htslib::bam::{header, Format, Read, Reader, Writer};
use std::path::{Path};
use structopt::StructOpt;

/// Filter Bam file by some flags or indicators
#[derive(StructOpt, PartialEq, Debug)]
#[structopt(setting=structopt::clap::AppSettings::ColoredHelp, name="Omics Tool Suite - Bam Utility - filter", author="Jingcheng Yang <yjcyxky@163.com>")]
pub struct Arguments {
  /// Bam file to process
  #[structopt(name="FILE")]
  input: String,

  /// A format for output file.
  #[structopt(name="format", short="O", long="format", possible_values=&["BAM", "SAM"], default_value="BAM")]
  format: String,

  /// A filtered expression for cigar. e.g. each(S) > 100
  #[structopt(name="cigar", short="c", long="cigar")]
  cigar: String
}

pub fn run(args: &Arguments) {
  info!("{} - Cigar Expression: {:?}", module_path!(), args.cigar);

  if Path::new(&args.input).exists() {
    filter(&args.input, &args.cigar, &args.format);
  } else {
    error!("{} - Not Found: {:?}", module_path!(), args.input);
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
