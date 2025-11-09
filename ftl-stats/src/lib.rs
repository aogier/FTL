//! # ftl-stats
//!
//! Libreria Rust per leggere statistiche Pi-hole FTL dalla shared memory.
//!
//! ## Quick Start
//!
//! ```no_run
//! use ftl_stats::FtlStats;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let stats = FtlStats::new()?;
//! println!("FTL PID: {}", stats.ftl_pid());
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod error;
pub mod shmem;

mod convert;
mod raw;

pub use api::stats::*;
pub use error::{FtlError, Result};

use shmem::reader::{ShmemConfig, ShmemReader};
use std::path::Path;

/// Interfaccia principale per leggere statistiche FTL
pub struct FtlStats {
    reader: ShmemReader,
}

impl FtlStats {
    /// Crea una nuova istanza con auto-discovery del PID di FTL
    ///
    /// Cerca il PID in `/run/pihole-FTL.pid` e apre tutti i segmenti
    /// di shared memory in `/dev/shm`.
    ///
    /// # Errors
    ///
    /// Ritorna errore se:
    /// - FTL non è in esecuzione
    /// - I file in shared memory non esistono
    /// - La versione della shared memory non è compatibile
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ftl_stats::FtlStats;
    /// let stats = FtlStats::new()?;
    /// # Ok::<(), ftl_stats::FtlError>(())
    /// ```
    pub fn new() -> Result<Self> {
        let config = ShmemConfig::auto_discover()?;
        let reader = ShmemReader::new(config)?;
        Ok(Self { reader })
    }

    /// Crea istanza con PID specifico
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ftl_stats::FtlStats;
    /// let stats = FtlStats::with_pid(12345)?;
    /// # Ok::<(), ftl_stats::FtlError>(())
    /// ```
    pub fn with_pid(pid: u32) -> Result<Self> {
        let config = ShmemConfig::with_pid(pid);
        let reader = ShmemReader::new(config)?;
        Ok(Self { reader })
    }

    /// Builder per configurazione custom
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ftl_stats::FtlStats;
    /// let stats = FtlStats::builder()
    ///     .pid(12345)
    ///     .shm_path("/custom/shm")
    ///     .build()?;
    /// # Ok::<(), ftl_stats::FtlError>(())
    /// ```
    pub fn builder() -> FtlStatsBuilder {
        FtlStatsBuilder::default()
    }

    /// PID del processo FTL
    pub fn ftl_pid(&self) -> u32 {
        self.reader.pid
    }

    /// Ottieni statistiche generali
    ///
    /// Include query totali/bloccate, percentuale blocco, domini unici,
    /// client attivi, QPS, e distribuzioni per tipo/status/reply.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ftl_stats::FtlStats;
    /// # let stats = FtlStats::new()?;
    /// let summary = stats.summary();
    /// println!("Blocked: {:.1}%", summary.percent_blocked);
    /// # Ok::<(), ftl_stats::FtlError>(())
    /// ```
    pub fn summary(&self) -> StatsSummary {
        self.reader.summary()
    }

    /// Query recenti (ultimi N)
    ///
    /// Ritorna le query più recenti in ordine inverso cronologico.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ftl_stats::FtlStats;
    /// # let stats = FtlStats::new()?;
    /// for query in stats.recent_queries(10) {
    ///     println!("{} -> {}", query.client_ip, query.domain);
    /// }
    /// # Ok::<(), ftl_stats::FtlError>(())
    /// ```
    pub fn recent_queries(&self, limit: usize) -> Vec<Query> {
        self.reader.recent_queries(limit)
    }

    /// Accesso raw ai counters (interno, per ora pubblico per testing)
    #[doc(hidden)]
    pub fn raw_counters(&self) -> &raw::countersStruct {
        self.reader.counters()
    }
}

/// Builder per FtlStats con configurazione custom
#[derive(Default)]
pub struct FtlStatsBuilder {
    pid: Option<u32>,
    shm_path: Option<std::path::PathBuf>,
}

impl FtlStatsBuilder {
    pub fn pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }

    pub fn shm_path(mut self, path: impl AsRef<Path>) -> Self {
        self.shm_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build(self) -> Result<FtlStats> {
        let mut config = if let Some(pid) = self.pid {
            ShmemConfig::with_pid(pid)
        } else {
            ShmemConfig::auto_discover()?
        };

        if let Some(path) = self.shm_path {
            config = config.with_shm_path(path);
        }

        let reader = ShmemReader::new(config)?;
        Ok(FtlStats { reader })
    }
}
