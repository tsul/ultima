use crate::parsers::color16::parse_color;
use crate::resources::{LoadFromMul, StandardMulLookup};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use image::RgbaImage;
use nom::{number::complete::le_u32, IResult};
use std::io::{Cursor, Error, ErrorKind, Seek, SeekFrom};

#[derive(Debug)]
enum ArtEncoding {
    Raw(Vec<u8>),
    Run(Vec<u8>),
}

fn parse_entry(input: &[u8]) -> IResult<&[u8], ArtEncoding> {
    let (i, flag) = le_u32(input)?;

    Ok((
        i,
        match flag {
            flag if flag == 0 || flag > 0xFFFF => ArtEncoding::Raw(input.to_vec()),
            _ => ArtEncoding::Run(input.to_vec()),
        },
    ))
}

pub struct Art {
    pub id: u16,
    pub image: RgbaImage,
}

impl Art {
    fn from_raw(id: u16, data: Vec<u8>) -> Result<Art, Error> {
        let mut reader = Cursor::new(data);

        let buffer: Vec<u8> = vec![0; 44 * 44 * 4];
        let mut writer = Cursor::new(buffer);

        let mut x: i64 = 21;
        let mut y: i64 = 0;
        let mut line_width = 2;

        while y < 22 {
            for o in 0..line_width {
                let color16 = reader.read_u16::<LittleEndian>()?;

                let (_, color32) = match parse_color(&color16.to_le_bytes()) {
                    Ok(color) => color,
                    Err(_) => {
                        return Err(Error::new(ErrorKind::InvalidData, "Unable to parse data"))
                    }
                };

                writer.seek(SeekFrom::Start((y * 44 + x + o) as u64 * 4))?;
                writer.write_u32::<LittleEndian>(color32)?;
            }

            y += 1;
            x -= 1;
            line_width += 2;
        }

        x = 0;
        line_width = 44;

        while y < 44 {
            for o in 0..line_width {
                let color16 = reader.read_u16::<LittleEndian>()?;

                let (_, color32) = match parse_color(&color16.to_le_bytes()) {
                    Ok(color) => color,
                    Err(_) => {
                        return Err(Error::new(ErrorKind::InvalidData, "Unable to parse data"))
                    }
                };

                writer.seek(SeekFrom::Start((y * 44 + x + o) as u64 * 4))?;
                writer.write_u32::<LittleEndian>(color32)?;
            }

            y += 1;
            x += 1;
            line_width -= 2;
        }

        match RgbaImage::from_vec(44, 44, writer.into_inner()) {
            None => Err(Error::new(ErrorKind::InvalidData, "Unable to create image")),
            Some(image) => Ok(Art { id, image }),
        }
    }

    fn from_run(id: u16, data: Vec<u8>) -> Result<Art, Error> {
        let mut reader = Cursor::new(data);

        reader.read_u32::<LittleEndian>()?;

        let width = reader.read_u16::<LittleEndian>()?;

        if width == 0 || width >= 1024 {
            return Err(Error::new(ErrorKind::InvalidData, "Data is invalid"));
        }

        let height = reader.read_u16::<LittleEndian>()?;

        if height == 0 || height >= 1024 {
            return Err(Error::new(ErrorKind::InvalidData, "Data is invalid"));
        }

        let buffer: Vec<u8> = vec![0; width as usize * height as usize * 4];
        let mut writer = Cursor::new(buffer);

        let mut l_start: Vec<u16> = vec![0; height as usize];

        for i in 0..height {
            let value = reader.read_u16::<LittleEndian>()?;
            l_start.insert(i as usize, value);
        }

        let mut x: u16 = 0;
        let mut y: u16 = 0;

        let d_start = reader.position();

        reader.seek(SeekFrom::Start(d_start + (l_start[y as usize] as u64) * 2))?;

        while y < height {
            let x_offset = reader.read_u16::<LittleEndian>()?;
            let run = reader.read_u16::<LittleEndian>()?;

            if x_offset + run >= 2048 {
                return Err(Error::new(ErrorKind::InvalidData, "Data is invalid"));
            }

            if x_offset + run != 0 {
                x += x_offset;

                for i in 0..run {
                    let color16 = reader.read_u16::<LittleEndian>()?;

                    let (_, color32) = match parse_color(&color16.to_le_bytes()) {
                        Ok(color) => color,
                        Err(_) => {
                            return Err(Error::new(ErrorKind::InvalidData, "Unable to parse data"))
                        }
                    };

                    writer.seek(SeekFrom::Start((x + i + (y * width)) as u64 * 4))?;
                    writer.write_u32::<LittleEndian>(color32)?;
                }

                x += run;
            } else {
                x = 0;
                y += 1;

                reader.seek(SeekFrom::Start(d_start + (l_start[y as usize] as u64) * 2))?;
            }
        }

        match RgbaImage::from_vec(width as u32, height as u32, writer.into_inner()) {
            None => Err(Error::new(ErrorKind::InvalidData, "Unable to create image")),
            Some(image) => Ok(Art { id, image }),
        }
    }
}

impl LoadFromMul<StandardMulLookup> for Art {
    fn load(id: u16, data: Vec<u8>, _: StandardMulLookup) -> Option<Art> {
        let data = match parse_entry(&data) {
            Err(_) => return None,
            Ok((_, data)) => data,
        };

        match data {
            ArtEncoding::Raw(d) => Art::from_raw(id, d).ok(),
            ArtEncoding::Run(d) => Art::from_run(id, d).ok(),
        }
    }
}
