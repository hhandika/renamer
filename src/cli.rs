use clap::{App, AppSettings, Arg};

use crate::input;

pub fn get_cli(version: &str) {
    let args = App::new("renamer")
        .version(version)
        .about("Find raq fastq and rename them")
        .author("Heru Handika <hhandi1@lsu.edu")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("find")
                .about("Find relevant fastq files")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Input directory to traverse.")
                        .takes_value(true)
                        .value_name("DIR")
                )
            )
        .get_matches();
    
    println!("Looking for files...");

    match args.subcommand() {
        ("find", Some(dir_matches)) => {
            if dir_matches.is_present("dir") {
                let path = dir_matches.value_of("dir").unwrap();
                input::process_input(&path);
            } else {
                println!("NO COMMANDS PROVIDED!");
            }
        }
        _ => unreachable!("UNREACHABLE COMMANDS!"),
    };
}