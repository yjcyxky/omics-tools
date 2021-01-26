extern crate exitcode;
extern crate omics_tools;

use log::*;
use omics_tools::vcf::convertor;
use omics_tools::vcf::util;
use std::path::Path;
use structopt::StructOpt;

/// Convert VCF file to a SQL Database File
#[derive(StructOpt, PartialEq, Debug)]
#[structopt(setting=structopt::clap::AppSettings::ColoredHelp, name="Omics Tool Suite - VCF Utility - makedb", author="Jingcheng Yang <yjcyxky@163.com>")]
pub struct Arguments {
  /// VCF file to process
  #[structopt(name = "FILE")]
  input: String,

  /// Output file.
  #[structopt(name = "output", short = "o", long = "output")]
  output: String,
}

pub fn run(args: &Arguments) {
  info!("{} - Make database: {:?}", module_path!(), args.input);

  if Path::new(&args.output).exists() {
    error!("{} exists!", &args.output);
    std::process::exit(exitcode::SOFTWARE)
  }

  if Path::new(&args.input).exists() {
    if !util::is_vcf_file(&args.input) && !util::is_vcf_gz_file(&args.input) {
      error!("{} is not a valid vcf/vcf.gz file.", &args.input);
    }

    makedb(&args.input, &args.output).unwrap();
  } else {
    error!("{} - Not Found: {:?}", module_path!(), args.input);
    std::process::exit(exitcode::NOINPUT)
  }
}

pub fn makedb(input: &str, output: &str) -> Result<Vec<String>, vcf::VCFError> {
  // let mut conn = rusqlite::Connection::open_in_memory().unwrap();
  let mut conn = rusqlite::Connection::open(output).unwrap();

  if util::is_vcf_file(input) {
    let mut reader = convertor::get_reader(input).unwrap();
    let schema = convertor::infer_schema(&reader);
    convertor::create_table(&mut conn, &schema);
    convertor::insert_rows(&mut conn, &mut reader)
  } else {
    let mut reader = convertor::get_reader_gz(input).unwrap();
    let schema = convertor::infer_schema(&reader);
    convertor::create_table(&mut conn, &schema);
    convertor::insert_rows(&mut conn, &mut reader)
  }
}
