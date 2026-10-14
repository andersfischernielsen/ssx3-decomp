//! Boot the recompiled game: machine setup shared by the windowed and
//! headless front ends.

use std::path::PathBuf;

use ssx3_ps2::{machine, Cpu};

pub fn repo_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Create the machine, load the disc and the executable, start the main thread.
pub fn boot() -> Box<Cpu> {
    machine::register_functions(&ssx3_ee::FUNCTIONS);
    // Recompiled IOP sound modules; used when SSX3_IOP=1 (see iop_bridge.rs).
    ssx3_iop::register_generated(&ssx3_iop_gen::GENERATED);
    let mut c = Cpu::new();
    let iso = std::env::var("SSX3_ISO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("SSX 3 (USA).iso"));
    if let Err(e) = c.sif.open_disc(iso.clone()) {
        eprintln!("warning: cannot open disc image {}: {e}", iso.display());
    }
    let elf_path = repo_root().join("orig/SLUS_207.72");
    let elf = std::fs::read(&elf_path).unwrap_or_else(|e| panic!("{}: {e}", elf_path.display()));
    machine::boot(&mut c, &elf, "cdrom0:\\SLUS_207.72;1").expect("boot");
    // Temporary debug switch until the IPU (MPEG decoder) lands: the boot
    // movies are selected by a bitmask at gp-0x1724 (0x4a19cc; initial value 7 =
    // EA logo | THX | intro, read by ssx_BootMoviesUpdate 0x1a27a0). Clearing it
    // skips them. SSX3_SKIP_MOVIES=0 keeps them.
    if std::env::var("SSX3_SKIP_MOVIES").map(|v| v != "0").unwrap_or(true) {
        c.bus.write32(0x004a_19cc, 0);
    }
    c
}

/// One-line status summary.
pub fn status(c: &Cpu) -> String {
    let threads: Vec<String> = c
        .kernel
        .threads
        .values()
        .map(|t| format!("{}:{:?}@{:x}p{}", t.id, t.status, if t.id == c.kernel.current { c.pc } else { t.ctx.pc }, t.priority))
        .collect();
    format!(
        "frame {} cycles {}M thread {} prims {} | {}",
        c.sched.frames,
        c.cycles / 1_000_000,
        c.kernel.current,
        c.bus.hw.gs.prims_drawn,
        threads.join(" ")
    )
}

/// Multi-line dump of every thread and semaphore.
pub fn dump(c: &Cpu) -> String {
    let mut out = String::new();
    for t in c.kernel.threads.values() {
        let (pc, ra, sp) = if t.id == c.kernel.current {
            (c.pc, c.g[31][0] as u32, c.g[29][0] as u32)
        } else {
            (t.ctx.pc, t.ctx.g[31][0] as u32, t.ctx.g[29][0] as u32)
        };
        out += &format!(
            "  thread {} {:?} prio {} entry {:#x} pc {:#x} ra {:#x} sp {:#x} wait {:?}/{}\n",
            t.id, t.status, t.priority, t.entry, pc, ra, sp, t.wait, t.wait_id
        );
        // Heuristic backtrace: words on the stack that are return addresses
        // (point just after a jal/jalr in translated code).
        let mut bt = Vec::new();
        for i in 0..512u32 {
            let v = c.bus.ram_u32(sp.wrapping_add(i * 4));
            if (0x0010_0000..0x0042_e590).contains(&v) && v & 3 == 0 && ssx3_ps2::machine::has_code(v) {
                let call = c.bus.ram_u32(v - 8);
                if call >> 26 == 3 || (call >> 26 == 0 && call & 63 == 9) {
                    bt.push(format!("{v:x}"));
                }
            }
            if bt.len() >= 16 {
                break;
            }
        }
        out += &format!("    stack: {}\n", bt.join(" "));
    }
    out += &format!("  cpu per thread (Mcycles): {:?}\n", c.kernel.thread_cycles.iter().map(|(k, v)| (*k, v / 1_000_000)).collect::<Vec<_>>());
    for (id, s) in &c.kernel.semas {
        if !s.waiters.is_empty() {
            out += &format!("  sema {id} count {} waiters {:?}\n", s.count, s.waiters);
        }
    }
    out
}
