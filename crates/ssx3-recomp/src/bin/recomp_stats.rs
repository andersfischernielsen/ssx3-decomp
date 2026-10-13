//! Small CLI over the embedded function inventory.
//!
//! ```text
//! recomp-stats                 # summary + list of named functions
//! recomp-stats 0x0012ab34      # look up a function by address
//! recomp-stats SetGsCrt        # search functions by name
//! ```

use ssx3_recomp::index;

fn main() {
    let idx = index();
    println!("functions : {}", idx.len());
    if let Some((lo, hi)) = idx.address_range() {
        println!("range     : {lo:08x}..{hi:08x}");
    }
    println!("named     : {}", idx.named_count());
    println!("auto      : {}", idx.len() - idx.named_count());

    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first() {
        None => {
            println!("\nnamed functions:");
            for f in idx.named() {
                println!("  {:08x}  {}", f.address, f.name);
            }
        }
        Some(q) => {
            let hex = q.trim_start_matches("0x");
            if hex.len() == 8 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                let addr = u32::from_str_radix(hex, 16).unwrap();
                match idx.by_address(addr) {
                    Some(f) => println!("{:08x}  {}", f.address, f.name),
                    None => match idx.containing(addr) {
                        Some(f) => println!(
                            "{addr:08x} falls inside {:#010x}  {}",
                            f.address, f.name
                        ),
                        None => println!("no function at or containing {addr:#010x}"),
                    },
                }
            } else {
                let hits: Vec<_> = idx.search(q).collect();
                println!("\n{} match(es) for {q:?}:", hits.len());
                for f in hits {
                    println!("  {:08x}  {}", f.address, f.name);
                }
            }
        }
    }
}
