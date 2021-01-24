use std::io::{self, BufWriter, Error};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::parser;

pub fn dry_run(path: &str) -> Result<(), Error> {
    let filenames = parser::parse_csv(path);


    filenames.iter()
        .for_each(|(old, new)| {
            display_result_dry(old, new);
        });
    Ok(())
}

fn display_result_dry(current: &PathBuf, new: &PathBuf) {
    let stdout = io::stdout();
    let mut buff = BufWriter::new(stdout);

    writeln!(buff, "{:?} \x1b[0;32m => \x1b[0m {:?}", current, new).unwrap();
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