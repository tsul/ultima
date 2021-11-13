use crate::resources::{Asset, MulLookup};
use std::io::{Error, ErrorKind};

pub trait ProvideFromDisk<T: Asset, L: MulLookup> {
    fn read_lookup_data(&mut self, id: u16) -> Result<Vec<u8>, Error>;
    fn read_resource_data(&mut self, id: u16, lookup: L) -> Result<Vec<u8>, Error>;

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

    fn load_resource(&mut self, id: u16) -> Result<T, Error> {
        let lookup = self.load_lookup(id)?;

        let data = self.read_resource_data(id, lookup)?;

        match T::load(data, lookup) {
            None => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unable to parse data for id {}", id),
            )),
            Some(data) => Ok(data),
        }
    }
}
