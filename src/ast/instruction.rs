use crate::decode;

use super::{
    parse::{parse_vec, ParseError, Result},
    wasm_type::{self, FunctionType},
};
use std::convert::TryFrom;
pub struct Expression {
    instrs: Vec<Instruction>,
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
}

impl Instruction {
    fn parse(data: &mut &[u8], by: u8) -> Result<Self> {
        match by {
            0x00..=0x11 => Ok(Self::Control(ControlInstruction::Nop)),
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

pub enum NumericInstruction {
    Const(ConstNumericInstruction),
}

impl NumericInstruction {
    fn parse(data: &mut &[u8], by: u8) -> Result<Self> {
        match by {
            0x41..=0x44 => Ok(Self::Const(ConstNumericInstruction::parse(data, by)?)),
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
