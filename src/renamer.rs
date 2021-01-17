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
                    
                    let files: Vec<String> = recs.split(',')
                        .map(|recs| recs.trim().to_string())
                        .collect();
                        
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

// fn split_csv_lines(lines: &String) -> Vec<String> {
//     let lines: Vec<String> = 
//     ) 
    
//     lines
// }