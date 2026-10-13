//! Small CLI over the embedded function inventory and decoder.
//!
//! ```text
//! recomp-stats                 # summary + list of named functions
//! recomp-stats 0x0012ab34      # look up a function by address
//! recomp-stats SetGsCrt        # search functions by name
//! recomp-stats --coverage      # decode the whole image and report coverage
//! recomp-stats --disasm entry  # disassemble a function
//! ```

use std::collections::BTreeMap;

use ssx3_recomp::boundaries::boundaries;
use ssx3_recomp::codegen;
use ssx3_recomp::flow;
use ssx3_recomp::index;
use ssx3_recomp::program::Program;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--coverage") => coverage(args.get(1).map(String::as_str)),
        Some("--flow") => flow_check(args.get(1).map(String::as_str)),
        Some("--todo") => todo_report(),
        Some("--disasm") => {
            let name = args.get(1).cloned().unwrap_or_else(|| "entry".into());
            disasm(&name);
        }
        _ => summary(&args),
    }
}

fn summary(args: &[String]) {
    let idx = index();
    println!("functions : {}", idx.len());
    if let Some((lo, hi)) = idx.address_range() {
        println!("range     : {lo:08x}..{hi:08x}");
    }
    println!("named     : {}", idx.named_count());
    println!("auto      : {}", idx.len() - idx.named_count());

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

fn open_program(path: Option<&str>) -> Program {
    let path = path.unwrap_or("orig/SLUS_207.72");
    Program::open(path).unwrap_or_else(|e| {
        eprintln!("could not open {path}: {e}");
        eprintln!("run tools/extract-from-iso.sh first");
        std::process::exit(1);
    })
}

fn coverage(path: Option<&str>) {
    let program = open_program(path);
    let mut total = 0usize;
    let mut known = 0usize;
    let mut unknown_by_opcode: BTreeMap<u32, usize> = BTreeMap::new();

    for f in program.functions() {
        for inst in program.decode(f) {
            total += 1;
            if inst.is_known() {
                known += 1;
            } else {
                *unknown_by_opcode.entry(inst.word >> 26).or_default() += 1;
            }
        }
    }

    let pct = 100.0 * known as f64 / total as f64;
    println!("functions decoded : {}", program.functions().len());
    println!("instructions      : {total}");
    println!("  decoded         : {known} ({pct:.2}%)");
    println!("  unrecognised    : {}", total - known);
    if total > known {
        println!("\nunrecognised by opcode:");
        for (op, n) in &unknown_by_opcode {
            println!("  opcode 0x{op:02x}: {n}");
        }
    }
}

fn todo_report() {
    use std::collections::BTreeMap;
    let program = open_program(None);
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut total = 0usize;
    let mut unsupported = 0usize;
    for f in program.functions() {
        for inst in program.decode(f) {
            total += 1;
            if !codegen::can_lower(&inst) {
                unsupported += 1;
                *counts.entry(inst.mnemonic).or_default() += 1;
            }
        }
    }
    println!("instructions      : {total}");
    println!("unsupported       : {unsupported}");
    let mut v: Vec<_> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\ntop unsupported mnemonics:");
    for (m, c) in v.iter().take(40) {
        println!("  {m:<16} {c}");
    }
}

fn flow_check(path: Option<&str>) {
    use std::collections::BTreeSet;

    let program = open_program(path);
    let b = boundaries();

    let mut exact = 0usize;
    let mut matched = 0usize;
    let mut extra = 0usize;
    let mut missing = 0usize;
    let mut total = 0usize;

    for f in program.functions() {
        let Some(runs) = b.runs(f.address) else { continue };
        total += 1;
        let max = (f.len() as usize / 4) + 64;
        let d = flow::disassemble(&program.elf, f.address, max);

        // Only compare within the function's body: recursive descent legitimately
        // spills into neighbours via fall-through.
        let ours: BTreeSet<u32> = d
            .instructions
            .range(f.address..f.end)
            .map(|(a, _)| *a)
            .collect();
        let gh: BTreeSet<u32> = runs.iter().flat_map(|&(lo, hi)| (lo..hi).step_by(4)).collect();

        matched += ours.intersection(&gh).count();
        extra += ours.difference(&gh).count();
        missing += gh.difference(&ours).count();
        if ours == gh {
            exact += 1;
        }
    }

    let pct = 100.0 * exact as f64 / total.max(1) as f64;
    println!("functions checked : {total}");
    println!("exact match       : {exact} ({pct:.2}%)");
    println!("instructions      : matched {matched}, extra {extra}, missing {missing}");
}

fn disasm(name: &str) {
    let program = open_program(None);
    let f = match name.strip_prefix("0x").and_then(|h| u32::from_str_radix(h, 16).ok()) {
        Some(addr) => index().by_address(addr),
        None => index().search(name).next(),
    }
    .unwrap_or_else(|| {
        eprintln!("no function matching {name:?}");
        std::process::exit(1);
    });
    println!("{} @ {:08x} ({} bytes)", f.name, f.address, f.len());
    for (i, inst) in program.decode(f).iter().enumerate() {
        let addr = f.address + (i as u32) * 4;
        println!("  {addr:08x}  {:08x}  {inst}", inst.word);
    }
}
