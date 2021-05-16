use thiserror::Error;

use crate::decode;
use std::convert::TryFrom;
#[derive(Error, Debug)]
pub(crate) enum ParseError {
    #[error("faild to decode: {0}")]
    Decode(#[from] decode::DecodeError),
    #[error("faild to convert: {0}")]
    Convert(#[from] std::num::TryFromIntError),
    #[error("read failed: {0}")]
    Read(#[from] std::io::Error),
    #[error("id={0} is unexpected section id.")]
    UnexpectedSectionId(u8),
    #[error("unexpected value in {title}. got=0x{got:0>2x}")]
    UnexpectedByteValue { title: String, got: u8 },
}

pub(crate) type Result<T> = std::result::Result<T, ParseError>;

pub(super) fn parse_vec<F, T>(data: &mut &[u8], func: F) -> Result<Vec<T>>
where
    F: Fn(&mut &[u8]) -> Result<T>,
    T: Sized,
{
    let num = u32::try_from(decode::decode_varint(data)?)?;
    let mut v = Vec::new();
    for _ in 0..num {
        v.push(func(data)?);
    }
    Ok(v)
}
