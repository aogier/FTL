use crate::api::stats::{QueryStatus, QueryType, ReplyType, StatsSummary};
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
            query_types: Self::extract_query_types(counters),
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

    fn extract_query_types(counters: &raw::countersStruct) -> HashMap<QueryType, u32> {
        let mut map = HashMap::new();

        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_A as usize)
        {
            if count > 0 {
                map.insert(QueryType::A, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_AAAA as usize)
        {
            if count > 0 {
                map.insert(QueryType::AAAA, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_PTR as usize)
        {
            if count > 0 {
                map.insert(QueryType::PTR, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_TXT as usize)
        {
            if count > 0 {
                map.insert(QueryType::TXT, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_MX as usize)
        {
            if count > 0 {
                map.insert(QueryType::MX, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_SRV as usize)
        {
            if count > 0 {
                map.insert(QueryType::SRV, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_NAPTR as usize)
        {
            if count > 0 {
                map.insert(QueryType::NAPTR, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_SOA as usize)
        {
            if count > 0 {
                map.insert(QueryType::SOA, count);
            }
        }
        if let Some(&count) = counters
            .querytype
            .get(raw::query_type::TYPE_ANY as usize)
        {
            if count > 0 {
                map.insert(QueryType::ANY, count);
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
