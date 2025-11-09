/// Informazioni sulla versione di FTL e shared memory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FtlVersion {
    /// Versione del formato shared memory (da ShmSettings.version)
    pub shmem_version: i32,

    /// PID del processo FTL
    pub pid: u32,
}

impl FtlVersion {
    /// Crea FtlVersion dai dati di ShmSettings
    pub fn from_settings(version: i32, pid: i32) -> Self {
        Self {
            shmem_version: version,
            pid: pid as u32,
        }
    }

    /// Controlla se la versione è supportata
    pub fn is_supported(&self) -> bool {
        // Supportiamo solo SHARED_MEMORY_VERSION 14
        self.shmem_version == 14
    }

    /// Descrizione della versione
    pub fn description(&self) -> String {
        format!("FTL (shmem v{}, PID {})", self.shmem_version, self.pid)
    }
}
