//! Staging module oc322 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc322`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0037_0b60, dynamic_emitter_ctor, "DynamicEmitter_ctor"),
    (
        0x0030_3130,
        wscript_fn_object_exists,
        "WScriptFn_ObjectExists",
    ),
    (0x0022_97d8, crowd_man2d_remove, "CrowdMan2d_Remove"),
    (
        0x002a_b150,
        stream_array_get_current_request,
        "StreamArray_GetCurrentRequest",
    ),
    (0x0037_0018, emitter_ctor, "Emitter_ctor"),
    (0x003b_7f60, sndstrm_set_pan, "SNDSTRM_SetPan"),
    (0x002a_9ed8, stream_buffer_set_pan, "StreamBuffer_SetPan"),
    (
        0x0013_6c40,
        rider_wipeout_state_enter,
        "RiderWipeoutState_Enter",
    ),
    (0x003d_72a0, spch_alloc_queue_slot, "SPCH_AllocQueueSlot"),
    (0x0035_9460, spline_modifier_ctor, "SplineModifier_ctor"),
];

/// The `gp` register as an address.
#[inline]
fn gp_addr(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `gp + off`.
#[inline]
fn gp_oc322(c: &Cpu, off: u32) -> u32 {
    gp_addr(c).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// DynamicEmitter_ctor (0x370b60)
// ---------------------------------------------------------------------------

/// The dynamic-emitter view's vtable.
pub(crate) const DYNAMIC_EMITTER_CTOR_VTABLE: u32 = 0x49_3118;
/// The 0x10-byte block `memset` clears.
pub(crate) const DYNAMIC_EMITTER_CTOR_BLOCK: u32 = 0x1e4;
/// The two 16-bit id slots (both marked free).
pub(crate) const DYNAMIC_EMITTER_CTOR_IDS: u32 = 0x1f4;

/// `sGmView* DynamicEmitter_ctor(sGmView* self)` (0x370b60): clear the view
/// header, install the vtable, construct the emitter base at `+0x20`, set the
/// default state words, mark both id slots free (`-1`) and zero the 0x10-byte
/// block.
pub fn dynamic_emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[V0][0] = sx(DYNAMIC_EMITTER_CTOR_VTABLE);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x0c), 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f8), c.g[V0][0]);
        call_ptr(c, 0x002f_7a68, 0x0037_0b90)?;
        c.g[V1][0] = u64::MAX; // -1: both id slots are free
        c.g[V0][0] = 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x178), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(DYNAMIC_EMITTER_CTOR_BLOCK));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x174), c.g[V0][0]);
        c.g[A2][0] = u64::MAX;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x170), 0);
        c.g[A1][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x17c), 0);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(DYNAMIC_EMITTER_CTOR_IDS));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1e0), 0);
        loop {
            c.cycles += 7;
            c.sh(c.g[V1][0] as u32, c.g[A2][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[A1][0] < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0037_0bc8);
                }
                continue;
            }
            c.cycles += 8;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x10;
            // `memset(self->block, 0, 0x10)`
            call_ptr(c, 0x0041_6210, 0x0037_0bf0)?;
            c.g[V0][0] = c.g[S0][0];
            frame.restore_ra(c, 0);
            frame.restore(c, 16, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_ObjectExists (0x303130)
// ---------------------------------------------------------------------------

/// `D_004A57B4`: the one-time initialisation flag of the shared id buffer.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_INIT: u32 = 0x26c4;
/// `D_004A3D18`: the script value-type table (indexed by the entry's slot).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_TYPES: u32 = 0xc28;
/// `D_004A47B8`: the id table head.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_TABLE: u32 = 0x16c8;
/// `D_004A28A8`: the world object the id `-1` maps to (`+0x290`).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_WORLD: u32 = 0xce8;
/// `D_004FBC50` (`D_3130`): the shared id buffer (3 words), `0x500000-0x43b0`.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_IDS: u32 = 0x004f_bc50;
/// The local copy of the ids (`buf`) and the 0x10-byte entry record
/// (`{int slot; int val; int pad; int type}`).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_BUF: u32 = 0x10;
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_ENTRY: u32 = 0x10;

/// `V3* WScriptFn_ObjectExists(V3* ret, int n, Ent* e)` (0x303130): the
/// script bridge's `ObjectExists`; copy the shared id buffer (`D_3130`),
/// overlay the `n` entries (`slot`, `value`, `type`; a float value when the
/// entry's type differs from the slot's script type and that type is 2), look
/// the first id up in the world's object table and report
/// `ret[2] = 1`, `*ret = object != 0 && object->field_0xc`, storing the flag
/// in the script object at `+8`.
pub fn wscript_fn_object_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 19, 0x30);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        frame.save(c, 16, 0x60);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_INIT));
        c.g[S3][0] = c.g[A2][0]; // the entry array
        c.g[S1][0] = c.g[A0][0]; // the script object
        c.g[S2][0] = c.g[A1][0]; // the entry count
        c.g[A2][0] = WSCRIPT_FN_OBJECT_EXISTS_IDS.wrapping_add(0x43b0) as u64;
        if c.g[V0][0] == 0 {
            // One-time init of the shared id buffer: `D_3130[0] = -1`,
            // `D_3130[1] = 1` and the tag byte copied onto itself.
            c.cycles += 10;
            c.g[V1][0] = sx(WSCRIPT_FN_OBJECT_EXISTS_IDS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(8));
            c.g[V0][0] |= 0xffff;
            c.sw(WSCRIPT_FN_OBJECT_EXISTS_IDS, c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb((c.g[V1][0] as u32).wrapping_add(8), c.g[A1][0]);
            c.sw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_INIT), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[A0][0]);
            c.sb(c.g[SP][0] as u32, c.g[A1][0]);
        }
        // `buf[0] = -1; memcpy(buf, D_3130, 12)` and the entry overlay.
        'overlay: {
            c.cycles += 10;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_BUF));
            c.g[V0][0] |= 0xffff;
            c.g[A1][0] = sx(WSCRIPT_FN_OBJECT_EXISTS_IDS);
            c.g[A2][0] = 0xc;
            c.sw(
                (c.g[SP][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_BUF),
                c.g[V0][0],
            );
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6574, 0x0030_31a8)?;
            let no_entries = (c.g[S2][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if no_entries {
                break 'overlay;
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_TYPES));
            c.g[A3][0] = 0x2; // the script type of a float slot
            loop {
                'store: {
                    'value: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32); // the slot
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc)); // type
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32); // the slot's type
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                            break 'value;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                            break 'value;
                        }
                        // A float value: `*(float*)d = (float)x->val`.
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        break 'store;
                    }
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_ENTRY));
                if !more {
                    break 'overlay;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_31c0);
                }
            }
        }
        // `look_3130(buf[0])`: the id -> object lookup.
        'lookup: {
            {
                c.cycles += 4;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let id_is_minus_one = c.g[V0][0] == 0;
                c.g[A1][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_TABLE));
                if id_is_minus_one {
                    // id -1: the world's own object.
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_WORLD));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    break 'lookup;
                }
                'table: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let absent = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if absent {
                        break 'table;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if found {
                        break 'lookup;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // Report: `*ret = obj && obj->field_0xc`, `ret[2] = 1`, and the flag
        // goes into the script object at `+8`.
        'report: {
            'nonnull: {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 1;
                    break 'nonnull;
                }
                'live: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let dead = c.g[V0][0] == 0;
                    c.g[S0][0] = 1;
                    if dead {
                        break 'live;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S0][0]);
                    call_ptr(c, 0x0022_6610, 0x0030_3284)?;
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    break 'report;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                break 'nonnull;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[V0][0]);
            call_ptr(c, 0x0022_6610, 0x0030_329c)?;
            c.sw(c.g[V0][0] as u32, 0);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdMan2d_Remove (0x2297d8)
