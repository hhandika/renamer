use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Error};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::checker;
use std::process;

pub fn parse_csv(path: &str, dryrun: bool) -> HashMap<PathBuf, PathBuf> {
    let file = File::open(path).unwrap();
    let buff = BufReader::new(&file);

    let mut filenames: HashMap<PathBuf, PathBuf> = HashMap::new();
    let mut lcounts = 1; // Indexing line.
    let mut errors = 0;
    println!("Checking csv input...");
    buff.lines()
        .filter_map(|ok| ok.ok())
        .enumerate()
        .for_each(|(i, recs)| {
            match i {
                0 => lcounts+=1, // Ignoring the header
                _ => { 
                    let files = split_csv_lines(&recs, &lcounts);
                    let old_names = PathBuf::from(&files[0]);
                    let mut new_names = PathBuf::from(&files[1]);
                    new_names = construct_new_names(&old_names, &new_names);
                    checker::check_input_errors(&old_names, &new_names, &mut errors);
                    filenames.insert(old_names, new_names);
                    lcounts += 1;
                }
            }
            
        });

    println!("\nEntries found: {}", lcounts - 2); // exclude header.
    check_input(&errors, dryrun);

    filenames
}

fn split_csv_lines(lines: &str, lcounts: &u32) -> Vec<String> {
    let files: Vec<String> = lines.split(',')
        .map(|e| e.trim().to_string())
        .collect();

    let cols = files.len();

    if cols < 2 || files[1].is_empty() {
        panic!("INVALID CSV INPUT! ONLY ONE COLUMN FOUND IN LINE {}.", lcounts);
    } 
    
    if cols > 2 {
        println!("\x1b[0;33mLINE {} HAS MORE THAN TWO COLUMNS.\
            ASSUMING THE FIRST TWO ARE THE FILENAMES.\x1b[0m", lcounts);
    }
    
    files   
}

fn construct_new_names(old_names: &PathBuf, prop_names: &PathBuf) -> PathBuf {

    let parent_path = old_names.parent().unwrap();
    let filenames = prop_names.file_name().unwrap();
    let mut new_names = parent_path.join(filenames);

    match_extension(old_names, &mut new_names)
        .expect("Can't match file extension");

    new_names
}

fn match_extension(old_name: &PathBuf, new_names: &mut PathBuf) -> Result<(), Error>{
    let ext = old_name.extension().unwrap();
    if ext != new_names.extension().unwrap() {
        new_names.set_extension(ext);
    }

    Ok(())
}

fn check_input(errors: &u32, dryrun: bool) {
    if *errors > 0 {
        checker::display_errors(errors);
        if !dryrun {
            get_user_input_err().unwrap();
        }
    }
}

fn get_user_input_err() -> Result<(), Error> {
    println!("\nWould you like to continue: [y]es/[n]o? ");
    loop {
        let input = io::stdin()
            .bytes()
            .next()
            .and_then(|ok| ok.ok())
            .map(|k| k as u8)
            .unwrap();
        
        match input {
            b'y' => break,
            b'n' => process::exit(1),
            _ => println!("Incorrect input! Please, try again...")
        };
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_csv_lines_test() {
        let lines = String::from("./test/old_names.fastq.gz,./test/new_names.fastq.gz");
        let lcounts = 1;
        let res = split_csv_lines(&lines, &lcounts);

        assert_eq!(2, res.len());
    }

    #[test]
    fn multicols_csv_split_test() {
        let lines = String::from("./test/old_names.fastq.gz,\
            ./test/new_names.fastq.gz,\
            ./test/new_names.fastq.gz");
        let lcounts = 1;
        let res = split_csv_lines(&lines, &lcounts);
        
        assert_eq!(3, res.len());
    }

    #[test]
    #[should_panic]
    fn split_csv_empty_col_panic_test() {
        let empty_cols = String::from("./test/old_names.fastq.gz,");
        let lcols = 1;
        split_csv_lines(&empty_cols, &lcols);
    }

    #[test]
    #[should_panic]
    fn split_csv_one_cols_panic_test() {
        let one_col = String::from("./test/old_names.fastq.gz");
        let lcols = 1;
        split_csv_lines(&one_col, &lcols);
    }

    #[test]
    fn parse_csv_test() {
        let input = "test_files/input.csv";
        let old = PathBuf::from("test_files/valid.fastq.gz");
        let new = PathBuf::from("test_files/valid_new.fastq.gz");

        let filenames = parse_csv(&input, true);

        for (old_names, new_names) in filenames {
            assert_eq!(old, old_names);
            assert_eq!(new, new_names);
        }
    }

    #[test]
    fn parse_multicols_csv_test() {
        let input = "test_files/multicols_input.csv";
        let old = PathBuf::from("test_files/valid2.fastq.gzip");
        let new = PathBuf::from("test_files/valid_new2.fastq.gzip");

        let filenames = parse_csv(&input, true);

        for (old_names, new_names) in filenames {
            assert_eq!(old, old_names);
            assert_eq!(new, new_names);
        }
    }

    #[test]
    #[should_panic]
    fn parse_csv_panic_test() {
        let input = "test_files/invalid_input.csv";
        parse_csv(&input, true);
    }

    #[test]
    #[should_panic(expected="INVALID CSV INPUT! ONLY ONE COLUMN FOUND IN LINE 3.")]
    fn parse_csv_panic_message_test() {
        let input = "test_files/invalid_input.csv";
        parse_csv(&input, true);
    }

    #[test]
    fn construct_path_test() {
        let old_name = PathBuf::from("data/old.fq.gz");
        let prop_name = PathBuf::from("new.fq.gz");
        let prop_path = PathBuf::from("data/new.fq.gz");

        let new_names = PathBuf::from("data/new.fq.gz");

        assert_eq!(new_names, construct_new_names(&old_name, &prop_name));
        assert_eq!(new_names, construct_new_names(&old_name, &prop_path));
    }

    #[test]
    fn construct_path_ext_match_test() {
        let old_name = PathBuf::from("data/old.fq.gzip");
        let prop_name = PathBuf::from("new.fq.gz");

        let new_names = PathBuf::from("data/new.fq.gzip");

        assert_eq!(new_names, construct_new_names(&old_name, &prop_name));
    }

    #[test]
    fn match_extension_test() {
        let old_name = PathBuf::from("data/old.fq.gzip");
        let mut new_names = PathBuf::from("data/new.fq.gz");

        let res = PathBuf::from("data/new.fq.gzip");
        match_extension(&old_name, &mut new_names).unwrap();
        
        assert_eq!(res,new_names);
    }

    #[test]
    fn match_extension_ok_test() {
        let old_name = PathBuf::from("data/old.fq.gzip");
        let mut new_names = PathBuf::from("data/new.fq.gzip");
        let res = match_extension(&old_name, &mut new_names);
        
        assert!(res.is_ok());
    }

}