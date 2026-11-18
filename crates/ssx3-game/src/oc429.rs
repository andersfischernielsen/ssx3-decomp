//! Staging module oc429 (wave wave220): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc429`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `EA_MemFree(void*)`.
const OC429_MEM_FREE: u32 = 0x0031_7e98;
/// `__builtin_delete(void*)`.
const OC429_DELETE: u32 = 0x0031_7e50;

/// `base + off` as a 32-bit guest address.
#[inline(always)]
fn oc429_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `if (a0 != 0) EA_MemFree(a0)`: the compiled null check on `a0` (2 cycles,
/// 3 more with the call), returning to `ret`.
#[inline(always)]
fn oc429_free_a0(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        call_ptr(c, OC429_MEM_FREE, ret)?;
    }
    Ok(())
}

/// `cAnimModel` (RiderModel) field offsets used by the destructor.
pub(crate) const RIDERMODEL_NUM_VARIANTS: u32 = 0x4;
pub(crate) const RIDERMODEL_NUM_GROUPS: u32 = 0x8;
pub(crate) const RIDERMODEL_GROUPS: u32 = 0xc;
pub(crate) const RIDERMODEL_TEX_COUNT: u32 = 0x12c;
pub(crate) const RIDERMODEL_TEX_LIST: u32 = 0x130;

