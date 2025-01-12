use super::{get_uleb128_33, read_uleb128, Result};
use crate::decoder::util::{self, read_u8};

use std::io::Read;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Table;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LuajitConstants {
    pub up_value_references: Vec<u8>,
    pub complex_constants: Vec<ComplexConstantValue>,
    pub numeric_constants: Vec<LuaJitNumericConstant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComplexConstantValue {
    String(String),
    Table,
    Child(Box<Self>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuaJitNumericConstant {
    Int(u32),
    Number(u64),
}

/// Private since encoder is not implemented yet
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ConstantTypeRaw {
    Child = 0,
    Tab = 1,
    I64 = 2,
    U64 = 3,
    Complex = 4,
    Str = 5,
}

impl LuajitConstants {
    pub(super) fn from_read<R: Read>(
        r: &mut R,
        up_values_count: u8,
        complex_constants_count: u32,
        numeric_constants_count: u32,
    ) -> Result<Self> {
        let mut up_value_references = Vec::with_capacity(up_values_count.into());
        for _ in 0..up_values_count {
            up_value_references.push(read_u8(r)?)
        }

        let mut complex_constants = Vec::with_capacity(complex_constants_count as _);
        for _ in 0..complex_constants_count {
            let complex_constant = ComplexConstantValue::from_read(r)?;
            complex_constants.push(complex_constant);
        }

        let mut numeric_constants = Vec::new();
        for _ in 0..numeric_constants_count {
            let numeric_constant = LuaJitNumericConstant::from_read(r)?;
            numeric_constants.push(numeric_constant);
        }

        let constants = Self {
            up_value_references,
            complex_constants,
            numeric_constants,
        };
        Ok(constants)
    }
}

impl ComplexConstantValue {
    fn from_read<R: Read>(r: &mut R) -> Result<Self> {
        let constant_type_raw = read_uleb128(r)?;
        let constant_type = ConstantTypeRaw::from(constant_type_raw);

        let complex_constant_value = match constant_type {
            ConstantTypeRaw::Child => todo!(),
            ConstantTypeRaw::Tab => todo!(),
            ConstantTypeRaw::I64 => todo!(),
            ConstantTypeRaw::U64 => todo!(),
            ConstantTypeRaw::Complex => todo!(),
            ConstantTypeRaw::Str => {
                let length = constant_type_raw - ConstantTypeRaw::Str as u32;
                let string = util::read_string(r, length as usize)?;
                ComplexConstantValue::String(string)
            }
        };

        Ok(complex_constant_value)
    }
}

impl LuaJitNumericConstant {
    fn from_read<R: Read>(r: &mut R) -> Result<Self> {
        let (is_num, lo) = get_uleb128_33(r)?;

        let bc_k_num = if is_num {
            //let hi: u64 = read_uleb128(r)?.into();
            let hi: u64 = lo.into();

            let uleb128: u64 = read_uleb128(r)?.into();
            let number = hi | uleb128 << 32;

            Self::Number(number)
        } else {
            Self::Int(get_uleb128_33(r)?.1)
        };

        Ok(bc_k_num)
    }
}

impl From<u32> for ConstantTypeRaw {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Child,
            1 => Self::Tab,
            2 => Self::I64,
            3 => Self::U64,
            4 => Self::Complex,
            _ => Self::Str,
        }
    }
}
