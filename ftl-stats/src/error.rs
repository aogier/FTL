use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FtlError {
    #[error("FTL process not found")]
    FtlNotRunning,

    #[error("PID file not found at {0}")]
    PidFileNotFound(PathBuf),

    #[error("Failed to read PID file: {0}")]
    PidFileReadError(#[source] std::io::Error),

    #[error("Invalid PID in file: {0}")]
    InvalidPid(String),

    #[error("Shared memory version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: i32, found: i32 },

    #[error("Invalid magic byte: expected 0x{expected:02x}, found 0x{found:02x}")]
    InvalidMagicByte { expected: u8, found: u8 },

    #[error("Shared memory segment 'FTL-{pid}-{segment}' not found")]
    ShmemNotFound { pid: u32, segment: String },

    #[error("Failed to open shared memory: {0}")]
    ShmemOpenError(#[from] std::io::Error),

    #[error("Shared memory file is too small: expected at least {expected} bytes, found {actual}")]
    ShmemTooSmall { expected: usize, actual: usize },

    #[error("Invalid string offset: {offset} (buffer size: {buffer_size})")]
    InvalidStringOffset { offset: usize, buffer_size: usize },

    #[error("UTF-8 decode error at offset {offset}: {source}")]
    Utf8Error {
        offset: usize,
        #[source]
        source: std::str::Utf8Error,
    },

    #[error("Array index out of bounds: index {index}, max {max}")]
    OutOfBounds { index: usize, max: usize },

    #[error("Invalid array size in counters: {field} is {value}")]
    InvalidArraySize { field: String, value: u32 },
}

pub type Result<T> = std::result::Result<T, FtlError>;
