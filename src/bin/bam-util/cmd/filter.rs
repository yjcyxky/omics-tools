use bam::{record::cigar::Operation, RecordWriter, BamWriter, BamReader};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{path::Path, io};

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
    );
}

pub fn run(matches: &ArgMatches) {
  let inputpath = matches.value_of("INPUT").unwrap();
  if Path::new(inputpath).exists() {
    filter(inputpath);
  } else {
    println!("Not Found: {:?}", inputpath);
  }
}

pub fn filter(inputpath: &str) {
  let reader = BamReader::from_path(inputpath, 10).unwrap();
  let header = reader.header().clone();
  let output = io::BufWriter::new(io::stdout());
  let mut writer = BamWriter::from_stream(output, header).unwrap();

  for record in reader {
    let record = record.unwrap();
    let length = record.query_len();
    let filtered: Vec<(u32, Operation)> = record.cigar().iter().filter(|cigar| cigar.1 == Operation::AlnMatch).collect();
    let mut filtered_item_len: u32 = 0;

    for (idx, _) in filtered {
      filtered_item_len += idx
    }

    if f64::from(filtered_item_len) / f64::from(length) > 0.5 && filtered_item_len > 100 {
      writer.write(&record).unwrap();
    }
  }

  writer.flush().unwrap();
}
