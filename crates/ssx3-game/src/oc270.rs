//! Staging wave 209 (oc270): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_5f98, snd_fire_events, "SND_FireEvents"),
    (0x002a_68b0, crowd_audio_update_pitch_bend, "CrowdAudio_UpdatePitchBend"),
    (0x002f_c0d0, world_obj_factory_object, "WorldObjFactory_Object"),
    (0x002a_a1b8, stream_buffer_is_playing, "StreamBuffer_IsPlaying"),
    (0x003a_6d18, world_get_section_script_func, "World_GetSectionScriptFunc"),
    (0x0011_1aa0, rider_base_trigger_dynamic_spray, "RiderBase_TriggerDynamicSpray"),
    (0x0034_fb00, world_obj_ctor, "WorldObj_ctor"),
    (0x0037_c8c0, ps2_graphics_man_v46, "PS2GraphicsMan_v46"),
];

// ---------------------------------------------------------------------------
// SND_FireEvents (0x3b5f98)
// ---------------------------------------------------------------------------

/// The sound library's event-handler table (`D_0050A8E8`).
const SND_EVENT_TABLE: u32 = 0x0050_a8e8;
/// Number of registered handlers (signed byte).
const SND_EVENT_HANDLER_COUNT: u32 = 0x179;
/// The handler function pointers.
const SND_EVENT_HANDLERS: u32 = 0x1c8;
/// Queued event count of an event source (signed byte).
const SND_SOURCE_PENDING: u32 = 0x17;
/// Queued first words (`a[]`) and second words (`b[]`).
const SND_SOURCE_QUEUE_A: u32 = 0x34;
const SND_SOURCE_QUEUE_B: u32 = 0x44;

