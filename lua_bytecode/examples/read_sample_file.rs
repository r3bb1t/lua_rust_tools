use lua_bytecode::decoder::{error::Result, luajit::DecodedLuaJitBytecode};

fn main() -> Result<()> {
    let raw_file = std::include_bytes!("./files/compiled_1");
    let decoded = DecodedLuaJitBytecode::from_read(&mut &raw_file[..])?;

    println!("{decoded:#?}");
    Ok(())
}
