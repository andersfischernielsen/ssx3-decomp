//! Staging module oc324 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc324`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `MEM_copy` (EA_MemCopy, 0x3e6574).
pub(crate) const OC324_MEM_COPY: u32 = 0x003e_6574;

/// `LunoVMRegister_AsInt` (0x226610): returns its argument.
pub(crate) const OC324_LUNO_REGISTER_AS_INT: u32 = 0x0022_6610;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_61b0,
        wscript_fn_get_progress_flag_bit,
        "WScriptFn_GetProgressFlagBit",
    ),
    (
        0x0030_12f0,
        wscript_fn_mission_command,
        "WScriptFn_MissionCommand",
    ),
    (0x002d_1bf0, world_add_focus_object, "World_AddFocusObject"),
    (
        0x002a_a210,
        stream_buffer_get_current_request,
        "StreamBuffer_GetCurrentRequest",
    ),
    (
        0x003d_7a50,
        spch_select_request_for_channel,
        "SPCH_SelectRequestForChannel",
    ),
    (
        0x0035_54b0,
        mod_obj_set_transform_modifier,
        "ModObj_SetTransformModifier",
    ),
    (
        0x0029_f378,
        ssx_audio_system_get_grunt_voice,
        "SSXAudioSystem_GetGruntVoice",
    ),
    (
        0x0011_7990,
        rider_metrix_get_trick_score,
        "RiderMetrix_GetTrickScore",
    ),
    (
        0x002f_cff0,
        world_obj_factory_mesh_anim,
        "WorldObjFactory_MeshAnim",
    ),
];

/// The career-flag block a WScript BIND call unpacks from (`gp + 0x2790`: a
/// word, then the bind tag byte, then the "initialised" flag).
pub(crate) const OC324_CAREER_FLAGS: u32 = 0x2790;

/// The "initialised" word of that block (`gp + 0x2798`).
pub(crate) const OC324_CAREER_FLAGS_INIT: u32 = 0x2798;

/// The mission-command call's static argument description (`D_004FBEE0`).
#[cfg(test)]
pub(crate) const OC324_MISSION_DESC: u32 = 0x004f_bee0;

/// The "no change" result a script call copies into its slot
/// (`D_004C9098`, 12 bytes).
pub(crate) const OC324_NO_CHANGE: u32 = 0x004c_9098;

// ---------------------------------------------------------------------------
// 0x003061b0 WScriptFn_GetProgressFlagBit

/// `int* WScriptFn_GetProgressFlagBit(int *ret, int n, Ent *e)` (0x3061b0): a
/// Luno script call (the binder `BIND_UNPACK_ARGS` of the career flag block)
/// that answers one progress-flag bit. The first unpacked value picks the bit
/// of `D_005308d0` (0: bit 6, 1: bit 8, 2: bit 7) and the flag is its
/// complement (1 when the bit is clear); any other value answers 1. The
/// result goes to the slot `LunoVMRegister_AsInt(ret)` returns.
pub fn wscript_fn_get_progress_flag_bit(c: &mut Cpu) {
    /// The career flags word the selected bit is read from.
    const FLAGS: u32 = 0x0053_08d0;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC324_CAREER_FLAGS_INIT)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !initialised {
            // Guarded static-local init: zero the word, keep the tag byte.
            c.cycles += 5;
            c.g[V1][0] = Addr(gp.wrapping_add(OC324_CAREER_FLAGS + 4)).lbu(c);
            c.g[V0][0] = 1;
            Addr(gp.wrapping_add(OC324_CAREER_FLAGS_INIT)).sw(c, c.g[V0][0]);
            Addr(gp.wrapping_add(OC324_CAREER_FLAGS)).sw(c, 0);
            c.sb(frame.at(c, 0), c.g[V1][0]);
        }
        // Copy the 8-byte flag block into a stack buffer, then expand the
        // argument list into it.
        c.cycles += 7;
        c.g[S0][0] = sx(frame.at(c, 0x10));
        c.g[A2][0] = 8;
        c.g[A1][0] = sx(gp.wrapping_add(OC324_CAREER_FLAGS));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, OC324_MEM_COPY, 0x0030_6204)?;
        let count = c.g[S1][0] as i64;
        c.g[A2][0] = c.g[S2][0];
        if count > 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(gp.wrapping_add(0xcb8));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                // A flag whose type is 2 is stored as a float, everything
                // else as the raw word.
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_6218);
                }
            }
        }
        // The selected flag bit: the first unpacked value picks the bit.
        c.cycles += 4;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = 1;
        let key = c.g[A0][0];
        c.g[V1][0] = 0x0053_0000;
        if key == 1 {
            c.cycles += 4;
            c.g[S0][0] = 0;
            c.g[V0][0] = Addr(FLAGS).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
            c.cycles += 2;
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[A1][0];
            }
        } else if (key as i64) < 2 {
            c.cycles += 3;
            if key == 0 {
                c.cycles += 2;
                c.g[S0][0] = 0;
                c.cycles += 3;
                c.g[V0][0] = Addr(FLAGS).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.cycles += 2;
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A1][0];
                }
            } else {
                // A negative value: no flag, answer 1. (The value test's
                // arm, then the switch's default arm.)
                c.cycles += 4;
                c.g[S0][0] = 1;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if key == 2 {
                c.cycles += 3;
                c.g[S0][0] = 0;
                c.g[V0][0] = Addr(FLAGS).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                c.cycles += 2;
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A1][0];
                }
            } else {
                c.cycles += 2;
                c.g[S0][0] = 1;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(8), c.g[V0][0]);
        call_ptr(c, OC324_LUNO_REGISTER_AS_INT, 0x0030_62dc)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.g[V0][0] = c.g[S3][0];
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
// 0x003012f0 WScriptFn_MissionCommand

