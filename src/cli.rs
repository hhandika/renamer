use clap::{App, AppSettings, Arg};

use crate::finder;
use crate::dryrun;
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
                        .help("Input directory to traverse.")
                        .takes_value(true)
                        .value_name("DIR")
                )

                .arg(
                    Arg::with_name("specify")
                        .short("s")
                        .long("specify")
                        .help("Specify file extension.")
                        .takes_value(true)
                        .default_value("fastq")
                        .value_name("EXTENSION")
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
                        .help("Dry run. Print the expected results.")
                        .takes_value(false)
                )
        )
        .get_matches();

    match args.subcommand() {

        ("find", Some(find_matches)) => {
            if find_matches.is_present("dir") {
                let path = find_matches.value_of("dir").unwrap();
                let ext = find_matches.value_of("specify").unwrap();

                finder::process_input(path, ext);


            } else {
                println!("NO COMMANDS PROVIDED!");
            }
        }
        
        ("rename", Some(rename_matches)) => {
            if rename_matches.is_present("input") {
                let input = rename_matches.value_of("input").unwrap();

                if rename_matches.is_present("dry-run") {
                    dryrun::dry_run(input).unwrap();
                } else {
                    renamer::rename_files(input).unwrap();
                }
            }
        }
        
        _ => unreachable!("UNREACHABLE COMMANDS!"),
    };
}