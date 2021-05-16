use super::{
    parse::{parse_vec, ParseError, Result},
    wasm_type::FunctionType,
};
use crate::decode;
use std::convert::TryFrom;
use std::io::{Cursor, Seek, SeekFrom};
pub struct Section {
    pub id: u8,
    pub payload_len: u32,
    pub payload_data: SectionData,
}

impl Section {
    pub(crate) fn parse(data: &mut Cursor<&[u8]>) -> Result<Self> {
        let id = u8::try_from(decode::decode_varint(data)?)?;

        let payload_len = u32::try_from(decode::decode_varint(data)?)?;

        let payload_data = SectionData::parse(data, id, payload_len as usize)?;

        Ok(Self {
            id,
            payload_len,
            payload_data,
        })
    }

    pub(crate) fn parse_multi(data: &mut Cursor<&[u8]>) -> Result<Vec<Self>> {
        let current_position = data.position();
        let end = data.seek(SeekFrom::End(0))?;
        data.seek(SeekFrom::Start(current_position))?;
        let mut v = Vec::new();
        while end > data.position() {
            let value = Self::parse(data)?;
            v.push(value);
        }
        Ok(v)
    }
}

pub enum SectionData {
    Custom(CustomSection),
    Type(TypeSection),
    Import,
    Function(FunctionSection),
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code(CodeSection),
    Data,
    DataCount,
}

impl SectionData {
    fn parse(data: &mut Cursor<&[u8]>, id: u8, payload_len: usize) -> Result<Self> {
        let payload_data = decode::decode_len(data, payload_len)?;
        match id {
            0 => Ok(Self::Custom(CustomSection {})), // custom section は何もしない
            1 => Ok(Self::Type(TypeSection::parse(
                &mut payload_data.as_slice(),
            )?)),
            3 => Ok(Self::Function(FunctionSection::parse(
                &mut payload_data.as_slice(),
            )?)),
            10 => Ok(Self::Code(CodeSection {})),
            _ => Err(ParseError::UnexpectedSectionId(id)),
        }
    }
}
pub struct CustomSection {}

pub struct TypeSection {
    pub(super) funcs: Vec<FunctionType>,
}
impl TypeSection {
    fn parse(data: &mut &[u8]) -> Result<Self> {
        let num = u32::try_from(decode::decode_varint(data)?)?;
        let mut v = Vec::new();
        for _ in 0..num {
            v.push(FunctionType::parse(data)?);
        }
        Ok(Self { funcs: v })
    }
}
pub struct FunctionSection {
    pub(super) indexies: Vec<u32>,
}

impl FunctionSection {
    fn parse(data: &mut &[u8]) -> Result<Self> {
        let v = parse_vec(data, |data| {
            Ok(u32::try_from(decode::decode_varint(data)?)?)
        })?;
        Ok(Self { indexies: v })
    }
}
pub struct CodeSection {}
