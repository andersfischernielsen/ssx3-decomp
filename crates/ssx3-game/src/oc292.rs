//! Staging wave 211 (oc292): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::game::sound::soundsys::{
    boost_empty_store_entry, AUDIO_SYS_MONITOR, AUDIO_SYS_PUSH_BANK, AUDIO_SYS_PUSH_SAMPLE,
    AUDIO_SYS_SND_ROOT, MONITOR_PUSH_COUNT, MONITOR_PUSH_QUEUE,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0029_7950,
    ssx_audio_system_start_object_loop,
    "SSXAudioSystem_StartObjectLoop",
)];

// ---------------------------------------------------------------------------
// SSXAudioSystem_StartObjectLoop

/// `SSXAudioSystem` word that gates object loops: they start only while it
/// is negative.
const SOL_GATE: u32 = 0x62a8;
/// `SSXAudioSystem` object-loop slots: 30 records of 0x30 bytes.
const SOL_SLOTS: u32 = 0x5a00;
const SOL_SLOT_SIZE: u32 = 0x30;
const SOL_SLOT_COUNT: u32 = 30;
/// Slot fields: in use (1 = playing), the object, the bone (-1: the whole
/// object), the 3D position the push entry points at, the sound id and the
/// bank-monitor handle.
const SOL_ACTIVE: u32 = 0x0;
const SOL_OBJECT: u32 = 0x4;
const SOL_BONE: u32 = 0x8;
const SOL_POSITION: u32 = 0x10;
const SOL_SOUND: u32 = 0x20;
const SOL_HANDLE: u32 = 0x24;
/// The object's scene node (vtable at node + 0xc) and its fallback position.
const SOL_OBJ_NODE: u32 = 0xc;
const SOL_OBJ_POSITION: u32 = 0x40;
/// Node vtable slots ({delta: i16, pad, fn}): the node's transform, the
/// bone lookup, the bone's transform (all return a matrix; its
/// translation row is at +0x30).
const SOL_VT_TRANSFORM: u32 = 0xc0;
const SOL_VT_HAS_BONE: u32 = 0xd0;
const SOL_VT_BONE_TRANSFORM: u32 = 0xe8;
/// Sound ids with a track check: 0x66 only plays on tracks whose record
/// says so; 201 gets the long radius on track kind 8.
const SOL_TRACK_SOUND: u64 = 0x66;
const SOL_RADIUS_SOUND: u64 = 0xc9;
/// Push-queue tables of the bank monitor (`*sys->sndRoot`): the gain
/// pointers and the priorities, indexed by the push count.
const SOL_MONITOR_GAINS: u32 = 0x214;
const SOL_MONITOR_PRIORITIES: u32 = 0x250;
/// The push entry's position pointer array (`monitor + 0x1fc + 4 * count`).
const SOL_MONITOR_POSITIONS: u32 = 0x1fc;

const SOL_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};
const SOL_SET_RADIUS_VANISH: Callee = Callee {
    addr: 0x002a_9988,
    f: ee::s08::SndPlayParams_SetRadiusVanish,
};
/// `cBE_getBE`, `SSXAudioSystem_GetBELibrary`, `BELibrary_GetInterface`,
/// the current track record (`0x43d950 + 100 * track`), `memset`,
/// `SSXAudioSystem_GetChannelGainPtr`, `Audio_PlayPushedSound` (called
/// through the function table).
const SOL_GET_BE: u32 = 0x0014_dc80;
const SOL_GET_BE_LIBRARY: u32 = 0x0028_b1e8;
const SOL_GET_INTERFACE: u32 = 0x0014_dd58;
const SOL_TRACK_RECORD: u32 = 0x0014_4bc0;
const SOL_MEMSET: u32 = 0x0041_6210;
const SOL_GET_CHANNEL_GAIN: u32 = 0x0028_7968;
const SOL_PLAY_PUSHED: u32 = 0x0029_06b8;

