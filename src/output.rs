use std::path::PathBuf;
use std::fs::File;
use std::io::{LineWriter, Write, Result};

pub fn write_to_csv(recs: &mut [PathBuf], outdir: &str) -> Result<()> {
    let fname = format!("{}/fastq_records.csv", outdir);
    let csv = File::create(fname).unwrap();
    let mut line = LineWriter::new(csv);

    writeln!(line, "Path,Filenames,Names,Id").unwrap();

    recs.sort_by(|a, b| a.cmp(&b));

    recs.iter()
        .for_each(|r| {
            let path = r.parent().unwrap().to_string_lossy().into_owned();
            let file = r.file_name().unwrap().to_string_lossy().into_owned();
            let name = r.file_stem().unwrap().to_string_lossy().into_owned();
            let id = split_file_names(&name);
            writeln!(line, "{},{},{},{}", path, file, name.replace(".fastq", ""), id).unwrap();
        });
    
    Ok(())

}

fn split_file_names(fnames: &String) -> String {
    let word: Vec<&str> = fnames.split('_').collect();
    let id = format!("{}_{}_{}", word[0], word[1], word[2]);

    id
}