/// `void RiderModel_dtor(RiderModel* self, int flags)` (0x30d540,
/// `cAnimModel::~cAnimModel`): free the model's allocations (`+0x38`, the four
/// slot blocks at `+0x44 + 8 * k`, per group/variant buffers, the texture
/// list entries via the graphics manager's vtable `+0x380` / `+0x190`, then
/// the plain buffers) and `__builtin_delete(self)` when `flags & 1`.
pub fn rider_model_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 22, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, 23, 0x30);
        frame.save(c, 30, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x38));
        let t = c.g[A0][0] == 0;
        c.g[S6][0] = c.lw(oc429_at(c.g[GP][0], 0x2a90));
        if !t {
            c.cycles += 2;
            call_ptr(c, OC429_MEM_FREE, 0x0030_d588)?;
        }

        'slots: {
            c.cycles += 4;
            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
            c.g[S0][0] &= 0x1;
            let t = c.g[V0][0] == 0;
            c.sw(oc429_at(c.g[SP][0], 4), c.g[S0][0]);
            if t {
                break 'slots;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(oc429_at(c.g[S2][0], 0x44));
            c.g[S0][0] = 0;
            c.sw(sp(c), c.g[V0][0]);
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
            // k = 0..3 (slot blocks).
            loop {
                c.cycles += 6;
                c.g[V1][0] = c.lw(sp(c));
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = sx(oc429_at(c.g[V1][0], c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    call_ptr(c, OC429_MEM_FREE, 0x0030_d5c8)?;
                }
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_GROUPS));
                'groups: {
                    c.cycles += 4;
                    c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                    c.sw(oc429_at(c.g[SP][0], 8), c.g[S0][0]);
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S3][0] = 0;
                    if t {
                        break 'groups;
                    }
                    c.cycles += 3;
                    c.g[FP][0] = 0x58;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_VARIANTS));
                    // j = 0..num_groups.
                    loop {
                        'variants: {
                            c.cycles += 3;
                            c.g[S1][0] = 0;
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S7][0] = sx(oc429_at(c.g[S3][0], 1));
                            if t {
                                break 'variants;
                            }
                            c.cycles += 3;
                            c.mult(0, S4, S3, FP);
                            c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                            // variant = 0..num_variants.
                            loop {
                                c.cycles += 8;
                                c.g[S0][0] = sx((c.g[S1][0] as u32) << 4);
                                c.g[V1][0] = sx(oc429_at(c.g[S4][0], c.g[V1][0] as u32));
                                c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[S5][0] as u32));
                                c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x1c));
                                c.g[V0][0] = sx(oc429_at(c.g[S0][0], c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0xc));
                                if c.g[A0][0] != 0 {
                                    c.cycles += 3;
                                    call_ptr(c, OC429_MEM_FREE, 0x0030_d628)?;
                                }
                                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                                // gfx->vtbl[0x380](gfx, entry->f0)
                                c.cycles += 18;
                                c.g[A1][0] = c.lw(oc429_at(c.g[S6][0], 0x10d8));
                                c.g[V0][0] = sx(oc429_at(c.g[S4][0], c.g[V0][0] as u32));
                                c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S5][0] as u32));
                                c.g[A0][0] = c.lh(oc429_at(c.g[A1][0], 0x380));
                                c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x1c));
                                c.g[A2][0] = c.lw(oc429_at(c.g[A1][0], 0x384));
                                c.g[A0][0] = sx(oc429_at(c.g[S6][0], c.g[A0][0] as u32));
                                c.g[V1][0] = sx(oc429_at(c.g[S0][0], c.g[V1][0] as u32));
                                let target = c.g[A2][0] as u32;
                                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                call_ptr(c, target, 0x0030_d654)?;
                                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                                c.g[V0][0] = sx(oc429_at(c.g[S4][0], c.g[V0][0] as u32));
                                c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S5][0] as u32));
                                c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x1c));
                                c.g[V1][0] = sx(oc429_at(c.g[S0][0], c.g[V1][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                if c.g[A0][0] != 0 {
                                    c.cycles += 3;
                                    call_ptr(c, OC429_MEM_FREE, 0x0030_d67c)?;
                                }
                                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_VARIANTS));
                                c.cycles += 4;
                                c.g[S1][0] = sx(oc429_at(c.g[S1][0], 1));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0 {
                                    c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030_d600);
                                    }
                                    continue;
                                }
                                break;
                            }
                        }
                        // Free the group's variant array.
                        c.cycles += 7;
                        c.mult(0, V0, S3, FP);
                        c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[V1][0] as u32));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S5][0] as u32));
                        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x1c));
                        if c.g[A0][0] != 0 {
                            c.cycles += 3;
                            call_ptr(c, OC429_MEM_FREE, 0x0030_d6b4)?;
                        }
                        c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_GROUPS));
                        c.cycles += 4;
                        c.g[S3][0] = c.g[S7][0];
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_VARIANTS));
                            if event_due(c) {
                                deliver_event_here(c, 0x0030_d5e8);
                            }
                            continue;
                        }
                        break;
                    }
                }
                c.cycles += 4;
                c.g[S0][0] = c.lw(oc429_at(c.g[SP][0], 8));
                c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_d5a8);
                    }
                    continue;
                }
                break;
            }
        }

        // Texture list: release each entry with id >= 0 through the gfx
        // vtable `+0x190`.
        'tex: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_LIST));
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x30));
                break 'tex;
            }
            'texdone: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_COUNT));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                if t {
                    break 'texdone;
                }
                c.cycles += 2;
                c.g[S1][0] = 0;
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_LIST));
                    c.g[V0][0] = sx(oc429_at(c.g[S1][0], c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    if (c.g[A1][0] as i64) < 0 {
                        c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_COUNT));
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x10d8));
                        c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x190));
                        c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x194));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx(oc429_at(c.g[S6][0], c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0030_d720)?;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_COUNT));
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(oc429_at(c.g[S1][0], 0xc));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030_d6f8);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x30));
        }

        // The plain buffers, each freed if non-null.
        for (next, ret) in [
            (0x34u32, 0x0030_d748u32),
            (0x2c, 0x0030_d75c),
            (0xc, 0x0030_d770),
            (0x1c, 0x0030_d784),
            (0x3c, 0x0030_d798),
            (0x130, 0x0030_d7ac),
            (0x24, 0x0030_d7c0),
            (0x28, 0x0030_d7d4),
            (0x20, 0x0030_d7e8),
        ] {
            oc429_free_a0(c, ret)?;
            c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], next));
        }
        // Last buffer (the next load is `flags & 1` from the frame).
        c.cycles += 2;
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 4));
        if !t {
            c.cycles += 3;
            call_ptr(c, OC429_MEM_FREE, 0x0030_d7fc)?;
            c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 4));
        }
        // flags & 1: delete the object itself.
        c.cycles += 2;
        let t = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xa0);
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, OC429_DELETE, 0x0030_d810)?;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptList_AddChoice(ScriptList* self, int a1, int a2, int lookup,
/// int a4, int a5, int a6)` (0x275ed0): queue a script choice on the list's
/// command queue (`self + 0xc`). Returns 0 when the queue is full or the
/// script lookup (`ScriptEngine_FindScriptIndex`) fails, else 1; after the
/// push, a list in mode 3 whose head entry is a finished movie / request
/// player (state 4) calls `ScriptList_CheckNextReady`.
///
/// Has resume entries: a resumed pc (after an unwound callee) goes to the
/// generated `__entries` body.
pub fn script_list_add_choice(c: &mut Cpu) {
    if c.pc != 0x0027_5ed0 {
        return ee::s07::f_00275ed0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[T1][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, 22, 0x30);
        c.g[S5][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S6][0] = c.g[A3][0];
        c.g[A0][0] = sx(oc429_at(c.g[S0][0], 0xc));
        call_ptr(c, 0x0028_3c58, 0x0027_5f18)?; // ScriptCmdQueue_IsFull
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            'queued: {
                c.cycles += 9;
                c.g[V0][0] = u64::MAX;
                c.sw(oc429_at(c.g[SP][0], 8), c.g[S1][0]);
                c.sw(oc429_at(c.g[SP][0], 0xc), c.g[V0][0]);
                c.sw(sp(c), c.g[S5][0]);
                c.sw(oc429_at(c.g[SP][0], 4), c.g[V0][0]);
                c.sw(oc429_at(c.g[SP][0], 0x10), c.g[S2][0]);
                c.sw(oc429_at(c.g[SP][0], 0x14), c.g[S3][0]);
                let no_lookup = c.g[S6][0] == 0;
                c.sw(oc429_at(c.g[SP][0], 0x18), c.g[S4][0]);
                if !no_lookup {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[T0][0] = c.g[S4][0];
                    call_ptr(c, 0x0027_97a0, 0x0027_5f5c)?; // ScriptEngine_FindScriptIndex
                    if (c.g[V0][0] as i64) >= 0 {
                        c.sw(oc429_at(c.g[SP][0], 4), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        break 'queued;
                    }
                }
                c.cycles += 7;
                c.g[S1][0] = sx(oc429_at(c.g[S0][0], 0xc));
                c.g[A1][0] = c.g[SP][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0028_3c80, 0x0027_5f7c)?; // ScriptCmdQueue_Push
                c.g[V1][0] = c.lw(oc429_at(c.g[S0][0], 0xa4));
                let t = c.g[V1][0] == 0;
                c.g[V0][0] = 1;
                if t {
                    break 'queued;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0xbc));
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 1;
                if t {
                    break 'queued;
                }
                'done: {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0028_3d28, 0x0027_5f9c)?; // ScriptCmdQueue_Count
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0027_7450, 0x0027_5fac)?; // ScriptList_PrepareEntry
                    c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0xa4));
                    c.g[V1][0] = 3;
                    let not_mode3 = c.g[A0][0] != c.g[V1][0];
                    c.sw(oc429_at(c.g[S0][0], 0xbc), c.g[V0][0]);
                    if not_mode3 {
                        break 'done;
                    }
                    'ready: {
                        'movie: {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = 0;
                            call_ptr(c, 0x0028_3d70, 0x0027_5fc8)?; // ScriptCmdQueue_At
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0027_7dd8, 0x0027_5fd4)?; // ScriptList_IsMovieEntry
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                break 'movie;
                            }
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = 0;
                            call_ptr(c, 0x0028_3d70, 0x0027_5fe8)?; // ScriptCmdQueue_At
                            c.g[A1][0] = c.lw(oc429_at(c.g[V0][0], 0xc));
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            call_ptr(c, 0x0028_2bf0, 0x0027_5ff4)?; // ScriptEngine_GetProcessInSeq
                            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x20));
                            c.g[V1][0] ^= 0x4;
                            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                            break 'ready;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0x500));
                        call_ptr(c, 0x0028_3bb8, 0x0027_600c)?; // ScriptMovie_GetId
                    }
                    c.cycles += 2;
                    let not_ready = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if not_ready {
                        break 'queued;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0027_76e0, 0x0027_601c)?; // ScriptList_CheckNextReady
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `DataNode* DataInstanceMan_AddData(DataMan* self, int name, void* data,
/// int size)` (0x275920, `cDataManager::AttachData`): take a node from the
/// free list (`AllocInfo`), fill its data / size / refcount (1), then ask
/// `BigFile_GetHeaderSize` (with the global abort flag at `0x450dd4`
/// cleared around it) whether `data` is a big file. A big file goes through
/// the owner's vtable `+0x20` slot and `AddBigEntries`, anything else
/// through the vtable `+0x10` slot. Returns the node.
pub fn data_instance_man_add_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S4, 0x30);
        frame.save(c, S3, 0x40);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S5, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, 22, 0x10);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0x45_0000;
        call_ptr(c, 0x0027_5cf8, 0x0027_595c)?; // DataInstanceMan_AllocInfo
        c.g[S2][0] = c.g[V0][0];
        c.g[S1][0] = c.lw(oc429_at(c.g[S0][0], 0xdd4));
        c.g[V0][0] = 1;
        c.sw(oc429_at(c.g[S2][0], 8), c.g[S3][0]);
        c.sw(oc429_at(c.g[S2][0], 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S3][0];
        c.sw(oc429_at(c.g[S2][0], 0xc), c.g[S5][0]);
        c.sw(oc429_at(c.g[S0][0], 0xdd4), 0);
        call_ptr(c, 0x003e_2580, 0x0027_5980)?; // BigFile_GetHeaderSize
        let not_big = c.g[V0][0] == 0;
        c.sw(oc429_at(c.g[S0][0], 0xdd4), c.g[S1][0]);
        if !not_big {
            c.cycles += 16;
            c.g[V0][0] = c.lw(oc429_at(c.g[S4][0], 0xc));
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x20));
            c.g[T0][0] = c.g[S2][0];
            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x24));
            c.g[T1][0] = sx(oc429_at(c.g[S2][0], 4));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(oc429_at(c.g[S4][0], c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_59b0)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0027_5b98, 0x0027_59c0)?; // DataInstanceMan_AddBigEntries
            c.g[V0][0] = c.g[S2][0];
        } else {
            c.cycles += 11;
            c.g[V0][0] = c.lw(oc429_at(c.g[S4][0], 0xc));
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x10));
            c.g[T0][0] = c.g[S2][0];
            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x14));
            c.g[T1][0] = sx(oc429_at(c.g[S2][0], 4));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(oc429_at(c.g[S4][0], c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_59f0)?;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEBAGTInt_Reset(void)` (0x14fad0): copy the 3 x 10 table of 0x1fe-byte
/// records at `0x4a788e` (stride 0xf88 per record, 0x9b50 per row, so the
/// source is not 8-aligned) into the packed table at `0x530ec0` (stride
/// 0x1fe, row stride 0x13ec). Each record is copied in 0x20-byte chunks (with
/// `ld`/`sd` when source and destination are both 8-aligned, else with
/// `ldl`/`ldr`/`sdl`/`sdr`), then a 0x1e-byte tail. A leaf; reproduces the
/// scratch registers and the multiplier results exactly.
pub fn beb_bagt_int_reset(c: &mut Cpu) {
    c.cycles += 14;
    c.g[SP][0] = sx(oc429_at(c.g[SP][0], 0u32.wrapping_sub(0x20)));
    c.g[V0][0] = 0x53_0000;
    c.g[V1][0] = 0x4a_0000;
    let sp0 = sp(c);
    c.sq(S0, oc429_at(c.g[SP][0], 0x10));
    c.sq(S1, sp0);
    c.g[T3][0] = sx(oc429_at(c.g[V0][0], 0xec0));
    c.g[T2][0] = sx(oc429_at(c.g[V1][0], 0x788e));
    c.g[A0][0] = 0;
    c.g[T5][0] = 0x13ec;
    c.g[T1][0] = 0x1fe;
    c.g[T4][0] = 0x9b50;
    c.g[T0][0] = 0xf88;
    c.mult(0, A2, A0, T4);
    // Rows (a0 = i).
    loop {
        c.cycles += 4;
        c.mult(0, T8, A0, T5);
        c.g[A1][0] = 0;
        c.g[A3][0] = sx(oc429_at(c.g[A0][0], 1));
        c.mult(0, V1, A1, T1);
        // Records of the row (a1 = j).
        loop {
            'record: {
                'aligned: {
                    c.cycles += 9;
                    c.mult(0, A0, A1, T0);
                    c.g[V0][0] = sx(oc429_at(c.g[V1][0], c.g[T8][0] as u32));
                    c.g[V1][0] = sx(oc429_at(c.g[A0][0], c.g[A2][0] as u32));
                    c.g[A0][0] = sx(oc429_at(c.g[V0][0], c.g[T3][0] as u32));
                    c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[T2][0] as u32));
                    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
                    c.g[V0][0] &= 0x7;
                    let is_aligned = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(oc429_at(c.g[A1][0], 1));
                    if is_aligned {
                        break 'aligned;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = sx(oc429_at(c.g[V1][0], 0x1e0));
                    loop {
                        c.cycles += 19;
                        c.ldl(T6, oc429_at(c.g[V1][0], 0x7));
                        c.ldr(T6, oc429_at(c.g[V1][0], 0));
                        c.ldl(T7, oc429_at(c.g[V1][0], 0xf));
                        c.ldr(T7, oc429_at(c.g[V1][0], 0x8));
                        c.ldl(S0, oc429_at(c.g[V1][0], 0x17));
                        c.ldr(S0, oc429_at(c.g[V1][0], 0x10));
                        c.ldl(S1, oc429_at(c.g[V1][0], 0x1f));
                        c.ldr(S1, oc429_at(c.g[V1][0], 0x18));
                        c.sdl(T6, oc429_at(c.g[A0][0], 0x7));
                        c.sdr(T6, oc429_at(c.g[A0][0], 0));
                        c.sdl(T7, oc429_at(c.g[A0][0], 0xf));
                        c.sdr(T7, oc429_at(c.g[A0][0], 0x8));
                        c.sdl(S0, oc429_at(c.g[A0][0], 0x17));
                        c.sdr(S0, oc429_at(c.g[A0][0], 0x10));
                        c.sdl(S1, oc429_at(c.g[A0][0], 0x1f));
                        c.sdr(S1, oc429_at(c.g[A0][0], 0x18));
                        c.g[V1][0] = sx(oc429_at(c.g[V1][0], 0x20));
                        let more = c.g[V1][0] != c.g[A1][0];
                        c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0x20));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0014_fb40);
                            }
                            continue;
                        }
                        c.cycles += 2;
                        c.g[A1][0] = c.g[V0][0];
                        break 'record;
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = sx(oc429_at(c.g[V1][0], 0x1e0));
                loop {
                    c.cycles += 11;
                    c.g[T6][0] = c.ld(oc429_at(c.g[V1][0], 0));
                    c.g[T7][0] = c.ld(oc429_at(c.g[V1][0], 0x8));
                    c.g[S0][0] = c.ld(oc429_at(c.g[V1][0], 0x10));
                    c.g[S1][0] = c.ld(oc429_at(c.g[V1][0], 0x18));
                    c.sd(oc429_at(c.g[A0][0], 0), c.g[T6][0]);
                    c.sd(oc429_at(c.g[A0][0], 0x8), c.g[T7][0]);
                    c.sd(oc429_at(c.g[A0][0], 0x10), c.g[S0][0]);
                    c.sd(oc429_at(c.g[A0][0], 0x18), c.g[S1][0]);
                    c.g[V1][0] = sx(oc429_at(c.g[V1][0], 0x20));
                    let more = c.g[V1][0] != c.g[A1][0];
                    c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_fb98);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                    break 'record;
                }
            }
            // The 0x1e-byte tail of the record.
            c.cycles += 21;
            c.ldl(V0, oc429_at(c.g[V1][0], 0x7));
            c.ldr(V0, oc429_at(c.g[V1][0], 0));
            c.ldl(T6, oc429_at(c.g[V1][0], 0xf));
            c.ldr(T6, oc429_at(c.g[V1][0], 0x8));
            c.ldl(T7, oc429_at(c.g[V1][0], 0x17));
            c.ldr(T7, oc429_at(c.g[V1][0], 0x10));
            c.lwl(S0, oc429_at(c.g[V1][0], 0x1b));
            c.lwr(S0, oc429_at(c.g[V1][0], 0x18));
            c.sdl(V0, oc429_at(c.g[A0][0], 0x7));
            c.sdr(V0, oc429_at(c.g[A0][0], 0));
            c.sdl(T6, oc429_at(c.g[A0][0], 0xf));
            c.sdr(T6, oc429_at(c.g[A0][0], 0x8));
            c.sdl(T7, oc429_at(c.g[A0][0], 0x17));
            c.sdr(T7, oc429_at(c.g[A0][0], 0x10));
            c.swl(S0, oc429_at(c.g[A0][0], 0x1b));
            c.swr(S0, oc429_at(c.g[A0][0], 0x18));
            c.g[V0][0] = c.lh(oc429_at(c.g[V1][0], 0x1c));
            c.sh(oc429_at(c.g[A0][0], 0x1c), c.g[V0][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 10) as u64;
            if c.g[V0][0] != 0 {
                c.mult(0, V1, A1, T1);
                if event_due(c) {
                    deliver_event_here(c, 0x0014_fb18);
                }
                continue;
            }
            break;
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[A3][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
        if c.g[V0][0] != 0 {
            c.mult(0, A2, A0, T4);
            if event_due(c) {
                deliver_event_here(c, 0x0014_fb08);
            }
            continue;
        }
        break;
    }
    c.cycles += 4;
    c.lq(S0, oc429_at(c.g[SP][0], 0x10));
    c.lq(S1, sp(c));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(oc429_at(c.g[SP][0], 0x20));
    c.pc = ra;
}

/// One 0x20-byte-chunk copy loop of a by-value struct copy in the compiler's
/// inlined memcpy (0x280 bytes; the caller copies the 8-byte tail): `ld`/`sd`
/// through `tmp` when the source is 8-aligned, else `ldl`/`ldr`/`sdl`/`sdr`
/// (11 / 19 cycles per chunk, plus 2 when the unaligned loop ends). `src`,
/// `dst` advance; `end` is the source end. `head_pc` is the loop head the
/// event check resumes at.
fn oc429_chunk_copy(
    c: &mut Cpu,
    unaligned: bool,
    src: usize,
    dst: usize,
    end: usize,
    tmp: [usize; 4],
    head_pc: u32,
) {
    loop {
        if unaligned {
            c.cycles += 19;
            for (k, &t) in tmp.iter().enumerate() {
                let o = 8 * k as u32;
                c.ldl(t, oc429_at(c.g[src][0], o + 7));
                c.ldr(t, oc429_at(c.g[src][0], o));
            }
            for (k, &t) in tmp.iter().enumerate() {
                let o = 8 * k as u32;
                c.sdl(t, oc429_at(c.g[dst][0], o + 7));
                c.sdr(t, oc429_at(c.g[dst][0], o));
            }
        } else {
            c.cycles += 11;
            for (k, &t) in tmp.iter().enumerate() {
                c.g[t][0] = c.ld(oc429_at(c.g[src][0], 8 * k as u32));
            }
            for (k, &t) in tmp.iter().enumerate() {
                c.sd(oc429_at(c.g[dst][0], 8 * k as u32), c.g[t][0]);
            }
        }
        c.g[src][0] = sx(oc429_at(c.g[src][0], 0x20));
        let more = c.g[src][0] != c.g[end][0];
        c.g[dst][0] = sx(oc429_at(c.g[dst][0], 0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, head_pc);
            }
            continue;
        }
        if unaligned {
            c.cycles += 2;
        }
        return;
    }
}

/// Where the game's options record lives (`GetOptions()` is a by-value copy
/// of it): the 0x288-byte struct at 0x535610.
#[cfg(test)]
pub(crate) const SSXAPP_OPTIONS: u32 = 0x0053_5610;

/// `void SSXApp_ApplyDisplayOptions(void)` (0x228c08): fetch the BE interface 4
/// (`BELibrary_GetInstance` / `GetInterface`), then copy the options record
/// (3 times, as the compiled by-value `GetOptions()` temporaries) and apply
/// it to the graphics manager (`gp + 0x2a90`): its vtable slot `+0x140` gets
/// the display mode (`options.word0 >> 20 & 3`, mapped 1 -> 1, 2 -> 2, else 0)
/// and slot `+0x60` gets `(options.byte6, options.byte(0x295 - 0x290 + 5))`.
/// The source is 8-aligned, so the `ldl`/`ldr` copy variants are dead.
pub fn ssx_app_apply_display_options(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x550);
        frame.save(c, S1, 0x530);
        frame.save(c, S0, 0x540);
        frame.save_ra(c, 0x520);
        call_ptr(c, 0x0014_dc80, 0x0022_8c20)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0022_8c2c)?; // BELibrary_GetInterface
        c.g[V1][0] = 0x53_0000;
        c.g[S1][0] = c.g[V1][0];
        c.g[V1][0] = sx(oc429_at(c.g[V1][0], 0x5610));
        c.g[V0][0] = c.g[V1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 2;
        c.g[V0][0] = sx(oc429_at(c.g[V1][0], 0x280));
        c.g[S0][0] = sx(oc429_at(c.g[SP][0], 0x290));
        // Copy 1: options -> sp[0..].
        if aligned {
            oc429_chunk_copy(c, false, V1, A0, V0, [T1, T2, T3, A1], 0x0022_8ca8);
        } else {
            oc429_chunk_copy(c, true, V1, A0, V0, [A1, A2, A3, T0], 0x0022_8c4c);
        }
        c.cycles += 10;
        c.ldl(A2, oc429_at(c.g[V1][0], 0x7));
        c.ldr(A2, oc429_at(c.g[V1][0], 0));
        c.sdl(A2, oc429_at(c.g[A0][0], 0x7));
        c.sdr(A2, oc429_at(c.g[A0][0], 0));
        c.g[V1][0] = 1;
        c.g[V0][0] = c.lw(sp(c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
        c.g[A0][0] = c.g[V0][0] & 0x3;
        let mode1 = c.g[A0][0] == c.g[V1][0];
        c.g[A2][0] = c.lw(oc429_at(c.g[GP][0], 0x2a90));
        // 0: slot +0x140 (7 cycles) with a1 = mode; 3: the same call with
        // a1 = 0 (6 cycles).
        let mut other = false;
        if mode1 {
            c.cycles += 1;
            c.g[A1][0] = 1;
        } else {
            c.cycles += 2;
            let zero = c.g[A0][0] == 0;
            c.g[V0][0] = 2;
            if zero {
                c.cycles += 2;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 2;
                let two = c.g[A0][0] == c.g[V0][0];
                c.g[A1][0] = 2;
                if !two {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    other = true;
                }
            }
        }
        c.cycles += if other { 6 } else { 7 };
        c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x10d8));
        c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x140));
        c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x144));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc429_at(c.g[A2][0], c.g[A0][0] as u32));
        call_ptr(c, target, if other { 0x0022_8d50 } else { 0x0022_8d34 })?;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0x2a90));

        // Copy 2: options -> sp[0..] again (the `GetOptions().b6` temporary).
        c.cycles += 8;
        c.g[A3][0] = sx(oc429_at(c.g[S1][0], 0x5610));
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = c.g[A3][0] & 0x7;
        c.g[V1][0] = c.lw(oc429_at(c.g[A0][0], 0x10d8));
        c.g[V0][0] = c.lh(oc429_at(c.g[V1][0], 0x60));
        c.g[T0][0] = sx(oc429_at(c.g[V1][0], 0x64));
        let aligned = c.g[A1][0] == 0;
        c.g[A0][0] = sx(oc429_at(c.g[A0][0], c.g[V0][0] as u32));
        c.cycles += 1;
        c.g[V0][0] = sx(oc429_at(c.g[A3][0], 0x280));
        if aligned {
            oc429_chunk_copy(c, false, A3, A2, V0, [T1, T2, T3, V1], 0x0022_8dd0);
        } else {
            oc429_chunk_copy(c, true, A3, A2, V0, [10, 11, V1, A1], 0x0022_8d78);
        }
        // Tail of copy 2, then copy 3: options -> s0 (`GetOptions().b5`).
        c.cycles += 11;
        c.ldl(A1, oc429_at(c.g[A3][0], 0x7));
        c.ldr(A1, oc429_at(c.g[A3][0], 0));
        c.sdl(A1, oc429_at(c.g[A2][0], 0x7));
        c.sdr(A1, oc429_at(c.g[A2][0], 0));
        c.g[A3][0] = sx(oc429_at(c.g[S1][0], 0x5610));
        c.g[A2][0] = c.g[S0][0];
        c.g[V0][0] = c.lbu(oc429_at(c.g[SP][0], 0x6));
        c.g[V1][0] = c.g[A3][0] & 0x7;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        let aligned = c.g[V1][0] == 0;
        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        c.cycles += 1;
        c.g[V1][0] = sx(oc429_at(c.g[A3][0], 0x280));
        if aligned {
            oc429_chunk_copy(c, false, A3, A2, V1, [V0, T1, T2, T3], 0x0022_8e84);
        } else {
            oc429_chunk_copy(c, true, A3, A2, V1, [V0, T1, T2, T3], 0x0022_8e2c);
        }
        c.cycles += 12;
        c.ldl(V0, oc429_at(c.g[A3][0], 0x7));
        c.ldr(V0, oc429_at(c.g[A3][0], 0));
        c.sdl(V0, oc429_at(c.g[A2][0], 0x7));
        c.sdr(V0, oc429_at(c.g[A2][0], 0));
        c.g[V0][0] = c.lw(c.g[T0][0] as u32);
        let target = c.g[V0][0] as u32;
        c.g[A2][0] = c.lb(oc429_at(c.g[SP][0], 0x295));
        call_ptr(c, target, 0x0022_8ecc)?;
        frame.restore(c, S0, 0x540);
        frame.restore(c, S1, 0x530);
        frame.restore_ra(c, 0x520);
        frame.close(c);
        Ok(())
    })
}

/// `bool cWorldCache::StreamerRequestSection(cWorldCache* self, uint idx,
/// float value)` (0x3a98c8, `a1 = idx`, `f12 = value`): record `value` for
/// section `idx` (entries of 8 bytes at `self + 0x14`: state, value) when
/// the section table (`self + 4`, count at `+8`) has the index, and advance
/// the state through the jump table at `0x494f10`: 0 -> 1, 3 -> 5, 4 -> not
/// accepted (returns 0), the other states unchanged. Returns 1 otherwise.
/// A leaf with resume entries (the generated `__entries` body).
pub fn world_cache_streamer_request_section(c: &mut Cpu) {
    if c.pc != 0x003a_98c8 {
        return ee::s13::f_003a98c8__generated(c);
    }
    'accepted: {
        c.cycles += 4;
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x4));
        if c.g[V0][0] == 0 {
            break 'accepted;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(oc429_at(c.g[V0][0], 0x8));
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let out_of_range = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
        if out_of_range {
            break 'accepted;
        }
        c.cycles += 9;
        c.g[V1][0] = sx(oc429_at(c.g[A2][0], 0x14));
        c.g[V0][0] = sx(oc429_at(c.g[A2][0], c.g[A0][0] as u32));
        c.g[A1][0] = c.g[V1][0];
        c.swc1(12, oc429_at(c.g[V0][0], 0x18));
        c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = (c.g[V1][0] < 7) as u64;
        let bad_state = c.g[V0][0] == 0;
        c.g[A3][0] = c.g[A0][0];
        if bad_state {
            break 'accepted;
        }
        c.cycles += 7;
        c.g[V0][0] = 0x49_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx(oc429_at(c.g[V0][0], 0x4f10));
        c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let target = c.g[A0][0] as u32;
        match target {
            // state 0: becomes 1.
            0x003a_9928 => {
                c.cycles += 4;
                c.g[V1][0] = sx(oc429_at(c.g[A1][0], c.g[A3][0] as u32));
                c.g[V0][0] = 1;
                let ra = c.g[RA][0] as u32;
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.pc = ra;
                return;
            }
            // state 3: becomes 5.
            0x003a_9938 => {
                c.cycles += 4;
                c.g[V1][0] = sx(oc429_at(c.g[A1][0], c.g[A3][0] as u32));
                c.g[V0][0] = 5;
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                break 'accepted;
            }
            // state 4: refused.
            0x003a_9948 => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = 0;
                c.pc = ra;
                return;
            }
            0x003a_9950 => break 'accepted,
            _ => {
                c.jump_out(target, 0x003a_98c8, 0x003a_9958);
                return;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = 1;
    c.pc = ra;
}

/// `void WScriptMan_SpawnMissions(WScriptMan* self)` (0x308c60): purge the
/// playback location, fetch BE interface 0, and, in a game of mode 4
/// (`byte 0x535c10`) whose AI object (`world->game->+0xc`) has `+0x7c == 1`,
/// create every mission script of every world section
/// (`WScriptMan_CreateMission(self, section, def, (section << 16) + section,
/// 0)`, defs 0x54 bytes apart), then tell the playback manager the race
/// location. Always ends with `WScriptMan_RunMissionSetupScripts(self)`.
pub fn wscript_man_spawn_missions(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
        frame.save(c, S4, 0x40);
        frame.save(c, S7, 0x10);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S5, 0x30);
        frame.save(c, 22, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x84));
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x34));
        call_ptr(c, 0x0026_c898, 0x0030_8c9c)?; // cPlaybackMan_purgeLocation
        call_ptr(c, 0x0014_dc80, 0x0030_8ca4)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0030_8cb0)?; // BELibrary_GetInterface
        c.g[S7][0] = c.g[V0][0];
        c.g[V1][0] = 4;
        c.g[V0][0] = 0x53_0000;
        c.g[A0][0] = c.lb(oc429_at(c.g[V0][0], 0x5c10));
        let not_mode4 = c.g[A0][0] != c.g[V1][0];
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
        'setup: {
            if not_mode4 {
                break 'setup;
            }
            c.cycles += 6;
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw(oc429_at(c.g[A0][0], 0x84));
            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0xc));
            c.g[A0][0] = c.lw(oc429_at(c.g[V1][0], 0x7c));
            let not_race = c.g[A0][0] != c.g[A1][0];
            c.g[S2][0] = 0;
            if not_race {
                break 'setup;
            }
            'sections: {
                c.cycles += 7;
                c.g[V0][0] = c.lw(oc429_at(c.g[S4][0], 0x28c));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0x10));
                call_ptr(c, 0x003a_9800, 0x0030_8cf0)?; // cWorldView_getNumSections
                c.g[S6][0] = c.g[V0][0];
                let none = (c.g[S6][0] as i64) <= 0;
                c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
                if !none {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(oc429_at(c.g[S4][0], 0x28c));
                    loop {
                        'section: {
                            c.cycles += 9;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[S5][0] = sx(oc429_at(c.g[S2][0], 1));
                            call_ptr(c, 0x003a_6c68, 0x0030_8d0c)?; // cWorld_getNumMissionScripts
                            c.g[A0][0] = c.lw(oc429_at(c.g[S4][0], 0x28c));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x003a_6c98, 0x0030_8d1c)?; // cWorld_getMissionScripts
                            if (c.g[S0][0] as i64) <= 0 {
                                c.g[S2][0] = c.g[S5][0];
                                break 'section;
                            }
                            c.cycles += 3;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[S3][0] = sx((c.g[S2][0] as u32) << 16);
                            c.g[A2][0] = c.g[S1][0];
                            loop {
                                c.cycles += 9;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A3][0] = sx(oc429_at(c.g[S3][0], c.g[S2][0] as u32));
                                c.g[T0][0] = 0;
                                c.g[S1][0] = sx(oc429_at(c.g[S1][0], 0x54));
                                call_ptr(c, 0x0030_9568, 0x0030_8d48)?; // WScriptMan_CreateMission
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                                let more = c.g[S0][0] != 0;
                                c.g[A2][0] = c.g[S1][0];
                                if more {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030_8d30);
                                    }
                                    continue;
                                }
                                c.cycles += 1;
                                c.g[S2][0] = c.g[S5][0];
                                break 'section;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S6][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = c.lw(oc429_at(c.g[S4][0], 0x28c));
                            if event_due(c) {
                                deliver_event_here(c, 0x0030_8d00);
                            }
                            continue;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
                        break 'sections;
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S7][0];
            c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x84));
            c.g[S0][0] = c.lw(oc429_at(c.g[V1][0], 0x34));
            call_ptr(c, 0x0014_4bc0, 0x0030_8d78)?; // BENewRaceInt_GetTrackInfo
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0x34));
            call_ptr(c, 0x0026_c6e8, 0x0030_8d84)?; // cPlaybackMan_initLocation
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0030_8db8, 0x0030_8d8c)?; // WScriptMan_RunMissionSetupScripts
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_KillProcess(ScriptEngine* self, int id)` (0x282540):
/// tear down script process `id` (always returns 6). Nothing for a missing
/// or inactive process; otherwise by process type (jump table at
/// `0x481f90`): type 1 releases its table entry through vtable `+0x40`
/// unless the entry is set, types 3..5 stop the process' sequence
/// (`StopProcessSeq`), others nothing; then `vtbl +0x60(id, proc->+0xc)`, its
/// sequence object (`InSeq_InstanceSetUserData(obj, 0)`, `InSeq_FreeSequence`),
/// the table refcount (`vtbl +0x48` when it drops to 0), mark the slot type 7
/// and `ClearProcess`. Resume entries go to the generated `__entries`.
pub fn script_engine_kill_process(c: &mut Cpu) {
    if c.pc != 0x0028_2540 {
        return ee::s07::f_00282540__generated(c);
    }
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x0028_2cb0, 0x0028_2560)?; // ScriptEngine_GetProcess
        c.g[S0][0] = c.g[V0][0];
        let missing = c.g[S0][0] == 0;
        c.g[V0][0] = 6;
        'done: {
            if missing {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0x20));
            let inactive = c.g[V0][0] == 0;
            c.g[V0][0] = 6;
            if inactive {
                break 'done;
            }
            'release: {
                'by_type: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(oc429_at(c.g[S0][0], 0x4));
                    c.g[V0][0] = (c.g[V1][0] < 7) as u64;
                    let bad_type = c.g[V0][0] == 0;
                    c.sh(oc429_at(c.g[S0][0], 0x10), 0);
                    if bad_type {
                        break 'by_type;
                    }
                    'stop_seq: {
                        c.cycles += 7;
                        c.g[V0][0] = 0x48_0000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], 0x1f90));
                        c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                        let target = c.g[A0][0] as u32;
                        match target {
                            // Type 1: release the table entry unless it is set.
                            0x0028_25a4 => {
                                c.cycles += 7;
                                c.g[A1][0] = c.lw(oc429_at(c.g[S0][0], 0x14));
                                c.g[V0][0] = c.lw(oc429_at(c.g[S1][0], 0x2b0));
                                c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                                c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[V1][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                let set = c.g[A0][0] != 0;
                                c.g[V0][0] = c.lw(oc429_at(c.g[S1][0], 0x2a8));
                                if set {
                                    break 'release;
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x40));
                                c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x44));
                                let method = c.g[V1][0] as u32;
                                c.g[A0][0] = sx(oc429_at(c.g[S1][0], c.g[A0][0] as u32));
                                call_ptr(c, method, 0x0028_25d0)?;
                                c.g[V0][0] = c.lw(oc429_at(c.g[S1][0], 0x2a8));
                                break 'release;
                            }
                            // Types 3, 4, 5: stop the process' sequence.
                            0x0028_25d8 => break 'stop_seq,
                            // Types 0, 2, 6: nothing.
                            0x0028_25e4 => break 'by_type,
                            _ => {
                                c.jump_out(target, 0x0028_2540, 0x0028_2698);
                                return Ok(());
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0028_2338, 0x0028_25e4)?; // ScriptEngine_StopProcessSeq
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(oc429_at(c.g[S1][0], 0x2a8));
            }
            // vtbl +0x60: the process is going away.
            'seq: {
                c.cycles += 9;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.lw(oc429_at(c.g[S0][0], 0xc));
                c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x60));
                c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x64));
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx(oc429_at(c.g[S1][0], c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_2600)?;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0x8));
                    break 'seq;
                }
                c.cycles += 7;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0027_4c10, 0x0028_2614)?; // InSeq_InstanceSetUserData
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lw(oc429_at(c.g[S1][0], 0x2b8));
                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                call_ptr(c, 0x0027_29a8, 0x0028_2624)?; // InSeq_FreeSequence
                c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0x8));
            }
            'mark: {
                'unref: {
                    c.cycles += 2;
                    let no_entry = (c.g[A0][0] as i64) < 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    if no_entry {
                        break 'unref;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S1][0], 0x2b0));
                    c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[A0][0] as u32));
                    c.g[V1][0] = c.lhu(oc429_at(c.g[V0][0], 0x4));
                    c.g[V1][0] = sx(oc429_at(c.g[V1][0], 0u32.wrapping_sub(1)));
                    c.sh(oc429_at(c.g[V0][0], 0x4), c.g[V1][0]);
                    c.g[V1][0] &= 0xffff;
                    let still_used = c.g[V1][0] != 0;
                    c.g[V0][0] = 7;
                    if still_used {
                        break 'mark;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S1][0], 0x2a8));
                    c.g[A1][0] = c.lw(oc429_at(c.g[S0][0], 0x8));
                    c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x48));
                    c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x4c));
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(oc429_at(c.g[S1][0], c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0028_2668)?;
                }
                c.cycles += 1;
                c.g[V0][0] = 7;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.sw(oc429_at(c.g[S0][0], 0x4), c.g[V0][0]);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0028_2c88, 0x0028_267c)?; // ScriptEngine_ClearProcess
            c.g[V0][0] = 6;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InstancesMan_CopyInstance(InstancesMan* self, Instance* dst, Instance*
/// src)` (0x351270): copy the 0xa0-byte instance (`EA_MemCopy(dst, src,
/// 0xa0)`) but keep `dst`'s own `+0x78` field, and flag the copy (`+8 |=
/// 0x2000`).
pub fn instances_man_copy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = c.g[A2][0];
        c.g[S1][0] = c.lw(oc429_at(c.g[S0][0], 0x78));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xa0;
        call_ptr(c, 0x003e_6574, 0x0035_1298)?; // EA_MemCopy
        c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0x8));
        c.sw(oc429_at(c.g[S0][0], 0x78), c.g[S1][0]);
        c.g[V0][0] |= 0x2000;
        frame.restore(c, S1, 0x10);
        c.sw(oc429_at(c.g[S0][0], 0x8), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderCommand* WScriptFn_RiderCommand(RiderCommand* out, int n, Ent* e)`
/// (0x2ff850): script function "rider command". On first use initialises the
/// static default command (`0x4fbae8`: id -1, two zero words, flag byte).
/// Copies it to a 16-byte stack record, overlays the `n` (index, value, ...)
/// entries of `e` (0x10 bytes each: `+0` slot, `+4` value, `+0xc` kind; a
/// value is converted to float when the slot's type in the table at
/// `0x446530` is 2 and the entry's kind differs), defaults the rider id from
/// the global at `gp + 0xce8` and calls `cRider_flagEventWorld(rider, v1,
/// v2)`; returns `out` filled from the 12-byte record at `0x4c9098`.
pub fn wscript_fn_rider_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0x254c));
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 10;
            c.g[V0][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)));
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = c.lbu(oc429_at(c.g[V0][0], 0xc));
            c.g[A0][0] = 1;
            c.sw(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)), c.g[V1][0]);
            c.sb(oc429_at(c.g[V0][0], 0xc), c.g[A1][0]);
            c.sw(oc429_at(c.g[GP][0], 0x254c), c.g[A0][0]);
            c.sw(oc429_at(c.g[V0][0], 0x4), 0);
            c.sw(oc429_at(c.g[V0][0], 0x8), 0);
            c.sb(sp(c), c.g[A1][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(oc429_at(c.g[SP][0], 0x10));
        c.g[A1][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)));
        c.g[A2][0] = 0x10;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x002f_f8bc)?; // EA_MemCopy
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(oc429_at(c.g[V0][0], 0x6530));
            c.g[A3][0] = 2;
            loop {
                'stored: {
                    'int_store: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                        c.g[V1][0] = c.lw(oc429_at(c.g[A2][0], 0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx(oc429_at(c.g[V0][0], c.g[S0][0] as u32));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[T0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x4));
                            break 'int_store;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x4));
                            break 'int_store;
                        }
                        c.cycles += 4;
                        c.lwc1(0, oc429_at(c.g[A2][0], 0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        break 'stored;
                    }
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(1)));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(oc429_at(c.g[A2][0], 0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002f_f8d8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = u64::MAX;
        let have_rider = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
        if !have_rider {
            c.cycles += 5;
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0xce8));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        }
        c.cycles += 24;
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A1][0] = c.lw(oc429_at(c.g[S0][0], 0x4));
        c.g[V0][0] = c.lw(oc429_at(c.g[A0][0], 0xc));
        c.lwc1(12, oc429_at(c.g[S0][0], 0x8));
        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x28));
        call_ptr(c, 0x0010_f1c0, 0x002f_f964)?; // cRider_flagEventWorld
        c.g[V1][0] = 0x4d_0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(oc429_at(c.g[V1][0], 0u32.wrapping_sub(0x6f68)));
        c.ldl(A0, oc429_at(c.g[A3][0], 0x7));
        c.ldr(A0, oc429_at(c.g[A3][0], 0));
        c.g[A1][0] = c.lw(oc429_at(c.g[A3][0], 0x8));
        c.sdl(A0, oc429_at(c.g[S3][0], 0x7));
        c.sdr(A0, oc429_at(c.g[S3][0], 0));
        c.sw(oc429_at(c.g[S3][0], 0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `t3Vector& CameraBoundObj_GetPosition(CameraBoundObj* self)` (0x176440,
/// `cCTObject::pos`): the position vector is the object's first member, so
/// it returns `self`.
pub fn camera_bound_obj_get_position(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}

/// `void MultiParticleMan_AddInstance(MultiParticleMan* self, int idx, int
/// arg)` (0x357ae8): forward to `MultiParticle_AddInstance(self->particles[idx],
/// arg)`.
pub fn multi_particle_man_add_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(oc429_at(c.g[A0][0], c.g[A1][0] as u32));
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.lw(c.g[A0][0] as u32);
        call_ptr(c, 0x0035_81b8, 0x0035_7b04)?; // MultiParticle_AddInstance
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the two `BERewardInt_GetNext*Threshold` functions (they
/// differ in the tier callee, the threshold table and their return
/// addresses): the lowest threshold of `kind` (`a3`, table rows of 4 ints,
/// 16 bytes apart at `0x460000 - table_off`) whose place is better than the
/// player's current one (`IsBetterPlace(self, tier, i)`), 0 when none.
/// `tier_call`/`tier_ret`: the tier getter (called with the caller's a0..a3)
/// and where it returns; `better_ret`: the `IsBetterPlace` return address;
/// `loop_pc`: the loop head the event check resumes at.
fn oc429_next_threshold(
    c: &mut Cpu,
    tier_call: u32,
    tier_ret: u32,
    better_ret: u32,
    loop_pc: u32,
    table_off: u32,
) -> crate::call::Flow {
    c.cycles += 22;
    let frame = Frame::open(c, 0x90);
    frame.save(c, S4, 0x40);
    frame.save(c, S5, 0x30);
    c.g[S4][0] = c.g[A3][0];
    c.g[S5][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    frame.save(c, S1, 0x70);
    frame.save(c, S2, 0x60);
    frame.save(c, S3, 0x50);
    c.g[S2][0] = 0;
    frame.save(c, 22, 0x20);
    c.g[S0][0] = 0;
    frame.save(c, S7, 0x10);
    frame.save_ra(c, 0);
    c.g[S3][0] = 0;
    call_ptr(c, tier_call, tier_ret)?;
    c.g[S7][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[S4][0] as u32) << 4);
    c.g[V0][0] = 0x46_0000;
    c.g[S6][0] = sx(oc429_at(c.g[V0][0], 0u32.wrapping_sub(table_off)));
    c.g[S1][0] = sx(oc429_at(c.g[V1][0], c.g[S6][0] as u32));
    loop {
        'next: {
            c.cycles += 6;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S7][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0015_6a60, better_ret)?; // BERewardInt_IsBetterPlace
            if c.g[V0][0] == 0 {
                c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                break 'next;
            }
            'take: {
                c.cycles += 2;
                let none_yet = c.g[S2][0] == 0;
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 4);
                if !none_yet {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                        break 'next;
                    }
                }
                break 'take;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(oc429_at(c.g[S3][0], c.g[V1][0] as u32));
            c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S6][0] as u32));
            c.g[S2][0] = c.lw(c.g[V0][0] as u32);
            c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
        }
        c.cycles += 4;
        c.g[S3][0] = sx(oc429_at(c.g[S3][0], 4));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = sx(oc429_at(c.g[S1][0], 4));
        if more {
            if event_due(c) {
                deliver_event_here(c, loop_pc);
            }
            continue;
        }
        break;
    }
    c.cycles += 12;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, 22, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int BERewardInt_GetNextMissionThreshold(BEReward* self, int profile, int
/// character, int kind)` (0x158d58): the next mission-medal threshold to aim
/// for (table at `0x45b018`), see [`oc429_next_threshold`].
pub fn be_reward_int_get_next_mission_threshold(c: &mut Cpu) {
    guest(c, |c| {
        oc429_next_threshold(
            c,
            0x0015_8a50,
            0x0015_8d98,
            0x0015_8dc0,
            0x0015_8db0,
            0x4fe8,
        )
    })
}

/// `int BERewardInt_GetNextCollectibleThreshold(BEReward* self, int profile,
/// int character, int kind)` (0x158c80): the next collectible-medal
/// threshold (table at `0x45afe8`), see [`oc429_next_threshold`].
pub fn be_reward_int_get_next_collectible_threshold(c: &mut Cpu) {
    guest(c, |c| {
        oc429_next_threshold(
            c,
            0x0015_89b0,
            0x0015_8cc0,
            0x0015_8ce8,
            0x0015_8cd8,
            0x5018,
        )
    })
}

/// `int ScriptAnimBankMan_AddBank(Man* self, int id, void* data)` (0x275500,
/// `cScriptAnimBankManager::LinkBank`): returns the slot of the bank `id`.
/// A bank not yet linked takes a free entry, loads the bank object
/// (`cAdfDataSource` alloc + `0x311220(mem, data)`), counts its non-empty
/// items (`+0xc`), builds the item map `pTableAnimID` for them (`+0x10`) and
/// registers the bank in the global anim database table (`gp + 0xd08`, by
/// the entry's slot byte `+4`).
pub fn script_anim_bank_man_add_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0027_5718, 0x0027_5528)?; // ScriptAnimBankMan_FindBank
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0x4));
        } else {
            'items: {
                c.cycles += 19;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0027_57f0, 0x0027_553c)?; // ScriptAnimBankMan_FindFreeEntry
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0x48_0000;
                c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0x1a98));
                c.sw(oc429_at(c.g[S0][0], 0x8), c.g[S2][0]);
                c.g[A0][0] = 0x18;
                c.g[A3][0] = 0;
                c.g[A2][0] = c.lw(c.g[S3][0] as u32);
                call_ptr(c, 0x0031_7d70, 0x0027_5560)?; // EA_MemAllocNamed
                c.g[A1][0] = c.lw(oc429_at(c.g[S0][0], 0x8));
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0031_1220, 0x0027_556c)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x4));
                c.g[S1][0] = c.lh(oc429_at(c.g[V0][0], 0x2));
                let empty = (c.g[S1][0] as i64) <= 0;
                c.sw(oc429_at(c.g[S0][0], 0xc), 0);
                if empty {
                    break 'items;
                }
                // Count the non-empty items.
                c.cycles += 2;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[S1][0];
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x8));
                    c.g[V0][0] = sx(oc429_at(c.g[A1][0], c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let skip = c.g[V1][0] == 0;
                    c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0u32.wrapping_sub(1)));
                    if !skip {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0xc));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], 1));
                        c.sw(oc429_at(c.g[S0][0], 0xc), c.g[V0][0]);
                    }
                    c.cycles += 2;
                    let more = c.g[A0][0] != 0;
                    c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0x14));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_5588);
                        }
                        continue;
                    }
                    break;
                }
            }
            'slot: {
                c.cycles += 3;
                c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0xc));
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = c.lbu(oc429_at(c.g[S0][0], 0x4));
                    break 'slot;
                }
                'map: {
                    c.cycles += 10;
                    c.g[A2][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x48_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0x1aa8));
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7e30, 0x0027_55d4)?; // operator_new_named
                    c.sw(oc429_at(c.g[S0][0], 0x10), c.g[V0][0]);
                    c.g[A2][0] = 0;
                    let empty = (c.g[S1][0] as i64) <= 0;
                    c.g[A0][0] = 0;
                    if empty {
                        break 'map;
                    }
                    // map[k++] = i for each non-empty item i.
                    c.cycles += 1;
                    c.g[A1][0] = 0;
                    loop {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x8));
                        c.g[V0][0] = sx(oc429_at(c.g[A1][0], c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        let skip = c.g[V1][0] == 0;
                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                        if !skip {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw(oc429_at(c.g[S0][0], 0x10));
                            c.g[A2][0] = sx(oc429_at(c.g[A2][0], 1));
                            c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[V1][0] as u32));
                            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                        }
                        c.cycles += 4;
                        c.g[A0][0] = sx(oc429_at(c.g[A0][0], 1));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0x14));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0027_55e8);
                            }
                            continue;
                        }
                        break;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lbu(oc429_at(c.g[S0][0], 0x4));
            }
            c.cycles += 5;
            c.g[V1][0] = c.lw(oc429_at(c.g[GP][0], 0xd08));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[S2][0]);
            c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0x4));
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ssx_LoadNetModules()` (0x228ee0): load the network IOP modules through
/// the module loader (`0x319718(loader, path, wait, arglen, args)`, loader at
/// `gp + 0xda4`): USBD, USBKB, DEV9, (`0x319930`), INET, NETCNF (its argument
/// string built by `sprintf`), INETCTL, SMAP, PPP, SPDUART (another
/// `sprintf`), PPPOE (only when both the SMAP and the SPDUART load worked);
/// then, if any network adapter loaded (`gp - 0x704`), `yield(10)` and the
/// LIBNET / DRTYSCKF / LGAUD / VOIPF modules and the voice start/stop
/// (`0x3f6a40` or `0x3f6ab0`); finally `0x266788()`. All modules go through
/// `call_ptr`: the constants are string addresses in the `0x47xxxx` data.
pub fn ssx_load_net_modules(c: &mut Cpu) {
    guest(c, |c| {
        const LOAD: u32 = 0x0031_9718;
        const SPRINTF: u32 = 0x0041_7828;
        c.cycles += 77;
        let frame = Frame::open(c, 0x140);
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        frame.save(c, S0, 0x130);
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5678)));
        frame.save(c, S1, 0x120);
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x5668)));
        frame.save(c, S2, 0x110);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x100);
        c.g[A3][0] = 0x2b;
        call_ptr(c, LOAD, 0x0022_8f14)?; // USBD.IRX ("hub")
        c.g[S2][0] = 0;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5638)));
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x5628)));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0x10;
        call_ptr(c, LOAD, 0x0022_8f38)?; // USBKB.IRX ("keybd")
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5618)));
        c.g[T0][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, LOAD, 0x0022_8f54)?; // DEV9.IRX
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        call_ptr(c, 0x0031_9930, 0x0022_8f5c)?;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5608)));
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x55f8)));
        c.g[A2][0] = 1;
        c.g[A3][0] = 0x12;
        call_ptr(c, LOAD, 0x0022_8f7c)?; // INET.IRX ("mem=96KB")
                                         // sprintf(buf, "icon...", ...)
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = 0x4a_0000;
        c.g[S0][0] = sx(oc429_at(c.g[V1][0], 0u32.wrapping_sub(0x66e8)));
        c.g[S1][0] = sx(oc429_at(c.g[V0][0], 0x29f0));
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S1][0];
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x55e0)));
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = 0;
        call_ptr(c, SPRINTF, 0x0022_8fb0)?;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = c.g[SP][0];
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x55a8)));
        c.g[A3][0] = sx(oc429_at(c.g[V0][0], 1));
        c.g[A2][0] = 1;
        call_ptr(c, LOAD, 0x0022_8fcc)?; // NETCNF.IRX (buf)
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x48_0000;
        c.g[T0][0] = 0x48_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5598)));
        c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x5588)));
        c.g[A2][0] = 1;
        c.g[A3][0] = 0x14;
        call_ptr(c, LOAD, 0x0022_8fec)?; // INETCTL.IRX ("-no_auto")
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5570)));
        c.sw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)), 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, LOAD, 0x0022_900c)?; // SMAP.IRX
        let smap_failed = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !smap_failed {
            c.cycles += 2;
            c.g[S2][0] = 1;
            c.sw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)), c.g[V0][0]);
        }
        // PPP, then the dial-up string and SPDUART.
        c.cycles += 23;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = 0x4a_0000;
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0x29f8));
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, LOAD, 0x0022_9038)?; // PPP.IRX
        c.g[A1][0] = 0x48_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5560)));
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = 0;
        call_ptr(c, SPRINTF, 0x0022_9054)?;
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
        c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5538)));
        c.g[A3][0] = sx(oc429_at(c.g[V0][0], 1));
        c.g[A2][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, LOAD, 0x0022_9070)?; // SPDUART.IRX (buf)
        let spd_failed = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !spd_failed {
            c.cycles += 1;
            c.sw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)), c.g[V0][0]);
        }
        // PPPOE when an adapter loaded and the SMAP one did.
        'pppoe: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)));
            if c.g[V0][0] == 0 {
                break 'pppoe;
            }
            c.cycles += 2;
            let no_smap = c.g[S2][0] == 0;
            c.g[A1][0] = 0x48_0000;
            if no_smap {
                break 'pppoe;
            }
            c.cycles += 6;
            c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
            c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5528)));
            c.g[A2][0] = 1;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, LOAD, 0x0022_90a8)?; // PPPOE.IRX
        }
        c.cycles += 5;
        c.g[A0][0] = 10;
        call_ptr(c, 0x003e_52b0, 0x0022_90b0)?; // THREAD_yieldticks(10)
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x704)));
        if c.g[V0][0] != 0 {
            'voip: {
                c.cycles += 35;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                call_ptr(c, 0x0031_97d0, 0x0022_90c4)?;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = 0x48_0000;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5518)));
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, LOAD, 0x0022_90e0)?; // LIBNET.IRX
                c.g[A0][0] = 0;
                call_ptr(c, 0x0040_b130, 0x0022_90e8)?;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = 0x48_0000;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x5508)));
                c.g[T0][0] = 0;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                call_ptr(c, LOAD, 0x0022_9104)?; // DRTYSCKF.IRX
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = 0x48_0000;
                c.g[T0][0] = 0x48_0000;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x54f8)));
                c.g[T0][0] = sx(oc429_at(c.g[T0][0], 0u32.wrapping_sub(0x54e8)));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0x12;
                call_ptr(c, LOAD, 0x0022_9124)?; // LGAUD.IRX ("rpc=0")
                c.g[A1][0] = 0x48_0000;
                c.g[A0][0] = c.lw(oc429_at(c.g[GP][0], 0xda4));
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(0x54d0)));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, LOAD, 0x0022_9140)?; // VOIPF.IRX
                if (c.g[V0][0] as i64) >= 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x003f_6a40, 0x0022_9150)?;
                    break 'voip;
                }
                c.cycles += 2;
                call_ptr(c, 0x003f_6ab0, 0x0022_9160)?;
            }
        }
        c.cycles += 8;
        call_ptr(c, 0x0026_6788, 0x0022_9168)?;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// One camera-algorithm allocation of `ScriptCamController_StartScriptCamera`:
/// `ctor(operator_new(size, name, 0x20000000, 0))`. `cycles` is the block's
/// charge (10, 9 for the last kind), `name_off` the string's offset below
/// `0x460000`, `alloc_ret` / `ctor_ret` the return addresses.
fn oc429_script_cam_new(
    c: &mut Cpu,
    cycles: u64,
    size: u64,
    name_off: u32,
    ctor: u32,
    alloc_ret: u32,
    ctor_ret: u32,
) -> crate::call::Flow {
    c.cycles += cycles;
    c.g[A1][0] = 0x46_0000;
    c.g[A0][0] = size;
    c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(name_off)));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, alloc_ret)?; // EA_MemAllocNamed
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, ctor, ctor_ret)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// `void ScriptCamController_StartScriptCamera(ScriptCamController* self,
/// ScriptCam* cam)` (0x169190): build the camera algorithm for the script
/// camera's kind (`GetBlockPayload(cam->+0x14->+8)`: 0 manual, 1 target, 2
/// subject, 3 algorithmic; other kinds do nothing), hand it the camera
/// (`SetScriptObj`), store it at `cam + 0x1c`, push it on the controller
/// (`PushAlgorithm(self, algo, 1)`) and set `self + 0x18 = 1`.
pub fn script_cam_controller_start_script_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc429_at(c.g[S1][0], 0x14));
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x8));
        call_ptr(c, 0x0027_43c8, 0x0016_91b8)?; // InSeq_ChannelGetId
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let is1 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        // The kind: 0 manual, 1 target, 2 subject, 3 algorithmic.
        let kind: Option<u32> = if is1 {
            Some(1)
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let is2 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if is2 {
                    Some(2)
                } else {
                    c.cycles += 2;
                    let is3 = c.g[V1][0] == c.g[V0][0];
                    frame.restore(c, S0, 0x30);
                    if is3 {
                        Some(3)
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S1, 0x20);
                        None
                    }
                }
            } else {
                c.cycles += 2;
                let is0 = c.g[V1][0] == 0;
                frame.restore(c, S0, 0x30);
                if is0 {
                    Some(0)
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x20);
                    None
                }
            }
        };
        if let Some(kind) = kind {
            match kind {
                0 => oc429_script_cam_new(
                    c,
                    10,
                    0xb0,
                    0x4150,
                    0x0016_94b8,
                    0x0016_9210,
                    0x0016_9218,
                )?,
                1 => oc429_script_cam_new(
                    c,
                    10,
                    0xb0,
                    0x4130,
                    0x0016_9cf8,
                    0x0016_9238,
                    0x0016_9240,
                )?,
                2 => oc429_script_cam_new(
                    c,
                    10,
                    0xa0,
                    0x4110,
                    0x0016_a458,
                    0x0016_9260,
                    0x0016_9268,
                )?,
                _ => {
                    oc429_script_cam_new(c, 9, 0xa0, 0x40f0, 0x0016_ad38, 0x0016_9288, 0x0016_9290)?
                }
            }
            c.cycles += 12;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0016_94a8, 0x0016_92a0)?; // ScriptCameraAlgo_SetScriptObj
            c.sw(oc429_at(c.g[S1][0], 0x1c), c.g[S0][0]);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = 1;
            call_ptr(c, 0x0016_9340, 0x0016_92b4)?; // ScriptCamController_PushAlgorithm
            c.g[V0][0] = 1;
            c.sw(oc429_at(c.g[S2][0], 0x18), c.g[V0][0]);
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InSeq_InstanceSetup(InSeq* self, SeqData* data, Owner* owner, int
/// playerId)` (0x2745c0, `a0..a3`): initialise a sequence instance. Stores
/// `playerId` / `owner` / `data` (twice: `+4`, `+0x10`) and the defaults
/// (`+0x34 = -1`, `+0x38 = +0x1c = 0`), the extension block at `data + 0xc`
/// when the header is 13 bytes or more, and, for the `data->+1` channels,
/// allocates the channel array (`InSeq_HeapNewArray(owner + 8, 0x3c * n +
/// 0x10)`, the count stored in front, `InSeq_Channel_ctor` for each) and sets
/// each up from the entity data after the header (`InSeq_ChannelSetup`
/// returns the bytes it consumed). Returns `data->+8`.
pub fn in_seq_instance_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S5, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S5][0] = u64::MAX;
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, 22, 0x10);
        frame.save_ra(c, 0);
        c.sw(c.g[S2][0] as u32, c.g[A3][0]);
        c.sw(oc429_at(c.g[S2][0], 0x8), c.g[A2][0]);
        c.sw(oc429_at(c.g[S2][0], 0x4), c.g[A1][0]);
        c.sw(oc429_at(c.g[S2][0], 0x10), c.g[A1][0]);
        c.sw(oc429_at(c.g[S2][0], 0x34), c.g[S5][0]);
        c.sw(oc429_at(c.g[S2][0], 0x38), 0);
        c.sw(oc429_at(c.g[S2][0], 0x1c), 0);
        c.g[V0][0] = c.lhu(oc429_at(c.g[A1][0], 0x2));
        c.g[V0][0] = (c.g[V0][0] < 0xd) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = sx(oc429_at(c.g[A1][0], 0xc));
            c.sw(oc429_at(c.g[S2][0], 0x14), c.g[V0][0]);
        }
        c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
        'done: {
            c.cycles += 4;
            c.g[V0][0] = c.lhu(oc429_at(c.g[A0][0], 0x2));
            c.g[V1][0] = c.lbu(oc429_at(c.g[A0][0], 0x1));
            let no_channels = c.g[V1][0] == 0;
            c.g[S3][0] = sx(oc429_at(c.g[A1][0], c.g[V0][0] as u32));
            if no_channels {
                break 'done;
            }
            'array: {
                c.cycles += 12;
                c.g[S0][0] = c.g[V1][0];
                c.g[A0][0] = 0x3c;
                c.mult(0, A0, S0, A0);
                c.g[A1][0] = sx(oc429_at(c.g[A2][0], 0x8));
                c.g[S1][0] = sx(oc429_at(c.g[S0][0], 0u32.wrapping_sub(1)));
                c.g[S6][0] = sx(oc429_at(c.g[S2][0], 0x18));
                c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0x10));
                call_ptr(c, 0x0027_24c8, 0x0027_4654)?; // InSeq_HeapNewArray
                c.g[S4][0] = sx(oc429_at(c.g[V0][0], 0x10));
                c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                let one = c.g[S1][0] == c.g[S5][0];
                c.g[S0][0] = c.g[S4][0];
                if one {
                    break 'array;
                }
                c.cycles += 1;
                c.g[S5][0] = u64::MAX;
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx(oc429_at(c.g[S1][0], 0u32.wrapping_sub(1)));
                    call_ptr(c, 0x0027_3938, 0x0027_4674)?; // InSeq_Channel_ctor
                    let more = c.g[S1][0] != c.g[S5][0];
                    c.g[S0][0] = sx(oc429_at(c.g[S0][0], 0x3c));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4668);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 5;
            c.sw(c.g[S6][0] as u32, c.g[S4][0]);
            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
            c.g[V1][0] = c.lbu(oc429_at(c.g[V0][0], 0x1));
            let none = c.g[V1][0] == 0;
            c.g[S0][0] = 0;
            if none {
                break 'done;
            }
            c.cycles += 2;
            c.g[S1][0] = 0;
            loop {
                c.cycles += 12;
                c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x18));
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                c.g[A0][0] = sx(oc429_at(c.g[A0][0], c.g[S1][0] as u32));
                call_ptr(c, 0x0027_39b8, 0x0027_46b0)?; // InSeq_ChannelSetup
                c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
                c.g[S3][0] = sx(oc429_at(c.g[S3][0], c.g[V0][0] as u32));
                c.g[V0][0] = c.lbu(oc429_at(c.g[V1][0], 0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(oc429_at(c.g[S1][0], 0x3c));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4698);
                    }
                    continue;
                }
                break 'done;
            }
        }
        c.cycles += 12;
        c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
        frame.restore(c, S0, 0x70);
        c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x8));
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_UpdateTriggers(CameraTriggerMan* self, uint mask)`
/// (0x16cf48): walk the trigger list (`self->+0` array, `+4` count). A
/// trigger whose mask (`+4`) matches `mask` and that is not registered yet
/// (`+0x20 == 0`) registers itself (`+0x20 = trigger`) in the world's
/// spatial structure: the shape's bounds (virtual `+0x30`/`+0x34` of the
/// shape at `+8`) are converted (`0x328f28`) and inserted (`0x328c20`). A
/// registered trigger whose mask no longer matches is removed from it
/// (`0x3284b8`); once the removed node's entries (`+0x10` list, 8 words at
/// `+0..0x1c` then up to 3 more) are all empty the node is freed
/// (`__builtin_delete`, then `+0x10 = 0`), and the trigger is unregistered.
///
/// A dispatcher-shaped function (its generated body is a `match pc` loop):
/// the port handles the normal entry; any other `pc` (a resume after an
/// unwound callee or event) runs the generated body (at install: its
/// `__generated` alias).
pub fn camera_trigger_man_update_triggers(c: &mut Cpu) {
    if c.pc != 0x0016_cf48 {
        return ee::s02::f_0016cf48__generated(c);
    }
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, 22, 0x50);
        c.g[S3][0] = 0;
        frame.save(c, S7, 0x40);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save_ra(c, 0x30);
        c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x4));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S7][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            c.g[S5][0] = u64::MAX;
            c.g[V0][0] = c.lw(c.g[S6][0] as u32);
            loop {
                // 0x16cf90: trigger i.
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[V0][0] as u32));
                c.g[S2][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x4));
                c.g[V0][0] &= c.g[S7][0];
                let no_match = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x20));
                // `true`: fall to 0x16d174 (`cycles += 1; v0 = count`) next;
                // `false`: the insert path already loaded the count.
                let reload_count;
                if !no_match {
                    c.cycles += 2;
                    let registered = c.g[V0][0] != 0;
                    c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                    if registered {
                        reload_count = true;
                    } else {
                        // Register: bounds -> box, insert into the world.
                        c.cycles += 33;
                        c.sw(oc429_at(c.g[S2][0], 0x20), c.g[S2][0]);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A3][0] = c.lw(oc429_at(c.g[S2][0], 0x8));
                        c.g[S1][0] = sx(oc429_at(c.g[S2][0], 0x14));
                        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
                        c.g[V1][0] = c.lw(oc429_at(c.g[A3][0], 0x24));
                        c.g[A2][0] = c.lw(oc429_at(c.g[V0][0], 0x84));
                        c.g[A1][0] = c.lh(oc429_at(c.g[V1][0], 0x30));
                        c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x34));
                        c.g[S0][0] = c.lw(oc429_at(c.g[A2][0], 0x20));
                        let bounds = c.g[V0][0] as u32;
                        c.g[A1][0] = sx(oc429_at(c.g[A3][0], c.g[A1][0] as u32));
                        call_ptr(c, bounds, 0x0016_cfe4)?;
                        c.g[A0][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x0032_8f28, 0x0016_cff0)?;
                        oc429_pick_node(c);
                        c.g[A3][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        c.g[A1][0] = 2;
                        call_ptr(c, 0x0032_8c20, 0x0016_d030)?; // insert
                        c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x4));
                        reload_count = false;
                    }
                } else {
                    c.cycles += 2;
                    let unregistered = c.g[V0][0] == 0;
                    c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                    if unregistered {
                        reload_count = true;
                    } else {
                        // Remove from the world.
                        c.cycles += 39;
                        c.g[A3][0] = c.lw(oc429_at(c.g[S2][0], 0x8));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[V1][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
                        c.g[S1][0] = sx(oc429_at(c.g[S2][0], 0x14));
                        c.g[V0][0] = c.lw(oc429_at(c.g[A3][0], 0x24));
                        c.g[A2][0] = c.lw(oc429_at(c.g[V1][0], 0x84));
                        c.g[A1][0] = c.lh(oc429_at(c.g[V0][0], 0x30));
                        c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x34));
                        c.g[S0][0] = c.lw(oc429_at(c.g[A2][0], 0x20));
                        let bounds = c.g[V1][0] as u32;
                        c.g[A1][0] = sx(oc429_at(c.g[A3][0], c.g[A1][0] as u32));
                        call_ptr(c, bounds, 0x0016_d06c)?;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S0][0], 0xa0));
                        c.g[A0][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        c.g[A1][0] = c.g[SP][0];
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], 1));
                        c.sw(oc429_at(c.g[S0][0], 0xa0), c.g[V0][0]);
                        call_ptr(c, 0x0032_8f28, 0x0016_d084)?;
                        // (The node pick keeps the node in s0 and the id in a2.)
                        c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 0x24));
                        c.g[V1][0] = sx(oc429_at(c.g[S0][0], 0x50));
                        c.g[A0][0] = c.lw(oc429_at(c.g[SP][0], 0x28));
                        c.g[A2][0] = c.g[S1][0];
                        c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        c.g[T0][0] = c.lw(oc429_at(c.g[SP][0], 0x2c));
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = c.g[S0][0];
                        }
                        c.g[A0][0] = ((c.g[S5][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        c.g[V0][0] = sx(oc429_at(c.g[V1][0], 0x28));
                        c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                        if c.g[A0][0] != 0 {
                            c.g[V0][0] = c.g[V1][0];
                        }
                        c.g[A1][0] = 2;
                        c.g[S0][0] = sx(oc429_at(c.g[V0][0], 0x14));
                        c.g[A3][0] = sx(oc429_at(c.g[SP][0], 0x20));
                        if c.g[T0][0] != 0 {
                            c.g[S0][0] = c.g[V0][0];
                        }
                        c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0x10));
                        c.g[T0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_84b8, 0x0016_d0cc)?; // remove
                        c.g[V1][0] = c.lw(oc429_at(c.g[S0][0], 0x10));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        // Is the node's entry list empty? `found`: a non-empty
                        // word was seen (keep the node).
                        let mut found = false;
                        'probe: {
                            if c.g[V0][0] != 0 {
                                c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                                found = true;
                                break 'probe;
                            }
                            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18] {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], off));
                                if c.g[V0][0] != 0 {
                                    c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                                    found = true;
                                    break 'probe;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x1c));
                            if c.g[V0][0] != 0 {
                                c.cycles += 1;
                                c.g[S4][0] = sx(oc429_at(c.g[S3][0], 1));
                                found = true;
                                break 'probe;
                            }
                            c.g[V1][0] = sx(oc429_at(c.g[V1][0], 0x20));
                            c.cycles += 1;
                            c.g[A0][0] = 0;
                            loop {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                let nonzero = c.g[V0][0] != 0;
                                c.g[A0][0] = sx(oc429_at(c.g[A0][0], 1));
                                if nonzero {
                                    if back_edge_unwinds(c, 0x0016_d134) {
                                        return Ok(());
                                    }
                                    found = true;
                                    break 'probe;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                                let more = c.g[V0][0] != 0;
                                c.g[V1][0] = sx(oc429_at(c.g[V1][0], 4));
                                if more {
                                    if back_edge_unwinds(c, 0x0016_d140) {
                                        return Ok(());
                                    }
                                    continue;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 1;
                                break 'probe;
                            }
                        }
                        if found {
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                        }
                        // 0x16d15c: free the emptied node.
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(oc429_at(c.g[S0][0], 0x10));
                            call_ptr(c, OC429_DELETE, 0x0016_d16c)?;
                            c.sw(oc429_at(c.g[S0][0], 0x10), 0);
                        }
                        c.sw(oc429_at(c.g[S2][0], 0x20), 0);
                        reload_count = true;
                    }
                }
                if reload_count {
                    // 0x16d174
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x4));
                }
                // 0x16d178: next trigger.
                c.cycles += 4;
                c.g[S3][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    if back_edge_unwinds(c, 0x0016_cf90) {
                        return Ok(());
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, 22, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The register shuffle both paths of `CameraTriggerMan_UpdateTriggers` do
/// after the box conversion (`0x328f28`) of the "register" path: the box
/// extents in `sp + 0x24..0x2c` select the nodes `s0 + 0x50`, `s0` or
/// `+ 0x28`; the chosen node + 0x14 (or the node itself) ends in `a0`, the
/// trigger's `+0x14` in `a2` (conditional moves, as compiled).
fn oc429_pick_node(c: &mut Cpu) {
    c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 0x24));
    c.g[V1][0] = sx(oc429_at(c.g[S0][0], 0x50));
    c.g[A1][0] = c.lw(oc429_at(c.g[SP][0], 0x28));
    c.g[A2][0] = c.g[S1][0];
    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[T0][0] = c.lw(oc429_at(c.g[SP][0], 0x2c));
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[S0][0];
    }
    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(oc429_at(c.g[V1][0], 0x28));
    c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
    if c.g[A1][0] != 0 {
        c.g[A0][0] = c.g[V1][0];
    }
    c.g[A3][0] = sx(oc429_at(c.g[SP][0], 0x20));
    c.g[V0][0] = sx(oc429_at(c.g[A0][0], 0x14));
    if c.g[T0][0] == 0 {
        c.g[A0][0] = c.g[V0][0];
    }
}

/// `void SSXApp_ParseCommandLine(SSXApp* self, int argc, char** argv)`
/// (0x227bf0): scan the arguments from 1. An option (`-name`, matched with
/// `strcasecmp` in the order track, username, password, realfe, nohud,
/// autolobby) acts as follows: `-track <path>` stores the file name
/// without directory and extension at `self + 0xcc` (a `cBXString` built,
/// trimmed at the last `/`, `\` or `:` and at the last `.`); `-username`
/// and `-password` take the next argument (`cBXString` at `self + 0xc4` /
/// `0xc8`); `-realfe` sets `self + 0x60 = 1`, `-nohud` clears `self +
/// 0x64`, `-autolobby` sets `self + 0x74 = 1` (only when the last compare
/// result is 0). Finally `gp + 0x1450 = (self->+0x64 == 0)` (disable-HUD).
///
/// A dispatcher-shaped function (its generated body is a `match pc` loop):
/// the port is the same state machine, with the original's addresses as
/// state names; a resume `pc` other than the entry runs the generated body
/// (at install: its `__generated` alias).
pub fn ssx_app_parse_command_line(c: &mut Cpu) {
    if c.pc != 0x0022_7bf0 {
        return ee::s05::f_00227bf0__generated(c);
    }
    // The states (blocks of the original), named by address.
    const CHECK_ARG: u32 = 0x0022_7de0;
    const ARG_DASH: u32 = 0x0022_7dec;
    const OPTION: u32 = 0x0022_7dfc;
    const NEXT_ARG: u32 = 0x0022_7ddc;
    const TRACK_OPT: u32 = 0x0022_7c40;
    const TRACK_TRIM_DIR: u32 = 0x0022_7c94;
    const TRACK_FIND_DOT: u32 = 0x0022_7cc4;
    const TRACK_TRIM_EXT: u32 = 0x0022_7cdc;
    const TRACK_STORE: u32 = 0x0022_7d00;
    const IS_USERNAME: u32 = 0x0022_7d20;
    const USERNAME_ARG: u32 = 0x0022_7d34;
    const USERNAME_STORE: u32 = 0x0022_7d44;
    const IS_PASSWORD: u32 = 0x0022_7d58;
    const PASSWORD_ARG: u32 = 0x0022_7d6c;
    const PASSWORD_STORE: u32 = 0x0022_7d7c;
    const IS_REALFE: u32 = 0x0022_7d90;
    const SET_REALFE: u32 = 0x0022_7da4;
    const IS_NOHUD: u32 = 0x0022_7dac;
    const CLEAR_NOHUD: u32 = 0x0022_7dc0;
    const IS_AUTOLOBBY: u32 = 0x0022_7dc8;
    const SKIP: u32 = 0x0022_7dd8;
    const TRACK_NEXT: u32 = 0x0022_7e18;
    const FINISH: u32 = 0x0022_7e28;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        c.g[S2][0] = sx(oc429_at(c.g[A2][0], 4));
        frame.save(c, S4, 0x70);
        c.g[S3][0] = 1;
        frame.save(c, S5, 0x60);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 22, 0x50);
        c.g[S5][0] = sx(oc429_at(c.g[SP][0], 0x10));
        frame.save(c, S7, 0x40);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x30);
        c.g[S7][0] = 1;
        frame.save(c, S0, 0xb0);
        c.g[FP][0] = 0x48_0000;
        frame.save(c, S1, 0xa0);
        frame.save_ra(c, 0x20);
        let mut pc = CHECK_ARG;
        loop {
            match pc {
                CHECK_ARG => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V1][0] = 0x2d;
                    pc = if done { FINISH } else { ARG_DASH };
                }
                ARG_DASH => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.lb(c.g[A0][0] as u32);
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                        if back_edge_unwinds(c, NEXT_ARG) {
                            return Ok(());
                        }
                        pc = NEXT_ARG;
                    } else {
                        pc = OPTION;
                    }
                }
                OPTION => {
                    // `-track`?
                    c.cycles += 7;
                    c.g[S0][0] = sx(oc429_at(c.g[A0][0], 1));
                    c.g[V0][0] = 0x4a_0000;
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0x29b0));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_aa88, 0x0022_7e10)?; // strcasecmp
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x48_0000;
                    if other {
                        if back_edge_unwinds(c, IS_USERNAME) {
                            return Ok(());
                        }
                        pc = IS_USERNAME;
                    } else {
                        pc = TRACK_NEXT;
                    }
                }
                TRACK_NEXT => {
                    c.cycles += 4;
                    c.g[S3][0] = sx(oc429_at(c.g[S3][0], 1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    if more {
                        if back_edge_unwinds(c, TRACK_OPT) {
                            return Ok(());
                        }
                        pc = TRACK_OPT;
                    } else {
                        pc = FINISH;
                    }
                }
                TRACK_OPT => {
                    // `cBXString track(argv[i])`; separators `/`, `\`, `:`.
                    c.cycles += 21;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_82d8, 0x0022_7c4c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2f;
                    call_ptr(c, 0x0031_8ca0, 0x0022_7c58)?; // FindFirstOf
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x5c;
                    call_ptr(c, 0x0031_8ca0, 0x0022_7c68)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x3a;
                    call_ptr(c, 0x0031_8ca0, 0x0022_7c78)?;
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[V1][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S1][0];
                    }
                    let no_sep = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = c.lw(sp(c));
                    pc = if no_sep {
                        TRACK_FIND_DOT
                    } else {
                        TRACK_TRIM_DIR
                    };
                }
                TRACK_TRIM_DIR => {
                    // track = track.right(len - sep - 1).
                    c.cycles += 12;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A2][0] = c.lw(oc429_at(c.g[V0][0], 0u32.wrapping_sub(8)));
                    c.g[A2][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(c.g[V1][0] as u32)));
                    c.g[A2][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(1)));
                    call_ptr(c, 0x0031_9028, 0x0022_7cac)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0031_8448, 0x0022_7cb8)?; // operator=
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0022_7cc4)?; // ~cBXString
                    pc = TRACK_FIND_DOT;
                }
                TRACK_FIND_DOT => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2e;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0031_8c18, 0x0022_7cd4)?; // FindLastOf
                    let no_dot = (c.g[V0][0] as i64) < 0;
                    c.g[A1][0] = c.g[SP][0];
                    pc = if no_dot { TRACK_STORE } else { TRACK_TRIM_EXT };
                }
                TRACK_TRIM_EXT => {
                    // track = track.left(dot).
                    c.cycles += 9;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x0031_90a8, 0x0022_7ce8)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0031_8448, 0x0022_7cf4)?;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0022_7d00)?;
                    pc = TRACK_STORE;
                }
                TRACK_STORE => {
                    c.cycles += 8;
                    c.g[A0][0] = sx(oc429_at(c.g[S4][0], 0xcc));
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_8448, 0x0022_7d0c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0022_7d18)?;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = NEXT_ARG;
                }
                IS_USERNAME => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0u32.wrapping_sub(0x5828)));
                    call_ptr(c, 0x0041_aa88, 0x0022_7d2c)?;
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x48_0000;
                    pc = if other { IS_PASSWORD } else { USERNAME_ARG };
                }
                USERNAME_ARG | PASSWORD_ARG => {
                    // The option's value is the next argument.
                    c.cycles += 4;
                    c.g[S3][0] = sx(oc429_at(c.g[S3][0], 1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                    let missing = c.g[V0][0] == 0;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = if missing {
                        FINISH
                    } else if pc == USERNAME_ARG {
                        USERNAME_STORE
                    } else {
                        PASSWORD_STORE
                    };
                }
                USERNAME_STORE | PASSWORD_STORE => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    let field = if pc == USERNAME_STORE { 0xc4 } else { 0xc8 };
                    c.g[A0][0] = sx(oc429_at(c.g[S4][0], field));
                    call_ptr(
                        c,
                        0x0031_84e8,
                        if pc == USERNAME_STORE {
                            0x0022_7d50
                        } else {
                            0x0022_7d88
                        },
                    )?;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = NEXT_ARG;
                }
                IS_PASSWORD => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0u32.wrapping_sub(0x5818)));
                    call_ptr(c, 0x0041_aa88, 0x0022_7d64)?;
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x4a_0000;
                    pc = if other { IS_REALFE } else { PASSWORD_ARG };
                }
                IS_REALFE => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0x29b8));
                    call_ptr(c, 0x0041_aa88, 0x0022_7d9c)?;
                    let other = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x4a_0000;
                    pc = if other { IS_NOHUD } else { SET_REALFE };
                }
                SET_REALFE => {
                    c.cycles += 2;
                    c.sw(oc429_at(c.g[S4][0], 0x60), c.g[S7][0]);
                    pc = SKIP;
                }
                IS_NOHUD => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(oc429_at(c.g[V0][0], 0x29c0));
                    call_ptr(c, 0x0041_aa88, 0x0022_7db8)?;
                    let other = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    pc = if other { IS_AUTOLOBBY } else { CLEAR_NOHUD };
                }
                CLEAR_NOHUD => {
                    c.cycles += 2;
                    c.sw(oc429_at(c.g[S4][0], 0x64), 0);
                    pc = SKIP;
                }
                IS_AUTOLOBBY => {
                    c.cycles += 4;
                    c.g[A1][0] = sx(oc429_at(c.g[FP][0], 0u32.wrapping_sub(0x5808)));
                    call_ptr(c, 0x0041_aa88, 0x0022_7dd0)?;
                    if c.g[V0][0] == 0 {
                        c.sw(oc429_at(c.g[S4][0], 0x74), c.g[S7][0]);
                    }
                    pc = SKIP;
                }
                SKIP => {
                    c.cycles += 1;
                    c.g[S2][0] = sx(oc429_at(c.g[S2][0], 4));
                    pc = NEXT_ARG;
                }
                NEXT_ARG => {
                    c.cycles += 1;
                    c.g[S3][0] = sx(oc429_at(c.g[S3][0], 1));
                    pc = CHECK_ARG;
                }
                _ => {
                    // FINISH
                    debug_assert_eq!(pc, FINISH);
                    break;
                }
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw(oc429_at(c.g[S4][0], 0x64));
        frame.restore(c, S0, 0xb0);
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, 22, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        c.sw(oc429_at(c.g[GP][0], 0x1450), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0030_d540, rider_model_dtor, "RiderModel_dtor"),
    (0x0027_5ed0, script_list_add_choice, "ScriptList_AddChoice"),
    (
        0x0027_5920,
        data_instance_man_add_data,
        "DataInstanceMan_AddData",
    ),
    (0x0014_fad0, beb_bagt_int_reset, "BEBAGTInt_Reset"),
    (
        0x0022_8c08,
        ssx_app_apply_display_options,
        "SSXApp_ApplyDisplayOptions",
    ),
    (
        0x003a_98c8,
        world_cache_streamer_request_section,
        "cWorldCache_StreamerRequestSection",
    ),
    (
        0x0030_8c60,
        wscript_man_spawn_missions,
        "WScriptMan_SpawnMissions",
    ),
    (
        0x0028_2540,
        script_engine_kill_process,
        "ScriptEngine_KillProcess",
    ),
    (
        0x0035_1270,
        instances_man_copy_instance,
        "InstancesMan_CopyInstance",
    ),
    (
        0x002f_f850,
        wscript_fn_rider_command,
        "WScriptFn_RiderCommand",
    ),
    (
        0x0017_6440,
        camera_bound_obj_get_position,
        "CameraBoundObj_GetPosition",
    ),
    (
        0x0035_7ae8,
        multi_particle_man_add_instance,
        "MultiParticleMan_AddInstance",
    ),
    (
        0x0015_8d58,
        be_reward_int_get_next_mission_threshold,
        "BERewardInt_GetNextMissionThreshold",
    ),
    (
        0x0015_8c80,
        be_reward_int_get_next_collectible_threshold,
        "BERewardInt_GetNextCollectibleThreshold",
    ),
    (
        0x0027_5500,
        script_anim_bank_man_add_bank,
        "ScriptAnimBankMan_AddBank",
    ),
    (0x0022_8ee0, ssx_load_net_modules, "ssx_LoadNetModules"),
    (
        0x0016_9190,
        script_cam_controller_start_script_camera,
        "ScriptCamController_StartScriptCamera",
    ),
    (0x0027_45c0, in_seq_instance_setup, "InSeq_InstanceSetup"),
    (
        0x0016_cf48,
        camera_trigger_man_update_triggers,
        "CameraTriggerMan_UpdateTriggers",
    ),
    (
        0x0022_7bf0,
        ssx_app_parse_command_line,
        "SSXApp_ParseCommandLine",
    ),
];

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Base of the fixtures: above the fakes' pointer range
    /// (`DATA..DATA + 0x40000`), so a callee writing through a fake's result
    /// cannot hit a fixture.
    pub(crate) const OC429_FX: u32 = DATA + 0x8_0000;

    /// Gate word of this module's callee fakes.
    pub(crate) const OC429_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC429_MAGIC: u32 = 0x0c42_9a11;
    /// Result queues of the faked callees: `[head, len, values: u64 x 30]`,
    /// 0x100 bytes per callee, in the order of `OC429_CALLEES`.
    pub(crate) const OC429_QUEUES: u32 = OC429_CTRL + 0x100;

    pub(crate) fn oc429_queue_base(addr: u32) -> u32 {
        let i = OC429_CALLEES
            .iter()
            .position(|&a| a == addr)
            .expect("callee without a fake");
        OC429_QUEUES + 0x100 * i as u32
    }

    /// The faked callee `addr` returns `values` (64-bit, in v0) from its next
    /// calls, then its default hashed pointer.
    pub(crate) fn oc429_force(c: &mut Cpu, addr: u32, values: &[u64]) {
        let q = oc429_queue_base(addr);
        c.bus.write32(q, 0);
        c.bus.write32(q + 4, values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            c.bus.write64(q + 8 + 8 * i as u32, *v);
        }
    }

    fn oc429_pop(c: &mut Cpu, addr: u32) -> Option<u64> {
        let q = oc429_queue_base(addr);
        let head = c.bus.read32(q);
        if head < c.bus.read32(q + 4) {
            c.bus.write32(q, head + 1);
            Some(c.bus.read64(q + 8 + 8 * head))
        } else {
            None
        }
    }

    /// A gated callee fake: logs the call like `fakes::fake`, then returns the
    /// next queued value (if any) in v0.
    pub(crate) fn oc429_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if let Some(v) = oc429_pop(c, addr) {
            c.g[V0][0] = v;
        }
    }

    /// Scrambled machine with the gated fakes on (and `pc` at `entry`);
    /// `calls` bounds the call index at which a fake may unwind.
    pub(crate) fn oc429_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc429_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC429_CTRL, OC429_MAGIC);
        r
    }

    pub(crate) fn oc429_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    pub(crate) fn oc429_gp(off: i32) -> u32 {
        (GAME_GP as i32 + off) as u32
    }

    macro_rules! oc429_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC429_CTRL) != OC429_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    oc429_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once
            /// per process; with the gate off they run what the slots held).
            pub(crate) fn oc429_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc429_fakes! {
        (oc429_fake_0010f1c0, 0x0010_f1c0, s00, OVERRIDE_0010F1C0, f_0010f1c0__generated),
        (oc429_fake_00144bc0, 0x0014_4bc0, s01, OVERRIDE_00144BC0, f_00144bc0__generated),
        (oc429_fake_0014dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc429_fake_0014dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc429_fake_00154278, 0x0015_4278, s01, OVERRIDE_00154278, f_00154278__generated),
        (oc429_fake_00154588, 0x0015_4588, s01, OVERRIDE_00154588, f_00154588__generated),
        (oc429_fake_00156a60, 0x0015_6a60, s01, OVERRIDE_00156A60, f_00156a60__generated),
        (oc429_fake_001589b0, 0x0015_89b0, s01, OVERRIDE_001589B0, f_001589b0__generated),
        (oc429_fake_00158a50, 0x0015_8a50, s01, OVERRIDE_00158A50, f_00158a50__generated),
        (oc429_fake_00169340, 0x0016_9340, s02, OVERRIDE_00169340, f_00169340__generated),
        (oc429_fake_001694a8, 0x0016_94a8, s02, OVERRIDE_001694A8, f_001694a8__generated),
        (oc429_fake_001694b8, 0x0016_94b8, s02, OVERRIDE_001694B8, f_001694b8__generated),
        (oc429_fake_00169cf8, 0x0016_9cf8, s02, OVERRIDE_00169CF8, f_00169cf8__generated),
        (oc429_fake_0016a458, 0x0016_a458, s02, OVERRIDE_0016A458, f_0016a458__generated),
        (oc429_fake_0016ad38, 0x0016_ad38, s02, OVERRIDE_0016AD38, f_0016ad38__generated),
        (oc429_fake_0026d628, 0x0026_d628, s07, OVERRIDE_0026D628, f_0026d628__generated),
        (oc429_fake_0026e950, 0x0026_e950, s07, OVERRIDE_0026E950, f_0026e950__generated),
        (oc429_fake_00272488, 0x0027_2488, s07, OVERRIDE_00272488, f_00272488__generated),
        (oc429_fake_002729a8, 0x0027_29a8, s07, OVERRIDE_002729A8, f_002729a8__generated),
        (oc429_fake_00273938, 0x0027_3938, s07, OVERRIDE_00273938, f_00273938__generated),
        (oc429_fake_002739b8, 0x0027_39b8, s07, OVERRIDE_002739B8, f_002739b8__generated),
        (oc429_fake_002743c8, 0x0027_43c8, s07, OVERRIDE_002743C8, f_002743c8__generated),
        (oc429_fake_00275718, 0x0027_5718, s07, OVERRIDE_00275718, f_00275718__generated),
        (oc429_fake_002757f0, 0x0027_57f0, s07, OVERRIDE_002757F0, f_002757f0__generated),
        (oc429_fake_00275b98, 0x0027_5b98, s07, OVERRIDE_00275B98, f_00275b98__generated),
        (oc429_fake_00275cf8, 0x0027_5cf8, s07, OVERRIDE_00275CF8, f_00275cf8__generated),
        (oc429_fake_00277450, 0x0027_7450, s07, OVERRIDE_00277450, f_00277450__generated),
        (oc429_fake_002776e0, 0x0027_76e0, s07, OVERRIDE_002776E0, f_002776e0__generated),
        (oc429_fake_00277dd8, 0x0027_7dd8, s07, OVERRIDE_00277DD8, f_00277dd8__generated),
        (oc429_fake_002797a0, 0x0027_97a0, s07, OVERRIDE_002797A0, f_002797a0__generated),
        (oc429_fake_00282338, 0x0028_2338, s07, OVERRIDE_00282338, f_00282338__generated),
        (oc429_fake_00282bf0, 0x0028_2bf0, s07, OVERRIDE_00282BF0, f_00282bf0__generated),
        (oc429_fake_00282c88, 0x0028_2c88, s07, OVERRIDE_00282C88, f_00282c88__generated),
        (oc429_fake_00282cb0, 0x0028_2cb0, s07, OVERRIDE_00282CB0, f_00282cb0__generated),
        (oc429_fake_00283bb8, 0x0028_3bb8, s07, OVERRIDE_00283BB8, f_00283bb8__generated),
        (oc429_fake_00283d28, 0x0028_3d28, s07, OVERRIDE_00283D28, f_00283d28__generated),
        (oc429_fake_00283d70, 0x0028_3d70, s07, OVERRIDE_00283D70, f_00283d70__generated),
        (oc429_fake_00309568, 0x0030_9568, s10, OVERRIDE_00309568, f_00309568__generated),
        (oc429_fake_00317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc429_fake_00317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc429_fake_00317e50, 0x0031_7e50, s10, OVERRIDE_00317E50, f_00317e50__generated),
        (oc429_fake_00317e98, 0x0031_7e98, s10, OVERRIDE_00317E98, f_00317e98__generated),
        (oc429_fake_00318260, 0x0031_8260, s10, OVERRIDE_00318260, f_00318260__generated),
        (oc429_fake_003182d8, 0x0031_82d8, s10, OVERRIDE_003182D8, f_003182d8__generated),
        (oc429_fake_00318448, 0x0031_8448, s10, OVERRIDE_00318448, f_00318448__generated),
        (oc429_fake_003184e8, 0x0031_84e8, s10, OVERRIDE_003184E8, f_003184e8__generated),
        (oc429_fake_00319028, 0x0031_9028, s10, OVERRIDE_00319028, f_00319028__generated),
        (oc429_fake_003190a8, 0x0031_90a8, s10, OVERRIDE_003190A8, f_003190a8__generated),
        (oc429_fake_003284b8, 0x0032_84b8, s10, OVERRIDE_003284B8, f_003284b8__generated),
        (oc429_fake_00328c20, 0x0032_8c20, s10, OVERRIDE_00328C20, f_00328c20__generated),
        (oc429_fake_00328f28, 0x0032_8f28, s10, OVERRIDE_00328F28, f_00328f28__generated),
        (oc429_fake_003581b8, 0x0035_81b8, s11, OVERRIDE_003581B8, f_003581b8__generated),
        (oc429_fake_003a6c68, 0x003a_6c68, s13, OVERRIDE_003A6C68, f_003a6c68__generated),
        (oc429_fake_003a6c98, 0x003a_6c98, s13, OVERRIDE_003A6C98, f_003a6c98__generated),
        (oc429_fake_003a9800, 0x003a_9800, s13, OVERRIDE_003A9800, f_003a9800__generated),
        (oc429_fake_003e2580, 0x003e_2580, s14, OVERRIDE_003E2580, f_003e2580__generated),
        (oc429_fake_003e6448, 0x003e_6448, s14, OVERRIDE_003E6448, f_003e6448__generated),
        (oc429_fake_004166f4, 0x0041_66f4, s15, OVERRIDE_004166F4, f_004166f4__generated),
        (oc429_fake_0041aa88, 0x0041_aa88, s15, OVERRIDE_0041AA88, f_0041aa88__generated),
    }

    pub(crate) const OC429_CALLEES: &[u32] = &[
        0x0010_f1c0,
        0x0014_4bc0,
        0x0014_dc80,
        0x0014_dd58,
        0x0015_4278,
        0x0015_4588,
        0x0015_6a60,
        0x0015_89b0,
        0x0015_8a50,
        0x0016_9340,
        0x0016_94a8,
        0x0016_94b8,
        0x0016_9cf8,
        0x0016_a458,
        0x0016_ad38,
        0x0026_d628,
        0x0026_e950,
        0x0027_2488,
        0x0027_29a8,
        0x0027_3938,
        0x0027_39b8,
        0x0027_43c8,
        0x0027_5718,
        0x0027_57f0,
        0x0027_5b98,
        0x0027_5cf8,
        0x0027_7450,
        0x0027_76e0,
        0x0027_7dd8,
        0x0027_97a0,
        0x0028_2338,
        0x0028_2bf0,
        0x0028_2c88,
        0x0028_2cb0,
        0x0028_3bb8,
        0x0028_3d28,
        0x0028_3d70,
        0x0030_9568,
        0x0031_7d70,
        0x0031_7e30,
        0x0031_7e50,
        0x0031_7e98,
        0x0031_8260,
        0x0031_82d8,
        0x0031_8448,
        0x0031_84e8,
        0x0031_9028,
        0x0031_90a8,
        0x0032_84b8,
        0x0032_8c20,
        0x0032_8f28,
        0x0035_81b8,
        0x003a_6c68,
        0x003a_6c98,
        0x003a_9800,
        0x003e_2580,
        0x003e_6448,
        0x0041_66f4,
        0x0041_aa88,
    ];
    /// `RiderModel_dtor`: null and non-null buffers, 0..3 groups x variants,
    /// textures with negative and valid ids, `flags & 1`, unwinds in any call.
    #[test]
    fn rider_model_dtor_matches() {
        covered(ee::s10::f_0030d540__generated, rider_model_dtor, 120, &|c, seed| {
            let mut r = oc429_machine(c, seed, 0x30d540, 80, 0x0030_d540);
            let this = OC429_FX + 0x400;
            let gfx = OC429_FX + 0x800;
            let gfx_vt = OC429_FX + 0x900;
            let groups = OC429_FX + 0x2000;
            let tex = OC429_FX + 0x6000;
            oc429_w(c, GAME_GP + 0x2a90, gfx);
            oc429_w(c, gfx + 0x10d8, gfx_vt);
            c.bus.write16(gfx_vt + 0x380, (r.next_u32() as u16) & !3);
            oc429_w(c, gfx_vt + 0x384, fakes::FAKE);
            c.bus.write16(gfx_vt + 0x190, (r.next_u32() as u16) & !3);
            oc429_w(c, gfx_vt + 0x194, fakes::FAKE + 0x10);
            let nv = seed % 3;
            let ng = (seed / 3) % 3;
            // Plain buffers: each present or absent by seed bit.
            for (i, off) in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c]
                .iter()
                .enumerate()
            {
                let present = (r.next_u32() >> i) & 1 == 1 || seed % 7 == 0;
                oc429_w(
                    c,
                    this + off,
                    if present {
                        OC429_FX + 0x100 + 0x10 * i as u32
                    } else {
                        0
                    },
                );
            }
            oc429_w(c, this + RIDERMODEL_NUM_VARIANTS, nv);
            oc429_w(c, this + RIDERMODEL_NUM_GROUPS, ng);
            let has_groups = seed % 5 != 0;
            oc429_w(
                c,
                this + RIDERMODEL_GROUPS,
                if has_groups { groups } else { 0 },
            );
            for k in 0..4u32 {
                let nonnull = r.next_u32() & 1;
                oc429_w(
                    c,
                    this + 0x44 + 8 * k,
                    nonnull * (OC429_FX + 0x180 + 0x10 * k),
                );
            }
            // Per group j, slot k: variant array at +0x1c; entries 0x10 apart.
            for j in 0..3u32 {
                for k in 0..4u32 {
                    let arr = OC429_FX + 0x3000 + (j * 4 + k) * 0x40;
                    let slot = groups + j * 0x58 + k * 8;
                    oc429_w(c, slot + 0x1c, if r.next_u32() & 3 == 0 { 0 } else { arr });
                    for v in 0..3u32 {
                        oc429_w(
                            c,
                            arr + 0x10 * v,
                            if r.next_u32() & 1 == 0 {
                                0
                            } else {
                                OC429_FX + 0x500
                            },
                        );
                        oc429_w(
                            c,
                            arr + 0x10 * v + 0xc,
                            if r.next_u32() & 1 == 0 {
                                0
                            } else {
                                OC429_FX + 0x540
                            },
                        );
                    }
                }
            }
            let ntex = (seed / 2) % 4;
            oc429_w(c, this + RIDERMODEL_TEX_COUNT, ntex);
            oc429_w(
                c,
                this + RIDERMODEL_TEX_LIST,
                if seed % 4 == 1 { 0 } else { tex },
            );
            for i in 0..4u32 {
                let id = if r.next_u32() & 1 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() & 0x7fff
                };
                oc429_w(c, tex + 0xc * i, id);
            }
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (r.next_u32() & 0xffff_fffe | (seed & 1)) as u64;
        });
    }

    /// `ScriptList_AddChoice`: queue full, lookup failing / succeeding / off,
    /// every list mode, the "next script already set up" early exit, and the
    /// mode-3 movie / request-player readiness check.
    #[test]
    fn script_list_add_choice_matches() {
        covered(
            ee::s07::f_00275ed0__generated,
            script_list_add_choice,
            200,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x275ed0, 14, 0x0027_5ed0);
                let this = OC429_FX + 0x400;
                let proc_ = OC429_FX + 0x1000;
                // Queue at +0xc: head +0x98, tail +0x9c, full flag +0xa0.
                oc429_w(c, this + 0x98, r.next_u32() % 5);
                oc429_w(c, this + 0x9c, r.next_u32() % 5);
                oc429_w(c, this + 0xa0, (seed % 7 == 0) as u32);
                oc429_w(c, this + 0xa4, [0, 3, 3, 1][(seed % 4) as usize]);
                oc429_w(c, this + 0xbc, if seed % 3 == 0 { 0xffff_fffe } else { 2 });
                oc429_w(c, this, OC429_FX + 0x800);
                oc429_w(c, proc_ + 0x20, if seed % 5 < 2 { 4 } else { r.next_u32() });
                oc429_force(c, 0x0027_97a0, &[if seed % 5 == 0 { u64::MAX } else { 3 }]);
                oc429_force(c, 0x0028_3d28, &[1 + (seed % 4) as u64]);
                oc429_force(c, 0x0027_7dd8, &[(seed % 2) as u64]);
                oc429_force(c, 0x0028_2bf0, &[proc_ as u64]);
                oc429_force(c, 0x0028_3bb8, &[((seed / 2) % 2) as u64]);
                c.g[A0][0] = this as u64;
                for reg in [A1, A2, T0, T1, T2] {
                    c.g[reg][0] = r.next_u32() as u64;
                }
                c.g[A3][0] = (seed % 2) as u64 * (1 + (r.next_u32() as u64 % 3));
            },
        );
    }

    /// `DataInstanceMan_AddData`: both vtable paths (big file or not) and
    /// unwinds in each of the calls; the abort flag is cleared around the
    /// header-size query and restored.
    #[test]
    fn data_instance_man_add_data_matches() {
        covered(
            ee::s07::f_00275920__generated,
            data_instance_man_add_data,
            80,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x275920, 4, 0x0027_5920);
                let this = OC429_FX + 0x400;
                let vt = OC429_FX + 0x600;
                oc429_w(c, this + 0xc, vt);
                for (slot, fk) in [(0x10u32, 0u32), (0x20, 1)] {
                    c.bus.write16(vt + slot, (r.next_u32() as u16) & !3);
                    oc429_w(c, vt + slot + 4, fakes::FAKE + 0x10 * fk);
                }
                oc429_w(c, 0x0045_0dd4, r.next_u32());
                oc429_force(c, 0x0027_5cf8, &[(OC429_FX + 0x800) as u64]);
                oc429_force(c, 0x003e_2580, &[if seed % 2 == 0 { 0 } else { 0x40 }]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `BEBAGTInt_Reset`: copy 30 unaligned records (the source table sits at
    /// `0x4a788e`, 6 mod 8, so the 8-aligned `ld`/`sd` loop is dead code in the
    /// game: no input reaches it). LO/HI are compared too.
    #[test]
    fn beb_bagt_int_reset_matches() {
        check_lohi(ee::s01::f_0014fad0__generated, beb_bagt_int_reset, 6, &|c, seed| {
            let mut r = Rng(0x14fa_d000 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0014_fad0;
            fill(c, &mut r, 0x004a_7888, 0x7400);
            // Dirty destination (not all of it is written).
            fill(c, &mut r, 0x0053_0ec0, 0xc00);
        });
    }

    /// `SSXApp_ApplyDisplayOptions`: each display mode (0, 1, 2 and the
    /// out-of-range 3), with the options record at `0x535610` random; the
    /// two graphics-manager vtable calls and the two BE calls are fakes (any
    /// of them can unwind). The record is 8-aligned in the game, so only the
    /// `ld`/`sd` copy loops run.
    #[test]
    fn ssx_app_apply_display_options_matches() {
        covered(
            ee::s05::f_00228c08__generated,
            ssx_app_apply_display_options,
            80,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x228c08, 4, 0x0022_8c08);
                let gfx = OC429_FX + 0x800;
                let vt = OC429_FX + 0x900;
                oc429_w(c, oc429_gp(0x2a90), gfx);
                oc429_w(c, gfx + 0x10d8, vt);
                c.bus.write16(vt + 0x140, (r.next_u32() as u16) & !3);
                oc429_w(c, vt + 0x144, fakes::FAKE);
                c.bus.write16(vt + 0x60, (r.next_u32() as u16) & !3);
                oc429_w(c, vt + 0x64, fakes::FAKE + 0x10);
                fill(c, &mut r, SSXAPP_OPTIONS, 0xa2);
                let w0 = r.next_u32() & !(3 << 20) | (seed % 4) << 20;
                oc429_w(c, SSXAPP_OPTIONS, w0);
            },
        );
    }

    /// `cWorldCache_StreamerRequestSection`: no section table, index out of
    /// range (including an index with garbage in the upper half), every
    /// state 0..8 (the jump table at `0x494f10` is written as in the game).
    #[test]
    fn world_cache_streamer_request_section_matches() {
        check(
            ee::s13::f_003a98c8__generated,
            world_cache_streamer_request_section,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a98_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_98c8;
                for (i, t) in [
                    0x003a_9928u32,
                    0x003a_9950,
                    0x003a_9950,
                    0x003a_9938,
                    0x003a_9948,
                    0x003a_9950,
                    0x003a_9950,
                ]
                .iter()
                .enumerate()
                {
                    c.bus.write32(0x0049_4f10 + 4 * i as u32, *t);
                }
                let this = OC429_FX + 0x400;
                let info = OC429_FX + 0x500;
                c.bus
                    .write32(this + 4, if seed % 9 == 0 { 0 } else { info });
                c.bus.write32(info + 8, 4);
                for i in 0..5u32 {
                    c.bus.write32(this + 0x14 + 8 * i, (seed / 3 + i) % 9);
                    c.bus.write32(this + 0x18 + 8 * i, r.next_u32());
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = match seed % 7 {
                    0 => 0xffff_ffff_0000_0001,
                    6 => 4 + (r.next_u32() % 4) as u64,
                    n => (n % 4) as u64,
                };
                c.f[12] = r.next_u32();
            },
        );
    }

    /// `WScriptMan_SpawnMissions`: not in mode 4, mode 4 without the race AI
    /// flag, and the full spawn (0..2 sections of 0..3 mission scripts, then
    /// the real `PlaybackMan_Init` over fakes). The purge (a real
    /// `PlaybackMan_FreeData` on an empty manager) and the final
    /// `RunMissionSetupScripts` (no sections: its short path) run real.
    #[test]
    fn wscript_man_spawn_missions_matches() {
        covered(
            ee::s10::f_00308c60__generated,
            wscript_man_spawn_missions,
            120,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x308c60, 30, 0x0030_8c60);
                let world = OC429_FX + 0x300;
                let x = OC429_FX + 0x340;
                let ai = OC429_FX + 0x380;
                let playback = OC429_FX + 0x1000;
                let this = OC429_FX + 0x2000;
                let world_obj = OC429_FX + 0x2400;
                oc429_w(c, oc429_gp(-0x848), world);
                oc429_w(c, world + 0x84, x);
                oc429_w(c, x + 0x34, playback);
                oc429_w(c, x + 0xc, ai);
                // An empty playback manager (nothing to free).
                oc429_w(c, playback + 0x78, 0xffff_ffff);
                oc429_w(c, playback + 0x54, seed % 3);
                oc429_w(c, this + 0x28c, world_obj);
                oc429_w(c, world_obj, OC429_FX + 0x2500);
                let mode4 = seed % 4 != 0;
                c.bus
                    .write8(0x0053_5c10, if mode4 { 4 } else { (seed % 3) as u8 });
                let race = seed % 5 != 1;
                oc429_w(c, ai + 0x7c, race as u32);
                let sections = (seed % 3) as u64;
                if mode4 && race {
                    oc429_force(c, 0x003a_9800, &[sections, 0]);
                } else {
                    oc429_force(c, 0x003a_9800, &[0]);
                }
                let counts: Vec<u64> = (0..2).map(|_| (r.next_u32() % 4) as u64).collect();
                oc429_force(c, 0x003a_6c68, &counts);
                oc429_force(c, 0x0015_4278, &[(seed % 3) as u64]);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `ScriptEngine_KillProcess`: missing / inactive process, every process
    /// type 0..8 (jump table at `0x481f90`, written as in the game), table
    /// entry set or not, refcount reaching 0 or not, a sequence object or
    /// none, unwinds in every call.
    #[test]
    fn script_engine_kill_process_matches() {
        covered(
            ee::s07::f_00282540__generated,
            script_engine_kill_process,
            200,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x282540, 8, 0x0028_2540);
                for (i, t) in [
                    0x0028_25e4u32,
                    0x0028_25a4,
                    0x0028_25e4,
                    0x0028_25d8,
                    0x0028_25d8,
                    0x0028_25d8,
                    0x0028_25e4,
                ]
                .iter()
                .enumerate()
                {
                    c.bus.write32(0x0048_1f90 + 4 * i as u32, *t);
                }
                let this = OC429_FX + 0x400;
                let vt = OC429_FX + 0x600;
                let table = OC429_FX + 0x800;
                let proc_ = OC429_FX + 0xa00;
                let obj = OC429_FX + 0xb00;
                oc429_w(c, this + 0x2a8, vt);
                oc429_w(c, this + 0x2b0, table);
                oc429_w(c, this + 0x2b8, OC429_FX + 0xc00);
                for (slot, fk) in [(0x40u32, 0u32), (0x60, 1), (0x48, 2)] {
                    c.bus.write16(vt + slot, (r.next_u32() as u16) & !3);
                    oc429_w(c, vt + slot + 4, fakes::FAKE + 0x10 * fk);
                }
                // Table entries: [+0 flag, +4 refcount (u16)].
                for i in 0..4u32 {
                    oc429_w(c, table + 8 * i, if r.next_u32() & 1 == 0 { 0 } else { 1 });
                    c.bus
                        .write16(table + 8 * i + 4, 1 + (r.next_u32() % 2) as u16);
                }
                oc429_w(c, proc_ + 0x20, if seed % 9 == 1 { 0 } else { 1 });
                oc429_w(c, proc_ + 4, seed % 9);
                oc429_w(c, proc_ + 0xc, r.next_u32());
                oc429_w(c, proc_ + 0x14, r.next_u32() % 4);
                oc429_w(c, proc_, if r.next_u32() & 1 == 0 { 0 } else { obj });
                oc429_w(
                    c,
                    proc_ + 8,
                    if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 4
                    },
                );
                oc429_w(c, obj, OC429_FX + 0xd00);
                oc429_w(c, OC429_FX + 0xd00, r.next_u32());
                oc429_force(
                    c,
                    0x0028_2cb0,
                    &[if seed % 11 == 0 { 0 } else { proc_ as u64 }],
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `InstancesMan_CopyInstance` (the 0xa0-byte copy runs for real).
    #[test]
    fn instances_man_copy_instance_matches() {
        check(
            ee::s11::f_00351270__generated,
            instances_man_copy_instance,
            40,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x351270, 0, 0x0035_1270);
                let dst = OC429_FX + 0x400;
                let src = OC429_FX + 0x800;
                fill(c, &mut r, dst, 0x28);
                fill(c, &mut r, src, 0x28);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = src as u64;
            },
        );
    }

    /// `WScriptFn_RiderCommand`: first call (static defaults written) and
    /// later calls, 0..4 entries (int stores and the int-to-float conversion
    /// for slots of type 2), the rider id defaulted from the global or given.
    /// `EA_MemCopy` runs for real; the rider call is a fake.
    #[test]
    fn wscript_fn_rider_command_matches() {
        covered(
            ee::s10::f_002ff850__generated,
            wscript_fn_rider_command,
            160,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x2ff850, 1, 0x002f_f850);
                let world = OC429_FX + 0x300;
                let x = OC429_FX + 0x340;
                let riders = OC429_FX + 0x700;
                oc429_w(c, oc429_gp(-0x848), world);
                oc429_w(c, world + 0x84, x);
                oc429_w(c, x + 0xc, riders);
                // `riders[idx] + 0x28` is the rider pointer: idx 0..3 at +0x28...
                for i in 0..4u32 {
                    oc429_w(c, riders + 0x28 + 4 * i, OC429_FX + 0x1000 + 0x10 * i);
                }
                // The default record, its flag, the global rider id.
                oc429_w(c, oc429_gp(0x254c), (seed % 3 != 0) as u32);
                fill(c, &mut r, 0x004f_bae8, 4);
                oc429_w(
                    c,
                    0x004f_bae8,
                    if seed % 4 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 4
                    },
                );
                oc429_w(c, oc429_gp(0xce8), OC429_FX + 0x1200);
                oc429_w(c, OC429_FX + 0x1200, r.next_u32() % 4);
                // Slot types.
                // (Slot 0, the rider index, never holds a float: it indexes a table.)
                for k in 0..4u32 {
                    let ty = if k == 0 {
                        [0u32, 1, 3][(r.next_u32() % 3) as usize]
                    } else {
                        r.next_u32() % 4
                    };
                    oc429_w(c, 0x0044_6530 + 4 * k, ty);
                }
                fill(c, &mut r, 0x004c_9098, 3);
                let n = (seed / 3) % 5;
                let ents = OC429_FX + 0x1400;
                for i in 0..n.max(1) {
                    let e = ents + 0x10 * i;
                    oc429_w(c, e, r.next_u32() % 4);
                    oc429_w(c, e + 4, r.next_u32() % 4);
                    oc429_w(c, e + 8, r.next_u32());
                    oc429_w(c, e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = (OC429_FX + 0x1800) as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = ents as u64;
            },
        );
    }

    /// `CameraBoundObj_GetPosition` returns its argument.
    #[test]
    fn camera_bound_obj_get_position_matches() {
        check(
            ee::s02::f_00176440__generated,
            camera_bound_obj_get_position,
            20,
            &|c, seed| {
                let mut r = Rng(0x17_6440 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0017_6440;
            },
        );
    }

    /// `MultiParticleMan_AddInstance`: the callee is a fake (any call can unwind).
    #[test]
    fn multi_particle_man_add_instance_matches() {
        covered(
            ee::s11::f_00357ae8__generated,
            multi_particle_man_add_instance,
            40,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x357ae8, 1, 0x0035_7ae8);
                let this = OC429_FX + 0x400;
                for i in 0..8u32 {
                    oc429_w(c, this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// Setup of the two `BERewardInt_GetNext*Threshold` tests: the table (4
    /// rows of 4 thresholds), the tier getter and `IsBetterPlace` as fakes
    /// with forced answers.
    pub(crate) fn oc429_threshold_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32, table: u32) {
        let mut r = oc429_machine(c, seed, salt, 5, entry);
        for i in 0..16u32 {
            // Positive thresholds; a few zero ones.
            let v = if r.next_u32() % 6 == 0 {
                0
            } else {
                r.next_u32() % 500
            };
            oc429_w(c, table + 4 * i, v);
        }
        let better: Vec<u64> = (0..4).map(|_| (r.next_u32() & 1) as u64).collect();
        oc429_force(c, 0x0015_6a60, &better);
        c.g[A0][0] = r.next_u32() as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = (r.next_u32() % 4) as u64;
    }

    #[test]
    fn be_reward_int_get_next_mission_threshold_matches() {
        covered(
            ee::s01::f_00158d58__generated,
            be_reward_int_get_next_mission_threshold,
            120,
            &|c, seed| oc429_threshold_setup(c, seed, 0x158d58, 0x0015_8d58, 0x0045_b018),
        );
    }

    #[test]
    fn be_reward_int_get_next_collectible_threshold_matches() {
        covered(
            ee::s01::f_00158c80__generated,
            be_reward_int_get_next_collectible_threshold,
            120,
            &|c, seed| oc429_threshold_setup(c, seed, 0x158c80, 0x0015_8c80, 0x0045_afe8),
        );
    }

    /// `ScriptAnimBankMan_AddBank`: bank already linked, and a new bank with
    /// 0..4 items (some empty), with and without any non-empty item.
    #[test]
    fn script_anim_bank_man_add_bank_matches() {
        covered(
            ee::s07::f_00275500__generated,
            script_anim_bank_man_add_bank,
            160,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x275500, 6, 0x0027_5500);
                let this = OC429_FX + 0x400;
                let entry = OC429_FX + 0x800;
                let bank = OC429_FX + 0x900;
                let hdr = OC429_FX + 0xa00;
                let items = hdr + 0x10;
                let table = OC429_FX + 0xd00;
                oc429_w(c, this, r.next_u32());
                oc429_w(c, oc429_gp(0xd08), table);
                oc429_w(c, entry + 4, r.next_u32() % 4);
                oc429_w(c, entry + 0xc, r.next_u32());
                // The bank object is built from `data` (= a2 = `hdr`) by the
                // real `cAdfDataSource_ctor` / `SetData`: header at `hdr`
                // (count at +2), items 0x14 apart from `hdr + 0x10`.
                let n = seed % 5;
                c.bus.write16(hdr + 2, n as u16);
                let mut any = false;
                for i in 0..4u32 {
                    let present = (seed / 5 + i) % 3 != 0;
                    any |= present && i < n;
                    oc429_w(
                        c,
                        items + 0x14 * i,
                        if present { OC429_FX + 0xe00 } else { 0 },
                    );
                }
                let _ = any;
                oc429_force(
                    c,
                    0x0027_5718,
                    &[if seed % 6 == 0 { entry as u64 } else { 0 }],
                );
                oc429_force(c, 0x0027_57f0, &[entry as u64]);
                oc429_force(c, 0x0031_7d70, &[bank as u64]);
                oc429_force(c, 0x0031_7e30, &[(OC429_FX + 0xc00) as u64]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = hdr as u64;
            },
        );
    }

    use ssx3_ee_rt::funcs::{register_functions, FnEntry};

    /// Set in the child process of the tests that need an empty function
    /// table (every callee then lands in the logging fake).
    pub(crate) const OC429_CHILD_ENV: &str = "SSX3_OC429_CHILD";

    /// Result queue of the module-loader stub: `[head, len, values: u64 ...]`.
    pub(crate) const OC429_LOAD_Q: u32 = DATA + 0x20_1000;

    /// `0x319718` (the IOP module loader): logs the call, then returns the
    /// next queued result (negative = load failed).
    pub(crate) fn oc429_stub_load(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_9718, ret);
        if c.pc != ret {
            return;
        }
        let head = c.bus.read32(OC429_LOAD_Q);
        if head < c.bus.read32(OC429_LOAD_Q + 4) {
            c.bus.write32(OC429_LOAD_Q, head + 1);
            c.g[V0][0] = c.bus.read64(OC429_LOAD_Q + 8 + 8 * head);
        }
    }

    pub(crate) static OC429_STUBS: [FnEntry; 1] = [FnEntry {
        start: 0x0031_9718,
        end: 0x0031_971c,
        f: oc429_stub_load,
    }];

    /// Run test `name` of this module in a child process (the same test
    /// binary, `--exact`) where `body` runs against the stub function table:
    /// a table swap in the shared test process would race with the other
    /// tests' `fresh()`.
    pub(crate) fn oc429_in_child(name: &str, body: fn()) {
        if std::env::var_os(OC429_CHILD_ENV).is_some() {
            return body();
        }
        let path = format!(
            "{}::{name}",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([path.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC429_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child failed:\n{text}"
        );
    }

    /// Queue `n` loader results, each failing (-1) with the given chance.
    pub(crate) fn oc429_load_results(c: &mut Cpu, r: &mut Rng, n: u32) {
        c.bus.write32(OC429_LOAD_Q, 0);
        c.bus.write32(OC429_LOAD_Q + 4, n);
        for i in 0..n {
            let v: u64 = if r.next_u32() % 3 == 0 { u64::MAX } else { 5 };
            c.bus.write64(OC429_LOAD_Q + 8 + 8 * i, v);
        }
    }

    pub(crate) fn oc429_lnm_checked() {
        covered(
            ee::s05::f_00228ee0__generated,
            ssx_load_net_modules,
            100,
            &|c, seed| {
                register_functions(&OC429_STUBS);
                let mut r = machine(c, seed, 0x228ee0, 30);
                c.pc = 0x0022_8ee0;
                oc429_w(c, oc429_gp(0xda4), r.next_u32());
                oc429_load_results(c, &mut r, 20);
            },
        );
    }

    /// `ssx_LoadNetModules`: every combination of failing SMAP / SPDUART
    /// loads (PPPOE, the voice modules and their start/stop), unwinds in any
    /// of the ~25 calls. Runs in a child process whose function table holds
    /// only the loader stub (all other callees are unported).
    #[test]
    fn ssx_load_net_modules_matches() {
        oc429_in_child("ssx_load_net_modules_matches", oc429_lnm_checked);
    }

    /// `ScriptCamController_StartScriptCamera`: every script camera kind 0..3
    /// and the kinds that do nothing (negative, 4+); unwinds in each call.
    #[test]
    fn script_cam_controller_start_script_camera_matches() {
        covered(
            ee::s02::f_00169190__generated,
            script_cam_controller_start_script_camera,
            140,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x169190, 5, 0x0016_9190);
                let this = OC429_FX + 0x400;
                let cam = OC429_FX + 0x800;
                oc429_w(c, cam + 0x14, OC429_FX + 0x900);
                oc429_w(c, OC429_FX + 0x900 + 8, r.next_u32());
                let kind: u64 = [0, 1, 2, 3, 4, u64::MAX, 7, 0xffff_ffff][(seed % 8) as usize];
                oc429_force(c, 0x0027_43c8, &[kind]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = cam as u64;
            },
        );
    }

    /// `InSeq_InstanceSetup`: header sizes below / above 13 (extension
    /// block), 0..3 channels (array created and each channel set up), unwinds
    /// in any call.
    #[test]
    fn in_seq_instance_setup_matches() {
        covered(
            ee::s07::f_002745c0__generated,
            in_seq_instance_setup,
            160,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x2745c0, 8, 0x0027_45c0);
                let this = OC429_FX + 0x400;
                let data = OC429_FX + 0x600;
                let owner = OC429_FX + 0x800;
                c.bus.write8(data + 1, (seed % 4) as u8);
                c.bus
                    .write16(data + 2, if seed % 3 == 0 { 0x10 } else { 0xc });
                oc429_w(c, data + 8, r.next_u32());
                oc429_force(c, 0x0027_2488, &[(OC429_FX + 0xa00) as u64]);
                let sizes: Vec<u64> = (0..3).map(|_| 4 + (r.next_u32() % 8) as u64).collect();
                oc429_force(c, 0x0027_39b8, &sizes);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = data as u64;
                c.g[A2][0] = owner as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `CameraTriggerMan_UpdateTriggers`: 0..3 triggers, each matching the
    /// mask or not and registered or not (register / remove / skip), the
    /// four node choices from the box extents (random stack words), nodes
    /// with an empty or non-empty entry list (freed or kept), unwinds in
    /// each call.
    #[test]
    fn camera_trigger_man_update_triggers_matches() {
        covered(
            ee::s02::f_0016cf48__generated,
            camera_trigger_man_update_triggers,
            200,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x16cf48, 12, 0x0016_cf48);
                let this = OC429_FX + 0x400;
                let list = OC429_FX + 0x600;
                let world = OC429_FX + 0x300;
                let x = OC429_FX + 0x340;
                let y = OC429_FX + 0xa00;
                let shape_vt = OC429_FX + 0x740;
                let entries = OC429_FX + 0xc00;
                oc429_w(c, oc429_gp(-0x848), world);
                oc429_w(c, world + 0x84, x);
                oc429_w(c, x + 0x20, y);
                // The node lists of every node choice.
                for node in [0u32, 0x14, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c] {
                    oc429_w(c, y + node + 0x10, entries);
                }
                oc429_w(c, y + 0xa0, r.next_u32());
                // Entry list: all empty, or one word set.
                let k = r.next_u32() % 12;
                for i in 0..11u32 {
                    oc429_w(c, entries + 4 * i, (i == k) as u32 * 0x1234);
                }
                c.bus.write16(shape_vt + 0x30, (r.next_u32() as u16) & !3);
                oc429_w(c, shape_vt + 0x34, fakes::FAKE);
                let n = seed % 4;
                oc429_w(c, this, list);
                oc429_w(c, this + 4, n);
                let mask = 1 + r.next_u32() % 3;
                for i in 0..3u32 {
                    let t = OC429_FX + 0x800 + 0x100 * i;
                    oc429_w(c, list + 4 * i, t);
                    oc429_w(c, t + 4, r.next_u32() % 4);
                    oc429_w(c, t + 8, OC429_FX + 0x700 + 0x10 * i);
                    oc429_w(c, OC429_FX + 0x700 + 0x10 * i + 0x24, shape_vt);
                    oc429_w(c, t + 0x14, r.next_u32());
                    oc429_w(c, t + 0x20, if r.next_u32() & 1 == 0 { 0 } else { t });
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = mask as u64;
            },
        );
    }

    /// `SSXApp_ParseCommandLine`: 1..5 arguments of every kind (`-track`,
    /// `-username`, `-password`, `-realfe`, `-nohud`, `-autolobby`, an unknown
    /// option, a plain word), options missing their value, the track path
    /// with and without a separator or an extension. The `cBXString`
    /// constructor / assignments and `strcasecmp` are fakes (`strcasecmp` with
    /// forced results: the test walks the argument list as the function does
    /// to queue them); the separator searches run for real.
    #[test]
    fn ssx_app_parse_command_line_matches() {
        covered(
            ee::s05::f_00227bf0__generated,
            ssx_app_parse_command_line,
            200,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x227bf0, 24, 0x0022_7bf0);
                let this = OC429_FX + 0x400;
                let argv = OC429_FX + 0x2000;
                let argc = 1 + (r.next_u32() % 5);
                // Kind per argument: 0 track, 1 username, 2 password, 3 realfe,
                // 4 nohud, 5 autolobby, 6 unknown option, 7 plain word.
                let kinds: Vec<u32> = (0..argc).map(|_| r.next_u32() % 8).collect();
                for i in 0..argc {
                    let s = OC429_FX + 0x2100 + 0x10 * i;
                    oc429_w(c, argv + 4 * i, s);
                    let dash = kinds[i as usize] != 7;
                    c.bus.write8(s, if dash { b'-' } else { b'a' });
                    c.bus.write8(s + 1, b'x');
                    c.bus.write8(s + 2, 0);
                }
                let mut strcmp: Vec<u64> = vec![];
                let mut i = 1;
                while i < argc {
                    let k = kinds[i as usize];
                    if k != 7 {
                        // strcasecmp: 1 for each option before the match, then 0.
                        for opt in 0..6u32 {
                            if opt == k {
                                strcmp.push(0);
                                break;
                            }
                            strcmp.push(1);
                        }
                        match k {
                            0 => {
                                i += 1;
                                if i >= argc {
                                    break;
                                }
                            }
                            1 | 2 => {
                                i += 1;
                                if i >= argc {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    i += 1;
                }
                oc429_force(c, 0x0041_aa88, &strcmp);
                // The track string object at the frame base (sp = SP - 0xc0):
                // its character data pointer (length word in front). The
                // `cBXString` search functions run for real on it; the
                // constructor is a fake, so the string is whatever is here.
                let text: &[u8] = [
                    &b"dir/trk.ext\0"[..],
                    &b"a\\b\\c.d\0"[..],
                    &b"c:foo\0"[..],
                    &b"plain\0"[..],
                    &b"x.y/z.w:v\0"[..],
                    &b".hidden\0"[..],
                ][(seed % 6) as usize];
                c.bus.write32(crate::difftest::SP - 0xc0, OC429_FX + 0x1000);
                oc429_w(c, OC429_FX + 0x1000 - 8, text.len() as u32 - 1);
                for (k, b) in text.iter().enumerate() {
                    c.bus.write8(OC429_FX + 0x1000 + k as u32, *b);
                }
                oc429_w(c, this + 0x64, r.next_u32() % 2);
                oc429_w(c, this + 0x60, r.next_u32());
                oc429_w(c, this + 0x74, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = argc as u64;
                c.g[A2][0] = argv as u64;
            },
        );
    }
}
