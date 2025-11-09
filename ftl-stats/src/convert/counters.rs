use crate::api::stats::{QueryStatus, QueryType, ReplyType, StatsSummary};
use crate::convert::constants::{FTL_6_2_TYPE_MAP, FTL_6_3_TYPE_MAP};
use crate::raw;
use crate::shmem::reader::ShmemReader;
use std::collections::HashMap;

impl ShmemReader {
    /// Genera summary statistiche
    pub fn summary(&self) -> StatsSummary {
        let counters = self.counters();
        let settings = self.settings();

        let queries_total = counters.queries;
        let queries_blocked = Self::count_blocked_queries(counters);
        let queries_forwarded = counters
            .status
            .get(raw::query_status::QUERY_FORWARDED as usize)
            .copied()
            .unwrap_or(0);
        let queries_cached = counters
            .status
            .get(raw::query_status::QUERY_CACHE as usize)
            .copied()
            .unwrap_or(0);

        let percent_blocked = if queries_total > 0 {
            (queries_blocked as f32 / queries_total as f32) * 100.0
        } else {
            0.0
        };

        // QPS: media degli ultimi 30 secondi (da settings)
        let qps = settings.qps.iter().sum::<u32>() as f32 / raw::QPS_AVGLEN as f32;

        StatsSummary {
            queries_total,
            queries_blocked,
            queries_forwarded,
            queries_cached,
            percent_blocked,
            domains_unique: counters.domains,
            clients_total: counters.clients,
            clients_active: Self::count_active_clients(self),
            upstreams_total: counters.upstreams,
            queries_per_second: qps,
            query_types: self.extract_query_types_from_queries(),
            status_distribution: Self::extract_status_distribution(counters),
            reply_types: Self::extract_reply_types(counters),
            gravity_size: counters.database.gravity as u32,
        }
    }

    fn count_blocked_queries(counters: &raw::countersStruct) -> u32 {
        let mut blocked = 0;

        // Gravity
        if let Some(&count) = counters
            .status
            .get(raw::query_status::QUERY_GRAVITY as usize)
        {
            blocked += count;
        }

        // Regex
        if let Some(&count) = counters
            .status
            .get(raw::query_status::QUERY_REGEX as usize)
        {
            blocked += count;
        }

        // Denylist
        if let Some(&count) = counters
            .status
            .get(raw::query_status::QUERY_DENYLIST as usize)
        {
            blocked += count;
        }

        // External blocked variations
        if let Some(&count) = counters
            .status
            .get(raw::query_status::QUERY_EXTERNAL_BLOCKED_IP as usize)
        {
            blocked += count;
        }
        if let Some(&count) = counters
            .status
            .get(raw::query_status::QUERY_EXTERNAL_BLOCKED_NULL as usize)
        {
            blocked += count;
        }
        if let Some(&count) = counters
            .status
            .get(raw::query_status::QUERY_EXTERNAL_BLOCKED_NXRA as usize)
        {
            blocked += count;
        }

        blocked
    }

    fn count_active_clients(&self) -> u32 {
        // Client attivi = hanno fatto almeno una query
        self.clients_array()
            .iter()
            .filter(|c| c.magic == raw::MAGICBYTE as u8 && c.count > 0)
            .count() as u32
    }

    /// Estrae distribuzione query types dall'array counters.querytype[]
    /// Usa mappature version-specific per gestire cambiamenti nell'ordine degli enum
    fn extract_query_types_from_queries(&self) -> HashMap<QueryType, u32> {
        let counters = self.counters();

        // Rileva versione FTL dalla size del countersStruct
        let counters_size = std::mem::size_of::<raw::countersStruct>();
        let type_map = if counters_size >= 344 {
            FTL_6_3_TYPE_MAP  // FTL 6.3+
        } else {
            FTL_6_2_TYPE_MAP  // FTL 6.2
        };

        let mut map = HashMap::new();

        // Itera sulla mappatura e legge i count dall'array
        for &(index, query_type) in type_map.iter() {
            if let Some(&count) = counters.querytype.get(index) {
                if count > 0 {
                    map.insert(query_type, count);
                }
            }
        }

        map
    }

    fn extract_status_distribution(counters: &raw::countersStruct) -> HashMap<QueryStatus, u32> {
        let mut map = HashMap::new();

        for (idx, &count) in counters.status.iter().enumerate() {
            if count > 0 {
                let status = crate::convert::raw_status_to_enum(idx as u32);
                map.insert(status, count);
            }
        }

        map
    }

    fn extract_reply_types(counters: &raw::countersStruct) -> HashMap<ReplyType, u32> {
        let mut map = HashMap::new();

        if let Some(&count) = counters
            .reply
            .get(raw::reply_type::REPLY_NODATA as usize)
        {
            if count > 0 {
                map.insert(ReplyType::NoData, count);
            }
        }
        if let Some(&count) = counters
            .reply
            .get(raw::reply_type::REPLY_NXDOMAIN as usize)
        {
            if count > 0 {
                map.insert(ReplyType::NxDomain, count);
            }
        }
        if let Some(&count) = counters
            .reply
            .get(raw::reply_type::REPLY_CNAME as usize)
        {
            if count > 0 {
                map.insert(ReplyType::Cname, count);
            }
        }
        if let Some(&count) = counters.reply.get(raw::reply_type::REPLY_IP as usize) {
            if count > 0 {
                map.insert(ReplyType::Ip, count);
            }
        }

        map
    }
}
