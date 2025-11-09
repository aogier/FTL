# ftl-stats

Libreria Rust per leggere statistiche Pi-hole FTL dalla shared memory.

## Status

✅ **Fase 1 e 2 Complete** - MVP Funzionante

- ✅ Fase 1: Setup e bindgen
- ✅ Fase 2: Conversioni ergonomiche e API base
- 🔄 Fase 3: API completa (in sviluppo)
- ⏳ Fase 4: Esempi e test aggiuntivi
- ⏳ Fase 5: Documentazione completa

## Features

- ✅ **Binding automatici** tramite bindgen (sempre sincronizzati con FTL)
- ✅ **Lettura zero-copy** da `/dev/shm` tramite memory mapping
- ✅ **Type-safe** accesso a shared memory
- ✅ **Auto-discovery** del PID di FTL
- ✅ **API ergonomica** - StatsSummary, Query, enums idiomatici
- ✅ **Compatibile container** - supporto PID e path personalizzati

## Quick Start

```rust
use ftl_stats::FtlStats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Auto-discover FTL
    let stats = FtlStats::new()?;

    // Statistiche complete
    let summary = stats.summary();
    println!("Total queries: {}", summary.queries_total);
    println!("Blocked: {} ({:.1}%)",
        summary.queries_blocked,
        summary.percent_blocked);

    // Query recenti
    for query in stats.recent_queries(10) {
        println!("{:?} {} -> {}",
            query.query_type,
            query.client_ip,
            query.domain);
    }

    Ok(())
}
```

## Testing con Pi-hole in Container

### Scenario tipico
Pi-hole gira in un container Docker, la libreria gira sull'host.

### Metodo Rapido: Script Helper

Il modo più semplice per testare:

```bash
# Usa lo script helper (verifica tutto automaticamente)
./test-with-docker.sh pihole summary
./test-with-docker.sh pihole basic

# Oppure con container name diverso
./test-with-docker.sh my-pihole-container summary
```

Lo script:
- ✅ Trova automaticamente il PID di FTL
- ✅ Verifica che i file shmem siano accessibili
- ✅ Esegue l'esempio con i parametri corretti
- ✅ Mostra errori dettagliati se qualcosa non funziona

### Metodo Manuale

#### 1. Trova il PID di FTL nel container

```bash
docker exec pihole pidof pihole-FTL
# oppure
docker exec pihole cat /run/pihole-FTL.pid
```

#### 2. Assicurati che /dev/shm sia condiviso

```yaml
# docker-compose.yml
services:
  pihole:
    volumes:
      - /dev/shm:/dev/shm  # ← Importante!
```

#### 3. Esegui gli esempi

```bash
# Con auto-discovery (se FTL_PID file è accessibile)
cargo run --example summary

# Specificando il PID
cargo run --example summary 12345

# Specificando PID e path custom
cargo run --example summary 12345 /custom/shm
```

## API Reference

### Connessione

```rust
// Auto-discovery (legge /run/pihole-FTL.pid)
let stats = FtlStats::new()?;

// PID specifico
let stats = FtlStats::with_pid(12345)?;

// Builder con configurazione custom
let stats = FtlStats::builder()
    .pid(12345)
    .shm_path("/custom/shm")
    .build()?;
```

### Statistiche

```rust
// Summary completo
let summary = stats.summary();
// → queries_total, queries_blocked, percent_blocked
// → domains_unique, clients_active, queries_per_second
// → query_types: HashMap<QueryType, u32>
// → status_distribution: HashMap<QueryStatus, u32>
// → reply_types: HashMap<ReplyType, u32>

// Query recenti
let queries = stats.recent_queries(100);
// → Vec<Query> con timestamp, type, domain, client, status, reply, etc.

// Top domini (con/senza bloccati)
let top_domains = stats.top_domains(10, false); // Solo allowed
let all_domains = stats.top_domains(10, true);  // Include blocked
// → Vec<DomainStats> con domain, count, blocked_count, last_query

// Top client
let top_clients = stats.top_clients(10);
// → Vec<ClientStats> con ip, name, mac, count, blocked_count, last_query

// Upstream server
let upstreams = stats.upstreams();
// → Vec<UpstreamStats> con ip, port, count, failed, response_time_avg_ms

// Overtime data (serie temporale)
let overtime = stats.overtime_data();
// → Vec<OvertimeSlot> con timestamp, total, blocked, cached, forwarded
```

