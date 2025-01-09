use std::{error::Error, fs::File};

use lua_decoder::decoder::luajit::{header::LuaJitHeader, prototype::LuaJitState};

fn main() -> Result<(), Box<dyn Error>> {
    let compiled_filename = "examples/files/compiled_1";
    let mut open_file = File::open(compiled_filename)?;
    //let mut bytes = std::include_bytes!("./files/compiled_1");

    let header = LuaJitHeader::from_read(&mut open_file)?;

    dbg!(&header);

    //let mut prototypes = vec![];

    //while let Ok(prototype) = LuaJitState::from_read(&mut open_file, &header) {
    //    prototypes.push(prototype);
    //}

    let prototype = LuaJitState::from_read(&mut open_file, &header)?;

    //println!("{prototypes:#x?}");

    Ok(())
}
