//! Staging module oc1067 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1067`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002d_9538,
        avalanche_set_node_emitter,
        "Avalanche_SetNodeEmitter",
    ),
    (0x002d_96e0, avalanche_bake_all, "Avalanche_BakeAll"),
    (0x002d_b700, ccrowd_render2d_dtor, "cCrowdRender2D_dtor"),
    (
        0x002d_b890,
        ccrowd_render2d_release_textures,
        "cCrowdRender2D_ReleaseTextures",
    ),
];

/// `cAvalanche::setAvalancheNodeEmitterData(id, params)` (`Avalanche_SetNodeEmitter`,
/// 0x2d9538). Walks the avalanche groups (`cAvalanche_m_avalancheList`, gp+0x9c0)
/// and their node lists for the node whose instance is `id`. A permanent node
/// (base type 2 at `+0xf0`) is left alone; any other gets `+0xf2 = 1` and the
/// 0xd8 bytes of `*params` copied over its start. Nothing found: no change.
///
/// Register and cycle results follow the generated body block by block.
pub fn avalanche_set_node_emitter(c: &mut Cpu) {
    'done: {
        c.cycles += 3;
        c.g[A2][0] = Addr(c.g[GP][0] as u32).off(0x9c0).lw(c);
        if c.g[A2][0] == 0 {
            break 'done;
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x4).lw(c);
        'groups: loop {
            'group: {
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    c.g[A2][0] = Addr(c.g[A2][0] as u32).off(0x8).lw(c);
                    break 'group;
                }
                c.cycles += 4;
                c.g[A3][0] = 0x2;
                c.g[T0][0] = 0x1;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xf8).lw(c);
                'nodes: loop {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A0][0] {
                        c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0xf4).lw(c);
                        c.cycles += 2;
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xf8).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x002d_9560);
                            }
                            continue 'nodes;
                        }
                        c.cycles += 1;
                        c.g[A2][0] = Addr(c.g[A2][0] as u32).off(0x8).lw(c);
                        break 'group;
                    }
                    // The instance matches: this node's base type decides.
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xf0).lhu(c);
                    c.g[A0][0] = c.g[V1][0];
                    if c.g[V0][0] == c.g[A3][0] {
                        break 'done;
                    }
                    copy_emitter_data(c);
                    return;
                }
            }
            c.cycles += 2;
            if c.g[A2][0] == 0 {
                break 'done;
            }
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x4).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002d_9548);
            }
            continue 'groups;
        }
    }
    c.cycles += 2;
    ret(c);
}

/// The emitter part of `Avalanche_SetNodeEmitter` for a matched, non-permanent
/// node (A0 = node, A1 = params): `+0xf2 = T0` (1), then `*params` (0xd8
/// bytes) into the node. The copy is 8-byte doublewords when both pointers
/// are 8-byte aligned, else unaligned `ldl/ldr` pairs for the first 0xc0 bytes;
/// both finish with the same 0x18-byte tail. Returns to the caller.
fn copy_emitter_data(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.g[A1][0] | c.g[A0][0];
    c.g[V0][0] &= 0x7;
    let aligned = c.g[V0][0] == 0;
    let flag = c.g[T0][0];
    Addr(c.g[V1][0] as u32).off(0xf2).sb(c, flag);
    if aligned {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
        loop {
            c.cycles += 11;
            let src = c.g[A1][0] as u32;
            let dst = c.g[A0][0] as u32;
            c.g[V1][0] = Addr(src).ld(c);
            c.g[A2][0] = Addr(src.wrapping_add(0x8)).ld(c);
            c.g[A3][0] = Addr(src.wrapping_add(0x10)).ld(c);
            c.g[T0][0] = Addr(src.wrapping_add(0x18)).ld(c);
            let (v1, a2, a3, t0) = (c.g[V1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
            Addr(dst).sd(c, v1);
            Addr(dst.wrapping_add(0x8)).sd(c, a2);
            Addr(dst.wrapping_add(0x10)).sd(c, a3);
            Addr(dst.wrapping_add(0x18)).sd(c, t0);
            c.g[A1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_95e0);
            }
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
        loop {
            c.cycles += 19;
            let src = c.g[A1][0] as u32;
            let dst = c.g[A0][0] as u32;
            c.g[V1][0] = Addr(src).read_u64_unaligned(c);
            c.g[A2][0] = Addr(src.wrapping_add(0x8)).read_u64_unaligned(c);
            c.g[A3][0] = Addr(src.wrapping_add(0x10)).read_u64_unaligned(c);
            c.g[T0][0] = Addr(src.wrapping_add(0x18)).read_u64_unaligned(c);
            let (v1, a2, a3, t0) = (c.g[V1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
            Addr(dst).write_u64_unaligned(c, v1);
            Addr(dst.wrapping_add(0x8)).write_u64_unaligned(c, a2);
            Addr(dst.wrapping_add(0x10)).write_u64_unaligned(c, a3);
            Addr(dst.wrapping_add(0x18)).write_u64_unaligned(c, t0);
            c.g[A1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9588);
            }
        }
    }
    // Tail: the last 0x18 bytes of the 0xd8-byte struct.
    c.cycles += 13;
    let src = c.g[A1][0] as u32;
    let dst = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(src).read_u64_unaligned(c);
    c.g[V1][0] = Addr(src.wrapping_add(0x8)).read_u64_unaligned(c);
    c.g[A2][0] = Addr(src.wrapping_add(0x10)).read_u64_unaligned(c);
    let (v0, v1, a2) = (c.g[V0][0], c.g[V1][0], c.g[A2][0]);
    Addr(dst).write_u64_unaligned(c, v0);
    Addr(dst.wrapping_add(0x8)).write_u64_unaligned(c, v1);
    Addr(dst.wrapping_add(0x10)).write_u64_unaligned(c, a2);
    ret(c);
}

/// `Avalanche_BakeAll` (0x2d96e0): frees the histories
/// (`Avalanche_FreeHistories`, 0x2d9a80), bakes each group of the avalanche
/// list (`gp+0x9c0`, linked through `+8`) with `Avalanche_BakeGroup` (0x2d9660),
/// then exports and reloads the history files (0x2d8258, 0x2d87d0).
pub fn avalanche_bake_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002d_9a80, 0x002d_96f4)?;
        c.g[S0][0] = Addr(c.g[GP][0] as u32).off(0x9c0).lw(c);
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002d_9660, 0x002d_9708)?;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9700);
                }
            }
        }
        c.cycles += 8;
        call_ptr(c, 0x002d_8258, 0x002d_971c)?;
        call_ptr(c, 0x002d_87d0, 0x002d_9724)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cCrowdRender2D::~cCrowdRender2D` (`cCrowdRender2D_dtor`, 0x2db700): releases
/// the textures (`cCrowdRender2D_ReleaseTextures`, 0x2db890), then, when the
/// delete flag (bit 0 of the second argument) is set, frees the object with
/// `__builtin_delete` (0x317e50).
pub fn ccrowd_render2d_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x002d_b890, 0x002d_b71c)?;
        c.g[S0][0] &= 0x1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002d_b730)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cCrowdRender2D::ReleaseTextures` (0x2db890): for the one record at