/// `void SND_FireEvents(Source* s, int arg)` (0x3b5f98): pops the source's
/// queued events from the back; for each one builds a message
/// `{arg, a[n], b[n]}` on the stack and hands it to every registered handler
/// of the sound event table, in order.
pub fn snd_fire_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x90);
        frame.save(c, S6, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        let source = Addr(r32(c, S2));
        let pending = source.off(SND_SOURCE_PENDING).lb(c);
        c.g[V0][0] = pending;
        c.g[V1][0] = source.off(SND_SOURCE_PENDING).lbu(c);
        if pending as i64 > 0 {
            c.cycles += 6;
            c.g[V0][0] = 0x51_0000;
            c.g[S5][0] = sx(r32(c, S2).wrapping_add(SND_SOURCE_QUEUE_A));
            c.g[FP][0] = 0x51_0000;
            c.g[S7][0] = sx(SND_EVENT_TABLE);
            c.g[S4][0] = sx(r32(c, S2).wrapping_add(SND_SOURCE_QUEUE_B));
            loop {
                // Pop one event and build the message at 0(sp).
                c.cycles += 14;
                let n = r32(c, V1).wrapping_sub(1);
                c.g[V1][0] = sx(n);
                let msg = sp(c);
                let arg = c.g[S6][0];
                Addr(msg).sw(c, arg);
                Addr(r32(c, S2)).off(SND_SOURCE_PENDING).sb(c, sx(n));
                let index = ((n << 24) as i32 >> 22) as u32;
                c.g[S1][0] = 0;
                let b_at = r32(c, S4).wrapping_add(index);
                let a_at = r32(c, S5).wrapping_add(index);
                c.g[V1][0] = sx(b_at);
                c.g[A1][0] = Addr(b_at).lw(c);
                c.g[A0][0] = Addr(a_at).lw(c);
                let handlers = Addr(r32(c, S7)).off(SND_EVENT_HANDLER_COUNT).lb(c);
                c.g[V0][0] = handlers;
                let (a, b) = (c.g[A0][0], c.g[A1][0]);
                Addr(msg).off(4).sw(c, a);
                Addr(msg).off(8).sw(c, b);
                if handlers as i64 > 0 {
                    // Call every handler with the message.
                    c.cycles += 4;
                    c.g[S3][0] = sx(r32(c, FP).wrapping_sub(0x5718));
                    c.g[S0][0] = sx(r32(c, S3).wrapping_add(SND_EVENT_HANDLERS));
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                    loop {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                        let handler = r32(c, V1);
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                        call_ptr(c, handler, 0x003b_6048)?;
                        let count = Addr(r32(c, S3)).off(SND_EVENT_HANDLER_COUNT).lb(c);
                        let more = (c.g[S1][0] as i64) < (count as i64);
                        c.g[V0][0] = more as u64;
                        if !more {
                            break;
                        }
                        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_6038);
                        }
                    }
                }
                c.cycles += 3;
                let source = Addr(r32(c, S2));
                let pending = source.off(SND_SOURCE_PENDING).lb(c);
                c.g[V0][0] = pending;
                c.g[V1][0] = source.off(SND_SOURCE_PENDING).lbu(c);
                if pending as i64 <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_5ff0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdAudio_UpdatePitchBend (0x2a68b0)
// ---------------------------------------------------------------------------

/// Sound handle of a bank-monitor instance (negative: not playing).
const CROWD_VOICE_HANDLE: u32 = 0x4;
/// CrowdAudio: index of the active pitch-bend curve, and its time left.
const CROWD_BEND_CURVE: u32 = 0x184;
const CROWD_BEND_TIME: u32 = 0x188;
/// gp-relative pointer to the crowd sound table (`D_004A36F8`), 0x54-byte
/// entries; the bend curve fields start 0x1c into an entry.
const CROWD_TABLE_GP: u32 = 0x608;
const CROWD_ENTRY_SIZE: u32 = 0x54;
const CROWD_ENTRY_BEND: u32 = 0x1c;
/// Curve fields (relative to the bend start): ramp-down length, ramp-up
/// length, bend depth.
const BEND_FALL: u32 = 0x48;
const BEND_RISE: u32 = 0x4c;
const BEND_DEPTH: u32 = 0x50;

/// `void CrowdAudio_UpdatePitchBend(CrowdAudio* self, BankMonitorInstance*
/// voice, float dt)` (0x2a68b0): a voice that is not playing is reset;
/// otherwise, while a pitch bend is running, counts its time down by `dt`
/// and sets the voice's pitch bend (0x40 = centre) from the curve: rising
/// while the time left is under the rise length, then falling.
pub fn crowd_audio_update_pitch_bend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let handle = Addr(r32(c, S1)).off(CROWD_VOICE_HANDLE).lw(c);
        c.g[V0][0] = handle;
        c.g[A1][0] = c.g[A0][0];
        if (handle as i64) < 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            Callee { addr: 0x002a_bb38, f: ee::s08::f_002abb38 }.call(c, 0x002a_68d8)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 6;
            let this = Addr(r32(c, A1));
            let time = this.off(CROWD_BEND_TIME).read_fpu(c);
            c.f[1] = time.0;
            c.f[0] = 0;
            let idle = time.le(FpuF32::ZERO);
            abi::set_fcc(c, idle);
            frame.restore(c, S0, 0x20);
            if !idle {
                c.cycles += 13;
                let curve = this.off(CROWD_BEND_CURVE).lw(c);
                c.g[V1][0] = CROWD_ENTRY_SIZE as u64;
                let left = time - FpuF32(c.f[12]);
                c.f[12] = left.0;
                let table = Addr(c.g[GP][0] as u32).off(CROWD_TABLE_GP).lw(c) as u32;
                let offset = abi::mult(c, curve as u32, CROWD_ENTRY_SIZE)
                    .wrapping_add(CROWD_ENTRY_BEND);
                this.off(CROWD_BEND_TIME).write_fpu(c, left);
                c.g[V0][0] = sx(offset);
                let bend = Addr(table.wrapping_add(offset));
                c.g[A0][0] = sx(bend.0);
                let rise = bend.off(BEND_RISE).read_fpu(c);
                c.f[1] = rise.0;
                let falling = abi::c_lt_s(c, rise, left);
                c.g[S0][0] = 0x40;
                let scaled = if falling {
                    // Past the rise: depth * (fall - (left - rise)) / fall.
                    c.cycles += 8;
                    let over = left - rise;
                    let fall = bend.off(BEND_FALL).read_fpu(c);
                    let depth = bend.off(BEND_DEPTH).read_fpu(c);
                    let remaining = fall - over;
                    c.f[1] = remaining.0;
                    c.f[2] = fall.0;
                    Some((depth * remaining).div(fall))
                } else {
                    c.cycles += 3;
                    if abi::c_lt_s(c, FpuF32::ZERO, left) {
                        // Rising: depth * left / rise.
                        c.cycles += 3;
                        let depth = bend.off(BEND_DEPTH).read_fpu(c);
                        Some((depth * left).div(rise))
                    } else {
                        None
                    }
                };
                if let Some(scaled) = scaled {
                    c.cycles += 3;
                    c.f[0] = scaled.0;
                    let whole = scaled.to_i32() as u32;
                    c.f[1] = whole;
                    c.g[V0][0] = sx(whole);
                    c.g[S0][0] = sx(whole.wrapping_add(0x40));
                }
                c.cycles += 8;
                call_ptr(c, 0x003b_58a0, 0x002a_6978)?; // SND_Lock
                c.g[A0][0] = Addr(r32(c, S1)).off(CROWD_VOICE_HANDLE).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003b_c6b8, 0x002a_6984)?; // SND_SetPitchBend
                call_ptr(c, 0x003b_58d8, 0x002a_698c)?; // SND_Unlock
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_Object (0x2fc0d0)
// ---------------------------------------------------------------------------

/// Guard word of the factory's static default parameter block.
const WOF_DEFAULTS_READY_GP: u32 = 0x24f0;
/// The static default parameter block `{id = -1, 0, 0, tag}` (16 bytes).
const WOF_DEFAULTS: u32 = 0x004f_b4f0;
/// Parameter kinds by slot (`D_00445F90`): kind 2 is a float slot.
const WOF_PARAM_KINDS: u32 = 0x0044_5f90;
const WOF_KIND_FLOAT: u64 = 2;
/// gp-relative: the world-object directory (`id & 0xff` selects a section,
/// `id >> 8` an object in it) and the world (its default object at +0x290).
const WOF_DIRECTORY_GP: u32 = 0x16c8;
const WOF_WORLD_GP: u32 = 0xce8;
const WOF_WORLD_DEFAULT_OBJECT: u32 = 0x290;
/// The factory's result value (12 bytes, returned by copy).
const WOF_RESULT: u32 = 0x004c_9098;
/// Object fields: the object's active node.
const WOF_OBJECT_NODE: u32 = 0xc;

/// `Result WorldObjFactory_Object(Param* params, int count)` (0x2fc0d0;
/// a0 = the hidden result slot, a1 = count, a2 = params): fills a 4-word
/// parameter block from the defaults and the `{slot, value, -, kind}`
/// parameters (an int given for a float slot is converted), looks the object
/// id up (-1: the world's default object), and unless the object already
/// has an active node of kind 0x11 for these parameters, creates an
/// `Object` (0x2c bytes) for it. Returns the factory's result value.
pub fn world_obj_factory_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        let ready = Addr(c.g[GP][0] as u32).off(WOF_DEFAULTS_READY_GP).lw(c);
        c.g[V0][0] = ready;
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x50_0000;
        frame.save_ra(c, 0x20);
        let defaults = Addr(WOF_DEFAULTS);
        if ready == 0 {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 11;
            c.g[V1][0] = sx(WOF_DEFAULTS);
            c.g[V0][0] = u64::MAX;
            let tag = defaults.off(0xc).lbu(c);
            c.g[A1][0] = tag;
            defaults.sw(c, u64::MAX);
            c.g[A0][0] = 1;
            defaults.off(0xc).sb(c, tag);
            Addr(c.g[GP][0] as u32).off(WOF_DEFAULTS_READY_GP).sw(c, 1);
            defaults.off(4).sw(c, 0);
            defaults.off(8).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S1][0] = sx(block);
        c.g[A1][0] = sx(WOF_DEFAULTS);
        c.g[A2][0] = 0x10;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = sx(block);
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_c154)?; // memcpy
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(WOF_PARAM_KINDS);
            c.g[A3][0] = WOF_KIND_FLOAT;
            loop {
                c.cycles += 8;
                let param = Addr(r32(c, A2));
                let slot4 = (param.lw(c) as u32) << 2;
                let given_kind = param.off(0xc).lw(c);
                c.g[V1][0] = given_kind;
                let dest = Addr(slot4.wrapping_add(r32(c, S1)));
                c.g[A0][0] = sx(dest.0);
                let kind = Addr(slot4.wrapping_add(r32(c, T0))).lw(c);
                c.g[V0][0] = kind;
                if given_kind != kind && kind == c.g[A3][0] {
                    // An int for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).lw(c) as u32;
                    let converted = FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if given_kind == kind { 1 } else { 2 + 1 };
                    let value = param.off(4).lw(c);
                    c.g[V0][0] = value;
                    dest.sw(c, value);
                }
                c.cycles += 3;
                let left = r32(c, A1).wrapping_sub(1);
                c.g[A1][0] = sx(left);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if left == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c170);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        let id = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let directory = Addr(c.g[GP][0] as u32).off(WOF_DIRECTORY_GP).lw(c);
        c.g[A1][0] = directory;
        if !id == 0 {
            c.cycles += 3;
            let world = Addr(c.g[GP][0] as u32).off(WOF_WORLD_GP).lw(c);
            c.g[V0][0] = world;
            c.g[S0][0] = Addr(world as u32).off(WOF_WORLD_DEFAULT_OBJECT).lw(c);
        } else {
            c.cycles += 8;
            let id = id as u32;
            let sections = Addr(directory as u32).lw(c);
            let table = Addr(sections as u32).off(8).lw(c);
            c.g[A0][0] = table;
            let at = ((id & 0xff) << 2).wrapping_add(table as u32);
            c.g[V0][0] = sx(at);
            let section = Addr(at).lw(c);
            c.g[V1][0] = section;
            let object = if section != 0 {
                c.cycles += 7;
                let entries = Addr(section as u32).off(0x1c).lw(c);
                c.g[A0][0] = entries;
                let entry = Addr(((id >> 8) << 2).wrapping_add(entries as u32)).lw(c);
                c.g[V1][0] = entry;
                let index = (entry as u32) >> 8;
                let object = if index != 0 {
                    sx(index << 2)
                } else {
                    c.cycles += 1;
                    0
                };
                c.g[V0][0] = object;
                c.cycles += 1;
                object
            } else {
                c.cycles += 2;
                0
            };
            c.g[A2][0] = object;
            c.cycles += 1;
            c.g[S0][0] = object;
        }
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            let node = Addr(r32(c, S0)).off(WOF_OBJECT_NODE).lw(c);
            c.g[A0][0] = node;
            c.g[A1][0] = 0x11;
            let mut create = true;
            if node != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                Callee { addr: 0x002f_ae38, f: ee::s10::f_002fae38 }.call(c, 0x002f_c240)?;
                create = c.g[V0][0] == 0;
            }
            if create {
                c.cycles += 13;
                c.g[A1][0] = 0x4a_3bf8;
                c.g[A0][0] = 0x2c;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                Callee { addr: 0x0031_7d70, f: ee::s10::f_00317d70 }.call(c, 0x002f_c260)?;
                let block = Addr(r32(c, S1));
                let flag = block.off(8).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A3][0] = block.off(4).lw(c);
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[T0][0] = (flag != 0) as u64;
                call_ptr(c, 0x0035_6db0, 0x002f_c27c)?; // Object_ctor
            }
        }
        // Return the factory's result value by copy.
        c.cycles += 16;
        let result = Addr(r32(c, S4).wrapping_sub(0x4d_0000 - WOF_RESULT));
        c.g[A1][0] = sx(result.0);
        let head = result.read_u64_unaligned(c);
        let tail = result.off(8).lw(c);
        c.g[V1][0] = tail;
        let out = Addr(r32(c, S2));
        out.write_u64_unaligned(c, head);
        out.off(8).sw(c, tail);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_IsPlaying (0x2aa1b8)
