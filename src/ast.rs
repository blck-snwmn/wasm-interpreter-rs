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

pub struct Module<'a> {
    pub magic_number: uint32,
    pub version: uint32,
    pub sections: Option<&'a [Section]>,
}

impl<'a> Module<'a> {
    pub(crate) fn new(
        magic_number: uint32,
        version: uint32,
        sections: Option<&'a [Section]>,
    ) -> Self {
        Self {
            magic_number,
            version,
            sections,
        }
    }
}

pub struct Section {
    id: varuint7,
    payload_len: varuint32,
    payload_data: SectionData,
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
