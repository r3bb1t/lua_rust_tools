use std::io::Read;

use super::{
    header::{HeaderFlags, LuaJitHeader, LuaJitVersion},
    opcodes::{LuaJit20Opcode, LuaJit21Opcode, LuaJitOpcode},
    read_uint, Result,
};

pub enum InstructionPretty {
    FuncV,
    FuncF,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum InstructionOperandsFormat {
    Abc,
    Ad,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InstructionOperands {
    Abc { a: i8, b: i8, c: i8 },
    Ad { a: u8, d: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub(super) enum ArgumentType {
    T_VAR = 0,
    T_DST = 1,

    T_BS = 2,
    T_RBS = 3,

    T_UV = 4,

    T_LIT = 5,
    T_SLIT = 6,

    T_PRI = 7,
    T_NUM = 8,
    T_STR = 9,

    T_TAB = 10,
    T_FUN = 11,
    T_CDT = 12,
    T_JMP = 13,
    //SLOT_FALSE = 30000,
    //SLOT_TRUE = 30001,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LuaJitInstruction {
    pub opcode: LuaJitOpcode,
    pub operands: InstructionOperands,
    pub arg_count: u8,

    // It's here because it will be needed for enconder
    // (i will probably remove it in future)
    raw_instruction: u32,
}

impl LuaJitInstruction {
    pub(crate) fn from_read<R: Read>(
        r: &mut R,
        luajit_header: &LuaJitHeader,
        //protoype_flags: &PrototypeFlags,
        complex_constants_count: u32,
    ) -> Result<Self> {
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

        let (operands, arg_count) =
            get_instruction_opeands_v2(code_word, complex_constants_count, &opcode)?;

        let instruction = Self {
            opcode,
            operands,
            arg_count,

            raw_instruction: code_word,
        };
        Ok(instruction)
    }
}

// FIXME: this function is a mess. Sometimes it decodes correctly, sometimes it don't
fn get_instruction_opeands_v2(
    code_word: u32,
    complex_constants_count: u32,
    opcode: &LuaJitOpcode,
) -> Result<(InstructionOperands, u8)> {
    let args: [Option<ArgumentType>; 3] = opcode.into();
    let args_count = args.clone().into_iter().filter(Option::is_some).count();
    let [a_ty, b_ty, c_ty] = args;

    let instruction_operands_format = InstructionOperandsFormat::from(opcode);

    let operands = match instruction_operands_format {
        InstructionOperandsFormat::Abc => {
            let mut a = (code_word >> 8) & 0xff;
            let mut c = (code_word >> 16) & 0xff;
            let mut b = (code_word >> 24) & 0xff;

            if let Some(a_type) = a_ty {
                a = process_operand(a_type, a, complex_constants_count);
            }
            if let Some(b_type) = b_ty {
                b = process_operand(b_type, b, complex_constants_count);
            }
            if let Some(c_type) = c_ty {
                c = process_operand(c_type, c, complex_constants_count);
            }

            InstructionOperands::Abc {
                a: a.try_into()?,
                b: b.try_into()?,
                c: c.try_into()?,
            }
        }
        InstructionOperandsFormat::Ad => {
            let mut a = (code_word >> 8) & 0xff;
            let mut d = (code_word >> 16) & 0xffff;

            if let Some(a_type) = a_ty {
                a = process_operand(a_type, a, complex_constants_count);
            }
            if let Some(c_type) = c_ty {
                d = process_operand(c_type, d, complex_constants_count);
            }
            InstructionOperands::Ad {
                d: d.try_into()?,
                a: a.try_into()?,
            }
        }
    };

    Ok((operands, args_count.try_into()?))
}

fn process_operand(op_type: ArgumentType, operand: u32, complex_constants_count: u32) -> u32 {
    match op_type {
        ArgumentType::T_STR | ArgumentType::T_TAB | ArgumentType::T_FUN | ArgumentType::T_CDT => {
            complex_constants_count - operand - 1
        }
        ArgumentType::T_JMP => operand - 0x8000,
        _ => operand,
    }
}
