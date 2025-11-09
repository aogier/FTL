use crate::api::stats::ClientStats;
use crate::error::Result;
use crate::raw;
use crate::shmem::reader::ShmemReader;
use std::time::{SystemTime, UNIX_EPOCH};

impl ShmemReader {
    /// Ottieni i top N client per numero di query
    pub fn top_clients(&self, limit: usize) -> Vec<ClientStats> {
        let clients = self.clients_array();
        let mut client_stats: Vec<ClientStats> = clients
            .iter()
            .filter_map(|raw_client| self.convert_client(raw_client).ok())
            .collect();

        // Ordina per count decrescente
        client_stats.sort_by(|a, b| b.count.cmp(&a.count));
        client_stats.truncate(limit);
        client_stats
    }

    /// Converti raw clientsData a ClientStats
    fn convert_client(&self, raw_client: &raw::clientsData) -> Result<ClientStats> {
        // Leggi IP dalla string table
        let ip = self.strings.read_string(raw_client.ippos as usize)?;

        // Leggi nome se disponibile
        let name = if raw_client.namepos > 0 {
            self.strings
                .read_string(raw_client.namepos as usize)
                .ok()
                .map(String::from)
        } else {
            None
        };

        // Leggi MAC address dall'hwaddr se disponibile
        let mac = if raw_client.hwlen > 0 {
            // Converti hwaddr a stringa MAC formato standard
            let hwlen = raw_client.hwlen as usize;
            if hwlen <= raw_client.hwaddr.len() {
                let mac_bytes = &raw_client.hwaddr[..hwlen];
                let mac_str = mac_bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(":");
                Some(mac_str)
            } else {
                None
            }
        } else {
            None
        };

        // Converti timestamp (lastQuery è f64)
        let last_query = UNIX_EPOCH + std::time::Duration::from_secs_f64(raw_client.lastQuery);

        Ok(ClientStats {
            ip: ip.to_string(),
            name,
            mac,
            count: raw_client.count.max(0) as u32,
            blocked_count: raw_client.blockedcount.max(0) as u32,
            last_query,
        })
    }
}
