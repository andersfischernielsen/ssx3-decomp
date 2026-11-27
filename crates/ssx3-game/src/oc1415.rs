//! Staging module oc1415 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1415`.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `DLHeap_Consolidate` (`0x0031eee8`).
    pub(crate) const DLHEAP_CONSOLIDATE: Callee = Callee {
        addr: 0x0031_eee8,
        f: ee::s10::DLHeap_Consolidate,
    };

    /// `DLHeap_Memalign` (`0x0031f2c8`).
    pub(crate) const DLHEAP_MEMALIGN: Callee = Callee {
        addr: 0x0031_f2c8,
        f: ee::s10::DLHeap_Memalign,
    };

    /// `MD5_Init` (`0x0031d368`).
    pub(crate) const MD5_INIT: Callee = Callee {
        addr: 0x0031_d368,
        f: ee::s10::MD5_Init,
    };

    /// `MD5_Update` (`0x0031d3a8`).
    pub(crate) const MD5_UPDATE: Callee = Callee {
        addr: 0x0031_d3a8,
        f: ee::s10::MD5_Update,
    };

    /// `MD5_Final` (`0x0031d4d0`).
    pub(crate) const MD5_FINAL: Callee = Callee {
        addr: 0x0031_d4d0,
        f: ee::s10::MD5_Final,
    };

    /// `Serialize_WriteU32` (`0x002d21d0`).
    pub(crate) const SERIALIZE_WRITE_U32: Callee = Callee {
        addr: 0x002d_21d0,
        f: ee::s09::Serialize_WriteU32,
    };

    /// `DLHeap_SysTrim` (`0x0031e6d8`).
    pub(crate) const DLHEAP_SYSTRIM: Callee = Callee {
        addr: 0x0031_e6d8,
        f: ee::s10::DLHeap_SysTrim,
    };

    /// `VidEnginePlayer_Close` (`0x002535f8`).
    pub(crate) const VIDENGINEPLAYER_CLOSE: Callee = Callee {
        addr: 0x0025_35f8,
        f: ee::s06::VidEnginePlayer_Close,
    };

    /// `VidEnginePlayer_InitOpenParams` (`0x00253390`).
    pub(crate) const VIDENGINEPLAYER_INIT_OPEN_PARAMS: Callee = Callee {
        addr: 0x0025_3390,
        f: ee::s06::VidEnginePlayer_InitOpenParams,
    };

    /// `VidEnginePlayer_Open` (`0x002534a8`).
    pub(crate) const VIDENGINEPLAYER_OPEN: Callee = Callee {
        addr: 0x0025_34a8,
        f: ee::s06::VidEnginePlayer_Open,
    };
}

/// `void *DLHeap_Pvalloc(DLHeap *heap, unsigned int bytes)` (0x31fb00):
/// page-rounded allocation through `DLHeap_Memalign` (consolidating first
/// when the heap has no fast chunk pending).
pub fn dlheap_pvalloc(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[V0][0] != 0u64;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            DLHEAP_CONSOLIDATE.call(c, 0x0031_fb30)?;
        }
        c.cycles += 13;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x35c));
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
        c.g[A2][0] = c.g[A2][0] & c.g[V0][0];
        DLHEAP_MEMALIGN.call(c, 0x0031_fb4c)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `uint64_t MD5_HashString64(const char *s, void *out)` (0x317690): hash
