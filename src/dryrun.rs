use std::io::Error;

// use crate::checker;
use crate::parser;

pub fn dry_run(path: &str) -> Result<(), Error> {
    let dryrun = true;
    parser::parse_csv(path, dryrun);
    // let mut errors = 0;

    // println!("Checking files...");
    // filenames.iter()
    //     .for_each(|(old, new)| {
    //         checker::check_input_errors(old, new, &mut errors);
    //     });

    // if errors > 0 {
    //     checker::display_errors(&errors);
    // }

    Ok(())
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