//! Staging module oc417 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc417`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0031_0ce8,
        rider_model_get_bone_mask,
        "RiderModel_GetBoneMask",
    ),
    (
        0x0030_3df8,
        wscript_fn_event_type_matches,
        "WScriptFn_EventTypeMatches",
    ),
    (
        0x0017_2fe0,
        camera_trigger_read_sphere_trigger_data,
        "CameraTrigger_ReadSphereTriggerData",
    ),
    (
        0x0031_0f18,
        anim_data_base_alloc_chan_cache,
        "AnimDataBase_AllocChanCache",
    ),
    (
        0x0015_1a88,
        be_init_char_profile_bolt_map,
        "BE_InitCharProfileBoltMap",
    ),
    (0x0027_3dc0, in_seq_channel_stop, "InSeq_ChannelStop"),
    (
        0x0030_6cb0,
        wscript_fn_multi_particle_control,
        "WScriptFn_MultiParticleControl",
    ),
    (
        0x0030_5478,
        wscript_fn_add_magnet_modifier,
        "WScriptFn_AddMagnetModifier",
    ),
    (
        0x0027_a0d8,
        script_engine_get_target_pose,
        "ScriptEngine_GetTargetPose",
    ),
    (0x0012_3640, rider_attach_to_script, "Rider_AttachToScript"),
    (
        0x0022_d598,
        section_man_request_unload,
        "SectionMan_RequestUnload",
    ),
    (0x0022_5338, c_luno_table_set, "cLunoTable_Set"),
];

// ---------------------------------------------------------------------------
// Small register-relative accessors (`lw off(reg)` and friends). They read the
// base register themselves, so the call never borrows `c` twice.

/// `lw dst, off(reg)` (sign-extended).
#[inline]
fn oc417_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lw(c)
}
/// `lh dst, off(reg)` (sign-extended).
#[inline]
fn oc417_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lh(c)
}
/// `lb dst, off(reg)` (sign-extended).
#[inline]
fn oc417_lb(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lb(c)
}
/// `lbu dst, off(reg)`.
#[inline]
fn oc417_lbu(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lbu(c)
}
/// `sw src, off(reg)`.
#[inline]
fn oc417_sw(c: &mut Cpu, reg: usize, off: u32, src: usize) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    Addr(a).sw(c, v);
}
/// `sw zero, off(reg)`.
#[inline]
fn oc417_sw0(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).sw(c, 0);
}
/// `sh src, off(reg)`.
#[inline]
fn oc417_sh(c: &mut Cpu, reg: usize, off: u32, src: usize) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    Addr(a).sh(c, v);
}
/// `sh zero, off(reg)`.
#[inline]
fn oc417_sh0(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).sh(c, 0);
}
/// `addu` of two registers' low words, sign-extended.
#[inline]
fn oc417_add(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------
// RiderModel_GetBoneMask (0x310ce8)

/// `cAnimModel::getGraftMask` field offsets.
pub(crate) const BONE_MASK_MODEL_PARTS: u32 = 0xc;
pub(crate) const BONE_MASK_MODEL_NUM_BONES: u32 = 0x10;
pub(crate) const BONE_MASK_MODEL_MAX_PART: u32 = 0x14;
pub(crate) const BONE_MASK_MODEL_PART_INDEX: u32 = 0x1c;
/// `sAnimModelPart` (0x58 bytes): first bone, morph slot, bone-name list,
/// bone count.
pub(crate) const BONE_MASK_PART_FIRST_BONE: u32 = 0x4;
pub(crate) const BONE_MASK_PART_MORPH: u32 = 0xc;
pub(crate) const BONE_MASK_PART_NAMES: u32 = 0x38;
pub(crate) const BONE_MASK_PART_NUM_BONES: u32 = 0x44;
/// Size of one bone-name slot in a part's list.
pub(crate) const BONE_MASK_NAME_STRIDE: u32 = 0x50;
/// The "morph" token string (`str_morph`).
#[cfg(test)]
pub(crate) const BONE_MASK_STR_MORPH: u32 = 0x004a_3df0;
/// `strncmp`-like comparison (0x416b18).
const BONE_MASK_STRNCMP: u32 = 0x0041_6b18;

/// `u64 getGraftMask(cAnimModel* model, int part, const char* names)`
/// (0x310ce8): the bone/morph bit mask of a comma-separated list of names
/// for one part of a rider model. For each token it first checks the part's
/// morph slot (bit `morph + numBones`), else the part's bone names (bit
/// `firstBone + j` of the first bone whose full name equals the token).
/// 0 when the part index is out of range or has no table entry.
pub fn rider_model_get_bone_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = c.g[A1][0];
        let model_slot = frame.at(c, 0);
        c.sw(model_slot, c.g[A0][0]);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = oc417_lw(c, A0, BONE_MASK_MODEL_MAX_PART);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let out_of_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A2][0];
        'body: {
            let mut valid = !out_of_range;
            if valid {
                c.cycles += 4;
                c.g[V0][0] = oc417_lw(c, SP, 0);
                c.g[A1][0] = oc417_lw(c, V0, BONE_MASK_MODEL_PART_INDEX);
                let no_table = c.g[A1][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                if no_table {
                    valid = false;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A1][0]);
                    c.g[A1][0] = oc417_lw(c, V0, 0);
                    let found = (c.g[A1][0] as i64) >= 0;
                    c.g[S2][0] = c.g[A0][0];
                    valid = found;
                }
            }
            if !valid {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            // The part record and the mask accumulator.
            c.cycles += 9;
            c.g[V0][0] = 0x58;
            c.g[A0][0] = oc417_lw(c, SP, 0);
            c.mult(0, V0, A1, V0);
            c.g[S7][0] = 0;
            c.g[A2][0] = oc417_lbu(c, S2, 0);
            c.g[V1][0] = oc417_lw(c, A0, BONE_MASK_MODEL_PARTS);
            c.g[A0][0] = oc417_lb(c, S2, 0);
            c.g[S6][0] = oc417_add(c.g[V1][0], c.g[V0][0]);
            if c.g[A0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[S7][0];
                break 'body;
            }
            'tokens: loop {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                c.g[S3][0] = c.g[S2][0];
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[A0][0] = oc417_lw(c, S6, BONE_MASK_PART_MORPH);
                // Find the end of the token (NUL or ',').
                'scan: {
                    if c.g[V0][0] == 0 {
                        break 'scan;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = 0x2c;
                    if c.g[V0][0] == c.g[A1][0] {
                        break 'scan;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x2c;
                    c.g[S3][0] = oc417_add(c.g[S3][0], 1);
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = oc417_lb(c, S3, 0);
                        if c.g[V0][0] == 0 {
                            break 'scan;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[S3][0] = oc417_add(c.g[S3][0], 1);
                            if event_due(c) {
                                deliver_event_here(c, 0x0031_0da0);
                            }
                            continue;
                        }
                        break 'scan;
                    }
                }
                c.cycles += 2;
                let no_morph = (c.g[A0][0] as i64) < 0;
                c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                'token_done: {
                    if !no_morph {
                        // `strncmp(token, "morph", len) == 0`: the morph bit.
                        c.cycles += 7;
                        c.g[V0][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = oc417_add(c.g[V0][0], 0x3df0);
                        c.g[A2][0] = c.g[S5][0];
                        call_ptr(c, BONE_MASK_STRNCMP, 0x0031_0dd0)?;
                        if c.g[V0][0] == 0 {
                            c.cycles += 8;
                            c.g[A1][0] = oc417_lw(c, SP, 0);
                            c.g[A0][0] = 1;
                            c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_MORPH);
                            c.g[V1][0] = oc417_lw(c, A1, BONE_MASK_MODEL_NUM_BONES);
                            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                            c.g[A0][0] <<= c.g[V0][0] & 63;
                            c.g[S7][0] |= c.g[A0][0];
                            break 'token_done;
                        }
                        c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_NAMES);
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_NAMES);
                    }
                    // Search the part's bone names for the token.
                    c.cycles += 5;
                    c.g[S4][0] = 0;
                    c.g[FP][0] = 1;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S0][0] = oc417_add(c.g[V0][0], c.g[S5][0]);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_NUM_BONES);
                        c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        c.g[A0][0] = c.g[S1][0];
                        if c.g[V0][0] == 0 {
                            break 'token_done;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S5][0];
                        call_ptr(c, BONE_MASK_STRNCMP, 0x0031_0e34)?;
                        let next = if c.g[V0][0] != 0 {
                            c.g[S0][0] = oc417_add(c.g[S0][0], BONE_MASK_NAME_STRIDE as u64);
                            true
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = oc417_lb(c, S0, 0);
                            if c.g[V0][0] != 0 {
                                c.g[S0][0] = oc417_add(c.g[S0][0], BONE_MASK_NAME_STRIDE as u64);
                                true
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = oc417_lw(c, S6, BONE_MASK_PART_FIRST_BONE);
                                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[S4][0]);
                                c.g[V0][0] = c.g[FP][0] << (c.g[V0][0] & 63);
                                c.g[S7][0] |= c.g[V0][0];
                                false
                            }
                        };
                        if !next {
                            break 'token_done;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_0e10);
                        }
                        c.cycles += 2;
                        c.g[S1][0] = oc417_add(c.g[S1][0], BONE_MASK_NAME_STRIDE as u64);
                        c.g[S4][0] = oc417_add(c.g[S4][0], 1);
                    }
                }
                // Next token: skip the ',' (if that is what ended this one).
                c.cycles += 7;
                c.g[V0][0] = oc417_lb(c, S3, 0);
                c.g[V0][0] ^= 0x2c;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                c.g[S2][0] = oc417_add(c.g[S3][0], c.g[V0][0]);
                c.g[V1][0] = oc417_lb(c, S2, 0);
                let more = c.g[V1][0] != 0;
                c.g[A2][0] = oc417_lbu(c, S2, 0);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0d78);
                    }
                    continue 'tokens;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S7][0];
                break 'body;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_EventTypeMatches (0x303df8)

/// `int WScriptFn_EventTypeMatches(unsigned i)` (0x303df8): is the world
/// script's event type `i` (below 8) equal to the current event byte
/// (`*0x535c10`)? Looks the BE interface up first (result unused), then
/// compares `table[i]` (`0x4465f8`) with the byte. 0 for `i >= 8`.
pub fn wscript_fn_event_type_matches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < 8) as u64;
        let in_range = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if in_range {
            c.cycles += 14;
            call_ptr(c, 0x0014_dc80, 0x0030_3e20)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x0030_3e2c)?;
            c.g[V1][0] = 0x44_0000;
            c.g[V0][0] = 0x53_0000;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = oc417_add(c.g[V1][0], 0x65f8);
            c.g[A1][0] = oc417_lb(c, V0, 0x5c10);
            c.g[A0][0] = oc417_add(c.g[A0][0], c.g[V1][0]);
            c.g[V0][0] = oc417_lw(c, A0, 0);
            c.g[V0][0] ^= c.g[A1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CameraTrigger_ReadSphereTriggerData (0x172fe0)

/// The serializer `cCTVolumeEllipse` reads: the five fields' accessor offsets
/// in the trigger's descriptor (`+0x24`): (accessor offset, reader function,
/// return address of the accessor call, return address of the reader call).
const SPHERE_TRIGGER_FIELDS: [(u32, u32, u32, u32); 5] = [
    (0x08, 0x002d_25a0, 0x0017_3014, 0x0017_3020),
    (0x28, 0x002d_25a0, 0x0017_3038, 0x0017_3044),
    (0x10, 0x002d_2510, 0x0017_305c, 0x0017_3068),
    (0x18, 0x002d_2510, 0x0017_3080, 0x0017_308c),
    (0x20, 0x002d_2510, 0x0017_30a4, 0x0017_30b0),
];

/// `int get_cCTVolumeEllipse(reader, cCTVolumeEllK2* obj)` (0x172fe0): read
/// the camera-trigger ellipse volume from a serializer: clears `obj+0x28`,
/// then for each of its five fields (center, axis, size x/y/z) calls the
/// field's accessor through the descriptor at `obj+0x24` (vtable-style
/// `{delta, function}` pairs) and feeds the returned pointer to the reader
/// (a vector reader for the first two, a float reader for the others).
/// Returns the sum of the readers' results.
pub fn camera_trigger_read_sphere_trigger_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        oc417_sw0(c, S0, 0x28);
        for (i, &(field, reader, ret_accessor, ret_reader)) in
            SPHERE_TRIGGER_FIELDS.iter().enumerate()
        {
            c.g[V1][0] = oc417_lw(c, S0, 0x24);
            c.g[A0][0] = oc417_lh(c, V1, field);
            c.g[V0][0] = oc417_lw(c, V1, field + 4);
            let accessor = c.g[V0][0] as u32;
            c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, accessor, ret_accessor)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, reader, ret_reader)?;
            if i == 0 {
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.g[S1][0] = oc417_add(c.g[S1][0], c.g[V0][0]);
            }
        }
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimDataBase_AllocChanCache (0x310f18)

/// `operator new(size, name, flags, extra)` (0x317e30).
const CHAN_CACHE_NEW_NAMED: Callee = Callee {
    addr: 0x0031_7e30,
    f: ee::s10::operator_new_named,
};

/// `tCmpChanCache* gGenCacheBucket(int partID, int numChan)` (0x310f18):
/// build the channel cache of animation part `partID`: a ring of 20 nodes
/// (0x24 bytes each, one block named "AnimChanCache", stored at
/// `0x4fe7a0[partID]`), each node owning a separately allocated array of
/// `numChan` 20-byte elements (`+4`), `used` (`+0`) 0, and `next`/`prev`
/// (`+0x20`/`+0x1c`) pointing at its neighbours. The first node's `prev`
/// and the last node's `next` close the ring (`+0x2ac`, `+0x2cc`), and the
/// ring head also goes to `0x4fe860[partID]`.
pub fn anim_data_base_alloc_chan_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x50_0000;
        frame.save(c, S1, 0x70);
        c.g[A2][0] = 0x2000_0000;
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S7, 0x10);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save(c, S5, 0x30);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S6, 0x20);
        c.g[S7][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1860));
        c.g[S5][0] = c.g[S0][0];
        c.g[S6][0] = c.g[A1][0];
        c.g[S0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
        frame.save(c, S2, 0x60);
        c.g[A0][0] = 0x2d0;
        frame.save(c, S3, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62b0));
        frame.save(c, S4, 0x40);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[S0][0];
        CHAN_CACHE_NEW_NAMED.call(c, 0x0031_0f7c)?;
        c.g[S3][0] = 0x13;
        c.g[V1][0] = 0x14;
        oc417_sw(c, S0, 0, V0);
        c.mult(0, S4, S1, V1);
        c.g[S1][0] = 0;
        loop {
            c.cycles += 22;
            c.g[T0][0] = oc417_lw(c, S2, 0);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x62b0));
            c.g[A2][0] = 0x2000_0000;
            c.g[T0][0] = oc417_add(c.g[S1][0], c.g[T0][0]);
            c.g[A3][0] = 0;
            c.g[V0][0] = oc417_add(c.g[T0][0], 0x24);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            oc417_sw(c, T0, 0x20, V0);
            c.g[V1][0] = oc417_lw(c, S2, 0);
            c.g[V1][0] = oc417_add(c.g[S1][0], c.g[V1][0]);
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x24));
            oc417_sw(c, V1, 0x1c, T0);
            c.g[V0][0] = oc417_lw(c, S2, 0);
            c.g[V0][0] = oc417_add(c.g[S1][0], c.g[V0][0]);
            oc417_sw0(c, V0, 0);
            c.g[S0][0] = oc417_lw(c, S2, 0);
            c.g[S0][0] = oc417_add(c.g[S1][0], c.g[S0][0]);
            CHAN_CACHE_NEW_NAMED.call(c, 0x0031_0fdc)?;
            c.g[S1][0] = oc417_add(c.g[S1][0], 0x24);
            let more = (c.g[S3][0] as i64) >= 0;
            oc417_sw(c, S0, 4, V0);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_0f90);
                }
                continue;
            }
            break;
        }
        c.cycles += 23;
        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1860));
        c.g[A2][0] = 0x50_0000;
        c.g[A1][0] = oc417_add(c.g[S5][0], c.g[A1][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x17a0));
        c.g[A0][0] = oc417_lw(c, A1, 0);
        c.g[A2][0] = oc417_add(c.g[S5][0], c.g[A2][0]);
        frame.restore(c, S0, 0x80);
        c.g[V1][0] = oc417_add(c.g[A0][0], 0x2ac);
        frame.restore(c, S1, 0x70);
        oc417_sw(c, A0, 0x1c, V1);
        frame.restore(c, S2, 0x60);
        c.g[V0][0] = oc417_lw(c, A1, 0);
        frame.restore(c, S3, 0x50);
        oc417_sw(c, V0, 0x2cc, V0);
        frame.restore(c, S4, 0x40);
        c.g[V0][0] = oc417_lw(c, A1, 0);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        oc417_sw(c, A2, 0, V0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE_InitCharProfileBoltMap (0x151a88)

/// `Bolts_GetCharBolts(db, kind)` (0x14d998).
const BOLT_MAP_GET_CHAR_BOLTS: Callee = Callee {
    addr: 0x0014_d998,
    f: ee::s01::Bolts_GetCharBolts,
};
/// `Bolts_GetCharBoltCount(db, kind)` (0x14d988).
const BOLT_MAP_GET_CHAR_BOLT_COUNT: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};
/// `BE_ApplyDefaultCharProfileBolts(profile)` (0x1513b8).
const BOLT_MAP_APPLY_DEFAULT_BOLTS: Callee = Callee {
    addr: 0x0015_13b8,
    f: ee::s01::BE_ApplyDefaultCharProfileBolts,
};
/// The char-profile bolt table of a profile record: kind (byte at +0xbc0),
/// lookup (`short*` at +0x288), count (+0x28c), items (+0x290, 4 bytes: id,
/// flags); 525 item slots.
pub(crate) const BOLT_MAP_KIND: u32 = 0xbc0;
pub(crate) const BOLT_MAP_LOOKUP: u32 = 0x288;
pub(crate) const BOLT_MAP_COUNT: u32 = 0x28c;
pub(crate) const BOLT_MAP_ITEMS: u32 = 0x290;
pub(crate) const BOLT_MAP_SLOTS: i64 = 525;

