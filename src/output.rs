use std::path::PathBuf;
use std::fs::File;
use std::io::{LineWriter, Write};

pub fn write_to_csv(recs: &mut [PathBuf]) {
    let fname = "data/fastq_records.csv";
    let csv = File::create(fname).unwrap();
    let mut line = LineWriter::new(csv);

    writeln!(line, "Path,Filenames").unwrap();

    recs.sort_by(|a, b| a.cmp(&b));

    recs.iter()
        .for_each(|r| {
            let path = r.parent().unwrap().to_string_lossy().into_owned();
            let file = r.file_name().unwrap().to_string_lossy().into_owned();
            writeln!(line, "{},{}", path, file).unwrap();
        })

}
