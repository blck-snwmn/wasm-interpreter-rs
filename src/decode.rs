use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum DecodeError {
    #[error("unexpected format. end come after MSB is 0")]
    UnexpectFormat,
    #[error("no expected type value. got={0}")]
    UnexpectedWireDataValue(u128),
    #[error("read failed: {0}")]
    FailedToRead(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, DecodeError>;

pub(crate) fn decode_varint<T: std::io::Read>(data: &mut T) -> Result<u64> {
    // とりあえずu64で読んで、その後工程で varuint32へ変換できるかどうか見る方針

    let mut sum = 0;
    let mut loop_count = 0;
    loop {
        let mut buf = [0; 1];
        let result = data.read_exact(&mut buf);
        if result.is_err() {
            break Err(DecodeError::UnexpectFormat);
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
            break Ok(sum);
        }
    }
}

pub(crate) fn decode_len<T: std::io::Read>(data: &mut T, len: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0; len];
    data.read_exact(&mut buf)?;
    Ok(buf)
}

fn decode_nbit<T: std::io::Read, const SIZE: usize>(data: &mut T) -> Result<[u8; SIZE]> {
    let mut buf = [0; SIZE];
    data.read_exact(&mut buf)?;
    Ok(buf)
}

pub(crate) fn decode_8bit<T: std::io::Read>(data: &mut T) -> Result<[u8; 1]> {
    decode_nbit(data)
}
pub(crate) fn decode_16bit<T: std::io::Read>(data: &mut T) -> Result<[u8; 2]> {
    decode_nbit(data)
}
pub(crate) fn decode_32bit<T: std::io::Read>(data: &mut T) -> Result<[u8; 4]> {
    decode_nbit(data)
}
