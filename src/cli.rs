use clap::{App, AppSettings, Arg};

use crate::finder;
use crate::parser;
use crate::renamer;


pub fn get_cli(version: &str) {
    let args = App::new("renamer")
        .version(version)
        .about("Automates file renaming across directories")
        .author("Heru Handika <hhandi1@lsu.edu>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("find")
                .about("Finds relevant fastq files")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Inputs directory to traverse.")
                        .takes_value(true)
                        .value_name("DIR")
                )

                .arg(
                    Arg::with_name("wildcard")
                        .short("c")
                        .long("wildcard")
                        .help("Finds using wildcards.")
                        .conflicts_with_all(&[ "dir", "specify"])
                        .multiple(true)
                        .value_name("WILDCARD")
                )

                .arg(
                    Arg::with_name("specify")
                        .short("s")
                        .long("specify")
                        .help("Specifies file extension.")
                        .takes_value(true)
                        .default_value("fastq")
                        .value_name("EXTENSION")
                )

                .arg(
                    Arg::with_name("bpa")
                        .long("bpa")
                        .help("BPA database file format.")
                        .takes_value(false)
                )
        )

        .subcommand(
            App::new("rename")
            .about("Renames files given csv input.")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("Input file.")
                        .takes_value(true)
                        .value_name("INPUT_FILE")
                )

                .arg(
                    Arg::with_name("dry-run")
                        .long("dry")
                        .help("Dry run. Checks input first.")
                        .takes_value(false)
                )
        )
        .get_matches();

    match args.subcommand() {

        ("find", Some(find_matches)) => {
            if find_matches.is_present("dir") {
                let path = find_matches.value_of("dir").unwrap();
                let ext = find_matches.value_of("specify").unwrap();

                if find_matches.is_present("bpa") {
                    finder::process_input_dir(path, ext, true)
                } else {
                    finder::process_input_dir(path, ext, false);
                }      

            } else if find_matches.is_present("wildcard") {
                let entries: Vec<&str> = find_matches
                    .values_of("wildcard")
                    .unwrap()
                    .collect();

                if find_matches.is_present("bpa") {
                    finder::process_input_wcard(&entries, true);
                } else {
                    finder::process_input_wcard(&entries, false);
                }
                
            } else {
                println!("NO COMMANDS PROVIDED!");
            }
        }
        
        ("rename", Some(rename_matches)) => {
            if rename_matches.is_present("input") {
                let input = rename_matches.value_of("input").unwrap();

                if rename_matches.is_present("dry-run") {
                    let dryrun = true;
                    parser::parse_csv(input, dryrun);

                } else {
                    renamer::rename_files(input).unwrap();
                }
            }
        }
        
        _ => unreachable!("UNREACHABLE COMMANDS!"),
    };
}