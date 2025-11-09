//! Raw FFI bindings generati da bindgen.
//!
//! Questo modulo contiene le struct C generate automaticamente.
//! Non usare direttamente - usa invece l'API in `crate::api`.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

// Include il codice generato da bindgen
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Costante non generata da bindgen (definita in shmem.c)
pub const SHARED_MEMORY_VERSION: i32 = 14;

// Re-export per comodità
pub use query_status as QueryStatus;
pub use query_type as QueryType;
pub use reply_type as ReplyType;
pub use dnssec_status as DnssecStatus;
pub use privacy_level as PrivacyLevel;
