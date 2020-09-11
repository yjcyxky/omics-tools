extern crate clap;
use clap::{App, AppSettings};

mod cmd;

fn main() {
    let app = App::new("samtools-rs")
        .version("1.0")
        .setting(AppSettings::GlobalVersion)
        .author("Jingcheng Yang <yjcyxky@163.com>")
        .about("A suite of programs for interacting with high-throughput sequencing data.");

    let matches = app.subcommand(cmd::filter::subcommand()).get_matches();

    if let Some(matches) = matches.subcommand_matches(cmd::filter::COMMAND_NAME) {
        cmd::filter::run(matches);
    }
}