/// `void SSXAudioSystem_StartObjectLoop(SSXAudioSystem* sys, Object* obj,
/// int sound, int bone)` (0x297950): start a looping 3D sound attached to
/// an object (or one of its bones). Sound 0 does nothing; sound 0x66 only
/// on tracks whose record allows it; nothing while the system's loop gate
/// (`+0x62a8`) is non-negative or the object has no scene node. A loop of
/// the same sound already playing on the object is stopped first. The loop
/// takes a free slot of 30 (none free: nothing), whose position comes from
/// the node's (or bone's) transform or the object's position; the sound id
/// selects bank 2 (< 100), 8 (< 150), 9 (< 200) or 4 with the sample index
/// relative to the bank's base. It is pushed on the bank monitor's queue
/// (volume 100, pan 90, 50, 127; radius 300, or 400 with priority 50 for
/// sound 201 on track kind 8, otherwise priority 80), played, and the
/// handle kept in the slot.
pub fn ssx_audio_system_start_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A1][0]; // obj
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0]; // sound
        frame.save(c, S5, 0x40);
        c.g[S3][0] = c.g[A0][0]; // sys
        frame.save(c, S0, 0x90);
        c.g[S5][0] = c.g[A3][0]; // bone
        frame.save(c, S4, 0x50);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0xa0);
        start_object_loop_body(c, frame)?;
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// Everything between prologue and epilogue; every way out restores s0
/// (the original reloads it in branch delay slots, or at 0x297e84).
fn start_object_loop_body(c: &mut Cpu, frame: Frame) -> Flow {
    if !start_object_loop_allowed(c, frame)? {
        return Ok(());
    }
    if !start_object_loop_claim_slot(c, frame)? {
        return Ok(());
    }
    start_object_loop_position(c)?;
    start_object_loop_bank(c);
    start_object_loop_radius(c)?;
    start_object_loop_push_and_play(c)?;
    c.cycles += 1;
    frame.restore(c, S0, 0x90);
    Ok(())
}

/// The early outs: sound 0, sound 0x66 on a track whose record word is 0,
/// a non-negative gate, an object without a scene node. Leaves a1 = 1.
fn start_object_loop_allowed(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    if c.g[S2][0] == 0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V0][0] = SOL_TRACK_SOUND;
    if c.g[S2][0] != c.g[V0][0] {
        c.g[V0][0] = Addr(r32(c, S3)).off(SOL_GATE).lw(c);
    } else {
        c.cycles += 10;
        call_ptr(c, SOL_GET_BE, 0x0029_79a4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, SOL_GET_INTERFACE, 0x0029_79b0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, SOL_TRACK_RECORD, 0x0029_79b8)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        frame.restore(c, S0, 0x90);
        if c.g[V1][0] == 0 {
            return Ok(false);
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, S3)).off(SOL_GATE).lw(c);
    }
    c.cycles += 2;
    frame.restore(c, S0, 0x90);
    if (c.g[V0][0] as i64) >= 0 {
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S1)).off(SOL_OBJ_NODE).lw(c);
    c.g[A1][0] = 1;
    Ok(c.g[V0][0] != 0)
}

/// Stop the first slot playing this sound on this object, then take the
/// first free slot (s4 = its index). False (s0 restored) when all 30 are
/// in use.
fn start_object_loop_claim_slot(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[S0][0] = c.g[S3][0];
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(SOL_SLOT_COUNT * SOL_SLOT_SIZE));
    c.g[V1][0] = c.g[S0][0];
    loop {
        // 0x2979e8: slot at v1 (= s0).
        c.cycles += 3;
        let slot = Addr(r32(c, V1)).off(SOL_SLOTS);
        c.g[V0][0] = slot.off(SOL_ACTIVE).lw(c);
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 3;
            c.g[V0][0] = slot.off(SOL_OBJECT).lw(c);
            if c.g[V0][0] == c.g[S1][0] {
                c.cycles += 3;
                c.g[V0][0] = slot.off(SOL_SOUND).lw(c);
                if c.g[V0][0] == c.g[S2][0] {
                    c.g[V0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
                    start_object_loop_stop_slot(c)?;
                    break;
                }
                c.cycles += 1;
            }
        }
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(SOL_SLOT_SIZE));
        // 0x297a10
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V1][0] = c.g[S0][0];
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0029_79e8);
        }
    }

    // 0x297a1c: the first free slot.
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S3)).off(SOL_SLOTS + SOL_ACTIVE).lw(c);
    c.g[S4][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = 1;
        Addr(r32(c, S3)).off(SOL_SLOTS + SOL_ACTIVE).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 1;
        c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
        loop {
            // 0x297a5c
            c.cycles += 3;
            let more = (c.g[S4][0] as i64) < SOL_SLOT_COUNT as i64;
            c.g[V0][0] = SOL_SLOT_SIZE as u64;
            if !more {
                break;
            }
            c.cycles += 5;
            c.mult(0, V1, S4, V0);
            c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
            c.g[V1][0] = Addr(r32(c, A0)).off(SOL_SLOTS + SOL_ACTIVE).lw(c);
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 1;
                Addr(r32(c, A0)).off(SOL_SLOTS + SOL_ACTIVE).sw(c, c.g[V0][0]);
                break;
            }
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x0029_7a5c);
            }
        }
    }

    // 0x297a84
    c.cycles += 3;
    let full = c.g[S4][0] == SOL_SLOT_COUNT as u64;
    c.g[V0][0] = SOL_SLOT_SIZE as u64;
    if full {
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        return Ok(false);
    }
    Ok(true)
}

