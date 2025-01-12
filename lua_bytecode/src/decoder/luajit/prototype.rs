use super::{
    constants::LuajitConstants,
    debuginfo::DebugInformation,
    header::{HeaderFlags, LuaJitHeader, LuaJitVersion},
    opcodes::{LuaJit20Opcode, LuaJit21Opcode, LuaJitOpcode},
    read_uint, read_uleb128, Error, Result,
};
use crate::decoder::util::{read_u8, Endianness};

use bitflags::bitflags;
use std::io::Read;

bitflags! {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct PrototypeFlags: u8 {
        const FLAG_HAS_CHILD = 0b00000001;
        const FLAG_IS_VARIADIC = 0b00000010;
        const FLAG_HAS_FFI = 0b00000100;
        const FLAG_JIT_DISABLED = 0b00001000;
        const FLAG_HAS_ILOOP = 0b00010000;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LuaJitPrototype {
    flags: PrototypeFlags,
    arguments_count: u8,
    frame_size: u8,

    first_line_number: Option<u32>,
    lines_count: Option<u32>,

    //instructions: Vec<Instruction>,
    instructions: Vec<LuaJitOpcode>,
    constants: LuajitConstants,
    debug_info: DebugInformation,
}

enum ConstantType {
    BcdumpKgcChild = 0,
    BcdumpKgcTab = 1,
    BcdumpKgcI64 = 2,
    BcdumpKgcU64 = 3,
    BcdumpKgcComplex = 4,
    BcdumpKgcStr = 5,
}

impl LuaJitPrototype {
    pub fn from_read<R: Read>(r: &mut R, header: &LuaJitHeader) -> Result<Self> {
        let size = read_uleb128(r)?;

        if size == 0 {
            return Err(Error::LuajitInvalidSizeOfChunk);
        };

        let raw_flags = read_u8(r)?;
        //let flags = PrototypeFlags::from_bits(raw_flags)
        //    .ok_or(Error::LuaJitInvalidHeaderFlags(raw_flags.into()))?;
        let flags = PrototypeFlags::from_bits_retain(raw_flags);

        let arguments_count = read_u8(r)?;
        let frame_size = read_u8(r)?;
        let up_values_count = read_u8(r)?;

        let complex_constants_count = read_uleb128(r)?;
        let numeric_constants_count = read_uleb128(r)?;
        let instructions_count = read_uleb128(r)?;

        let debug_info_size = if header.flags.contains(HeaderFlags::BCDUMP_F_STRIP) {
            0
        } else {
            read_uleb128(r)?
        };

        let first_line_number;
        let lines_count;

        if debug_info_size != 0 {
            first_line_number = Some(read_uleb128(r)?);
            lines_count = Some(read_uleb128(r)?);
        } else {
            first_line_number = None;
            lines_count = None;
        }

        let mut instructions = vec![];
        for _ in 0..instructions_count {
            let instruction = Self::read_instructions(r, header, &flags)?;
            instructions.push(instruction);
        }

        let constants = LuajitConstants::from_read(
            r,
            up_values_count,
            complex_constants_count,
            numeric_constants_count,
        )?;

        //let numeric_constants = BcKnum::read_numeric_constants(r, numeric_constants_count)?;

        let endianness = match header.flags.contains(HeaderFlags::BCDUMP_F_BE) {
            true => Endianness::BigEndian,
            false => Endianness::LittleEndian,
        };
        let debug_info = DebugInformation::from_read(
            r,
            first_line_number.unwrap_or(0),
            &header.flags,
            instructions_count,
            constants.up_value_references.len(),
            &endianness,
        )?;

        let s = Self {
            flags,
            arguments_count,
            frame_size,
            first_line_number,
            lines_count,
            instructions,
            constants,
            debug_info,
        };

        //println!("{:?}", numeric_constants);

        Ok(s)
    }

    fn read_instructions<R: Read>(
        r: &mut R,
        luajit_header: &LuaJitHeader,
        protoype_flags: &PrototypeFlags,
        //) -> Result<Vec<Instruction>> {
    ) -> Result<LuaJitOpcode> {
        //let header = if protoype_flags.contains(PrototypeFlags::FLAG_IS_VARIADIC) {
        //    InstructionPretty::FuncV
        //} else {
        //    InstructionPretty::FuncF
        //};

        let code_word = read_uint(r, luajit_header.flags.contains(HeaderFlags::BCDUMP_F_BE))?;

        let opcode_raw = code_word & 0xff;

        let opcode = match luajit_header.version {
            LuaJitVersion::LuaJit2_0 => LuaJitOpcode::Lj20(LuaJit20Opcode::try_from(opcode_raw)?),
            LuaJitVersion::LuaJit2_1 => LuaJitOpcode::Lj21(LuaJit21Opcode::try_from(opcode_raw)?),
        };

        Ok(opcode)
    }
}

impl From<u32> for ConstantType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::BcdumpKgcChild,
            1 => Self::BcdumpKgcTab,
            2 => Self::BcdumpKgcI64,
            3 => Self::BcdumpKgcU64,
            4 => Self::BcdumpKgcComplex,
            _ => Self::BcdumpKgcStr,
        }
    }
}