// ---------------------------------------------------------------------------

/// The 64 crowd-instance slot words, at `self + 8`.
pub(crate) const CROWD_MAN2D_REMOVE_SLOTS: u32 = 0x8;

/// `void CrowdMan2d_Remove(self, int handle)` (0x2297d8): scan the 64 slot
/// words at `+8` for `handle` and clear the first match
/// (`CrowdMan2d_ClearSlot`). The generated body is a dispatcher and can be
/// re-entered at the loop head after an event unwind, so the replacement
/// keeps its blocks as states.
pub fn crowd_man2d_remove(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0022_97d8 => {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[A2][0] = c.g[A1][0]; // the handle to find
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A1][0] = 0; // the slot index
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(CROWD_MAN2D_REMOVE_SLOTS));
                    pc = 0x0022_97f4;
                }
                0x0022_97f0 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    pc = 0x0022_97f4;
                }
                0x0022_97f4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 64) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    pc = if done { 0x0022_9818 } else { 0x0022_9800 };
                }
                0x0022_9800 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let same = c.g[V0][0] == c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                    if same {
                        pc = 0x0022_980c;
                    } else {
                        if back_edge_unwinds(c, 0x0022_97f0) {
                            return Ok(());
                        }
                        pc = 0x0022_97f0;
                    }
                }
                0x0022_980c => {
                    c.cycles += 3;
                    Callee {
                        addr: 0x0022_9b90,
                        f: ee::s05::CrowdMan2d_ClearSlot,
                    }
                    .call(c, 0x0022_9814)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    pc = 0x0022_9818;
                }
                0x0022_9818 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0022_97d8, 0x0022_9820);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// StreamArray_GetCurrentRequest (0x2ab150)
// ---------------------------------------------------------------------------

/// The array of stream buffers, at `self + 8`.
pub(crate) const STREAM_ARRAY_GET_CURRENT_REQUEST_BUFFERS: u32 = 0x8;

/// `int StreamArray_GetCurrentRequest(self, int i)` (0x2ab150): the current
/// request of stream `i`, or `-1` when that slot is empty.
pub fn stream_array_get_current_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] =
            c.lw((c.g[A0][0] as u32).wrapping_add(STREAM_ARRAY_GET_CURRENT_REQUEST_BUFFERS));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = u64::MAX; // -1
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002a_a210,
                f: ee::s08::StreamBuffer_GetCurrentRequest,
            }
            .call(c, 0x002a_b178)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter_ctor (0x370018)
// ---------------------------------------------------------------------------

