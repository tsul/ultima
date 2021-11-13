use crate::parsers::color16::parse_color;
use crate::resources::MulLookup;
use crate::resources::{Asset, ImageAsset};
use image::RgbaImage;
use nom::multi::many0;

pub struct TexMap {
    pub image: RgbaImage,
}

impl Asset for TexMap {
    fn load(data: Vec<u8>, lookup: impl MulLookup) -> Option<TexMap> {
        let dim = if lookup.size() == 0x2000 { 64 } else { 128 };

        let colors = match many0(parse_color)(&data) {
            Err(_) => return None,
            Ok((_, colors)) => colors,
        };

        let mut buffer = Vec::new();
        for val in &colors {
            buffer.extend_from_slice(&val.to_le_bytes())
        }

        match RgbaImage::from_vec(dim, dim, buffer) {
            None => None,
            Some(image) => Some(TexMap { image }),
        }
    }
}

impl ImageAsset for TexMap {
    fn get_image(&self) -> &RgbaImage {
        &self.image
    }
}
