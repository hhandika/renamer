use clap::{App, AppSettings, Arg};

use crate::finder;
use crate::renamer;

pub fn get_cli(version: &str) {
    let args = App::new("renamer")
        .version(version)
        .about("Automate file renaming across directories")
        .author("Heru Handika <hhandi1@lsu.edu>")
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

                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("Output directory to save results")
                        .takes_value(true)
                        .default_value(".")
                        .value_name("OUTPUT-DIR")
                )
        )
        .subcommand(
            App::new("rename")
            .about("Rename files given csv input.")
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
                let outdir = find_matches.value_of("output").unwrap();

                finder::process_input(&path, &outdir);
            } else {
                println!("NO COMMANDS PROVIDED!");
            }
        }
        
        ("rename", Some(rename_matches)) => {
            if rename_matches.is_present("input") {
                let input = rename_matches.value_of("input").unwrap();

                if rename_matches.is_present("dry-run") {
                    renamer::dry_run(&input).unwrap();
                } else {
                    renamer::rename_files(&input).unwrap();
                }
            }
        }
        _ => unreachable!("UNREACHABLE COMMANDS!"),
    };
}