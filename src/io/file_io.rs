use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use serde_json::to_string;
use crate::machine::dfa::HBRecord;

pub struct FileWriter{
    pub(crate) file_path: String
}

impl FileWriter {
    fn open_file_append(file_path: &str) -> io::Result<File> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
    }

    pub fn write_records_to_file_append(&self, record: HBRecord) -> io::Result<()> {
        // Open the file in append mode
        let mut file = FileWriter::open_file_append( &self.file_path)?;
        let json_string = to_string(&record).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Serialization of HBRecord failed: {:?}", e),
            )
        })?;
        writeln!(file, "{}", json_string)?;
        Ok(())
    }
}

