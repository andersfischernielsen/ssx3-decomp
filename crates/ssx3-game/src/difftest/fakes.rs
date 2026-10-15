//! Fake callees for testing replacements that make calls.
//!
//! Callees at addresses outside the function table (vtable slots a test
//! points at [`FAKE`], jump tables in empty RAM) land in the `miss` /
//! `jump_out` / `syscall` hooks, which [`machine`] points at [`fake`]. The
//! fake records everything the caller set up (every GPR, both halves,
//! f12-f15, cycles, the target) in a log in RAM at [`LOG`], so the RAM
//! comparison checks the register dance; then it returns a pointer in v0
//! (or the result the setup forced at [`LOG_RESULTS`]), clobbers the
//! caller-saved registers, and either returns to `ra` or (at the call index
//! stored at [`LOG_UNWIND_AT`]) unwinds with `pc = 0xdead_0000 | n << 4`.
//! [`covered`] is `check` plus a requirement that the fakes were reached
//! and some calls unwound.

use super::{check, fill, scramble, Rng, DATA, GAME_GP, SP};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;

pub const LOG: u32 = 0x0090_0000;
pub const LOG_NEXT: u32 = LOG;
pub const LOG_UNWIND_AT: u32 = LOG + 4;
pub const LOG_COUNT: u32 = LOG + 8;
/// Results the setup forces for fake call n (0: a pointer from a hash).
pub const LOG_RESULTS: u32 = LOG + 0x10;
pub const LOG_START: u32 = LOG + 0x40;
/// Fake method addresses (below the first translated function).
pub const FAKE: u32 = 0x0000_4000;

/// A 64-bit hash (murmur3 finaliser) for fake results.
pub fn mix(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    x
}

thread_local! {
    /// Fake calls and unwinds made in this test thread.
    static STATS: std::cell::Cell<(u32, u32)> = const { std::cell::Cell::new((0, 0)) };
}

/// `check`, and also require that the setups reached the fakes (the
/// calls were really compared) and made some of them unwind.
pub fn covered(
    generated: fn(&mut Cpu),
    hand: fn(&mut Cpu),
    seeds: u32,
    setup: &dyn Fn(&mut Cpu, u32),
) {
    STATS.with(|s| s.set((0, 0)));
    check(generated, hand, seeds, setup);
    let (calls, unwinds) = STATS.with(|s| s.get());
    assert!(calls >= seeds / 4 && unwinds > 0, "fake calls {calls}, unwinds {unwinds}");
}

/// The `miss` hook: a call to `target` returning to `ra`.
pub fn fake_call(c: &mut Cpu, target: u32) {
    let ra = c.g[RA][0] as u32;
    fake(c, target, ra)
}

/// A syscall: returns to `c.pc` (already the return address).
pub fn fake_syscall(c: &mut Cpu, at: u32) {
    let ret = c.pc;
    fake(c, at, ret)
}

/// Log the call, set results, clobber temporaries, return to `ret` or unwind.
pub fn fake(c: &mut Cpu, target: u32, ret: u32) {
    let n = c.bus.read32(LOG_COUNT);
    c.bus.write32(LOG_COUNT, n + 1);
    let mut p = c.bus.read32(LOG_NEXT);
    if p == 0 {
        p = LOG_START;
    }
    c.bus.write32(p, target);
    c.bus.write32(p + 4, c.pc);
    c.bus.write64(p + 8, c.cycles);
    for r in 0..32 {
        c.bus.write64(p + 0x10 + 16 * r as u32, c.g[r][0]);
        c.bus.write64(p + 0x18 + 16 * r as u32, c.g[r][1]);
    }
    for i in 0..4 {
        c.bus.write32(p + 0x210 + 4 * i, c.f[12 + i as usize]);
    }
    c.bus.write32(LOG_NEXT, p + 0x220);
    let h = mix(c.g[A0][0] ^ (n as u64) << 40 ^ c.g[A1][0].rotate_left(17) ^ c.cycles);
    let forced = if n < 8 { c.bus.read32(LOG_RESULTS + 4 * n) } else { 0 };
    c.g[V0][0] = if forced != 0 { forced as u64 } else { 0x0080_0000 + (h & 0x3fff0) };
    c.g[V1][0] = mix(h);
    for (i, r) in [AT, A0, A1, A2, A3, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9].into_iter().enumerate() {
        c.g[r][0] = mix(h + i as u64);
    }
    c.f[0] = mix(h + 99) as u32;
    c.cycles += 3 + (h & 7);
    let unwind = n == c.bus.read32(LOG_UNWIND_AT);
    STATS.with(|s| {
        let (a, b) = s.get();
        s.set((a + 1, b + unwind as u32));
    });
    c.pc = if unwind {
        0xdead_0000 | n << 4
    } else {
        ret
    };
}

/// The `jump_out` hook: treated as a call to `pc`.
pub fn fake_jump_out(c: &mut Cpu, pc: u32, _start: u32, _end: u32) {
    fake_call(c, pc)
}

/// Scrambled registers, the fake hooks, a game-like gp, and the call
/// index (if any) at which a fake unwinds.
pub fn machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
    let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
    scramble(c, &mut r);
    c.g[GP][0] = GAME_GP as u64;
    c.hooks.miss = fake_call;
    c.hooks.jump_out = fake_jump_out;
    c.hooks.syscall = fake_syscall;
    let unwind = if calls > 0 && r.next_u32() % 3 == 0 { r.next_u32() % calls } else { u32::MAX };
    c.bus.write32(LOG_UNWIND_AT, unwind);
    // Random bytes around the stack frame (the saves overwrite them).
    fill(c, &mut r, SP - 0x100, 0x40);
    r
}

/// A random pointer into the test data area (16-byte aligned).
pub fn ptr(r: &mut Rng) -> u32 {
    DATA + (r.next_u32() & 0xf_fff0)
}

/// A vtable at a random address whose slot `off` is `{delta, fake}`.
pub fn vtable(c: &mut Cpu, r: &mut Rng, slots: &[u32]) -> u32 {
    let vt = ptr(r);
    for (i, &off) in slots.iter().enumerate() {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
    }
    vt
}
