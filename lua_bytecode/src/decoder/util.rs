use super::error::{Error, Result};
use std::io::Read;

fn load_block<R: Read, T: Into<usize>>(r: &mut R, size: T) -> Result<Vec<u8>> {
    let size = size.into();

    let mut buf = vec![0; size];
    r.read_exact(&mut buf)?;

    Ok(buf)
}

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Endianness {
    BigEndian,
    LittleEndian,
}

pub(crate) fn get_varying_size_num<R, S, E>(r: &mut R, int_size: S, endianness: E) -> Result<u64>
where
    R: Read,
    S: Into<usize>,
    E: Into<Endianness>,
{
    let int_size = int_size.into();

    if int_size == 1 {
        let byte: u64 = read_u8(r)?.into();
        return Ok(byte);
    }
    let endianness: Endianness = endianness.into();
    let loaded = load_block(r, int_size)?;

    let uint32 = match endianness {
        Endianness::BigEndian => match int_size {
            2 => {
                let loaded_as_array = loaded
                    .try_into()
                    .map_err(|_| Error::ConvertError("Vec<u8>", "[u8; 2]"))?;
                u16::from_be_bytes(loaded_as_array).into()
            }
            4 => {
                let loaded_as_array = loaded
                    .try_into()
                    .map_err(|_| Error::ConvertError("Vec<u8>", "[u8;4]"))?;
                u32::from_be_bytes(loaded_as_array).into()
            }
            8 => {
                let loaded_as_array = loaded
                    .try_into()
                    .map_err(|_| Error::ConvertError("Vec<u8>", "[u8;4]"))?;
                u64::from_be_bytes(loaded_as_array)
            }
            _ => unreachable!(),
        },
        Endianness::LittleEndian => match int_size {
            2 => {
                let loaded_as_array = loaded
                    .try_into()
                    .map_err(|_| Error::ConvertError("Vec<u8>", "[u8; 2]"))?;
                let val = u16::from_le_bytes(loaded_as_array);
                val.into()
            }
            4 => {
                let loaded_as_array = loaded
                    .try_into()
                    .map_err(|_| Error::ConvertError("Vec<u8>", "[u8;4]"))?;
                u32::from_le_bytes(loaded_as_array).into()
            }
            8 => {
                let loaded_as_array = loaded
                    .try_into()
                    .map_err(|_| Error::ConvertError("Vec<u8>", "[u8;4]"))?;
                u64::from_le_bytes(loaded_as_array)
            }
            _ => unreachable!("{int_size}"),
        },
    };
    Ok(uint32)
}
