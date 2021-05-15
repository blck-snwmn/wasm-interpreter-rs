use crate::decode;
use std::{
    convert::TryFrom,
    io::{Cursor, Seek, SeekFrom},
};
use thiserror::Error;
#[derive(Error, Debug)]
pub(crate) enum ParseError {
    #[error("faild to decode: {0}")]
    Decode(#[from] decode::DecodeError),
    #[error("faild to convert: {0}")]
    Convert(#[from] std::num::TryFromIntError),
    #[error("read failed: {0}")]
    Read(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, ParseError>;

// uintN
pub(crate) type uint8 = u8;
pub(crate) type uint32 = u32;
pub(crate) type uint64 = u64;

// varintN
pub(crate) type varuint1 = u8;
pub(crate) type varuint7 = u8;
pub(crate) type varuint32 = u32;

// varuintN
pub(crate) type varint7 = i8;
pub(crate) type varint32 = i32;
pub(crate) type varint64 = i64;

pub struct Module {
    pub magic_number: uint32,
    pub version: uint32,
    pub sections: Vec<Section>,
}

impl Module {
    pub(crate) fn new(magic_number: uint32, version: uint32, sections: Vec<Section>) -> Self {
        Self {
            magic_number,
            version,
            sections,
        }
    }

    pub(crate) fn parse(data: &mut Cursor<&[u8]>) -> Result<Self> {
        let magic_number = u32::from_le_bytes(decode::decode_32bit(data)?);
        let version = u32::from_le_bytes(decode::decode_32bit(data)?);
        let sections = Section::parse_multi(data)?;
        Ok(Self::new(magic_number, version, sections))
    }
}

pub struct Section {
    pub id: varuint7,
    pub payload_len: varuint32,
    pub payload_data: Vec<u8>,
}

impl Section {
    pub(crate) fn parse(data: &mut Cursor<&[u8]>) -> Result<Self> {
        let id = u8::try_from(decode::decode_varint(data)?)?;

        let payload_len = u32::try_from(decode::decode_varint(data)?)?;

        let payload_data = decode::decode_len(data, payload_len as usize)?;

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
    Type,
    Import,
    Function,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_parse_module() {
        {
            let min_input: &[u8] = &[0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
            let min_input = &mut Cursor::new(min_input);
            let result = Module::parse(min_input);
            if let Err(e) = &result {
                println!("{:?}", e);
            }
            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(result.magic_number, 0x6d736100);
            assert_eq!(result.version, 1);
            assert!(result.sections.is_empty());
            let current = min_input.position();
            let end = min_input.seek(SeekFrom::End(0)).unwrap();
            assert_eq!(current, end);
        }
        {
            let input: &[u8] = &[
                0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // magic number, version
                0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f, // type section
                0x03, 0x02, 0x01, 0x00, // function section
                0x0a, 0x06, 0x01, 0x04, 0x00, 0x41, 0x2a, 0x0b, // code section
                0x00, 0x12, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x61, 0x64,
                0x64, 0x02, 0x03, 0x01, 0x00, 0x00, // custom section
            ];
            let input = &mut Cursor::new(input);
            let result = Module::parse(input);
            if let Err(e) = &result {
                println!("{:?}", e);
            }
            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(result.magic_number, 0x6d736100);
            assert_eq!(result.version, 1);
            assert!(!result.sections.is_empty());
        }
    }
}
