use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Wrong header")]
    WrongHeader,

    #[error("Luajit: wrong flags in header: {0:#b}")]
    LuaJitInvalidHeaderFlags(u32),

    #[error("Luajit wrong flags in prototype: {0:#b}")]
    LuaJitInvalidPrototypeFlags(u32),

    #[error("Luajit: Invalid/Unknown version: {0:#x}")]
    LuaJitInvalidVersion(u8),

    #[error("Luajit: Invalid constant type: {0:#x}")]
    LuaJitInvalidConstantType(u32),

    #[error("Invalid header signature: {0:#x}")]
    LuaJitInvalidSignature(u8),

    #[error("Invalid opcode number: {0:#x}")]
    LuaJitInvalidOpcodeNumber(u32),

    #[error("Ivalid debug variable type: {0}")]
    LuaJitInvalidDebugVariableType(u8),

    #[error("An error occured while converting from {0} to {1}")]
    ConvertError(&'static str, &'static str),

    // Froms
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    FromBytesWithNulError(#[from] std::ffi::FromBytesWithNulError),

    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),

    #[error(transparent)]
    TryFromIntError(#[from] core::num::TryFromIntError),
}
