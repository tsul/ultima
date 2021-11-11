use nom::{number::complete::le_u32, sequence::tuple, IResult};

#[derive(Debug, Copy, Clone)]
pub struct IndexEntry {
    pub lookup: u32,
    pub size: usize,
}

pub fn parse_index_entry(input: &[u8]) -> IResult<&[u8], IndexEntry> {
    let (i, (lookup, size)) = tuple((le_u32, le_u32))(input)?;

    Ok((
        i,
        IndexEntry {
            lookup,
            size: size as usize,
        },
    ))
}
