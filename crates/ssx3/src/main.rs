//! SSX 3 (USA) native entry point.
//!
//! At this stage this just brings up the machine model so the scaffold is
//! runnable. The wgpu window and the recompiled `entry` are wired in next.

use ssx3_core::{memory_map, Memory};
use ssx3_gs::Gs;

fn main() {
    let mem = Memory::new();
    let gs = Gs::new();

    println!("SSX 3 (USA) — SLUS_207.72");
    println!(
        "  image base : {:#010x}   entry: {:#010x}",
        memory_map::IMAGE_BASE,
        memory_map::ENTRY_POINT
    );
    println!("  main RAM   : {} MiB", mem.ram.len() / (1024 * 1024));
    println!("  scratchpad : {} KiB", mem.scratchpad.len() / 1024);
    println!("  GS VRAM    : {} MiB", gs.vram.len() / (1024 * 1024));
}
