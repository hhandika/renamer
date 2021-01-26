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

    println!("Renaming files...");
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
    let filenames = prop_names.file_name().unwrap();
    let mut new_names = parent_path.join(filenames);
    
    match_extension(old_names, &mut new_names)
        .expect("Can't match file extension");

    if new_names.is_file() {
        new_names = create_duplicate_names(&new_names);
    }

    new_names
}

fn match_extension(old_name: &PathBuf, new_names: &mut PathBuf) -> Result<(), Error>{
    let ext = old_name.extension().unwrap();
    if ext != new_names.extension().unwrap() {
        new_names.set_extension(ext);
    }

    Ok(())
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
    fn construct_path_test() {
        let old_name = PathBuf::from("data/old.fq.gz");
        let prop_name = PathBuf::from("new.fq.gz");
        let prop_path = PathBuf::from("data/new.fq.gz");

        let new_names = PathBuf::from("data/new.fq.gz");

        assert_eq!(new_names, construct_path_new_names(&old_name, &prop_name));
        assert_eq!(new_names, construct_path_new_names(&old_name, &prop_path));
    }

    #[test]
    fn construct_path_ext_match_test() {
        let old_name = PathBuf::from("data/old.fq.gzip");
        let prop_name = PathBuf::from("new.fq.gz");

        let new_names = PathBuf::from("data/new.fq.gzip");

        assert_eq!(new_names, construct_path_new_names(&old_name, &prop_name));
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

    #[test]
    fn construct_path_dup_test() {
        let old_name = PathBuf::from("test_files/old.fq.gz"); // arbitrary file
        let prop_name = PathBuf::from("valid.fastq.gz"); // exist in test_files dir
        let prop_fq = PathBuf::from("valid2.fq.gz"); // exist in test_files dir

        let new_names = PathBuf::from("test_files/valid_renamerdup.fastq.gz");
        let new_fq = PathBuf::from("test_files/valid2_renamerdup.fq.gz");

        assert_eq!(new_names, construct_path_new_names(&old_name, &prop_name));
        assert_eq!(new_fq, construct_path_new_names(&old_name, &prop_fq));
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

}