use crate::parsers::color16::parse_color;
use crate::resources::{Asset, MulLookup};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use image::RgbaImage;
use nom::{number::complete::le_u32, IResult};
use std::io::{Cursor, Seek, SeekFrom};

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
    pub image: RgbaImage,
}

impl Art {
    fn from_raw(data: Vec<u8>) -> Option<Art> {
        let mut reader = Cursor::new(data);

        let buffer: Vec<u8> = vec![0; 44 * 44 * 4];
        let mut writer = Cursor::new(buffer);

        let mut x: i64 = 21;
        let mut y: i64 = 0;
        let mut line_width = 2;

        while y < 22 {
            for o in 0..line_width {
                let color16 = reader.read_u16::<LittleEndian>().unwrap();
                let (_, color32) = parse_color(&color16.to_le_bytes()).unwrap();

                writer
                    .seek(SeekFrom::Start((y * 44 + x + o) as u64 * 4))
                    .unwrap();
                writer.write_u32::<LittleEndian>(color32).unwrap();
            }

            y += 1;
            x -= 1;
            line_width += 2;
        }

        x = 0;
        line_width = 44;

        while y < 44 {
            for o in 0..line_width {
                let color16 = reader.read_u16::<LittleEndian>().unwrap();
                let (_, color32) = parse_color(&color16.to_le_bytes()).unwrap();

                writer
                    .seek(SeekFrom::Start((y * 44 + x + o) as u64 * 4))
                    .unwrap();
                writer.write_u32::<LittleEndian>(color32).unwrap();
            }

            y += 1;
            x += 1;
            line_width -= 2;
        }

        match RgbaImage::from_vec(44, 44, writer.into_inner()) {
            None => None,
            Some(image) => Some(Art { image }),
        }
    }

    fn from_run(_: Vec<u8>) -> Option<Art> {
        None
    }
}

impl Asset for Art {
    fn load(data: Vec<u8>, _: impl MulLookup) -> Option<Art> {
        let data = match parse_entry(&data) {
            Err(_) => return None,
            Ok((_, data)) => data,
        };

        match data {
            ArtEncoding::Raw(d) => Art::from_raw(d),
            ArtEncoding::Run(d) => Art::from_run(d),
        }
    }
}
