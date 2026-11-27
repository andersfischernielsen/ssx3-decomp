//! Staging module oc1418 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1418`.

use crate::call::{call_ptr, guest, tail_wrapper, Callee, Frame, Target};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `MD5_HashString64` (`0x00317690`).
    pub(crate) const MD5_HASH_STRING64: Callee = Callee {
        addr: 0x0031_7690,
        f: ee::s10::f_00317690,
    };

    /// `DLHeap_IAlloc` (`0x0031f858`).
    pub(crate) const DLHEAP_IALLOC: Callee = Callee {
        addr: 0x0031_f858,
        f: ee::s10::f_0031f858,
    };

    /// `MEM_Free` (`0x002523a8`).
    pub(crate) const MEM_FREE: Callee = Callee {
        addr: 0x0025_23a8,
        f: ee::s06::f_002523a8,
    };

    /// `VidEnginePlayer_StaticInit` (`0x00253b58`).
    pub(crate) const VID_ENGINE_PLAYER_STATIC_INIT: Callee = Callee {
        addr: 0x0025_3b58,
        f: ee::s06::VidEnginePlayer_StaticInit,
    };

    /// `VD_StaticInit` (`0x00254e60`).
    pub(crate) const VD_STATIC_INIT: Callee = Callee {
        addr: 0x0025_4e60,
        f: ee::s06::VD_StaticInit,
    };

    /// `VD_TextureSink_DrawFrame` (`0x002548d0`).
    pub(crate) const VD_TEXTURE_SINK_DRAW_FRAME: Callee = Callee {
        addr: 0x0025_48d0,
        f: ee::s06::VD_TextureSink_DrawFrame,
    };
}

/// `int PS2ExecMan_ForwardSlot12(PS2ExecMan *man)` (0x320a90): tail-call the
/// slot-12 handler through the object's slot table (the target lives at
/// `*(*(man + 8) + 8) + 0x64`; the argument is `*(man + 8)` plus the signed
/// half at `+0x60`).
pub fn ps2_exec_man_forward_slot12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0032_0ab0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *DLHeap_IndependentCalloc(DLHeap *heap, ...)` (0x31f810): allocate
/// chunks through `DLHeap_IAlloc` with zeroing (the `zero` flag is 3).
pub fn dlheap_independent_calloc(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = c.g[A3][0];
        c.sw(frame.at(c, 0), c.g[A2][0]);
        c.g[A3][0] = 0x3;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        DLHEAP_IALLOC.call(c, 0x0031_f82c)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void *DLHeap_IndependentComalloc(DLHeap *heap, ...)` (0x31f838):
