use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

fn parse_csv(path: &str) -> HashMap<PathBuf, PathBuf> {
    let file = File::open(path).unwrap();
    let buff = BufReader::new(&file);

    let mut filenames: HashMap<PathBuf, PathBuf> = HashMap::new();

    buff.lines()
        .filter_map(|ok| ok.ok())
        .enumerate()
        .for_each(|(i, recs)| {
            match i {
                0 => (),
                _ => { 
                    let files = split_csv_lines(&recs);
                    let old_names = PathBuf::from(files[0].to_string());
                    let new_names = PathBuf::from(files[1].to_string());
                    filenames.insert(old_names, new_names);
                }
            }
            
        });

    filenames
}

pub fn rename_files(path: &str) -> Result<(), Error> {
    let filenames = parse_csv(path);

    let mut temp: HashMap<PathBuf, PathBuf> = HashMap::new();

    for (old_names, new_names) in filenames.iter() {
        match fs::rename(old_names, new_names) {
            Ok(file) => file,
            Err(error) => match error.kind() {

                ErrorKind::PermissionDenied => { 
                    let input =  get_user_input();
                    match input {
                        b'r' => fs::rename(old_names, new_names).unwrap(),
                        b'c' => continue,
                        b'a' => {
                            roll_back_renaming(&temp);
                            process::abort();
                        }
                        _ => ()
                    }
                }

                ErrorKind::NotFound => {
                    println!("{:?} \x1b[0;41mNOT FOUND!\x1b[0m", old_names);
                    continue;
                }

                unknown_error => {
                    panic!("CAN'T RENAME THE FILE {:?}", unknown_error);
                }
            }
        }
        temp.insert(new_names.to_path_buf(), old_names.to_path_buf());
        println!("{:?} \x1b[0;36m => \x1b[0m {:?}", old_names, new_names);
    }

    Ok(())
}

fn roll_back_renaming(filenames: &HashMap<PathBuf, PathBuf>) {
    println!("Rolling back!");
    filenames.iter()
        .for_each(|(new, old)| {
            fs::rename(new, old).unwrap();
            println!("{:?} \x1b[0;36m => \x1b[0m {:?}", new, old);
        });
}

fn get_user_input() -> u8 {
    println!("What would you like to do: [r]retry, [c]continue, [a]bort? ");

    let input = io::stdin()
        .bytes()
        .next()
        .and_then(|ok| ok.ok())
        .map(|byte| byte as u8);
    
    input.unwrap()
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