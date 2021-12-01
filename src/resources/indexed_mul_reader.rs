use crate::resources::MulLookup;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom};
use std::marker::PhantomData;

pub trait LoadFromMul<L>
where
    Self: std::marker::Sized,
    L: MulLookup,
{
    fn load(id: u16, data: Vec<u8>, lookup: L) -> Result<Self, Error>;
}

#[derive(Debug)]
pub struct IndexedMulReader<T, L> {
    idx: BufReader<File>,
    mul: BufReader<File>,
    output_type: PhantomData<T>,
    lookup_type: PhantomData<L>,
}

impl<T: LoadFromMul<L>, L: MulLookup> IndexedMulReader<T, L> {
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

    pub fn load_asset(&mut self, id: u16) -> Result<T, Error> {
        let lookup = self.load_lookup(id)?;

        let data = self.read_asset_data(id, lookup)?;

        T::load(id, data, lookup)
    }

    fn read_lookup_data(&mut self, id: u16) -> Result<Vec<u8>, Error> {
        let mut buffer = vec![0; std::mem::size_of::<L>()];
        let offset = id as usize * buffer.len();

        self.idx.seek(SeekFrom::Start(offset as u64))?;
        self.idx.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    fn read_asset_data(&mut self, id: u16, lookup: L) -> Result<Vec<u8>, Error> {
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

    fn load_lookup(&mut self, id: u16) -> Result<L, Error> {
        let data = self.read_lookup_data(id)?;

        match L::from_bytes(&data) {
            Some(lookup) => Ok(lookup),
            None => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unable to parse index entry {}", id),
            )),
        }
    }
}
