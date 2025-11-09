// Debug example to print struct sizes
use ftl_stats::raw;

fn main() {
    println!("=== Struct Sizes ===");
    println!("ShmSettings: {} bytes", std::mem::size_of::<raw::ShmSettings>());
    println!("countersStruct: {} bytes", std::mem::size_of::<raw::countersStruct>());
    println!("queriesData: {} bytes", std::mem::size_of::<raw::queriesData>());
    println!("clientsData: {} bytes", std::mem::size_of::<raw::clientsData>());
    println!("domainsData: {} bytes", std::mem::size_of::<raw::domainsData>());
    println!("upstreamsData: {} bytes", std::mem::size_of::<raw::upstreamsData>());
    println!("overTimeData: {} bytes", std::mem::size_of::<raw::overTimeData>());
    println!("DNSCacheData: {} bytes", std::mem::size_of::<raw::DNSCacheData>());
}
