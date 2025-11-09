use crate::api::stats::{DnssecStatus, Query, QueryStatus, QueryType, ReplyType};
use crate::convert::constants::*;
use crate::convert::{timestamp_to_systime, validate_magic};
use crate::error::Result;
use crate::raw;
use crate::shmem::reader::ShmemReader;

impl ShmemReader {
    /// Ottieni query recenti (ultime N)
    pub fn recent_queries(&self, limit: usize) -> Vec<Query> {
        let queries = self.queries_array();

        queries
            .iter()
            .rev() // Più recenti per prime
            .filter(|q| q.magic == raw::MAGICBYTE as u8)
            .take(limit)
            .filter_map(|q| self.convert_query(q).ok())
            .collect()
    }

    /// Converti raw query a Query ergonomica
    fn convert_query(&self, raw_query: &raw::queriesData) -> Result<Query> {
        validate_magic(raw_query.magic)?;

        // Risolvi domain name
        let domain = self.resolve_domain_name(raw_query.domainID)?;

        // Risolvi client info
        let (client_ip, client_name) = self.resolve_client_info(raw_query.clientID)?;

        // Risolvi upstream (se presente)
        let upstream = if raw_query.upstreamID >= 0 {
            self.resolve_upstream_name(raw_query.upstreamID as u32)
                .ok()
        } else {
            None
        };

        // CNAME resolution
        let cname = if raw_query.CNAME_domainID >= 0 {
            Some(self.resolve_domain_name(raw_query.CNAME_domainID as u32)?)
        } else {
            None
        };

        // Gli enum bindgen sono Clone ma non Copy, quindi cloniamo e convertiamo
        let type_val = raw_query.type_.clone() as u32;
        let status_val = raw_query.status.clone() as u32;
        let reply_val = raw_query.reply.clone() as u32;
        let dnssec_val = raw_query.dnssec.clone() as u32;

        Ok(Query {
            id: raw_query.id,
            timestamp: timestamp_to_systime(raw_query.timestamp),
            query_type: Self::convert_query_type(type_val),
            domain,
            client_ip,
            client_name,
            status: Self::convert_query_status(status_val),
            reply: Self::convert_reply_type(reply_val),
            response_time_ms: raw_query.response * 1000.0, // sec → ms
            upstream,
            dnssec: Self::convert_dnssec_status(dnssec_val),
            blocked: Self::is_blocked(status_val),
            cname,
        })
    }

    pub(crate) fn resolve_domain_name(&self, domain_id: u32) -> Result<String> {
        let domains = self.domains_array();
        let domain = domains
            .get(domain_id as usize)
            .ok_or(crate::error::FtlError::OutOfBounds {
                index: domain_id as usize,
                max: domains.len(),
            })?;

        let domain_str = self.strings.read_string(domain.domainpos)?;
        Ok(domain_str.to_string())
    }

    pub(crate) fn resolve_client_info(&self, client_id: u32) -> Result<(String, Option<String>)> {
        let clients = self.clients_array();
        let client = clients
            .get(client_id as usize)
            .ok_or(crate::error::FtlError::OutOfBounds {
                index: client_id as usize,
                max: clients.len(),
            })?;

        let ip = self.strings.read_string(client.ippos)?.to_string();
        let name = if client.namepos > 0 {
            Some(self.strings.read_string(client.namepos)?.to_string())
        } else {
            None
        };

        Ok((ip, name))
    }