/// 0x297a34: `BankMonitor_StopHandle(&(*sys->sndRoot)->monitor,
/// slot->handle, 0.0f, 1)` and free the slot (s0 = slot - 0x5a00, v0 =
/// sys->sndRoot).
fn start_object_loop_stop_slot(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[A2][0] = 1;
    c.g[A1][0] = Addr(r32(c, S0)).off(SOL_SLOTS + SOL_HANDLE).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    c.f[12] = 0; // fade time 0.0
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MONITOR_PUSH_QUEUE));
    SOL_STOP_HANDLE.call(c, 0x0029_7a4c)?;
    Addr(r32(c, S0)).off(SOL_SLOTS + SOL_ACTIVE).sw(c, 0);
    if event_due(c) {
        deliver_event_here(c, 0x0029_7a1c);
    }
    Ok(())
}

/// `node->vtable[slot](node + delta, ...)` with the node (`obj->node`) in
/// `node_reg`; returns to `ret`.
fn start_object_loop_node_call(c: &mut Cpu, node_reg: usize, slot: u32, ret: u32) -> Flow {
    c.g[node_reg][0] = Addr(r32(c, S1)).off(SOL_OBJ_NODE).lw(c);
    c.g[V1][0] = Addr(r32(c, node_reg)).off(0xc).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).off(slot).lh(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(slot + 4).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, node_reg).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// Fill the slot (s0 = sys + 0x30 * s4): the object, and the position
/// (also kept at sp+0): the node's translation for bone -1, else the
/// bone's translation, or the object's position when the node has no such
/// bone.
fn start_object_loop_position(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[V1][0] = u64::MAX;
    c.mult(0, A0, S4, V0);
    c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
    let whole_object = c.g[S5][0] == c.g[V1][0];
    Addr(r32(c, S0)).off(SOL_SLOTS + SOL_OBJECT).sw(c, c.g[S1][0]);
    let reg = if whole_object {
        c.cycles += 8;
        start_object_loop_node_call(c, A1, SOL_VT_TRANSFORM, 0x0029_7abc)?;
        c.lq(V1, r32(c, V0).wrapping_add(0x30));
        V1
    } else {
        c.cycles += 8;
        start_object_loop_node_call(c, A1, SOL_VT_HAS_BONE, 0x0029_7adc)?;
        let has_bone = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[S5][0];
        if has_bone {
            c.cycles += 7;
            start_object_loop_node_call(c, A2, SOL_VT_BONE_TRANSFORM, 0x0029_7afc)?;
            c.lq(V1, r32(c, V0).wrapping_add(0x30));
            V1
        } else {
            c.cycles += 3;
            c.lq(V0, r32(c, S1).wrapping_add(SOL_OBJ_POSITION));
            c.sq(V0, r32(c, S0).wrapping_add(SOL_SLOTS + SOL_POSITION));
            c.sq(V0, sp(c));
            return Ok(());
        }
    };
    c.cycles += 3;
    c.sq(reg, r32(c, S0).wrapping_add(SOL_SLOTS + SOL_POSITION));
    c.sq(reg, sp(c));
    Ok(())
}