/// `void BE_InitCharProfileBoltMap(profile, kind)` (0x151a88): fetch the
/// character's bolt list (`Bolts_GetCharBolts`/`Count` of the global bolt
/// database at `0x4a6750`), record `kind` and the count, allocate the
/// id -> item lookup (`gp-0x1f08` shorts, named "Profile_Bolts_Map") on
/// first use and fill it with -1, enter each bolt (flags 0, its id at
/// `+4` of the 0x38-byte record) as item i with `lookup[id] = i`, mark the
/// remaining slots up to 525 unused (id -1, flags 0), then apply the default
/// bolts.
pub fn be_init_char_profile_bolt_map(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x4a_0000;
        frame.save(c, S3, 0x10);
        c.g[S0][0] = oc417_add(c.g[S0][0], 0x6750);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BOLT_MAP_GET_CHAR_BOLTS.call(c, 0x0015_1ab8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        BOLT_MAP_GET_CHAR_BOLT_COUNT.call(c, 0x0015_1ac8)?;
        c.g[S0][0] = c.g[V0][0];
        let kind_addr = (c.g[S3][0] as u32).wrapping_add(BOLT_MAP_KIND);
        c.sb(kind_addr, c.g[S1][0]);
        c.g[V0][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
        let have_lookup = c.g[V0][0] != 0;
        oc417_sw(c, S3, BOLT_MAP_COUNT, S0);
        if !have_lookup {
            c.cycles += 8;
            c.g[A0][0] = oc417_lw(c, GP, 0u32.wrapping_sub(0x1f08));
            c.g[A1][0] = 0x46_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5858));
            c.g[A2][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0015_1af8)?;
            oc417_sw(c, S3, BOLT_MAP_LOOKUP, V0);
        }
        // lookup[0..gp-0x1f08] = -1.
        c.cycles += 3;
        c.g[V0][0] = oc417_lw(c, GP, 0u32.wrapping_sub(0x1f08));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[A1][0] = u64::MAX;
            c.g[V0][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
            loop {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 1);
                c.g[A0][0] = oc417_add(c.g[A0][0], 1);
                c.g[V1][0] = oc417_add(c.g[V1][0], c.g[V0][0]);
                let at = c.g[V1][0] as u32;
                c.sh(at, c.g[A1][0]);
                c.g[V0][0] = oc417_lw(c, GP, 0u32.wrapping_sub(0x1f08));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1b10);
                    }
                    continue;
                }
                break;
            }
        }
        // Items 0..count from the bolt list.
        c.cycles += 2;
        let no_bolts = (c.g[S0][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !no_bolts {
            c.cycles += 2;
            c.g[A1][0] = oc417_add(c.g[S3][0], BOLT_MAP_ITEMS as u64);
            loop {
                c.cycles += 13;
                oc417_sh0(c, A1, 2);
                c.g[V0][0] = oc417_lh(c, S2, 4);
                oc417_sh(c, A1, 0, V0);
                c.g[A1][0] = oc417_add(c.g[A1][0], 4);
                c.g[V0][0] = oc417_lh(c, S2, 4);
                c.g[V1][0] = oc417_lw(c, S3, BOLT_MAP_LOOKUP);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                oc417_sh(c, V0, 0, A0);
                c.g[A0][0] = oc417_add(c.g[A0][0], 1);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = oc417_add(c.g[S2][0], 0x38);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1b40);
                    }
                    continue;
                }
                break;
            }
        }
        // Unused slots count..525.
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < BOLT_MAP_SLOTS) as u64;
        let full = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        if !full {
            c.cycles += 3;
            c.g[A1][0] = u64::MAX;
            c.g[V0][0] = oc417_add(c.g[V0][0], BOLT_MAP_ITEMS as u64);
            c.g[V1][0] = oc417_add(c.g[V0][0], c.g[S3][0]);
            loop {
                c.cycles += 7;
                oc417_sh(c, V1, 0, A1);
                c.g[A0][0] = oc417_add(c.g[A0][0], 1);
                oc417_sh0(c, V1, 2);
                c.g[V0][0] = ((c.g[A0][0] as i64) < BOLT_MAP_SLOTS) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = oc417_add(c.g[V1][0], 4);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1b90);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S3][0];
        BOLT_MAP_APPLY_DEFAULT_BOLTS.call(c, 0x0015_1bb4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_ChannelStop (0x273dc0)

/// `InSeq_HeapDelete(obj)` (0x2724e8).
const CHANNEL_STOP_HEAP_DELETE: Callee = Callee {
    addr: 0x0027_24e8,
    f: ee::s07::InSeq_HeapDelete,
};
/// `InSeq_HeapFree(heap, block)` (0x2723a8).
const CHANNEL_STOP_HEAP_FREE: Callee = Callee {
    addr: 0x0027_23a8,
    f: ee::s07::InSeq_HeapFree,
};

/// Which way `InSeq_ChannelStop` disposes of one channel player.
#[derive(Clone, Copy)]
enum ChannelStopCase {
    /// Kind 0: `InSeq_HeapDelete(player)`.
    Delete,
    /// Kinds 1 and 2: `player->vfunc(3)` when the player is not null.
    ReleaseA,
    /// Kind 3: the same virtual call (its own copy of the code).
    ReleaseB,
    /// Any other kind: nothing.
    Skip,
}

/// `void cInSeqChannel::stop()` (0x273dc0): when playing (`+0x28`), tell the
/// entity comm object (`+0x2c`, a vtable call with the system header's byte
/// 3), clear it, then for each of the header's channel count (byte 2) stop
/// the channel player (vtable `+0x4`, entries `0x10/0x14`: stop; the kind
/// virtual at `0x38/0x3c`: 0 deletes the player, 1-3 call its release
/// virtual with 3), free the player array from the owner's heap and clear
/// the playing flag.
pub fn in_seq_channel_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = oc417_lw(c, S1, 0x28);
        let idle = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        'stopped: {
            if idle {
                break 'stopped;
            }
            c.cycles += 3;
            c.g[T0][0] = oc417_lw(c, S1, 0x2c);
            let no_comm = c.g[T0][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !no_comm {
                c.cycles += 7;
                c.g[V0][0] = oc417_lw(c, T0, 0);
                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                c.g[A0][0] = oc417_lh(c, V0, 0x10);
                c.g[A3][0] = oc417_lw(c, V0, 0x14);
                c.g[A2][0] = oc417_lbu(c, V1, 3);
                let target = c.g[A3][0] as u32;
                c.g[A0][0] = oc417_add(c.g[T0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0027_3e0c)?;
            }
            c.cycles += 5;
            oc417_sw0(c, S1, 0x2c);
            c.g[V1][0] = oc417_lw(c, S1, 0x1c);
            c.g[V0][0] = oc417_lbu(c, V1, 2);
            if c.g[V0][0] == 0 {
                oc417_sw0(c, S1, 0x28);
            } else {
                c.cycles += 2;
                c.g[S2][0] = 0;
                c.cycles += 2;
                c.g[V0][0] = oc417_lw(c, S1, 0x24);
                loop {
                    // Stop the player.
                    c.cycles += 20;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                    c.g[A2][0] = oc417_lw(c, V0, 0);
                    c.g[V1][0] = oc417_lw(c, A2, 4);
                    c.g[A0][0] = oc417_lh(c, V1, 0x10);
                    c.g[A1][0] = oc417_lw(c, V1, 0x14);
                    let target = c.g[A1][0] as u32;
                    c.g[A0][0] = oc417_add(c.g[A2][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0027_3e50)?;
                    // Ask its kind.
                    c.g[V0][0] = oc417_lw(c, S1, 0x24);
                    c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                    c.g[A1][0] = oc417_lw(c, V0, 0);
                    c.g[V1][0] = oc417_lw(c, A1, 4);
                    c.g[A0][0] = oc417_lh(c, V1, 0x38);
                    c.g[V0][0] = oc417_lw(c, V1, 0x3c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = oc417_add(c.g[A1][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0027_3e70)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = 1;
                    let is_one = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                    let case = if is_one {
                        ChannelStopCase::ReleaseA
                    } else {
                        c.cycles += 2;
                        let at_least_two = c.g[V0][0] == 0;
                        c.g[V0][0] = 2;
                        if !at_least_two {
                            c.cycles += 2;
                            if c.g[V1][0] == 0 {
                                c.g[V0][0] = oc417_lw(c, S1, 0x24);
                                ChannelStopCase::Delete
                            } else {
                                c.cycles += 2;
                                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                                ChannelStopCase::Skip
                            }
                        } else {
                            c.cycles += 2;
                            let is_two = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 3;
                            if is_two {
                                ChannelStopCase::ReleaseA
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = oc417_lw(c, S1, 0x24);
                                    ChannelStopCase::ReleaseB
                                } else {
                                    c.cycles += 2;
                                    c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                                    ChannelStopCase::Skip
                                }
                            }
                        }
                    };
                    match case {
                        ChannelStopCase::Delete => {
                            c.cycles += 5;
                            c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                            c.g[A0][0] = oc417_lw(c, V0, 0);
                            CHANNEL_STOP_HEAP_DELETE.call(c, 0x0027_3ebc)?;
                            c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                        }
                        ChannelStopCase::ReleaseA => {
                            c.cycles += 5;
                            c.g[V0][0] = oc417_lw(c, S1, 0x24);
                            c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                            c.g[A2][0] = oc417_lw(c, V0, 0);
                            let null = c.g[A2][0] == 0;
                            c.g[A1][0] = 3;
                            if null {
                                c.cycles += 1;
                                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = oc417_lw(c, A2, 4);
                                c.g[A0][0] = oc417_lh(c, V0, 0x50);
                                c.g[V1][0] = oc417_lw(c, V0, 0x54);
                                let target = c.g[V1][0] as u32;
                                c.g[A0][0] = oc417_add(c.g[A2][0], c.g[A0][0]);
                                call_ptr(c, target, 0x0027_3eec)?;
                                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                            }
                        }
                        ChannelStopCase::ReleaseB => {
                            c.cycles += 4;
                            c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                            c.g[A2][0] = oc417_lw(c, V0, 0);
                            let null = c.g[A2][0] == 0;
                            c.g[A1][0] = 3;
                            if !null {
                                c.cycles += 5;
                                c.g[V0][0] = oc417_lw(c, A2, 4);
                                c.g[A0][0] = oc417_lh(c, V0, 0x50);
                                c.g[V1][0] = oc417_lw(c, V0, 0x54);
                                let target = c.g[V1][0] as u32;
                                c.g[A0][0] = oc417_add(c.g[A2][0], c.g[A0][0]);
                                call_ptr(c, target, 0x0027_3f18)?;
                            }
                            c.cycles += 1;
                            c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                        }
                        ChannelStopCase::Skip => {}
                    }
                    // Next player.
                    c.cycles += 5;
                    c.g[S2][0] = oc417_add(c.g[S2][0], 1);
                    c.g[V0][0] = oc417_lbu(c, V1, 2);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = oc417_lw(c, S1, 0x24);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_3e30);
                        }
                        continue;
                    }
                    break;
                }
                // Free the player array.
                c.cycles += 7;
                c.g[V0][0] = oc417_lw(c, S1, 8);
                c.g[A1][0] = oc417_lw(c, S1, 0x24);
                c.g[A0][0] = oc417_lw(c, V0, 8);
                c.g[A0][0] = oc417_add(c.g[A0][0], 8);
                CHANNEL_STOP_HEAP_FREE.call(c, 0x0027_3f44)?;
                oc417_sw0(c, S1, 0x24);
                oc417_sw0(c, S1, 0x28);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_MultiParticleControl (0x306cb0) and WScriptFn_AddMagnetModifier
// (0x305478): the same world-script argument decoding.

/// The `MEM_copy` (0x3e6574) both functions use to copy the default record.
const WSCRIPT_MEM_COPY: u32 = 0x003e_6574;

/// `WScriptFn_MultiParticleControl(Rec* result, int n, Ent* entries)`
/// (0x306cb0): on first use initialise the static default record
/// (`0x4fc050`: object id -1 ...), copy it to a 16-byte stack record, apply
/// the `n` 16-byte `{index, value, type, ..}` argument entries to the
/// record's words (an int argument for a float parameter is converted with
/// `cvt.s.w`), resolve the object id (`rec[1]`, -1 = the current object at
/// `world+0x290`, else the world cache's slot table) and, when found, call
/// `MultiParticle` add (`rec[2] == 0`, `0x357ae8`) or remove (`0x357b10`)
/// with the world's particle manager (`gp-0xf38 slot`, `rec[0]`) and the
/// object's `+0x78`. Copies the 12-byte default result (`0x4c9098`) to
/// `*result` and returns `result`.
pub fn wscript_fn_multi_particle_control(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = oc417_lw(c, GP, 0x27c8);
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
            c.cycles += 11;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fb0));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = oc417_lbu(c, V1, 0xc);
            c.g[V0][0] |= 0xffff;
            Addr((c.g[A2][0] as u32).wrapping_sub(0x3fb0)).sw(c, 0);
            c.g[A0][0] = 1;
            let a = (c.g[V1][0] as u32).wrapping_add(0xc);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
            oc417_sw(c, V1, 4, V0);
            oc417_sw(c, GP, 0x27c8, A0);
            oc417_sw0(c, V1, 8);
            let a = sp(c);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
        }
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = oc417_add(c.g[SP][0], 0x10);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fb0));
        c.g[A2][0] = 0x10;
        oc417_sw(c, SP, 0x14, V0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WSCRIPT_MEM_COPY, 0x0030_6d2c)?;
        let no_entries = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_entries {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = oc417_add(c.g[V0][0], 0x6970);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = oc417_lw(c, A2, 0);
                c.g[V1][0] = oc417_lw(c, A2, 0xc);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc417_add(c.g[V0][0], c.g[S0][0]);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[T0][0]);
                c.g[V0][0] = oc417_lw(c, V0, 0);
                // `entry.type != paramType && paramType == 2`: int -> float.
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = oc417_lw(c, A2, 4);
                    false
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = oc417_lw(c, A2, 4);
                        false
                    } else {
                        c.cycles += 4;
                        let a = (c.g[A2][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.cvt_s_w(0, 0);
                        let a = c.g[A0][0] as u32;
                        c.swc1(0, a);
                        true
                    }
                };
                if !convert {
                    c.cycles += 1;
                    oc417_sw(c, A0, 0, V0);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = oc417_add(c.g[A2][0], 0x10);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_6d48);
                    }
                    continue;
                }
                break;
            }
        }
        // Resolve the object (rec[1]).
        c.cycles += 4;
        c.g[A2][0] = oc417_lw(c, S0, 4);
        c.g[V0][0] = !(c.g[A2][0]);
        let looked_up = c.g[V0][0] != 0;
        c.g[A1][0] = oc417_lw(c, GP, 0x16c8);
        if !looked_up {
            c.cycles += 3;
            c.g[V0][0] = oc417_lw(c, GP, 0xce8);
            c.g[V1][0] = oc417_lw(c, V0, 0x290);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = oc417_lw(c, A1, 0);
            c.g[A0][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = oc417_lw(c, V0, 0);
            let no_slot = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_slot {
                c.cycles += 7;
                c.g[A0][0] = oc417_lw(c, V1, 0x1c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = oc417_lw(c, V0, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.g[V1][0] = 0x4d_0000;
        } else {
            let remove_instead = {
                c.cycles += 3;
                c.g[V0][0] = oc417_lw(c, S0, 8);
                let r = c.g[V0][0] != 0;
                c.g[A1][0] = oc417_lw(c, S0, 0);
                r
            };
            if !remove_instead {
                c.cycles += 5;
                c.g[A2][0] = oc417_lw(c, V1, 0x78);
                c.g[A0][0] = oc417_lw(c, GP, 0xf38);
                call_ptr(c, 0x0035_7ae8, 0x0030_6e10)?;
                c.g[V1][0] = 0x4d_0000;
            } else {
                c.cycles += 4;
                c.g[A2][0] = oc417_lw(c, V1, 0x78);
                c.g[A0][0] = oc417_lw(c, GP, 0xf38);
                call_ptr(c, 0x0035_7b10, 0x0030_6e24)?;
                c.g[V1][0] = 0x4d_0000;
            }
        }
        // Copy the default result record to *result.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let a = (c.g[A3][0] as u32).wrapping_add(7);
        c.ldl(4, a);
        let a = c.g[A3][0] as u32;
        c.ldr(4, a);
        c.g[A1][0] = oc417_lw(c, A3, 8);
        let a = (c.g[S3][0] as u32).wrapping_add(7);
        c.sdl(4, a);
        let a = c.g[S3][0] as u32;
        c.sdr(4, a);
        oc417_sw(c, S3, 8, A1);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptFn_AddMagnetModifier(Rec* result, int n, Ent* entries)`
/// (0x305478): like `WScriptFn_MultiParticleControl` but with the static
/// default record at `0x4fbd58` (object id -1, strength 400.0, radius 30.0)
/// and the parameter types at `0x4466a0`. When the resolved object has a
/// move node (`obj+0xc`) whose virtual `0x80` says it accepts magnets, adds
/// a magnet to it (`0x3559f8`: node, the node's virtual `0xc0` result,
/// strength, radius). Returns `result` with the default result record
/// (`0x4c9098`) copied in.
pub fn wscript_fn_add_magnet_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = oc417_lw(c, GP, 0x2750);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42a8));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = oc417_lbu(c, V1, 0xc);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x43c8_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41f0_0000;
            c.mtc1(1, 0);
            let a = (c.g[A2][0] as u32).wrapping_sub(0x42a8);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
            let a = (c.g[V1][0] as u32).wrapping_add(0xc);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
            let a = (c.g[V1][0] as u32).wrapping_add(4);
            c.swc1(1, a);
            let a = (c.g[V1][0] as u32).wrapping_add(8);
            c.swc1(0, a);
            oc417_sw(c, GP, 0x2750, A0);
            let a = sp(c);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
        }
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = oc417_add(c.g[SP][0], 0x10);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42a8));
        c.g[A2][0] = 0x10;
        oc417_sw(c, SP, 0x10, V0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, WSCRIPT_MEM_COPY, 0x0030_5504)?;
        let no_entries = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_entries {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = oc417_add(c.g[V0][0], 0x66a0);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = oc417_lw(c, A2, 0);
                c.g[V1][0] = oc417_lw(c, A2, 0xc);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc417_add(c.g[V0][0], c.g[S1][0]);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[T0][0]);
                c.g[V0][0] = oc417_lw(c, V0, 0);
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = oc417_lw(c, A2, 4);
                    false
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = oc417_lw(c, A2, 4);
                        false
                    } else {
                        c.cycles += 4;
                        let a = (c.g[A2][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.cvt_s_w(0, 0);
                        let a = c.g[A0][0] as u32;
                        c.swc1(0, a);
                        true
                    }
                };
                if !convert {
                    c.cycles += 1;
                    oc417_sw(c, A0, 0, V0);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = oc417_add(c.g[A2][0], 0x10);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_5520);
                    }
                    continue;
                }
                break;
            }
        }
        // Resolve the object (rec[0]).
        c.cycles += 4;
        c.g[A2][0] = oc417_lw(c, S1, 0);
        c.g[V0][0] = !(c.g[A2][0]);
        let looked_up = c.g[V0][0] != 0;
        c.g[A1][0] = oc417_lw(c, GP, 0x16c8);
        if !looked_up {
            c.cycles += 3;
            c.g[V0][0] = oc417_lw(c, GP, 0xce8);
            c.g[V0][0] = oc417_lw(c, V0, 0x290);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = oc417_lw(c, A1, 0);
            c.g[A0][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = oc417_lw(c, V0, 0);
            let no_slot = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_slot {
                c.cycles += 7;
                c.g[A0][0] = oc417_lw(c, V1, 0x1c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = oc417_lw(c, V0, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // 'b_305624: the move node and the magnet.
        'magnet: {
            'result_v0: {
                c.cycles += 2;
                let no_object = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4d_0000;
                if no_object {
                    break 'result_v0;
                }
                c.cycles += 3;
                c.g[S0][0] = oc417_lw(c, V0, 0xc);
                let no_node = c.g[S0][0] == 0;
                c.g[V0][0] = c.g[S3][0];
                if no_node {
                    break 'magnet;
                }
                c.cycles += 7;
                c.g[V1][0] = oc417_lw(c, S0, 0xc);
                c.g[A0][0] = oc417_lh(c, V1, 0x80);
                c.g[V0][0] = oc417_lw(c, V1, 0x84);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0030_55ec)?;
                let rejects = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4d_0000;
                if rejects {
                    break 'result_v0;
                }
                c.cycles += 11;
                c.g[V1][0] = oc417_lw(c, S0, 0xc);
                c.g[A0][0] = oc417_lh(c, V1, 0xc0);
                c.g[V0][0] = oc417_lw(c, V1, 0xc4);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0030_5608)?;
                let a = (c.g[S1][0] as u32).wrapping_add(8);
                c.lwc1(13, a);
                c.g[A0][0] = c.g[S0][0];
                let a = (c.g[S1][0] as u32).wrapping_add(4);
                c.lwc1(12, a);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0035_59f8, 0x0030_561c)?;
                c.g[V1][0] = 0x4d_0000;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let a = (c.g[A3][0] as u32).wrapping_add(7);
        c.ldl(4, a);
        let a = c.g[A3][0] as u32;
        c.ldr(4, a);
        c.g[A1][0] = oc417_lw(c, A3, 8);
        let a = (c.g[S3][0] as u32).wrapping_add(7);
        c.sdl(4, a);
        let a = c.g[S3][0] as u32;
        c.sdr(4, a);
        oc417_sw(c, S3, 8, A1);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SectionMan_RequestUnload (0x22d598)

/// `SectionMan_QueueSection(section)` (0x22e298).
const REQUEST_UNLOAD_QUEUE_SECTION: Callee = Callee {
    addr: 0x0022_e298,
    f: ee::s05::SectionMan_QueueSection,
};
/// The section map (`0x442168`, 16-byte entries: +4 slot, +8 state).
#[cfg(test)]
pub(crate) const REQUEST_UNLOAD_SECTION_MAP: u32 = 0x0044_2168;

/// `void cSectionMan::requestUnload(int section)` (0x22d598): when the
/// section's slot (`+4`) is not negative, state 3 (loaded) becomes 0
/// (unloaded) and states 1, 2 and 6 become 5 (unloading); any other state is
/// left alone. A section whose state changed is queued
/// (`SectionMan_QueueSection`). The state switch is a jump table at
/// `0x484c00`, one of three code labels per state.
pub fn section_man_request_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0x44_0000;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[V0][0];
        c.g[A3][0] = oc417_add(c.g[V0][0], 0x2168);
        c.g[A2][0] = sx((c.g[A1][0] as u32) << 4);
        c.g[V0][0] = oc417_add(c.g[A3][0], c.g[A2][0]);
        c.g[V1][0] = oc417_lw(c, V0, 4);
        let not_loaded = (c.g[V1][0] as i64) < 0;
        c.g[T1][0] = c.g[A0][0];
        if not_loaded {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.g[V0][0];
            c.g[V1][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = (c.g[V1][0] < 9) as u64;
            let state_out_of_range = c.g[V0][0] == 0;
            c.g[A2][0] = 0;
            if !state_out_of_range {
                c.cycles += 7;
                c.g[V0][0] = 0x48_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c00));
                c.g[V1][0] = oc417_add(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = oc417_lw(c, V1, 0);
                let label = c.g[A0][0] as u32;
                match label {
                    // State 3: back to 0.
                    0x0022_d5f0 => {
                        c.cycles += 6;
                        c.g[V0][0] = oc417_add(c.g[T0][0], 0x2168);
                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                        c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                        c.g[A2][0] = 1;
                        oc417_sw0(c, V0, 8);
                    }
                    // States 1, 2, 6: to 5.
                    0x0022_d608 => {
                        c.cycles += 6;
                        c.g[V0][0] = oc417_add(c.g[T0][0], 0x2168);
                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
                        c.g[V0][0] = oc417_add(c.g[V0][0], c.g[V1][0]);
                        c.g[A0][0] = 5;
                        oc417_sw(c, V0, 8, A0);
                        c.g[A2][0] = 1;
                    }
                    // Other states: unchanged.
                    0x0022_d620 => {}
                    _ => {
                        c.jump_out(label, 0x0022_d598, 0x0022_d63c);
                        return Ok(());
                    }
                }
            }
            c.cycles += 2;
            let unchanged = c.g[A2][0] == 0;
            frame.restore_ra(c, 0);
            if !unchanged {
                c.cycles += 2;
                c.g[A0][0] = c.g[T1][0];
                REQUEST_UNLOAD_QUEUE_SECTION.call(c, 0x0022_d630)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptEngine_GetTargetPose (0x27a0d8)

/// `void ScriptEngine_GetTargetPose(self, int id, obj, float* pos, int* flag,
/// float* pitch, float* yaw)` (0x27a0d8): the position and orientation a
/// script camera/object aims at, by target id. Clears `flag`, `pitch`,
/// `yaw` and sets `pos = (0, 0, 0, 1)`, then (jump table at `0x481d00`,
/// ids 1-60):
/// - 40-60: the rider `id - 40` (`ScriptEngine_GetRiderPose`);
/// - 1-18: the start point of rider `id + 2` (`GetRiderPathPose`);
/// - 19-39 and 20-22: a locator by kind (`ScriptEngine_GetLocatorPose`, kind
///   0-11, category 50 or 43 for the `ScriptData_GetInfo` objects), most of
///   them dropped onto the ground (`pos.z` from the BXNash collision probe
///   `0x3369d8`), 25 first lowering `pos.z` by 1000, 20-22 using the
///   offsets of the static table at `0x4d3680` (built on first use).
/// Written register for register (the jump table and its shared tails keep
/// the generated block structure).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_parens
)]
pub fn script_engine_get_target_pose(c: &mut Cpu) {
    'b_27a480: {
        'b_27a47c: {
            c.cycles += 27;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S3][0] = c.g[T0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S2][0] = c.g[A3][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S4][0] = c.g[T1][0];
            c.sw((c.g[SP][0] as u32), 0u64);
            c.g[S5][0] = c.g[T2][0];
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = c.g[A0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V1][0] = (c.g[T0][0] < 0x3cu64) as u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sw((c.g[S3][0] as u32), 0u64);
            c.sw((c.g[S4][0] as u32), 0u64);
            c.lq(2, (c.g[SP][0] as u32));
            c.sw((c.g[S5][0] as u32), 0u64);
            let t = c.g[V1][0] == 0u64;
            c.sq(2, (c.g[S2][0] as u32));
            if t {
                break 'b_27a47c;
            } else {
                'b_27a458: {
                    'b_27a440: {
                        'b_27a410: {
                            'b_27a404: {
                                'b_27a348: {
                                    'b_27a338: {
                                        'b_27a328: {
                                            'b_27a324: {
                                                'b_27a30c: {
                                                    'b_27a2fc: {
                                                        'b_27a2e0: {
                                                            'b_27a2c4: {
                                                                'b_27a2b4: {
                                                                    'b_27a280: {
                                                                        'b_27a24c: {
                                                                            'b_27a23c: {
                                                                                'b_27a22c: {
                                                                                    'b_27a21c: {
                                                                                        'b_27a1c8: {
                                                                                            'b_27a194: {
                                                                                                c.cycles += 7;
                                                                                                c.g[V0][0] = 0x480000;
                                                                                                c.g[V1][0] = sx((c.g[T0][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d00));
                                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                                                let t = c.g[A0][0] as u32;
                                                                                                match t {
0x0027a160 => {
c.cycles += 5;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x28));
c.g[RA][0] = sx(0x0027a16c);
c.g[A0][0] = c.g[S1][0];
c.pc = 0x00279f18; ee::s07::ScriptEngine_GetTargetObject(c);
if c.pc != 0x0027a16c { return; }
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S1][0];
if t {
break 'b_27a47c;
} else {
c.cycles += 8;
c.g[A1][0] = c.g[V0][0];
c.g[A2][0] = c.g[S2][0];
c.g[A3][0] = c.g[S3][0];
c.g[T0][0] = c.g[S4][0];
c.g[RA][0] = sx(0x0027a18c);
c.g[T1][0] = c.g[S5][0];
c.pc = 0x0027b948; ee::s07::ScriptEngine_GetRiderPose(c);
if c.pc != 0x0027a18c { return; }
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_27a480;
} else {
break 'b_27a194;
}
}
}
0x0027a194 => {
break 'b_27a194;
}
0x0027a1c8 => {
break 'b_27a1c8;
}
0x0027a21c => {
break 'b_27a21c;
}
0x0027a22c => {
break 'b_27a22c;
}
0x0027a23c => {
break 'b_27a23c;
}
0x0027a24c => {
break 'b_27a24c;
}
0x0027a280 => {
break 'b_27a280;
}
0x0027a2b4 => {
break 'b_27a2b4;
}
0x0027a2c4 => {
break 'b_27a2c4;
}
0x0027a2e0 => {
break 'b_27a2e0;
}
0x0027a2fc => {
break 'b_27a2fc;
}
0x0027a30c => {
break 'b_27a30c;
}
0x0027a338 => {
break 'b_27a338;
}
0x0027a348 => {
break 'b_27a348;
}
0x0027a404 => {
break 'b_27a404;
}
0x0027a458 => {
break 'b_27a458;
}
_ => { c.jump_out(t, 0x0027a0d8, 0x0027a4a0); return; }
}
                                                                                            }
                                                                                            c.cycles += 5;
                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                                                                                            c.g[RA][0] = sx(0x0027a1a0);
                                                                                            c.g[A0][0] = c.g[S1][0];
                                                                                            c.pc = 0x00279f18;
                                                                                            ee::s07::ScriptEngine_GetTargetObject(c);
                                                                                            if c.pc != 0x0027a1a0 { return; }
                                                                                            let t = c.g[V0][0] == 0u64;
                                                                                            c.g[A0][0] = c.g[S1][0];
                                                                                            if t {
                                                                                                break 'b_27a47c;
                                                                                            } else {
                                                                                                c.cycles += 8;
                                                                                                c.g[A1][0] = c.g[V0][0];
                                                                                                c.g[A2][0] = c.g[S2][0];
                                                                                                c.g[A3][0] = c.g[S3][0];
                                                                                                c.g[T0][0] = c.g[S4][0];
                                                                                                c.g[RA][0] = sx(0x0027a1c0);
                                                                                                c.g[T1][0] = c.g[S5][0];
                                                                                                c.pc = 0x0027b750;
                                                                                                ee::s07::ScriptEngine_GetRiderPathPose(c);
                                                                                                if c.pc != 0x0027a1c0 { return; }
                                                                                                let t = true;
                                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_27a480;
                                                                                                } else {
                                                                                                    break 'b_27a1c8;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            21;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S1]
                                                                                            [0];
                                                                                        c.g[A3]
                                                                                            [0] = c
                                                                                            .g[S3]
                                                                                            [0];
                                                                                        c.g[A1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        c.g[A2]
                                                                                            [0] = c
                                                                                            .g[S2]
                                                                                            [0];
                                                                                        c.g[T0]
                                                                                            [0] = c
                                                                                            .g[S4]
                                                                                            [0];
                                                                                        c.g[T1]
                                                                                            [0] = c
                                                                                            .g[S5]
                                                                                            [0];
                                                                                        c.g[T2]
                                                                                            [0] =
                                                                                            0x32;
                                                                                        c.g[RA][0] = sx(0x0027a1ec);
                                                                                        c.g[T3]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        c.pc = 0x0027bb08;
                                                                                        ee::s07::ScriptEngine_GetLocatorPose(c);
                                                                                        if c.pc != 0x0027a1ec { return; }
                                                                                        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                                                                                        c.g[A1]
                                                                                            [0] = c
                                                                                            .g[S2]
                                                                                            [0];
                                                                                        c.g[AT][0] = 0x447a0000;
                                                                                        c.mtc1(
                                                                                            1, 1,
                                                                                        );
                                                                                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                        c.g[A3]
                                                                                            [0] = c
                                                                                            .g[SP]
                                                                                            [0];
                                                                                        c.sub_s(
                                                                                            0, 0, 1,
                                                                                        );
                                                                                        c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                                                                                        if t {
                                                                                            break 'b_27a440;
                                                                                        } else {
                                                                                            break 'b_27a21c;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 4;
                                                                                    c.g[A0][0] =
                                                                                        c.g[S1][0];
                                                                                    c.g[A3][0] =
                                                                                        c.g[S3][0];
                                                                                    let t = true;
                                                                                    c.g[A1][0] =
                                                                                        0x2;
                                                                                    if t {
                                                                                        break 'b_27a410;
                                                                                    } else {
                                                                                        break 'b_27a22c;
                                                                                    }
                                                                                }
                                                                                c.cycles += 4;
                                                                                c.g[A0][0] =
                                                                                    c.g[S1][0];
                                                                                c.g[A3][0] =
                                                                                    c.g[S3][0];
                                                                                let t = true;
                                                                                c.g[A1][0] = 0x3;
                                                                                if t {
                                                                                    break 'b_27a410;
                                                                                } else {
                                                                                    break 'b_27a23c;
                                                                                }
                                                                            }
                                                                            c.cycles += 4;
                                                                            c.g[A0][0] = c.g[S1][0];
                                                                            c.g[A3][0] = c.g[S3][0];
                                                                            let t = true;
                                                                            c.g[A1][0] = 0x4;
                                                                            if t {
                                                                                break 'b_27a410;
                                                                            } else {
                                                                                break 'b_27a24c;
                                                                            }
                                                                        }
                                                                        c.cycles += 13;
                                                                        c.g[A0][0] = c.g[A2][0];
                                                                        c.g[RA][0] = sx(0x0027a258);
                                                                        c.g[S0][0] = 0x32;
                                                                        c.pc = 0x0027c070;
                                                                        ee::s07::ScriptData_GetInfo(
                                                                            c,
                                                                        );
                                                                        if c.pc != 0x0027a258 {
                                                                            return;
                                                                        }
                                                                        c.g[V1][0] =
                                                                            c.lb(
                                                                                (c.g[V0][0] as u32)
                                                                            );
                                                                        c.g[T2][0] = 0x2b;
                                                                        c.g[A0][0] = c.g[S1][0];
                                                                        c.g[A2][0] = c.g[S2][0];
                                                                        if c.g[V1][0] == 0 {
                                                                            c.g[T2][0] = c.g[S0][0];
                                                                        }
                                                                        c.g[A3][0] = c.g[S3][0];
                                                                        c.g[T0][0] = c.g[S4][0];
                                                                        c.g[T1][0] = c.g[S5][0];
                                                                        let t = true;
                                                                        c.g[A1][0] = 0x5;
                                                                        if t {
                                                                            break 'b_27a328;
                                                                        } else {
                                                                            break 'b_27a280;
                                                                        }
                                                                    }
                                                                    c.cycles += 13;
                                                                    c.g[A0][0] = c.g[A2][0];
                                                                    c.g[RA][0] = sx(0x0027a28c);
                                                                    c.g[S0][0] = 0x32;
                                                                    c.pc = 0x0027c070;
                                                                    ee::s07::ScriptData_GetInfo(c);
                                                                    if c.pc != 0x0027a28c {
                                                                        return;
                                                                    }
                                                                    c.g[V1][0] =
                                                                        c.lb((c.g[V0][0] as u32));
                                                                    c.g[T2][0] = 0x2b;
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[A2][0] = c.g[S2][0];
                                                                    if c.g[V1][0] == 0 {
                                                                        c.g[T2][0] = c.g[S0][0];
                                                                    }
                                                                    c.g[A3][0] = c.g[S3][0];
                                                                    c.g[T0][0] = c.g[S4][0];
                                                                    c.g[T1][0] = c.g[S5][0];
                                                                    let t = true;
                                                                    c.g[A1][0] = 0x6;
                                                                    if t {
                                                                        break 'b_27a328;
                                                                    } else {
                                                                        break 'b_27a2b4;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[A3][0] = c.g[S3][0];
                                                                let t = true;
                                                                c.g[A1][0] = 0x9;
                                                                if t {
                                                                    break 'b_27a410;
                                                                } else {
                                                                    break 'b_27a2c4;
                                                                }
                                                            }
                                                            c.cycles += 7;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            c.g[A2][0] = c.g[S2][0];
                                                            c.g[A3][0] = c.g[S3][0];
                                                            c.g[T0][0] = c.g[S4][0];
                                                            c.g[T1][0] = c.g[S5][0];
                                                            let t = true;
                                                            c.g[A1][0] = 0xa;
                                                            if t {
                                                                break 'b_27a324;
                                                            } else {
                                                                break 'b_27a2e0;
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[A2][0] = c.g[S2][0];
                                                        c.g[A3][0] = c.g[S3][0];
                                                        c.g[T0][0] = c.g[S4][0];
                                                        c.g[T1][0] = c.g[S5][0];
                                                        let t = true;
                                                        c.g[A1][0] = 0xb;
                                                        if t {
                                                            break 'b_27a324;
                                                        } else {
                                                            break 'b_27a2fc;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_sub(0x16));
                                                    let t = true;
                                                    c.g[A3][0] = c.g[S3][0];
                                                    if t {
                                                        break 'b_27a410;
                                                    } else {
                                                        break 'b_27a30c;
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x16));
                                                c.g[A2][0] = c.g[S2][0];
                                                c.g[A3][0] = c.g[S3][0];
                                                c.g[T0][0] = c.g[S4][0];
                                                c.g[T1][0] = c.g[S5][0];
                                                break 'b_27a324;
                                            }
                                            c.cycles += 1;
                                            c.g[T2][0] = 0x32;
                                            break 'b_27a328;
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0027a330);
                                        c.g[T3][0] = 0u64;
                                        c.pc = 0x0027bb08;
                                        ee::s07::ScriptEngine_GetLocatorPose(c);
                                        if c.pc != 0x0027a330 {
                                            return;
                                        }
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        if t {
                                            break 'b_27a480;
                                        } else {
                                            break 'b_27a338;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A3][0] = c.g[S3][0];
                                    let t = true;
                                    c.g[A1][0] = 0x7;
                                    if t {
                                        break 'b_27a410;
                                    } else {
                                        break 'b_27a348;
                                    }
                                }
                                'b_27a3ac: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x20e8));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0xc;
                                    if t {
                                        break 'b_27a3ac;
                                    } else {
                                        c.cycles += 22;
                                        c.g[V1][0] = 0x4d0000;
                                        c.g[AT][0] = 0x438c0000;
                                        c.mtc1(1, 4);
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3770));
                                        c.g[AT][0] = 0xffffffffc1a00000;
                                        c.mtc1(1, 0);
                                        c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.g[A0][0] = 0x1;
                                        c.g[AT][0] = 0x43aa0000;
                                        c.mtc1(1, 2);
                                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x47f4));
                                        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x47f0));
                                        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x3770));
                                        c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x18));
                                        c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        c.sw((c.g[GP][0] as u32).wrapping_add(0x20e8), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                                        c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        c.g[V0][0] = 0xc;
                                        break 'b_27a3ac;
                                    }
                                }
                                c.cycles += 22;
                                c.g[V1][0] = 0x4d0000;
                                c.mult(0, 2, 5, 2);
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3680));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[S2][0];
                                c.g[A3][0] = c.g[S3][0];
                                c.g[T0][0] = c.g[S4][0];
                                c.g[T1][0] = c.g[S5][0];
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[A1][0] = 0u64;
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[T2][0] = 0x32;
                                c.lwc1(1, (c.g[V0][0] as u32));
                                c.g[T3][0] = c.g[SP][0];
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                c.swc1(1, (c.g[SP][0] as u32));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[RA][0] = sx(0x0027a3fc);
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.pc = 0x0027bb08;
                                ee::s07::ScriptEngine_GetLocatorPose(c);
                                if c.pc != 0x0027a3fc {
                                    return;
                                }
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                if t {
                                    break 'b_27a480;
                                } else {
                                    break 'b_27a404;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S3][0];
                            c.g[A1][0] = 0u64;
                            break 'b_27a410;
                        }
                        c.cycles += 12;
                        c.g[A2][0] = c.g[S2][0];
                        c.g[T0][0] = c.g[S4][0];
                        c.g[T1][0] = c.g[S5][0];
                        c.g[T2][0] = 0x32;
                        c.g[RA][0] = sx(0x0027a428);
                        c.g[T3][0] = 0u64;
                        c.pc = 0x0027bb08;
                        ee::s07::ScriptEngine_GetLocatorPose(c);
                        if c.pc != 0x0027a428 {
                            return;
                        }
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A3][0] = c.g[SP][0];
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
                        break 'b_27a440;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027a448);
                    c.pc = 0x003369d8;
                    call_indirect(c, 0x003369d8);
                    if c.pc != 0x0027a448 {
                        return;
                    }
                    let t = c.g[V0][0] == 0u64;
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                    if t {
                        break 'b_27a47c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_27a47c;
                        } else {
                            break 'b_27a458;
                        }
                    }
                }
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = c.g[S4][0];
                c.g[T1][0] = c.g[S5][0];
                c.g[A1][0] = 0x8;
                c.g[T2][0] = 0x32;
                c.g[RA][0] = sx(0x0027a47c);
                c.g[T3][0] = 0u64;
                c.pc = 0x0027bb08;
                ee::s07::ScriptEngine_GetLocatorPose(c);
                if c.pc != 0x0027a47c {
                    return;
                }
                break 'b_27a47c;
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        break 'b_27a480;
    }
    c.cycles += 8;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Rider_AttachToScript (0x123640)