    pub(crate) fn resolve_upstream_name(&self, upstream_id: u32) -> Result<String> {
        let upstreams = self.upstreams_array();
        let upstream = upstreams
            .get(upstream_id as usize)
            .ok_or(crate::error::FtlError::OutOfBounds {
                index: upstream_id as usize,
                max: upstreams.len(),
            })?;

        if upstream.ippos > 0 {
            Ok(self.strings.read_string(upstream.ippos)?.to_string())
        } else if upstream.namepos > 0 {
            Ok(self.strings.read_string(upstream.namepos)?.to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    /// Converte query type usando valori hardcoded per stabilità cross-version
    fn convert_query_type(raw_type: u32) -> QueryType {
        match raw_type as usize {
            TYPE_A => QueryType::A,
            TYPE_AAAA => QueryType::AAAA,
            TYPE_ANY => QueryType::ANY,
            TYPE_SRV => QueryType::SRV,
            TYPE_SOA => QueryType::SOA,
            TYPE_PTR => QueryType::PTR,
            TYPE_TXT => QueryType::TXT,
            TYPE_NAPTR => QueryType::NAPTR,
            TYPE_MX => QueryType::MX,
            TYPE_DS => QueryType::DS,
            TYPE_RRSIG => QueryType::RRSIG,
            TYPE_DNSKEY => QueryType::DNSKEY,
            TYPE_NS => QueryType::NS,
            TYPE_SVCB => QueryType::SVCB,
            TYPE_HTTPS => QueryType::HTTPS,
            _ => QueryType::Other(raw_type as u16),
        }
    }

    fn convert_query_status(status: u32) -> QueryStatus {
        crate::convert::raw_status_to_enum(status)
    }

    fn convert_reply_type(reply: u32) -> ReplyType {
        match reply {
            x if x == raw::reply_type::REPLY_NODATA as u32 => ReplyType::NoData,
            x if x == raw::reply_type::REPLY_NXDOMAIN as u32 => ReplyType::NxDomain,
            x if x == raw::reply_type::REPLY_CNAME as u32 => ReplyType::Cname,
            x if x == raw::reply_type::REPLY_IP as u32 => ReplyType::Ip,
            x if x == raw::reply_type::REPLY_DOMAIN as u32 => ReplyType::Domain,
            x if x == raw::reply_type::REPLY_RRNAME as u32 => ReplyType::Rrname,
            x if x == raw::reply_type::REPLY_SERVFAIL as u32 => ReplyType::Servfail,
            x if x == raw::reply_type::REPLY_REFUSED as u32 => ReplyType::Refused,
            x if x == raw::reply_type::REPLY_NOTIMP as u32 => ReplyType::Notimp,
            x if x == raw::reply_type::REPLY_OTHER as u32 => ReplyType::Other,
            x if x == raw::reply_type::REPLY_DNSSEC as u32 => ReplyType::Dnssec,
            x if x == raw::reply_type::REPLY_NONE as u32 => ReplyType::None,
            x if x == raw::reply_type::REPLY_BLOB as u32 => ReplyType::Blob,
            _ => ReplyType::Unknown,
        }
    }

    fn convert_dnssec_status(dnssec: u32) -> DnssecStatus {
        match dnssec {
            x if x == raw::dnssec_status::DNSSEC_SECURE as u32 => DnssecStatus::Secure,
            x if x == raw::dnssec_status::DNSSEC_INSECURE as u32 => DnssecStatus::Insecure,
            x if x == raw::dnssec_status::DNSSEC_BOGUS as u32 => DnssecStatus::Bogus,
            x if x == raw::dnssec_status::DNSSEC_ABANDONED as u32 => DnssecStatus::Abandoned,
            _ => DnssecStatus::Unknown,
        }
    }

    fn is_blocked(status: u32) -> bool {
        matches!(
            status,
            x if x == raw::query_status::QUERY_GRAVITY as u32 ||
                 x == raw::query_status::QUERY_REGEX as u32 ||
                 x == raw::query_status::QUERY_DENYLIST as u32 ||
                 x == raw::query_status::QUERY_EXTERNAL_BLOCKED_IP as u32 ||
                 x == raw::query_status::QUERY_EXTERNAL_BLOCKED_NULL as u32 ||
                 x == raw::query_status::QUERY_EXTERNAL_BLOCKED_NXRA as u32
        )
    }
}
