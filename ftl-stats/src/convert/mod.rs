//! Conversioni da raw bindings a struct ergonomiche

pub mod counters;
pub mod queries;

use crate::error::{FtlError, Result};
use crate::raw;
use std::time::{SystemTime, UNIX_EPOCH};

/// Valida magic byte
pub fn validate_magic(magic: u8) -> Result<()> {
    if magic != raw::MAGICBYTE as u8 {
        return Err(FtlError::InvalidMagicByte {
            expected: raw::MAGICBYTE as u8,
            found: magic,
        });
    }
    Ok(())
}

/// Converte timestamp f64 a SystemTime
pub fn timestamp_to_systime(timestamp: f64) -> SystemTime {
    let duration = std::time::Duration::from_secs_f64(timestamp);
    UNIX_EPOCH + duration
}

/// Converte in_port_t a u16 (network byte order)
pub fn port_to_u16(port: libc::in_port_t) -> u16 {
    u16::from_be(port)
}

/// Converte raw query_status a enum ergonomico
pub fn raw_status_to_enum(status: u32) -> crate::api::stats::QueryStatus {
    use crate::api::stats::QueryStatus;

    match status {
        x if x == raw::query_status::QUERY_GRAVITY as u32 => QueryStatus::Gravity,
        x if x == raw::query_status::QUERY_FORWARDED as u32 => QueryStatus::Forwarded,
        x if x == raw::query_status::QUERY_CACHE as u32 => QueryStatus::Cache,
        x if x == raw::query_status::QUERY_REGEX as u32 => QueryStatus::Regex,
        x if x == raw::query_status::QUERY_DENYLIST as u32 => QueryStatus::Denylist,
        x if x == raw::query_status::QUERY_EXTERNAL_BLOCKED_IP as u32 => {
            QueryStatus::ExternalBlocked
        }
        x if x == raw::query_status::QUERY_EXTERNAL_BLOCKED_NULL as u32 => {
            QueryStatus::ExternalBlocked
        }
        x if x == raw::query_status::QUERY_EXTERNAL_BLOCKED_NXRA as u32 => {
            QueryStatus::ExternalBlocked
        }
        _ => QueryStatus::Other(status),
    }
}
