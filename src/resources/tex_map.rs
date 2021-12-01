use crate::parsers::color16::parse_color;
use crate::resources::MulLookup;
use crate::resources::{Asset, ImageAsset};
use image::RgbaImage;
use nom::multi::many0;
use std::io::{Error, ErrorKind};

pub struct TexMap {
    pub image: RgbaImage,
}

impl TexMap {
    fn from_raw(data: Vec<u8>, size: u32) -> Result<TexMap, Error> {
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
            Some(image) => Ok(TexMap { image }),
        }
    }
}

impl Asset for TexMap {
    fn load(data: Vec<u8>, lookup: impl MulLookup) -> Option<TexMap> {
        let size = if lookup.size() == 0x2000 { 64 } else { 128 };

        TexMap::from_raw(data, size).ok()
    }
}

impl ImageAsset for TexMap {
    fn get_image(&self) -> &RgbaImage {
        &self.image
    }
}
