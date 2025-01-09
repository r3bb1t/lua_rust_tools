use std::io::Read;

use crate::decoder::{
    error::{Error, Result},
    util,
};

pub mod header;
pub mod instruction;
pub mod opcodes;
//pub mod prototype;
pub mod prototype;

pub(super) fn read_uleb128<R: Read>(r: &mut R) -> Result<u32> {
    let mut value: u32 = util::read_u8(r)?.into();

    if value >= 0x80 {
        let mut bit_shift = 0;
        value &= 0x7f;

        loop {
            let byte: u32 = util::read_u8(r)?.into();

            bit_shift += 7;
            value |= (byte & 0x7f) << bit_shift;

            if byte < 0x80 {
                break;
            }
        }
    }
    Ok(value)
}

pub(super) fn read_uint<R: Read>(r: &mut R, is_big_endian: bool) -> Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    if is_big_endian {
        Ok(u32::from_be_bytes(buf))
    } else {
        Ok(u32::from_le_bytes(buf))
    }
}
