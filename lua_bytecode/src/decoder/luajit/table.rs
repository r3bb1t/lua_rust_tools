use super::Result;

use crate::decoder::{luajit::read_uleb128, util};
use std::io::Read;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuaJitTableItemTy {
    BcdumpKtabNil = 0,
    BcdumpKtabFalse = 1,
    BcdumpKtabTrue = 2,
    BcdumpKtabInt = 3,
    BcdumpKtabNum = 4,
    BcdumpKtabStr = 5,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuaJitTableItem {
    Nil,
    False,
    True,
    Str(String),
    Int(i32),
}

impl LuaJitTableItem {
    fn read_table_item<R: Read>(r: &mut R) -> Result<LuaJitTableItem> {
        let data_type_raw = read_uleb128(r)?;
        let data_type = LuaJitTableItemTy::from(data_type_raw);

        let item = match data_type {
            LuaJitTableItemTy::BcdumpKtabNil => LuaJitTableItem::Nil,
            LuaJitTableItemTy::BcdumpKtabFalse => LuaJitTableItem::False,
            LuaJitTableItemTy::BcdumpKtabTrue => LuaJitTableItem::True,
            LuaJitTableItemTy::BcdumpKtabInt => LuaJitTableItem::Int(read_uleb128(r)? as _),
            LuaJitTableItemTy::BcdumpKtabNum => todo!(),
            LuaJitTableItemTy::BcdumpKtabStr => {
                let length = data_type_raw - 5;
                let string = util::read_string(r, length as usize)?;
                LuaJitTableItem::Str(string)
            }
        };

        Ok(item)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LuaJitTable {
    array_items: Vec<LuaJitTableItem>,
    hash_items: Vec<(LuaJitTableItem, LuaJitTableItem)>,
}

impl LuaJitTable {
    pub fn read_table<R: Read>(r: &mut R) -> Result<Self> {
        let array_items_count = read_uleb128(r)?;
        let hash_items_count = read_uleb128(r)?;

        let mut array_items = Vec::with_capacity(array_items_count as usize);
        for _ in 0..array_items_count {
            let constant = LuaJitTableItem::read_table_item(r)?;
            array_items.push(constant);
        }

        let mut hash_items = Vec::with_capacity(array_items_count as usize);
        for _ in 0..hash_items_count {
            let key = LuaJitTableItem::read_table_item(r)?;
            let value = LuaJitTableItem::read_table_item(r)?;
            hash_items.push((key, value));
        }

        let table = Self {
            array_items,
            hash_items,
        };

        Ok(table)
    }
}

impl From<u32> for LuaJitTableItemTy {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::BcdumpKtabNil,
            1 => Self::BcdumpKtabFalse,
            2 => Self::BcdumpKtabTrue,
            3 => Self::BcdumpKtabInt,
            4 => Self::BcdumpKtabNum,
            _ => Self::BcdumpKtabStr,
        }
    }
}
