use super::{read_uleb128, util::read_u8, Error, Result};
use crate::decoder::util::{self, Endianness};

use bitflags::bitflags;
use std::io::Read;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LuaJitHeader {
    pub version: LuaJitVersion,
    pub flags: HeaderFlags,
    pub chunk_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuaJitVersion {
    LuaJit2_0,
    LuaJit2_1,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct HeaderFlags: u32 {

    /// Is big endian ?
    const BCDUMP_F_BE = 0b00000001;
    /// Is stripped ?
    const BCDUMP_F_STRIP = 0b00000010;
    /// Is ffi supported ?
    const BCDUMP_F_FFI = 0b00000100;
    const BCDUMP_F_FR2 = 0b00001000;
    const BCDUMP_F_KNOWN	= 0b00001000 * 2 - 1;
    const BCDUMP_F_DETERMINISTIC	= 0x80000000;
    }
}

impl LuaJitHeader {
    pub fn from_read<R: Read>(r: &mut R) -> Result<Self> {
        Self::check_header(r)?;

        let version_raw = read_u8(r)?;
        let version = LuaJitVersion::try_from(version_raw)?;
        let flags_raw = read_uleb128(r)?;

        let flags =
            HeaderFlags::from_bits(flags_raw).ok_or(Error::LuaJitInvalidHeaderFlags(flags_raw))?;

        let chunk_name = Self::read_name(r, &flags)?;

        let header = Self {
            version,
            flags,
            chunk_name,
        };

        Ok(header)
    }

    fn check_header<R: Read>(r: &mut R) -> Result<()> {
        const LUAJIT_MAGIC: &[u8; 3] = b"\x1bLJ";
        let mut buf = [0u8; 3];

        r.read_exact(&mut buf)?;

        if &buf != LUAJIT_MAGIC {
            Err(Error::WrongHeader)
        } else {
            Ok(())
        }
    }

    fn read_name<R: Read>(r: &mut R, header_flags: &HeaderFlags) -> Result<Option<String>> {
        if header_flags.contains(HeaderFlags::BCDUMP_F_STRIP) {
            Ok(None)
        } else {
            let len = read_uleb128(r)?;
            let as_string = util::read_string(r, len as usize)?;
            Ok(Some(as_string))
        }
    }
}

impl From<&HeaderFlags> for Endianness {
    fn from(val: &HeaderFlags) -> Self {
        match val.contains(HeaderFlags::BCDUMP_F_BE) {
            true => Endianness::BigEndian,
            false => Endianness::LittleEndian,
        }
    }
}

impl TryFrom<u8> for LuaJitVersion {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::LuaJit2_0),
            2 => Ok(Self::LuaJit2_1),
            _ => Err(Error::LuaJitInvalidVersion(value)),
        }
    }
}
