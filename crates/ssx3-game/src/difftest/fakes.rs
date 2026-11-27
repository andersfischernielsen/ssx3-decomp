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
    assert!(
        calls >= seeds / 4 && unwinds > 0,
        "fake calls {calls}, unwinds {unwinds}"
    );
}

/// `check`, and also require that for each address in `rets` some seed's
/// generated run made a logged call (a fake or a syscall) returning there:
/// the arm making that call really ran.
pub fn check_reaching(
    generated: fn(&mut Cpu),
    hand: fn(&mut Cpu),
    seeds: u32,
    rets: &[u32],
    setup: &dyn Fn(&mut Cpu, u32),
) {
    check(generated, hand, seeds, setup);
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..seeds {
        let mut c = super::fresh();
        setup(&mut c, seed);
        super::run_generated(&mut c, generated);
        seen.extend(logged_returns(&mut c));
    }
    for ret in rets {
        assert!(
            seen.contains(ret),
            "no seed made the call returning to {ret:#x}"
        );
    }
}

/// The return address (`ra`) of every call logged on this machine.
pub fn logged_returns(c: &mut Cpu) -> Vec<u32> {
    let n = c.bus.read32(LOG_COUNT);
    (0..n)
        .map(|i| c.bus.read64(LOG_START + 0x220 * i + 0x10 + 16 * RA as u32) as u32)
        .collect()
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
    let forced = if n < 8 {
        c.bus.read32(LOG_RESULTS + 4 * n)
    } else {
        0
    };
    c.g[V0][0] = if forced != 0 {
        forced as u64
    } else {
        0x0080_0000 + (h & 0x3fff0)
    };
    c.g[V1][0] = mix(h);
    for (i, r) in [AT, A0, A1, A2, A3, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9]
        .into_iter()
        .enumerate()
    {
        c.g[r][0] = mix(h + i as u64);
    }
    c.f[0] = mix(h + 99) as u32;
    c.cycles += 3 + (h & 7);
    let unwind = n == c.bus.read32(LOG_UNWIND_AT);
    STATS.with(|s| {
        let (a, b) = s.get();
        s.set((a + 1, b + unwind as u32));
    });
    c.pc = if unwind { 0xdead_0000 | n << 4 } else { ret };
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
    let unwind = if calls > 0 && r.next_u32() % 3 == 0 {
        r.next_u32() % calls
    } else {
        u32::MAX
    };
    c.bus.write32(LOG_UNWIND_AT, unwind);
    // Random bytes around the stack frame (the saves overwrite them).
    fill(c, &mut r, SP - 0x100, 0x40);
    r
}

/// [`machine`] plus the runtime a callee that allocates needs: the engine's
/// size-class pool allocator with its dlmalloc arena ([`test_allocator`]) and
/// the BELibrary dummy at `gp - 0x1f04`. Use it for fixtures whose real
/// callees (constructors, popups, string tables) call `EA_MemAllocNamed`:
/// without it they spin in `EAFile_AllocRequest` (pc 0x3de768 / 0x31ef40).
/// The registers the allocator setup used are scrambled again.
pub fn machine_rt(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
    let mut r = machine(c, seed, salt, calls);
    test_allocator(c);
    c.bus
        .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    scramble(c, &mut r);
    c.g[GP][0] = GAME_GP as u64;
    r
}

/// The engine's size-class pool allocator (enabled through `gp + 0xdd8`)
/// plus the dlmalloc arena the sub-constructors' allocations fall back to
/// (the heap at 0x4ff250, backed by `DATA+0x40000..+0x80000`). Leaves the
/// argument registers and `pc` as the arena setup call left them.
pub fn test_allocator(c: &mut Cpu) {
    // What the setup changes inside the game image is recorded, so a later
    // `elf_image` keeps it (see `difftest::RT_PATCH`).
    static SPAN: std::sync::OnceLock<(usize, usize)> = std::sync::OnceLock::new();
    let (at, len) = *SPAN.get_or_init(super::image_span);
    let before = c.bus.ram[at..at + len].to_vec();
    c.bus.write32(GAME_GP + 0xdd8, 1);
    // The BELibrary dummy: `getBE` finds a singleton instead of building one.
    c.bus
        .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    const POOLS: [(u32, u32, u32); 7] = [
        (0x2a70, 0x800, 8),
        (0x2a60, 0x2000, 0x10),
        (0x2a6c, 0x32000, 0x20),
        (0x2a7c, 0x25800, 0x40),
        (0x2a84, 0x25800, 0x80),
        (0x2a78, 0x20000, 0x100),
        (0x2a68, 0x20000, 0x200),
    ];
    let mut nodes = DATA + 0x24_000;
    for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
        let pool = DATA + 0x20_000 + 0x100 * i as u32;
        for n in 0..16u32 {
            let node = nodes + block * n;
            c.bus.write32(node, if n == 15 { 0 } else { node + block });
        }
        c.bus.write32(pool + head, nodes);
        c.bus.write32(GAME_GP + slot, pool);
        nodes += block * 16;
    }
    // The EAFile request and handle pools (the file manager at 0x519ab0):
    // a few free slots each, so real callees that open or queue a file do not
    // spin forever in `EAFile_AllocRequest` / `EAFile_AllocFileHandle` on an
    // exhausted pool. Counts at +0 (requests, 0x30-byte slots behind +0x28)
    // and +4 (handles, 0x110-byte slots behind +0x2c); a zeroed slot is free.
    const FILE_SLOTS: u32 = 4;
    let (requests, handles) = (DATA + 0x3e_0000, DATA + 0x3e_8000);
    c.bus.write32(0x0051_9ab0, FILE_SLOTS);
    c.bus.write32(0x0051_9ab0 + 4, FILE_SLOTS);
    c.bus.write32(0x0051_9ab0 + 0x28, requests);
    c.bus.write32(0x0051_9ab0 + 0x2c, handles);
    c.g[A0][0] = 0x004f_f250;
    c.g[A1][0] = (DATA + 0x4_0000) as u64;
    c.g[A2][0] = 0x4_0000;
    c.g[RA][0] = super::RA as u64;
    c.pc = 0x0031_ff60;
    ssx3_ee::s10::DLHeap_InitArena(c);
    let mut patch = Vec::new();
    let mut i = 0;
    while i < len {
        if c.bus.ram[at + i] != before[i] {
            let start = i;
            while i < len && (c.bus.ram[at + i] != before[i] || i - start < 1) {
                i += 1;
            }
            patch.push((at + start, c.bus.ram[at + start..at + i].to_vec()));
        } else {
            i += 1;
        }
    }
    super::RT_PATCH.with(|p| *p.borrow_mut() = patch);
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