/// allocate chunks through `DLHeap_IAlloc` without zeroing (the `zero` flag
/// is 0); the caller's chunk array passes through in `a3` (via `t0`), the
/// per-chunk sizes in `a2`.
pub fn dlheap_independent_comalloc(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0u64;
        DLHEAP_IALLOC.call(c, 0x0031_f84c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *VD_Free(void *p)` (0x255750): free `p` through the video free hook
/// (the function pointer at `0x509434`, reached as `*(0x510000 - 0x6bcc)`).
pub fn vd_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x510000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
        frame.save_ra(c, 0);
        let t = c.g[V0][0] as u32;
        call_ptr(c, t, 0x0025_5768)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *VD_Free_2(void *p)` (0x255778): free `p` through the video free
/// hook, as `VD_Free` (a second call site).
pub fn vd_free_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = 0x510000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
        frame.save_ra(c, 0);
        let t = c.g[V0][0] as u32;
        call_ptr(c, t, 0x0025_5790)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_GlobalCtors(void)` (0x254330): run the player's
/// static initializer with `(1, 0xffff)`, publishing its default tables.
pub fn vid_engine_player_global_ctors(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        VID_ENGINE_PLAYER_STATIC_INIT.call(c, 0x0025_4344)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VD_GlobalCtors(void)` (0x2557c0): run the video static initializer
/// with `(1, 0xffff)`, publishing its default tables.
pub fn vd_global_ctors(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        VD_STATIC_INIT.call(c, 0x0025_57d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MD5_HashString64Discard(const char *s)` (0x3176f0): run the 64-bit
/// MD5 string hash over `s` into stack scratch (the digest is discarded).
pub fn md5_hash_string64_discard(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        MD5_HASH_STRING64.call(c, 0x0031_7704)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEngine_MemFree(void *p)` (0x253af8): free `p` through `MEM_Free`.
pub fn vid_engine_mem_free(c: &mut Cpu) {
    use callee::*;
    tail_wrapper(c, None, Target::Direct(MEM_FREE), 0x0025_3b08);
}

/// `void VD_TextureSink_Redraw(VD_TextureSink *sink)` (0x254da0): redraw the
/// sink through `VD_TextureSink_DrawFrame`.
pub fn vd_texture_sink_redraw(c: &mut Cpu) {
    use callee::*;
    tail_wrapper(
        c,
        None,
        Target::Direct(VD_TEXTURE_SINK_DRAW_FRAME),
        0x0025_4db0,
    );
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0032_0a90,
        ps2_exec_man_forward_slot12,
        "PS2ExecMan_ForwardSlot12",
    ),
    (
        0x0031_f810,
        dlheap_independent_calloc,
        "DLHeap_IndependentCalloc",
    ),
    (0x0025_5750, vd_free, "VD_Free"),
    (0x0025_5778, vd_free_2, "VD_Free_2"),
    (
        0x0025_4330,
        vid_engine_player_global_ctors,
        "VidEnginePlayer_GlobalCtors",
    ),
    (0x0025_57c0, vd_global_ctors, "VD_GlobalCtors"),
    (
        0x0031_76f0,
        md5_hash_string64_discard,
        "MD5_HashString64Discard",
    ),
    (
        0x0031_f838,
        dlheap_independent_comalloc,
        "DLHeap_IndependentComalloc",
    ),
    (0x0025_3af8, vid_engine_mem_free, "VidEngine_MemFree"),
    (0x0025_4da0, vd_texture_sink_redraw, "VD_TextureSink_Redraw"),
];

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::containers::tests::{free_target, mutex_state};
    use crate::containers::MEM_MUTEX;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{elf_image, DATA};

    /// A 256 KiB dlmalloc arena at `ARENA` managed through the heap header
    /// at `HEAP`; returns the header (same shape as the `oc1419` fixture).
    pub(crate) fn heap_arena(c: &mut Cpu) -> u32 {
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

    /// `PS2ExecMan_ForwardSlot12`: the slot table yields the fake handler
    /// with a seed-steered signed offset, so the forwarded argument varies
    /// on both sides.
    #[test]
    fn ps2_exec_man_forward_slot12_matches() {
        check(
            ee::s10::f_00320a90__generated,
            ps2_exec_man_forward_slot12,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0a90, 0);
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                c.bus.write32(OBJ + 8, TAB1);
                c.bus.write32(TAB1 + 8, TAB2);
                let off = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 0x4000) as u16,
                    _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                };
                c.bus.write16(TAB2 + 0x60, off);
                c.bus.write32(TAB2 + 0x64, FAKE);
                c.g[A0][0] = OBJ as u64;
                c.pc = 0x0032_0a90;
            },
        );
    }

    /// `DLHeap_IndependentCalloc`: real `IAlloc` over a real arena; sizes
    /// 0, small and near-arena, with the element size in `a2` and the
    /// chunk array (filled by `IAlloc`) in `a3`.
    #[test]
    fn dlheap_independent_calloc_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x0031_f810, 0);
            let heap = heap_arena(c);
            let ra = c.g[RA][0];
            c.g[A0][0] = heap as u64;
            c.g[A1][0] = match seed % 4 {
                0 => 0,
                1 => 8 + (r.next_u32() % 0x100) as u64,
                2 => 0x1000 + (r.next_u32() % 0x1000) as u64,
                _ => (r.next_u32() % 0x2000) as u64,
            };
            c.g[A2][0] = [4u64, 8, 16, 32][seed as usize % 4];
            for i in 0..8 {
                c.bus.write32(DATA + 0x3000 + 4 * i, 0);
            }
            c.g[A3][0] = (DATA + 0x3000) as u64;
            c.g[RA][0] = ra;
            c.pc = 0x0031_f810;
        };
        check(
            ee::s10::f_0031f810__generated,
            dlheap_independent_calloc,
            60,
            &setup,
        );
        // The success path must run: seed 1 asks for a small live
        // allocation, so `IAlloc` fills the chunk array (pre-fix every
        // n > 0 exceeded the arena and only the failed exit ran).
        let mut c = crate::difftest::fresh();
        setup(&mut c, 1);
        crate::difftest::run_generated(&mut c, ee::s10::f_0031f810__generated);
        assert_ne!(
            c.bus.read32(DATA + 0x3000),
            0,
            "seed 1 left the chunk array empty: success path not reached"
        );
    }

    /// `DLHeap_IndependentComalloc`: as calloc, without zeroing; the
    /// per-chunk sizes array in `a2`, the chunk array in `a3`.
    #[test]
    fn dlheap_independent_comalloc_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x0031_f838, 0);
            let heap = heap_arena(c);
            let ra = c.g[RA][0];
            c.g[A0][0] = heap as u64;
            c.g[A1][0] = match seed % 4 {
                0 => 0,
                1 => 8 + (r.next_u32() % 0x100) as u64,
                2 => 0x1000 + (r.next_u32() % 0x1000) as u64,
                _ => (r.next_u32() % 0x2000) as u64,
            };
            c.g[A2][0] = (DATA + 0x3100) as u64;
            for i in 0..0x2000 {
                c.bus.write32(DATA + 0x3100 + 4 * i, 8 + (i % 4) * 8);
            }
            for i in 0..8 {
                c.bus.write32(DATA + 0xc000 + 4 * i, 0);
            }
            c.g[A3][0] = (DATA + 0xc000) as u64;
            c.g[RA][0] = ra;
            c.pc = 0x0031_f838;
        };
        check(
            ee::s10::f_0031f838__generated,
            dlheap_independent_comalloc,
            60,
            &setup,
        );
        // The success path must run: seed 1 asks for a small live
        // allocation, so `IAlloc` fills the chunk array (pre-fix every
        // n > 0 exceeded the arena and only the failed exit ran).
        let mut c = crate::difftest::fresh();
        setup(&mut c, 1);
        crate::difftest::run_generated(&mut c, ee::s10::f_0031f838__generated);
        assert_ne!(
            c.bus.read32(DATA + 0xc000),
            0,
            "seed 1 left the chunk array empty: success path not reached"
        );
    }

    /// `VD_Free` and `VD_Free_2`: the hook word points at the fake, whose
    /// hash result varies with the freed pointer on both sides.
    #[test]
    fn vd_free_matches() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00255750__generated as fn(&mut Cpu),
                vd_free as fn(&mut Cpu),
                0x0025_5750,
            ),
            (
                ee::s06::f_00255778__generated as fn(&mut Cpu),
                vd_free_2 as fn(&mut Cpu),
                0x0025_5778,
            ),
        ] {
            let setup = |c: &mut Cpu, seed: u32| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.bus.write32(OPEN_ALLOC_HOOK + 4, FAKE);
                c.g[A0][0] = (DATA + 0x100 + (r.next_u32() & 0xfff0)) as u64;
                c.pc = addr;
            };
            check(gen, hand, 300, &setup);
            assert_no_null_hook_calls(gen, 300, &setup);
        }
    }

    /// The video hook words (`0x510000 - 0x6bd0`).
    pub(crate) const OPEN_ALLOC_HOOK: u32 = 0x0050_9430;

    /// The video hook words must actually be reached: run the setup
    /// through the generated body and require that no call fell through
    /// to address 0. (The pre-fix fixtures planted the hook at 0x4a9430 /
    /// 0x4a9434 instead of 0x509430 / 0x509434, so both sides called
    /// address 0 and agreed by accident.)
    fn assert_no_null_hook_calls(gen: fn(&mut Cpu), seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, gen);
            let targets = crate::difftest::fakes::logged_targets(&mut c);
            assert!(
                targets.iter().all(|&t| t != 0),
                "seed {seed}: call to address 0 (hook word not planted): {targets:#x?}"
            );
        }
    }
    /// `VidEnginePlayer_GlobalCtors` and `VD_GlobalCtors`: the full
    /// `(1, 0xffff)` table init runs for real over the game image, so the
    /// float tables land identically on both sides.
    #[test]
    fn global_ctors_matches() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00254330__generated as fn(&mut Cpu),
                vid_engine_player_global_ctors as fn(&mut Cpu),
                0x0025_4330,
            ),
            (
                ee::s06::f_002557c0__generated as fn(&mut Cpu),
                vd_global_ctors as fn(&mut Cpu),
                0x0025_57c0,
            ),
        ] {
            check(gen, hand, 60, &|c, seed| {
                machine(c, seed, addr as u64, 0);
                elf_image(c);
                c.pc = addr;
            });
        }
    }

    /// `MD5_HashString64Discard`: strings from empty to 64 bytes cross the
    /// padding boundary, so `MD5_Update`/`MD5_Final` take both arms on both
    /// sides; the digest lands in stack scratch.
    #[test]
    fn md5_hash_string64_discard_matches() {
        check(
            ee::s10::f_003176f0__generated,
            md5_hash_string64_discard,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_76f0, 0);
                let len = match seed % 6 {
                    0 => 0,
                    1 => 55,
                    2 => 56,
                    3 => 57,
                    4 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let s = DATA + 0x100;
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                c.g[A0][0] = s as u64;
                c.pc = 0x0031_76f0;
            },
        );
    }

    /// `VidEngine_MemFree`: null (the early return, 14 cycles) or a live
    /// heap / pooled block through the real free path with the allocator
    /// mutex fixture.
    #[test]
    fn vid_engine_mem_free_matches() {
        check(
            ee::s06::f_00253af8__generated,
            vid_engine_mem_free,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0025_3af8, 0);
                mutex_state(c, &mut r, MEM_MUTEX);
                let p = free_target(c, &mut r);
                c.g[A0][0] = if seed % 3 == 2 { 0 } else { p };
                c.pc = 0x0025_3af8;
            },
        );
        crate::difftest::require_paths(ee::s06::f_00253af8__generated, 100, &[14], &|c, seed| {
            let mut r = machine(c, seed, 0x0025_3af8, 0);
            mutex_state(c, &mut r, MEM_MUTEX);
            let p = free_target(c, &mut r);
            c.g[A0][0] = if seed % 3 == 2 { 0 } else { p };
            c.pc = 0x0025_3af8;
        });
    }

    /// `VD_TextureSink_Redraw`: the sink's method table points every virtual
    /// call at the fake, and the row/column counts stay tiny (0 skips the
    /// loop, otherwise 1..3), so the draw runs its full dispatch on both
    /// sides without running away.
    #[test]
    fn vd_texture_sink_redraw_matches() {
        // Distinctive from FAKE: with FAKE slots the assert
        // passed with or without the gp plant, so it could not tell
        // the plants apart. VT_TARGET pins the fixture VT itself:
        // removing SINK+8 (the GM pointer this path reads) logs no
        // VT_TARGET and fails.
        const VT_TARGET: u32 = 0x0000_4002;
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_4da0, 0);
            elf_image(c);
            const SINK: u32 = DATA + 0x100;
            const MID: u32 = DATA + 0x200;
            const VT: u32 = DATA + 0x300;
            const ROWS: u32 = DATA + 0x800;
            const SCRATCH: u32 = DATA + 0x900;
            for i in 0..0x40 {
                c.bus.write32(SINK + 4 * i, 0x2222_0000 + i);
            }
            c.bus.write32(SINK + 8, MID);
            // The graphics manager DrawFrame reads (without
            // this it reads the game image, not the fixture MID).
            c.bus.write32(crate::difftest::GAME_GP + 0x2a90, MID);
            // Row count: 0 skips the outer loop, else {2, 3} rows.
            c.bus
                .write32(SINK + 0x1c, if seed % 3 == 0 { 0 } else { 1 + seed % 3 });
            // Column count: 0 skips the inner loop, else {2}.
            c.bus
                .write32(SINK + 0x20, if seed % 2 == 0 { 0 } else { 1 + seed % 2 });
            c.bus.write32(SINK + 0x3c, 0x4444_0000 + seed * 7);
            // Row base: real RAM prefilled with distinctive words.
            c.bus.write32(SINK + 0x14, ROWS);
            for i in 0..0x10 {
                c.bus.write32(ROWS + 4 * i, 0x5555_0000 + i);
            }
            c.bus.write32(MID + 0x10d8, VT);
            for i in 0..0x90 {
                c.bus.write32(VT + 4 * i, VT_TARGET);
            }
            // The scratch pointer the +0xe84 chain loads, with a
            // distinctive word at the slot it reads and writes.
            c.bus.write32(MID + 0xe84, SCRATCH);
            c.bus.write32(SCRATCH + 8, 0x6666_0000 + seed);
            c.g[A0][0] = SINK as u64;
            c.pc = 0x0025_4da0;
        };
        check(
            ee::s06::f_00254da0__generated,
            vd_texture_sink_redraw,
            100,
            &setup,
        );
        // The drawn cells must run through the fixture VT (VT_TARGET):
        // some seed logs a call targeting VT_TARGET (SINK+8 removed
        // logs none).
        let mut saw_vt = false;
        for seed in 0..crate::difftest::seed_count(100) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s06::f_00254da0__generated);
            if crate::difftest::fakes::logged_targets(&mut c).contains(&VT_TARGET) {
                saw_vt = true;
            }
        }
        assert!(saw_vt, "no seed drew through the fixture VT");
    }
}
