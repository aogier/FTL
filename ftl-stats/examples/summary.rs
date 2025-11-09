use ftl_stats::FtlStats;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════╗");
    println!("║    Pi-hole FTL Statistics Summary    ║");
    println!("╚══════════════════════════════════════╝\n");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    let stats = if args.len() >= 2 {
        // PID provided as argument
        let pid: u32 = args[1].parse().expect("Invalid PID");

        if args.len() >= 3 {
            // Both PID and SHM path provided
            let shm_path = &args[2];
            println!("Connecting to FTL PID {} at {}", pid, shm_path);
            FtlStats::builder()
                .pid(pid)
                .shm_path(shm_path)
                .build()?
        } else {
            // Only PID provided, use default /dev/shm
            println!("Connecting to FTL PID {} at /dev/shm", pid);
            FtlStats::with_pid(pid)?
        }
    } else {
        // Auto-discover
        println!("Auto-discovering FTL...");
        FtlStats::new()?
    };

    println!("✓ Connected to FTL (PID: {})\n", stats.ftl_pid());

    // Get summary
    let summary = stats.summary();

    println!("╔══════════════════════════════════════╗");
    println!("║             QUERY STATS              ║");
    println!("╠══════════════════════════════════════╣");
    println!("║ Total queries:     {:>10}       ║", summary.queries_total);
    println!(
        "║ Blocked:           {:>10} ({:>4.1}%) ║",
        summary.queries_blocked, summary.percent_blocked
    );
    println!("║ Forwarded:         {:>10}       ║", summary.queries_forwarded);
    println!("║ Cached:            {:>10}       ║", summary.queries_cached);
    println!("╠══════════════════════════════════════╣");
    println!("║ Unique domains:    {:>10}       ║", summary.domains_unique);
    println!("║ Total clients:     {:>10}       ║", summary.clients_total);
    println!("║ Active clients:    {:>10}       ║", summary.clients_active);
    println!("║ Upstreams:         {:>10}       ║", summary.upstreams_total);
    println!("╠══════════════════════════════════════╣");
    println!("║ Queries/sec:       {:>10.1}       ║", summary.queries_per_second);
    println!("║ Gravity size:      {:>10}       ║", summary.gravity_size);
    println!("╚══════════════════════════════════════╝");

    // Query types
    if !summary.query_types.is_empty() {
        println!("\n╔══════════════════════════════════════╗");
        println!("║           QUERY TYPES                ║");
        println!("╠══════════════════════════════════════╣");
        for (qtype, count) in &summary.query_types {
            println!("║ {:?}: {:>20}             ║", qtype, count);
        }
        println!("╚══════════════════════════════════════╝");
    }

    // Query status distribution
    if !summary.status_distribution.is_empty() {
        println!("\n╔══════════════════════════════════════╗");
        println!("║         STATUS DISTRIBUTION          ║");
        println!("╠══════════════════════════════════════╣");
        for (status, count) in &summary.status_distribution {
            println!("║ {:?}: {:>20}        ║", status, count);
        }
        println!("╚══════════════════════════════════════╝");
    }

    // Recent queries
    println!("\n╔══════════════════════════════════════╗");
    println!("║          RECENT QUERIES              ║");
    println!("╚══════════════════════════════════════╝");

    let recent = stats.recent_queries(5);
    for query in recent {
        let blocked = if query.blocked { "[BLOCKED]" } else { "" };
        println!(
            "{:?} {} -> {} {}",
            query.query_type, query.client_ip, query.domain, blocked
        );
    }

    Ok(())
}
