//! Staging module oc1419 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1419`.

use crate::call::{guest, tail_wrapper, Callee, Target};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `DLHeap_Free` (`0x0031ed60`).
    pub(crate) const DLHEAP_FREE: Callee = Callee {
        addr: 0x0031_ed60,
        f: ee::s10::DLHeap_Free,
    };

    /// `DLHeap_Malloc` (`0x0031e818`).
    pub(crate) const DLHEAP_MALLOC: Callee = Callee {
        addr: 0x0031_e818,
        f: ee::s10::DLHeap_Malloc,
    };
}

/// `int PS2ExecMan_WaitSema(PS2ExecMan *man)` (0x31aaf0): wait on the
/// semaphore id at `man + 0xc`, through the kernel's `WaitSema`.
pub fn ps2_exec_man_wait_sema(c: &mut Cpu) {
    tail_wrapper(c, Some(0xc), Target::Table(0x0042_3de0), 0x0031_ab00);
}

/// `void DLHeap_Free_Wrapper(DLHeap *heap, void *ptr)` (0x31f7f0): free
/// `ptr` back to `heap` through `DLHeap_Free`.
pub fn dlheap_free_wrapper(c: &mut Cpu) {
    use callee::*;
    tail_wrapper(c, None, Target::Direct(DLHEAP_FREE), 0x0031_f800);
}

/// `void *DLHeap_Malloc_Wrapper(DLHeap *heap, unsigned int bytes)`
/// (0x320038): allocate `bytes` out of `heap` through `DLHeap_Malloc`.
pub fn dlheap_malloc_wrapper(c: &mut Cpu) {
    use callee::*;
    tail_wrapper(c, None, Target::Direct(DLHEAP_MALLOC), 0x0032_0048);
}

/// `int PS2ExecMan_v06(void)` (0x320b08): return the calling thread's id
/// through the kernel's `GetThreadId`.
pub fn ps2_exec_man_v06(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0042_3c90), 0x0032_0b18);
}

