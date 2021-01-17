use std::path::PathBuf;
use std::fs::File;
use std::io::{LineWriter, Write, Result};

pub fn write_to_csv(recs: &mut [PathBuf], outdir: &str) -> Result<()> {
    let fname = format!("{}/fastq_records.csv", outdir);
    let csv = File::create(&fname).unwrap();
    let mut line = LineWriter::new(csv);

    writeln!(line, "full_path,filenames,id,read_id").unwrap();

    recs.sort_by(|a, b| a.cmp(&b));

    recs.iter()
        .for_each(|r| {
            let full_path = r.to_string_lossy().into_owned();
            let file = r.file_name().unwrap().to_string_lossy().into_owned();
            let id = Id::split_file_names(&file);
            writeln!(line, "{},{},{},{}", full_path, file, id.file_id, id.read_id).unwrap();
        });
        
    println!("Done! The result is saved as {}", &fname);
    Ok(())
}

struct Id {
    file_id: String,
    read_id: String,
}

impl Id {
    fn new() -> Self {
        Self {
            file_id: String::from("N/A"), 
            read_id: String::from("N/A")
        }
    }

    fn split_file_names(fnames: &String) -> Self {
        let word: Vec<&str> = fnames.split('_').collect();
        let mut id = Self::new();

        if word.len() == 7 {
            id.file_id = format!("{}_{}_{}", word[0], word[1], word[2]);
            id.read_id = format!("{}_{}_{}_{}", word[3], word[4], word[5], word[6]);
        }

        id
    }
}
