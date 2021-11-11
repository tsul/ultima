use nom::{bytes::complete::take, number::complete::le_u32, IResult};

#[derive(Debug)]
pub enum ItemArtData {
    Raw(Vec<u8>),
    Run(Vec<u8>),
}

pub fn parse_art_entry(input: &[u8], size: usize) -> IResult<&[u8], ItemArtData> {
    let (i, flag) = le_u32(input)?;
    let (i, data) = take(size - 4)(i)?;

    Ok((
        i,
        match flag {
            flag if flag == 0 || flag > 0xFFFF => ItemArtData::Raw(data.to_vec()),
            _ => ItemArtData::Run(data.to_vec()),
        },
    ))
}