/// Puts the gated test fake `fake` in an override slot, remembering in `prev`
/// what the slot held (the body the fake runs when its gate is off).
///
/// Each fake is hooked once per process: later calls do nothing. A fake that
/// re-records `prev` after another fake was chained on top of it would make
/// the two fall back to each other, and a test reaching the slot with neither
/// gate set would loop forever. Hooked once, every chain follows install
/// order and ends at the generated body. Never put the old body back on drop
/// either (a fake chained on top meanwhile would be lost or loop).
pub fn hook(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu), prev: &std::sync::atomic::AtomicUsize) {
    use std::collections::HashSet;
    use std::sync::Mutex;
    static HOOKED: Mutex<Option<HashSet<usize>>> = Mutex::new(None);
    let mut hooked = HOOKED.lock().unwrap_or_else(|p| p.into_inner());
    if !hooked
        .get_or_insert_with(HashSet::new)
        .insert(fake as usize)
    {
        return;
    }
    // SAFETY: override slots are plain `fn` pointers; every test hooks them
    // under this lock.
    unsafe {
        prev.store(*slot as usize, std::sync::atomic::Ordering::Relaxed);
        *slot = fake;
    }
}

/// What a gated fake runs with its gate off: the body [`hook`] found in the
/// slot (`prev`), or `generated` if the fake was never hooked.
pub fn fall_back(c: &mut Cpu, prev: &std::sync::atomic::AtomicUsize, generated: fn(&mut Cpu)) {
    let p = prev.load(std::sync::atomic::Ordering::Relaxed);
    if p == 0 {
        return generated(c);
    }
    // SAFETY: `hook` stored a `fn(&mut Cpu)` read from an override slot.
    let f = unsafe { std::mem::transmute::<usize, fn(&mut Cpu)>(p) };
    f(c)
}

/// The `prev` cell of a fake that has no static of its own (fakes a macro
/// stamps out without naming one): one leaked cell per fake, for [`hook`] to
/// fill and [`fall_back`] to read. Fakes look it up on every gate-off call,
/// so the common path is a shared read lock.
pub fn prev_cell(fake: fn(&mut Cpu)) -> &'static std::sync::atomic::AtomicUsize {
    use std::collections::HashMap;
    use std::sync::atomic::AtomicUsize;
    use std::sync::RwLock;
    static CELLS: RwLock<Option<HashMap<usize, &'static AtomicUsize>>> = RwLock::new(None);
    let key = fake as usize;
    if let Some(cell) = CELLS
        .read()
        .unwrap_or_else(|p| p.into_inner())
        .as_ref()
        .and_then(|m| m.get(&key).copied())
    {
        return cell;
    }
    let mut cells = CELLS.write().unwrap_or_else(|p| p.into_inner());
    *cells
        .get_or_insert_with(HashMap::new)
        .entry(key)
        .or_insert_with(|| Box::leak(Box::new(AtomicUsize::new(0))))
}

#[cfg(test)]
mod machine_rt_tests {
    use super::*;

    /// `machine_rt` leaves the runtime a real allocating callee needs.
    #[test]
    fn machine_rt_sets_up_the_runtime() {
        let mut c = super::super::fresh();
        machine_rt(&mut c, 3, 0x77, 0);
        assert_eq!(c.g[GP][0], GAME_GP as u64);
        assert_eq!(c.bus.read32(GAME_GP + 0xdd8), 1, "pool allocator enabled");
        assert_eq!(
            c.bus.read32(GAME_GP.wrapping_sub(0x1f04)),
            DATA + 0x30_0000,
            "BELibrary dummy"
        );
        assert_ne!(
            c.bus.read32(GAME_GP + 0x2a70),
            0,
            "size-class pool installed"
        );
    }
}