/// `s` with MD5, writing the digest to the stack context and its low word
/// to `out`, returning the low digest word.
pub fn md5_hash_string64(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, 17, 0x80);
        frame.save(c, 16, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        MD5_INIT.call(c, 0x0031_76b0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6810, 0x0031_76b8)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S0][0];
        MD5_UPDATE.call(c, 0x0031_76c8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(frame.at(c, 0x60));
        MD5_FINAL.call(c, 0x0031_76d4)?;
        c.g[V0][0] = c.ld(frame.at(c, 0x60));
        frame.restore(c, 16, 0x90);
        c.sd(c.g[S1][0] as u32, c.g[V0][0]);
        frame.restore(c, 17, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `int Serialize_WriteVec3(void *stream, float *v)` (0x2d2248): write the
/// 3 words of `v` (12 bytes).
pub fn serialize_write_vec3(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_2248 => {
                    c.cycles += 10;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, 16, 0x30);
                    frame.save(c, 17, 0x20);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x10);
                    c.g[S1][0] = 0x2;
                    frame.save_ra(c, 0);
                    c.g[S2][0] = c.g[A0][0];
                    c.lwc1(12, c.g[S0][0] as u32);
                    pc = 0x002d_2270;
                    continue;
                }
                0x002d_2270 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    SERIALIZE_WRITE_U32.call(c, 0x002d_2280)?;
                    if (c.g[S1][0] as i64) >= 0 {
                        c.lwc1(12, c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_2270);
                        }
                        pc = 0x002d_2270;
                        continue;
                    }
                    c.cycles += 7;
                    let frame = Frame { size: 0x40 };
                    c.g[V0][0] = 0xc;
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore(c, 18, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_2248, 0x002d_22a4);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_WriteVec4(void *stream, float *v)` (0x2d22a8): write the
/// 4 words of `v` (16 bytes).
pub fn serialize_write_vec4(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_22a8 => {
                    c.cycles += 10;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, 16, 0x30);
                    frame.save(c, 17, 0x20);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x10);
                    c.g[S1][0] = 0x3;
                    frame.save_ra(c, 0);
                    c.g[S2][0] = c.g[A0][0];
                    c.lwc1(12, c.g[S0][0] as u32);
                    pc = 0x002d_22d0;
                    continue;
                }
                0x002d_22d0 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    SERIALIZE_WRITE_U32.call(c, 0x002d_22e0)?;
                    if (c.g[S1][0] as i64) >= 0 {
                        c.lwc1(12, c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_22d0);
                        }
                        pc = 0x002d_22d0;
                        continue;
                    }
                    c.cycles += 7;
                    let frame = Frame { size: 0x40 };
                    c.g[V0][0] = 0x10;
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore(c, 18, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_22a8, 0x002d_2304);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_WriteMatrix(void *stream, float *m)` (0x2d2308): write the
/// 16 words of `m` (64 bytes).
pub fn serialize_write_matrix(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d_2308 => {
                    c.cycles += 10;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, 16, 0x30);
                    frame.save(c, 17, 0x20);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x10);
                    c.g[S1][0] = 0xf;
                    frame.save_ra(c, 0);
                    c.g[S2][0] = c.g[A0][0];
                    c.lwc1(12, c.g[S0][0] as u32);
                    pc = 0x002d_2330;
                    continue;
                }
                0x002d_2330 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    SERIALIZE_WRITE_U32.call(c, 0x002d_2340)?;
                    if (c.g[S1][0] as i64) >= 0 {
                        c.lwc1(12, c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_2330);
                        }
                        pc = 0x002d_2330;
                        continue;
                    }
                    c.cycles += 7;
                    let frame = Frame { size: 0x40 };
                    c.g[V0][0] = 0x40;
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore(c, 18, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_2308, 0x002d_2364);
                    return Ok(());
                }
            }
        }
    })
}

