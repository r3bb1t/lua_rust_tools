use super::error::{Error, Result};
use std::io::Read;

pub(crate) fn read_u8<R: Read>(r: &mut R) -> Result<u8> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0])
}

pub(crate) fn read_string<R: Read, T: Into<usize>>(r: &mut R, size: T) -> Result<String> {
    let size: usize = size.into();
    let loaded_bytes = load_block(r, size)?;
    let loaded_string = String::from_utf8(loaded_bytes)?;
    Ok(loaded_string)
}

fn load_block<R: Read, T: Into<usize>>(r: &mut R, size: T) -> Result<Vec<u8>> {
    let size = size.into();

    let mut buf = vec![0; size];
    r.read_exact(&mut buf)?;

    Ok(buf)
}
