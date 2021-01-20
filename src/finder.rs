
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

use crate::writer;

pub fn process_input(path: &str, outdir: &str) {
    let mut entries = traverse_dir(&path);

    writer::write_to_csv(&mut entries, outdir).unwrap();
}

fn traverse_dir(path: &str) -> Vec<PathBuf> {
    let files: Vec<DirEntry> = WalkDir::new(path).into_iter()
        .filter_map(|recs| recs.ok())
        .collect();
    
    let mut entries: Vec<PathBuf> = Vec::new();

    files.into_iter()
        .for_each(|e| {
            let files = String::from(e.path().to_string_lossy());
            match &files {
                s if s.ends_with(".fastq.gz") => entries.push(PathBuf::from(files)),
                s if s.ends_with(".fq.gz") => entries.push(PathBuf::from(files)),
                s if s.ends_with("fastq.gzip") => entries.push(PathBuf::from(files)),
                s if s.ends_with("fq.gzip") => entries.push(PathBuf::from(files)),
                _ => (),
            };
        });
    
    entries
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn traverse_dir_test() {
        let path = "test_files/";
        let res = traverse_dir(&path);

        assert_eq!(3, res.len());
    }
}