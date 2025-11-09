use crate::api::stats::DomainStats;
use crate::error::Result;
use crate::raw;
use crate::shmem::reader::ShmemReader;
use std::time::{SystemTime, UNIX_EPOCH};

impl ShmemReader {
    /// Ottieni i top N domini per numero di query
    pub fn top_domains(&self, limit: usize, include_blocked: bool) -> Vec<DomainStats> {
        let domains = self.domains_array();
        let mut domain_stats: Vec<DomainStats> = domains
            .iter()
            .filter(|d| d.magic == raw::MAGICBYTE as u8) // Solo domini validi
            .filter_map(|raw_domain| self.convert_domain(raw_domain).ok())
            .filter(|ds| include_blocked || ds.blocked_count == 0)
            .collect();

        // Ordina per count decrescente
        domain_stats.sort_by(|a, b| b.count.cmp(&a.count));
        domain_stats.truncate(limit);
        domain_stats
    }

    /// Converti raw domainsData a DomainStats
    fn convert_domain(&self, raw_domain: &raw::domainsData) -> Result<DomainStats> {
        // Leggi nome dominio dalla string table
        let domain = self.strings.read_string(raw_domain.domainpos as usize)?;

        // Converti timestamp (lastQuery è f64)
        let last_query = UNIX_EPOCH + std::time::Duration::from_secs_f64(raw_domain.lastQuery);

        Ok(DomainStats {
            domain: domain.to_string(),
            count: raw_domain.count.max(0) as u32,
            blocked_count: raw_domain.blockedcount.max(0) as u32,
            last_query,
        })
    }
}