/// `int* WScriptFn_MissionCommand(int *ret, int n, Ent *e)` (0x3012f0): the
/// Luno script call that issues a mission command. It expands the argument
/// block over the static description `D_004FBBE0` (three words, all -1) and,
/// unless the mission id is -1, runs `WScriptMan_MissionCommand` (forced, -1)
/// and returns its result; for id -1 it copies the "no change" result
/// `D_004C9098` (12 bytes) into `ret`.
pub fn wscript_fn_mission_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(0x25c8)).lw(c);
        frame.save(c, S3, 0x30);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialised {
            // Guarded static-local init: both words -1, tag byte kept.
            c.cycles += 11;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4420));
            c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(8).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4420), c.g[V0][0]);
            c.g[A0][0] = 0xffff_ffff_ffff_ffffu64;
            c.sb((c.g[V1][0] as u32).wrapping_add(8), c.g[A1][0]);
            c.g[V0][0] = 1;
            c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[A0][0]);
            c.sw(gp.wrapping_add(0x25c8), c.g[V0][0]);
            c.sb(frame.at(c, 0), c.g[A1][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 7;
        c.g[S0][0] = sx(frame.at(c, 0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4420));
        c.g[A2][0] = 0xc;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, OC324_MEM_COPY, 0x0030_1360)?;
        let count = c.g[S2][0] as i64;
        c.g[A2][0] = c.g[S3][0];
        if count > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx(gp.wrapping_add(0xb80));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1378);
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] |= 0xffff;
        let no_command = c.g[A1][0] == c.g[V0][0];
        c.g[A3][0] = 0xffff_ffff_ffff_ffffu64;
        if !no_command {
            // Issue the mission command; the script manager is the callee's
            // `this`, the mission id and value the first two words.
            c.cycles += 10;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[A0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            Callee {
                addr: 0x0030_a868,
                f: ee::s10::WScriptMan_MissionCommand,
            }
            .call(c, 0x0030_13e0)?;
            c.g[V1][0] = 1;
            c.g[S0][0] = c.g[V0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[V1][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, OC324_LUNO_REGISTER_AS_INT, 0x0030_13f4)?;
            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        } else {
            // No command: copy the "no change" result into the slot.
            c.cycles += 8;
            c.g[V0][0] = 0x004d_0000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
            c.ldl(V1, (c.g[A2][0] as u32).wrapping_add(7));
            c.ldr(V1, c.g[A2][0] as u32);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.sdl(V1, (c.g[S1][0] as u32).wrapping_add(7));
            c.sdr(V1, c.g[S1][0] as u32);
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[A0][0]);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
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
// 0x002d1bf0 World_AddFocusObject

/// `void World_AddFocusObject(FocusObject *obj)` (0x2d1bf0): add `obj` to the
/// focus manager hanging off the world (`world->+0x84 -> +0xc -> +0xa4`).
pub fn world_add_focus_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xa4).lw(c);
        Callee {
            addr: 0x0010_32c0,
            f: ee::s00::FocusMan_AddObject,
        }
        .call(c, 0x002d_1c10)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002aa210 StreamBuffer_GetCurrentRequest

/// `int StreamBuffer_GetCurrentRequest(StreamBuffer *self)` (0x2aa210): ask
/// the SND layer for the buffer's status; -1 when it has no request, else
/// the request's index (the second status word).
pub fn stream_buffer_get_current_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x002a_a020,
            f: ee::s08::StreamBuffer_GetStatus,
        }
        .call(c, 0x002a_a220)?;
        c.g[V1][0] = Addr(c.g[SP][0] as u32).lw(c);
        let no_request = c.g[V1][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffffu64;
        if !no_request {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[SP][0] as u32).off(4).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003d7a50 SPCH_SelectRequestForChannel

/// `int SPCH_SelectRequestForChannel(int channel)` (0x3d7a50): find the
/// event bound to `channel`, decode its request list (when the event's flag
/// byte has 0x10 set, the list is `{count, ids}` at an offset computed from
/// the event's counters), select a request from it and retry with no list
/// when the selection fails.
pub fn spch_select_request_for_channel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x660));
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx((c.g[S1][0] as u32) << 3);
        frame.save(c, RA, 0x30);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = 0;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lhu(c);
        c.g[V0][0] = 0xffff;
        Addr(frame.at(c, 0)).sw(c, 0);
        let unbound = c.g[V1][0] == c.g[V0][0];
        Addr(frame.at(c, 4)).sw(c, 0);
        if !unbound {
            // The bound-entry block itself (the two stores' delay slot, the
            // `li` pair and the call).
            c.cycles += 5;
            Callee {
                addr: 0x003d_7110,
                f: ee::s14::SPCH_FindEvent,
            }
            .call(c, 0x003d_7a94)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] != 0 {
                // The event: flag byte +0xa selects the request list.
                c.cycles += 5;
                c.g[T1][0] = Addr(c.g[A2][0] as u32).off(0xa).lbu(c);
                c.g[V0][0] = sx((c.g[T1][0] as u32) >> 4);
                c.g[V0][0] &= 1;
                let no_list = c.g[V0][0] == 0;
                c.g[A1][0] = 0;
                if !no_list {
                    c.cycles += 27;
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(7).lbu(c);
                    c.g[A3][0] = 0xffff_ffff_ffff_0000u64;
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(6).lbu(c);
                    c.g[A3][0] |= 0xfffc;
                    c.g[A1][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[V1][0] &= 0xf;
                    c.g[T0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 28) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(3));
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[A0][0] &= c.g[A3][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = 0xffff_ffff_ffff_fffcu64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(3));
                    c.g[V0][0] &= c.g[A1][0];
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(3));
                    c.g[V1][0] &= c.g[A1][0];
                    c.g[V0][0] = sx((c.g[T1][0] as u32) >> 3);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] &= 1;
                    let no_stride = c.g[V0][0] == 0;
                    c.g[A1][0] = 1;
                    if !no_stride {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(7));
                        c.g[V0][0] &= c.g[A3][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    // The list: a count byte then the request ids.
                    c.cycles += 4;
                    c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(2));
                    Addr(frame.at(c, 4)).sw(c, c.g[V0][0]);
                    Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
                }
                c.cycles += 1;
                c.g[S0][0] = c.g[A1][0];
            }
        }
        // First selection: with the list when one was decoded, else none.
        c.cycles += 2;
        let no_list = c.g[S0][0] == 0;
        c.g[A0][0] = sx(sp(c));
        if no_list {
            c.cycles += 3;
            c.g[A0][0] = 0;
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x003d_7760,
                f: ee::s14::SPCH_SelectRequest,
            }
            .call(c, 0x003d_7b64)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            Callee {
                addr: 0x003d_7760,
                f: ee::s14::SPCH_SelectRequest,
            }
            .call(c, 0x003d_7b50)?;
        }
        // A failed selection over the list retries with none.
        c.cycles += 2;
        let listed = c.g[S0][0] != 0;
        frame.restore(c, RA, 0x30);
        if listed {
            // The list path charges 2 more before looking at the selection's
            // result, whichever way it goes.
            c.cycles += 2;
            if (c.g[V0][0] as i64) < 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x003d_7760,
                    f: ee::s14::SPCH_SelectRequest,
                }
                .call(c, 0x003d_7b80)?;
                frame.restore(c, RA, 0x30);
                c.cycles += 1;
                frame.restore(c, S1, 0x20);
            } else {
                frame.restore(c, S1, 0x20);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S1, 0x20);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003554b0 ModObj_SetTransformModifier

