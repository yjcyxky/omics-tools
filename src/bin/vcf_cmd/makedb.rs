// External
use exitcode;
use log::*;
use structopt::StructOpt;

// Standard
use std::path::Path;

// Custom
extern crate omics_tools;
use omics_tools::vcf::convertor;
use omics_tools::vcf::util;

/// Convert VCF file to a SQL Database File
#[derive(StructOpt, PartialEq, Debug)]
#[structopt(setting=structopt::clap::AppSettings::ColoredHelp, name="Omics Tool Suite - VCF Utility - makedb", author="Jingcheng Yang <yjcyxky@163.com>")]
pub struct Arguments {
  /// VCF file to process
  #[structopt(name = "FILE")]
  input: String,

  /// Output file.
  #[structopt(
    name = "output",
    short = "o",
    long = "output",
    default_value = "vcf.db"
  )]
  output: String,
}

pub fn run(args: &Arguments) {
  info!("{} - Make database: {:?}", module_path!(), args.input);

  if Path::new(&args.output).exists() {
    error!("{} exists!", &args.output);
    std::process::exit(exitcode::DATAERR)
  }

  if Path::new(&args.input).exists() {
    if !util::is_vcf_file(&args.input) && !util::is_vcf_gz_file(&args.input) {
      error!("{} is not a valid vcf/vcf.gz file.", &args.input);
      std::process::exit(exitcode::DATAERR)
    }

    convertor::makedb(&args.input, &args.output).unwrap();
  } else {
    error!("{} - Not Found: {:?}", module_path!(), args.input);
    std::process::exit(exitcode::NOINPUT)
  }
}
