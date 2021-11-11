use nom::{number::complete::le_u16, IResult};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn to_le_bytes(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }
}

pub fn parse_color(input: &[u8]) -> IResult<&[u8], Color> {
    let (i, value) = le_u16(input)?;

    let r = (((value & 0x7C00) >> 10) << 3) as u8;
    let g = (((value & 0x3E0) >> 5) << 3) as u8;
    let b = ((value & 0x1F) << 3) as u8;

    Ok((
        i,
        Color {
            r,
            g,
            b,
            a: u8::MAX,
        },
    ))
}