/// `void Rider_AttachToScript(cRider* self)` (0x123640): put the rider under
/// script control (the NIS): starts the script object, sets sub state 13 and
/// main state 3, resets and plays the rider animation (index 5), records
/// facing (`+0x2d8 = +0x320`), the four bone slots `+0xb14..` to -1, reads the
/// script rider record (`0x27c958`: flags `+0xafc`/`+0xad0`), places the rider
/// at the script actor's frame (matrix multiply by `+0x700`, Euler angles in
/// degrees to radians by the global factor, `Quat_FromEuler`,
/// `Math_SinCos`, `Rider_BuildWorldMatrix`), clears velocity and 19 FX
/// tracks, switches off animation layers 5, 6, 8, 9 (`0x30e9e0`) and 4, 7
/// (`0x30ea80`), re-enables the equipment layers the character's bolts mark
/// (flag `0x40`/`0x10`) and ends with `Rider_HandleBoostInput(self, 0, 0)`.
/// Written register for register.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_parens
)]
pub fn rider_attach_to_script(c: &mut Cpu) {
    c.cycles += 40;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.g[S2][0] = c.g[A0][0];
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6d0));
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.g[S6][0] = 0x500000;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
    c.g[RA][0] = sx(0x0012367c);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.pc = 0x0027f9f8;
    call_indirect(c, 0x0027f9f8);
    if c.pc != 0x0012367c {
        return;
    }
    c.g[A0][0] = c.g[S2][0];
    c.g[RA][0] = sx(0x00123688);
    c.g[A1][0] = 0xd;
    c.pc = 0x0011fec8;
    ee::s00::Rider_SetSubState(c);
    if c.pc != 0x00123688 {
        return;
    }
    c.g[A1][0] = 0x3;
    c.g[RA][0] = sx(0x00123694);
    c.g[A0][0] = c.g[S2][0];
    c.pc = 0x0011fe78;
    ee::s00::Rider_SetMainState(c);
    if c.pc != 0x00123694 {
        return;
    }
    c.g[RA][0] = sx(0x0012369c);
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x784));
    c.pc = 0x00311a50;
    call_indirect(c, 0x00311a50);
    if c.pc != 0x0012369c {
        return;
    }
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x784));
    c.g[A1][0] = 0x5;
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 12);
    c.g[RA][0] = sx(0x001236b4);
    c.g[A2][0] = 0u64;
    c.pc = 0x003128e8;
    call_indirect(c, 0x003128e8);
    if c.pc != 0x001236b4 {
        return;
    }
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x320));
    c.g[V0][0] = 0x1;
    c.sw((c.g[S2][0] as u32).wrapping_add(0xac4), c.g[V0][0]);
    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x2d8), c.g[V1][0]);
    c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.sw((c.g[S2][0] as u32).wrapping_add(0xac8), 0u64);
    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x64));
    c.g[A1][0] = 0xffffffffffffffff;
    c.g[A0][0] = 0x3;
    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb14));
    'l_1236e0: loop {
        c.cycles += 7;
        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        let t = (c.g[A0][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x001236e0);
            }
            continue 'l_1236e0;
        } else {
            'b_123b0c: {
                c.cycles += 194;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 21);
                c.g[RA][0] = sx(0x00123710);
                c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xea0));
                c.pc = 0x0027c958;
                call_indirect(c, 0x0027c958);
                if c.pc != 0x00123710 {
                    return;
                }
                c.mtc1(0, 20);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x7));
                c.g[V0][0] = 0x1;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xb00), c.g[V0][0]);
                c.g[A1][0] = c.g[SP][0];
                c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xafc), c.g[V1][0]);
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                c.g[RA][0] = sx(0x00123744);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xad0), c.g[V0][0]);
                c.pc = 0x0027c9b0;
                call_indirect(c, 0x0027c9b0);
                if c.pc != 0x00123744 {
                    return;
                }
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x2c));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x700));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x78b0));
                c.mov_s(12, 20);
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.g[A1][0] = c.g[S3][0];
                c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.mul_s(1, 1, 0);
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.g[S2][0];
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x3c));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xaf8));
                c.mul_s(3, 3, 0);
                c.mul_s(2, 2, 0);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0xaf0));
                c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0xaf4));
                c.sq(2, (c.g[S2][0] as u32).wrapping_add(0xae0));
                c.lqc2(8, (c.g[S2][0] as u32).wrapping_add(0xae0));
                c.lqc2(4, (c.g[V1][0] as u32));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x744));
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x740));
                c.lwc1(13, (c.g[S2][0] as u32).wrapping_add(0xaf4));
                c.lwc1(14, (c.g[S2][0] as u32).wrapping_add(0xaf0));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.add_s(13, 13, 0);
                c.add_s(14, 14, 1);
                c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x110));
                c.g[RA][0] = sx(0x001237f0);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.pc = 0x001241c0;
                ee::s00::Quat_FromEuler(c);
                if c.pc != 0x001237f0 {
                    return;
                }
                c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x748));
                c.g[A1][0] = c.g[S4][0];
                c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0xaf8));
                c.g[A0][0] = c.g[S5][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x78ac));
                c.add_s(12, 12, 2);
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.sub_s(12, 12, 1);
                c.g[RA][0] = sx(0x0012381c);
                c.mul_s(12, 12, 0);
                c.pc = 0x0031be50;
                call_indirect(c, 0x0031be50);
                if c.pc != 0x0012381c {
                    return;
                }
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = c.g[S2][0];
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.lwc1(2, (c.g[S6][0] as u32).wrapping_sub(0xea0));
                c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.mul_s(1, 0, 1);
                c.mul_s(2, 0, 2);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.mul_s(0, 0, 4);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x120));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(5, (c.g[S2][0] as u32).wrapping_add(0x120));
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x120));
                c.g[RA][0] = sx(0x00123898);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = 0x0011e098;
                ee::s00::Rider_BuildWorldMatrix(c);
                if c.pc != 0x00123898 {
                    return;
                }
                c.g[V0][0] = 0x500000;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x77c));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
                c.lq(3, (c.g[V0][0] as u32));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0xb04));
                c.g[RA][0] = sx(0x001238b4);
                c.sq(3, (c.g[S2][0] as u32).wrapping_add(0x1e0));
                c.pc = 0x00111890;
                ee::s00::RiderBase_ResetFX(c);
                if c.pc != 0x001238b4 {
                    return;
                }
                c.swc1(21, (c.g[S2][0] as u32).wrapping_add(0x25c));
                c.g[A1][0] = 0x5;
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x1f8));
                c.g[A2][0] = 0u64;
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x1f0));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x1f4));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x204));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x1fc));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x200));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x210));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x208));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x20c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x21c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x214));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x218));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x228));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x220));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x224));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x234));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x22c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x230));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x240));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x238));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x23c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x24c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x244));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x248));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x258));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x250));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x254));
                c.swc1(21, (c.g[S2][0] as u32).wrapping_add(0x264));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x260));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x270));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x268));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x26c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x27c));
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x780));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2cc));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x274));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x278));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x288));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x280));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x284));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x294));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x28c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x290));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2a0));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x298));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x29c));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2ac));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2a4));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2a8));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2b8));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2b0));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2b4));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2c4));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2bc));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2c0));
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2d0));
                c.g[RA][0] = sx(0x001239a8);
                c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x2c8));
                c.pc = 0x0030e9e0;
                call_indirect(c, 0x0030e9e0);
                if c.pc != 0x001239a8 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x780));
                c.g[A1][0] = 0x6;
                c.g[RA][0] = sx(0x001239b8);
                c.g[A2][0] = 0u64;
                c.pc = 0x0030e9e0;
                call_indirect(c, 0x0030e9e0);
                if c.pc != 0x001239b8 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x780));
                c.g[A1][0] = 0x8;
                c.g[RA][0] = sx(0x001239c8);
                c.g[A2][0] = 0u64;
                c.pc = 0x0030e9e0;
                call_indirect(c, 0x0030e9e0);
                if c.pc != 0x001239c8 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x780));
                c.g[A1][0] = 0x9;
                c.g[RA][0] = sx(0x001239d8);
                c.g[A2][0] = 0u64;
                c.pc = 0x0030e9e0;
                call_indirect(c, 0x0030e9e0);
                if c.pc != 0x001239d8 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x780));
                c.g[A1][0] = 0x4;
                c.g[RA][0] = sx(0x001239e8);
                c.g[A2][0] = 0u64;
                c.pc = 0x0030ea80;
                call_indirect(c, 0x0030ea80);
                if c.pc != 0x001239e8 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x780));
                c.g[A1][0] = 0x7;
                c.g[RA][0] = sx(0x001239f8);
                c.g[A2][0] = 0u64;
                c.pc = 0x0030ea80;
                call_indirect(c, 0x0030ea80);
                if c.pc != 0x001239f8 {
                    return;
                }
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x6));
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = c.g[S2][0];
                    break 'b_123b0c;
                } else {
                    'b_123b08: {
                        c.cycles += 24;
                        c.g[RA][0] = sx(0x00123a0c);
                        c.pc = 0x0014dc80;
                        call_indirect(c, 0x0014dc80);
                        if c.pc != 0x00123a0c {
                            return;
                        }
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00123a18);
                        c.g[A1][0] = 0x9;
                        c.pc = 0x0014dd58;
                        call_indirect(c, 0x0014dd58);
                        if c.pc != 0x00123a18 {
                            return;
                        }
                        c.g[RA][0] = sx(0x00123a20);
                        c.g[S0][0] = c.g[V0][0];
                        c.pc = 0x0014bdb8;
                        call_indirect(c, 0x0014bdb8);
                        if c.pc != 0x00123a20 {
                            return;
                        }
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x86c));
                        c.g[S1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00123a30);
                        c.g[A0][0] = c.g[S0][0];
                        c.pc = 0x0014ad50;
                        call_indirect(c, 0x0014ad50);
                        if c.pc != 0x00123a30 {
                            return;
                        }
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00123a3c);
                        c.g[S4][0] = c.g[V0][0];
                        c.pc = 0x00123128;
                        ee::s00::Rider_GetCharacter(c);
                        if c.pc != 0x00123a3c {
                            return;
                        }
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00123a4c);
                        c.g[A1][0] = c.g[S0][0];
                        c.pc = 0x0014d988;
                        call_indirect(c, 0x0014d988);
                        if c.pc != 0x00123a4c {
                            return;
                        }
                        c.g[A0][0] = c.g[S1][0];
                        c.g[S3][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00123a5c);
                        c.g[A1][0] = c.g[S0][0];
                        c.pc = 0x0014d998;
                        call_indirect(c, 0x0014d998);
                        if c.pc != 0x00123a5c {
                            return;
                        }
                        let t = (c.g[S3][0] as i64) <= 0;
                        c.g[S1][0] = c.g[V0][0];
                        if t {
                            break 'b_123b08;
                        } else {
                            c.cycles += 1;
                            'l_123a68: loop {
                                'b_123b00: {
                                    c.cycles += 3;
                                    c.g[S0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x10));
                                    if (c.g[S0][0] as i64) < 0 {
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                        break 'b_123b00;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                        c.g[V0][0] = c.g[V0][0] & 0x40;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                            break 'b_123b00;
                                        } else {
                                            'b_123ab0: {
                                                'b_123aac: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] =
                                                        c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S4][0] as u32).wrapping_add(0x288)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    if t {
                                                        break 'b_123aac;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x290));
                                                        let t = true;
                                                        c.g[V0][0] = sx((c.g[S4][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        if t {
                                                            break 'b_123ab0;
                                                        } else {
                                                            break 'b_123aac;
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = 0u64;
                                                break 'b_123ab0;
                                            }
                                            'b_123afc: {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                                                c.g[V0][0] = c.g[V0][0] & 0x10;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_123afc;
                                                } else {
                                                    c.cycles += 15;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x780)
                                                        );
                                                    c.g[RA][0] = sx(0x00123acc);
                                                    c.g[A2][0] = 0u64;
                                                    c.pc = 0x0030ea80;
                                                    call_indirect(c, 0x0030ea80);
                                                    if c.pc != 0x00123acc {
                                                        return;
                                                    }
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x780)
                                                        );
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00123adc);
                                                    c.g[A2][0] = 0x1;
                                                    c.pc = 0x0030ea80;
                                                    call_indirect(c, 0x0030ea80);
                                                    if c.pc != 0x00123adc {
                                                        return;
                                                    }
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x780)
                                                        );
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00123aec);
                                                    c.g[A2][0] = 0x2;
                                                    c.pc = 0x0030ea80;
                                                    call_indirect(c, 0x0030ea80);
                                                    if c.pc != 0x00123aec {
                                                        return;
                                                    }
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x780)
                                                        );
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00123afc);
                                                    c.g[A2][0] = 0x3;
                                                    c.pc = 0x0030ea80;
                                                    call_indirect(c, 0x0030ea80);
                                                    if c.pc != 0x00123afc {
                                                        return;
                                                    }
                                                    break 'b_123afc;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                            break 'b_123b00;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S3][0] != 0u64;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00123a68);
                                    }
                                    continue 'l_123a68;
                                } else {
                                    break 'b_123b08;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S2][0];
                    break 'b_123b0c;
                }
            }
            c.cycles += 15;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x00123b18);
            c.g[A2][0] = 0u64;
            c.pc = 0x00114130;
            ee::s00::Rider_HandleBoostInput(c);
            if c.pc != 0x00123b18 {
                return;
            }
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
            c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
            c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
            c.pc = t;
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// cLunoTable_Set (0x225338)

