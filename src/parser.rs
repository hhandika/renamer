use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::checker;
use std::process;

pub fn parse_csv(path: &str) -> HashMap<PathBuf, PathBuf> {
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
                    let new_names = PathBuf::from(&files[1]);
                    checker::check_input_errors(&old_names, &new_names, &mut errors);
                    filenames.insert(old_names, new_names);
                    lcounts += 1;
                }
            }
            
        });

    if errors > 0 {check_input(&errors);}

    println!("\nFound {} entries", lcounts - 2); // exclude header.

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
        println!("LINE {} HAS MORE THAN TWO COLUMNS.\
            ASSUMING THE FIRST TWO ARE THE FILENAMES.", lcounts);
    }
    
    files   
}

fn check_input(errors: &u32) {
    checker::display_errors(errors);
    if !get_user_input_err() {
        process::abort();
    }
}

fn get_user_input_err() -> bool {
    println!("Would you like to continue: [y]es/[n]o? ");

    let mut input = false;
    loop {
        let key = io::stdin()
            .bytes()
            .next()
            .and_then(|ok| ok.ok())
            .map(|k| k as u8)
            .unwrap();
        
        match key {
            b'y' => { 
                input = true; 
                break;
            }

            b'n' => break,

            _ => println!("Incorrect input! Please, try again...")
        };
    }

    input
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

        let filenames = parse_csv(&input);

        for (old_names, new_names) in filenames {
            assert_eq!(old, old_names);
            assert_eq!(new, new_names);
        }
    }

    #[test]
    fn parse_multicols_csv_test() {
        let input = "test_files/multicols_input.csv";
        let old = PathBuf::from("test_files/valid2.fastq.gzip");
        let new = PathBuf::from("valid_new2.fastq.gz");

        let filenames = parse_csv(&input);

        for (old_names, new_names) in filenames {
            assert_eq!(old, old_names);
            assert_eq!(new, new_names);
        }
    }

    #[test]
    #[should_panic]
    fn parse_csv_panic_test() {
        let input = "test_files/invalid_input.csv";
        parse_csv(&input);
    }

    #[test]
    #[should_panic(expected="INVALID CSV INPUT! ONLY ONE COLUMN FOUND IN LINE 3.")]
    fn parse_csv_panic_message_test() {
        let input = "test_files/invalid_input.csv";
        parse_csv(&input);
    }


}