//! Status lines and thread dumps.

use ssx3_ps2::Cpu;

/// One-line status summary.
pub fn status(c: &Cpu) -> String {
    let threads: Vec<String> = ssx3_ps2::mr!(c)
        .kernel
        .threads
        .values()
        .map(|t| {
            format!(
                "{}:{:?}@{:x}p{}",
                t.id,
                t.status,
                if t.id == ssx3_ps2::mr!(c).kernel.current {
                    c.pc
                } else {
                    t.ctx.pc
                },
                t.priority
            )
        })
        .collect();
    format!(
        "frame {} cycles {}M thread {} prims {} | {}",
        ssx3_ps2::mr!(c).sched.frames,
        c.cycles / 1_000_000,
        ssx3_ps2::mr!(c).kernel.current,
        ssx3_ps2::mr!(c).hw.gs.prims_drawn(),
        threads.join(" ")
    )
}

/// Multi-line dump of every thread and semaphore.
pub fn dump(c: &Cpu) -> String {
    let mut out = String::new();
    for t in ssx3_ps2::mr!(c).kernel.threads.values() {
        let (pc, ra, sp) = if t.id == ssx3_ps2::mr!(c).kernel.current {
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
            if (0x0010_0000..0x0042_e590).contains(&v)
                && v & 3 == 0
                && ssx3_ps2::machine::has_code(v)
            {
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
    out += &format!(
        "  cpu per thread (Mcycles): {:?}\n",
        ssx3_ps2::mr!(c)
            .kernel
            .thread_cycles
            .iter()
            .map(|(k, v)| (*k, v / 1_000_000))
            .collect::<Vec<_>>()
    );
    for (id, s) in &ssx3_ps2::mr!(c).kernel.semas {
        if !s.waiters.is_empty() {
            out += &format!("  sema {id} count {} waiters {:?}\n", s.count, s.waiters);
        }
    }
    out
}
