# FTL Stats Examples

Esempi di utilizzo della libreria `ftl-stats`.

## Testing con Pi-hole in Container

### Scoprire il PID di FTL nel container

```bash
# Trova il PID del processo FTL nel container
docker exec pihole pidof pihole-FTL

# Oppure leggi il file PID
docker exec pihole cat /run/pihole-FTL.pid
```

### Eseguire gli esempi dall'host

Gli esempi accettano argomenti da riga di comando:

```bash
# Sintassi
cargo run --example <esempio> [PID] [SHM_PATH]
```

#### Opzione 1: Auto-discovery (solo se FTL gira sull'host)
```bash
cargo run --example basic
cargo run --example summary
```

#### Opzione 2: Specificare solo il PID
```bash
# Usa /dev/shm di default
cargo run --example basic 12345
cargo run --example summary 12345
```

#### Opzione 3: Specificare PID e path custom
```bash
# Se i file shmem sono montati in un path custom
cargo run --example basic 12345 /path/to/shm
cargo run --example summary 12345 /custom/shm
```

### Esempio completo con Docker

```bash
# 1. Trova il PID di FTL nel container
FTL_PID=$(docker exec pihole pidof pihole-FTL)
echo "FTL PID: $FTL_PID"

# 2. Verifica i file shmem nel container
docker exec pihole ls -la /dev/shm/FTL-*

# 3. Se /dev/shm è condiviso con l'host (raccomandato)
cargo run --example summary $FTL_PID

# 4. Se hai montato /dev/shm in un path custom
# (es. docker run -v /host/shm:/container/shm)
cargo run --example summary $FTL_PID /host/shm
```

### Troubleshooting

**Errore: "Shared memory segment 'FTL-XXX-counters' not found"**

Verifica che i file shmem siano accessibili dall'host:

```bash
# Nel container
docker exec pihole ls -la /dev/shm/FTL-*

# Sull'host (se /dev/shm è condiviso)
ls -la /dev/shm/FTL-*
```

Se i file non sono visibili sull'host, monta `/dev/shm`:

```yaml
# docker-compose.yml
services:
  pihole:
    volumes:
      - /dev/shm:/dev/shm  # Condividi shared memory con host
```

**Errore: "Version mismatch"**

La libreria supporta shared memory versione 14. Verifica la versione di FTL:

```bash
docker exec pihole pihole-FTL version
```

## Esempi Disponibili

### `basic.rs`
Mostra statistiche di base e contatori raw:
- Query totali, domini, client, upstream
- Dimensioni cache DNS
- Statistiche database (gravity, groups, lists)

### `summary.rs`
Dashboard completa con:
- Statistiche query (totali, bloccate, forwarded, cached)
- Percentuale blocco e QPS
- Distribuzione per tipo query (A, AAAA, PTR, etc.)
- Distribuzione per status (GRAVITY, FORWARDED, CACHE, etc.)
- Ultime 5 query recenti

## Build

```bash
# Build tutti gli esempi
cargo build --examples

# Build esempio specifico
cargo build --example basic
cargo build --example summary
```