/// Keep the bone in the slot; the bank (s6) and sample index (s5) from the
/// sound id: < 100 bank 2, < 150 bank 8, < 200 bank 9, else bank 4.
fn start_object_loop_bank(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = SOL_SLOT_SIZE as u64;
    c.g[V1][0] = ((c.g[S2][0] as i64) < 100) as u64;
    c.mult(0, A0, S4, V0);
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
    Addr(r32(c, V0)).off(SOL_SLOTS + SOL_BONE).sw(c, c.g[S5][0]);
    let sound = r32(c, S2);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[S6][0] = 2;
        c.g[S5][0] = c.g[S2][0];
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 150) as u64;
    c.g[S6][0] = 8;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[S5][0] = sx(sound.wrapping_sub(100));
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 200) as u64;
    c.g[S6][0] = 9;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[S5][0] = sx(sound.wrapping_sub(150));
        return;
    }
    c.cycles += 2;
    c.g[S6][0] = 4;
    c.g[S5][0] = sx(sound.wrapping_sub(200));
}

/// Keep the sound id in the slot; radius (f20) 300 and priority (s7) 80,
/// or 400 and 50 for sound 201 on a track whose record word is 8. Leaves
/// s0 = sys->monitor.
fn start_object_loop_radius(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[V0][0] = SOL_SLOT_SIZE as u64;
    c.g[AT][0] = 0x4396_0000; // 300.0f
    c.mtc1(AT, 20);
    c.mult(0, V1, S4, V0);
    c.g[S7][0] = 0x50;
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
    c.g[V1][0] = SOL_RADIUS_SOUND;
    Addr(r32(c, V0)).off(SOL_SLOTS + SOL_SOUND).sw(c, c.g[S2][0]);
    if c.g[S2][0] == c.g[V1][0] {
        c.cycles += 11;
        call_ptr(c, SOL_GET_BE_LIBRARY, 0x0029_7b98)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, SOL_GET_INTERFACE, 0x0029_7ba4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, SOL_TRACK_RECORD, 0x0029_7bac)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V0][0] = 8;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[S0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_MONITOR).lw(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[AT][0] = 0x43c8_0000; // 400.0f
        c.mtc1(AT, 20);
        c.g[S7][0] = 0x32;
    }
    c.cycles += 1;
    c.g[S0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_MONITOR).lw(c);
    Ok(())
}

