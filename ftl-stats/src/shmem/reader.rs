use crate::error::{FtlError, Result};
use crate::raw;
use crate::shmem::strings::StringBuffer;
use crate::shmem::version::FtlVersion;
use crate::shmem::{open_segment, ShmSegment};
use memmap2::Mmap;
use std::path::{Path, PathBuf};

/// Configurazione per ShmemReader
pub struct ShmemConfig {
    pub pid: u32,
    pub shm_path: PathBuf,
}

impl ShmemConfig {
    /// Auto-discover FTL PID e usa /dev/shm (o $FTL_SHM_PATH se impostato)
    pub fn auto_discover() -> Result<Self> {
        let pid = crate::shmem::pid::find_ftl_pid()?;
        let shm_path = std::env::var("FTL_SHM_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/dev/shm"));

        Ok(Self { pid, shm_path })
    }

    /// Usa PID specifico e /dev/shm (o $FTL_SHM_PATH se impostato)
    pub fn with_pid(pid: u32) -> Self {
        let shm_path = std::env::var("FTL_SHM_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/dev/shm"));

        Self { pid, shm_path }
    }

    /// Sovrascrivi il path di shared memory
    pub fn with_shm_path(mut self, path: impl AsRef<Path>) -> Self {
        self.shm_path = path.as_ref().to_path_buf();
        self
    }
}

/// Reader per tutti i segmenti di shared memory
pub struct ShmemReader {
    pub pid: u32,
    pub shm_path: PathBuf,
    pub version: FtlVersion,

    // Segmenti mappati
    pub settings: Mmap,
    pub counters: Mmap,
    pub queries: Mmap,
    pub clients: Mmap,
    pub domains: Mmap,
    pub upstreams: Mmap,
    pub overtime: Mmap,
    pub dns_cache: Mmap,
    pub strings: StringBuffer,
}

impl ShmemReader {
    pub fn new(config: ShmemConfig) -> Result<Self> {
        let pid = config.pid;
        let shm_path = config.shm_path;

        // Apri settings per primo (contiene metadata e versione)
        let settings_mmap = open_segment(pid, ShmSegment::Settings, &shm_path)?;

        // Leggi e valida versione
        let settings_ref = Self::read_settings(&settings_mmap)?;
        let version = FtlVersion::from_settings(settings_ref.version, settings_ref.pid);

        if !version.is_supported() {
            return Err(FtlError::VersionMismatch {
                expected: raw::SHARED_MEMORY_VERSION,
                found: version.shmem_version,
            });
        }

        // Log versione rilevata (può essere utile per debugging)
        eprintln!("FTL Stats: Detected {}", version.description());

        // Apri counters (serve per dimensioni array)
        let counters_mmap = open_segment(pid, ShmSegment::Counters, &shm_path)?;
        let counters = Self::read_counters(&counters_mmap)?;

        // Apri strings
        let strings_mmap = open_segment(pid, ShmSegment::Strings, &shm_path)?;
        let strings = StringBuffer::new(strings_mmap, counters.strings_MAX as usize);

        // Apri altri segmenti
        let queries_mmap = open_segment(pid, ShmSegment::Queries, &shm_path)?;
        let clients_mmap = open_segment(pid, ShmSegment::Clients, &shm_path)?;
        let domains_mmap = open_segment(pid, ShmSegment::Domains, &shm_path)?;
        let upstreams_mmap = open_segment(pid, ShmSegment::Upstreams, &shm_path)?;
        let overtime_mmap = open_segment(pid, ShmSegment::OverTime, &shm_path)?;
        let dns_cache_mmap = open_segment(pid, ShmSegment::DnsCache, &shm_path)?;

        Ok(Self {
            pid,
            shm_path,
            version,
            settings: settings_mmap,
            counters: counters_mmap,
            queries: queries_mmap,
            clients: clients_mmap,
            domains: domains_mmap,
            upstreams: upstreams_mmap,
            overtime: overtime_mmap,
            dns_cache: dns_cache_mmap,
            strings,
        })
    }

