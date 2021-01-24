use std::path::PathBuf;
use std::fs::File;
use std::io::{LineWriter, Write, Result};

pub fn write_to_csv(recs: &mut [PathBuf]) -> Result<()> {
    let fname = format!("fastq_records.csv");
    let csv = File::create(&fname).unwrap();
    let mut line = LineWriter::new(csv);

    writeln!(line, "full_path,parent,filenames,id,read_id").unwrap();

    recs.sort_by(|a, b| a.cmp(&b));

    recs.iter()
        .for_each(|r| {
            let mut id = Id::new(&r);
            id.split_file_names();

            writeln!(line, "{},{},{},{},{}", 
                id.full_path, 
                id.parent, 
                id.fname, 
                id.file_id, 
                id.read_id
            ).unwrap();
        });

    println!("Done! The result is saved as {}", &fname);
    Ok(())
}

struct Id {
    full_path: String,
    parent: String,
    fname: String,
    file_id: String,
    read_id: String,
}

impl Id {
    fn new(lines: &PathBuf) -> Self {
        Self {
            full_path: lines.to_string_lossy().into_owned(),
            parent: lines.parent().unwrap().to_string_lossy().into_owned(),
            fname: lines.file_name().unwrap().to_string_lossy().into_owned(),
            file_id: String::from("N/A"), 
            read_id: String::from("N/A")
        }
    }

    fn split_file_names(&mut self) {
        let word: Vec<&str> = self.fname.split('_').collect();

        if word.len() == 7 {
            self.file_id = format!("{}_{}_{}", word[0], word[1], word[2]);
            self.read_id = format!("{}_{}_{}_{}", word[3], word[4], word[5], word[6]);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Test for non-OMG database filename format.
    fn path_id_test() {
        let lines = PathBuf::from("data/HDWND_AAGT_A1_L1_R2_01.fastq.gz");
        let mut id = Id::new(&lines);
        id.split_file_names();

        assert_eq!("N/A", id.file_id);
        assert_eq!("N/A", id.read_id);
    }

    #[test]
    // Test for OMG database filenames. 
    fn path_modify_id_test() {
        let lines = PathBuf::from("data/26535_HDWND_AAGT_A1_L1_R2_01.fastq.gz");
        let mut id = Id::new(&lines);
        id.split_file_names();

        assert_eq!("data/26535_HDWND_AAGT_A1_L1_R2_01.fastq.gz", id.full_path);
        assert_eq!("data", id.parent);
        assert_eq!("26535_HDWND_AAGT_A1_L1_R2_01.fastq.gz", id.fname);
        assert_eq!("26535_HDWND_AAGT", id.file_id);
        assert_eq!("A1_L1_R2_01.fastq.gz", id.read_id);
    }
}