/// The emitter's vtable.
pub(crate) const EMITTER_CTOR_VTABLE: u32 = 0x49_3160;

/// `Emitter* Emitter_ctor(Emitter* self)` (0x370018): install the vtable,
/// construct the emitter base at `+0x10` and clear the two state words.
pub fn emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(EMITTER_CTOR_VTABLE);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18c), c.g[V0][0]);
        call_ptr(c, 0x002f_7a68, 0x0037_003c)?;
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x0c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x184), 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_SetPan (0x3b7f60)
// ---------------------------------------------------------------------------

/// The stream table `SNDSTRM_GetStream` indexes (relative to `0x510000`):
/// `0x50a0a0` the stream pointers, `0x50a92e` the stream count.
#[cfg(test)]
pub(crate) const SND_GETSTREAM_TABLE: u32 = 0x0050_a0a0;
#[cfg(test)]
pub(crate) const SND_GETSTREAM_COUNT: u32 = 0x0050_a92e;
/// The cached pan values in a stream record.
pub(crate) const SNDSTRM_SET_PAN_LEFT: u32 = 0xf4;
pub(crate) const SNDSTRM_SET_PAN_RIGHT: u32 = 0xf6;

/// `int SNDSTRM_SetPan(int handle, int left, int right)` (0x3b7f60): cache
/// the two pan values in the stream (`+0xf4`/`+0xf6`) and apply them to every
/// channel of the voice; `-8` when the handle has no stream.
pub fn sndstrm_set_pan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0]; // left
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0]; // right
        Callee {
            addr: 0x003b_6a38,
            f: ee::s13::SNDSTRM_GetStream,
        }
        .call(c, 0x003b_7f7c)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(0xffff_fff8); // -8: no such stream
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            c.sh(
                (c.g[V0][0] as u32).wrapping_add(SNDSTRM_SET_PAN_LEFT),
                c.g[S0][0],
            );
            c.sh(
                (c.g[V0][0] as u32).wrapping_add(SNDSTRM_SET_PAN_RIGHT),
                c.g[S1][0],
            );
            Callee {
                addr: 0x003b_8c00,
                f: ee::s13::SND_SetPan,
            }
            .call(c, 0x003b_7f9c)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_SetPan (0x2a9ed8)
// ---------------------------------------------------------------------------

/// The stream handle a `StreamBuffer` forwards to, at `+8`.
pub(crate) const STREAM_BUFFER_SET_PAN_HANDLE: u32 = 0x8;
/// The sound lock/unlock routine pointers (`D_0044FFxx`).
#[cfg(test)]
pub(crate) const STREAM_BUFFER_SET_PAN_LOCK_SLOT: u32 = 0x0044_c46c;
#[cfg(test)]
pub(crate) const STREAM_BUFFER_SET_PAN_UNLOCK_SLOT: u32 = 0x0044_c470;

/// `void StreamBuffer_SetPan(self, int left, int right)` (0x2a9ed8): set the
/// pan of the buffer's stream under the sound lock.
pub fn stream_buffer_set_pan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x003b_58a0, 0x002a_9efc)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STREAM_BUFFER_SET_PAN_HANDLE));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x003b_7f60, 0x002a_9f0c)?;
        call_ptr(c, 0x003b_58d8, 0x002a_9f14)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_Enter (0x136c40)
// ---------------------------------------------------------------------------

/// The wipeout motion's rider pointer and its sphere tree.
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_RIDER: u32 = 0x40;
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_TREE: u32 = 0x44;
/// The rider's root position (`+0x110`), surface normal (`+0x370`), velocity
/// (`+0x1e0`) and the air predictor (`+0x788`).
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_POS: u32 = 0x110;
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_UP: u32 = 0x370;
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_VEL: u32 = 0x1e0;
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_AIR: u32 = 0x788;
/// The motion's own copy of the position (`+0x50`, `+0x60`).
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_POS_A: u32 = 0x50;
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_POS_B: u32 = 0x60;
/// The launch speed literal (`3333.3335f`) in the `.lit4` pool at `gp-0x72c0`.
#[cfg(test)]
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_LAUNCH_SPEED: u32 = 0x4550_5556;

