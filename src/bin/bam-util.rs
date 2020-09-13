extern crate clap;
use clap::{App, AppSettings};

mod cmd;
use cmd::filter;

fn main() {
    let app = App::new("Omics Tool Suite - Bam Utility")
        .version("1.0")
        .setting(AppSettings::GlobalVersion)
        .author("Jingcheng Yang <yjcyxky@163.com>")
        .about("A suite of programs for interacting with bam file.");

    // You can add more subcommands on it.
    let subcommand = app.subcommand(filter::subcommand());

    let matches = subcommand.get_matches();

    if let Some(matches) = matches.subcommand_matches(filter::COMMAND_NAME) {
        filter::run(matches);
    }
}