// ---------------------------------------------------------------------------

/// SndRequestStatus: the request's state (2 = playing).
const SBIP_REQUEST_STATE_PLAYING: u64 = 2;

/// `bool StreamBuffer_IsPlaying(StreamBuffer* self)` (0x2aa1b8): whether the
/// stream's current request is playing (`StreamBuffer_GetRequestStatus` into
/// a frame-local status, state == 2).
pub fn stream_buffer_is_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        Callee { addr: 0x002a_a068, f: ee::s08::f_002aa068 }.call(c, 0x002a_a1cc)?;
        let state = Addr(sp(c)).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = (state == SBIP_REQUEST_STATE_PLAYING) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World_GetSectionScriptFunc (0x3a6d18)
// ---------------------------------------------------------------------------

/// World: the section script-cache table (`{?, caches}`), 12-byte caches.
const WSSF_SCRIPT_TABLE: u32 = 0x4;
const WSSF_TABLE_CACHES: u32 = 0x4;
const WSSF_CACHE_SIZE: u32 = 0xc;

/// `ScriptFunc* World_GetSectionScriptFunc(World* self, int* handle)`
/// (0x3a6d18): the handle's low byte selects a section's script cache, the
/// rest the entry; returns `tWScriptCache_GetTableEntry(cache, handle >> 8)`.
pub fn world_get_section_script_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = WSSF_CACHE_SIZE as u64;
        frame.save_ra(c, 0);
        let handle = Addr(r32(c, A1)).lw(c) as u32;
        let table = Addr(r32(c, A0)).off(WSSF_SCRIPT_TABLE).lw(c);
        c.g[A2][0] = table;
        let offset = abi::mult(c, handle & 0xff, WSSF_CACHE_SIZE);
        let caches = Addr(table as u32).off(WSSF_TABLE_CACHES).lw(c);
        c.g[V0][0] = caches;
        c.g[A1][0] = sx(handle >> 8);
        c.g[A0][0] = sx((caches as u32).wrapping_add(offset));
        Callee { addr: 0x003a_dc50, f: ee::s13::f_003adc50 }.call(c, 0x003a_6d44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBase_TriggerDynamicSpray (0x111aa0)
// ---------------------------------------------------------------------------

/// RiderBase: its dynamic spray effect (`cDynamicSprayFX`).
const RIDER_DYNAMIC_SPRAY: u32 = 0xb40;

/// `void RiderBase_TriggerDynamicSpray(RiderBase* self, Vec* pos, Vec* dir,
/// int kind, float speed)` (0x111aa0): queues a spray burst on the rider's
/// dynamic spray effect (`cDynamicSprayFX_QueueBurst(&self->spray, pos, dir,
/// kind, 0, speed)`).
pub fn rider_base_trigger_dynamic_spray(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(RIDER_DYNAMIC_SPRAY));
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        call_ptr(c, 0x002e_23e0, 0x0011_1ab4)?; // cDynamicSprayFX_QueueBurst
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObj_ctor (0x34fb00)
// ---------------------------------------------------------------------------

/// WorldObj: vtable (+0xc) and its world node (+0x18).
const WOBJ_VTABLE: u32 = 0xc;
const WOBJ_NODE: u32 = 0x18;
/// The WorldObj vtable (`D_00491C80`).
const WOBJ_VTABLE_ADDR: u32 = 0x0049_1c80;
/// Node flags (+8): 0x100 = already in the focus manager.
const WOBJ_NODE_FLAGS: u32 = 0x8;
const WOBJ_NODE_FOCUSED: u64 = 0x100;
/// Process kinds that never take focus.
const WOBJ_KINDS_UNFOCUSED: [u64; 3] = [0x6, 0x10, 0x16];

/// `WorldObj* WorldObj_ctor(WorldObj* self, int list, int kind, Node* node)`
/// (0x34fb00): constructs the sync process (`SyncProcess_ctor(self, list,
/// kind)`), sets the node and the WorldObj vtable, links the node back to
/// the object, and adds the node to the focus manager unless it is already
/// there or the kind is 6, 0x10 or 0x16. Returns `self`.
pub fn world_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        Callee { addr: 0x0035_4850, f: ee::s11::f_00354850 }.call(c, 0x0034_fb24)?;
        let this = Addr(r32(c, S2));
        let node = c.g[S0][0];
        this.off(WOBJ_NODE).sw(c, node);
        c.g[V1][0] = sx(WOBJ_VTABLE_ADDR);
        c.g[A0][0] = c.g[S2][0];
        this.off(WOBJ_VTABLE).sw(c, sx(WOBJ_VTABLE_ADDR));
        c.g[A1][0] = c.g[S0][0];
        Callee { addr: 0x0034_fc78, f: ee::s11::f_0034fc78 }.call(c, 0x0034_fb40)?;
        let focused = Addr(r32(c, S0)).off(WOBJ_NODE_FLAGS).lw(c) & WOBJ_NODE_FOCUSED != 0;
        c.g[V0][0] = c.g[S2][0];
        if !focused {
            // Compare the kind against 6, 0x10, 0x16 in turn (each delay
            // slot loads the next constant into v0, the last one `self`).
            c.cycles += 3;
            let kind = c.g[S1][0];
            c.g[V0][0] = 0x10;
            let add = if kind == WOBJ_KINDS_UNFOCUSED[0] {
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
                false
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x16;
                if kind == WOBJ_KINDS_UNFOCUSED[1] {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    false
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    kind != WOBJ_KINDS_UNFOCUSED[2]
                }
            };
            if add {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002d_1bf0, 0x0034_fb74)?; // World_AddFocusObject
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
            }
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

// ---------------------------------------------------------------------------
// PS2GraphicsMan_v46 (0x37c8c0)
// ---------------------------------------------------------------------------

/// Texture header: format byte (+0; the word's upper 24 bits are the
/// palette's offset), two shorts passed on (+4, +6), the mip LOD scale (+8),
/// flags (+0xc: 0x1000 = data offset at +0x10 instead of inline, 0x2000 =
/// alternate palette format, top nibble = mip count), data (+0x10).
const V46_TEX_DIM_A: u32 = 0x4;
const V46_TEX_DIM_B: u32 = 0x6;
const V46_TEX_LOD_SCALE: u32 = 0x8;
const V46_TEX_FLAGS: u32 = 0xc;
const V46_TEX_DATA: u32 = 0x10;
const V46_FLAG_OFFSET_DATA: u64 = 0x1000;
const V46_FLAG_ALT_PALETTE: u64 = 0x2000;
/// PS2GraphicsMan: the renderer interface (+0x10d8) whose method at
/// +0x178/+0x17c uploads the texture.
const V46_RENDERER: u32 = 0x10d8;
const V46_UPLOAD_DELTA: u32 = 0x178;
const V46_UPLOAD_FN: u32 = 0x17c;

/// The palette part of a paletted texture (formats 1 and 2), with `v0` = the
/// header's first word: palette at `tex + (word >> 8)` (none for 0), the
/// palette format `t2` (`alt` with flag 0x2000), and the palette data
/// pointer `t4` (inline at +0x10, or at the palette's +0x10 offset).
fn v46_palette(c: &mut Cpu, cycles: u64, format: u64, alt: u64) {
    c.cycles += cycles;
    let tex = r32(c, S0);
    c.g[T2][0] = format;
    let flags = Addr(tex).off(V46_TEX_FLAGS).lw(c);
    c.g[A1][0] = flags;
    c.g[A2][0] = alt;
    let offset = (r32(c, V0) as i32 >> 8) as u32;
    c.g[V0][0] = sx(offset);
    c.g[T3][0] = 1;
    let palette = if offset == 0 { 0 } else { tex.wrapping_add(offset) };
    c.g[V1][0] = sx(palette);
    c.g[A0][0] = flags & V46_FLAG_ALT_PALETTE;
    if flags & V46_FLAG_ALT_PALETTE != 0 {
        c.g[T2][0] = alt;
    }
    let palette_flags = Addr(palette).off(V46_TEX_FLAGS).lw(c) & V46_FLAG_OFFSET_DATA;
    c.g[V0][0] = palette_flags;
    c.g[T1][0] = flags;
    if palette_flags != 0 {
        c.cycles += 3;
        let rel = Addr(palette).off(V46_TEX_DATA).lw(c);
        c.g[V0][0] = rel;
        c.g[T4][0] = sx(palette.wrapping_add(rel as u32));
    } else {
        c.cycles += 2;
        c.g[T4][0] = sx(palette.wrapping_add(V46_TEX_DATA));
    }
}

/// `int PS2GraphicsMan_v46(PS2GraphicsMan* self, Texture* tex, a2, a3, t0,
/// t1)` (0x37c8c0): uploads a texture through the renderer: works out the
/// palette format and data (formats 1/2 paletted, 3 = format 2, 5 = 0, 0 and
/// others keep 0) and the texel data pointer, calls the renderer's upload
/// method with them; for a mipmapped texture with a LOD scale, sets the
/// texture filter with LOD bias `min(int(log2-ish(240 / (scale/16384))),
/// -135)` clamped below at -2047 (soft-float double chain). Returns the
/// upload's handle.
pub fn ps2_graphics_man_v46(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[T5][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[T6][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[T7][0] = c.g[T1][0];
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        let tex = Addr(r32(c, S0));
        let format = tex.lbu(c);
        c.g[V1][0] = format;
        c.g[V0][0] = 2;
        c.g[T4][0] = 0;
        // Whether the format's flags still have to be tested (the shared
        // `v0 = t1 & 0x1000` tail).
        let mut test_flags = true;
        if format == 2 {
            c.g[V0][0] = tex.lw(c);
            v46_palette(c, 14, 9, 0xa);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if format < 3 {
                c.cycles += 2;
                if format == 1 {
                    c.g[V0][0] = tex.lw(c);
                    v46_palette(c, 13, 0xb, 0xc);
                } else {
                    c.cycles += 2;
                    c.g[T1][0] = tex.off(V46_TEX_FLAGS).lw(c);
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 5;
                if format == 3 {
                    c.cycles += 3;
                    c.g[T2][0] = 2;
                    c.g[T1][0] = tex.off(V46_TEX_FLAGS).lw(c);
                } else {
                    c.cycles += 2;
                    let flags = tex.off(V46_TEX_FLAGS).lw(c);
                    c.g[T1][0] = flags;
                    if format == 5 {
                        c.cycles += 1;
                        c.g[T2][0] = 0;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = flags & V46_FLAG_OFFSET_DATA;
                        test_flags = false;
                    }
                }
            }
        }
        if test_flags {
            c.cycles += 1;
            c.g[V0][0] = c.g[T1][0] & V46_FLAG_OFFSET_DATA;
        }
        // The texel data.
        c.cycles += 2;
        let tex = r32(c, S0);
        c.g[A1][0] = sx(tex.wrapping_add(V46_TEX_DATA));
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let rel = Addr(tex).off(V46_TEX_DATA).lw(c);
            c.g[V0][0] = rel;
            c.g[A1][0] = sx(tex.wrapping_add(rel as u32));
        }
        // Upload through the renderer: (self + delta, data, a2, dim_a, dim_b,
        // flags >> 28, t2, t3, palette data, a3, t0, t1 on the stack).
        c.cycles += 19;
        let renderer = Addr(r32(c, S2)).off(V46_RENDERER).lw(c) as u32;
        c.g[V1][0] = sx(renderer);
        c.g[A2][0] = c.g[T5][0];
        c.g[A3][0] = Addr(tex).off(V46_TEX_DIM_A).lh(c);
        c.g[T1][0] = sx(r32(c, T1) >> 28);
        let delta = Addr(renderer).off(V46_UPLOAD_DELTA).lh(c);
        let out = sp(c);
        let (t4, t6, t0, t7) = (c.g[T4][0], c.g[T6][0], c.g[T0][0], c.g[T7][0]);
        Addr(out).sw(c, t4);
        Addr(out).off(8).sw(c, t6);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(delta as u32));
        Addr(out).off(0x10).sw(c, t0);
        Addr(out).off(0x18).sw(c, t7);
        let upload = Addr(renderer).off(V46_UPLOAD_FN).lw(c);
        c.g[V0][0] = upload;
        c.g[T0][0] = Addr(tex).off(V46_TEX_DIM_B).lh(c);
        call_ptr(c, upload as u32, 0x0037_ca18)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0xffff_ffff_f000_0000;
        let tex = Addr(r32(c, S0));
        let mips = tex.off(V46_TEX_FLAGS).lw(c) & 0xffff_ffff_f000_0000;
        c.g[V0][0] = c.g[S1][0];
        if mips != 0 {
            c.cycles += 3;
            let scale = tex.off(V46_TEX_LOD_SCALE).lh(c);
            c.g[V0][0] = scale;
            if scale == 0 {
                c.g[V0][0] = c.g[S1][0];
            } else {
                // LOD bias from 240 / (scale / 16384), in doubles.
                c.cycles += 29;
                let step = FpuF32::from_i32(scale as i32);
                let unit = FpuF32(0x3880_0000);
                let distance = FpuF32(0x4370_0000);
                c.f[0] = unit.0;
                c.f[1] = distance.0;
                c.f[12] = distance.div(step * unit).0;
                call_ptr(c, 0x0041_3af8, 0x0037_ca64)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0040_d610, 0x0037_ca6c)?;
                c.g[A1][0] = Addr(0x0049_2db0).ld(c);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0041_1fd8, 0x0037_ca7c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = 0;
                call_ptr(c, 0x0041_1f70, 0x0037_ca88)?;
                c.g[A1][0] = 0x8060u64 << 47;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0041_1fd8, 0x0037_ca98)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0041_2608, 0x0037_caa0)?;
                let bias = c.g[V0][0];
                c.g[V1][0] = bias;
                let too_low = (bias as i64) < -2047;
                c.g[V0][0] = too_low as u64;
                c.g[A0][0] = -135i64 as u64;
                if too_low {
                    c.cycles += 1;
                    c.g[T1][0] = -2047i64 as u64;
                } else {
                    c.cycles += 4;
                    let above = -135i64 < bias as i64;
                    c.g[V0][0] = above as u64;
                    c.g[T1][0] = if above { -135i64 as u64 } else { bias };
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 5;
                c.g[A3][0] = 1;
                c.g[T0][0] = 0;
                Callee { addr: 0x0037_d090, f: ee::s12::f_0037d090 }.call(c, 0x0037_cadc)?;
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{fill, DATA};

    /// `SND_FireEvents`: a source with 0..4 queued events (or a negative
    /// count: nothing to do) and random queue words; the event table has
    /// -1..3 handlers, all fakes that log the message pointer and sometimes
    /// unwind.
    #[test]
    fn snd_fire_events_matches() {
        const SOURCE: u32 = DATA + 0x100;
        covered(ee::s13::f_003b5f98__generated, snd_fire_events, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_5f98, 8);
            c.pc = 0x003b_5f98;
            fill(c, &mut r, SOURCE, 0x20);
            let pending = [0u8, 1, 2, 3, 4, 0x80, 0xff][seed as usize % 7];
            c.bus.write8(SOURCE + SND_SOURCE_PENDING, pending);
            let handlers = [1u8, 2, 3, 0, 0xff][(seed / 7) as usize % 5];
            c.bus.write8(SND_EVENT_TABLE + SND_EVENT_HANDLER_COUNT, handlers);
            for i in 0..4 {
                c.bus
                    .write32(SND_EVENT_TABLE + SND_EVENT_HANDLERS + 4 * i, FAKE + 0x10 * i);
            }
            c.g[A0][0] = sx(SOURCE);
            c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
        });
    }

    /// `CrowdAudio_UpdatePitchBend`: a voice with a negative handle (reset;
    /// its fade-out already running so it returns at once) or a playing one;
    /// bend time left <= 0 (idle) or running, with dt so that the time left
    /// lands in the falling part, the rising part or at/below zero. The
    /// crowd table (gp+0x608) has 4 entries of random curves. SND_Lock and
    /// SND_Unlock run for real and call their callbacks (fakes);
    /// SND_SetPitchBend runs for real with no channels (handle not found).
    #[test]
    fn crowd_audio_update_pitch_bend_matches() {
        use crate::difftest::GAME_GP;
        const SELF: u32 = DATA + 0x100;
        const VOICE: u32 = DATA + 0x400;
        const TABLE: u32 = DATA + 0x800;
        covered(ee::s08::f_002a68b0__generated, crowd_audio_update_pitch_bend, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_68b0, 4);
            c.pc = 0x002a_68b0;
            fill(c, &mut r, SELF, 0x70);
            fill(c, &mut r, VOICE, 0x10);
            fill(c, &mut r, TABLE, 0x60);
            c.bus.write32(GAME_GP + CROWD_TABLE_GP, TABLE);
            // Lock/unlock callbacks; no sound channels.
            c.bus.write32(0x0044_c46c, FAKE);
            c.bus.write32(0x0044_c470, FAKE + 0x10);
            c.bus.write16(SND_EVENT_TABLE + 0x17e, 0);
            let playing = seed % 6 != 0;
            c.bus.write32(VOICE + CROWD_VOICE_HANDLE, if playing { r.next_u32() & 0x7fff_ffff } else { 0x8000_0000 | r.next_u32() });
            c.bus.write32(VOICE + 0x54, 1); // fade-out already running
            let curve = r.next_u32() % 4;
            c.bus.write32(SELF + CROWD_BEND_CURVE, curve);
            let bend = TABLE + curve * CROWD_ENTRY_SIZE + CROWD_ENTRY_BEND;
            let fall = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
            let rise = 0.25 + (r.next_u32() % 100) as f32 / 50.0;
            c.bus.write32(bend + BEND_FALL, fall.to_bits());
            c.bus.write32(bend + BEND_RISE, rise.to_bits());
            c.bus.write32(bend + BEND_DEPTH, r.f32(64.0));
            let time = match seed % 5 {
                0 => -((r.next_u32() % 3) as f32),
                _ => (r.next_u32() % 400) as f32 / 50.0,
            };
            c.bus.write32(SELF + CROWD_BEND_TIME, time.to_bits());
            // dt: the whole time (ends exactly at 0), an edge value, or a frame-ish step.
            c.f[12] = match (seed / 5) % 4 {
                0 => time.to_bits(),
                1 => r.edge_f32(),
                _ => ((r.next_u32() % 100) as f32 / 60.0).to_bits(),
            };
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(VOICE);
        });
    }

    /// `WorldObjFactory_Object`: the static defaults initialised or not;
    /// -1..4 parameters with random slots/kinds (ints into float slots get
    /// converted; slot 0 is the object id); the id is -1 (the world's
    /// default object, sometimes none) or a section/object pair whose
    /// section or entry may be empty. Found objects have no node (an
    /// `Object` is allocated from the pool allocator and constructed for
    /// real: process list, counters, flags) or a node whose modifier check
    /// is a fake returning non-zero (nothing created). Node flag 0x100 is
    /// always set so WorldObj_ctor does not reach World_AddFocusObject.
    /// Not reached: a node whose modifier check fails (needs a zero result
    /// through the real AttachModifier chain).
    #[test]
    fn world_obj_factory_object_matches() {
        use crate::difftest::{check, GAME_GP};
        use crate::game::fe::feasyncfile::tests::alloc_base_3;
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const NODE: u32 = DIR + 0xc00;
        const VT: u32 = DIR + 0xd00;
        const PROCS: u32 = DIR + 0x1000;
        const TAILS: u32 = DIR + 0x1800;
        const OBJS: u32 = DIR + 0x2000;
        check(ee::s10::f_002fc0d0__generated, world_obj_factory_object, 300, &|c, seed| {
            let mut r = alloc_base_3(c, seed, 0x2f_c0d0, 1);
            c.pc = 0x002f_c0d0;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // Static defaults.
            let ready = seed % 4 != 0;
            w(c, GAME_GP + WOF_DEFAULTS_READY_GP, ready as u32);
            fill(c, &mut r, WOF_DEFAULTS, 4);
            if ready && seed % 3 == 0 {
                w(c, WOF_DEFAULTS, u32::MAX);
            }
            for i in 0..4 {
                w(c, WOF_PARAM_KINDS + 4 * i, r.next_u32() % 4);
            }
            // Directory: sections 0..2 present, 3 empty; entries 0..2 objects, 3 empty.
            w(c, GAME_GP + WOF_DIRECTORY_GP, DIR);
            w(c, DIR, SECTIONS);
            w(c, SECTIONS + 8, TABLE);
            for i in 0..4 {
                w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                let obj = OBJS + 0x100 * i;
                w(c, ENTRIES + 4 * i, if i == 3 { r.next_u32() & 0xff } else { (obj >> 2) << 8 | (r.next_u32() & 0xff) });
                fill(c, &mut r, obj, 0x10);
                w(c, obj + 8, r.next_u32() | 0x100);
                w(c, obj + WOF_OBJECT_NODE, if (seed / 4) % 3 == 0 { NODE } else { 0 });
            }
            w(c, SECTION + 0x1c, ENTRIES);
            w(c, GAME_GP + WOF_WORLD_GP, WORLD);
            w(c, WORLD + WOF_WORLD_DEFAULT_OBJECT, if seed % 7 == 0 { 0 } else { OBJS + 0x100 });
            // The node's modifier check: a fake (non-zero result).
            w(c, NODE + 0xc, VT);
            c.bus.write16(VT + 0x50, 0);
            w(c, VT + 0x54, FAKE);
            // Process list for the new object (type 1).
            w(c, GAME_GP + 0x2898 + 4, PROCS);
            w(c, PROCS + 0x44, r.next_u32());
            w(c, PROCS + 0x44 + 8, TAILS);
            w(c, PROCS + 0x44 + 0x28, TAILS + 0x40);
            // The result value.
            fill(c, &mut r, WOF_RESULT, 3);
            // Parameters.
            let count: u32 = if seed % 11 == 0 { u32::MAX } else { seed % 5 };
            for i in 0..4 {
                let p = PARAMS + 0x10 * i;
                let slot = r.next_u32() % 4;
                let value = if slot == 0 {
                    match r.next_u32() % 4 {
                        0 => u32::MAX,
                        _ => (r.next_u32() % 4) | (r.next_u32() % 4) << 8,
                    }
                } else {
                    r.next_u32()
                };
                w(c, p, slot);
                w(c, p + 4, value);
                w(c, p + 8, r.next_u32());
                w(c, p + 0xc, r.next_u32() % 4);
            }
            c.g[A0][0] = sx(RESULT_SLOT + 4 * (seed % 2));
            c.g[A1][0] = sx(count);
            c.g[A2][0] = sx(PARAMS);
        });
    }

    /// `StreamBuffer_IsPlaying`: the fixture of
    /// `stream_buffer_get_request_status_matches` (fake lock hooks, one SND
    /// stream slot, known/unknown streams and requests), with the status
    /// word in the frame preset to 2 (playing) or junk where the real
    /// callees leave it alone.
    #[test]
    fn stream_buffer_is_playing_matches() {
        use crate::difftest::{check, SP};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(ee::s08::f_002aa1b8__generated, stream_buffer_is_playing, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_a1b8, 4);
            c.pc = 0x002a_a1b8;
            c.g[A0][0] = sx(SELF);
            c.bus.write32(SP - 0x20, if seed % 2 == 0 { 2 } else { r.next_u32() % 4 });
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            c.bus.write8(0x0051_0000 - 0x56d2, 1);
            c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
            let id = if seed % 3 == 0 { 1 + r.next_u32() % 4 } else { 0 };
            c.bus.write32(SELF + 0x8, id);
            c.bus.write32(STREAM + 0x124, if seed % 5 == 1 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(STREAM + 0x11c, REQ);
            c.bus.write32(REQ + 0xc, if seed % 4 < 2 { 0x8000_0000 | r.next_u32() } else { r.next_u32() & 0x7fff_ffff });
            c.bus.write16(STREAM + 0x14, 0);
            c.bus.write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
        });
    }

    /// `World_GetSectionScriptFunc`: handles over 4 sections' caches (type 3
    /// with a function table, or another type: null) and 8 entries; the real
    /// tWScriptCache_GetTableEntry.
    #[test]
    fn world_get_section_script_func_matches() {
        use crate::difftest::check;
        const WORLD: u32 = DATA + 0x100;
        const HANDLE: u32 = DATA + 0x200;
        const TABLE: u32 = DATA + 0x300;
        const CACHES: u32 = DATA + 0x400;
        const HEADER: u32 = DATA + 0x600;
        const FUNCS: u32 = DATA + 0x800;
        check(ee::s13::f_003a6d18__generated, world_get_section_script_func, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3a_6d18, 0);
            c.pc = 0x003a_6d18;
            c.bus.write32(WORLD + WSSF_SCRIPT_TABLE, TABLE);
            c.bus.write32(TABLE + WSSF_TABLE_CACHES, CACHES);
            for i in 0..4 {
                let cache = CACHES + WSSF_CACHE_SIZE * i;
                fill(c, &mut r, cache, 3);
                c.bus.write16(cache, if (seed + i) % 3 == 0 { r.next_u32() as u16 } else { 3 });
                c.bus.write32(cache + 8, HEADER);
            }
            c.bus.write32(HEADER + 0x3c, FUNCS);
            fill(c, &mut r, FUNCS, 8);
            let handle = (seed % 4) | (seed / 4 % 8) << 8;
            c.bus.write32(HANDLE, handle);
            c.g[A0][0] = sx(WORLD);
            c.g[A1][0] = sx(HANDLE);
        });
    }

    /// `RiderBase_TriggerDynamicSpray`: a rider whose spray effect (+0xb40)
    /// holds random state (strength, short range, queued position near or
    /// far), random burst position/direction, kind and speed; the real
    /// cDynamicSprayFX_QueueBurst runs.
    #[test]
    fn rider_base_trigger_dynamic_spray_matches() {
        use crate::difftest::{check, put_vec};
        const RIDER: u32 = DATA + 0x1000;
        const SPRAY: u32 = RIDER + RIDER_DYNAMIC_SPRAY;
        const POS: u32 = DATA + 0x400;
        const DIR: u32 = DATA + 0x410;
        check(ee::s00::f_00111aa0__generated, rider_base_trigger_dynamic_spray, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x11_1aa0, 1);
            c.pc = 0x0011_1aa0;
            fill(c, &mut r, SPRAY, 0x80);
            let spread = [30.0, 120.0, 300.0, 1000.0][seed as usize % 4];
            c.bus.write32(SPRAY + 0xe0, r.f32(if seed % 5 == 0 { 100.0 } else { 10.0 }));
            c.bus.write32(SPRAY + 0x124, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(SPRAY, RIDER); // its rider, with random carve flags
            fill(c, &mut r, RIDER + 0xac0, 0x12);
            put_vec(c, SPRAY + 0x100, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            put_vec(c, SPRAY + 0xf0, [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()]);
            put_vec(c, POS, [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()]);
            put_vec(c, DIR, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            c.g[A0][0] = sx(RIDER);
            c.g[A1][0] = sx(POS);
            c.g[A2][0] = sx(DIR);
            c.g[A3][0] = r.next_u32() as u64;
            c.f[12] = if seed % 7 == 0 { r.edge_f32() } else { r.f32(50.0) };
        });
    }

    /// `WorldObj_ctor`: lists 0..3 (each a process-list slot whose "pending"
    /// bit picks the tail at +8 or +0x28), kinds 6/0x10/0x16 (no focus) or
    /// others (counters in the typed-process table), nodes already focused
    /// (flag 0x100) or not; the real SyncProcess_ctor chain, LinkInstance
    /// and World_AddFocusObject (FocusMan_AddObject on a focus manager
    /// reached through the world at gp-0x848).
    #[test]
    fn world_obj_ctor_matches() {
        use crate::difftest::{check, GAME_GP};
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x200;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(ee::s11::f_0034fb00__generated, world_obj_ctor, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x34_fb00, 0);
            c.pc = 0x0034_fb00;
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            fill(c, &mut r, SELF, 0x10);
            fill(c, &mut r, NODE, 0x10);
            let flags = r.next_u32();
            w(c, NODE + WOBJ_NODE_FLAGS, if seed % 4 == 0 { flags | 0x100 } else { flags & !0x100 });
            let list = seed % 4;
            w(c, GAME_GP + 0x2898 + 4, PROCS);
            for i in 0..4 {
                w(c, PROCS + 0x44 * i, r.next_u32());
                w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
            }
            let kind = match (seed / 4) % 6 {
                0 => 6,
                1 => 0x10,
                2 => 0x16,
                _ => r.next_u32() % 0x20,
            };
            fill(c, &mut r, 0x0044_aff0, 0x20);
            // World -> game -> race -> focus manager.
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, WORLD + 0x100);
            w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
            w(c, WORLD + 0x200 + 0xa4, FOCUS);
            w(c, FOCUS + 0xd4, if seed % 3 == 0 { 0 } else { r.next_u32() });
            w(c, FOCUS + 0xd8, r.next_u32() % 8);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(kind);
            c.g[A3][0] = sx(NODE);
        });
    }

    /// `PS2GraphicsMan_v46`: formats 0..6 (paletted 1/2 with the palette at
    /// +0x40 or none, its data inline or at an offset; 3, 5 and others),
    /// texel data inline or at an offset; the renderer's upload method is a
    /// fake returning a small texture handle (sometimes unwinding); textures
    /// with no mips, mips but LOD scale 0, or a LOD scale (positive or
    /// negative) that runs the soft-float chain and the real
    /// PS2GraphicsMan_SetTextureFilter / TexMan_SetTex1Filter.
    #[test]
    fn ps2_graphics_man_v46_matches() {
        use crate::difftest::check;
        use crate::difftest::fakes::LOG_RESULTS;
        const GM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x300;
        const TEXMAN: u32 = DATA + 0x400;
        const ENTRY: u32 = DATA + 0x500;
        const TEX: u32 = DATA + 0x1000;
        check(ee::s12::f_0037c8c0__generated, ps2_graphics_man_v46, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x37_c8c0, 1);
            c.pc = 0x0037_c8c0;
            crate::difftest::elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            w(c, GM + V46_RENDERER, VT);
            c.bus.write16(VT + V46_UPLOAD_DELTA, (r.next_u32() % 8 * 4) as u16);
            w(c, VT + V46_UPLOAD_FN, FAKE);
            w(c, LOG_RESULTS, 1 + seed % 7);
            w(c, GM + 0x18f4, TEXMAN);
            for k in 0..8 {
                w(c, TEXMAN + 8 + 4 * k, ENTRY + 0x10 * k);
            }
            fill(c, &mut r, ENTRY, 0x40);
            // The texture and its palette.
            fill(c, &mut r, TEX, 0x20);
            let format = seed % 7;
            let palette_off = if seed % 3 == 0 { 0 } else { 0x40 };
            w(c, TEX, format | palette_off << 8);
            let mut flags = r.next_u32() & 0x0fff_ffff;
            if (seed / 7) % 3 != 0 {
                flags |= (1 + r.next_u32() % 15) << 28;
            }
            w(c, TEX + V46_TEX_FLAGS, flags);
            w(c, TEX + V46_TEX_DATA, 0x20 + 4 * (r.next_u32() % 8));
            let scale: i16 = match (seed / 21) % 4 {
                0 => 0,
                1 => -((1 + r.next_u32() % 512) as i16),
                _ => (1 + r.next_u32() % 4096) as i16,
            };
            c.bus.write16(TEX + V46_TEX_LOD_SCALE, scale as u16);
            w(c, TEX + 0x40 + V46_TEX_FLAGS, r.next_u32());
            w(c, TEX + 0x40 + V46_TEX_DATA, 0x20 + 4 * (r.next_u32() % 8));
            c.g[A0][0] = sx(GM);
            c.g[A1][0] = sx(TEX);
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
        });
    }
}
