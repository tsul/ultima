use crate::parsers::index::{parse_index_entry, IndexEntry};
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct IndexFile {
    pub filename: String,
    reader: BufReader<File>,
}

impl IndexFile {
    pub fn new(filename: &str) -> Self {
        let file_handle = File::open(filename).unwrap();

        IndexFile {
            filename: String::from(filename),
            reader: BufReader::with_capacity(12, file_handle),
        }
    }

    pub fn read_entry(&mut self, id: u64) -> Result<IndexEntry, Error> {
        let mut buffer = [0; 12];

        self.reader.seek(SeekFrom::Start(id * 12))?;
        self.reader.read(&mut buffer)?;

        let (_, entry) = parse_index_entry(&buffer).unwrap();

        Ok(entry)
    }
}
