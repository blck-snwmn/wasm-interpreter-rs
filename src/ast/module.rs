use std::io::Cursor;

use crate::decode;

use super::parse::Result;
use super::section::Section;

pub struct Module {
    pub magic_number: u32,
    pub version: u32,
    pub sections: Vec<Section>,
}

impl Module {
    pub(crate) fn new(magic_number: u32, version: u32, sections: Vec<Section>) -> Self {
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
