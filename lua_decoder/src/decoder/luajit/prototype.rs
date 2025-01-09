use crate::decoder::{
    luajit::{
        header::{HeaderFlags, LuaJitVersion},
        opcodes::{LuaJit20Opcode, LuaJitOpcode},
        read_uint, read_uleb128,
    },
    util::{self, read_u8},
};

use super::{header::LuaJitHeader, opcodes::LuaJit21Opcode, Error, Result};
use bitflags::bitflags;
use std::{io::Read, usize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Instruction {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Constant {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct DebugInformation {}

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
pub struct LuaJitState {
    size: u32,
    flags: PrototypeFlags,
    arguments_count: u8,
    frame_size: u8,
    up_values_count: u8,
    complex_constants_count: u32,
    numeric_constants_count: u32,
    instructions_count: u32,
    first_line_number: Option<u32>,
    lines_count: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeV2 {
    flags: PrototypeFlags,
    arguments_count: u8,
    frame_size: u8,

    first_line_number: u32,
    lines_count: u32,

    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
    debug_info: Vec<DebugInformation>,
}

enum ConstantType {
    BcdumpKgcChild = 0,
    BcdumpKgcTab = 1,
    BcdumpKgcI64 = 2,
    BcdumpKgcU64 = 3,
    BcdumpKgcComplex = 4,
    BcdumpKgcStr = 5,
}

impl LuaJitState {
    pub fn from_read<R: Read>(r: &mut R, header: &LuaJitHeader) -> Result<Self> {
        let size = read_uleb128(r)?;

        if size == 0 {
            return Err(Error::LuajitInvalidSizeOfChunk);
        };

        let raw_flags = read_u8(r)?;
        let flags = PrototypeFlags::from_bits(raw_flags)
            .ok_or(Error::LuaJitInvalidHeaderFlags(raw_flags.into()))?;

        let arguments_count = util::read_u8(r)?;
        let frame_size = util::read_u8(r)?;
        let up_values_count = util::read_u8(r)?;

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

        dbg!(first_line_number, lines_count);

        let mut instructions = vec![];
        for _ in 0..instructions_count {
            let instruction = Self::read_instructions(r, header, &flags)?;
            instructions.push(instruction);
        }

        let up_values = Self::read_up_values(r, up_values_count)?;

        let complex_constants = Self::read_complex_constants(r, complex_constants_count)?;

        dbg!(up_values);

        todo!()
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
        println!("{opcode:?}");

        Ok(opcode)
    }

    fn read_up_values<R: Read>(r: &mut R, up_values_count: u8) -> Result<Vec<u8>> {
        let mut up_values = Vec::with_capacity(up_values_count.into());

        for _ in 0..up_values_count {
            up_values.push(read_u8(r)?)
        }
        Ok(up_values)
    }

    fn read_complex_constants<R: Read>(r: &mut R, complex_constants_count: u32) -> Result<u8> {
        for _ in 0..complex_constants_count {
            let constant_type_raw = read_uleb128(r)?;
            let constant_type = ConstantType::from(constant_type_raw);

            match constant_type {
                ConstantType::BcdumpKgcChild => todo!(),
                ConstantType::BcdumpKgcTab => todo!(),
                ConstantType::BcdumpKgcI64 => todo!(),
                ConstantType::BcdumpKgcU64 => todo!(),
                ConstantType::BcdumpKgcComplex => todo!(),
                ConstantType::BcdumpKgcStr => {
                    let length = constant_type_raw - ConstantType::BcdumpKgcStr as u32;
                    let string = util::read_string(r, length as usize)?;
                    dbg!(string);
                }
            }
        }
        todo!()
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
