use std::collections::HashMap;
use std::time::SystemTime;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Enumerazioni Rust-friendly (cleaned up)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum QueryType {
    A,
    AAAA,
    ANY,
    SRV,
    SOA,
    PTR,
    TXT,
    NAPTR,
    MX,
    DS,
    RRSIG,
    DNSKEY,
    NS,
    SVCB,
    HTTPS,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum QueryStatus {
    Unknown,
    Gravity,         // Bloccato da gravity
    Forwarded,       // Inoltrato a upstream
    Cache,           // Risposto da cache
    Regex,           // Bloccato da regex
    Denylist,        // Bloccato da denylist esplicita
    ExternalBlocked, // Bloccato da upstream
    DatabaseBusy,
    SpecialDomain, // Pi-hole internal domain
    CacheStale,    // Cache stale
    InProgress,
    Other(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum ReplyType {
    Unknown,
    NoData,
    NxDomain,
    Cname,
    Ip,
    Domain,
    Rrname,
    Servfail,
    Refused,
    Notimp,
    Other,
    Dnssec,
    None,
    Blob,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum DnssecStatus {
    Unknown,
    Secure,
    Insecure,
    Bogus,
    Abandoned,
}

/// Statistiche generali Pi-hole
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct StatsSummary {
    // Query totals
    pub queries_total: u32,
    pub queries_blocked: u32,
    pub queries_forwarded: u32,
    pub queries_cached: u32,
    pub percent_blocked: f32,

    // Entities
    pub domains_unique: u32,
    pub clients_total: u32,
    pub clients_active: u32,
    pub upstreams_total: u32,

    // Performance
    pub queries_per_second: f32,

    // Distributions
    pub query_types: HashMap<QueryType, u32>,
    pub status_distribution: HashMap<QueryStatus, u32>,
    pub reply_types: HashMap<ReplyType, u32>,

    // Database
    pub gravity_size: u32,
}

/// Singola query DNS
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Query {
    pub id: i32,
    pub timestamp: SystemTime,
    pub query_type: QueryType,
    pub domain: String,
    pub client_ip: String,
    pub client_name: Option<String>,
    pub status: QueryStatus,
    pub reply: ReplyType,
    pub response_time_ms: f64,
    pub upstream: Option<String>,
    pub dnssec: DnssecStatus,
    pub blocked: bool,
    pub cname: Option<String>,
}

/// Statistiche per dominio
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct DomainStats {
    pub domain: String,
    pub count: u32,
    pub blocked_count: u32,
    pub last_query: SystemTime,
}

/// Informazioni client
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ClientStats {
    pub ip: String,
    pub name: Option<String>,
    pub mac: Option<String>,
    pub count: u32,
    pub blocked_count: u32,
    pub last_query: SystemTime,
}

/// Statistiche upstream server
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct UpstreamStats {
    pub ip: String,
    pub name: Option<String>,
    pub port: u16,
    pub count: u32,
    pub failed: u32,
    pub response_time_avg_ms: f64,
    pub last_query: SystemTime,
}

/// Slot temporale overtime (10 minuti)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct OvertimeSlot {
    pub timestamp: SystemTime,
    pub total: u32,
    pub blocked: u32,
    pub cached: u32,
    pub forwarded: u32,
}

/// Statistiche DNS cache
#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct CacheStats {
    pub size: u32,
    pub capacity: u32,
    pub utilization_percent: f32,
}
