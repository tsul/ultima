use nom::{number::complete::le_u16, IResult};

pub type Color32 = u32;

pub fn parse_color(input: &[u8]) -> IResult<&[u8], Color32> {
    let (i, value) = le_u16(input)?;
    let value = value as u32;

    let r = ((value & 0x7C00) >> 10) << 3;
    let g = ((value & 0x3E0) >> 5) << 3;
    let b = (value & 0x1F) << 3;

    Ok((i, r | g << 8 | b << 16 | u32::MAX << 24))
}