/// `void RiderWipeoutState_Enter(motion, int launched)` (0x136c40): enter the
/// wipeout rider state; create the motion's sphere tree on first use, copy
/// the rider's position into the motion, and either pull the rider back along
/// the surface normal (a plain crash) or kick off the launch predictor.
pub fn rider_wipeout_state_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_TREE));
        let has_tree = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !has_tree {
            c.cycles += 9;
            c.g[A0][0] = 0x2b0;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x7df0)); // &D_00458210
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0013_6c78)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0032_9910, 0x0013_6c80)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_TREE),
                c.g[V0][0],
            );
        }
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
        let rider = c.g[A0][0] as u32;
        c.lq(2, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS));
        c.g[A1][0] = sx(rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS));
        c.sq(
            2,
            (c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS_A),
        );
        c.lq(3, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS));
        let launched = c.g[S1][0] != 0;
        c.sq(
            3,
            (c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS_B),
        );
        if launched {
            c.cycles += 6;
            c.g[V0][0] = 1;
            c.g[A2][0] = sx(rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.lwc1(12, gp_oc322(c, 0xffff_8d40)); // -0x72c0
            c.g[A0][0] = c.lw(rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_AIR));
            call_ptr(c, 0x0011_35b8, 0x0013_6d14)?;
        } else {
            // `pos -= n * dot(pos, n)`: slide the rider back onto the surface.
            c.cycles += 23;
            c.lqc2(3, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL));
            c.lqc2(5, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_UP));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.mfc1(3, 0);
            c.lqc2(4, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_UP));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
            c.sq(2, c.g[SP][0] as u32);
            c.lqc2(
                3,
                (c.g[V1][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL),
            );
            c.lqc2(4, c.g[SP][0] as u32);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(
                5,
                (c.g[V1][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL),
            );
            c.sw(c.g[S0][0] as u32, 0);
        }
        c.cycles += 10;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
        c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x2dc), 0);
        frame.restore(c, 17, 0x30);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
        frame.restore(c, 16, 0x40);
        frame.restore_ra(c, 0x20);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2e0), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_AllocQueueSlot (0x3d72a0)
// ---------------------------------------------------------------------------

/// `D_0044FFF8`: the speech queue. Each of the 16 records is 0x60 bytes:
/// `+0x64` when it was queued, `+0x6a` the used flag, `+0x6b` the channel id,
/// `+0x6c` the "in flight" halfword, `+0x70` the request record (`+2` its
/// priority, `+4` its owner). The per-channel "skip" words are at `+0x40` and
/// the per-channel counters at `+0`.
#[cfg(test)]
pub(crate) const SPCH_ALLOC_QUEUE_SLOT_BASE: u32 = 0x0044_fff8;
pub(crate) const SPCH_ALLOC_QUEUE_SLOT_RECORD: u32 = 0x60;
pub(crate) const SPCH_ALLOC_QUEUE_SLOT_SLOTS: u64 = 16;

