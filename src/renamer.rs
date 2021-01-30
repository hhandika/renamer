use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Error, ErrorKind};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

use crate::parser;

pub fn rename_files(path: &str) -> Result<(), Error> {
    let dryrun = false;
    let filenames = parser::parse_csv(path, dryrun);

    // Keep track file renaming.
    // Opposite insertion. The new_names is the key.
    let mut temp: HashMap<PathBuf, PathBuf> = HashMap::new();
    let mut rename_count = 0;

    println!("Renaming files...");
    for (origin, destination) in filenames.iter() {
        let new_names = check_new_names(&destination);
        
        match fs::rename(origin, &new_names) {
            Ok(()) => {
                temp.insert(new_names.to_path_buf(), origin.to_path_buf());
                display_result(origin, &new_names);
                rename_count += 1;
            },

            Err(error) => match error.kind() {

                ErrorKind::PermissionDenied => { 
                    println!("Can't rename {:?}. It may be used by another program.", origin);
                    
                    let input =  get_user_input();
                    match input {

                        b'r' => { 
                            match fs::rename(origin, &new_names) {
                                Ok(()) => {
                                    temp.insert(new_names.to_path_buf(), origin.to_path_buf());
                                    display_result(origin, &new_names);
                                    rename_count += 1;
                                }
                                Err(_) => {
                                    println!("Still can't rename {:?}. Skipping it...", origin);
                                    continue;
                                }
                            }
                        }

                        b'c' => {
                            println!("Skipping {:?}", origin);
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
                    println!("{:?} \x1b[0;41mNOT FOUND!\x1b[0m", origin);
                    continue;
                }

                unknown_error => {
                    panic!("CAN'T RENAME THE FILE {:?}", unknown_error);
                }
            }
        }
    }
    println!("\nTotal files renamed: {}", rename_count);

    Ok(())
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

fn check_new_names(destination: &PathBuf) -> PathBuf {
    if destination.is_file() {
        create_duplicate_names(&destination)
    } else {
        PathBuf::from(destination)
    }
}

fn create_duplicate_names(fpath: &PathBuf) -> PathBuf {
    let stem = fpath.file_stem().unwrap().to_string_lossy();
    let ext = fpath.extension().unwrap().to_string_lossy();
    let mut new_names = format!("{}_renamerdup.{}", &stem, &ext);

    match &stem {
        s if s.ends_with(".fastq") => {
            let new_stem = stem.replace(".fastq", "_renamerdup.fastq");
            new_names = format!("{}.{}", &new_stem, &ext);
        }

        s if s.ends_with(".fq") => {
            let new_stem = stem.replace(".fq", "_renamerdup.fq");
            new_names = format!("{}.{}", &new_stem, &ext);
        }

        s if s.ends_with(".fasta") => {
            let new_stem = stem.replace(".fasta", "_renamerdup.fasta");
            new_names = format!("{}.{}", &new_stem, &ext);
        }

        _ => (),
    }

    let fnames = PathBuf::from(&new_names);
    let path = fpath.parent().unwrap();

    path.join(&fnames)
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
    #[should_panic]
    fn rename_file_panic_test() {
        let input = "test_files/invalid_input.csv";
        rename_files(&input).unwrap();
    }

    #[test]
    fn create_duplicat_name_test() {
        let fname = PathBuf::from("data/some_input.fastq.gz");
        let fname_fq = PathBuf::from("data/some_input.fq.gz");

        let res = PathBuf::from("data/some_input_renamerdup.fastq.gz");
        let res_fq = PathBuf::from("data/some_input_renamerdup.fq.gz");
        assert_eq!(res, create_duplicate_names(&fname));
        assert_eq!(res_fq, create_duplicate_names(&fname_fq));
    }

    #[test]
    fn check_duplicate_test() {
        let prop_name = PathBuf::from("test_files/valid.fastq.gz"); // exist in test_files dir
        let prop_fq = PathBuf::from("test_files/valid2.fq.gz"); // exist in test_files dir

        let new_names = PathBuf::from("test_files/valid_renamerdup.fastq.gz");
        let new_fq = PathBuf::from("test_files/valid2_renamerdup.fq.gz");

        assert_eq!(new_names, check_new_names(&prop_name));
        assert_eq!(new_fq, check_new_names(&prop_fq));
    }

}