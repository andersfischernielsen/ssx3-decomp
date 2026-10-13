//! Run recompiled entry.

use ssx3_core::elf::Elf;
use ssx3_core::memory_map;
use ssx3_runtime::{generated, Cpu};

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "orig/SLUS_207.72".into());
    let start = args
        .next()
        .and_then(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(memory_map::ENTRY_POINT);

    let elf = match Elf::from_file(&path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("could not read {path}: {e}");
            eprintln!("run tools/extract-from-iso.sh first");
            std::process::exit(1);
        }
    };

    let mut cpu = Cpu::new();
    elf.load_into(&mut cpu.mem);
    cpu.trace_limit = std::env::var("SSX3_TRACE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if let Ok(s) = std::env::var("SSX3_STEPS") {
        if let Ok(n) = s.parse() {
            cpu.step_limit = n;
        }
    }

    println!("running {:08x} ...", start);
    generated::run_entry(&mut cpu);

    println!("steps      : {}", cpu.steps);
    {
        use std::collections::BTreeMap;
        let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
        for n in &cpu.syscalls {
            *counts.entry(*n).or_default() += 1;
        }
        println!("syscalls   : {} call(s)", cpu.syscalls.len());
        for (n, c) in counts.iter().take(12) {
            println!("  {n:#04x} {:<20} x{c}", Cpu::syscall_name(*n));
        }
    }
    println!(
        "registers  : {} {} {} {}",
        cpu.dump_gpr(2),
        cpu.dump_gpr(3),
        cpu.dump_gpr(28),
        cpu.dump_gpr(31)
    );
    match &cpu.trap {
        Some(t) => println!("stopped    : {:#010x}: {}", t.pc, t.reason),
        None => println!("stopped    : returned normally"),
    }

    if !cpu.trace.is_empty() {
        println!("\ntrace:");
        for (pc, w) in &cpu.trace {
            println!("  {pc:08x}  {w:08x}");
        }
    }
}