/// `int SPCH_AllocQueueSlot(int owner, int channel, int skip_own)` (0x3d72a0):
/// take a free queue record; failing that, steal the oldest expired one,
/// then one owned by `channel` and decrement its owner counter. Returns the
/// slot index or `-1`.
pub fn spch_alloc_queue_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0x0045_0000;
        frame.save(c, 20, 0x40);
        let base = sx((c.g[A3][0] as u32).wrapping_sub(8));
        c.g[V0][0] = base;
        frame.save(c, 19, 0x30);
        c.g[S4][0] = c.g[A0][0]; // the owner value
        frame.save(c, 18, 0x20);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a)); // &r[0].used
        frame.save(c, 17, 0x10);
        c.g[S2][0] = c.g[A1][0]; // the channel
        frame.save(c, 16, 0);
        c.g[S1][0] = u64::MAX; // the result
        frame.save(c, 31, 0x50);
        c.g[S3][0] = u64::MAX; // the slot to leave alone
        c.g[S0][0] = 0; // the index
                        // Phase 1: the first unused record.
        let mut found = false;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
            if c.g[V0][0] != 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[S0][0];
                found = true;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S0][0] < SPCH_ALLOC_QUEUE_SLOT_SLOTS) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SPCH_ALLOC_QUEUE_SLOT_RECORD));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_72e0);
            }
        }
        // The generated phase-2 success leaves through `break 'b_3d73f8`,
        // past the shared `cycles += 1; v0 = s1` tail (it sets v0 itself);
        // phase 1 and phase 3 leave through `'b_3d73f4`, which runs it.
        let mut stolen = false;
        if !found {
            // The per-channel slot to leave alone (only with `skip_own`).
            'skip: {
                c.cycles += 2;
                let no_skip = c.g[A2][0] == 0;
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(8));
                if no_skip {
                    break 'skip;
                }
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
            }
            c.cycles += 7;
            c.g[S0][0] = 0;
            Callee {
                addr: 0x003d_b9a0,
                f: ee::s14::SPCH_GetTime,
            }
            .call(c, 0x003d_731c)?;
            c.g[A1][0] = c.g[V0][0]; // the current time
            c.g[V0][0] = 0x0045_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x64));
            // Phase 2: a record that has been queued for at least its
            // priority's time (steal it and free it).
            'scan2: loop {
                'skip2: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(8));
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        break 'skip2;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(2));
                    if c.g[V1][0] == 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        break 'skip2;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        break 'skip2;
                    }
                    c.cycles += 2;
                    if c.g[S3][0] == c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        break 'skip2;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x003d_7c38,
                        f: ee::s14::SPCH_FreeQueueSlot,
                    }
                    .call(c, 0x003d_7374)?;
                    c.g[V0][0] = c.g[S1][0];
                    stolen = true;
                    break 'scan2;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[S0][0] < SPCH_ALLOC_QUEUE_SLOT_SLOTS) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SPCH_ALLOC_QUEUE_SLOT_RECORD));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_7330);
                }
            }
            if !stolen {
                // Phase 3: steal a record this channel already owns.
                c.cycles += 4;
                c.g[V0][0] = 0x0045_0000;
                c.g[S0][0] = 0;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
                c.g[A2][0] = c.g[A1][0];
                'scan3: loop {
                    'next3: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x6c));
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            break 'next3;
                        }
                        'try3: {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x70));
                            c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(4));
                            c.g[V0][0] = (c.g[S4][0] < c.g[V0][0]) as u64;
                            let too_high = c.g[V0][0] != 0;
                            c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x6b));
                            if too_high {
                                break 'try3;
                            }
                            c.cycles += 2;
                            if c.g[A0][0] != c.g[S2][0] {
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                                break 'next3;
                            }
                            c.cycles += 2;
                            let is_skip = c.g[S0][0] == c.g[S3][0];
                            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                            if is_skip {
                                break 'try3;
                            }
                            c.cycles += 7;
                            c.sb((c.g[A1][0] as u32).wrapping_add(0x6a), 0);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[S1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            break 'scan3;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        break 'next3;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < SPCH_ALLOC_QUEUE_SLOT_SLOTS) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(SPCH_ALLOC_QUEUE_SLOT_RECORD));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_7398);
                    }
                }
            }
        }
        if !stolen {
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 8;
        frame.restore(c, 31, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore(c, 16, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SplineModifier_ctor (0x359460)
// ---------------------------------------------------------------------------

/// The `cSplineModifier` vtable.
pub(crate) const SPLINE_MODIFIER_CTOR_VTABLE: u32 = 0x48_f250;
/// `D_004FF120`: the quad written to both `sMpQuad` slots (`+0xb0`, `+0xc0`).
#[cfg(test)]
pub(crate) const SPLINE_MODIFIER_CTOR_QUAD: u32 = 0x004f_f120;
/// The `tAddSplineData` fields the constructor reads.
pub(crate) const SPLINE_MODIFIER_CTOR_D_PATH: u32 = 0x4;
pub(crate) const SPLINE_MODIFIER_CTOR_D_A: u32 = 0x8;
pub(crate) const SPLINE_MODIFIER_CTOR_D_B: u32 = 0xc;
pub(crate) const SPLINE_MODIFIER_CTOR_D_SPEED: u32 = 0x10;
pub(crate) const SPLINE_MODIFIER_CTOR_D_ANGLE: u32 = 0x14;
pub(crate) const SPLINE_MODIFIER_CTOR_D_OVERRIDE: u32 = 0x18;
pub(crate) const SPLINE_MODIFIER_CTOR_D_JITTER: u32 = 0x1c;
pub(crate) const SPLINE_MODIFIER_CTOR_D_FLAG: u32 = 0x20;
pub(crate) const SPLINE_MODIFIER_CTOR_D_SCALE: u32 = 0x24;
pub(crate) const SPLINE_MODIFIER_CTOR_D_END: u32 = 0x28;
/// The `.lit4` constants: 0.0174533f (degrees to radians) at `gp-0x2a64` and
/// 0.1f at `gp-0x2a60`.
#[cfg(test)]
pub(crate) const SPLINE_MODIFIER_CTOR_DEG2RAD: u32 = 0x3c8e_fa36;
#[cfg(test)]
pub(crate) const SPLINE_MODIFIER_CTOR_TENTH: u32 = 0x3dcc_cccd;

/// `cSplineModifier* cSplineModifier_ctor(self, const tAddSplineData& sdata)`
/// (0x359460): build a spline modifier from a descriptor: the path and speed,
/// the per-frame angle in radians and the per-second speed scale, the initial
/// distance (0, or the descriptor's override plus a random jitter, clamped
/// into [0, path length]), the flags and the two default `sMpQuad`s.
pub fn spline_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, 16, 0x20);
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(SPLINE_MODIFIER_CTOR_VTABLE);
        frame.save_fpr(c, 20, 0x30);
        c.g[V1][0] |= 0xffff;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.g[A1][0]; // the descriptor
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd8), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xdc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xe4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0);
        c.lwc1(1, gp_oc322(c, 0xffff_d59c)); // -0x2a64: degrees -> radians
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_A));
        c.g[AT][0] = 0x42c8_0000; // 100.0f
        c.mtc1(1, 2);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_B));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_ANGLE),
        );
        c.mul_s(0, 0, 1);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x38));
        c.lwc1(
            1,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_SCALE),
        );
        c.mul_s(1, 1, 2);
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_END),
        );
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x44));
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_PATH));
        Callee {
            addr: 0x0034_51c0,
            f: ee::s11::SplineModifier_InitPath,
        }
        .call(c, 0x0035_94ec)?;
        c.lwc1(
            12,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_SPEED),
        );
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0035_9688,
            f: ee::s11::SplineModifier_SetSpeed,
        }
        .call(c, 0x0035_94f8)?;
        // The initial distance: 0 with a forward speed, else the path length.
        c.mtc1(0, 1);
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_SPEED),
        );
        c.c_le_s(1, 0);
        if c.fcc() {
            c.cycles += 2;
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x3c));
        } else {
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
            c.cycles += 1;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3c));
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_FLAG));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        c.mtc1(0, 20); // f20 = 0.0f
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.lwc1(
            0,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_OVERRIDE),
        );
        c.c_eq_s(0, 20);
        if !c.fcc() {
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3c));
        }
        // `dist += rand(-1, 1) * jitter`, clamped into [0, path length].
        c.cycles += 14;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
        c.mtc1(1, 12);
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 13);
        call_ptr(c, 0x0031_7830, 0x0035_955c)?;
        c.lwc1(
            1,
            (c.g[S1][0] as u32).wrapping_add(SPLINE_MODIFIER_CTOR_D_JITTER),
        );
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x3c));
        c.mul_s(0, 0, 1);
        c.add_s(2, 2, 0);
        c.c_lt_s(2, 20);
        let mut clamp = c.fcc();
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x3c));
        if !clamp {
            c.cycles += 5;
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
            c.c_lt_s(0, 2);
            let over = !c.fcc();
            c.g[A1][0] = 0x0050_0000;
            clamp = !over;
        }
        if clamp {
            c.cycles += 2;
            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x3c));
            c.g[A1][0] = 0x0050_0000;
        }
        c.cycles += 16;
        c.g[A2][0] = 1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xee0)); // &D_004FF120
        c.g[V0][0] = c.g[S0][0];
        c.lq(3, c.g[A1][0] as u32);
        frame.restore(c, 17, 0x10);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0xb0));
        frame.restore_ra(c, 0);
        c.lq(4, c.g[A1][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A2][0]);
        c.sq(4, (c.g[S0][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd4), 0);
        frame.restore(c, 16, 0x20);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    /// `DynamicEmitter_ctor`: the view is constructed over fresh memory; the
    /// real emitter base constructor and `memset` run.
    #[test]
    fn dynamic_emitter_ctor_matches() {
        check(
            ee::s12::f_00370b60__generated,
            dynamic_emitter_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x370b_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                fill(c, &mut r, DATA, 0x84);
            },
        );
    }

    /// `WScriptFn_ObjectExists`: the one-time id-buffer init, the entry
    /// overlay (matching type, float slot, int slot), and every id lookup
    /// outcome (`-1`, absent low byte, absent high byte, found live/dead).
    #[test]
    fn wscript_fn_object_exists_matches() {
        check(
            ee::s10::f_00303130__generated,
            wscript_fn_object_exists,
            240,
            &|c, seed| {
                let mut r = Rng(0x3031_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let obj = DATA + 0x2000;
                let entries = DATA + 0x2100;
                let vt = DATA + 0x2200;
                let x = DATA + 0x2300;
                let base = DATA + 0x2400;
                let high = DATA + 0x2800;
                let desc = DATA + 0x2c00;
                let target = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                let n = [-1i64, 0, 1, 2, 3][(seed % 5) as usize];
                c.g[A1][0] = n as u64;
                c.g[A2][0] = entries as u64;
                // The script value types of the three slots.
                for i in 0..4u32 {
                    c.bus.write32(GAME_GP + 0xc28 + 4 * i, i % 3);
                }
                c.bus.write32(GAME_GP + 0x26c4, seed as u32 % 2);
                // The shared id buffer: the all-ones id, or a real one whose
                // low/high bytes select the lookup outcome.
                let id = match seed % 5 {
                    0 => 0xffff_ffffu32, // the world object
                    1 => 0x0000_0101,    // lo=1, hi=1: found
                    2 => 0x0000_0103,    // lo=3: no such object
                    3 => 0x0000_0001,    // hi=0: no such entry
                    _ => 0x0000_0101,
                };
                c.bus.write32(WSCRIPT_FN_OBJECT_EXISTS_IDS, id);
                c.bus
                    .write32(WSCRIPT_FN_OBJECT_EXISTS_IDS + 4, r.next_u32());
                c.bus
                    .write32(WSCRIPT_FN_OBJECT_EXISTS_IDS + 8, r.next_u32());
                // The entries: slot, value, type (0..3).
                for i in 0..4u32 {
                    let e = entries + 16 * i;
                    c.bus.write32(e, (seed as u32 >> (2 * i)) % 3);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, (seed as u32 >> (4 + i)) % 4);
                }
                // The id -> object tables.
                c.bus.write32(GAME_GP + 0x16c8, x);
                c.bus.write32(x, x + 0x40);
                c.bus.write32(x + 0x48, base);
                c.bus.write32(GAME_GP + 0xce8, vt);
                c.bus
                    .write32(vt + 0x290, if seed % 3 == 0 { 0 } else { target });
                for lo in 0..4u32 {
                    c.bus
                        .write32(base + 4 * lo, if lo == 1 || lo == 2 { desc } else { 0 });
                }
                c.bus.write32(desc + 0x1c, high);
                for hi in 0..4u32 {
                    c.bus
                        .write32(high + 4 * hi, if hi == 1 { (target >> 2) << 8 } else { 0 });
                }
                c.bus
                    .write32(target + 0xc, if seed % 5 < 2 { 0 } else { 1 });
                c.bus.write32(target + 0x10, r.next_u32());
            },
        );
    }

    /// `CrowdMan2d_Remove`: the handle found at some slot (cleared) or in no
    /// slot (all 64 scanned).
    #[test]
    fn crowd_man2d_remove_matches() {
        check(
            ee::s05::f_002297d8__generated,
            crowd_man2d_remove,
            200,
            &|c, seed| {
                let mut r = Rng(0x2297_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0022_97d8;
                c.g[A0][0] = DATA as u64;
                let handle = 0x1000 + (seed % 8) as u32;
                c.g[A1][0] = handle as u64;
                let hit = if seed % 3 == 0 { None } else { Some(seed % 64) };
                for i in 0..64u32 {
                    let v = if Some(i) == hit { handle } else { r.next_u32() };
                    c.bus.write32(DATA + CROWD_MAN2D_REMOVE_SLOTS + 4 * i, v);
                }
            },
        );
    }

    /// `StreamArray_GetCurrentRequest`: an empty slot (`-1`) or a real stream
    /// buffer whose status is read.
    #[test]
    fn stream_array_get_current_request_matches() {
        check(
            ee::s08::f_002ab150__generated,
            stream_array_get_current_request,
            200,
            &|c, seed| {
                let mut r = Rng(0x2ab1_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let i = seed % 4;
                c.g[A1][0] = i as u64;
                let buffers = DATA + 0x100;
                c.bus.write32(DATA + 8, buffers);
                for k in 0..4u32 {
                    let empty = (seed + k) % 3 == 0;
                    c.bus.write32(
                        buffers + 4 * k,
                        if empty { 0 } else { DATA + 0x200 + 0x40 * k },
                    );
                    c.bus.write32(DATA + 0x208 + 0x40 * k, k);
                }
                // The sound lock routines the status path calls, and an empty
                // stream table (the status call returns at once).
                c.bus.write32(STREAM_BUFFER_SET_PAN_LOCK_SLOT, 0x0022_6610);
                c.bus
                    .write32(STREAM_BUFFER_SET_PAN_UNLOCK_SLOT, 0x0022_6610);
                c.bus.write8(SND_GETSTREAM_COUNT, 0);
            },
        );
    }

    /// `Emitter_ctor`: the emitter is constructed over fresh memory; the real
    /// base constructor runs.
    #[test]
    fn emitter_ctor_matches() {
        check(
            ee::s12::f_00370018__generated,
            emitter_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3700_1800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                fill(c, &mut r, DATA, 0x80);
            },
        );
    }

    /// `SNDSTRM_SetPan`: a missing stream (`-8`) and real streams of every
    /// handle in the table (the pan values are cached and `SND_SetPan` runs;
    /// its channel lookup finds none, so it returns at once).
    #[test]
    fn sndstrm_set_pan_matches() {
        check(
            ee::s13::f_003b7f60__generated,
            sndstrm_set_pan,
            200,
            &|c, seed| {
                let mut r = Rng(0x3b7f_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (seed % 5) as u64; // the stream handle
                c.g[A1][0] = r.next_u32() as u64; // left pan
                c.g[A2][0] = r.next_u32() as u64; // right pan
                let count = if seed % 3 == 0 { 0u8 } else { 5 };
                c.bus.write8(SND_GETSTREAM_COUNT, count);
                for h in 0..4u32 {
                    c.bus.write32(
                        SND_GETSTREAM_TABLE + 4 * h,
                        if h < count as u32 {
                            DATA + 0x1000 + 0x100 * h
                        } else {
                            0
                        },
                    );
                    c.bus.write32(DATA + 0x1004 + 0x100 * h, h);
                }
            },
        );
    }

    /// `StreamBuffer_SetPan`: the sound lock is taken, the buffer's stream
    /// gets the pan and the lock is released.
    #[test]
    fn stream_buffer_set_pan_matches() {
        check(
            ee::s08::f_002a9ed8__generated,
            stream_buffer_set_pan,
            200,
            &|c, seed| {
                let mut r = Rng(0x2a9e_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let handle = seed % 4;
                c.bus.write32(DATA + STREAM_BUFFER_SET_PAN_HANDLE, handle);
                // The lock routines SND_Lock / SND_Unlock call, and the stream
                // table the pan call looks the handle up in.
                c.bus.write32(STREAM_BUFFER_SET_PAN_LOCK_SLOT, 0x0022_6610);
                c.bus
                    .write32(STREAM_BUFFER_SET_PAN_UNLOCK_SLOT, 0x0022_6610);
                c.bus.write8(SND_GETSTREAM_COUNT, ((seed % 4) + 1) as u8);
                for h in 0..4u32 {
                    c.bus
                        .write32(SND_GETSTREAM_TABLE + 4 * h, DATA + 0x2000 + 0x100 * h);
                    c.bus.write32(DATA + 0x2004 + 0x100 * h, h);
                }
            },
        );
    }

    /// `RiderWipeoutState_Enter`: with and without a sphere tree (the pool
    /// allocator runs over a test heap) and both the crash (normal slide) and
    /// the launch (air predictor) paths.
    #[test]
    fn rider_wipeout_state_enter_matches() {
        check(
            ee::s01::f_00136c40__generated,
            rider_wipeout_state_enter,
            200,
            &|c, seed| {
                let mut r = Rng(0x136c_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let motion = DATA;
                let rider = DATA + 0x800;
                let has_tree = seed % 4 != 1;
                if !has_tree {
                    // `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap
                    // over test memory, for the allocation path.
                    c.g[A0][0] = 0x004f_f250;
                    c.g[A1][0] = (DATA + 0x4_0000) as u64;
                    c.g[A2][0] = 0x4_0000;
                    c.g[RA][0] = crate::difftest::RA as u64;
                    c.pc = 0x0031_ff60;
                    ee::s10::DLHeap_InitArena(c);
                }
                c.g[A0][0] = motion as u64;
                c.g[A1][0] = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(motion + 0x40, rider);
                c.bus
                    .write32(motion + 0x44, if has_tree { DATA + 0x1800 } else { 0 });
                for off in [0x110u32, 0x1e0, 0x370] {
                    put_vec(
                        c,
                        rider + off,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
                c.bus.write32(rider + 0x788, DATA + 0x2000);
                c.bus.write32(rider + 0x2dc, r.next_u32());
                c.bus.write32(rider + 0x2e0, r.next_u32());
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x72c0),
                    &[RIDER_WIPEOUT_STATE_ENTER_LAUNCH_SPEED],
                );
            },
        );
    }

    /// `SPCH_AllocQueueSlot`: a free record, an expired one (stolen and
    /// freed), the `skip_own` slot, a record this channel owns (stolen and
    /// counted down) and the exhausted queue (`-1`).
    #[test]
    fn spch_alloc_queue_slot_matches() {
        check(
            ee::s14::f_003d72a0__generated,
            spch_alloc_queue_slot,
            240,
            &|c, seed| {
                let mut r = Rng(0x3d72_a000 ^ seed as u64);
                scramble(c, &mut r);
                let mode = seed % 5;
                let arg0 = match mode {
                    3 => 0u64,   // owner below every record's owner value
                    4 => 0x8000, // owner above it
                    _ => ((seed % 6) * 3) as u64,
                };
                c.g[A0][0] = arg0;
                c.g[A1][0] = (seed % 4) as u64; // the channel
                c.g[A2][0] = (seed % 2) as u64; // skip this channel's own slot
                let base = SPCH_ALLOC_QUEUE_SLOT_BASE;
                // The per-channel "skip" words.
                for i in 0..16u32 {
                    c.bus.write32(base + 0x40 + 4 * i, i % 5);
                }
                for i in 0..16u32 {
                    let rec = base + SPCH_ALLOC_QUEUE_SLOT_RECORD * i;
                    // `c`: the used flag; mode 0 leaves a free record behind.
                    let used = if mode == 0 && i == 1 { 0 } else { 1 };
                    c.bus.write8(rec + 0x6a, used);
                    c.bus.write8(rec + 0x6b, (i % 4) as u8);
                    // `s8`: the "in flight" halfword (mode 4 leaves every record
                    // in flight, so no phase can take one).
                    let s8 = match mode {
                        0 => (i % 3) as u16,
                        4 => 1,
                        _ => 0,
                    };
                    c.bus.write16(rec + 0x6c, s8);
                    c.bus.write32(rec + 0x64, 1000 + i);
                    let req = DATA + 0x1000 + 0x10 * i;
                    let prio = if mode == 2 || mode == 3 {
                        0
                    } else {
                        1 + (i % 5)
                    };
                    c.bus.write16(req + 2, prio as u16);
                    c.bus.write16(req + 4, (i * 2) as u16);
                    c.bus.write32(rec + 0x70, req);
                }
            },
        );
    }

    /// `SplineModifier_ctor`: forward and backward speeds (the initial
    /// distance), a descriptor override, zero and non-zero jitter (the clamp
    /// and no-clamp paths), and every float edge case.
    #[test]
    fn spline_modifier_ctor_matches() {
        check(
            ee::s11::f_00359460__generated,
            spline_modifier_ctor,
            240,
            &|c, seed| {
                let mut r = Rng(0x3594_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let sdata = DATA + 0x400;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = sdata as u64;
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_PATH, r.next_u32() % 64);
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_A, r.next_u32());
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_B, r.next_u32());
                let speed = if seed % 3 == 0 {
                    r.f32(100.0) | 0x8000_0000
                } else {
                    r.f32(100.0)
                };
                c.bus.write32(sdata + SPLINE_MODIFIER_CTOR_D_SPEED, speed);
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_ANGLE, r.edge_f32());
                c.bus.write32(
                    sdata + SPLINE_MODIFIER_CTOR_D_OVERRIDE,
                    if seed % 3 == 0 { 0 } else { r.f32(20.0) },
                );
                c.bus.write32(
                    sdata + SPLINE_MODIFIER_CTOR_D_JITTER,
                    if seed % 7 == 0 { 0 } else { r.f32(10.0) },
                );
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_FLAG, (seed % 2) as u32);
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_SCALE, r.edge_f32());
                c.bus
                    .write32(sdata + SPLINE_MODIFIER_CTOR_D_END, r.edge_f32());
                // The `.lit4` constants the ctor and SetSpeed read, the RNG state
                // and the default `sMpQuad`.
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x2a64),
                    &[SPLINE_MODIFIER_CTOR_DEG2RAD],
                );
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x2a60),
                    &[SPLINE_MODIFIER_CTOR_TENTH],
                );
                c.bus.write32(0x004f_f030, r.next_u32() | 1);
                put_vec(
                    c,
                    SPLINE_MODIFIER_CTOR_QUAD,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
            },
        );
    }
}
