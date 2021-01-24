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

    // Keep track file renaming.
    // Opposite insertion. The new_names is the key.
    let mut temp: HashMap<PathBuf, PathBuf> = HashMap::new();

    println!("\nRenaming files...");
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
    let mut lcounts = 1; // Indexing line.
    println!("Checking csv input...");
    buff.lines()
        .filter_map(|ok| ok.ok())
        .enumerate()
        .for_each(|(i, recs)| {
            match i {
                0 => lcounts+=1, // Ignoring the header
                _ => { 
                    let files = split_csv_lines(&recs, &lcounts);
                    let old_names = PathBuf::from(files[0].to_string());
                    let new_names = PathBuf::from(files[1].to_string());
                    filenames.insert(old_names, new_names);
                    lcounts += 1;
                }
            }
            
        });

    filenames
}

fn split_csv_lines(lines: &str, lcounts: &u32) -> Vec<String> {
    let files: Vec<String> = lines.split(',')
        .map(|recs| recs.trim().to_string())
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

    if new_names.is_file() {
        new_names = create_duplicate_names(&new_names);
    }

    new_names
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
    fn parse_multicols_csv_test() {
        let input = "test_files/multicols_input.csv";
        let old = PathBuf::from("data/valid2.fastq.gz");
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
    fn create_duplicat_name_test() {
        let fname = PathBuf::from("data/some_input.fastq.gz");
        let fname_fq = PathBuf::from("data/some_input.fq.gz");

        let res = PathBuf::from("data/some_input_renamerdup.fastq.gz");
        let res_fq = PathBuf::from("data/some_input_renamerdup.fq.gz");
        assert_eq!(res, create_duplicate_names(&fname));
        assert_eq!(res_fq, create_duplicate_names(&fname_fq));
    }

}