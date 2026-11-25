//! Staging module oc560 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc560`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_c898,
        wscriptman_debugprintf,
        "WScriptMan_DebugPrintf",
    ),
    (0x0030_d348, wscriptman_pop_event, "WScriptMan_PopEvent"),
    (0x0030_d368, wscriptman_peek_event, "WScriptMan_PeekEvent"),
    (
        0x0030_d3e0,
        wscriptprocess_run_callback,
        "WScriptProcess_RunCallback",
    ),
    (
        0x0030_d400,
        wscripttask_run_callback,
        "WScriptTask_RunCallback",
    ),
    (
        0x0030_d498,
        wscriptman_global_ctors,
        "WScriptMan_GlobalCtors",
    ),
    (
        0x0031_0458,
        rider_model_get_part_point_world,
        "RiderModel_GetPartPointWorld",
    ),
    (
        0x0031_19f8,
        animtable_get_layer_name,
        "AnimTable_GetLayerName",
    ),
    (
        0x0031_1a30,
        animtable_get_anim_name,
        "AnimTable_GetAnimName",
    ),
    (
        0x0031_1d88,
        animcontroller_set_sequences_user_data,
        "AnimController_SetSequencesUserData",
    ),
    (
        0x0031_3b38,
        animsequence_remove_trigger,
        "AnimSequence_RemoveTrigger",
    ),
    (0x0031_4a60, mdffile_load, "MdfFile_Load"),
    (0x0031_4ae0, mdffile_has_model, "MdfFile_HasModel"),
    (0x0031_4b70, mdffile_find_model, "MdfFile_FindModel"),
    (0x0031_4db0, gameanimmap_dtor, "GameAnimMap_dtor"),
    (0x0031_58e8, xform_set_rotation_z, "XForm_SetRotationZ"),
    (0x0031_5a00, animmath_global_ctors, "AnimMath_GlobalCtors"),
    (
        0x0031_6878,
        animtables_global_ctors,
        "AnimTables_GlobalCtors",
    ),
];

/// `int WScriptMan_DebugPrintf(WScriptMan* self, ...)` (0x30c898): the release
/// build's printf stub. It spills the variadic argument registers into its
/// frame (a1..a3 and t0..t3 as 64-bit words, f12..f18 as float varargs) and
/// returns `self`.
pub fn wscriptman_debugprintf(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    for (i, reg) in [A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
        let v = c.g[reg][0];
        let a = frame.at(c, 0x48 + 8 * i as u32);
        c.sd(a, v);
    }
    for (i, f) in [12usize, 14, 16, 18].into_iter().enumerate() {
        let a = frame.at(c, 0x38 + 4 * i as u32);
        c.swc1(f, a);
    }
    frame.close(c);
}

