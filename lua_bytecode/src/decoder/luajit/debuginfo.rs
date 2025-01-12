use super::{header::HeaderFlags, Error, Result};
use crate::decoder::{
    luajit::read_uleb128,
    util::{self, read_u8, Endianness},
};

use std::io::Read;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct VariableInfo {
    start_addr: u32,
    end_addr: u32,
    variable_visibility_type: VariableVisibility,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum VariableVisibility {
    Visible = 0,
    Internal = 1,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InternalVarType {
    End = 0,
    ForIdx = 1,
    ForStop = 2,
    ForStep = 3,
    ForGen = 4,
    ForState = 5,
    ForCtl = 6,
    Max = 7,
}

impl VariableInfo {
    fn read_variable_infos<R: Read>(r: &mut R, endianness: &Endianness) -> Result<Vec<Self>> {
        const INTERNAL_VAR_NAMES: [Option<&str>; 7] = [
            None,
            Some("<index>"),
            Some("<limit>"),
            Some("<step>"),
            Some("<generator>"),
            Some("<state>"),
            Some("<control>"),
        ];
        let mut last_addr = 0u32;

        let mut variable_infos = vec![];
        loop {
            let internal_var_type = read_u8(r)?;
            let name;
            let variable_visibility_type;

            if internal_var_type >= InternalVarType::Max as u8 {
                let prefix = match endianness {
                    Endianness::BigEndian => internal_var_type.to_le_bytes(),
                    Endianness::LittleEndian => internal_var_type.to_le_bytes(),
                };
                let suffix = read_z_string(r)?;

                // TODO: Find cheaper alternative
                name = String::from_utf8(prefix.to_vec())? + &suffix;
                variable_visibility_type = VariableVisibility::Visible;
            } else if internal_var_type == InternalVarType::End as u8 {
                break;
            } else {
                let index = internal_var_type;
                // TODO: don't unwrap
                name = INTERNAL_VAR_NAMES
                    .get(index as usize)
                    .unwrap()
                    .unwrap()
                    .to_owned();

                variable_visibility_type = VariableVisibility::Internal;
            }

            let start_addr = last_addr + read_uleb128(r)?;
            let end_addr = start_addr + read_uleb128(r)?;

            last_addr = start_addr;
            let s = Self {
                start_addr,
                end_addr,
                name,
                variable_visibility_type,
            };

            variable_infos.push(s);
        }
        Ok(variable_infos)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DebugInformation {
    addr_to_line_map: Vec<u64>,
    upvalue_variables_names: Vec<String>,
    variable_infos: Vec<VariableInfo>,
}

impl DebugInformation {
    pub(crate) fn from_read<R: Read>(
        r: &mut R,
        line_offset: u32,
        header_flags: &HeaderFlags,
        instructions_count: u32,
        up_values_count: usize,
        endianness: &Endianness,
    ) -> Result<Self> {
        let addr_to_line_map =
            Self::read_line_info(r, line_offset, header_flags, instructions_count)?;
        let upvalue_variables_names = Self::read_upvalue_names(r, up_values_count)?;
        let variable_infos = VariableInfo::read_variable_infos(r, endianness)?;

        let s = Self {
            addr_to_line_map,
            upvalue_variables_names,
            variable_infos,
        };

        Ok(s)
    }

    fn read_line_info<R: Read>(
        r: &mut R,
        line_offset: u32,
        header_flags: &HeaderFlags,
        instructions_count: u32,
    ) -> Result<Vec<u64>> {
        let line_offset: u64 = line_offset.into();
        let line_info_size: u8 = if line_offset >= 0x10000 {
            4
        } else if line_offset >= 0x100 {
            2
        } else {
            1
        };

        let mut line_info = Vec::with_capacity(instructions_count as usize);
        for _ in 0..instructions_count {
            // FIXME: Spotted bug here during work with the last successfully decoded prototype
            let line_number = util::get_varying_size_num(r, line_info_size, header_flags)?;
            line_info.push(line_offset + line_number);
        }

        Ok(line_info)
    }

    fn read_upvalue_names<R: Read>(r: &mut R, up_values_count: usize) -> Result<Vec<String>> {
        let mut upvalue_names = Vec::with_capacity(up_values_count);
        for _ in 0..up_values_count {
            let name = read_z_string(r)?;
            upvalue_names.push(name);
        }
        Ok(upvalue_names)
    }
}

impl TryFrom<u8> for InternalVarType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::End),
            1 => Ok(Self::ForIdx),
            2 => Ok(Self::ForStop),
            3 => Ok(Self::ForStep),
            4 => Ok(Self::ForGen),
            5 => Ok(Self::ForState),
            6 => Ok(Self::ForCtl),
            7 => Ok(Self::Max),
            _ => Err(Error::LuaJitInvalidDebugVariableType(value)),
        }
    }
}

fn read_z_string<R: Read>(r: &mut R) -> Result<String> {
    let mut buf = vec![];
    loop {
        let byte = read_u8(r)?;
        if byte == 0 {
            return Ok(String::from_utf8(buf)?);
        } else {
            buf.push(byte);
        }
    }
}
