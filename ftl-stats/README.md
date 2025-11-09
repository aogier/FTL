# ftl-stats

Libreria Rust per leggere statistiche Pi-hole FTL dalla shared memory.

## Status

🚧 **Work in Progress** - Fase 2/5 in corso

- ✅ Fase 1: Setup e bindgen (completata)
- 🔄 Fase 2: Conversioni ergonomiche (in corso)
- ⏳ Fase 3: API pubblica
- ⏳ Fase 4: Esempi e test
- ⏳ Fase 5: Documentazione

## Features

- ✅ **Binding automatici** tramite bindgen (sempre sincronizzati con FTL)
- ✅ **Lettura zero-copy** da `/dev/shm` tramite memory mapping
- ✅ **Type-safe** accesso a shared memory
- ✅ **Auto-discovery** del PID di FTL
- ⏳ API ergonomica (in sviluppo)

## Quick Start (current)

```rust
use ftl_stats::FtlStats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stats = FtlStats::new()?;
    println!("FTL PID: {}", stats.ftl_pid());

    let counters = stats.raw_counters();
    println!("Total queries: {}", counters.queries);

    Ok(())
}
```

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
cargo build --release

# Run example
cargo run --example basic
```

## Architecture

- **bindgen**: genera automaticamente binding dalle struct C di FTL
- **memmap2**: memory mapping sicuro per accesso a `/dev/shm`
- **thiserror**: error handling ergonomico

## License

MIT OR Apache-2.0
