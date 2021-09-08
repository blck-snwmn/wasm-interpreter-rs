use super::parse::{parse_vec, ParseError, Result};
use crate::decode;
use std::convert::TryFrom;
pub enum Type {
    Function(FunctionType),
    Result(ResultType),
    Value(ValueType),
    Number(NumberType),
    Reference(ReferenceType),
}

#[derive(Clone)]
pub struct FunctionType {
    pub params_types: ResultType,
    pub return_types: ResultType,
}
impl FunctionType {
    pub(super) fn parse(data: &mut &[u8]) -> Result<Self> {
        let x = u8::try_from(decode::decode_varint(data)?)?;
        if x != 0x60 {
            return Err(ParseError::UnexpectedByteValue {
                title: "function type".to_string(),
                got: x,
            });
        }
        let params_types = ResultType::parse(data)?;
        let return_types = ResultType::parse(data)?;
        Ok(Self {
            params_types,
            return_types,
        })
    }
}

#[derive(Clone)]
pub struct ResultType {
    pub(super) valu_types: Vec<ValueType>,
}
impl ResultType {
    fn parse(data: &mut &[u8]) -> Result<Self> {
        let v = parse_vec(data, |data| ValueType::parse(data))?;
        Ok(Self { valu_types: v })
    }
}

#[derive(Clone)]
pub enum ValueType {
    Number(NumberType),
    Reference(ReferenceType),
}
impl ValueType {
    pub(super) fn parse(data: &mut &[u8]) -> Result<Self> {
        let by = u8::try_from(decode::decode_varint(data)?)?;
        if let Some(num_type) = NumberType::new(by) {
            Ok(Self::Number(num_type))
        } else if let Some(ref_type) = ReferenceType::new(by) {
            Ok(Self::Reference(ref_type))
        } else {
            Err(ParseError::UnexpectedByteValue {
                title: "value type".to_string(),
                got: by,
            })
        }
    }
}

#[derive(Clone)]
pub enum NumberType {
    I32,
    I64,
    F32,
    F64,
}
impl NumberType {
    fn new(by: u8) -> Option<Self> {
        match by {
            0x7f => Some(Self::I32),
            0x7E => Some(Self::I64),
            0x7D => Some(Self::F32),
            0x7C => Some(Self::F64),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub enum ReferenceType {
    FunctionRef,
    ExternRef,
}
impl ReferenceType {
    fn new(by: u8) -> Option<Self> {
        match by {
            0x70 => Some(Self::FunctionRef),
            0x6f => Some(Self::ExternRef),
            _ => None,
        }
    }
}
