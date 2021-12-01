mod parsers;
mod readers;
mod resources;

use crate::resources::{Art, IndexedMulReader, ProvideFromDisk, StandardMulLookup, TexMap};
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut tex_map_reader = IndexedMulReader::<TexMap, StandardMulLookup>::new(
        "/Users/tsul/Outlands/texidx.mul",
        "/users/tsul/Outlands/texmaps.mul",
    );

    let tex_map = tex_map_reader.load_resource(0x137)?;

    tex_map
        .image
        .save_with_format("output/tex-0x137.png", image::ImageFormat::Png)
        .unwrap();

    let mut art_reader = IndexedMulReader::<Art, StandardMulLookup>::new(
        "/Users/tsul/Outlands/artidx.mul",
        "/users/tsul/Outlands/art.mul",
    );

    let art = art_reader.load_resource(0x137)?;

    art.image
        .save_with_format("output/art-0x137.png", image::ImageFormat::Png)
        .unwrap();

    let art = art_reader.load_resource(0x420 + 0x4000)?;

    art.image
        .save_with_format(
            format!("output/art-{}.png", 0x420 + 0x4000),
            image::ImageFormat::Png,
        )
        .unwrap();

    Ok(())
}
