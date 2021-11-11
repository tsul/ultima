mod parsers;
mod readers;

use crate::parsers::{art::ItemArtData, color16::parse_color};
use crate::readers::{art::ItemArt, tex_maps::TexMaps};
use image::save_buffer_with_format;
use nom::multi::many0;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut item_art = ItemArt::new();
    let entry = item_art.read_entry(0x4200)?;

    match entry {
        ItemArtData::Raw(data) => match many0(parse_color)(&data).ok() {
            None => println!("Nothing to see here"),
            Some((_, colors)) => println!("raw, {:?}, {}", colors, colors.len()),
        },
        ItemArtData::Run(data) => println!("run, {:?}, {}", data, data.len()),
    }

    let mut tex_maps = TexMaps::new();
    let entry = tex_maps.read_entry(0x137)?;

    match many0(parse_color)(&entry).ok() {
        None => println!("Nothing to see here"),
        Some((_, colors)) => {
            let buffer: Vec<u8> = colors.iter().flat_map(|c| c.to_le_bytes()).collect();

            save_buffer_with_format(
                "test.png",
                &buffer,
                64,
                64,
                image::ColorType::Rgba8,
                image::ImageFormat::Png,
            )
            .unwrap()
        }
    }

    Ok(())
}
