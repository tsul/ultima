use nom::{bytes::complete::take, IResult};

pub type TexMapData = Vec<u8>;

pub fn parse_tex_map(input: &[u8], size: usize) -> IResult<&[u8], TexMapData> {
    let (i, data) = take(size)(input)?;

    Ok((i, data.to_vec()))
}
