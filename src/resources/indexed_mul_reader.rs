use crate::resources::{Asset, MulLookup, ProvideFromDisk};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct IndexedMulReader<T, L> {
    idx: BufReader<File>,
    mul: BufReader<File>,
    output_type: PhantomData<T>,
    lookup_type: PhantomData<L>,
}

impl<T: Asset, L: MulLookup> IndexedMulReader<T, L> {
    pub fn new(idx_filename: &str, mul_filename: &str) -> IndexedMulReader<T, L> {
        let idx_handle = File::open(idx_filename).unwrap();
        let mul_handle = File::open(mul_filename).unwrap();

        IndexedMulReader {
            idx: BufReader::new(idx_handle),
            mul: BufReader::new(mul_handle),
            output_type: PhantomData,
            lookup_type: PhantomData,
        }
    }
}

impl<T: Asset, L: MulLookup> ProvideFromDisk<T, L> for IndexedMulReader<T, L> {
    fn read_lookup_data(&mut self, id: u16) -> Result<Vec<u8>, Error> {
        let mut buffer = vec![0; std::mem::size_of::<L>()];
        let offset = id as usize * buffer.len();

        self.idx.seek(SeekFrom::Start(offset as u64))?;
        self.idx.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    fn read_resource_data(&mut self, id: u16, lookup: L) -> Result<Vec<u8>, Error> {
        let offset = match lookup.offset() {
            offset if offset == 0xFFFFFFFF => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("No data found for id {}", id),
                ))
            }
            offset => offset,
        };

        let mut buffer = vec![0; lookup.size()];

        self.mul.seek(SeekFrom::Start(offset))?;
        self.mul.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}