### Tipi Supportati

- **QueryType**: A, AAAA, PTR, TXT, MX, SRV, SOA, ANY, etc.
- **QueryStatus**: Gravity, Forwarded, Cache, Regex, Denylist, etc.
- **ReplyType**: NoData, NxDomain, Cname, Ip, Servfail, etc.
- **DnssecStatus**: Secure, Insecure, Bogus, Abandoned

## Building

### Prerequisiti

- Rust 1.70+
- **libclang** (per bindgen)

```bash
# Ubuntu/Debian
sudo apt install libclang-dev

# Fedora
sudo dnf install clang-devel

# macOS
brew install llvm
```

### Build

```bash
# Library
cargo build --release

# Esempi
cargo build --examples

# Run esempio specifico
cargo run --example basic
cargo run --example summary
```

### Optional Features

```toml
[dependencies]
ftl-stats = { version = "0.1", features = ["json"] }
```

- `json`: Abilita serializzazione JSON con serde

## Examples

Vedi [`examples/README.md`](examples/README.md) per la guida completa.

- **`basic.rs`**: Statistiche base e contatori raw
- **`summary.rs`**: Dashboard completa con distribuzioni
- **`comprehensive.rs`**: Esempio completo di tutte le API (domains, clients, upstreams, overtime)

## Architecture

```
FTL (C) → /dev/shm/FTL-<PID>-* → bindgen → raw:: → convert → API pubblica
              ↓                      ↓         ↓       ↓
          shared memory        auto-generated safe  ergonomic
                              C bindings     access Rust types
```

### Components

- **bindgen**: Genera binding automatici dalle struct C di FTL (versione 14)
- **memmap2**: Memory mapping sicuro per accesso zero-copy a `/dev/shm`
- **thiserror**: Error handling ergonomico
- **shmem**: Modulo per discovery e apertura segmenti shared memory
- **convert**: Layer di conversione da raw bindings a tipi Rust idiomatici
- **api**: Struct pubbliche ergonomiche (StatsSummary, Query, etc.)

## Compatibility

- **FTL Shared Memory Version**: 14
- **Platform**: Linux (x86_64, ARM)
- **Tested with**: Pi-hole FTL v5.x, v6.x
- **Version Tolerance**:
  - ✅ Validazione flessibile delle dimensioni struct (supporta struct di dimensioni diverse)
  - ✅ Query types hardcoded (MX, SOA, etc.) per stabilità cross-version
  - ⚠️  Versioni molto vecchie di FTL potrebbero avere enum con valori diversi

### Note sulla compatibilità enum

I valori degli enum dei query types sono **hardcoded** nella libreria basandosi su FTL v6.x. Questo garantisce che:
- Una query MX viene sempre identificata correttamente come MX (valore 9)
- Non ci sono discrepanze tra versioni diverse di FTL
- La libreria non dipende dai valori generati da bindgen che potrebbero cambiare

**Versioni supportate**: FTL v5.18+ (dove l'ordine degli enum è stabile)

## Troubleshooting

**"Shared memory segment not found"**
```bash
# Verifica che i file esistano nel container
docker exec pihole ls -la /dev/shm/FTL-*

# Verifica che siano accessibili dall'host
ls -la /dev/shm/FTL-*
```

**"Version mismatch"**
```bash
# Verifica versione FTL
docker exec pihole pihole-FTL version
```

**"Permission denied"**
```bash
# Verifica permessi
ls -la /dev/shm/FTL-*
# I file dovrebbero essere leggibili (r--)
```

## License

MIT OR Apache-2.0