/// `int Serialize_WriteQuat(void *stream, float *q)` (0x2d2368): write the
/// 4 words of `q` (16 bytes).
pub fn serialize_write_quat(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0xc));
        SERIALIZE_WRITE_U32.call(c, 0x002d_2388)?;
        c.lwc1(12, c.g[S0][0] as u32);
        c.g[A0][0] = c.g[S1][0];
        SERIALIZE_WRITE_U32.call(c, 0x002d_2394)?;
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S1][0];
        SERIALIZE_WRITE_U32.call(c, 0x002d_23a0)?;
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S1][0];
        SERIALIZE_WRITE_U32.call(c, 0x002d_23ac)?;
        frame.restore(c, 16, 0x20);
        c.g[V0][0] = 0x10;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *DLHeap_Valloc(DLHeap *heap, unsigned int bytes)` (0x31faa8):
/// page-aligned allocation through `DLHeap_Memalign` (consolidating first
/// when the heap has no fast chunk pending).
pub fn dlheap_valloc(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[V0][0] != 0u64;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            DLHEAP_CONSOLIDATE.call(c, 0x0031_fad8)?;
        }
        c.cycles += 10;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x35c));
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        DLHEAP_MEMALIGN.call(c, 0x0031_fae8)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *DLHeap_Trim(DLHeap *heap, unsigned int pad)` (0x31fb68): release
/// the heap's top chunk back through `DLHeap_SysTrim`.
pub fn dlheap_trim(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 17, 0x20);
        c.g[S1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = c.g[S1][0];
        DLHEAP_CONSOLIDATE.call(c, 0x0031_fb90)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        DLHEAP_SYSTRIM.call(c, 0x0031_fba0)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int VidEnginePlayer_Reopen(VidEnginePlayer *player, void *params)`
/// (0x253460): close the player, reset the open params, and open it again
/// on `params`.
pub fn vid_engine_player_reopen(c: &mut Cpu) {
    use callee::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        VIDENGINEPLAYER_CLOSE.call(c, 0x0025_347c)?;
        c.g[A0][0] = c.g[SP][0];
        VIDENGINEPLAYER_INIT_OPEN_PARAMS.call(c, 0x0025_3484)?;
        c.sw(frame.at(c, 0), c.g[S1][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        VIDENGINEPLAYER_OPEN.call(c, 0x0025_3494)?;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_fb00, dlheap_pvalloc, "DLHeap_Pvalloc"),
    (0x0031_7690, md5_hash_string64, "MD5_HashString64"),
    (0x002d_2248, serialize_write_vec3, "Serialize_WriteVec3"),
    (0x002d_22a8, serialize_write_vec4, "Serialize_WriteVec4"),
    (0x002d_2308, serialize_write_matrix, "Serialize_WriteMatrix"),
    (0x002d_2368, serialize_write_quat, "Serialize_WriteQuat"),
    (0x0031_faa8, dlheap_valloc, "DLHeap_Valloc"),
    (0x0031_fb68, dlheap_trim, "DLHeap_Trim"),
    (
        0x0025_3460,
        vid_engine_player_reopen,
        "VidEnginePlayer_Reopen",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{require_paths, Rng, DATA};
    use crate::oc1418::tests::heap_arena;

    /// The `miss` hook with a fixed 4-cycle cost: the fake's hash-based cost
    /// would otherwise make single-path wrappers' totals seed-varying.
    fn norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        crate::difftest::fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// Allocate `n` bytes out of `heap` and free every other block, so the
    /// Consolidate guard and the trim paths see live, dead and top chunks.
    fn heap_traffic(c: &mut Cpu, heap: u32, r: &mut Rng, blocks: u32) {
        let ra = c.g[RA][0];
        let mut ptrs = Vec::new();
        for _ in 0..blocks {
            c.g[A0][0] = heap as u64;
            c.g[A1][0] = (0x20 + r.next_u32() % 0x800) as u64;
            c.g[RA][0] = 0x0000_7770;
            c.pc = 0x0031_e818;
            ee::s10::DLHeap_Malloc(c);
            assert_eq!(c.pc, 0x0000_7770);
            ptrs.push(c.g[V0][0] as u32);
        }
        for (i, &p) in ptrs.iter().enumerate() {
            if i % 2 == 0 {
                c.g[A0][0] = heap as u64;
                c.g[A1][0] = p as u64;
                c.g[RA][0] = 0x0000_7770;
                c.pc = 0x0031_ed60;
                ee::s10::DLHeap_Free(c);
                assert_eq!(c.pc, 0x0000_7770);
            }
        }
        c.g[RA][0] = ra;
    }

    /// `DLHeap_Pvalloc` (0x31fb00): page-rounded allocation out of a real
    /// arena, fresh or with traffic (the Consolidate guard reads the heap
    /// head either way).
    #[test]
    fn dlheap_pvalloc_matches() {
        check(
            ee::s10::f_0031fb00__generated,
            dlheap_pvalloc,
            60,
            &|c, seed| {
                pvalloc_fixture(c, seed, 0x0031_fb00, 0x0031_fb00);
            },
        );
        require_paths(
            ee::s10::f_0031fb00__generated,
            60,
            &[621, 643, 651, 1177],
            &|c, seed| {
                pvalloc_fixture(c, seed, 0x0031_fb00, 0x0031_fb00);
            },
        );
    }

    /// `DLHeap_Valloc` (0x31faa8): page-aligned allocation; same fixture
    /// classes as `Pvalloc`.
    #[test]
    fn dlheap_valloc_matches() {
        check(
            ee::s10::f_0031faa8__generated,
            dlheap_valloc,
            60,
            &|c, seed| {
                pvalloc_fixture(c, seed, 0x0031_faa8, 0x0031_faa8);
            },
        );
        require_paths(
            ee::s10::f_0031faa8__generated,
            60,
            &[605, 640, 648, 713, 1174],
            &|c, seed| {
                pvalloc_fixture(c, seed, 0x0031_faa8, 0x0031_faa8);
            },
        );
    }

    /// Shared fixture for the two page allocators.
    fn pvalloc_fixture(c: &mut Cpu, seed: u32, salt: u32, addr: u32) {
        let mut r = machine(c, seed, salt as u64, 0);
        let heap = heap_arena(c);
        if seed % 3 == 2 {
            heap_traffic(c, heap, &mut r, 2 + seed % 3);
        }
        let ra = c.g[RA][0];
        c.g[A0][0] = heap as u64;
        c.g[A1][0] = match seed % 6 {
            0 => 0,
            1 => 1,
            2 => 0x1000,
            3 => 0x1001,
            4 => 0x10000 + (r.next_u32() % 0x1000) as u64,
            _ => (r.next_u32() % 0x2000) as u64,
        };
        c.g[A2][0] = 0xcccc_0000 + seed as u64;
        c.g[A3][0] = 0xdddd_0000 + seed as u64;
        c.g[RA][0] = ra;
        c.pc = addr;
    }

    /// `DLHeap_Trim`: consolidate and trim a trafficked arena, or an idle
    /// one (nothing to release).
    #[test]
    fn dlheap_trim_matches() {
        check(
            ee::s10::f_0031fb68__generated,
            dlheap_trim,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_fb68, 0);
                let heap = heap_arena(c);
                if seed % 2 == 0 {
                    heap_traffic(c, heap, &mut r, 3);
                }
                let ra = c.g[RA][0];
                c.g[A0][0] = heap as u64;
                c.g[A1][0] = [0u64, 0x1000, 0x1_0000][seed as usize % 3];
                c.g[RA][0] = ra;
                c.pc = 0x0031_fb68;
            },
        );
        require_paths(
            ee::s10::f_0031fb68__generated,
            60,
            &[291, 324, 357, 899, 915],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_fb68, 0);
                let heap = heap_arena(c);
                if seed % 2 == 0 {
                    heap_traffic(c, heap, &mut r, 3);
                }
                let ra = c.g[RA][0];
                c.g[A0][0] = heap as u64;
                c.g[A1][0] = [0u64, 0x1000, 0x1_0000][seed as usize % 3];
                c.g[RA][0] = ra;
                c.pc = 0x0031_fb68;
            },
        );
    }

    /// `MD5_HashString64`: strings from empty across the padding boundary,
    /// hashed into a prefilled out word on both sides.
    #[test]
    fn md5_hash_string64_matches() {
        check(
            ee::s10::f_00317690__generated,
            md5_hash_string64,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7690, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let out = DATA + 0x100;
                let s = DATA + 0x200;
                for i in 0..4 {
                    c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
                }
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                c.g[A0][0] = out as u64;
                c.g[A1][0] = s as u64;
                c.pc = 0x0031_7690;
            },
        );
        require_paths(
            ee::s10::f_00317690__generated,
            100,
            &[
                1995, 2033, 2040, 2054, 2056, 2068, 2084, 2089, 2096, 2105, 2110, 2112, 2124, 2133,
                2152, 3052, 3586, 3593, 3635,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7690, 0);
                let len = match seed % 8 {
                    0 => 0,
                    1 => 1,
                    2 => 55,
                    3 => 56,
                    4 => 57,
                    5 => 63,
                    6 => 64,
                    _ => (r.next_u32() % 65) as usize,
                };
                let out = DATA + 0x100;
                let s = DATA + 0x200;
                for i in 0..4 {
                    c.bus.write32(out + 4 * i, 0xeeee_0000 + i);
                }
                for i in 0..len {
                    c.bus.write8(s + i as u32, 1 + (r.next_u32() % 95) as u8);
                }
                c.bus.write8(s + len as u32, 0);
                c.g[A0][0] = out as u64;
                c.g[A1][0] = s as u64;
                c.pc = 0x0031_7690;
            },
        );
    }

    /// A stream over a prefilled byte buffer plus a float array with
    /// seed-steered values (zeros, ones, edge floats, random bits).
    fn serialize_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, words: usize) -> (u32, u32) {
        const STREAM: u32 = DATA + 0x100;
        const BUF: u32 = DATA + 0x200;
        const VEC: u32 = DATA + 0x400;
        c.bus.write32(STREAM, BUF);
        for i in 0..0x40u32 {
            c.bus.write32(BUF + 4 * i, 0x7777_0000 + i);
        }
        for i in 0..words as u32 {
            c.bus.write32(
                VEC + 4 * i,
                match (seed + i) % 5 {
                    0 => 0,
                    1 => 0x3f80_0000,
                    2 => [0x7f80_0000, 0xff80_0000, 0x7fc0_0000, 0x0080_0000]
                        [(r.next_u32() % 4) as usize],
                    3 => 0x4049_0fdb,
                    _ => r.next_u32(),
                },
            );
        }
        (STREAM, VEC)
    }

    /// The vector writers: every word lands in the stream buffer on both
    /// sides, with the loop's event check covered by `check_events` below.
    /// `require_paths` pins the single total (3, 4, 16 iterations).
    #[test]
    fn serialize_write_vec3_matches() {
        check(
            ee::s09::f_002d2248__generated,
            serialize_write_vec3,
            60,
            &|c, seed| {
                vec_fixture(c, seed, 0x002d_2248, 3);
            },
        );
        require_paths(ee::s09::f_002d2248__generated, 60, &[80], &|c, seed| {
            vec_fixture(c, seed, 0x002d_2248, 3);
        });
    }

    /// `Serialize_WriteVec4`: 4 words, 16 bytes out.
    #[test]
    fn serialize_write_vec4_matches() {
        check(
            ee::s09::f_002d22a8__generated,
            serialize_write_vec4,
            60,
            &|c, seed| {
                vec_fixture(c, seed, 0x002d_22a8, 4);
            },
        );
        require_paths(ee::s09::f_002d22a8__generated, 60, &[101], &|c, seed| {
            vec_fixture(c, seed, 0x002d_22a8, 4);
        });
    }

    /// `Serialize_WriteMatrix`: 16 words, 64 bytes out.
    #[test]
    fn serialize_write_matrix_matches() {
        check(
            ee::s09::f_002d2308__generated,
            serialize_write_matrix,
            60,
            &|c, seed| {
                vec_fixture(c, seed, 0x002d_2308, 16);
            },
        );
        require_paths(ee::s09::f_002d2308__generated, 60, &[353], &|c, seed| {
            vec_fixture(c, seed, 0x002d_2308, 16);
        });
    }

    /// Shared fixture for the vector writers.
    fn vec_fixture(c: &mut Cpu, seed: u32, addr: u32, words: usize) {
        let mut r = machine(c, seed, addr as u64, 0);
        let (stream, vec) = serialize_fixture(c, &mut r, seed, words);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = vec as u64;
        c.pc = addr;
    }

    driven!(vec_gen, ee::s09::f_002d2248__generated);
    driven!(vec_hand, serialize_write_vec3);
    driven!(vec4_gen, ee::s09::f_002d22a8__generated);
    driven!(vec4_hand, serialize_write_vec4);
    driven!(mat_gen, ee::s09::f_002d2308__generated);
    driven!(mat_hand, serialize_write_matrix);

    /// The vector loops under delivered events. Each head (0x2d2270, 0x2d22d0,
    /// 0x2d2330) is structured: the port must carry on, never resume there.
    #[test]
    fn serialize_write_vecs_events() {
        for (gen, hand, addr, words, head) in [
            (
                vec_gen as fn(&mut Cpu),
                vec_hand as fn(&mut Cpu),
                0x002d_2248,
                3,
                0x002d_2270,
            ),
            (
                vec4_gen as fn(&mut Cpu),
                vec4_hand as fn(&mut Cpu),
                0x002d_22a8,
                4,
                0x002d_22d0,
            ),
            (
                mat_gen as fn(&mut Cpu),
                mat_hand as fn(&mut Cpu),
                0x002d_2308,
                16,
                0x002d_2330,
            ),
        ] {
            let resumes = check_events(gen, hand, 60, &|c, seed| {
                let mut r = machine(c, seed, addr as u64, 0);
                let (stream, vec) = serialize_fixture(c, &mut r, seed, words);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = vec as u64;
                schedule(c, &mut r, seed, addr)
            });
            assert!(
                resumes.get(&head).is_none(),
                "unwound at structured head {head:#x}: {resumes:x?}"
            );
        }
    }

    /// `Serialize_WriteQuat`: the four words land in order on both sides.
    #[test]
    fn serialize_write_quat_matches() {
        check(
            ee::s09::f_002d2368__generated,
            serialize_write_quat,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_2368, 0);
                let (stream, vec) = serialize_fixture(c, &mut r, seed, 4);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = vec as u64;
                c.pc = 0x002d_2368;
            },
        );
        require_paths(ee::s09::f_002d2368__generated, 100, &[83], &|c, seed| {
            let mut r = machine(c, seed, 0x002d_2368, 0);
            let (stream, vec) = serialize_fixture(c, &mut r, seed, 4);
            c.g[A0][0] = stream as u64;
            c.g[A1][0] = vec as u64;
            c.pc = 0x002d_2368;
        });
    }

    /// The video hook words (`0x510000 - 0x6bd0`).
    const OPEN_ALLOC_HOOK: u32 = 0x0050_9430;

    /// `VidEnginePlayer_Reopen`: close (quiet or loud), reset the params,
    /// and open again on the probe struct.
    #[test]
    fn vid_engine_player_reopen_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x0025_3460, 0);
            c.hooks.miss = norm_miss;
            const P: u32 = DATA + 0x100;
            const PARAMS: u32 = DATA + 0x400;
            for i in 0..0x20u32 {
                c.bus.write32(P + 4 * i, 0x8888_0000 + i);
            }
            for i in 0..8u32 {
                c.bus.write32(PARAMS + 4 * i, 0x9999_0000 + i);
            }
            // Close arms: +0x38 quiet vs loud (+0x40/+0x3c/+0xc steer
            // the loud path); +0x24 steers Open's S2.
            c.bus.write32(P + 0x38, (seed % 2) as u32);
            c.bus.write32(P + 0x40, ((seed / 2) % 2) as u32);
            c.bus
                .write32(P + 0x3c, if (seed / 4) % 2 == 0 { 0 } else { DATA + 0x500 });
            c.bus.write32(P + 0xc, ((seed / 8) % 2) as u32);
            c.bus.write32(P + 0x24, ((seed / 16) % 2) as u32);
            for i in 0..8u32 {
                c.bus.write32(DATA + 0x500 + 4 * i, FAKE);
            }
            c.bus.write32(OPEN_ALLOC_HOOK, FAKE);
            c.g[A0][0] = P as u64;
            c.g[A1][0] = PARAMS as u64;
            c.pc = 0x0025_3460;
        };
        check(
            ee::s06::f_00253460__generated,
            vid_engine_player_reopen,
            60,
            &setup,
        );
        require_paths(
            ee::s06::f_00253460__generated,
            60,
            &[322, 325, 335, 338, 344, 357, 393, 396, 415, 418, 437],
            &setup,
        );
        // The Open hook site must be reached on every seed: a logged call
        // returning to the hook's return address (0x25356c opened,
        // 0x2535bc !opened) must target the planted FAKE, not address 0.
        // (A blanket no-null assertion cannot be used here: nested Close
        // calls log their own targets on some seeds.)
        for seed in 0..crate::difftest::seed_count(60) {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s06::f_00253460__generated);
            let targets = crate::difftest::fakes::logged_targets(&mut c);
            let rets = crate::difftest::fakes::logged_returns(&mut c);
            let hook_targets: Vec<u32> = rets
                .into_iter()
                .zip(targets)
                .filter(|(ret, _)| *ret == 0x0025_356c || *ret == 0x0025_35bc)
                .map(|(_, t)| t)
                .collect();
            assert!(
                !hook_targets.is_empty() && hook_targets.iter().all(|&t| t == FAKE),
                "seed {seed}: hook site not reached with FAKE: {hook_targets:#x?}"
            );
        }
    }
}
