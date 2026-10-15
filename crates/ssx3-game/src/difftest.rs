//! Differential tests: run a generated function and its replacement from the
//! same state on two fresh machines and compare everything observable
//! (GPRs, FPU, VU0 VF/VI/Q, pc, cycles, RAM, scratchpad).
//!
//! Run with the game profile so the generated crates are reused:
//! `cargo test --profile game -p ssx3-game`.
//!
//! - [`check`]: the default comparison (hard state must match);
//!   [`check_lohi`] also requires equal LO/HI (opt in: some replacements,
//!   e.g. `__muldi3`, legitimately leave different LO/HI);
//!   [`check_paths`] also requires every listed path (total cycle cost) to
//!   be taken by some seed.
//! - [`fakes`]: fake callees for functions that make calls (log every
//!   register to RAM, forced results, unwinds at chosen call indices).
//! - [`events`]: delivering real events inside loops and re-entering
//!   dispatcher-shaped functions as the trampoline does.
//! - Input helpers: [`Rng`], [`scramble`], [`fill`], [`put_vec`],
//!   [`put_words`], [`Rng::edge_f32`], [`GAME_GP`].

pub mod events;
pub mod fakes;

use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Where tests put their data (inside RAM, away from the stack).
pub const DATA: u32 = 0x0080_0000;
/// Initial stack pointer and a sentinel return address.
pub const SP: u32 = 0x01f0_0000;
pub const RA: u32 = 0x0012_3450;
/// The value of `gp` the game runs with (globals are addressed relative to
/// it; set it when a function reads gp-relative data).
pub const GAME_GP: u32 = 0x004a_30f0;

/// A fresh CPU on a fresh machine, with `sp`/`ra` set and events off.
pub fn fresh() -> Box<Cpu> {
    // Indirect calls (virtual methods, function pointers) resolve through the
    // generated function table.
    ssx3_ee_rt::funcs::register_functions(&ssx3_ee::FUNCTIONS);
    let mut c = ssx3_ps2::new_cpu();
    c.g[29][0] = SP as u64;
    c.g[31][0] = RA as u64;
    c.next_event = u64::MAX;
    c
}

/// Everything compared. `hard` differences fail a test; `soft` ones (VU0
/// status/MAC/clip flags, caller-saved temporaries) are reported only. The
/// replay's full RAM comparison is the final check that soft state never
/// reaches memory. `lohi` (LO/HI of both pipelines) fails only in
/// [`check_lohi`].
#[derive(Debug)]
pub struct Report {
    pub hard: Vec<String>,
    pub soft: Vec<String>,
    pub lohi: Vec<String>,
}

/// Run `generated` and `hand` after the same `setup`, return the differences.
pub fn compare(generated: GuestFn, hand: GuestFn, setup: &dyn Fn(&mut Cpu)) -> Report {
    let mut a = fresh();
    let mut b = fresh();
    setup(&mut a);
    setup(&mut b);
    generated(&mut a);
    hand(&mut b);
    let mut hard = Vec::new();
    let mut soft = Vec::new();
    for r in 0..32 {
        // Results, callee-saved and special registers must match; caller-saved
        // temporaries (at, a0-a3, t0-t9, k0/k1; FPU f2-f19) are reported only:
        // callers can't rely on them after a call.
        let gpr_hard = matches!(r, 0 | 2 | 3 | 16..=23 | 28..=31);
        if a.g[r] != b.g[r] {
            let msg = format!("gpr {r}: {:x?} vs {:x?}", a.g[r], b.g[r]);
            if gpr_hard { hard.push(msg) } else { soft.push(msg) }
        }
        let fpr_hard = matches!(r, 0 | 1 | 20..=31);
        if a.f[r] != b.f[r] {
            let msg = format!("f{r}: {:#x} vs {:#x}", a.f[r], b.f[r]);
            if fpr_hard { hard.push(msg) } else { soft.push(msg) }
        }
        let (va, vb) = (a.bus.plat.vf(r), b.bus.plat.vf(r));
        if va != vb {
            hard.push(format!("vf{r}: {va:x?} vs {vb:x?}"));
        }
    }
    for reg in 0..23u32 {
        let (x, y) = (a.bus.plat.read_ctrl(reg), b.bus.plat.read_ctrl(reg));
        if x != y {
            let msg = format!("vu0 ctrl {reg}: {x:#x} vs {y:#x}");
            if (16..=18).contains(&reg) {
                soft.push(msg);
            } else {
                hard.push(msg);
            }
        }
    }
    for (name, x, y) in [
        ("fcr31", a.fcr31 as u64, b.fcr31 as u64),
        ("pc", a.pc as u64, b.pc as u64),
        ("cycles", a.cycles, b.cycles),
    ] {
        if x != y {
            hard.push(format!("{name}: {x:#x} vs {y:#x}"));
        }
    }
    let mut lohi = Vec::new();
    for p in 0..2 {
        for (name, x, y) in [("lo", a.lo[p], b.lo[p]), ("hi", a.hi[p], b.hi[p])] {
            if x != y {
                lohi.push(format!("{name}[{p}]: {x:#x} vs {y:#x}"));
            }
        }
    }
    for (name, ma, mb) in [("ram", &a.bus.ram, &b.bus.ram), ("spr", &a.bus.spr, &b.bus.spr)] {
        let mut n = 0;
        for i in (0..ma.len()).step_by(4) {
            if ma[i..i + 4] != mb[i..i + 4] {
                if n < 8 {
                    hard.push(format!(
                        "{name} {i:#x}: {:02x?} vs {:02x?}",
                        &ma[i..i + 4],
                        &mb[i..i + 4]
                    ));
                }
                n += 1;
            }
        }
        if n > 8 {
            hard.push(format!("{name}: {n} differing words in total"));
        }
    }
    Report { hard, soft, lohi }
}

