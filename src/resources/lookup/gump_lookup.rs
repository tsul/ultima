use crate::resources::MulLookup;
use nom::{
    number::complete::{le_u16, le_u32},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct GumpLookup {
    pub offset: u32,
    pub size: u32,
    pub height: u16,
    pub width: u16,
}

impl MulLookup for GumpLookup {
    fn from_bytes(data: &[u8]) -> Option<GumpLookup> {
        match parse_index_entry(data) {
            Err(_) => None,
            Ok((_, lookup)) => Some(lookup),
        }
    }

    fn offset(&self) -> u64 {
        self.offset as u64
    }

    fn size(&self) -> usize {
        self.size as usize
    }
}

pub fn parse_index_entry(input: &[u8]) -> IResult<&[u8], GumpLookup> {
    let (i, (offset, size, height, width)) = tuple((le_u32, le_u32, le_u16, le_u16))(input)?;

    Ok((
        i,
        GumpLookup {
            offset,
            size,
            width,
            height,
        },
    ))
}
