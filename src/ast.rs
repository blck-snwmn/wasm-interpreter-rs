use crate::decode;
use std::{
    convert::TryFrom,
    io::{Cursor, Seek, SeekFrom},
};
use thiserror::Error;

mod instruction;
mod module;
mod parse;
mod section;
mod wasm_type;

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

#[cfg(test)]
mod test {

    use crate::ast::{section::SectionData, wasm_type::ValueType};

    use super::*;
    #[test]
    fn test_parse_module() {
        {
            // (module)
            let min_input: &[u8] = &[0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
            let min_input = &mut Cursor::new(min_input);
            let result = module::Module::parse(min_input);
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
            // (module
            //   (func $add  (result i32)
            //   i32.const 42
            //   )
            // )
            let input: &[u8] = &[
                0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // magic number, version
                0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f, // type section
                0x03, 0x02, 0x01, 0x00, // function section
                // code section
                0x0a, // id
                0x06, // length
                0x01, // number of element
                0x04, // length of code
                0x00, // locals
                0x41, 0x2a, // expr
                0x0b, // end
                // custom section
                0x00, 0x12, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x61, 0x64,
                0x64, 0x02, 0x03, 0x01, 0x00, 0x00,
            ];
            let input = &mut Cursor::new(input);
            let result = module::Module::parse(input);
            if let Err(e) = &result {
                println!("{:?}", e);
            }
            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(result.magic_number, 0x6d736100);
            assert_eq!(result.version, 1);
            assert!(!result.sections.is_empty());

            let elm = &result.sections.get(0).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Type(_)));

            if let SectionData::Type(ty) = elm {
                assert_eq!(ty.funcs.len(), 1);
                for f in &ty.funcs {
                    let ps = &f.params_types;
                    assert_eq!(ps.valu_types.len(), 0);

                    let rs = &f.return_types;
                    assert_eq!(rs.valu_types.len(), 1);
                    let r = rs.valu_types.get(0).unwrap();
                    assert!(matches!(r, ValueType::Number(_)));
                }
            }

            let elm = &result.sections.get(1).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Function(_)));

            if let SectionData::Function(fs) = elm {
                assert_eq!(fs.indexies.len(), 1);
                assert_eq!(*fs.indexies.get(0).unwrap(), 0x00);
            }

            let elm = &result.sections.get(2).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Code(_)));

            let elm = &result.sections.get(3).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Custom(_)));
        }
    }
}