/// The shape of the two event-queue forwards: a 0x10 frame holding `ra`, `a0`
/// moved to the queue at `self + 0x1c4`, one call returning to `ret`, then
/// `ra` restored and the frame closed.
fn wscriptman_event_queue_forward(c: &mut Cpu, callee: Callee, ret: u32) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c4));
    guest(c, |c| {
        callee.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_PopEvent(WScriptMan* self)` (0x30d348): pop the next
/// script event off the event queue (`WScriptMan_EventQueuePop`).
pub fn wscriptman_pop_event(c: &mut Cpu) {
    let callee = Callee {
        addr: 0x0030_c820,
        f: ee::s10::f_0030c820,
    };
    wscriptman_event_queue_forward(c, callee, 0x0030_d358);
}

/// `void WScriptMan_PeekEvent(WScriptMan* self)` (0x30d368): look at the next
/// script event without removing it (`WScriptMan_EventQueuePeek`).
pub fn wscriptman_peek_event(c: &mut Cpu) {
    let callee = Callee {
        addr: 0x0030_c870,
        f: ee::s10::f_0030c870,
    };
    wscriptman_event_queue_forward(c, callee, 0x0030_d378);
}

/// `void WScriptProcess_runScriptCode1`-style forward (0x30d3e0): run the
/// manager's script code (`self->Manager`, at `self + 0x10`).
pub fn wscriptprocess_run_callback(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(0x10),
        Target::Direct(Callee {
            addr: 0x0030_ac98,
            f: ee::s10::f_0030ac98,
        }),
        0x0030_d3f0,
    );
}

/// `WScriptTask_RunCallback` (0x30d400): run the task function of the
/// manager (`self + 0x10`), the same forward as the process one.
pub fn wscripttask_run_callback(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(0x10),
        Target::Direct(Callee {
            addr: 0x0030_ada8,
            f: ee::s10::f_0030ada8,
        }),
        0x0030_d410,
    );
}

/// The shape of the three `*_GlobalCtors` stubs: a 0x10 frame holding `ra`,
/// then `StaticInit(1, 0xffff)` (the static constructors of one unit), and
/// return what that returns.
fn static_ctor_stub(c: &mut Cpu, init: Callee, ret: u32) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = 0x1;
    c.g[A1][0] = 0xffff;
    guest(c, |c| {
        init.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_GlobalCtors(void)` (0x30d498).
pub fn wscriptman_global_ctors(c: &mut Cpu) {
    let init = Callee {
        addr: 0x0030_c8d8,
        f: ee::s10::f_0030c8d8,
    };
    static_ctor_stub(c, init, 0x0030_d4ac);
}

/// `void AnimMath_GlobalCtors(void)` (0x315a00).
pub fn animmath_global_ctors(c: &mut Cpu) {
    let init = Callee {
        addr: 0x0031_50f8,
        f: ee::s10::f_003150f8,
    };
    static_ctor_stub(c, init, 0x0031_5a14);
}

/// `void AnimTables_GlobalCtors(void)` (0x316878).
pub fn animtables_global_ctors(c: &mut Cpu) {
    let init = Callee {
        addr: 0x0031_5a20,
        f: ee::s10::f_00315a20,
    };
    static_ctor_stub(c, init, 0x0031_688c);
}

/// `Vec4 RiderModel_GetPartPointWorld(RiderModel* model, int id, int index)`
/// (0x310458, with `out` in a0): hot spot `index` of the model part whose id
/// is `id`, as a world point. The hot spot's local point (w = 1) goes through
/// the part's bone matrix (`skel + (boneId + firstBone) * 64`) on VU0. Writes
/// `out` and returns it in v0; `t4vect_Origin` when no part has the id.
///
/// Args: a0 = out, a1 = model, a2 = id, a3 = index.
pub fn rider_model_get_part_point_world(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_310524: {
        'b_310514: {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[T2][0] = c.g[A0][0];
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[T0][0] = 0;
            if t {
                break 'b_310514;
            }
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 4);
            c.g[T1][0] = 0;
            'part_loop: loop {
                'b_310504: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    let part = (c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32);
                    c.g[A0][0] = sx(part);
                    c.g[V1][0] = c.lw(part);
                    if c.g[V1][0] != c.g[A2][0] {
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                        break 'b_310504;
                    }
                    c.cycles += 28;
                    c.g[V0][0] = c.lw(part.wrapping_add(0x3c));
                    c.g[A0][0] = c.lw(part.wrapping_add(0x4));
                    let pt = (c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32);
                    c.g[V0][0] = sx(pt);
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x34));
                    c.lwc1(0, pt);
                    c.g[V1][0] = c.lw(pt.wrapping_add(0xc));
                    c.swc1(0, frame.at(c, 0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.lwc1(1, pt.wrapping_add(4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    let mat = c.g[V1][0] as u32;
                    c.swc1(1, frame.at(c, 4));
                    c.lwc1(0, pt.wrapping_add(8));
                    c.swc1(2, frame.at(c, 0xc));
                    c.swc1(0, frame.at(c, 8));
                    c.lqc2(8, frame.at(c, 0));
                    c.lqc2(4, mat);
                    c.lqc2(5, mat.wrapping_add(0x10));
                    c.lqc2(6, mat.wrapping_add(0x20));
                    c.lqc2(7, mat.wrapping_add(0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, frame.at(c, 0x10));
                    c.lq(2, frame.at(c, 0x10));
                    let out = c.g[T2][0] as u32;
                    c.sq(2, out);
                    break 'b_310524;
                }
                c.cycles += 4;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x58));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0480);
                    }
                    continue 'part_loop;
                }
                break 'b_310514;
            }
        }
        // No part has the id: the origin point.
        c.cycles += 4;
        c.g[V0][0] = 0x500000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        let origin = c.g[V0][0] as u32;
        c.lq(3, origin);
        let out = c.g[T2][0] as u32;
        c.sq(3, out);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T2][0];
    frame.close(c);
}

