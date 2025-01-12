use lua_bytecode::decoder::{error::Result, luajit::DecodedLuaJitBytecode};

use std::fs::File;

fn main() -> Result<()> {
    let compiled_filename = "lua_bytecode/examples/files/compiled_1";
    let open_file = File::open(compiled_filename)?;

    let decoded = DecodedLuaJitBytecode::from_read(open_file)?;
    println!("{decoded:#?}");

    Ok(())
}
