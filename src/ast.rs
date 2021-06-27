pub mod instruction;
pub mod module;
pub mod parse;
pub mod section;
pub mod wasm_type;

// TODO
// parser 関数は公開しない。構造体は公開する（メソッドやフィールドは非公開がよさそう）
// このファイルにmoduleをパースする公開関数を作る

#[cfg(test)]
mod test {

    use super::*;
    use crate::ast::{
        instruction::*,
        section::SectionData,
        wasm_type::{NumberType, ValueType},
    };
    use std::io::{Cursor, Seek, SeekFrom};
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
                    assert!(matches!(r, ValueType::Number(NumberType::I32)));
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

            if let SectionData::Code(cs) = elm {
                assert_eq!(cs.codes.len(), 1);
                let c = cs.codes.get(0).unwrap();
                assert_eq!(c.locals.len(), 0);

                let exp = &c.expression;
                assert_eq!(exp.instrs.len(), 1);
                let instr = exp.instrs.get(0).unwrap();
                assert!(matches!(
                    instr,
                    Instruction::Numeric(NumericInstruction::Const(
                        ConstNumericInstruction::ConstI32(42)
                    ))
                ));
            }

            let elm = &result.sections.get(3).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Custom(_)));
        }
        {
            // (module
            // (func $add (param $lhs i32) (param $rhs i32) (result i32)
            //     get_local $lhs
            //     get_local $rhs
            //     i32.add)
            // (export "add" (func $add))
            // )
            let input: &[u8] = &[
                0x00, 0x61, 0x73, 0x6d, // magic number
                0x01, 0x00, 0x00, 0x00, // version
                // type section
                0x01, 0x07, // id, length
                0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, // a
                // function section
                0x03, 0x02, // id, length
                0x01, 0x00, // a
                // export section
                0x07, 0x07, // id, length
                0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, // a
                // code section
                0x0a, 0x09, // id, length
                0x01, // number of element
                0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b, // a
                // custom section (ignore)
                0x00, 0x1c, // id, length
                0x04, 0x6e, 0x61, 0x6d, 0x65, 0x01, 0x06, 0x01, 0x00, 0x03, 0x61, 0x64, 0x64, 0x02,
                0x0d, 0x01, 0x00, 0x02, 0x00, 0x03, 0x6c, 0x68, 0x73, 0x01, 0x03, 0x72, 0x68, 0x73,
            ];
            let input = &mut Cursor::new(input);
            let result = module::Module::parse(input);
            if let Err(e) = &result {
                println!("{:?}", e.to_string());
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
                    assert_eq!(ps.valu_types.len(), 2);
                    let r0 = ps.valu_types.get(0).unwrap();
                    assert!(matches!(r0, ValueType::Number(n) if matches!(n, NumberType::I32)));
                    let r1 = ps.valu_types.get(1).unwrap();
                    assert!(matches!(r1, ValueType::Number(n)if matches!(n, NumberType::I32)));

                    let rs = &f.return_types;
                    assert_eq!(rs.valu_types.len(), 1);
                    let r = rs.valu_types.get(0).unwrap();
                    assert!(matches!(r, ValueType::Number(n) if matches!(n, NumberType::I32)));
                }
            }

            let elm = &result.sections.get(1).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Function(_)));

            if let SectionData::Function(fs) = elm {
                assert_eq!(fs.indexies.len(), 1);
                assert_eq!(*fs.indexies.get(0).unwrap(), 0x00);
            }
            let elm = &result.sections.get(2).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Export));

            let elm = &result.sections.get(3).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Code(_)));

            if let SectionData::Code(cs) = elm {
                assert_eq!(cs.codes.len(), 1);
                let c = cs.codes.get(0).unwrap();
                assert_eq!(c.locals.len(), 0);

                let exp = &c.expression;
                assert_eq!(exp.instrs.len(), 3);
                let x = exp.instrs.get(0);
                assert!(matches!(
                    exp.instrs.get(0),
                    Some(Instruction::Variable(VariableInstruction::LocalGet(0x00)))
                ));
                assert!(matches!(
                    exp.instrs.get(1),
                    Some(Instruction::Variable(VariableInstruction::LocalGet(0x01)))
                ));
                assert!(matches!(
                    exp.instrs.get(2),
                    Some(Instruction::Numeric(NumericInstruction::Plain(
                        PlainNumericInstruction::AddI32
                    )))
                ));
            }

            let elm = &result.sections.get(4).unwrap().payload_data;
            assert!(matches!(elm, SectionData::Custom(_)));
        }
    }
}
