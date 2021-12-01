use crate::parsers::color16::parse_color;
use crate::resources::{GumpLookup, LoadFromMul};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use image::RgbaImage;
use std::io::{Cursor, Error, ErrorKind, Seek, SeekFrom};

pub struct Gump {
    pub id: u16,
    pub image: RgbaImage,
}

impl Gump {
    fn from_run(id: u16, data: Vec<u8>, width: u16, height: u16) -> Result<Gump, Error> {
        let mut reader = Cursor::new(data);

        let mut row_lookup: Vec<u32> = vec![0; height as usize];

        for i in 0..height {
            let value = reader.read_u32::<LittleEndian>()?;
            row_lookup.insert(i as usize, value);
        }

        let buffer: Vec<u8> = vec![0; width as usize * height as usize * 4];
        let mut writer = Cursor::new(buffer);

        for y in 0..height {
            reader.seek(SeekFrom::Start(row_lookup[y as usize] as u64 * 4))?;

            let mut x = 0;

            while x <= width {
                let color16 = reader.read_u16::<LittleEndian>()?;

                let (_, color32) = match parse_color(&color16.to_le_bytes()) {
                    Ok(color) => color,
                    Err(_) => {
                        return Err(Error::new(ErrorKind::InvalidData, "Unable to parse data"))
                    }
                };

                let count = reader.read_u16::<LittleEndian>()?;

                for c in 0..count {
                    writer.seek(SeekFrom::Start((x as u16 + c + (y * width)) as u64 * 4))?;
                    writer.write_u32::<LittleEndian>(color32)?;
                }

                x += count;
            }
        }

        match RgbaImage::from_vec(width as u32, height as u32, writer.into_inner()) {
            None => Err(Error::new(ErrorKind::InvalidData, "Unable to create image")),
            Some(image) => Ok(Gump { id, image }),
        }
    }
}

impl LoadFromMul<GumpLookup> for Gump {
    fn load(id: u16, data: Vec<u8>, lookup: GumpLookup) -> Result<Gump, Error> {
        Gump::from_run(id, data, lookup.width, lookup.height)
    }
}
