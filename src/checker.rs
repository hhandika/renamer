use std::io::{self, BufWriter};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn check_input_errors(old: &PathBuf, new: &PathBuf, errors: &mut u32) {
    let stdout = io::stdout();
    let mut buff = BufWriter::new(stdout);

    if old.is_file() && !new.is_file() {
        write!(buff, "[ OK ]\t\t").unwrap();
    } else if !old.is_file() && !new.is_file() {
        write!(buff, "\x1b[0;41m[Error 1]\x1b[0m\t").unwrap();
        *errors += 1;
    } else if !old.is_file() && new.is_file() {
        write!(buff, "\x1b[0;41m[Error 2]\x1b[0m\t").unwrap();
        *errors += 1;
    } else {
        write!(buff, "\x1b[0;41m[Error 3]\x1b[0m\t").unwrap();
        *errors += 1;
    } 

    writeln!(buff, "{:?} \x1b[0;36m => \x1b[0m {:?}", old, new).unwrap();
}

pub fn display_errors(counts: &u32) {
    let stdout = io::stdout();
    let mut buff = BufWriter::new(stdout);
    writeln!(buff, "Errors found: {}", counts).unwrap();
    writeln!(buff, "\nError Kinds:").unwrap();
    writeln!(buff, "Error 1: The original file is not found").unwrap();
    writeln!(buff, "Error 2: The original file is not found, a file exists for the proposed name.").unwrap();
    writeln!(buff, "Error 3: The original file is found, a file exists for the proposed name.").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input_errors_test() {
        let old = PathBuf::from("test_files/valid.fastq.gz");
        let new = PathBuf::from("test_files/valid_new.fastq.gz");
        let mut errors = 0;
        check_input_errors(&old, &new, &mut errors);

        assert_eq!(0, errors);
    }

}