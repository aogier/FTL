use crate::error::{FtlError, Result};
use memmap2::Mmap;
use std::ffi::CStr;

/// Reader per il buffer di stringhe condiviso
pub struct StringBuffer {
    mmap: Mmap,
    max_size: usize,
}

impl StringBuffer {
    pub fn new(mmap: Mmap, max_size: usize) -> Self {
        Self { mmap, max_size }
    }

    /// Leggi stringa all'offset specificato
    pub fn read_string(&self, offset: usize) -> Result<&str> {
        if offset >= self.max_size {
            return Err(FtlError::InvalidStringOffset {
                offset,
                buffer_size: self.max_size,
            });
        }

        if offset >= self.mmap.len() {
            return Err(FtlError::InvalidStringOffset {
                offset,
                buffer_size: self.mmap.len(),
            });
        }

        // Trova il null terminator
        let slice = &self.mmap[offset..];
        let null_pos = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());

        // Converti a &str
        std::str::from_utf8(&slice[..null_pos]).map_err(|e| FtlError::Utf8Error {
            offset,
            source: e,
        })
    }

    /// Leggi stringa come CStr (utile per debug)
    pub fn read_cstr(&self, offset: usize) -> Result<&CStr> {
        if offset >= self.max_size {
            return Err(FtlError::InvalidStringOffset {
                offset,
                buffer_size: self.max_size,
            });
        }

        if offset >= self.mmap.len() {
            return Err(FtlError::InvalidStringOffset {
                offset,
                buffer_size: self.mmap.len(),
            });
        }

        // Safety: verifichiamo che offset sia valido
        unsafe {
            let ptr = self.mmap.as_ptr().add(offset);
            Ok(CStr::from_ptr(ptr as *const i8))
        }
    }
}
