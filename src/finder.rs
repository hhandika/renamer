use std::path::PathBuf;

use walkdir::WalkDir;

use crate::writer;

pub fn process_input(path: &str, ext: &str) {
    let mut entries = traverse_dir(path, ext);
    println!("Found {} files", entries.len());
    writer::write_to_csv(&mut entries).unwrap();
}

fn traverse_dir(path: &str, ext: &str) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = Vec::new();

    WalkDir::new(path).into_iter()
        .filter_map(|recs| recs.ok())
        .for_each(|e| {
            let files = String::from(e.path().to_string_lossy());
            if ext == "fastq" {
                match_fastq(&files, &mut entries);
            } else {
                match_any(&files, ext, &mut entries);
            }
        });
    
    entries
}

fn match_fastq(files: &str, entries: &mut Vec<PathBuf>) {
    match files {
        s if s.ends_with(".fastq.gz") => entries.push(PathBuf::from(files)),
        s if s.ends_with(".fq.gz") => entries.push(PathBuf::from(files)),
        s if s.ends_with("fastq.gzip") => entries.push(PathBuf::from(files)),
        s if s.ends_with("fq.gzip") => entries.push(PathBuf::from(files)),
        _ => (),
    };
}

fn match_any(files: &str, ext: &str, entries: &mut Vec<PathBuf>) {
    if files.ends_with(ext) {
        entries.push(PathBuf::from(files))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn traverse_dir_fastq_test() {
        let path = "test_files/";
        let ext = "fastq";
        let res = traverse_dir(&path, &ext);

        assert_eq!(3, res.len());
    }

    #[test]
    fn traverse_dir_any_test() {
        let path = "test_files/";
        let ext = "csv";
        let res = traverse_dir(&path, &ext);

        assert_eq!(3, res.len());
    }
}