/// `void VD_TextureSink_SetFrameRate(VD_TextureSink *sink)` (0x254400):
/// store the incoming frame-rate floats (`f12`, `f13`) at `sink + 0x4c`
/// and `sink + 0x50`.
pub fn vd_texture_sink_set_frame_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.swc1(13, (c.g[A0][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x4c));
        c.pc = t;
        Ok(())
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_aaf0, ps2_exec_man_wait_sema, "PS2ExecMan_WaitSema"),
    (0x0031_f7f0, dlheap_free_wrapper, "DLHeap_Free_Wrapper"),
    (0x0032_0038, dlheap_malloc_wrapper, "DLHeap_Malloc_Wrapper"),
    (0x0032_0b08, ps2_exec_man_v06, "PS2ExecMan_v06"),
    (
        0x0025_4400,
        vd_texture_sink_set_frame_rate,
        "VD_TextureSink_SetFrameRate",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_paths;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate for the `WaitSema` / `GetThreadId` fakes below: the syscalls run
    /// for real unless a test plants this magic word.
    const CTRL: u32 = DATA + 0x1f00;
    const MAGIC: u32 = 0x5711_5e4d;

    static PREV_WAIT_SEMA: AtomicUsize = AtomicUsize::new(0);
    static PREV_GET_THREAD_ID: AtomicUsize = AtomicUsize::new(0);

    /// The fake kernel call: log the call like `fakes::fake_call` (so the
    /// strict-args check sees the wrapper's call arguments), then pin the
    /// cost to the real syscall's 4 cycles — the fake's hash-based cost
    /// would otherwise make the wrapper's cycle total seed-varying.
    fn fake_kernel_call(c: &mut Cpu, addr: u32) {
        let before = c.cycles;
        fakes::fake_call(c, addr);
        c.cycles = before + 4;
    }

    fn fake_wait_sema(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_WAIT_SEMA, ee::s15::f_00423de0__generated);
        }
        fake_kernel_call(c, 0x0042_3de0);
    }

    fn fake_get_thread_id(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_GET_THREAD_ID, ee::s15::f_00423c90__generated);
        }
        fake_kernel_call(c, 0x0042_3c90);
    }

    fn install_kernel_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            fake_wait_sema,
            &PREV_WAIT_SEMA,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423C90),
            fake_get_thread_id,
            &PREV_GET_THREAD_ID,
        );
    }

    /// A 256 KiB dlmalloc arena at `ARENA` managed through the heap header
    /// at `HEAP`; returns the header. Mirrors the `DLHeap` fixtures in
    /// `game/bx/cubicspline.rs`.
    fn heap_arena(c: &mut Cpu) -> u32 {
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        let ra = c.g[RA][0];
        for (i, a) in [HEAP as u64, ARENA as u64, 0x4_0000].iter().enumerate() {
            c.g[[A0, A1, A2][i]][0] = *a;
        }
        c.g[RA][0] = 0x0000_7770;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, 0x0000_7770, "InitArena did not return");
        c.g[RA][0] = ra;
        HEAP
    }

    /// Allocate `n` bytes out of `heap` for real; the setup asserts every
    /// fixture block lands, so a silent arena failure cannot vacate the
    /// test.
    fn heap_alloc(c: &mut Cpu, heap: u32, n: u64) -> u32 {
        c.g[A0][0] = heap as u64;
        c.g[A1][0] = n;
        c.g[RA][0] = 0x0000_7770;
        c.pc = 0x0031_e818;
        ee::s10::DLHeap_Malloc(c);
        assert_eq!(c.pc, 0x0000_7770, "Malloc did not return");
        let p = c.g[V0][0] as u32;
        assert_ne!(p, 0, "fixture malloc failed");
        p
    }

    /// `PS2ExecMan_WaitSema`: the sema id comes from `man + 0xc` (varied
    /// per seed); the fake `WaitSema` returns a hash `V0`, so the wrapper's
    /// return path runs with a distinctive value on both sides.
    #[test]
    fn ps2_exec_man_wait_sema_matches() {
        check_paths(
            ee::s10::f_0031aaf0__generated,
            ps2_exec_man_wait_sema,
            300,
            &[11],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_aaf0, 0);
                install_kernel_fakes();
                c.bus.write32(CTRL, MAGIC);
                let man = DATA + 0x100 + (r.next_u32() & 0xf0);
                c.bus.write32(man + 0xc, 0x1000 + seed);
                c.g[A0][0] = man as u64;
                c.pc = 0x0031_aaf0;
            },
        );
    }

    /// `DLHeap_Free_Wrapper`: free one live block out of a real arena (plus
    /// a null free on every fourth seed), so the free merges with live
    /// neighbours on both sides.
    #[test]
    fn dlheap_free_wrapper_matches() {
        check(
            ee::s10::f_0031f7f0__generated,
            dlheap_free_wrapper,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_f7f0, 0);
                let heap = heap_arena(c);
                let ra = c.g[RA][0];
                let mut ptrs = Vec::new();
                for i in 0..3 {
                    ptrs.push(heap_alloc(
                        c,
                        heap,
                        0x20 + (r.next_u32() % 0x400) as u64 + i * 0x111,
                    ));
                }
                let target = if seed % 4 == 3 {
                    0
                } else {
                    ptrs[(seed as usize) % ptrs.len()]
                };
                c.g[A0][0] = heap as u64;
                c.g[A1][0] = target as u64;
                c.g[RA][0] = ra;
                c.pc = 0x0031_f7f0;
            },
        );
    }

    /// `DLHeap_Malloc_Wrapper`: allocate out of a real arena, sizes from 0
    /// to beyond the arena (the failure returns null on both sides).
    #[test]
    fn dlheap_malloc_wrapper_matches() {
        check(
            ee::s10::f_00320038__generated,
            dlheap_malloc_wrapper,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0038, 0);
                let heap = heap_arena(c);
                let ra = c.g[RA][0];
                for _ in 0..seed % 3 {
                    heap_alloc(c, heap, (r.next_u32() % 0x300) as u64);
                }
                c.g[A0][0] = heap as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 0x10_0000 + (r.next_u32() % 0x1000) as u64,
                    _ => (r.next_u32() % 0x2000) as u64,
                };
                c.g[RA][0] = ra;
                c.pc = 0x0032_0038;
            },
        );
    }

    /// `PS2ExecMan_v06`: the fake `GetThreadId` returns a hash `V0`.
    #[test]
    fn ps2_exec_man_v06_matches() {
        check_paths(
            ee::s10::f_00320b08__generated,
            ps2_exec_man_v06,
            300,
            &[11],
            &|c, seed| {
                machine(c, seed, 0x0032_0b08, 0);
                install_kernel_fakes();
                c.bus.write32(CTRL, MAGIC);
                c.pc = 0x0032_0b08;
            },
        );
    }

    /// `VD_TextureSink_SetFrameRate`: distinctive nonzero floats land at
    /// `sink + 0x4c` / `sink + 0x50`.
    #[test]
    fn vd_texture_sink_set_frame_rate_matches() {
        check_paths(
            ee::s06::f_00254400__generated,
            vd_texture_sink_set_frame_rate,
            300,
            &[3],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0025_4400, 0);
                let sink = DATA + 0x100 + (r.next_u32() & 0xf0);
                for i in 0..8 {
                    c.bus.write32(sink + 4 * i, 0x1111_1111 * (i + 1));
                }
                c.f[12] = 0x3f80_0000 + seed;
                c.f[13] = 0x4000_0000 + seed * 3;
                c.g[A0][0] = sink as u64;
                c.pc = 0x0025_4400;
            },
        );
    }
}
