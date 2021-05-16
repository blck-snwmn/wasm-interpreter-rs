use crate::decode;

use super::{
    parse::{parse_vec, ParseError, Result},
    wasm_type::{self, FunctionType},
};
use std::convert::TryFrom;
pub struct Expression {
    pub(super) instrs: Vec<Instruction>,
}

impl Expression {
    pub(super) fn parse(data: &mut &[u8]) -> Result<Self> {
        let mut v = Vec::new();
        loop {
            let by = u8::try_from(decode::decode_varint(data)?)?;
            match by {
                0x0b => break Ok(Self { instrs: v }),
                by => match Instruction::parse(data, by) {
                    Ok(instr) => v.push(instr),
                    Err(e) => break Err(e),
                },
            }
        }
    }
}

pub enum Instruction {
    Control(ControlInstruction),
    Numeric(NumericInstruction),
    Variable(VariableInstruction),
}

impl Instruction {
    fn parse(data: &mut &[u8], by: u8) -> Result<Self> {
        match by {
            0x00..=0x11 => Ok(Self::Control(ControlInstruction::Nop)),
            0x20..=0x24 => Ok(Self::Variable(VariableInstruction::parse(data, by)?)),
            0x41..=0xC4 => Ok(Self::Numeric(NumericInstruction::parse(data, by)?)),
            _ => Err(ParseError::UnexpectedByteValue {
                title: "Instruction".to_string(),
                got: by,
            }),
        }
    }
}

pub enum ControlInstruction {
    Unreachable,
    Nop,
    Block,
    Loop,
    IfElse,
    Br,
    BrIf,
    BrTable,
    Return,
    Call,
    CallIndirect,
}

impl ControlInstruction {
    fn new(by: u8) -> Option<Self> {
        match by {
            0x00 => Some(ControlInstruction::Unreachable),
            0x01 => Some(ControlInstruction::Nop),
            0x02 => Some(ControlInstruction::Block),
            0x03 => Some(ControlInstruction::Loop),
            0x04 => Some(ControlInstruction::IfElse),
            0x0C => Some(ControlInstruction::Br),
            0x0D => Some(ControlInstruction::BrIf),
            0x0E => Some(ControlInstruction::BrTable),
            0x0F => Some(ControlInstruction::Return),
            0x10 => Some(ControlInstruction::Call),
            0x11 => Some(ControlInstruction::CallIndirect),
            _ => None,
        }
    }
}

pub enum VariableInstruction {
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),
}
impl VariableInstruction {
    fn parse(data: &mut &[u8], by: u8) -> Result<Self> {
        let index = u32::try_from(decode::decode_varint(data)?)?;
        match by {
            0x20 => Ok(Self::LocalGet(index)),
            0x21 => Ok(Self::LocalSet(index)),
            0x22 => Ok(Self::LocalTee(index)),
            0x23 => Ok(Self::GlobalGet(index)),
            0x24 => Ok(Self::GlobalSet(index)),
            _ => Err(ParseError::UnexpectedByteValue {
                title: "VariableInstruction".to_string(),
                got: by,
            }),
        }
    }
}

pub enum NumericInstruction {
    Const(ConstNumericInstruction),
    Plain(PlainNumericInstruction),
    SaturatingTruncation,
}

impl NumericInstruction {
    fn parse(data: &mut &[u8], by: u8) -> Result<Self> {
        match by {
            0x41..=0x44 => Ok(Self::Const(ConstNumericInstruction::parse(data, by)?)),
            0x45..=0xC4 => Ok(Self::Plain(PlainNumericInstruction::parse(data, by)?)),
            0xFC => Ok(Self::SaturatingTruncation),
            _ => Err(ParseError::UnexpectedByteValue {
                title: "NumericInstruction".to_string(),
                got: by,
            }),
        }
    }
}

pub enum ConstNumericInstruction {
    ConstI32(i32),
    ConstI64(i64),
    ConstF32(f32),
    ConstF64(f64),
}

impl ConstNumericInstruction {
    fn parse(data: &mut &[u8], by: u8) -> Result<Self> {
        let value = decode::decode_varint(data)?;
        match by {
            0x41 => {
                if value > u32::MAX as u64 {
                    return Err(ParseError::UnexpectedValue(format!(
                        "unexpected value. this value is greater than {}(u64::MAX)",
                        u64::MAX
                    )));
                }
                Ok(Self::ConstI32(value as i32))
            }
            0x42 => Ok(Self::ConstI64(value as i64)),
            0x43 => {
                if value > u32::MAX as u64 {
                    return Err(ParseError::UnexpectedValue(format!(
                        "unexpected value. this value is greater than {}(u64::MAX)",
                        u64::MAX
                    )));
                }
                Ok(Self::ConstF32(value as f32))
            }
            0x44 => Ok(Self::ConstF64(value as f64)),
            _ => Err(ParseError::UnexpectedByteValue {
                title: "ConstNumericInstruction".to_string(),
                got: by,
            }),
        }
    }
}

pub enum PlainNumericInstruction {
    AddI32,
}

impl PlainNumericInstruction {
    fn parse(data: &mut &[u8], by: u8) -> Result<Self> {
        match by {
            0x6a => Ok(Self::AddI32),
            _ => Err(ParseError::UnexpectedByteValue {
                title: "PlainNumericInstruction".to_string(),
                got: by,
            }),
        }
    }
}
