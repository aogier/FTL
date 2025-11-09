use crate::error::{FtlError, Result};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

pub mod pid;
pub mod reader;
pub mod strings;
pub mod version;

/// Segmenti di shared memory disponibili
#[derive(Debug, Clone, Copy)]
pub enum ShmSegment {
    Lock,
    Strings,
    Counters,
    Domains,
    Clients,
    Queries,
    Upstreams,
    OverTime,
    Settings,
    DnsCache,
    PerClientRegex,
    FifoLog,
    ClientsLookup,
    DomainsLookup,
    DnsCacheLookup,
    Recycler,
}

impl ShmSegment {
    /// Nome del segmento come appare in /dev/shm
    pub fn name(&self) -> &'static str {
        match self {
            Self::Lock => "lock",
            Self::Strings => "strings",
            Self::Counters => "counters",
            Self::Domains => "domains",
            Self::Clients => "clients",
            Self::Queries => "queries",
            Self::Upstreams => "upstreams",
            Self::OverTime => "overTime",
            Self::Settings => "settings",
            Self::DnsCache => "dns-cache",
            Self::PerClientRegex => "per-client-regex",
            Self::FifoLog => "fifo-log",
            Self::ClientsLookup => "clients-lookup",
            Self::DomainsLookup => "domains-lookup",
            Self::DnsCacheLookup => "dns-cache-lookup",
            Self::Recycler => "recycler",
        }
    }
}

/// Apre un segmento di shared memory
pub fn open_segment(
    pid: u32,
    segment: ShmSegment,
    shm_path: impl AsRef<Path>,
) -> Result<Mmap> {
    let filename = format!("FTL-{}-{}", pid, segment.name());
    let full_path = shm_path.as_ref().join(&filename);

    let file = File::open(&full_path).map_err(|_| FtlError::ShmemNotFound {
        pid,
        segment: segment.name().to_string(),
    })?;

    // Safety: mmap è unsafe ma qui è read-only
    let mmap = unsafe { Mmap::map(&file)? };

    Ok(mmap)
}
