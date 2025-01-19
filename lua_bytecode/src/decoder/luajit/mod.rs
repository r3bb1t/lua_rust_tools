use super::{
    error::{Error, Result},
    util::{self, read_u8},
};
use header::LuaJitHeader;
use prototype::LuaJitPrototype;
use std::io::Read;

pub mod constants;
pub mod debuginfo;
pub mod header;
pub mod instruction;
pub mod opcodes;
pub mod prototype;
pub mod table;

#[derive(Clone, Debug, PartialEq)]
pub struct DecodedLuaJitBytecode {
    pub header: LuaJitHeader,
    pub prototypes: Vec<LuaJitPrototype>,
}

impl DecodedLuaJitBytecode {
    pub fn from_read<R: Read>(mut r: R) -> Result<Self> {
        let header = LuaJitHeader::from_read(&mut r)?;

        let mut prototypes = vec![];

        let mut prototype_count = 0;
        loop {
            match LuaJitPrototype::from_read(&mut r, &header, &mut prototype_count) {
                Ok(prototype_option) => match prototype_option {
                    Some(prototype) => prototypes.push(prototype),
                    None => break,
                },
                Err(e) => return Err(e),
            }
        }

        let decoded = Self { header, prototypes };

        Ok(decoded)
    }
}

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

fn get_uleb128_33<R: Read>(r: &mut R) -> Result<(bool, u32)> {
    let first_byte: u32 = read_u8(r)?.into();

    let is_number_bit = first_byte & 0x1;
    let mut value: u32 = first_byte >> 1u32;

    if value >= 0x40 {
        let mut bit_shift: i8 = -1;
        value &= 0x3F;

        loop {
            let byte: u32 = read_u8(r)?.into();

            bit_shift += 7;
            value |= (byte & 0x7F) << bit_shift;
            if byte < 0x80 {
                break;
            }
        }
    }

    Ok((is_number_bit != 0, value))
}