/// `const char* AnimTable_GetLayerName(int anim)` (0x3119f8): the name of the
/// layer the animation's entry names, from the layer name table at 0x44af80.
pub fn animtable_get_layer_name(c: &mut Cpu) {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A1][0];
    guest(c, |c| {
        Callee {
            addr: 0x0031_15c8,
            f: ee::s10::f_003115c8,
        }
        .call(c, 0x0031_1a08)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = 0x450000;
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5080));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        frame.close(c);
        Ok(())
    });
}

/// `const char* AnimTable_GetAnimName(int anim)` (0x311a30): forwards the
/// animation index to the name stub.
pub fn animtable_get_anim_name(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A1][0];
    guest(c, |c| {
        Callee {
            addr: 0x0031_1860,
            f: ee::s10::f_00311860,
        }
        .call(c, 0x0031_1a40)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void AnimController_SetSequencesUserData(AnimController* self, void* data)`
/// (0x311d88): sets `IsFlipped` (+0x80) of every playing sequence of every
/// sequencer (six of them, eight bytes apart from `self + 0x50`) to `data`.
/// The sequences come from `AnimSeqList_GetAt`, and each loop checks for an
/// event at its back edge.
pub fn animcontroller_set_sequences_user_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 18, 0x30);
        frame.save(c, 20, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 17, 0x40);
        c.g[A0][0] = 0;
        frame.save(c, 19, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
        'seq_loop: loop {
            'b_311e04: {
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[A0][0] as u32) << 3);
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                if t {
                    break 'b_311e04;
                }
                c.cycles += 2;
                c.g[S1][0] = c.g[A1][0];
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                'play_loop: loop {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    Callee {
                        addr: 0x0031_4760,
                        f: ee::s10::f_00314760,
                    }
                    .call(c, 0x0031_1de8)?;
                    let value = c.g[S4][0];
                    let node = (c.g[V0][0] as u32).wrapping_add(0x80);
                    c.sw(node, value);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_1dd8);
                        }
                        continue 'play_loop;
                    }
                    break 'b_311e04;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 6) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                if event_due(c) {
                    deliver_event_here(c, 0x0031_1db8);
                }
                continue 'seq_loop;
            }
            c.cycles += 8;
            frame.restore(c, 16, 0x50);
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            frame.restore(c, 19, 0x20);
            frame.restore(c, 20, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// `void cAnimSequence_removeTrigger(cAnimSequence* self, int id, float time)`
/// (0x313b38): unlink and free the first trigger in `self->Triggers`
/// (`self + 0xac`) with this id and time that is not an end trigger.
pub fn animsequence_remove_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xac));
        let empty = c.g[V0][0] == 0;
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0xac));
        'b_313b9c: {
            if empty {
                break 'b_313b9c;
            }
            c.cycles += 1;
            'trigger_scan: loop {
                'b_313b94: {
                    'b_313b90: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        let id_differs = c.g[V0][0] != c.g[A1][0];
                        c.g[A0][0] = c.g[V1][0];
                        if id_differs {
                            break 'b_313b90;
                        }
                        c.cycles += 5;
                        let node = c.g[A0][0] as u32;
                        c.lwc1(0, node.wrapping_add(8));
                        let same_time = FpuF32(c.f[0]).eq_s(FpuF32(c.f[12]));
                        crate::abi::set_fcc(c, same_time);
                        if !same_time {
                            c.g[V0][0] = c.lw(node.wrapping_add(0xc));
                            break 'b_313b94;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(node.wrapping_add(4));
                        let is_end = c.g[V0][0] != 0;
                        c.g[V0][0] = c.lw(node.wrapping_add(0xc));
                        if is_end {
                            break 'b_313b94;
                        }
                        c.cycles += 4;
                        let next = c.g[V0][0];
                        let link = c.g[A2][0] as u32;
                        c.sw(link, next);
                        Callee {
                            addr: 0x0031_7e50,
                            f: ee::s10::f_00317e50,
                        }
                        .call(c, 0x0031_3b88)?;
                        frame.restore_ra(c, 0);
                        c.cycles += 2;
                        frame.close(c);
                        return Ok(());
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                }
                c.cycles += 2;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_3b50);
                    }
                    continue 'trigger_scan;
                }
                break 'b_313b9c;
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void cMdfArchive_loadArchive(cMdfArchive* self, int name, int flags)`
/// (0x314a60): load the archive pack through `FILE_loadpack`, store its data
/// pointer in `self`, and parse its header.
pub fn mdffile_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, 0x003e_22f0, 0x0031_4a7c)?;
        c.g[A0][0] = c.g[S0][0];
        let archive = c.g[S0][0] as u32;
        let data = c.g[V0][0];
        c.sw(archive, data);
        c.g[A1][0] = c.g[V0][0];
        Callee {
            addr: 0x0031_4aa0,
            f: ee::s10::f_00314aa0,
        }
        .call(c, 0x0031_4a8c)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool cMdfArchive_hasModel(cMdfArchive* self, const char* name)`
/// (0x314ae0): whether one of the archive's model headers (0x60 bytes each,
/// from `self->ModelHeaders`) has this name, case-insensitively.
pub fn mdffile_has_model(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, 19, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        'b_314b50: {
            'b_314b4c: {
                if t {
                    break 'b_314b4c;
                }
                c.cycles += 2;
                c.g[S2][0] = 0;
                'model_loop: loop {
                    'b_314b38: {
                        c.cycles += 6;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        call_ptr(c, 0x0041_aa88, 0x0031_4b28)?;
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            break 'b_314b38;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        break 'b_314b50;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x60));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_4b18);
                        }
                        continue 'model_loop;
                    }
                    break 'b_314b4c;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `const char* cMdfArchive_getModelPartPtr(cMdfArchive* self, const char* name)`
/// (0x314b70): the model header with this name (case-insensitive), or null.
pub fn mdffile_find_model(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 18, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = 0;
        frame.save(c, 17, 0x30);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        'b_314be4: {
            'b_314be0: {
                if t {
                    break 'b_314be0;
                }
                c.cycles += 2;
                c.g[S1][0] = 0;
                'model_loop: loop {
                    'b_314bcc: {
                        c.cycles += 6;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        call_ptr(c, 0x0041_aa88, 0x0031_4bb8)?;
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_314bcc;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        break 'b_314be4;
                    }
                    c.cycles += 5;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_4ba8);
                        }
                        continue 'model_loop;
                    }
                    break 'b_314be0;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cGameAnimMap::~cGameAnimMap(int flags)` (0x314db0): clears the global
/// map pointer (`GP + 0xd8c`) and frees `self` when flag bit 0 is set.
pub fn gameanimmap_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A1][0] & 1;
        frame.save_ra(c, 0);
        let free = c.g[A1][0] != 0;
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd8c), 0);
        if free {
            c.cycles += 2;
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::f_00317e50,
            }
            .call(c, 0x0031_4dcc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `f[n]` as an FPU value.
fn xform_f(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `f[n] = v`.
fn xform_set_f(c: &mut Cpu, n: usize, v: FpuF32) {
    c.f[n] = v.0;
}

/// `void cRiderAnimBase_setHeadingOffset(XForm* self, float angle)` (0x3158e8):
/// the offset's orientation is a rotation of `-angle` about the Z axis
/// (`sin`/`cos` of half the angle), and its position the origin.
pub fn xform_set_rotation_z(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x60);
        let neg = -xform_f(c, 12);
        xform_set_f(c, 0, neg);
        frame.save(c, 16, 0x50);
        c.g[V0][0] = 0x500000;
        frame.save(c, 17, 0x40);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        frame.save(c, 18, 0x30);
        c.g[S1][0] = 0x500000;
        frame.save_ra(c, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = sx(frame.at(c, 0x10));
        let origin = c.g[V0][0] as u32;
        c.lq(3, origin);
        c.g[A1][0] = sx(frame.at(c, 0x14));
        let half = xform_f(c, 0) * xform_f(c, 12);
        xform_set_f(c, 12, half);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xea0));
        c.g[RA][0] = sx(0x0031_5934);
        let position = (c.g[S2][0] as u32).wrapping_add(0x30);
        c.sq(3, position);
        Callee {
            addr: 0x0031_be50,
            f: ee::s10::f_0031be50,
        }
        .call(c, 0x0031_5934)?;
        let axis = c.g[S0][0] as u32;
        c.lwc1(1, axis.wrapping_add(8));
        c.lwc1(0, frame.at(c, 0x10));
        c.lwc1(2, (c.g[S1][0] as u32).wrapping_sub(0xea0));
        c.lwc1(4, axis.wrapping_add(4));
        let z = xform_f(c, 0) * xform_f(c, 1);
        xform_set_f(c, 1, z);
        let x = xform_f(c, 0) * xform_f(c, 2);
        xform_set_f(c, 2, x);
        c.lwc1(3, frame.at(c, 0x14));
        let y = xform_f(c, 0) * xform_f(c, 4);
        xform_set_f(c, 0, y);
        frame.restore(c, 16, 0x50);
        c.swc1(3, frame.at(c, 0xc));
        c.swc1(1, frame.at(c, 0x8));
        c.swc1(2, frame.at(c, 0x0));
        c.swc1(0, frame.at(c, 0x4));
        frame.restore(c, 17, 0x40);
        c.lq(2, frame.at(c, 0x0));
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        let quat = (c.g[S2][0] as u32).wrapping_add(0x40);
        c.sq(2, quat);
        frame.restore(c, 18, 0x30);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the test fakes below act only when this holds MAGIC in the
    /// machine's own RAM (each test writes it in its fixture).
    const OC560_GATE: u32 = DATA + 0xf_f000;
    const OC560_MAGIC: u32 = 0x560c_0de5;

    static OC560_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static OC560_PREV_LOADPACK: AtomicUsize = AtomicUsize::new(0);
    static OC560_PREV_PARSE_HEADER: AtomicUsize = AtomicUsize::new(0);

    fn oc560_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC560_GATE) != OC560_MAGIC {
            return fakes::fall_back(c, &OC560_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        ret(c);
    }

    fn oc560_fake_loadpack(c: &mut Cpu) {
        if c.bus.read32(OC560_GATE) != OC560_MAGIC {
            return fakes::fall_back(c, &OC560_PREV_LOADPACK, ee::s14::f_003e22f0__generated);
        }
        c.g[V0][0] = (DATA + 0x1000) as u64;
        ret(c);
    }

    fn oc560_fake_parse_header(c: &mut Cpu) {
        if c.bus.read32(OC560_GATE) != OC560_MAGIC {
            return fakes::fall_back(c, &OC560_PREV_PARSE_HEADER, ee::s10::f_00314aa0__generated);
        }
        ret(c);
    }

    fn oc560_hook_delete() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc560_fake_delete,
            &OC560_PREV_DELETE,
        );
    }

    fn oc560_hook_loadpack() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003E22F0),
            oc560_fake_loadpack,
            &OC560_PREV_LOADPACK,
        );
    }

    fn oc560_hook_parse_header() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00314AA0),
            oc560_fake_parse_header,
            &OC560_PREV_PARSE_HEADER,
        );
    }

    /// A NUL-terminated string at `addr`.
    fn oc560_put_str(c: &mut Cpu, addr: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// An archive at `DATA + 0x100` with three model headers (0x60 bytes each
    /// at `DATA + 0x300`) and the name the caller searches for at `DATA + 0x400`.
    fn oc560_model_fixture(c: &mut Cpu, query: &[u8]) {
        let this = DATA + 0x100;
        let arch = DATA + 0x200;
        let names = DATA + 0x300;
        c.bus.write32(this + 4, arch);
        c.bus.write16(arch + 4, 3);
        c.bus.write32(this + 8, names);
        oc560_put_str(c, names, b"alpha");
        oc560_put_str(c, names + 0x60, b"beta");
        oc560_put_str(c, names + 0xc0, b"gamma");
        oc560_put_str(c, DATA + 0x400, query);
    }

    #[test]
    fn wscriptman_debugprintf_matches() {
        check(
            ee::s10::f_0030c898__generated,
            wscriptman_debugprintf,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c89_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_c898;
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn wscriptman_pop_event_matches() {
        check(
            ee::s10::f_0030d348__generated,
            wscriptman_pop_event,
            60,
            &|c, seed| {
                let mut r = Rng(0x0d34_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d348;
                c.g[A0][0] = DATA as u64;
                // An empty queue (head == tail): the callee's early return.
                let q = r.next_u32();
                c.bus.write32(DATA + 0x1c4, q);
                c.bus.write32(DATA + 0x1c8, q);
            },
        );
    }

    #[test]
    fn wscriptman_peek_event_matches() {
        check(
            ee::s10::f_0030d368__generated,
            wscriptman_peek_event,
            60,
            &|c, seed| {
                let mut r = Rng(0x0d36_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d368;
                c.g[A0][0] = DATA as u64;
                let q = r.next_u32();
                c.bus.write32(DATA + 0x1c4, q);
                c.bus.write32(DATA + 0x1c8, q);
            },
        );
    }

    #[test]
    fn wscriptprocess_run_callback_matches() {
        check(
            ee::s10::f_0030d3e0__generated,
            wscriptprocess_run_callback,
            30,
            &|c, seed| {
                let mut r = Rng(0x0d3e_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d3e0;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, DATA + 0x200);
            },
        );
    }

    #[test]
    fn wscripttask_run_callback_matches() {
        check(
            ee::s10::f_0030d400__generated,
            wscripttask_run_callback,
            30,
            &|c, seed| {
                let mut r = Rng(0x0d40_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d400;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, DATA + 0x200);
            },
        );
    }

    #[test]
    fn wscriptman_global_ctors_matches() {
        check(
            ee::s10::f_0030d498__generated,
            wscriptman_global_ctors,
            10,
            &|c, seed| {
                let mut r = Rng(0x0d49_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d498;
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn animmath_global_ctors_matches() {
        check(
            ee::s10::f_00315a00__generated,
            animmath_global_ctors,
            10,
            &|c, seed| {
                let mut r = Rng(0x0031_5a00 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_5a00;
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn animtables_global_ctors_matches() {
        check(
            ee::s10::f_00316878__generated,
            animtables_global_ctors,
            10,
            &|c, seed| {
                let mut r = Rng(0x0031_6878 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_6878;
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn rider_model_get_part_point_world_matches() {
        check(
            ee::s10::f_00310458__generated,
            rider_model_get_part_point_world,
            200,
            &|c, seed| {
                let mut r = Rng(0x0031_0458 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_0458;
                let model = DATA;
                let parts = DATA + 0x100;
                let hot = DATA + 0x400;
                let skel = DATA + 0x800;
                // numParts 0..3: the zero-part path and parts with ids 5, 7, 9.
                c.bus.write32(model + 0x8, seed % 4);
                c.bus.write32(model + 0xc, parts);
                c.bus.write32(model + 0x34, skel);
                for i in 0..3u32 {
                    let p = parts + i * 0x58;
                    c.bus.write32(p, 5 + 2 * i);
                    c.bus.write32(p + 0x4, r.next_u32() % 3);
                    c.bus.write32(p + 0x3c, hot);
                }
                for k in 0..4u32 {
                    let h = hot + k * 16;
                    c.bus.write32(h, r.next_u32());
                    c.bus.write32(h + 0x4, r.next_u32());
                    c.bus.write32(h + 0x8, r.next_u32());
                    c.bus.write32(h + 0xc, r.next_u32() % 4);
                }
                fill(c, &mut r, skel, 128);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = model as u64;
                // Found (ids 5, 7, 9) or missing (id 3).
                c.g[A2][0] = if seed % 5 == 0 {
                    3
                } else {
                    (5 + 2 * (seed % 3)) as u64
                };
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn animtable_get_layer_name_matches() {
        check(
            ee::s10::f_003119f8__generated,
            animtable_get_layer_name,
            60,
            &|c, seed| {
                let mut r = Rng(0x0031_19f8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_19f8;
                let idx = seed % 6;
                c.g[A1][0] = idx as u64;
                // The anim entry at 0x446990 + idx * 0x1c names a layer; the layer
                // table at 0x44af80 names the layer.
                let entry = 0x0044_6990 + idx * 0x1c;
                c.bus.write32(entry + 0xc, seed % 8);
                c.bus.write32(0x0044_af80 + (seed % 8) * 4, r.next_u32());
            },
        );
    }

    #[test]
    fn animtable_get_anim_name_matches() {
        check(
            ee::s10::f_00311a30__generated,
            animtable_get_anim_name,
            40,
            &|c, seed| {
                let mut r = Rng(0x0031_1a30 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_1a30;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn animcontroller_set_sequences_user_data_matches() {
        check(
            ee::s10::f_00311d88__generated,
            animcontroller_set_sequences_user_data,
            80,
            &|c, seed| {
                let mut r = Rng(0x0031_1d88 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_1d88;
                let this = DATA;
                let base = DATA + 0x1000;
                c.bus.write32(this + 0x50, base);
                // Six sequencers of 8 bytes: NumPlaying (0..2) and a list head.
                for i in 0..6u32 {
                    let entry = base + i * 8;
                    c.bus.write32(entry, (seed + i) % 3);
                    let head = DATA + 0x4000 + i * 0x400;
                    c.bus.write32(entry + 0x4, head);
                    for k in 0..3u32 {
                        let node = head + k * 0x100;
                        let next = if k == 2 { 0 } else { node + 0x100 };
                        c.bus.write32(node + 0xc8, next);
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn animsequence_remove_trigger_matches() {
        oc560_hook_delete();
        check(
            ee::s10::f_00313b38__generated,
            animsequence_remove_trigger,
            120,
            &|c, seed| {
                let mut r = Rng(0x0031_3b38 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_3b38;
                c.bus.write32(OC560_GATE, OC560_MAGIC);
                let nodes = [DATA + 0x3000, DATA + 0x3100, DATA + 0x3200, DATA + 0x3300];
                // (ID, IsEndTrigger, TriggerTime) for each trigger in the list.
                let spec: [(u32, u32, f32); 4] =
                    [(1, 0, 0.5), (2, 1, 0.5), (2, 0, 1.0), (3, 0, 0.5)];
                for (k, &(id, end, time)) in spec.iter().enumerate() {
                    let n = nodes[k];
                    c.bus.write32(n, id);
                    c.bus.write32(n + 0x4, end);
                    c.bus.write32(n + 0x8, time.to_bits());
                    let next = if k == 3 { 0 } else { nodes[k + 1] };
                    c.bus.write32(n + 0xc, next);
                }
                c.bus.write32(DATA + 0xac, nodes[0]);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = [1u64, 2, 3, 4][(seed % 4) as usize];
                c.f[12] = if seed % 2 == 0 {
                    0.5f32.to_bits()
                } else {
                    0.75f32.to_bits()
                };
            },
        );
    }

    #[test]
    fn mdffile_load_matches() {
        oc560_hook_loadpack();
        oc560_hook_parse_header();
        check(
            ee::s10::f_00314a60__generated,
            mdffile_load,
            40,
            &|c, seed| {
                let mut r = Rng(0x0031_4a60 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_4a60;
                c.bus.write32(OC560_GATE, OC560_MAGIC);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn mdffile_has_model_matches() {
        check(
            ee::s10::f_00314ae0__generated,
            mdffile_has_model,
            80,
            &|c, seed| {
                let mut r = Rng(0x0031_4ae0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_4ae0;
                // Case-insensitive hits and misses.
                let queries: [&[u8]; 4] = [b"BETA", b"gamma", b"delta", b"alpha"];
                oc560_model_fixture(c, queries[(seed % 4) as usize]);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
            },
        );
    }

    #[test]
    fn mdffile_find_model_matches() {
        check(
            ee::s10::f_00314b70__generated,
            mdffile_find_model,
            80,
            &|c, seed| {
                let mut r = Rng(0x0031_4b70 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_4b70;
                let queries: [&[u8]; 4] = [b"Gamma", b"alpha", b"omega", b"BETA"];
                oc560_model_fixture(c, queries[(seed % 4) as usize]);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
            },
        );
    }

    #[test]
    fn gameanimmap_dtor_matches() {
        oc560_hook_delete();
        check(
            ee::s10::f_00314db0__generated,
            gameanimmap_dtor,
            40,
            &|c, seed| {
                let mut r = Rng(0x0031_4db0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_4db0;
                c.bus.write32(OC560_GATE, OC560_MAGIC);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn xform_set_rotation_z_matches() {
        check(
            ee::s10::f_003158e8__generated,
            xform_set_rotation_z,
            200,
            &|c, seed| {
                let mut r = Rng(0x0031_58e8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0031_58e8;
                // The origin (t4vect_Origin) and the Z axis (t4vect_ZAxis).
                fill(c, &mut r, 0x004f_f130, 4);
                c.bus.write32(0x004f_f160, 0);
                c.bus.write32(0x004f_f164, 0);
                c.bus.write32(0x004f_f168, 0x3f80_0000);
                c.bus.write32(0x004f_f16c, 0);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.f32(6.0);
            },
        );
    }
}
