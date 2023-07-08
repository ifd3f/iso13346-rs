use nom::{bytes::complete::take, combinator::map_res, IResult};

pub trait Parse
where
    Self: Sized,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self>;
}

impl<T> Parse for T
where
    T: TryFrom<u8>,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(1usize), |d: &[u8]| Self::try_from(d[0]))(input)
    }
}