    /// Leggi ShmSettings in modo sicuro
    fn read_settings(mmap: &Mmap) -> Result<&raw::ShmSettings> {
        if mmap.len() < std::mem::size_of::<raw::ShmSettings>() {
            return Err(FtlError::ShmemTooSmall {
                expected: std::mem::size_of::<raw::ShmSettings>(),
                actual: mmap.len(),
            });
        }

        // Safety: abbiamo verificato la dimensione
        let settings = unsafe { &*(mmap.as_ptr() as *const raw::ShmSettings) };
        Ok(settings)
    }

    /// Leggi countersStruct in modo sicuro
    fn read_counters(mmap: &Mmap) -> Result<&raw::countersStruct> {
        if mmap.len() < std::mem::size_of::<raw::countersStruct>() {
            // Se il file è più piccolo della struct, usa la dimensione del file
            // Questo permette compatibilità con versioni dove countersStruct è più piccola
            eprintln!(
                "Warning: counters file size ({}) is smaller than expected struct size ({})",
                mmap.len(),
                std::mem::size_of::<raw::countersStruct>()
            );
        }

        // Safety: accediamo solo fino alla dimensione del file
        let counters = unsafe { &*(mmap.as_ptr() as *const raw::countersStruct) };
        Ok(counters)
    }

    /// Ottieni riferimento a ShmSettings
    pub fn settings(&self) -> &raw::ShmSettings {
        // Safety: abbiamo già validato in new()
        unsafe { &*(self.settings.as_ptr() as *const raw::ShmSettings) }
    }

    /// Ottieni riferimento a countersStruct
    pub fn counters(&self) -> &raw::countersStruct {
        // Safety: abbiamo già validato in new()
        unsafe { &*(self.counters.as_ptr() as *const raw::countersStruct) }
    }

    /// Ottieni array di queries
    pub fn queries_array(&self) -> &[raw::queriesData] {
        let counters = self.counters();
        let max_queries = counters.queries_MAX as usize;
        let ptr = self.queries.as_ptr() as *const raw::queriesData;

        // Safety: FTL ha allocato esattamente questo numero di elementi
        unsafe { std::slice::from_raw_parts(ptr, max_queries) }
    }

    /// Ottieni array di clients
    pub fn clients_array(&self) -> &[raw::clientsData] {
        let counters = self.counters();
        let max_clients = counters.clients_MAX as usize;
        let ptr = self.clients.as_ptr() as *const raw::clientsData;

        unsafe { std::slice::from_raw_parts(ptr, max_clients) }
    }

    /// Ottieni array di domini
    pub fn domains_array(&self) -> &[raw::domainsData] {
        let counters = self.counters();
        let max_domains = counters.domains_MAX as usize;
        let ptr = self.domains.as_ptr() as *const raw::domainsData;

        unsafe { std::slice::from_raw_parts(ptr, max_domains) }
    }

    /// Ottieni array di upstreams
    pub fn upstreams_array(&self) -> &[raw::upstreamsData] {
        let counters = self.counters();
        let max_upstreams = counters.upstreams_MAX as usize;
        let ptr = self.upstreams.as_ptr() as *const raw::upstreamsData;

        unsafe { std::slice::from_raw_parts(ptr, max_upstreams) }
    }

    /// Ottieni array overtime
    pub fn overtime_array(&self) -> &[raw::overTimeData] {
        // Calcola quanti slot overtime ci sono in base alla dimensione del file
        let slot_size = std::mem::size_of::<raw::overTimeData>();
        let num_slots = self.overtime.len() / slot_size;
        let ptr = self.overtime.as_ptr() as *const raw::overTimeData;

        unsafe { std::slice::from_raw_parts(ptr, num_slots) }
    }

    /// Ottieni versione rilevata
    pub fn ftl_version(&self) -> &FtlVersion {
        &self.version
    }
}
