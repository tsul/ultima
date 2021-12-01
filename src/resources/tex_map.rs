use crate::parsers::color16::parse_color;
use crate::resources::{LoadFromMul, StandardMulLookup};
use image::RgbaImage;
use nom::multi::many0;
use std::io::{Error, ErrorKind};

pub struct TexMap {
    pub id: u16,
    pub image: RgbaImage,
}

impl TexMap {
    fn from_raw(id: u16, data: Vec<u8>, size: u32) -> Result<TexMap, Error> {
        let colors = match many0(parse_color)(&data) {
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Unable to parse data")),
            Ok((_, colors)) => colors,
        };

        let mut buffer = Vec::new();
        for val in &colors {
            buffer.extend_from_slice(&val.to_le_bytes())
        }

        match RgbaImage::from_vec(size, size, buffer) {
            None => return Err(Error::new(ErrorKind::InvalidData, "Unable to create image")),
            Some(image) => Ok(TexMap { id, image }),
        }
    }
}

impl LoadFromMul<StandardMulLookup> for TexMap {
    fn load(id: u16, data: Vec<u8>, lookup: StandardMulLookup) -> Result<TexMap, Error> {
        let size = if lookup.size == 0x2000 { 64 } else { 128 };

        TexMap::from_raw(id, data, size)
    }
}
