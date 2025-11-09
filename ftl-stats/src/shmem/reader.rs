use crate::error::{FtlError, Result};
use crate::raw;
use crate::shmem::strings::StringBuffer;
use crate::shmem::{open_segment, validate_segment_size, ShmSegment};
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

        // Apri settings per primo (contiene metadata)
        let settings_mmap = open_segment(pid, ShmSegment::Settings, &shm_path)?;
        // ShmSettings è abbastanza stabile, ma usiamo una dimensione minima per compatibilità
        const MIN_SETTINGS_SIZE: usize = 140;
        validate_segment_size(
            &settings_mmap,
            MIN_SETTINGS_SIZE,
            ShmSegment::Settings,
        )?;

        // Valida versione
        let settings = Self::read_settings(&settings_mmap)?;
        if settings.version != raw::SHARED_MEMORY_VERSION {
            return Err(FtlError::VersionMismatch {
                expected: raw::SHARED_MEMORY_VERSION,
                found: settings.version,
            });
        }

        // Apri counters (serve per dimensioni array)
        let counters_mmap = open_segment(pid, ShmSegment::Counters, &shm_path)?;
        // Note: Non validiamo la dimensione esatta perché countersStruct può variare
        // tra versioni di FTL. Verifichiamo solo che sia abbastanza grande per i campi base.
        // I campi critici sono tutti all'inizio della struct (queries, upstreams, clients, etc.)
        // quindi una dimensione minima di 256 bytes dovrebbe essere sufficiente.
        const MIN_COUNTERS_SIZE: usize = 256;
        validate_segment_size(
            &counters_mmap,
            MIN_COUNTERS_SIZE,
            ShmSegment::Counters,
        )?;

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

    /// Leggi ShmSettings
    fn read_settings(mmap: &Mmap) -> Result<&raw::ShmSettings> {
        // Safety: validato size sopra
        let settings = unsafe { &*(mmap.as_ptr() as *const raw::ShmSettings) };
        Ok(settings)
    }

    /// Leggi countersStruct
    fn read_counters(mmap: &Mmap) -> Result<&raw::countersStruct> {
        // Safety: validato size sopra
        let counters = unsafe { &*(mmap.as_ptr() as *const raw::countersStruct) };
        Ok(counters)
    }

    /// Accesso safe a settings
    pub fn settings(&self) -> &raw::ShmSettings {
        unsafe { &*(self.settings.as_ptr() as *const raw::ShmSettings) }
    }

    /// Accesso safe a counters
    pub fn counters(&self) -> &raw::countersStruct {
        unsafe { &*(self.counters.as_ptr() as *const raw::countersStruct) }
    }

    /// Accesso a array di queries
    pub fn queries_array(&self) -> &[raw::queriesData] {
        let counters = self.counters();
        let count = counters.queries.min(counters.queries_MAX) as usize;

        unsafe {
            std::slice::from_raw_parts(self.queries.as_ptr() as *const raw::queriesData, count)
        }
    }

    /// Accesso a array di clients
    pub fn clients_array(&self) -> &[raw::clientsData] {
        let counters = self.counters();
        let count = counters.clients.min(counters.clients_MAX) as usize;

        unsafe {
            std::slice::from_raw_parts(self.clients.as_ptr() as *const raw::clientsData, count)
        }
    }

    /// Accesso a array di domains
    pub fn domains_array(&self) -> &[raw::domainsData] {
        let counters = self.counters();
        let count = counters.domains.min(counters.domains_MAX) as usize;

        unsafe {
            std::slice::from_raw_parts(self.domains.as_ptr() as *const raw::domainsData, count)
        }
    }

    /// Accesso a array di upstreams
    pub fn upstreams_array(&self) -> &[raw::upstreamsData] {
        let counters = self.counters();
        let count = counters.upstreams.min(counters.upstreams_MAX) as usize;

        unsafe {
            std::slice::from_raw_parts(
                self.upstreams.as_ptr() as *const raw::upstreamsData,
                count,
            )
        }
    }

    /// Accesso a array overtime
    pub fn overtime_array(&self) -> &[raw::overTimeData] {
        unsafe {
            std::slice::from_raw_parts(
                self.overtime.as_ptr() as *const raw::overTimeData,
                raw::OVERTIME_SLOTS as usize,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Richiede FTL in esecuzione
    fn test_shmem_reader_creation() {
        let config = ShmemConfig::auto_discover().unwrap();
        let reader = ShmemReader::new(config).unwrap();

        let settings = reader.settings();
        assert_eq!(settings.version, raw::SHARED_MEMORY_VERSION);

        let counters = reader.counters();
        println!("Total queries: {}", counters.queries);
    }
}
