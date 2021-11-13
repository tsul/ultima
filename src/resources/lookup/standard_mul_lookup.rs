use crate::resources::MulLookup;
use nom::{number::complete::le_u32, sequence::tuple, IResult};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct StandardMulLookup {
    offset: u32,
    size: u32,
    extra: u32,
}

impl MulLookup for StandardMulLookup {
    fn from_bytes(data: &[u8]) -> Option<StandardMulLookup> {
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

pub fn parse_index_entry(input: &[u8]) -> IResult<&[u8], StandardMulLookup> {
    let (i, (offset, size, extra)) = tuple((le_u32, le_u32, le_u32))(input)?;

    Ok((
        i,
        StandardMulLookup {
            offset,
            size,
            extra,
        },
    ))
}
