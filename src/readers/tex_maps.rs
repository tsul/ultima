use crate::parsers::tex_maps::{parse_tex_map, TexMapData};
use crate::readers::index::IndexFile;
use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct TexMaps {
    index: IndexFile,
    reader: BufReader<File>,
}

impl TexMaps {
    pub fn new() -> Self {
        let file_handle = File::open("/Users/tsul/Outlands/texmaps.mul").unwrap();

        TexMaps {
            index: IndexFile::new("/Users/tsul/Outlands/texidx.mul"),
            reader: BufReader::new(file_handle),
        }
    }

    pub fn read_entry(&mut self, id: u64) -> Result<TexMapData, Error> {
        let index_entry = self.index.read_entry(id).unwrap();
        let mut buffer = vec![0; index_entry.size];

        self.reader
            .seek(SeekFrom::Start(index_entry.lookup as u64))?;
        self.reader.read(&mut buffer)?;

        let (_, tex_map) = parse_tex_map(&buffer, index_entry.size).unwrap();

        Ok(tex_map)
    }
}