/// 0x297bcc: push a new entry on the monitor's queue (s0 = monitor, s1 =
/// its queue at +0x1d8): count += 1, copy the 24-byte template into the
/// entry's record, clear its fields, volume 100 / pan 90, 50 / 127 / 1,
/// bank s6 and sample s5 in the system's push tables, the channel gain
/// pointer, the slot's position, radius f20 (vanish -1), priority s7; then
/// `Audio_PlayPushedSound(sys)` and keep the handle in the slot.
fn start_object_loop_push_and_play(c: &mut Cpu) -> Flow {
    c.cycles += 174;
    let monitor = r32(c, S0);
    c.g[T1][0] = 0x18; // record size
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x8;
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(MONITOR_PUSH_QUEUE));
    let queue = r32(c, S1);
    c.g[S2][0] = SOL_SLOT_SIZE as u64;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    Addr(monitor).off(MONITOR_PUSH_COUNT).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = Addr(queue).off(0x20).lw(c);
    c.mult(0, V1, V1, T1);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.ldl(A0, monitor.wrapping_add(0x1e3));
    c.ldr(A0, monitor.wrapping_add(0x1dc));
    c.ldl(A3, monitor.wrapping_add(0x1eb));
    c.ldr(A3, monitor.wrapping_add(0x1e4));
    c.ldl(T0, monitor.wrapping_add(0x1f3));
    c.ldr(T0, monitor.wrapping_add(0x1ec));
    let record = r32(c, V1);
    c.sdl(A0, record.wrapping_add(0x7));
    c.sdr(A0, record);
    c.sdl(A3, record.wrapping_add(0xf));
    c.sdr(A3, record.wrapping_add(0x8));
    c.sdl(T0, record.wrapping_add(0x17));
    c.sdr(T0, record.wrapping_add(0x10));
    // monitor->positions[count] = 0 (set below).
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(r32(c, V0)));
    Addr(r32(c, V0)).off(SOL_MONITOR_POSITIONS).sw(c, 0);
    boost_empty_store_entry(c, (V1, V0), 0x34, 2, 0, 0);
    // table 0x38 [count] = the first byte of the entry's record.
    c.g[A3][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[T0][0] = Addr(queue).off(0x20).lw(c);
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, A0, A0, T1);
    c.g[V0][0] = Addr(queue).off(0x38).lw(c);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = Addr(r32(c, A0)).lb(c);
    Addr(r32(c, A3)).sw(c, c.g[V1][0]);
    boost_empty_store_entry(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8)
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(queue.wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, SOL_MEMSET, 0x0029_7ca0)?;

    // The entry's parameters.
    c.g[A3][0] = 0x64; // volume
    c.g[T0][0] = 0x5a;
    c.g[T1][0] = 0x32;
    boost_empty_store_entry(c, (V1, V0), 0x60, 2, 0, 0);
    c.g[T2][0] = 0x7f;
    c.g[T3][0] = 0x1;
    c.g[A1][0] = 0x5; // channel kind of GetChannelGainPtr
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S3][0];
    boost_empty_store_entry(c, (V1, V0), 0x64, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x68, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x6c, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x4, 0);
    let (a3, t0, t1, t2, t3) = (c.g[A3][0], c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0x0, a3);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x8, t0);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0xc, t1);
    boost_empty_store_entry(c, (V0, V1), 0x74, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x80, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x84, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x78, 2, 0, t2);
    boost_empty_store_entry(c, (V0, V1), 0x7c, 2, 0, t3);

    // sys->pushBank[count] = bank (s6); sys->pushSample[count] = sample (s5).
    for (field, reg) in [(AUDIO_SYS_PUSH_BANK, S6), (AUDIO_SYS_PUSH_SAMPLE, S5)] {
        let sys = r32(c, S3);
        c.g[V1][0] = Addr(sys).off(AUDIO_SYS_MONITOR).lw(c);
        c.g[A3][0] = Addr(sys).off(field).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(MONITOR_PUSH_COUNT).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
        Addr(r32(c, V0)).sw(c, c.g[reg][0]);
    }

    // gains[count] = SSXAudioSystem_GetChannelGainPtr(sys, 5, 0);
    // positions[count] = &slot->position.
    c.g[S0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    call_ptr(c, SOL_GET_CHANNEL_GAIN, 0x0029_7dfc)?;
    c.mult(0, V1, S4, S2);
    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
    c.mtc1(AT, 13);
    c.mov_s(12, 20);
    let root_monitor = r32(c, A0);
    c.g[A1][0] = Addr(root_monitor).off(SOL_MONITOR_GAINS).lw(c);
    c.g[A2][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
    c.g[V1][0] = Addr(root_monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(SOL_SLOTS + SOL_POSITION));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    Addr(r32(c, V1)).off(SOL_MONITOR_POSITIONS).sw(c, c.g[A2][0]);

    // SndPlayParams_SetRadiusVanish(&monitor->queue, radius, -1.0f)
    c.g[V0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MONITOR_PUSH_QUEUE));
    SOL_SET_RADIUS_VANISH.call(c, 0x0029_7e54)?;

    // priorities[count] = s7; handle = Audio_PlayPushedSound(sys).
    c.g[A1][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.g[A0][0] = c.g[S3][0];
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A1][0] = Addr(r32(c, V1)).off(SOL_MONITOR_PRIORITIES).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    Addr(r32(c, V0)).sw(c, c.g[S7][0]);
    call_ptr(c, SOL_PLAY_PUSHED, 0x0029_7e78)?;
    c.mult(0, V1, S4, S2);
    c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
    Addr(r32(c, S2)).off(SOL_SLOTS + SOL_HANDLE).sw(c, c.g[V0][0]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `SSXAudioSystem_StartObjectLoop` on the audio world fixture of the
    /// soundsys tests (real memset, GetChannelGainPtr, SetRadiusVanish and
    /// `Audio_PlayPushedSound` with no bank loaded; the real
    /// `BankMonitor_StopHandle` finds no instance). Sound ids 0, 0x66,
    /// 201, every bank range and a negative one; track record word 0, 3
    /// or 8 (the real cBE_getBE / GetBELibrary and track-record getter, a
    /// fake GetInterface method); the gate non-negative; no scene node;
    /// slots all free, all in use, mixed, with or without a slot already
    /// playing this sound on this object; bone -1 (node transform) or a
    /// bone the node has (fake) or lacks (a real leaf returning 0: the
    /// object's position).
    #[test]
    fn ssx_audio_system_start_object_loop_matches() {
        use crate::difftest::fakes::{vtable, FAKE};
        use crate::difftest::{check_lohi, fill, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::audio;
        const OBJ: u32 = DATA + 0x2a_0000;
        const OTHER_OBJ: u32 = DATA + 0x2a_0800;
        const NODE: u32 = DATA + 0x2a_1000;
        const INSTANCES: u32 = DATA + 0x2b_0000;
        const GAINS: u32 = DATA + 0x2c_0000;
        const PRIORITIES: u32 = DATA + 0x2c_1000;
        const TRACK_RECORD: u32 = 0x0043_d950;
        const RETURNS_ZERO: u32 = 0x0014_06c0;
        const SOUNDS: [i32; 13] = [0, 0x66, 0x66, 5, 99, 100, 149, 150, 199, 200, 201, 250, -3];
        check_lohi(
            ee::s08::f_00297950__generated,
            ssx_audio_system_start_object_loop,
            260,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_7950, 0x0029_7950);
                let sys = audio::SYS;
                // BELibrary_GetInterface: interface 0 with a fake method;
                // the current track (0) and its record word.
                let iface = DATA + 0x26_1000;
                let ivt = DATA + 0x26_1100;
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, ivt);
                c.bus.write16(ivt + 8, 0);
                c.bus.write32(ivt + 0xc, FAKE + 0x40);
                c.bus.write32(0x0053_5c08, 0);
                c.bus
                    .write32(TRACK_RECORD, [0u32, 8, 3, 8][(r.next_u32() % 4) as usize]);
                // The monitor's gain / priority tables and instances (free).
                c.bus.write32(audio::MONITOR + SOL_MONITOR_GAINS, GAINS);
                c.bus
                    .write32(audio::MONITOR + SOL_MONITOR_PRIORITIES, PRIORITIES);
                c.bus
                    .write32(audio::MONITOR + MONITOR_PUSH_QUEUE + 0x8ec, INSTANCES);
                // The gate.
                let gate = if seed % 13 == 6 {
                    r.next_u32() & 0x7fff_ffff
                } else {
                    0x8000_0000 | r.next_u32()
                };
                c.bus.write32(sys + SOL_GATE, gate);
                // The object, its node and the node's vtable.
                let sound = SOUNDS[(r.next_u32() % SOUNDS.len() as u32) as usize];
                let node = if seed % 17 == 9 { 0 } else { NODE };
                c.bus.write32(OBJ + SOL_OBJ_NODE, node);
                fill(c, &mut r, OBJ + SOL_OBJ_POSITION, 4);
                let nvt = vtable(
                    c,
                    &mut r,
                    &[SOL_VT_TRANSFORM, SOL_VT_HAS_BONE, SOL_VT_BONE_TRANSFORM],
                );
                c.bus.write32(NODE + 0xc, nvt);
                if r.next_u32() % 3 == 0 {
                    c.bus.write32(nvt + SOL_VT_HAS_BONE + 4, RETURNS_ZERO);
                }
                let bone = if r.next_u32() % 3 == 0 {
                    -1i32
                } else {
                    (r.next_u32() % 4) as i32
                };
                // The slots.
                let mode = r.next_u32() % 4;
                let same_sound_odds = if r.next_u32() % 2 == 0 { 3 } else { 0 };
                for k in 0..SOL_SLOT_COUNT {
                    let slot = sys + SOL_SLOTS + k * SOL_SLOT_SIZE;
                    let active = match mode {
                        0 => 0,
                        1 => 1 + r.next_u32() % 2,
                        _ => r.next_u32() % 3,
                    };
                    let object = if r.next_u32() % 2 == 0 { OBJ } else { OTHER_OBJ };
                    let id = if same_sound_odds != 0 && r.next_u32() % same_sound_odds == 0 {
                        sound as u32
                    } else {
                        0x1000 + r.next_u32() % 300
                    };
                    c.bus.write32(slot + SOL_ACTIVE, active);
                    c.bus.write32(slot + SOL_OBJECT, object);
                    c.bus.write32(slot + SOL_SOUND, id);
                    c.bus.write32(slot + SOL_HANDLE, r.next_u32() % 0x100);
                }
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(OBJ);
                c.g[A2][0] = sx(sound as u32);
                c.g[A3][0] = sx(bone as u32);
            },
        );
    }
}