/// `void cMoveNode::addTransformModifier(cMoveNode *self, cTransformModifier *mod)`
/// (0x3554b0): notify the vtable's slot 49 (prepare), create or reset the
/// node's modifier block, store `mod` at its head, notify slot 48 (refresh)
/// and flag the node's object as transformed (clear 0x20, set 0x40).
pub fn mod_obj_set_transform_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        // vtable[49](self)
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x188).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x18c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_54dc)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
        let has_block = c.g[A0][0] != 0;
        if has_block {
            c.cycles += 3;
            Callee {
                addr: 0x0035_30d0,
                f: ee::s11::ModifierBlock_FreeTransform,
            }
            .call(c, 0x0035_5500)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0035_5468,
                f: ee::s11::ModObj_CreateModifierBlock,
            }
            .call(c, 0x0035_54f0)?;
        }
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
        // Store the modifier at the block's head and refresh the node.
        c.cycles += 18;
        c.sw(c.g[V0][0] as u32, c.g[S1][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x180).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x184).lw(c);
        let refresh = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, refresh, 0x0035_551c)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_0000u64;
        c.g[V1][0] |= 0xffdf;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] &= c.g[V1][0];
        frame.restore_ra(c, 0);
        c.g[V0][0] |= 0x40;
        c.sw((c.g[A0][0] as u32).wrapping_add(8), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0029f378 SSXAudioSystem_GetGruntVoice

/// `int SSXAudioSystem_GetGruntVoice(SSXAudioSystem *self, Rider *rider)`
/// (0x29f378): the "grunt" voice id for the rider's current character: the
/// character lookup for kinds 0, else 6 for kinds 14, 16 and 21, and 7 for
/// everything else.
pub fn ssx_audio_system_get_grunt_voice(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        // Rider_GetPlayerCharacterInfo(rider) -> character id.
        call_ptr(c, 0x0012_3168, 0x0029_f390)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0xe;
        let is14 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 15) as u64;
        if !is14 {
            c.cycles += 2;
            let ge15 = c.g[V0][0] == 0;
            c.g[V0][0] = 0x10;
            if ge15 {
                c.cycles += 2;
                let is16 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x15;
                if !is16 {
                    c.cycles += 2;
                    let is21 = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if !is21 {
                        // Any other kind: 7.
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        c.cycles += 3;
                        frame.restore_ra(c, 0);
                        frame.close(c);
                        return Ok(());
                    }
                }
            } else {
                c.cycles += 2;
                let is0 = c.g[V1][0] == 0;
                c.g[V0][0] = 0x7;
                if is0 {
                    // Kind 0: the rider's character voice.
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0012_3128, 0x0029_f3d8)?;
                    frame.restore(c, S0, 0x10);
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        // Kinds 14, 16 and 21: 6.
        c.cycles += 2;
        c.g[V0][0] = 6;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00117990 RiderMetrix_GetTrickScore

/// `int RiderMetrix_GetTrickScore(RiderMetrix *self)` (0x117990): the
/// accumulated score at 0.01 points a unit, rounded to whole tens:
/// `(int)(v * 10000 + 5)` floored to a multiple of 10.
pub fn rider_metrix_get_trick_score(c: &mut Cpu) {
    c.cycles += 5;
    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[A1][0] = 0xa;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x1c4));
    // (A branch on the multiplier that goes both ways in the original.)
    c.cycles += 14;
    c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(0x14));
    c.mul_s(0, 0, 1);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x7b04));
    c.g[AT][0] = 0x40a0_0000;
    c.mtc1(AT, 3);
    c.mul_s(0, 0, 2);
    c.mul_s(0, 0, 4);
    c.add_s(0, 0, 3);
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    c.div(0, V0, A1);
    c.g[V1][0] = c.hi[0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    ret(c);
}