/// `void cLunoTable::set(cLunoTable* self, const cLunoValue* key, const
/// cLunoValue* val)` (0x225338): the Luno script hash table's insert/update
/// of `key -> val` (keys of type 0 are ignored): the bucket is `key.word0 %
/// modulus`; an empty bucket takes key and value in place, a matching key
/// is overwritten (releasing a replaced table value, copying a pair, else
/// the word; a value that becomes nil unlinks and deletes its node), and
/// otherwise a new 0x20-byte `cLunoTableEntry` node is allocated and linked
/// after the chain's last node. Written register for register (the original
/// has this exact unrolled shape; its blocks are kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_parens
)]
pub fn c_luno_table_set(c: &mut Cpu) {
    'b_2257c0: {
        'b_2257bc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let t = c.g[A3][0] == 0u64;
            c.g[S2][0] = c.g[A2][0];
            if t {
                break 'b_2257bc;
            } else {
                'b_225380: {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[T0][0] = c.g[V0][0];
                    c.divu(0, 2, 3);
                    if c.g[V1][0] == 0u64 {
                        break 'b_225380;
                    } else {
                        break 'b_225380;
                    }
                }
                'b_225554: {
                    'b_22554c: {
                        'b_225544: {
                            'b_22552c: {
                                'b_225524: {
                                    'b_225510: {
                                        'b_225448: {
                                            c.cycles += 7;
                                            c.g[A1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.hi[0];
                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                                            c.g[S0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                break 'b_225448;
                                            } else {
                                                'b_2253bc: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[S3][0] = 0x4;
                                                    if c.g[V0][0] != c.g[S3][0] {
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8),
                                                            0u64,
                                                        );
                                                        break 'b_2253bc;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x002253b8);
                                                        c.g[A1][0] = 0x2;
                                                        c.pc = 0x00224df0;
                                                        ee::s05::cLunoTable_dtor(c);
                                                        if c.pc != 0x002253b8 {
                                                            return;
                                                        }
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8),
                                                            0u64,
                                                        );
                                                        break 'b_2253bc;
                                                    }
                                                }
                                                'b_225404: {
                                                    'b_2253e0: {
                                                        c.cycles += 4;
                                                        c.sw((c.g[S0][0] as u32), 0u64);
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        let t = c.g[V1][0] != c.g[S3][0];
                                                        c.g[V0][0] = 0x5;
                                                        if t {
                                                            break 'b_2253e0;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x002253d8);
                                                            c.g[A1][0] = c.g[S1][0];
                                                            c.pc = 0x00225068;
                                                            ee::s05::cLunoTable_Assign(c);
                                                            if c.pc != 0x002253d8 {
                                                                return;
                                                            }
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_225404;
                                                            } else {
                                                                break 'b_2253e0;
                                                            }
                                                        }
                                                    }
                                                    'b_225400: {
                                                        'b_2253fc: {
                                                            c.cycles += 2;
                                                            if c.g[V1][0] != c.g[V0][0] {
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[S1][0] as u32));
                                                                break 'b_2253fc;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.ldl(
                                                                    2,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                c.ldr(2, (c.g[S1][0] as u32));
                                                                c.sdl(
                                                                    2,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                let t = true;
                                                                c.sdr(2, (c.g[S0][0] as u32));
                                                                if t {
                                                                    break 'b_225400;
                                                                } else {
                                                                    break 'b_2253fc;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                                                        break 'b_225400;
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_225404;
                                                }
                                                'b_225424: {
                                                    c.cycles += 5;
                                                    c.g[S3][0] = 0x4;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                    c.g[S1][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[V0][0] != c.g[S3][0];
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x8),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        break 'b_225424;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x00225424);
                                                        c.g[A1][0] = 0x2;
                                                        c.pc = 0x00224df0;
                                                        ee::s05::cLunoTable_dtor(c);
                                                        if c.pc != 0x00225424 {
                                                            return;
                                                        }
                                                        break 'b_225424;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                let t = c.g[V1][0] == c.g[S3][0];
                                                c.g[V0][0] = 0x5;
                                                if t {
                                                    break 'b_225510;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[V0][0];

                                                    if t {
                                                        break 'b_22552c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                        if t {
                                                            break 'b_225544;
                                                        } else {
                                                            break 'b_225448;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_2254dc: {
                                            'b_2254d4: {
                                                c.cycles += 2;
                                                if c.g[A3][0] == c.g[A0][0] {
                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                    break 'b_2254d4;
                                                } else {
                                                    'b_225484: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x2;
                                                        let t = c.g[A0][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_225484;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A3][0] != c.g[V0][0];

                                                            if t {
                                                                break 'b_225484;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.mtc1(8, 1);
                                                                c.cvt_s_w(1, 1);
                                                                c.lwc1(0, (c.g[S0][0] as u32));
                                                                c.c_eq_s(0, 1);
                                                                let t = c.fcc();

                                                                if t {
                                                                    break 'b_2254dc;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[V0][0] = 0u64;
                                                                    if t {
                                                                        break 'b_2254dc;
                                                                    } else {
                                                                        break 'b_225484;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_2254b8: {
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x3;
                                                        if t {
                                                            break 'b_2254b8;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = 0x2;
                                                            if c.g[V1][0] != c.g[V0][0] {
                                                                c.g[V0][0] = 0x3;
                                                                break 'b_2254b8;
                                                            } else {
                                                                c.cycles += 7;
                                                                c.lwc1(0, (c.g[S1][0] as u32));
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[S0][0] as u32));
                                                                c.cvt_w_s(1, 0);
                                                                c.mfc1(3, 1);
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] ^ c.g[V1][0];
                                                                let t = true;
                                                                c.g[V0][0] =
                                                                    (c.g[V0][0] < 0x1u64) as u64;
                                                                if t {
                                                                    break 'b_2254dc;
                                                                } else {
                                                                    break 'b_2254b8;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = c.g[A0][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_2254dc;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = 0x1;
                                                        if c.g[V1][0] != c.g[V0][0] {
                                                            c.g[V0][0] = 0u64;
                                                            break 'b_2254dc;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                            break 'b_2254d4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = c.g[V0][0] ^ c.g[T0][0];
                                            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                            break 'b_2254dc;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = 0x4;
                                        if t {
                                            break 'b_225554;
                                        } else {
                                            'b_2254fc: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                let t = c.g[V0][0] != c.g[S3][0];
                                                c.g[S1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xc));
                                                if t {
                                                    break 'b_2254fc;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[RA][0] = sx(0x002254fc);
                                                    c.g[A1][0] = 0x2;
                                                    c.pc = 0x00224df0;
                                                    ee::s05::cLunoTable_dtor(c);
                                                    if c.pc != 0x002254fc {
                                                        return;
                                                    }
                                                    break 'b_2254fc;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] != c.g[S3][0];
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_225524;
                                            } else {
                                                break 'b_225510;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x0022551c);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.pc = 0x00225068;
                                    ee::s05::cLunoTable_Assign(c);
                                    if c.pc != 0x0022551c {
                                        return;
                                    }
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_22554c;
                                    } else {
                                        break 'b_225524;
                                    }
                                }
                                c.cycles += 2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                    break 'b_225544;
                                } else {
                                    break 'b_22552c;
                                }
                            }
                            c.cycles += 6;
                            c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                            c.ldr(2, (c.g[S2][0] as u32));
                            c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x13));
                            c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_22554c;
                            } else {
                                break 'b_225544;
                            }
                        }
                        c.cycles += 2;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        break 'b_22554c;
                    }
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        break 'b_2257bc;
                    } else {
                        break 'b_225554;
                    }
                }
                'b_2256bc: {
                    c.cycles += 4;
                    c.g[S3][0] = c.g[S0][0];
                    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                    if c.g[S0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        break 'b_2256bc;
                    } else {
                        c.cycles += 5;
                        c.g[A2][0] = 0x2;
                        c.g[A1][0] = 0x1;
                        c.g[S4][0] = 0x4;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        'l_225578: loop {
                            'b_225600: {
                                'b_2255f8: {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if c.g[V1][0] == c.g[A0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                        break 'b_2255f8;
                                    } else {
                                        'b_2255b8: {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] != c.g[A2][0];

                                            if t {
                                                break 'b_2255b8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[A1][0];

                                                if t {
                                                    break 'b_2255b8;
                                                } else {
                                                    c.cycles += 7;
                                                    c.mtc1(8, 1);
                                                    c.cvt_s_w(1, 1);
                                                    c.lwc1(0, (c.g[S0][0] as u32));
                                                    c.c_eq_s(0, 1);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_225600;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] = 0u64;
                                                        if t {
                                                            break 'b_225600;
                                                        } else {
                                                            break 'b_2255b8;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_2255e4: {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] != c.g[A1][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_2255e4;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[A2][0];

                                                if t {
                                                    break 'b_2255e4;
                                                } else {
                                                    c.cycles += 7;
                                                    c.lwc1(0, (c.g[S1][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                    c.cvt_w_s(1, 0);
                                                    c.mfc1(3, 1);
                                                    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                                                    let t = true;
                                                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                                    if t {
                                                        break 'b_225600;
                                                    } else {
                                                        break 'b_2255e4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[A0][0] != c.g[V0][0] {
                                            c.g[V0][0] = 0u64;
                                            break 'b_225600;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[A1][0] {
                                                c.g[V0][0] = 0u64;
                                                break 'b_225600;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                                break 'b_2255f8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.g[V0][0] ^ c.g[T0][0];
                                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                break 'b_225600;
                            }
                            'b_2256ac: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0u64 {
                                    c.g[S3][0] = c.g[S0][0];
                                    break 'b_2256ac;
                                } else {
                                    'b_225620: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        let t = c.g[V0][0] != c.g[S4][0];
                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                                        if t {
                                            break 'b_225620;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x00225620);
                                            c.g[A1][0] = 0x2;
                                            c.pc = 0x00224df0;
                                            ee::s05::cLunoTable_dtor(c);
                                            if c.pc != 0x00225620 {
                                                return;
                                            }
                                            break 'b_225620;
                                        }
                                    }
                                    'b_225670: {
                                        'b_225648: {
                                            c.cycles += 5;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V1][0] != c.g[S4][0];
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_225648;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x00225640);
                                                c.g[A1][0] = c.g[S2][0];
                                                c.pc = 0x00225068;
                                                ee::s05::cLunoTable_Assign(c);
                                                if c.pc != 0x00225640 {
                                                    return;
                                                }
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_225670;
                                                } else {
                                                    break 'b_225648;
                                                }
                                            }
                                        }
                                        'b_225668: {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                break 'b_225668;
                                            } else {
                                                c.cycles += 6;
                                                c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                                                c.ldr(2, (c.g[S2][0] as u32));
                                                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x13));
                                                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_225670;
                                                } else {
                                                    break 'b_225668;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        break 'b_225670;
                                    }
                                    c.cycles += 4;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    if c.g[V1][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        break 'b_2257c0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        let t = c.g[S0][0] == 0u64;
                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                        if t {
                                            break 'b_2257bc;
                                        } else {
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                            c.g[A1][0] = 0x3;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x002256a4);
                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            c.pc = t;
                                            call_indirect(c, t);
                                            if c.pc != 0x002256a4 {
                                                return;
                                            }
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_2257c0;
                                            } else {
                                                break 'b_2256ac;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                            if c.g[S0][0] != 0u64 {
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if event_due(c) {
                                    deliver_event_here(c, 0x00225578);
                                }
                                continue 'l_225578;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                break 'b_2256bc;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x480000;
                if t {
                    break 'b_2257bc;
                } else {
                    'b_225740: {
                        'b_225718: {
                            c.cycles += 16;
                            c.g[A0][0] = 0x20;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6780));
                            c.g[A2][0] = 0x20000000;
                            c.g[RA][0] = sx(0x002256d8);
                            c.g[A3][0] = 0u64;
                            c.pc = 0x00317d70;
                            call_indirect(c, 0x00317d70);
                            if c.pc != 0x002256d8 {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x480000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6770));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                            c.g[V1][0] = 0x4;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            let t = c.g[A0][0] != c.g[V1][0];
                            c.sw((c.g[S0][0] as u32), 0u64);
                            if t {
                                break 'b_225718;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00225710);
                                c.g[A1][0] = c.g[S1][0];
                                c.pc = 0x00225068;
                                ee::s05::cLunoTable_Assign(c);
                                if c.pc != 0x00225710 {
                                    return;
                                }
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_225740;
                                } else {
                                    break 'b_225718;
                                }
                            }
                        }
                        'b_22573c: {
                            'b_225738: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                if c.g[A0][0] != c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                    break 'b_225738;
                                } else {
                                    c.cycles += 5;
                                    c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x7));
                                    c.ldr(2, (c.g[S1][0] as u32));
                                    c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                                    let t = true;
                                    c.sdr(2, (c.g[S0][0] as u32));
                                    if t {
                                        break 'b_22573c;
                                    } else {
                                        break 'b_225738;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                            break 'b_22573c;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        break 'b_225740;
                    }
                    'b_225760: {
                        c.cycles += 5;
                        c.g[S4][0] = 0x4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V0][0] != c.g[S4][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        if t {
                            break 'b_225760;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00225760);
                            c.g[A1][0] = 0x2;
                            c.pc = 0x00224df0;
                            ee::s05::cLunoTable_dtor(c);
                            if c.pc != 0x00225760 {
                                return;
                            }
                            break 'b_225760;
                        }
                    }
                    'b_2257b0: {
                        'b_225784: {
                            c.cycles += 4;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            let t = c.g[V1][0] != c.g[S4][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            if t {
                                break 'b_225784;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x0022577c);
                                c.g[A1][0] = c.g[S2][0];
                                c.pc = 0x00225068;
                                ee::s05::cLunoTable_Assign(c);
                                if c.pc != 0x0022577c {
                                    return;
                                }
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_2257b0;
                                } else {
                                    break 'b_225784;
                                }
                            }
                        }
                        'b_2257a8: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x5;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                break 'b_2257a8;
                            } else {
                                c.cycles += 6;
                                c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                                c.ldr(2, (c.g[S2][0] as u32));
                                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x13));
                                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_2257b0;
                                } else {
                                    break 'b_2257a8;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        break 'b_2257b0;
                    }
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                    break 'b_2257bc;
                }
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        break 'b_2257c0;
    }
    c.cycles += 7;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Control block of the gated fakes: the gate word, the call counter, the
    /// call index that unwinds, forced results (`+0x10 + 4 * id`), forced
    /// `f0` results (`+0x80 + 4 * id`) and the call log (`+0x100`).
    pub(crate) const OC417_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC417_MAGIC: u32 = 0x0c41_7a17;

    /// A gated fake callee: records its arguments, returns the forced result
    /// of its id, and unwinds at the chosen call index.
    fn oc417_fake(c: &mut Cpu, id: u32) {
        let n = c.bus.read32(OC417_CTRL + 4);
        c.bus.write32(OC417_CTRL + 4, n.wrapping_add(1));
        let p = OC417_CTRL + 0x100 + (n & 0x3f) * 0x40;
        c.bus.write32(p, id);
        c.bus.write32(p + 4, c.pc);
        for i in 0..4usize {
            let v = c.g[A0 + i][0];
            c.bus.write64(p + 8 + 8 * i as u32, v);
        }
        let (f12, f13) = (c.f[12], c.f[13]);
        c.bus.write32(p + 0x28, f12);
        c.bus.write32(p + 0x2c, f13);
        let (ra, spv, cyc) = (c.g[RA][0] as u32, c.g[SP][0] as u32, c.cycles);
        c.bus.write32(p + 0x30, ra);
        c.bus.write32(p + 0x34, spv);
        c.bus.write64(p + 0x38, cyc);
        let forced = c.bus.read32(OC417_CTRL + 0x10 + 4 * id);
        c.g[V0][0] = sx(forced);
        c.g[V1][0] = 0x5151_0000 + n as u64;
        c.f[0] = c.bus.read32(OC417_CTRL + 0x80 + 4 * id);
        c.cycles += 5;
        if n == c.bus.read32(OC417_CTRL + 8) {
            c.pc = 0xdead_0000 | n << 4;
        } else {
            c.pc = c.g[RA][0] as u32;
        }
    }

    macro_rules! oc417_gated {
        ($name:ident, $prev:ident, $id:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC417_CTRL) == OC417_MAGIC {
                    return oc417_fake(c, $id);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    // Fake ids (index into the forced-result table).
    const FK_BE_INSTANCE: u32 = 0;
    const FK_BE_INTERFACE: u32 = 1;
    const FK_READ_U32: u32 = 2;
    const FK_NEW_NAMED: u32 = 3;
    const FK_BOLT_COUNT: u32 = 4;
    const FK_BOLTS: u32 = 5;
    const FK_HEAP_FREE: u32 = 6;
    const FK_QUEUE_SECTION: u32 = 7;
    const FK_ADD_MAGNET: u32 = 8;
    const FK_LUNO_DTOR: u32 = 9;
    const FK_LUNO_ASSIGN: u32 = 10;
    const FK_MEM_ALLOC: u32 = 11;
    const FK_BE_TABLE: u32 = 12;
    const FK_BE_DB_ENTRY: u32 = 13;
    const FK_GET_CHARACTER: u32 = 14;
    const FK_SET_SUB_STATE: u32 = 15;
    const FK_SET_MAIN_STATE: u32 = 16;
    const FK_RESET_FX: u32 = 17;
    const FK_BOOST_INPUT: u32 = 18;
    const FK_BUILD_MATRIX: u32 = 19;
    const FK_QUAT_EULER: u32 = 20;
    const FK_ANIM_LAYER_A: u32 = 21;
    const FK_ANIM_LAYER_B: u32 = 22;
    const FK_ANIM_RESET: u32 = 23;
    const FK_ANIM_PLAY: u32 = 24;
    const FK_SINCOS: u32 = 25;

    oc417_gated!(
        fake_be_instance,
        PREV_BE_INSTANCE,
        FK_BE_INSTANCE,
        ee::s01::f_0014dc80__generated
    );
    oc417_gated!(
        fake_be_interface,
        PREV_BE_INTERFACE,
        FK_BE_INTERFACE,
        ee::s01::f_0014dd58__generated
    );
    oc417_gated!(
        fake_read_u32,
        PREV_READ_U32,
        FK_READ_U32,
        ee::s09::f_002d2510__generated
    );
    oc417_gated!(
        fake_new_named,
        PREV_NEW_NAMED,
        FK_NEW_NAMED,
        ee::s10::f_00317e30__generated
    );
    oc417_gated!(
        fake_bolt_count,
        PREV_BOLT_COUNT,
        FK_BOLT_COUNT,
        ee::s01::f_0014d988__generated
    );
    oc417_gated!(
        fake_bolts,
        PREV_BOLTS,
        FK_BOLTS,
        ee::s01::f_0014d998__generated
    );
    oc417_gated!(
        fake_heap_free,
        PREV_HEAP_FREE,
        FK_HEAP_FREE,
        ee::s07::f_002723a8__generated
    );
    oc417_gated!(
        fake_queue_section,
        PREV_QUEUE_SECTION,
        FK_QUEUE_SECTION,
        ee::s05::f_0022e298__generated
    );
    oc417_gated!(
        fake_add_magnet,
        PREV_ADD_MAGNET,
        FK_ADD_MAGNET,
        ee::s11::f_003559f8__generated
    );
    oc417_gated!(
        fake_luno_dtor,
        PREV_LUNO_DTOR,
        FK_LUNO_DTOR,
        ee::s05::f_00224df0__generated
    );
    oc417_gated!(
        fake_luno_assign,
        PREV_LUNO_ASSIGN,
        FK_LUNO_ASSIGN,
        ee::s05::f_00225068__generated
    );
    oc417_gated!(
        fake_mem_alloc,
        PREV_MEM_ALLOC,
        FK_MEM_ALLOC,
        ee::s10::f_00317d70__generated
    );
    oc417_gated!(
        fake_be_table,
        PREV_BE_TABLE,
        FK_BE_TABLE,
        ee::s01::f_0014bdb8__generated
    );
    oc417_gated!(
        fake_be_db_entry,
        PREV_BE_DB_ENTRY,
        FK_BE_DB_ENTRY,
        ee::s01::f_0014ad50__generated
    );
    oc417_gated!(
        fake_get_character,
        PREV_GET_CHARACTER,
        FK_GET_CHARACTER,
        ee::s00::f_00123128__generated
    );
    oc417_gated!(
        fake_set_sub_state,
        PREV_SET_SUB_STATE,
        FK_SET_SUB_STATE,
        ee::s00::f_0011fec8__generated
    );
    oc417_gated!(
        fake_set_main_state,
        PREV_SET_MAIN_STATE,
        FK_SET_MAIN_STATE,
        ee::s00::f_0011fe78__generated
    );
    oc417_gated!(
        fake_reset_fx,
        PREV_RESET_FX,
        FK_RESET_FX,
        ee::s00::f_00111890__generated
    );
    oc417_gated!(
        fake_boost_input,
        PREV_BOOST_INPUT,
        FK_BOOST_INPUT,
        ee::s00::f_00114130__generated
    );
    oc417_gated!(
        fake_build_matrix,
        PREV_BUILD_MATRIX,
        FK_BUILD_MATRIX,
        ee::s00::f_0011e098__generated
    );
    oc417_gated!(
        fake_quat_euler,
        PREV_QUAT_EULER,
        FK_QUAT_EULER,
        ee::s00::f_001241c0__generated
    );
    oc417_gated!(
        fake_anim_layer_a,
        PREV_ANIM_LAYER_A,
        FK_ANIM_LAYER_A,
        ee::s10::f_0030e9e0__generated
    );
    oc417_gated!(
        fake_anim_layer_b,
        PREV_ANIM_LAYER_B,
        FK_ANIM_LAYER_B,
        ee::s10::f_0030ea80__generated
    );
    oc417_gated!(
        fake_anim_reset,
        PREV_ANIM_RESET,
        FK_ANIM_RESET,
        ee::s10::f_00311a50__generated
    );
    oc417_gated!(
        fake_anim_play,
        PREV_ANIM_PLAY,
        FK_ANIM_PLAY,
        ee::s10::f_003128e8__generated
    );
    oc417_gated!(
        fake_sincos,
        PREV_SINCOS,
        FK_SINCOS,
        ee::s10::f_0031be50__generated
    );

    /// Put every fake of this module in its override slot (once per
    /// process; with the gate word clear they run what the slot held).
    fn oc417_hook_fakes() {
        use std::ptr::addr_of_mut;
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu), &AtomicUsize); 26] = [
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                fake_be_instance,
                &PREV_BE_INSTANCE,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                fake_be_interface,
                &PREV_BE_INTERFACE,
            ),
            (
                addr_of_mut!(ee::s09::OVERRIDE_002D2510),
                fake_read_u32,
                &PREV_READ_U32,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00317E30),
                fake_new_named,
                &PREV_NEW_NAMED,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014D988),
                fake_bolt_count,
                &PREV_BOLT_COUNT,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014D998),
                fake_bolts,
                &PREV_BOLTS,
            ),
            (
                addr_of_mut!(ee::s07::OVERRIDE_002723A8),
                fake_heap_free,
                &PREV_HEAP_FREE,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_0022E298),
                fake_queue_section,
                &PREV_QUEUE_SECTION,
            ),
            (
                addr_of_mut!(ee::s11::OVERRIDE_003559F8),
                fake_add_magnet,
                &PREV_ADD_MAGNET,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_00224DF0),
                fake_luno_dtor,
                &PREV_LUNO_DTOR,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_00225068),
                fake_luno_assign,
                &PREV_LUNO_ASSIGN,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00317D70),
                fake_mem_alloc,
                &PREV_MEM_ALLOC,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014BDB8),
                fake_be_table,
                &PREV_BE_TABLE,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014AD50),
                fake_be_db_entry,
                &PREV_BE_DB_ENTRY,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00123128),
                fake_get_character,
                &PREV_GET_CHARACTER,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011FEC8),
                fake_set_sub_state,
                &PREV_SET_SUB_STATE,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011FE78),
                fake_set_main_state,
                &PREV_SET_MAIN_STATE,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00111890),
                fake_reset_fx,
                &PREV_RESET_FX,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00114130),
                fake_boost_input,
                &PREV_BOOST_INPUT,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                fake_build_matrix,
                &PREV_BUILD_MATRIX,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_001241C0),
                fake_quat_euler,
                &PREV_QUAT_EULER,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0030E9E0),
                fake_anim_layer_a,
                &PREV_ANIM_LAYER_A,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0030EA80),
                fake_anim_layer_b,
                &PREV_ANIM_LAYER_B,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00311A50),
                fake_anim_reset,
                &PREV_ANIM_RESET,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_003128E8),
                fake_anim_play,
                &PREV_ANIM_PLAY,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0031BE50),
                fake_sincos,
                &PREV_SINCOS,
            ),
        ];
        for (slot, fake, prev) in slots {
            fakes::hook(slot, fake, prev);
        }
    }

    /// Scrambled machine with a game gp, the fakes' gate open and (one seed
    /// in three, if `calls > 0`) a gated fake call that unwinds. The
    /// generic fakes (`fakes::machine`) take the vtable calls; their own
    /// unwinding is chosen with the same `calls`.
    fn oc417_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc417_hook_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC417_CTRL, OC417_MAGIC);
        let unwind = if calls > 0 && seed % 3 == 1 {
            r.next_u32() % calls
        } else {
            u32::MAX
        };
        c.bus.write32(OC417_CTRL + 8, unwind);
        for id in 0..32 {
            c.bus.write32(OC417_CTRL + 0x10 + 4 * id, 0);
        }
        r
    }

    /// Force the result of fake `id`.
    fn oc417_force(c: &mut Cpu, id: u32, v: u32) {
        c.bus.write32(OC417_CTRL + 0x10 + 4 * id, v);
    }

    /// A short NUL-terminated string.
    fn oc417_put_str(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    // ----------------------------------------------------------------------
    // RiderModel_GetBoneMask

    /// A rider model with 4 parts; the part table, bone names and a token
    /// list in random combinations: part index in or out of range, no table,
    /// negative table entries, tokens that are the morph name, a bone name,
    /// or nothing known, empty tokens, trailing commas.
    #[test]
    fn rider_model_get_bone_mask_matches() {
        const MODEL: u32 = DATA + 0x1000;
        const TABLE: u32 = DATA + 0x1100;
        const PARTS: u32 = DATA + 0x1200;
        const NAMES: u32 = DATA + 0x2000;
        const TOKENS: u32 = DATA + 0x3000;
        const WORDS: [&str; 6] = ["head", "arm", "leg", "torso", "morph", "bogus"];
        check(
            ee::s10::f_00310ce8__generated,
            rider_model_get_bone_mask,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4170_0001, 0);
                c.pc = 0x0031_0ce8;
                oc417_put_str(c, BONE_MASK_STR_MORPH, "morph");
                c.bus.write32(MODEL + BONE_MASK_MODEL_PARTS, PARTS);
                c.bus
                    .write32(MODEL + BONE_MASK_MODEL_NUM_BONES, r.next_u32() % 40);
                c.bus.write32(
                    MODEL + BONE_MASK_MODEL_MAX_PART,
                    if seed % 9 == 0 { 1 } else { 3 },
                );
                c.bus.write32(
                    MODEL + BONE_MASK_MODEL_PART_INDEX,
                    if seed % 11 == 0 { 0 } else { TABLE },
                );
                for i in 0..4u32 {
                    let entry = if (seed / 3 + i) % 7 == 0 {
                        0xffff_ffff
                    } else {
                        i
                    };
                    c.bus.write32(TABLE + 4 * i, entry);
                    let part = PARTS + 0x58 * i;
                    c.bus
                        .write32(part + BONE_MASK_PART_FIRST_BONE, r.next_u32() % 30);
                    let morph = if (seed + i) % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 20
                    };
                    c.bus.write32(part + BONE_MASK_PART_MORPH, morph);
                    c.bus
                        .write32(part + BONE_MASK_PART_NAMES, NAMES + 0x140 * i);
                    c.bus
                        .write32(part + BONE_MASK_PART_NUM_BONES, 1 + r.next_u32() % 4);
                    for j in 0..4u32 {
                        let w = WORDS[((seed + i + j) % 5) as usize];
                        oc417_put_str(c, NAMES + 0x140 * i + BONE_MASK_NAME_STRIDE * j, w);
                    }
                }
                let mut tokens = String::new();
                if seed % 7 == 3 {
                    tokens.push(',');
                }
                for t in 0..(seed % 5) {
                    if t > 0 {
                        tokens.push(',');
                    }
                    tokens.push_str(WORDS[(r.next_u32() % 6) as usize]);
                }
                if seed % 8 == 5 {
                    tokens.push(',');
                }
                oc417_put_str(c, TOKENS, &tokens);
                c.g[A0][0] = MODEL as u64;
                c.g[A1][0] = match seed % 10 {
                    0 => 4,
                    1 => 0xffff_ffff_ffff_ffff,
                    n => (n % 4) as u64,
                };
                c.g[A2][0] = TOKENS as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // WScriptFn_EventTypeMatches

    /// Event type below and above 8; table entry equal to the current event
    /// byte or not; the BE lookups faked (and unwinding at one of them).
    #[test]
    fn wscript_fn_event_type_matches_matches() {
        check(
            ee::s10::f_00303df8__generated,
            wscript_fn_event_type_matches,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0002, 2);
                c.pc = 0x0030_3df8;
                let current = r.next_u32() % 4;
                c.bus.write8(0x0053_5c10, current as u8);
                for i in 0..8u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        current
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_65f8 + 4 * i, v);
                }
                c.g[A0][0] = (seed % 12) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // CameraTrigger_ReadSphereTriggerData

    /// The trigger object with its descriptor: five `{delta, function}`
    /// accessors (generic fakes: they return pointers) and the readers
    /// (`Serialize_ReadU32` faked with forced results; `0x2d25a0` runs for
    /// real on top of it).
    #[test]
    fn camera_trigger_read_sphere_trigger_data_matches() {
        const OBJ: u32 = DATA + 0x400;
        const DESC: u32 = DATA + 0x800;
        check(
            ee::s02::f_00172fe0__generated,
            camera_trigger_read_sphere_trigger_data,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0003, 10);
                c.pc = 0x0017_2fe0;
                oc417_force(c, FK_READ_U32, r.next_u32() % 5 + 1);
                c.bus.write32(OBJ + 0x24, DESC);
                for (k, off) in [0x8u32, 0x28, 0x10, 0x18, 0x20].into_iter().enumerate() {
                    let delta = (r.next_u32() % 8) * 4;
                    c.bus.write16(DESC + off, delta as u16);
                    c.bus.write32(DESC + off + 4, fakes::FAKE + 0x10 * k as u32);
                }
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = OBJ as u64;
                c.bus.write32(OBJ + 0x28, r.next_u32());
            },
        );
    }

    // ----------------------------------------------------------------------
    // AnimDataBase_AllocChanCache

    /// Several part ids and channel counts; `operator new` faked: it returns
    /// the block at `DATA + 0x4000` (20 nodes of 0x24 bytes plus the ring
    /// links) for every allocation.
    #[test]
    fn anim_data_base_alloc_chan_cache_matches() {
        check(
            ee::s10::f_00310f18__generated,
            anim_data_base_alloc_chan_cache,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0004, 22);
                c.pc = 0x0031_0f18;
                oc417_force(c, FK_NEW_NAMED, DATA + 0x4000);
                c.g[A0][0] = (r.next_u32() % 40) as u64;
                c.g[A1][0] = (r.next_u32() % 12) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // BE_InitCharProfileBoltMap

    /// Profile with or without an allocated lookup, the bolt list (faked
    /// `Bolts_GetCharBolts/Count`: a pointer to 0x38-byte records whose
    /// `+0xe` is set so the default-bolt pass takes its short path), item
    /// counts below and above the profile's capacity, empty lookups.
    #[test]
    fn be_init_char_profile_bolt_map_matches() {
        const PROFILE: u32 = DATA + 0x1000;
        const BOLTS: u32 = DATA + 0x6000;
        const LOOKUP: u32 = DATA + 0x8000;
        check(
            ee::s01::f_00151a88__generated,
            be_init_char_profile_bolt_map,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0005, 6);
                c.pc = 0x0015_1a88;
                let total = [0u32, 1, 12, 30, 40][(seed % 5) as usize];
                c.bus.write32(GAME_GP - 0x1f08, total);
                let count = [0u32, 1, 3, 6, 9][((seed / 5) % 5) as usize];
                oc417_force(c, FK_BOLT_COUNT, count);
                oc417_force(c, FK_BOLTS, BOLTS);
                oc417_force(c, FK_NEW_NAMED, LOOKUP);
                for i in 0..count {
                    let rec = BOLTS + 0x38 * i;
                    c.bus.write16(rec + 4, (r.next_u32() % 40) as u16);
                    c.bus.write16(rec + 0xe, 1);
                }
                c.bus.write32(
                    PROFILE + BOLT_MAP_LOOKUP,
                    if seed % 3 == 0 { 0 } else { LOOKUP },
                );
                c.g[A0][0] = PROFILE as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // InSeq_ChannelStop

    /// A channel with up to three players of every kind (0 delete, 1-3
    /// release, others nothing; the kind virtual is `0x27c958`, which reads
    /// `obj+0x20`, so the fixture chooses the kind), with or without the
    /// entity comm object, playing or idle, 0-3 players.
    #[test]
    fn in_seq_channel_stop_matches() {
        const CHAN: u32 = DATA + 0x1000;
        const COMM: u32 = DATA + 0x1200;
        const HEADER: u32 = DATA + 0x1400;
        const PLAYERS: u32 = DATA + 0x1600;
        const OWNER: u32 = DATA + 0x1800;
        const HEAP: u32 = DATA + 0x1900;
        const COMM_VT: u32 = DATA + 0x3200;
        const PLAYER_VT: u32 = DATA + 0x3000;
        check(ee::s07::f_00273dc0__generated, in_seq_channel_stop, 300, &|c, seed| {
            let mut r = oc417_machine(c, seed, 0x4170_0006, 8);
            c.pc = 0x0027_3dc0;
            c.bus
                .write32(CHAN + 0x28, if seed % 7 == 6 { 0 } else { 1 });
            c.bus
                .write32(CHAN + 0x2c, if seed % 3 == 0 { 0 } else { COMM });
            c.bus.write32(COMM, COMM_VT);
            c.bus.write16(COMM_VT + 0x10, (r.next_u32() % 4 * 4) as u16);
            c.bus.write32(COMM_VT + 0x14, fakes::FAKE + 0x40);
            c.bus.write32(CHAN + 0x1c, HEADER);
            c.bus.write8(HEADER + 2, (seed / 3 % 4) as u8);
            c.bus.write8(HEADER + 3, r.next_u32() as u8);
            c.bus.write32(CHAN + 0x24, PLAYERS);
            c.bus.write32(CHAN + 8, OWNER);
            c.bus.write32(OWNER + 8, HEAP);
            c.bus.write32(HEAP + 8, r.next_u32());
            c.bus.write16(PLAYER_VT + 0x10, 0);
            c.bus.write32(PLAYER_VT + 0x14, fakes::FAKE);
            c.bus.write16(PLAYER_VT + 0x38, 0);
            c.bus.write32(PLAYER_VT + 0x3c, 0x0027_c958);
            c.bus.write16(PLAYER_VT + 0x50, 0);
            c.bus.write32(PLAYER_VT + 0x54, fakes::FAKE + 0x10);
            for i in 0..3u32 {
                let obj = DATA + 0x2000 + 0x100 * i;
                c.bus.write32(PLAYERS + 4 * i, obj);
                c.bus.write32(obj + 4, PLAYER_VT);
                c.bus
                    .write32(obj + 0x20, (seed / 12 + i * 2 + r.next_u32() % 3) % 6);
            }
            c.g[A0][0] = CHAN as u64;
        });
    }

    // ----------------------------------------------------------------------
    // WScriptFn_MultiParticleControl / WScriptFn_AddMagnetModifier

    const WS_WORLD: u32 = DATA + 0x2000;
    const WS_CACHE_PTR: u32 = DATA + 0x2800;
    const WS_CACHE: u32 = DATA + 0x2900;
    const WS_SLOTS: u32 = DATA + 0x2a00;
    const WS_SLOT: u32 = DATA + 0x2b00;
    const WS_ENTRIES: u32 = DATA + 0x2c00;
    const WS_OBJ: u32 = DATA + 0x4000;
    const WS_OBJ2: u32 = DATA + 0x4100;
    const WS_NODE: u32 = DATA + 0x4400;
    const WS_NODE_VT: u32 = DATA + 0x4600;
    const WS_MGR: u32 = DATA + 0x5000;
    const WS_ARGS: u32 = DATA + 0x1000;
    const WS_RESULT: u32 = DATA + 0x1800;

    /// The world, its object cache and two objects ids can name, written
    /// the way both functions look them up: `-1` is the current object
    /// (`world+0x290`), `slot | index << 8` goes through the cache's slot
    /// table.
    fn oc417_wscript_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(GAME_GP + 0xce8, WS_WORLD);
        c.bus
            .write32(WS_WORLD + 0x290, if seed % 5 == 0 { 0 } else { WS_OBJ2 });
        c.bus.write32(GAME_GP + 0x16c8, WS_CACHE_PTR);
        c.bus.write32(WS_CACHE_PTR, WS_CACHE);
        c.bus.write32(WS_CACHE + 8, WS_SLOTS);
        c.bus.write32(WS_SLOTS, WS_SLOT);
        c.bus.write32(WS_SLOTS + 5 * 4, WS_SLOT);
        c.bus.write32(WS_SLOT + 0x1c, WS_ENTRIES);
        c.bus.write32(WS_ENTRIES, (WS_OBJ >> 2) << 8);
        c.bus.write32(
            WS_ENTRIES + 3 * 4,
            if seed % 4 == 0 { 0 } else { (WS_OBJ >> 2) << 8 },
        );
        c.bus.write32(WS_OBJ + 0x78, r.next_u32());
        c.bus.write32(WS_OBJ2 + 0x78, r.next_u32());
        c.bus.write32(GAME_GP + 0xf38, WS_MGR);
        for i in 0..4u32 {
            c.bus.write32(WS_MGR + 4 * i, WS_MGR + 0x400);
        }
    }

    /// The 12-byte default result record both functions copy out.
    fn oc417_wscript_result(c: &mut Cpu, r: &mut Rng) {
        for i in 0..3u32 {
            c.bus.write32(0x004c_9098 + 4 * i, r.next_u32());
        }
    }

    /// `n` argument entries (`{index, value, ?, type}`): index 0-3 of the
    /// 4-word record, the id word taken from the ids above.
    fn oc417_wscript_args(c: &mut Cpu, r: &mut Rng, seed: u32, id_index: u32, types: u32) {
        let n = (seed / 2) % 4;
        for i in 0..n {
            let e = WS_ARGS + 0x10 * i;
            let idx = r.next_u32() % 4;
            c.bus.write32(e, idx);
            let value = if idx == id_index {
                [0xffff_ffffu32, 0x0000, 0x0305, 0x0107, 0x0900][(r.next_u32() % 5) as usize]
            } else if idx == 0 {
                r.next_u32() % 4
            } else {
                r.next_u32() % 3
            };
            c.bus.write32(e + 4, value);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            c.bus.write32(types + 4 * i, r.next_u32() % 3);
        }
    }

    /// Both initialisation states of the static record, 0-3 entries (int to
    /// float conversions included), objects found through the current object,
    /// the slot table or not at all, add and remove.
    #[test]
    fn wscript_fn_multi_particle_control_matches() {
        check(
            ee::s10::f_00306cb0__generated,
            wscript_fn_multi_particle_control,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4170_0007, 0);
                c.pc = 0x0030_6cb0;
                oc417_wscript_world(c, &mut r, seed);
                oc417_wscript_result(c, &mut r);
                oc417_wscript_args(c, &mut r, seed, 1, 0x0044_6970);
                let initialised = seed % 3 != 0;
                c.bus.write32(GAME_GP + 0x27c8, initialised as u32);
                if initialised {
                    for i in 0..4u32 {
                        let w = if i == 1 {
                            0xffff_ffff
                        } else {
                            r.next_u32() % 3
                        };
                        c.bus.write32(0x004f_c050 + 4 * i, w);
                    }
                }
                c.g[A0][0] = WS_RESULT as u64;
                c.g[A1][0] = ((seed / 2) % 4) as u64;
                c.g[A2][0] = WS_ARGS as u64;
            },
        );
    }

    /// As above for the magnet modifier: the object's move node, its
    /// "accepts magnets" virtual (`0x27c070` reads the node, so the fixture
    /// chooses the answer) and the faked magnet call.
    #[test]
    fn wscript_fn_add_magnet_modifier_matches() {
        check(
            ee::s10::f_00305478__generated,
            wscript_fn_add_magnet_modifier,
            300,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0008, 4);
                c.pc = 0x0030_5478;
                oc417_wscript_world(c, &mut r, seed);
                oc417_wscript_result(c, &mut r);
                oc417_wscript_args(c, &mut r, seed, 0, 0x0044_66a0);
                for obj in [WS_OBJ, WS_OBJ2] {
                    c.bus
                        .write32(obj + 0xc, if seed % 7 == 0 { 0 } else { WS_NODE });
                }
                c.bus.write32(WS_NODE + 0xc, WS_NODE_VT);
                c.bus.write32(WS_NODE + 0x14, (seed / 3) % 2);
                c.bus.write16(WS_NODE_VT + 0x80, 0);
                c.bus.write32(WS_NODE_VT + 0x84, 0x0027_c070);
                c.bus
                    .write16(WS_NODE_VT + 0xc0, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(WS_NODE_VT + 0xc4, fakes::FAKE);
                c.bus.write32(GAME_GP + 0x2750, (seed % 3 != 0) as u32);
                if seed % 3 != 0 {
                    for i in 0..4u32 {
                        let w = if i == 0 {
                            0xffff_ffff
                        } else {
                            r.next_u32() % 5
                        };
                        c.bus.write32(0x004f_bd58 + 4 * i, w);
                    }
                }
                c.g[A0][0] = WS_RESULT as u64;
                c.g[A1][0] = ((seed / 2) % 4) as u64;
                c.g[A2][0] = WS_ARGS as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // SectionMan_RequestUnload

    /// Every section state 0-9 (the jump table `0x484c00` is written with
    /// the three code labels, and, for one seed in 40, a wild address that
    /// leaves through the `jump_out` hook), negative and non-negative slots,
    /// the faked queue call.
    #[test]
    fn section_man_request_unload_matches() {
        check(
            ee::s05::f_0022d598__generated,
            section_man_request_unload,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0009, 1);
                c.pc = 0x0022_d598;
                let labels = [0x0022_d5f0u32, 0x0022_d608, 0x0022_d620];
                for state in 0..9u32 {
                    let label = match state {
                        3 => labels[0],
                        1 | 2 | 6 => labels[1],
                        _ => labels[2],
                    };
                    let label = if seed % 40 == 39 && state == 4 {
                        0x0022_d640
                    } else {
                        label
                    };
                    c.bus.write32(0x0048_4c00 + 4 * state, label);
                }
                let idx = seed % 4;
                let section = REQUEST_UNLOAD_SECTION_MAP + 0x10 * idx;
                let slot = if (seed / 4) % 5 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() % 3
                };
                c.bus.write32(section + 4, slot);
                c.bus.write32(section + 8, (seed / 4 + idx) % 10);
                c.g[A0][0] = (r.next_u32() % 16) as u64;
                c.g[A1][0] = idx as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // ScriptEngine_GetTargetPose

    /// Target ids across the jump table (the ELF image provides the table
    /// and its constants; every other state is zero, so the real script
    /// callees find nothing and take their short paths): rider targets
    /// (1-18, 40-60), locator kinds 0-11, the object categories of 29/30,
    /// the static offset table of 20-22 and ids outside the table. The
    /// ids that drop the pose onto the ground (19, 23-28, 31, 34-36) call
    /// the collision probe `0x3369d8` for real and are left out of the
    /// fixture.
    #[test]
    fn script_engine_get_target_pose_matches() {
        const SELF: u32 = DATA + 0x1000;
        const OBJ: u32 = DATA + 0x1400;
        const INFO: u32 = DATA + 0x1500;
        const POS: u32 = DATA + 0x1800;
        const OUTS: u32 = DATA + 0x1900;
        const IDS: [u32; 38] = [
            1, 5, 18, 40, 44, 60, 29, 30, 32, 33, 37, 38, 39, 24, 20, 21, 22, 2, 9, 12, 41, 50, 55,
            59, 29, 30, 20, 22, 0, 61, 62, 100, 3, 4, 6, 7, 8, 10,
        ];
        check(
            ee::s07::f_0027a0d8__generated,
            script_engine_get_target_pose,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = oc417_machine(c, seed, 0x4170_000a, 6);
                c.pc = 0x0027_a0d8;
                c.bus.write32(OBJ + 0x14, INFO);
                c.bus.write8(INFO, (seed % 3 == 0) as u8);
                c.bus.write32(GAME_GP + 0x20e8, (seed % 4 == 1) as u32);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = IDS[(seed % 38) as usize] as u64;
                c.g[A2][0] = OBJ as u64;
                c.g[A3][0] = POS as u64;
                c.g[T0][0] = OUTS as u64;
                c.g[T1][0] = (OUTS + 4) as u64;
                c.g[T2][0] = (OUTS + 8) as u64;
                for i in 0..8u32 {
                    c.bus.write32(POS + 4 * i, r.next_u32());
                }
            },
        );
    }

    // ----------------------------------------------------------------------
    // Rider_AttachToScript

    /// A rider whose script actor record asks for the equipment pass (or
    /// not), bolts with and without the `0x40` flag and the item flag
    /// `0x10`, every callee but the three script helpers faked (the script
    /// object start `0x27f9f8` and the actor frame `0x27c9b0` run for real
    /// on mostly zero data).
    #[test]
    fn rider_attach_to_script_matches() {
        const RIDER: u32 = DATA + 0x1000;
        const ACTOR: u32 = DATA + 0x3400;
        const REC: u32 = DATA + 0x3500;
        const DB: u32 = DATA + 0x3800;
        const LOOKUP: u32 = DATA + 0x3900;
        const BOLTS: u32 = DATA + 0x3a00;
        check(
            ee::s00::f_00123640__generated,
            rider_attach_to_script,
            100,
            &|c, seed| {
                elf_image(c);
                let mut r = oc417_machine(c, seed, 0x4170_000b, 40);
                c.pc = 0x0012_3640;
                c.g[A0][0] = RIDER as u64;
                c.bus.write32(RIDER + 0x784, DATA + 0x3000);
                c.bus.write32(RIDER + 0x780, DATA + 0x3100);
                c.bus.write32(RIDER + 0x77c, DATA + 0x3200);
                c.bus.write32(RIDER + 0x320, r.next_u32() % 2);
                c.bus.write32(RIDER + 0x6d8, ACTOR);
                c.bus.write32(RIDER + 0x86c, r.next_u32() % 4);
                for off in (0x700..0x750).step_by(4) {
                    let v = r.f32(4.0);
                    c.bus.write32(RIDER + off, v);
                }
                c.bus.write32(ACTOR + 0x20, REC);
                c.bus.write8(REC + 4, (seed % 2) as u8);
                c.bus.write8(REC + 6, (seed % 3 != 0) as u8);
                c.bus.write8(REC + 7, (seed % 5 < 2) as u8);
                // The equipment pass.
                let count = (seed / 3) % 5;
                oc417_force(c, FK_BE_TABLE, DATA + 0x3700);
                oc417_force(c, FK_BE_DB_ENTRY, DB);
                oc417_force(c, FK_GET_CHARACTER, r.next_u32() % 6);
                oc417_force(c, FK_BOLT_COUNT, count);
                oc417_force(c, FK_BOLTS, BOLTS);
                c.bus.write32(DB + 0x288, LOOKUP);
                for k in 0..4u32 {
                    let item = if (seed + k) % 5 == 0 { 0xffff } else { k % 3 };
                    c.bus.write16(LOOKUP + 2 * k, item as u16);
                    let flags = if r.next_u32() % 2 == 0 { 0x10 } else { 0 };
                    c.bus.write16(DB + 0x290 + 4 * k + 2, flags);
                }
                for i in 0..count {
                    let e = BOLTS + 0x38 * i;
                    let id = [0xffu32, 3, 5, 9, 0x80][((seed + i) % 5) as usize];
                    c.bus.write8(e + 0x10, id as u8);
                    c.bus.write16(e + 4, (r.next_u32() % 4) as u16);
                    c.bus
                        .write32(e + 0x34, if r.next_u32() % 3 == 0 { 0 } else { 0x40 });
                }
            },
        );
    }

    // ----------------------------------------------------------------------
    // cLunoTable_Set

    /// A one to four bucket table with chains: keys of every type (0 is
    /// ignored), a missing value type (delete) or a value of each type,
    /// matching and non-matching chain keys, an empty or occupied bucket and
    /// the append of a new node (allocation, table-reference assignment and
    /// the node's destructor are faked).
    #[test]
    fn c_luno_table_set_matches() {
        const SELF: u32 = DATA + 0x1000;
        const TABLE: u32 = DATA + 0x1100;
        const NODES: u32 = DATA + 0x1200;
        const CHAIN: u32 = DATA + 0x1600;
        const KEY: u32 = DATA + 0x1040;
        const VAL: u32 = DATA + 0x1080;
        const NODE_VT: u32 = DATA + 0x1800;
        const NEW_NODE: u32 = DATA + 0x1a00;
        check(ee::s05::f_00225338__generated, c_luno_table_set, 300, &|c, seed| {
            let mut r = oc417_machine(c, seed, 0x4170_000c, 6);
            c.pc = 0x0022_5338;
            oc417_force(c, FK_MEM_ALLOC, NEW_NODE);
            let modulus = 1 + seed % 4;
            c.bus.write32(SELF, TABLE);
            c.bus.write32(TABLE + 0xc, modulus);
            c.bus.write32(TABLE + 0x10, NODES);
            c.bus.write16(NODE_VT + 8, (r.next_u32() % 4 * 4) as u16);
            c.bus.write32(NODE_VT + 0xc, fakes::FAKE);
            let key_type = if seed % 13 == 0 {
                0
            } else {
                1 + (seed / 2) % 5
            };
            let key_word = r.next_u32() % 16;
            c.bus.write32(KEY, key_word);
            c.bus.write32(KEY + 4, r.next_u32() % 4);
            c.bus.write32(KEY + 8, key_type);
            let val_type = (seed / 7) % 6;
            c.bus.write32(VAL, r.next_u32());
            c.bus.write32(VAL + 4, r.next_u32());
            c.bus.write32(VAL + 8, val_type);
            let bucket = NODES + 0x20 * (key_word % modulus);
            let occupied = (seed / 3) % 3 != 0;
            let same_key = (seed / 5) % 2 == 0;
            // Node layout: key (+0, type at +8), value (+0xc, type at +0x14),
            // next (+0x18), vtable (+0x1c).
            let put_node =
                |c: &mut Cpu, at: u32, key_same: bool, vtype: u32, next: u32, r: &mut Rng| {
                    let word = if key_same {
                        key_word
                    } else {
                        key_word.wrapping_add(1 + r.next_u32() % 3)
                    };
                    c.bus.write32(at, word);
                    let key_hi = c.bus.read32(KEY + 4);
                    c.bus.write32(at + 4, key_hi);
                    c.bus.write32(at + 8, key_type);
                    c.bus.write32(at + 0xc, r.next_u32());
                    c.bus.write32(at + 0x10, r.next_u32());
                    c.bus.write32(at + 0x14, vtype);
                    c.bus.write32(at + 0x18, next);
                    c.bus.write32(at + 0x1c, NODE_VT);
                };
            let chain_len = (seed / 11) % 3;
            let first_chain = if chain_len == 0 { 0 } else { CHAIN };
            let bucket_type = if occupied { 1 + r.next_u32() % 5 } else { 0 };
            put_node(c, bucket, same_key, bucket_type, first_chain, &mut r);
            for k in 0..chain_len {
                let at = CHAIN + 0x20 * k;
                let next = if k + 1 < chain_len { at + 0x20 } else { 0 };
                let chain_same = (seed / 17 + k) % 2 == 0;
                put_node(c, at, chain_same, 1 + r.next_u32() % 5, next, &mut r);
            }
            c.g[A0][0] = SELF as u64;
            c.g[A1][0] = KEY as u64;
            c.g[A2][0] = VAL as u64;
        });
    }
}