/// Assert no hard differences over many `setup(seed)` states.
pub fn check(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
    for seed in 0..seeds {
        let r = compare(generated, hand, &|c| setup(c, seed));
        assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
    }
}

/// [`check`], and LO/HI must match too (for replacements of code with
/// `mult`/`div`, whose LO/HI a later `mflo`/`mfhi` could read).
pub fn check_lohi(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
    for seed in 0..seeds {
        let r = compare(generated, hand, &|c| setup(c, seed));
        assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
        assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
    }
}

/// Require that the generated function takes every listed path (identified
/// by its total cycle cost) for some seed, so the seeds really cover them.
pub fn require_paths(generated: GuestFn, seeds: u32, paths: &[u64], setup: &dyn Fn(&mut Cpu, u32)) {
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..seeds {
        let mut c = fresh();
        setup(&mut c, seed);
        let before = c.cycles;
        generated(&mut c);
        seen.insert(c.cycles - before);
    }
    for p in paths {
        assert!(seen.contains(p), "path costing {p} cycles never taken (seen {seen:?})");
    }
}

/// [`check`] plus [`require_paths`].
pub fn check_paths(
    generated: GuestFn,
    hand: GuestFn,
    seeds: u32,
    paths: &[u64],
    setup: &dyn Fn(&mut Cpu, u32),
) {
    check(generated, hand, seeds, setup);
    require_paths(generated, seeds, paths, setup);
}

/// [`check_lohi`] plus [`require_paths`].
pub fn check_paths_lohi(
    generated: GuestFn,
    hand: GuestFn,
    seeds: u32,
    paths: &[u64],
    setup: &dyn Fn(&mut Cpu, u32),
) {
    check_lohi(generated, hand, seeds, setup);
    require_paths(generated, seeds, paths, setup);
}

/// Small deterministic RNG for test inputs.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
    /// A float in [-range, range).
    pub fn f32(&mut self, range: f32) -> u32 {
        let u = self.next_u32() as f32 / u32::MAX as f32;
        ((u * 2.0 - 1.0) * range).to_bits()
    }
}

/// Random values in every register a function might read by accident: GPRs
/// (both 64-bit halves; not zero/gp/sp/ra), FPRs, the FPU condition bit and
/// VU0 VF1-VF31. Call before the test's own setup so incidental leftovers
/// (v1, upper halves, FCC) are actually checked.
pub fn scramble(c: &mut Cpu, r: &mut Rng) {
    for reg in 1..32 {
        if matches!(reg, 28 | 29 | 31) {
            continue;
        }
        c.g[reg] = [
            (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        ];
    }
    for f in c.f.iter_mut() {
        *f = r.next_u32();
    }
    c.fcr31 = if r.next_u32() & 1 != 0 { ssx3_ee_rt::fpu::FCC } else { 0 };
    for reg in 1..32 {
        let v = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
        c.bus.plat.set_vf(reg, v);
    }
}

/// Fill `words` 32-bit words at `addr` with random values.
pub fn fill(c: &mut Cpu, r: &mut Rng, addr: u32, words: u32) {
    for i in 0..words {
        c.bus.write32(addr + i * 4, r.next_u32());
    }
}

impl Rng {
    /// A float bit pattern biased toward edge cases: ±0, denormals,
    /// exponent 255 (PS2 has no inf/NaN), ±1, small integers and halves.
    pub fn edge_f32(&mut self) -> u32 {
        let sign = (self.next_u32() & 1) << 31;
        match self.next_u32() % 10 {
            0 => sign,
            1 => sign | (self.next_u32() & 0x007f_ffff),
            2 => sign | 0x7f80_0000 | (self.next_u32() & 0x007f_ffff),
            3 => sign | 0x3f80_0000,
            4 => sign | ((self.next_u32() % 64) as f32).to_bits(),
            5 => sign | ((self.next_u32() % 64) as f32 + 0.5).to_bits(),
            _ => self.f32(1000.0),
        }
    }
}

/// Write 32-bit words at `addr` (constant tables such as `.lit4`
/// literals: the test machine's RAM starts empty).
pub fn put_words(c: &mut Cpu, addr: u32, words: &[u32]) {
    for (i, w) in words.iter().enumerate() {
        c.bus.write32(addr + 4 * i as u32, *w);
    }
}

/// Write a quadword of four float bit patterns.
pub fn put_vec(c: &mut Cpu, addr: u32, v: [u32; 4]) {
    let q = (v[0] as u128) | (v[1] as u128) << 32 | (v[2] as u128) << 64 | (v[3] as u128) << 96;
    c.bus.write128(addr, q);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rider_build_world_matrix() {
        check(
            ssx3_ee::s00::f_0011e098__generated,
            crate::rider::build_world_matrix,
            300,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
                c.g[4][0] = DATA as u64;
                put_vec(c, DATA + 0x110, [r.f32(1e5), r.f32(1e5), r.f32(1e5), r.f32(2.0)]);
                put_vec(c, DATA + 0x120, [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)]);
                for reg in 1..32 {
                    let v = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
                    c.bus.plat.set_vf(reg, v);
                }
            },
        );
    }
}
