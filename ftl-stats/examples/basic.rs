use ftl_stats::FtlStats;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("FTL Stats - Basic Example\n");

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
