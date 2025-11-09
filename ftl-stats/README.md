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

### Metodo 1: Variabili d'ambiente (Raccomandato)

Il modo più semplice per testare con path custom:

```bash
# Imposta le variabili d'ambiente
export FTL_PID=12345
export FTL_SHM_PATH=/mnt/shm-v6

# Esegui qualsiasi esempio
cargo run --example comprehensive
cargo run --example summary
```

Gli esempi rileveranno automaticamente le variabili d'ambiente!

### Metodo 2: Script Helper per Docker

Trova automaticamente il PID dal container:

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

### Metodo 3: Test con versioni multiple

Per testare con versioni diverse di FTL contemporaneamente:

```bash
# Organizza i file shm in directory separate
mkdir -p /mnt/shm-v5 /mnt/shm-v6
ln -s /dev/shm/FTL-<PID_V5>-* /mnt/shm-v5/
ln -s /dev/shm/FTL-<PID_V6>-* /mnt/shm-v6/

# Test su versione v5
./test-versions.sh v5 /mnt/shm-v5 comprehensive

# Test su versione v6
./test-versions.sh v6 /mnt/shm-v6 comprehensive

# Oppure usa variabili d'ambiente
FTL_SHM_PATH=/mnt/shm-v5 cargo run --example comprehensive
FTL_SHM_PATH=/mnt/shm-v6 cargo run --example comprehensive
```

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

// Leggi da variabili d'ambiente (FTL_PID, FTL_SHM_PATH)
let stats = FtlStats::builder()
    .from_env()
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
- **Tested with**: Pi-hole FTL v6.2, v6.3+
- **Auto-detection**: La libreria rileva automaticamente la versione e si adatta

### Come funziona la compatibilità

La libreria usa un approccio ibrido per massimizzare la compatibilità:

1. **Rilevamento versione automatico**
   - Legge `ShmSettings.version` all'avvio
   - Mostra un warning se rileva dimensioni struct inaspettate
   - Supporta SHARED_MEMORY_VERSION 14

2. **Enum values hardcoded**
   - I valori degli enum (TYPE_A=1, TYPE_MX=9, etc.) sono **hardcoded**
   - Garantisce che query MX sia sempre MX, anche su versioni diverse
   - Basato su FTL v6.x stabile

3. **Struct flessibili**
   - Non valida dimensioni esatte delle struct
   - Permette dimensioni diverse tra FTL 6.2 e 6.3
   - Accede solo ai campi che sappiamo esistere

### Versioni testate

| FTL Version | Status | Note |
|-------------|--------|------|
| 6.3+ | ✅ Fully supported | Versione di riferimento |
| 6.2 | ✅ Supported | Struct di dimensioni diverse, ma compatibile |
| 5.x | ⚠️  Untested | Potrebbe funzionare se shmem v14 |

**Nota**: Se usi una versione non testata, la libreria mostrerà warning ma tenterà comunque di funzionare.

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
