use crate::api::stats::UpstreamStats;
use crate::error::Result;
use crate::raw;
use crate::shmem::reader::ShmemReader;
use std::time::{SystemTime, UNIX_EPOCH};

impl ShmemReader {
    /// Ottieni statistiche di tutti gli upstream server
    pub fn upstreams(&self) -> Vec<UpstreamStats> {
        let upstreams = self.upstreams_array();
        upstreams
            .iter()
            .filter(|u| u.magic == raw::MAGICBYTE as u8) // Solo upstream validi
            .filter_map(|raw_upstream| self.convert_upstream(raw_upstream).ok())
            .collect()
    }

    /// Converti raw upstreamsData a UpstreamStats
    fn convert_upstream(&self, raw_upstream: &raw::upstreamsData) -> Result<UpstreamStats> {
        // Leggi IP dalla string table
        let ip = self.strings.read_string(raw_upstream.ippos as usize)?;

        // Leggi nome se disponibile
        let name = if raw_upstream.namepos > 0 {
            self.strings
                .read_string(raw_upstream.namepos as usize)
                .ok()
                .map(String::from)
        } else {
            None
        };

        // Estrai porta
        let port = raw_upstream.port;

        // Response time medio (rtime è già in millisecondi)
        let response_time_avg_ms = raw_upstream.rtime;

        // Converti timestamp (lastQuery è f64)
        let last_query = UNIX_EPOCH + std::time::Duration::from_secs_f64(raw_upstream.lastQuery);

        Ok(UpstreamStats {
            ip: ip.to_string(),
            name,
            port,
            count: raw_upstream.count.max(0) as u32,
            failed: raw_upstream.failed.max(0) as u32,
            response_time_avg_ms,
            last_query,
        })
    }
}