// ---------------------------------------------------------------------------
// 0x002fcff0 WorldObjFactory_MeshAnim

/// The mesh-animation modifier's static description (`D_004FB5E8`, 0x40
/// bytes: id, model handle, speeds, sizes, count, colour).
#[cfg(test)]
pub(crate) const OC324_MESH_ANIM_DESC: u32 = 0x004f_b5e8;

/// The mesh-animation modifier's init flag (`D_004A55FC`).
pub(crate) const OC324_MESH_ANIM_INIT: u32 = 0x250c;

/// `void* WorldObjFactory_MeshAnim(void *ret, int n, Ent *e)` (0x2fcff0): the
/// Luno call that attaches a mesh-animation modifier to the object named by
/// the argument block: expand the block over the static description, resolve
/// the object (id -1: the world's current object), skip when the object
/// already runs one (the factory's type-11 modifier is active), else
/// allocate 0x6c0 bytes and build the anim onto it. `ret` receives the
/// "no change" result (`D_004C9098`, 12 bytes).
pub fn world_obj_factory_mesh_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xb0);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC324_MESH_ANIM_INIT)).lw(c);
        frame.save(c, S0, 0xa0);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A1][0]; // n
        frame.save(c, S4, 0x60);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S1, 0x90);
        c.g[S4][0] = c.g[A2][0]; // e
        frame.save(c, S3, 0x70);
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x50);
        if !initialised {
            // Guarded static-local init of the 0x40-byte description.
            c.cycles += 38;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a18));
            c.g[V1][0] = 0xffff_ffff_ffff_0000u64;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x3c).lbu(c);
            c.g[V1][0] |= 0xffff;
            c.mtc1(0, 0);
            c.g[A2][0] = 0xffff_ffff_ffff_ffffu64;
            c.g[AT][0] = 0x4416_0000;
            c.mtc1(AT, 7);
            c.g[A1][0] = 1;
            c.lwc1(1, gp.wrapping_sub(0x338c));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.g[AT][0] = 0x4000_0000;
            c.mtc1(AT, 3);
            c.g[AT][0] = 0x4396_0000;
            c.mtc1(AT, 4);
            c.g[AT][0] = 0x43b4_0000;
            c.mtc1(AT, 5);
            c.g[AT][0] = 0xffff_ffff_bf80_0000u64;
            c.mtc1(AT, 6);
            c.sw((c.g[A3][0] as u32).wrapping_sub(0x4a18), c.g[V1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[A0][0]);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(8));
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x18));
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x1c));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x28));
            c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x2c));
            c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[A2][0]);
            c.sw(gp.wrapping_add(OC324_MESH_ANIM_INIT), c.g[A1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
            c.sb(frame.at(c, 0), c.g[A0][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 12;
        c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x40;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a18));
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw(frame.at(c, 0x10), c.g[V0][0]);
        call_ptr(c, OC324_MEM_COPY, 0x002f_d0e0)?;
        // `lui s3, 0x4d`: the page base `D_004C9098` is built off (kept for
        // the register state a callee's call log records).
        c.g[S3][0] = 0x004d_0000;
        let count = c.g[S0][0] as i64;
        c.g[A2][0] = c.g[S4][0];
        if count > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6060));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_d100);
                }
            }
        }
        // Resolve the object the block names (-1: the current one).
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let named = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if named {
            // The packed id names an instance table entry.
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                // conv(): a table index of 0 means "no object".
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x290).lw(c);
        }
        // No object to attach to: the "no change" result only. An object
        // whose type-11 modifier node is already active: the same. Both
        // return before the allocation.
        c.cycles += 2;
        let no_object = c.g[S0][0] == 0;
        let mut build_anim = false;
        if !no_object {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
            let no_node = c.g[A0][0] == 0;
            c.g[A1][0] = 0xb;
            if no_node {
                build_anim = true;
            } else {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                Callee {
                    addr: 0x002f_ae38,
                    f: ee::s10::WorldObjFactory_AttachModifier,
                }
                .call(c, 0x002f_d1d0)?;
                // A node that answers "active" skips the allocation.
                build_anim = c.g[V0][0] == 0;
            }
        }
        if build_anim {
            c.cycles += 14;
            c.g[A1][0] = 0x0049_0000;
            c.g[A0][0] = 0x6c0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68f8));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            Callee {
                addr: 0x0031_7d70,
                f: ee::s10::EA_MemAllocNamed,
            }
            .call(c, 0x002f_d1f0)?;
            c.g[V1][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = c.g[S0][0];
            c.g[T0][0] = Addr(c.g[V1][0] as u32).off(0x294).lw(c);
            c.g[A3][0] = c.g[S1][0];
            c.g[T1][0] = 0;
            call_ptr(c, 0x0035_1b40, 0x002f_d210)?;
        }
        // The result slot: the "no change" description D_004C9098
        // (0x4d0000 - 0x6f68).
        c.cycles += 16;
        c.g[A1][0] = sx(OC324_NO_CHANGE);
        c.ldl(V0, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(V0, c.g[A1][0] as u32);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        c.sdl(V0, (c.g[S2][0] as u32).wrapping_add(7));
        c.sdr(V0, c.g[S2][0] as u32);
        c.sw((c.g[S2][0] as u32).wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// The career flag block (initialised or not, and a known or unknown
    /// first value), the per-flag types, the flags word and an argument list
    /// of -1..3 entries; both store forms (raw word and float).
    #[test]
    fn wscript_fn_get_progress_flag_bit_matches() {
        use crate::difftest::fakes::machine;
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        check(
            ee::s10::f_003061b0__generated,
            wscript_fn_get_progress_flag_bit,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_61b0, 0);
                c.pc = 0x0030_61b0;
                let gp = GAME_GP;
                // An uninitialised block is zeroed by the function itself.
                c.bus.write32(
                    gp + OC324_CAREER_FLAGS_INIT,
                    if seed % 3 == 0 { 0 } else { 1 },
                );
                let key = match seed % 6 {
                    0 => 0u32,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0xffff_ffff,
                    _ => r.next_u32() % 4,
                };
                c.bus.write32(gp + OC324_CAREER_FLAGS, key);
                c.bus.write32(gp + OC324_CAREER_FLAGS + 4, r.next_u32());
                // The career flags word the bit is read from.
                c.bus.write32(0x0053_08d0, r.next_u32());
                // The per-slot types the entries are compared against.
                for i in 0..4u32 {
                    let ty = if (i + seed as u32) % 3 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(gp + 0xcb8 + 4 * i, ty);
                }
                let n = (seed % 5) as i32 - 1;
                for i in 0..4u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, (seed + i) % 4);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    /// The mission description (a -1 id or a real one), the mission id and
    /// value words, and a script manager whose command answers -1 at once
    /// (the value is never 0xf / 0x13 and the game state is never 4), so
    /// both the "no command" copy and the command path run.
    #[test]
    fn wscript_fn_mission_command_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{machine, FAKE};
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const MAN: u32 = DATA + 0x400;
        const BE_LIB: u32 = DATA + 0x1000;
        const BE_IFACE: u32 = DATA + 0x1100;
        const BE_VT: u32 = DATA + 0x1200;
        check(
            ee::s10::f_003012f0__generated,
            wscript_fn_mission_command,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_12f0, 0);
                elf_image(c);
                c.pc = 0x0030_12f0;
                let gp = GAME_GP;
                // The static description (id, value, reserved) and its flag.
                c.bus.write32(
                    OC324_MISSION_DESC,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 64
                    },
                );
                c.bus.write32(OC324_MISSION_DESC + 4, 5 + seed as u32 % 7);
                c.bus.write32(OC324_MISSION_DESC + 8, r.next_u32());
                c.bus
                    .write32(gp + 0x25c8, if seed % 3 == 0 { 0 } else { 1 });
                // The per-slot types of the argument expansion.
                for i in 0..4u32 {
                    let ty = if (i + seed as u32) % 3 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(gp + 0xb80 + 4 * i, ty);
                }
                // The script manager and the back end.
                c.bus.write32(gp + 0xce8, MAN);
                c.bus.write32(gp - 0x1f04, BE_LIB);
                c.bus.write32(gp - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VT);
                c.bus.write16(BE_VT + 8, 0);
                c.bus.write32(BE_VT + 0xc, FAKE + 0x80);
                c.bus.write8(0x0053_5c10, (r.next_u32() % 3) as u8);
                // Entries only touch slots 2 and 3: the id and value words
                // stay as described.
                let n = (seed % 4) as i32 - 1;
                for i in 0..3u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, 2 + i % 2);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    /// A world whose systems block holds the focus manager, with a focus
    /// count of 0..4 and the added object's flags.
    #[test]
    fn world_add_focus_object_matches() {
        use crate::difftest::fakes::machine;
        const WORLD: u32 = DATA + 0x1000;
        const SYSTEMS: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        const OBJ: u32 = DATA + 0x4000;
        check(
            ee::s09::f_002d1bf0__generated,
            world_add_focus_object,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_1bf0, 0);
                c.pc = 0x002d_1bf0;
                let gp = GAME_GP;
                c.bus.write32(gp.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x84, SYSTEMS);
                c.bus.write32(SYSTEMS + 0xc, DATA + 0x2500);
                c.bus.write32(DATA + 0x2500 + 0xa4, FOCUS);
                c.bus
                    .write32(FOCUS + 0xd4, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(FOCUS + 0xd8, (seed % 5) as u32);
                c.bus.write32(OBJ + 8, r.next_u32());
                c.g[A0][0] = sx(OBJ);
            },
        );
    }

    /// A StreamBuffer with a known, unknown or empty SND stream, with and
    /// without a request; the SND lock / unlock hooks are fakes.
    #[test]
    fn stream_buffer_get_current_request_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x22_0000;
        const REQ: u32 = DATA + 0x24_0000;
        check(
            ee::s08::f_002aa210__generated,
            stream_buffer_get_current_request,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a210, 2);
                c.pc = 0x002a_a210;
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 3 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(REQ + 0xc, r.next_u32());
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// A channel table entry that is unbound, one whose event is missing and
    /// one with a bound event (with and without the request-list flag byte),
    /// over an all-zero SPCH state.
    #[test]
    fn spch_select_request_for_channel_matches() {
        use crate::difftest::fakes::machine;
        const SLOTS: u32 = 0x0044_ffb8;
        const EVENT: u32 = DATA + 0x2000;
        check(
            ee::s14::f_003d7a50__generated,
            spch_select_request_for_channel,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_7a50, 0);
                c.pc = 0x003d_7a50;
                let idx = seed % 8;
                let entry = 0x0045_0660 + 8 * idx;
                // Zero the SPCH state the lookup and the selection read.
                for a in (0x0045_0000..0x0045_1000).step_by(4) {
                    c.bus.write32(a, 0);
                }
                let h = 0x1000u32 + seed % 8;
                c.bus.write16(entry, h as u16);
                c.bus.write8(entry + 2, (seed % 5) as u8);
                c.bus.write8(entry + 3, (seed / 5 % 5) as u8);
                let bound = match seed % 4 {
                    0 => false, // unbound: handle 0xffff
                    1 => false, // bound, but no event slot matches
                    _ => true,
                };
                if seed % 4 == 0 {
                    c.bus.write16(entry, 0xffff);
                }
                if bound {
                    // One of the eight event slots carries the event.
                    c.bus.write32(SLOTS + 8 * (seed % 8), EVENT);
                    for i in 0..0x40u32 {
                        c.bus.write8(EVENT + i, 0);
                    }
                    c.bus.write8(EVENT + 9, (seed % 5) as u8);
                    c.bus.write8(EVENT + 8, (seed / 5 % 5) as u8);
                    c.bus.write16(EVENT + 0x10, 1);
                    c.bus.write16(EVENT + 0x18, 0);
                    c.bus.write16(EVENT, h as u16);
                    // The event's counters and flags.
                    c.bus
                        .write8(EVENT + 0xa, if seed % 3 == 0 { 0x10 } else { 0 });
                    c.bus.write8(EVENT + 6, (seed % 4) as u8);
                    c.bus.write8(EVENT + 7, (seed % 3) as u8);
                    c.bus.write8(EVENT + 0xc, 1);
                    c.bus.write16(EVENT + 0xe, 0);
                }
                c.g[A0][0] = idx as u64;
            },
        );
    }

    /// A node with a vtable whose slots 48 and 49 are fakes (unwinding for
    /// some seeds) and a modifier block that is either present (freed first)
    /// or created by the real constructor on a test heap.
    #[test]
    fn mod_obj_set_transform_modifier_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const NODE: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x800;
        const OBJ: u32 = DATA + 0xc00;
        const BLOCK: u32 = DATA + 0xd00;
        const MOD: u32 = DATA + 0xe00;
        check(
            ee::s11::f_003554b0__generated,
            mod_obj_set_transform_modifier,
            150,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x35_54b0);
                c.pc = 0x0035_54b0;
                for i in 0..0x20u32 {
                    c.bus.write32(NODE + 4 * i, 0);
                    c.bus.write32(OBJ + 4 * i, 0);
                    c.bus.write32(BLOCK + 4 * i, 0);
                    c.bus.write32(MOD + 4 * i, 0);
                }
                for i in 0..0x70u32 {
                    c.bus.write32(VT + 4 * i, 0);
                }
                c.bus.write32(NODE + 0xc, VT);
                c.bus.write16(VT + 0x188, (4 * (r.next_u32() % 4)) as u16);
                c.bus.write32(VT + 0x18c, FAKE);
                c.bus.write16(VT + 0x180, (4 * (r.next_u32() % 4)) as u16);
                c.bus.write32(VT + 0x184, FAKE + 0x10);
                c.bus.write32(NODE + 0x18, OBJ);
                c.bus.write32(OBJ + 8, r.next_u32());
                c.bus
                    .write32(NODE + 0x1c, if seed % 2 == 0 { 0 } else { BLOCK });
                c.g[A0][0] = sx(NODE);
                c.g[A1][0] = sx(MOD);
            },
        );
    }

    /// The player slot table and character records the real
    /// `Rider_GetPlayerCharacterInfo` / `Rider_GetCharacter` read, with the
    /// character id covering 0, 14, 16, 21 and other kinds.
    #[test]
    fn ssx_audio_system_get_grunt_voice_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const RIDER: u32 = DATA + 0x1000;
        const LIB: u32 = DATA + 0x8000;
        const IFACE: u32 = DATA + 0x8100;
        const IFACE_VT: u32 = DATA + 0x8200;
        check(
            ee::s08::f_0029f378__generated,
            ssx_audio_system_get_grunt_voice,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_f378, 0);
                c.pc = 0x0029_f378;
                let gp = GAME_GP;
                c.bus.write32(gp - 0x1f04, LIB);
                c.bus.write32(gp - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, IFACE);
                c.bus.write32(IFACE + 4, IFACE_VT);
                c.bus.write16(IFACE_VT + 8, (4 * (r.next_u32() % 4)) as u16);
                c.bus.write32(IFACE_VT + 0xc, FAKE);
                // The player slot table and the character records.
                for i in 0..8u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
                }
                for i in 0..0x1c * 8 / 4 {
                    c.bus.write32(0x0053_4fe0 + 4 * i, r.next_u32());
                }
                let slot = (seed % 8) as u32;
                let id = match seed % 6 {
                    0 => 0u32,
                    1 => 14,
                    2 => 16,
                    3 => 21,
                    4 => 1 + r.next_u32() % 13,
                    _ => r.next_u32() % 32,
                };
                c.bus.write8(0x0053_4fe0 + 0x1c * slot + 0x12, id as u8);
                c.bus.write32(RIDER + 0x86c, slot);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(RIDER);
            },
        );
    }

    /// Random score factors (edge-case floats included) and the 10000 /
    /// 10 constants the function loads from the game image.
    #[test]
    fn rider_metrix_get_trick_score_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::machine;
        const SELF: u32 = DATA + 0x100;
        check(
            ee::s00::f_00117990__generated,
            rider_metrix_get_trick_score,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_7990, 0);
                elf_image(c);
                c.pc = 0x0011_7990;
                c.bus.write32(GAME_GP - 0x7b04, 10_000.0f32.to_bits());
                c.bus.write32(SELF + 0x18, r.edge_f32());
                c.bus.write32(SELF + 0x14, r.edge_f32());
                c.bus.write32(SELF + 0x1c4, r.f32(2.0));
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// A description block with ids of -1 and of a table entry, an object
    /// with and without a modifier node, over the game image and a test
    /// heap (the constructor allocates).
    #[test]
    fn world_obj_factory_mesh_anim_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::machine;
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const INST: u32 = DATA + 0x8000;
        const INST_HDR: u32 = DATA + 0x8100;
        const TABLE: u32 = DATA + 0x8200;
        const ENTRY: u32 = DATA + 0x8300;
        const SLOTS: u32 = DATA + 0x8400;
        const OBJECT: u32 = DATA + 0x8800;
        check(
            ee::s10::f_002fcff0__generated,
            world_obj_factory_mesh_anim,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_cff0, 0);
                elf_image(c);
                crate::game::bx::bxstring::tests::heap(c);
                c.pc = 0x002f_cff0;
                let gp = GAME_GP;
                // The static description and its flag.
                c.bus.write32(OC324_MESH_ANIM_DESC, r.next_u32());
                c.bus.write32(OC324_MESH_ANIM_DESC + 4, seed as u32 % 8);
                for i in 2..14u32 {
                    c.bus.write32(OC324_MESH_ANIM_DESC + 4 * i, r.f32(10.0));
                }
                c.bus
                    .write32(gp + OC324_MESH_ANIM_INIT, if seed % 3 == 0 { 0 } else { 1 });
                // The per-slot types of the argument expansion.
                for i in 0..16u32 {
                    let ty = if (i + seed as u32) % 4 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_6060 + 4 * i, ty);
                }
                // The instance tables the id resolves through.
                c.bus.write32(gp + 0x16c8, INST);
                c.bus.write32(INST, INST_HDR);
                c.bus.write32(INST_HDR + 8, TABLE);
                for i in 0..4u32 {
                    c.bus.write32(TABLE + 4 * i, ENTRY);
                }
                c.bus.write32(ENTRY + 0x1c, SLOTS);
                for i in 0..8u32 {
                    let word = match (seed + i) % 3 {
                        0 => 0,
                        _ => (OBJECT >> 2) << 8 | (r.next_u32() & 0xff),
                    };
                    c.bus.write32(SLOTS + 4 * i, word);
                }
                // The object the modifier attaches to: no node (skip the
                // active-node test) or a node that already is a type-9
                // modifier (the test answers 1, so it is skipped).
                let node = DATA + 0x8900;
                let node_vt = DATA + 0x8a00;
                c.bus
                    .write32(OBJECT + 0xc, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(OBJECT + 8, 0);
                c.bus.write32(node, 9);
                c.bus.write32(node + 0xc, node_vt);
                c.bus.write16(node_vt + 0x50, 0);
                c.bus.write32(node_vt + 0x54, 0x002a_fa58);
                c.bus.write32(gp + 0xce8, DATA + 0x8b00);
                // The argument block: id (or -1) and model handle.
                let n = (seed % 4) as i32 - 1;
                for i in 0..4u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, 14 + i % 2);
                    c.bus.write32(e + 4, r.next_u32() % 100);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }
}