/// `0x445d3c` (the loop only runs once: `A0` starts at 0), reads its count at
/// `+0` and, for each of the `count` pointers in the list at `+0x94`, calls the
/// function pointer at `*(*(gp+0x2a90) + 0x10d8) + 0x194` with that pointer
/// (and the `lh +0x190` offset of the table added to the table pointer). Then
/// it calls `0x2523a8` with the object (`s7`) and returns.
pub fn ccrowd_render2d_release_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S5, 0x40);
        frame.save(c, S4, 0x50);
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d3c));
        frame.save(c, S7, 0x20);
        c.g[S4][0] = c.g[S5][0];
        frame.save(c, S6, 0x30);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, FP, 0x10);
        c.g[A0][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x94));
        frame.save(c, S1, 0x80);
        c.g[FP][0] = 0xd8;
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xd8;
        'outer: loop {
            'record: {
                c.cycles += 6;
                c.g[S1][0] = 0;
                c.mult(0, 2, 4, 2);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if empty {
                    break 'record;
                }
                c.cycles += 4;
                c.mult(0, 2, 4, 30);
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A2][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
                loop {
                    c.cycles += 12;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002d_b928)?;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
                    if !more {
                        break 'record;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_b908);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            let again = (c.g[A0][0] as i64) <= 0;
            c.g[V0][0] = 0xd8;
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x002d_b8e0);
                }
                continue 'outer;
            }
            break 'outer;
        }
        c.cycles += 14;
        c.g[A0][0] = Addr(c.g[S7][0] as u32).lw(c);
        call_ptr(c, 0x0025_23a8, 0x002d_b94c)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const OC1067_CTRL: u32 = DATA + 0xf_1100;
    const OC1067_MAGIC: u32 = 0x1067_c0de;
    static OC1067_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1067_PREV_BAKE: AtomicUsize = AtomicUsize::new(0);
    static OC1067_PREV_RELEASE: AtomicUsize = AtomicUsize::new(0);
    static OC1067_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC1067_PREV_TAIL: AtomicUsize = AtomicUsize::new(0);
    static OC1067_PREV_EXPORT: AtomicUsize = AtomicUsize::new(0);
    static OC1067_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);

    /// Gated fakes for the callees of the two drivers: they only log the call
    /// while the gate word is set, else run the body they replaced.
    fn oc1067_free_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_FREE, ee::s09::f_002d9a80__generated);
        }
        fakes::fake_call(c, 0x002d_9a80);
    }
    fn oc1067_bake_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_BAKE, ee::s09::f_002d9660__generated);
        }
        fakes::fake_call(c, 0x002d_9660);
    }
    fn oc1067_release_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_RELEASE, ee::s09::f_002db890__generated);
        }
        fakes::fake_call(c, 0x002d_b890);
    }
    fn oc1067_delete_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }
    fn oc1067_export_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_EXPORT, ee::s09::f_002d8258__generated);
        }
        fakes::fake_call(c, 0x002d_8258);
    }
    fn oc1067_load_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_LOAD, ee::s09::f_002d87d0__generated);
        }
        fakes::fake_call(c, 0x002d_87d0);
    }
    fn oc1067_tail_fake(c: &mut Cpu) {
        if c.bus.read32(OC1067_CTRL) != OC1067_MAGIC {
            return fakes::fall_back(c, &OC1067_PREV_TAIL, ee::s06::f_002523a8__generated);
        }
        fakes::fake_call(c, 0x0025_23a8);
    }
    /// Hooks the fakes once per process and raises this machine's gate.
    fn oc1067_gate(c: &mut Cpu) {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9A80),
            oc1067_free_fake,
            &OC1067_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9660),
            oc1067_bake_fake,
            &OC1067_PREV_BAKE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002DB890),
            oc1067_release_fake,
            &OC1067_PREV_RELEASE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1067_delete_fake,
            &OC1067_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1067_tail_fake,
            &OC1067_PREV_TAIL,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D8258),
            oc1067_export_fake,
            &OC1067_PREV_EXPORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D87D0),
            oc1067_load_fake,
            &OC1067_PREV_LOAD,
        );
        c.bus.write32(OC1067_CTRL, OC1067_MAGIC);
    }

    /// 0..3 records of the pointer list, with the table's offset and calls
    /// that unwind now and then.
    #[test]
    fn ccrowd_render2d_release_textures_matches() {
        check(
            ee::s09::f_002db890__generated,
            ccrowd_render2d_release_textures,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_b890, 4);
                oc1067_gate(c);
                let obj = DATA + 0x2000;
                let table = DATA + 0x3000;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), obj);
                c.bus.write32(obj + 0x10d8, table);
                c.bus.write16(table + 0x190, ((seed % 3) * 0x10) as u16);
                c.bus.write32(table + 0x194, 0x0012_3000);
                let count = seed % 4;
                c.bus.write32(0x0044_5d3c, count);
                for i in 0..count {
                    c.bus
                        .write32(0x0044_5d3c + 0x94 + 4 * i, DATA + 0x5000 + 0x10 * i);
                }
                c.g[A0][0] = sx(DATA + 0x6000);
            },
        );
    }

    /// Both flag values (delete or not), with and without unwinding calls.
    #[test]
    fn ccrowd_render2d_dtor_matches() {
        check(
            ee::s09::f_002db700__generated,
            ccrowd_render2d_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_b700, 2);
                oc1067_gate(c);
                c.g[A1][0] = sx(seed);
            },
        );
    }

    /// 0..3 groups in the avalanche list (or none); the callees are logged
    /// fakes, and some seeds unwind at one of their calls.
    #[test]
    fn avalanche_bake_all_matches() {
        check(
            ee::s09::f_002d96e0__generated,
            avalanche_bake_all,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_96e0, 3);
                oc1067_gate(c);
                let mut head = 0u32;
                for g in (0..seed % 4).rev() {
                    let group = DATA + 0x1000 + g * 0x40;
                    c.bus.write32(group + 0x8, head);
                    head = group;
                }
                c.bus.write32(GAME_GP.wrapping_add(0x9c0), head);
            },
        );
    }

    /// Node groups at `gp+0x9c0`: 0..3 groups, each with 0..3 nodes (ids
    /// 1..12); a target id that is sometimes absent; permanent and other base
    /// types; `params` and the nodes at both alignments.
    #[test]
    fn avalanche_set_node_emitter_matches() {
        check(
            ssx3_ee::s09::f_002d9538__generated,
            avalanche_set_node_emitter,
            300,
            &|c, seed| {
                let mut r = Rng(0x002d_9538 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let groups = seed % 4;
                let mut head = 0u32;
                for g in (0..groups).rev() {
                    let group = DATA + 0x1000 + g * 0x40;
                    let count = (seed / (g + 1)) % 4;
                    let skew = if (seed / 4) % 2 == 1 { 4 } else { 0 };
                    let mut first = 0u32;
                    for k in (0..count).rev() {
                        let node = DATA + 0x4000 + (g * 4 + k) * 0x100 + skew;
                        let base = if (seed + k) % 4 == 0 { 2 } else { (g + k) % 2 };
                        c.bus.write32(node + 0xf8, 1 + g * 4 + k);
                        c.bus.write32(node + 0xf4, first);
                        c.bus.write16(node + 0xf0, base as u16);
                        c.bus.write8(node + 0xf2, 0);
                        fill(c, &mut r, node, 0x36);
                        first = node;
                    }
                    c.bus.write32(group + 0x4, first);
                    c.bus.write32(group + 0x8, head);
                    head = group;
                }
                c.bus.write32(GAME_GP.wrapping_add(0x9c0), head);
                let target = if seed % 5 == 4 {
                    99
                } else {
                    1 + r.next_u32() % 12
                };
                c.g[A0][0] = sx(target);
                let params = DATA + 0x2000 + (seed % 4);
                fill(c, &mut r, params, 0x36);
                c.g[A1][0] = sx(params);
            },
        );
    }
}
