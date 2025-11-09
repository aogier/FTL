//! Mappature query type per diverse versioni FTL
//!
//! L'ordine dell'enum query_type è cambiato tra le versioni FTL,
//! quindi manteniamo mappature esplicite per ogni versione nota.

use crate::api::stats::QueryType;

/// Mappatura query types per FTL 6.3+ (struct size >= 344 bytes)
pub const FTL_6_3_TYPE_MAP: &[(usize, QueryType)] = &[
    (1, QueryType::A),
    (2, QueryType::AAAA),
    (3, QueryType::ANY),
    (4, QueryType::SRV),
    (5, QueryType::SOA),
    (6, QueryType::PTR),
    (7, QueryType::TXT),
    (8, QueryType::NAPTR),
    (9, QueryType::MX),
    (10, QueryType::DS),
    (11, QueryType::RRSIG),
    (12, QueryType::DNSKEY),
    (13, QueryType::NS),
    // 14 è TYPE_OTHER, lo skippiamo
    (15, QueryType::SVCB),
    (16, QueryType::HTTPS),
];

/// Mappatura query types per FTL 6.2 (struct size < 344 bytes)
/// In questa versione, MX e SOA erano in posizioni diverse
pub const FTL_6_2_TYPE_MAP: &[(usize, QueryType)] = &[
    (1, QueryType::A),
    (2, QueryType::AAAA),
    (3, QueryType::ANY),
    (4, QueryType::SRV),
    (5, QueryType::MX),     // Diverso da 6.3!
    (6, QueryType::SOA),    // Diverso da 6.3!
    (7, QueryType::PTR),    // Shiftato rispetto a 6.3
    (8, QueryType::TXT),    // Shiftato rispetto a 6.3
    (9, QueryType::NAPTR),  // Shiftato rispetto a 6.3
    (10, QueryType::DS),
    (11, QueryType::RRSIG),
    (12, QueryType::DNSKEY),
    (13, QueryType::NS),
    // TYPE_OTHER sarebbe 14
    (15, QueryType::SVCB),
    (16, QueryType::HTTPS),
];
