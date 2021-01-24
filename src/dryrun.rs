use std::io::{self, BufWriter, Error};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::parser;

pub fn dry_run(path: &str) -> Result<(), Error> {
    let filenames = parser::parse_csv(path);
    let mut errors = false;

    println!("Checking files...");
    filenames.iter()
        .for_each(|(old, new)| {
            errors = display_result_dry(old, new);
        });

    if errors {
        display_errors();
    }

    Ok(())
}

fn display_result_dry(old: &PathBuf, new: &PathBuf) -> bool {
    let stdout = io::stdout();
    let mut buff = BufWriter::new(stdout);
    let mut errors = false;

    if old.is_file() && !new.is_file() {
        write!(buff, "[ OK ]\t").unwrap();
    } else if !old.is_file() && !new.is_file() {
        write!(buff, "\x1b[0;41m[Error1]\x1b[0m\t").unwrap();
        errors = true;
    } else if !old.is_file() && new.is_file() {
        write!(buff, "\x1b[0;41m[Error2]\x1b[0m\t").unwrap();
        errors = true;
    } else if !old.is_file() && new.is_file() {
        write!(buff, "\x1b[0;41m[Error3]\x1b[0m\t").unwrap();
        errors = true;
    } else {
        panic!("Unknown errors when displaying the dry run results.");
    }

    writeln!(buff, "{:?} \x1b[0;36m => \x1b[0m {:?}", old, new).unwrap();

    errors
}

fn display_errors() {
    let stdout = io::stdout();
    let mut buff = BufWriter::new(stdout);
    writeln!(buff, "\nFound errors:").unwrap();
    writeln!(buff, "Error 1: The original file is not found").unwrap();
    writeln!(buff, "Error 2: The original file is not found, a file exists for the proposed name.").unwrap();
    writeln!(buff, "Error 3: The original file is found, a file exists for the proposed name.").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dry_run_test() {
        let input = "test_files/input.csv";
        let res = dry_run(input);

        assert!(res.is_ok());
    }

}