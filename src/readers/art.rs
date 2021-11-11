use crate::parsers::art::{parse_art_entry, ItemArtData};
use crate::readers::index::IndexFile;
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct ItemArt {
    index: IndexFile,
    reader: BufReader<File>,
}

impl ItemArt {
    pub fn new() -> Self {
        let file_handle = File::open("/Users/tsul/Outlands/art.mul").unwrap();

        ItemArt {
            index: IndexFile::new("/Users/tsul/Outlands/artidx.mul"),
            reader: BufReader::new(file_handle),
        }
    }

    pub fn read_entry(&mut self, id: u64) -> Result<ItemArtData, Error> {
        let index_entry = self.index.read_entry(id).unwrap();
        let mut buffer = vec![0; index_entry.size];

        self.reader
            .seek(SeekFrom::Start(index_entry.lookup as u64))?;
        self.reader.read(&mut buffer)?;

        let (_, art_data) = parse_art_entry(&buffer, index_entry.size).unwrap();

        Ok(art_data)
    }
}
