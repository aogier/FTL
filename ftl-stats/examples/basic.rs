use ftl_stats::FtlStats;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("FTL Stats - Basic Example\n");

    // Auto-discover FTL PID
    let stats = FtlStats::new()?;

    println!("✓ Connected to FTL (PID: {})", stats.ftl_pid());

    // Accesso raw ai counters per testing
    let counters = stats.raw_counters();

    println!("\n=== Statistiche FTL ===");
    println!("Total queries:      {}", counters.queries);
    println!("Domains:            {}", counters.domains);
    println!("Clients:            {}", counters.clients);
    println!("Upstreams:          {}", counters.upstreams);
    println!("Queries MAX:        {}", counters.queries_MAX);
    println!("DNS cache size:     {}", counters.dns_cache_size);
    println!("DNS cache MAX:      {}", counters.dns_cache_MAX);

    println!("\n=== Database ===");
    println!("Gravity entries:    {}", counters.database.gravity);
    println!("Groups:             {}", counters.database.groups);
    println!("Lists:              {}", counters.database.lists);

    Ok(())
}
