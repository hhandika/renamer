use std::fs::{self, File};
use std::io::{prelude::*, BufReader, Error};
// use std::path::PathBuf;
use std::collections::HashMap;

pub fn rename_files(path: &str) -> Result<(), Error> {
    let file = File::open(path).unwrap();
    let buff = BufReader::new(&file);

    let mut filenames = HashMap::new();

    buff.lines()
        .filter_map(|ok| ok.ok())
        .enumerate()
        .for_each(|(i, recs)| {
            match i {
                0 => (),
                _ => { 
                    let files = split_csv_lines(&recs);
                    filenames.insert(files[0].to_string(), files[1].to_string());
                }
            }
            
        });
    
    for (old_names, new_names) in filenames.iter() {
        fs::rename(old_names, new_names).unwrap();
        println!("{} => {}", old_names, new_names);
    }

    Ok(())
}

fn split_csv_lines(lines: &String) -> Vec<String> {
    let files: Vec<String> = lines.split(',')
        .map(|recs| recs.trim().to_string())
        .collect();

    if files.len() != 2 {
        panic!("INVALID CSV INPUT! ONLY TWO COLUMNS ALLOWED.")
    }

    files
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_csv_lines_test() {
        let lines = String::from("./test/old_names.fastq.gz,./test/new_names.fastq.gz");
        let res = vec!["./test/old_names.fastq.gz","./test/new_names.fastq.gz"];

        assert_eq!(res, split_csv_lines(&lines));
    }

    #[test]
    #[should_panic]
    fn split_csv_panic_test() {
        let lines = String::from("./test/old_names.fastq.gz,\
            ./test/new_names.fastq.gz,\
            ./test/other_names.fastq.gz");
        split_csv_lines(&lines);
    }
}