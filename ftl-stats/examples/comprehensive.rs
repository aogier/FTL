use ftl_stats::FtlStats;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FTL Stats - Comprehensive Example ===\n");

    // Gestione argomenti CLI o variabili d'ambiente
    let args: Vec<String> = env::args().collect();

    let stats = if args.len() >= 2 {
        // CLI arguments (backward compatibility)
        let pid: u32 = args[1].parse().expect("Invalid PID");
        if args.len() >= 3 {
            let shm_path = &args[2];
            FtlStats::builder().pid(pid).shm_path(shm_path).build()?
        } else {
            FtlStats::with_pid(pid)?
        }
    } else if env::var("FTL_PID").is_ok() || env::var("FTL_SHM_PATH").is_ok() {
        // Environment variables
        FtlStats::builder().from_env().build()?
    } else {
        // Auto-discover
        FtlStats::new()?
    };

    println!("FTL PID: {}\n", stats.ftl_pid());

    // === SUMMARY ===
    println!("━━━ SUMMARY ━━━");
    let summary = stats.summary();
    println!("Total queries: {}", summary.queries_total);
    println!("Blocked queries: {} ({:.1}%)", summary.queries_blocked, summary.percent_blocked);
    println!("Cached: {}", summary.queries_cached);
    println!("Forwarded: {}", summary.queries_forwarded);
    println!("Unique domains: {}", summary.domains_unique);
    println!("Active clients: {}", summary.clients_active);
    println!("Queries/sec: {:.2}", summary.queries_per_second);
    println!();

    // === TOP DOMAINS ===
    println!("━━━ TOP 10 DOMAINS (allowed) ━━━");
    for (i, domain) in stats.top_domains(10, false).iter().enumerate() {
        println!("{:2}. {} - {} queries",
                 i + 1, domain.domain, domain.count);
    }
    println!();

    // === TOP BLOCKED DOMAINS ===
    println!("━━━ TOP 10 BLOCKED DOMAINS ━━━");
    let blocked_domains: Vec<_> = stats.top_domains(100, true)
        .into_iter()
        .filter(|d| d.blocked_count > 0)
        .take(10)
        .collect();

    for (i, domain) in blocked_domains.iter().enumerate() {
        println!("{:2}. {} - {} blocked",
                 i + 1, domain.domain, domain.blocked_count);
    }
    println!();

    // === TOP CLIENTS ===
    println!("━━━ TOP 10 CLIENTS ━━━");
    for (i, client) in stats.top_clients(10).iter().enumerate() {
        let name_info = client.name.as_ref()
            .map(|n| format!(" ({})", n))
            .unwrap_or_default();
        println!("{:2}. {}{} - {} queries, {} blocked",
                 i + 1, client.ip, name_info, client.count, client.blocked_count);
    }
    println!();

    // === UPSTREAMS ===
    println!("━━━ UPSTREAM SERVERS ━━━");
    for upstream in stats.upstreams() {
        let name_info = upstream.name.as_ref()
            .map(|n| format!(" ({})", n))
            .unwrap_or_default();
        println!("{}:{}{} - {} queries, {} failed, {:.2}ms avg",
                 upstream.ip, upstream.port, name_info,
                 upstream.count, upstream.failed, upstream.response_time_avg_ms);
    }
    println!();

    // === RECENT QUERIES ===
    println!("━━━ LAST 5 QUERIES ━━━");
    for query in stats.recent_queries(5) {
        let blocked_indicator = if query.blocked { "🚫" } else { "✓ " };
        println!("{} {} → {} ({:?})",
                 blocked_indicator, query.client_ip, query.domain, query.query_type);
    }
    println!();

    // === OVERTIME DATA ===
    println!("━━━ OVERTIME STATS (last 5 slots) ━━━");
    let overtime = stats.overtime_data();
    for slot in overtime.iter().rev().take(5) {
        println!("{:?} - Total: {}, Blocked: {}, Cached: {}, Forwarded: {}",
                 slot.timestamp, slot.total, slot.blocked, slot.cached, slot.forwarded);
    }
    println!();

    // === QUERY TYPES ===
    println!("━━━ QUERY TYPE DISTRIBUTION ━━━");
    let mut types: Vec<_> = summary.query_types.iter().collect();
    types.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
    for (qtype, count) in types.iter().take(5) {
        println!("{:?}: {}", qtype, count);
    }
    println!();

    // === STATUS DISTRIBUTION ===
    println!("━━━ STATUS DISTRIBUTION ━━━");
    let mut statuses: Vec<_> = summary.status_distribution.iter().collect();
    statuses.sort_by(|a, b| b.1.cmp(a.1));
    for (status, count) in statuses {
        println!("{:?}: {}", status, count);
    }

    Ok(())
}
