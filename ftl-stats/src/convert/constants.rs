//! Valori hardcoded degli enum per garantire compatibilità cross-version
//!
//! Questi valori sono basati su FTL v6.x e rimangono stabili anche quando
//! bindgen genera valori diversi da versioni differenti di FTL.

/// Query type values (basato su enum query_type in FTL)
pub const TYPE_A: usize = 1;
pub const TYPE_AAAA: usize = 2;
pub const TYPE_ANY: usize = 3;
pub const TYPE_SRV: usize = 4;
pub const TYPE_SOA: usize = 5;
pub const TYPE_PTR: usize = 6;
pub const TYPE_TXT: usize = 7;
pub const TYPE_NAPTR: usize = 8;
pub const TYPE_MX: usize = 9;
pub const TYPE_DS: usize = 10;
pub const TYPE_RRSIG: usize = 11;
pub const TYPE_DNSKEY: usize = 12;
pub const TYPE_NS: usize = 13;
pub const TYPE_SVCB: usize = 15;
pub const TYPE_HTTPS: usize = 16;
