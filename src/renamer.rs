use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

pub fn dry_run(path: &str) -> Result<(), Error> {
    let filenames = parse_csv(path);

    filenames.iter()
        .for_each(|(old, new)| {
            display_result(old, new);
        });
    
    Ok(())
}

pub fn rename_files(path: &str) -> Result<(), Error> {
    let filenames = parse_csv(path);

    let mut temp: HashMap<PathBuf, PathBuf> = HashMap::new();

    for (old_names, prop_names) in filenames.iter() {
        let new_names = construct_path_new_names(old_names, prop_names);

        match fs::rename(old_names, &new_names) {
            Ok(()) => {
                temp.insert(new_names.to_path_buf(), old_names.to_path_buf());
                display_result(old_names, &new_names);
            },

            Err(error) => match error.kind() {

                ErrorKind::PermissionDenied => { 
                    println!("Can't rename {:?}. It may be used by another program.", old_names);
                    
                    let input =  get_user_input();
                    match input {

                        b'r' => { 
                            match fs::rename(old_names, &new_names) {
                                Ok(()) => {
                                    temp.insert(new_names.to_path_buf(), old_names.to_path_buf());
                                    display_result(old_names, &new_names);
                                }
                                Err(_) => {
                                    println!("Still can't rename {:?}. Skipping it...", old_names);
                                    continue;
                                }
                            }
                        }

                        b'c' => {
                            println!("Skipping {:?}", old_names);
                            continue;
                        },

                        b'a' => {
                            roll_back_renaming(&temp);
                            process::abort();
                        }
                        _ => panic!("UNKNOWN ERRORS COMING FROM USER INPUTS!")
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
    }

    Ok(())
}

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

fn split_csv_lines(lines: &String) -> Vec<String> {
    let files: Vec<String> = lines.split(',')
        .map(|recs| recs.trim().to_string())
        .collect();

    if files.len() != 2 {
        panic!("INVALID CSV INPUT! ONLY TWO COLUMNS ALLOWED.")
    }

    files
}

fn get_user_input() -> u8 {
    println!("What would you like to do: [r]etry/[c]ontinue/[a]bort? ");

    let mut input;
    loop {
        input = io::stdin()
            .bytes()
            .next()
            .and_then(|ok| ok.ok())
            .map(|key| key as u8)
            .unwrap();
        
        match input {
            b'c' | b'r' | b'a' => break,
            _ => println!("Incorrect input! Please, try again...")
        };
    }

    input
}

fn construct_path_new_names(
        old_names: &PathBuf, 
        prop_names: &PathBuf
    ) -> PathBuf {

    let parent_path = old_names.parent().unwrap();
    let new_names = prop_names.file_name().unwrap();

    parent_path.join(new_names)
}

fn roll_back_renaming(filenames: &HashMap<PathBuf, PathBuf>) {
    println!("Rolling back!");
    filenames.iter()
        .for_each(|(new, old)| {
            fs::rename(new, old).unwrap();
            display_result(new, old);
        });
}

// I call it current and new for the function arguments
// Because this function is used for rolling back as well.
fn display_result(current: &PathBuf, new: &PathBuf) {
    let stdout = io::stdout();
    let mut buff = io::BufWriter::new(stdout);

    writeln!(buff, "{:?} \x1b[0;36m => \x1b[0m {:?}", current, new).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_csv_lines_test() {
        let lines = String::from("./test/old_names.fastq.gz,./test/new_names.fastq.gz");
        let res = split_csv_lines(&lines);

        assert_eq!(2, res.len());
    }

    #[test]
    #[should_panic]
    fn split_csv_panic_test() {
        let lines = String::from("./test/old_names.fastq.gz,\
            ./test/new_names.fastq.gz,\
            ./test/other_names.fastq.gz");
        split_csv_lines(&lines);
    }

    #[test]
    fn dry_run_test() {
        let input = "test_files/input.csv";
        let res = dry_run(input);

        assert!(res.is_ok());
    }

    #[test]
    fn parse_csv_test() {
        let input = "test_files/input.csv";
        let old = PathBuf::from("data/valid.fastq.gz");
        let new = PathBuf::from("data/valid_new.fastq.gz");

        let filenames = parse_csv(&input);

        for (old_names, new_names) in filenames {
            assert_eq!(old, old_names);
            assert_eq!(new, new_names);
        }
    }

    #[test]
    fn construct_path_test() {
        let old_name = PathBuf::from("data/old.fq.gz");
        let prop_name = PathBuf::from("new.fq.gz");
        let prop_path = PathBuf::from("data/new.fq.gz");

        let new_names = PathBuf::from("data/new.fq.gz");

        assert_eq!(new_names, construct_path_new_names(&old_name, &prop_name));
        assert_eq!(new_names, construct_path_new_names(&old_name, &prop_path));
    }
}