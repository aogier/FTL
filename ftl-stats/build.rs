use std::env;
use std::path::PathBuf;

fn main() {
    // Path al codice sorgente FTL
    // Può essere sovrascritto con env var FTL_SOURCE_PATH
    let ftl_src = env::var("FTL_SOURCE_PATH").unwrap_or_else(|_| {
        // Default: assume FTL sia nella parent directory
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        format!("{}/../src", manifest_dir)
    });

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed={}", ftl_src);

    // Configurazione bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Include path per header FTL
        .clang_arg(format!("-I{}", ftl_src))
        .clang_arg(format!("-I{}/webserver", ftl_src))
        // === Allowlist: genera solo ciò che ci serve ===
        // Strutture dati principali
        .allowlist_type("ShmSettings")
        .allowlist_type("countersStruct")
        .allowlist_type("queriesData")
        .allowlist_type("query_flags")
        .allowlist_type("database_flags")
        .allowlist_type("clientsData")
        .allowlist_type("client_flags")
        .allowlist_type("domainsData")
        .allowlist_type("upstreamsData")
        .allowlist_type("upstream_flags")
        .allowlist_type("overTimeData")
        .allowlist_type("DNSCacheData")
        .allowlist_type("fifologData")
        .allowlist_type("lookup_table")
        .allowlist_type("recycle_table")
        .allowlist_type("recycler_tables")
        // Enumerazioni
        .allowlist_type("query_status")
        .allowlist_type("query_type")
        .allowlist_type("reply_type")
        .allowlist_type("dnssec_status")
        .allowlist_type("privacy_level")
        // Costanti importanti
        .allowlist_var("SHARED_MEMORY_VERSION")
        .allowlist_var("OVERTIME_SLOTS")
        .allowlist_var("OVERTIME_INTERVAL")
        .allowlist_var("MAGICBYTE")
        .allowlist_var("TYPE_MAX")
        .allowlist_var("QUERY_STATUS_MAX")
        .allowlist_var("QUERY_REPLY_MAX")
        .allowlist_var("QPS_AVGLEN")
        // Opzioni di generazione
        .derive_debug(true)
        .derive_default(false)
        .derive_copy(false) // Struct troppo grandi
        .derive_eq(false)
        .impl_debug(true)
        .size_t_is_usize(true)
        .use_core()
        .detect_include_paths(true)
        // Traduci enum in Rust enum (non costanti)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        // Layout test per validare correttezza
        .layout_tests(true)
        .generate()
        .expect("Unable to generate bindings");

    // Scrivi binding generati
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:warning=Bindings generated successfully");
}
