use std::io::{Cursor, Read};

use thiserror::Error;

use crate::ast;
#[derive(Error, Debug)]
enum DecodeError {
    #[error("unexpected format. end come after MSB is 0")]
    UnexpectFormat,
    #[allow(dead_code)]
    #[error("unexpected red size. got={0}, want={1}")]
    UnexpectRepeatSize(u128, u128),
    #[error("no expected type value. got={0}")]
    UnexpectedWireDataValue(u128),
    #[error("")]
    FailedToRead(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, DecodeError>;

fn decode_varints<T: std::io::Read>(data: &mut T) -> Result<u64> {
    // とりあえずu64で読んで、その後工程で varuint32へ変換できるかどうか見る方針

    // iterate take_util とかでもできるよ
    let mut sum = 0;
    let mut loop_count = 0;
    loop {
        let mut buf = [0; 1];
        let result = data.read_exact(&mut buf);
        if result.is_err() {
            return Err(DecodeError::UnexpectFormat);
        }
        // MSB は後続のバイトが続くかどうかの判定に使われる
        // 1 の場合、後続が続く
        let top = buf[0] & 0b10000000;
        let buf: u64 = (buf[0] & 0b01111111) as u64;
        // little endian
        let buf = buf << (7 * loop_count);
        sum += buf;
        loop_count += 1;
        if top != 0b10000000 {
            return Ok(sum);
        }
    }
}

fn decode_nbit<T: std::io::Read, const SIZE: usize>(data: &mut T) -> Result<[u8; SIZE]> {
    let mut buf = [0; SIZE];
    data.read_exact(&mut buf)?;
    Ok(buf)
}

fn decode_8bit<T: std::io::Read>(data: &mut T) -> Result<[u8; 1]> {
    decode_nbit(data)
}
fn decode_16bit<T: std::io::Read>(data: &mut T) -> Result<[u8; 2]> {
    decode_nbit(data)
}
fn decode_32bit<T: std::io::Read>(data: &mut T) -> Result<[u8; 4]> {
    decode_nbit(data)
}
fn decode_module<T: std::io::Read>(data: &mut T) -> Result<ast::Module> {
    let magic_number = u32::from_le_bytes(decode_32bit(data)?);
    let version = u32::from_le_bytes(decode_32bit(data)?);
    Ok(ast::Module::new(magic_number, version, None))
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn read_module() {
        {
            let mut min_input: &[u8] = &[0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
            let result = decode_module(&mut min_input);
            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(result.magic_number, 0x6d736100);
            assert_eq!(result.version, 1);
            assert!(result.sections.is_none());
        }
    }
}
