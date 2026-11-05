//! Staging wave 206 (oc251): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

// ---------------------------------------------------------------------------
// VfxTube_UpdateColorTable (0x3725b0)

/// gp-relative globals of the tube colour table: the wanted colour as ints
/// (r, g, b) and the colour last written to the table (floats).
const TUBE_COLOR_INT: u32 = 0x10f0;
const TUBE_COLOR_LAST: u32 = 0x1140;
/// The tube vertex table (0x106 vertices of 0x30 bytes) and its colour
/// fields.
const TUBE_VERTS: u32 = 0x0050_1440;
const TUBE_VERT_SIZE: u32 = 0x30;
const TUBE_VERTS_END: u32 = TUBE_VERTS + 0x106 * TUBE_VERT_SIZE;
const TUBE_VERT_R: u32 = 0x10;

/// `void VfxTube_UpdateColorTable()` (0x3725b0): when the tube colour (three
/// ints at gp+0x10f0) differs from the floats last applied (gp+0x1140),
/// writes the colour into all 0x106 tube vertices and remembers it.
pub fn vfx_tube_update_color_table(c: &mut Cpu) {
    let gp = r32(c, GP);
    let int_at = |i: u32| Addr(gp.wrapping_add(TUBE_COLOR_INT + 4 * i));
    let last_at = |i: u32| Addr(gp.wrapping_add(TUBE_COLOR_LAST + 4 * i));

    // Compare channel by channel (the next "last" value is loaded in the
    // branch delay slot, so f0 always holds one channel ahead).
    c.cycles += 7;
    let mut same = true;
    for i in 0..3u32 {
        if i == 1 {
            c.cycles += 6;
        } else if i == 2 {
            c.cycles += 5;
        }
        let want = FpuF32::from_i32(int_at(i).lw(c) as i32);
        c.f[1] = want.0;
        let last = FpuF32(c.f[0]);
        let last = if i == 0 { last_at(0).read_fpu(c) } else { last };
        let eq = last.eq_s(want);
        abi::set_fcc(c, eq);
        if i < 2 {
            c.f[0] = last_at(i + 1).read_fpu(c).0;
        }
        if !eq {
            same = false;
            break;
        }
    }

    if !same {
        c.cycles += 8;
        c.g[V0][0] = 0x50_0000;
        let r = int_at(0).lw(c);
        let g = int_at(1).lw(c);
        let b = int_at(2).lw(c);
        c.g[A2][0] = r;
        c.g[A1][0] = g;
        c.g[A0][0] = b;
        c.g[A3][0] = sx(TUBE_VERTS_END);
        let mut v = TUBE_VERTS;
        Addr(v + TUBE_VERT_R).sw(c, r);
        loop {
            c.cycles += 7;
            Addr(v + TUBE_VERT_R + 4).sw(c, g);
            Addr(v + TUBE_VERT_R + 8).sw(c, b);
            v += TUBE_VERT_SIZE;
            if v < TUBE_VERTS_END {
                Addr(v + TUBE_VERT_R).sw(c, r);
                if event_due(c) {
                    c.g[V1][0] = sx(v);
                    c.g[V0][0] = 1;
                    deliver_event_here(c, 0x0037_2618);
                }
                continue;
            }
            break;
        }
        c.g[V1][0] = sx(v);
        c.g[V0][0] = 0;

        c.cycles += 9;
        for i in 0..3u32 {
            let f = FpuF32::from_i32(int_at(i).lw(c) as i32);
            c.f[i as usize] = f.0;
        }
        for i in 0..3u32 {
            let f = FpuF32(c.f[i as usize]);
            last_at(i).write_fpu(c, f);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// Rider_IsOnGround_Wrapper (0x122cd0)

/// `Rider_IsOnGround` (0x1231a8).
const RIDER_IS_ON_GROUND: Callee = Callee {
    addr: 0x0012_31a8,
    f: ee::s00::Rider_IsOnGround,
};

/// `bool Rider_IsOnGround_Wrapper(Rider *rider)` (0x122cd0): forwards to
/// `Rider_IsOnGround`.
pub fn rider_is_on_ground_wrapper(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(RIDER_IS_ON_GROUND),
        0x0012_2ce0,
    );
}

// ---------------------------------------------------------------------------
// StreamBuffer_UpdateVolume (0x2a9f80)

/// StreamBuffer fields: the SNDSTRM stream handle, the volume byte, the
/// pointer to the volume scale (a float) and the starvation counter.
const SB_HANDLE: u32 = 0x8;
const SB_VOLUME: u32 = 0xc;
const SB_SCALE_PTR: u32 = 0x10;
const SB_STARVED: u32 = 0x80;

/// `StreamBuffer_Stop` (0x2aa408).
const STREAM_BUFFER_STOP: Callee = Callee {
    addr: 0x002a_a408,
    f: ee::s08::StreamBuffer_Stop,
};

/// `void StreamBuffer_UpdateVolume(StreamBuffer *sb)` (0x2a9f80): stops the
/// stream when its starvation counter (+0x80) has passed two seconds of
/// audio frames (and clears it); otherwise, with a volume scale (+0x10),
/// sets the stream's volume to `volume byte * scale` clamped to 0..127,
/// under the sound lock.
pub fn stream_buffer_update_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0028_b240, 0x002a_9f98)?; // Audio_GetFrameRate

        let sb = Addr(r32(c, S1));
        let starved = sb.off(SB_STARVED).lw(c);
        c.g[V1][0] = starved;
        let limit = sx(r32(c, V0) << 1);
        c.g[V0][0] = ((limit as i64) < (starved as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(sb.0);
            STREAM_BUFFER_STOP.call(c, 0x002a_9fb4)?;
            Addr(r32(c, S1)).off(SB_STARVED).sw(c, 0);
        } else {
            let scale_ptr = sb.off(SB_SCALE_PTR).lw(c);
            c.g[V1][0] = scale_ptr;
            c.cycles += 2;
            c.g[A0][0] = 0x7f;
            if scale_ptr != 0 {
                c.cycles += 18;
                let volume = sb.off(SB_VOLUME).lb(c);
                c.g[V0][0] = volume;
                let scale = Addr(scale_ptr as u32).read_fpu(c);
                c.f[1] = scale.0;
                let vol_f = FpuF32::from_i32(volume as i32);
                let prod = vol_f * scale;
                c.f[0] = prod.0;
                let mut v = prod.to_i32();
                c.f[1] = v as u32;
                let below = v < 128;
                c.g[V0][0] = below as u64;
                if !below {
                    v = 0x7f;
                }
                let nonneg = -1 < v;
                c.g[V1][0] = nonneg as u64;
                if !nonneg {
                    v = 0;
                }
                c.g[S0][0] = sx(v as u32);
                call_ptr(c, 0x003b_58a0, 0x002a_9ff8)?; // SND_Lock
                c.g[A0][0] = Addr(r32(c, S1)).off(SB_HANDLE).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003b_7e70, 0x002a_a004)?; // SNDSTRM_SetVolume
                call_ptr(c, 0x003b_58d8, 0x002a_a00c)?; // SND_Unlock
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputBinding_EvalBool (0x320c48)

/// `InputMap_EvalBool` (0x321108).
const INPUT_MAP_EVAL_BOOL: Callee = Callee {
    addr: 0x0032_1108,
    f: ee::s10::InputMap_EvalBool,
};

/// `bool InputBinding_EvalBool(InputBinding *binding, int index)`
/// (0x320c48): evaluates the binding's input map (+4) as a bool, with the
/// binding's first word and `index` as the map's arguments.
pub fn input_binding_eval_bool(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let binding = Addr(r32(c, A0));
        c.g[A1][0] = binding.lw(c);
        c.g[A0][0] = binding.off(0x4).lw(c);
        INPUT_MAP_EVAL_BOOL.call(c, 0x0032_0c60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_GetVolume (0x3b86c0)

/// `SND_HandleToChannel` (0x3ba6e8).
const SND_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};
/// The pointer to the sound channel parameter array (0x8c bytes per
/// channel) and the channel's volume (s16).
const SND_CHANNEL_PARAMS: u32 = 0x0050_aad8;
const SND_CHANNEL_PARAM_SIZE: u32 = 0x8c;
const SND_CHANNEL_VOLUME: u32 = 0x3a;

/// `short SND_GetVolume(int handle)` (0x3b86c0): the volume of the channel
/// playing `handle`, or -8 when the handle is not playing.
pub fn snd_get_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        SND_HANDLE_TO_CHANNEL.call(c, 0x003b_86d0)?;
        let channel = c.g[V0][0];
        c.g[V1][0] = SND_CHANNEL_PARAM_SIZE as u64;
        if (channel as i64) < 0 {
            c.cycles += 1;
            c.g[V0][0] = (-8i64) as u64;
        } else {
            c.cycles += 6;
            c.g[A0][0] = 0x51_0000;
            let off = abi::mult(c, channel as u32, SND_CHANNEL_PARAM_SIZE);
            let params = Addr(SND_CHANNEL_PARAMS).lw(c);
            let entry = (params as u32).wrapping_add(off);
            c.g[V1][0] = sx(entry);
            c.g[V0][0] = Addr(entry).off(SND_CHANNEL_VOLUME).lh(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdMan3d_Update (0x3440c8)

/// `CrowdMan3d_GroupUpdate` (0x3449f0).
const CROWD_MAN3D_GROUP_UPDATE: Callee = Callee {
    addr: 0x0034_49f0,
    f: ee::s11::CrowdMan3d_GroupUpdate,
};
/// The crowd manager's five groups (0x1f4 bytes each); a group whose id
/// (+8) is -1 is unused.
const CROWD_GROUP_SIZE: u32 = 0x1f4;
const CROWD_GROUP_ID: u32 = 0x8;

/// `void CrowdMan3d_Update(CrowdMan3d *man)` (0x3440c8): updates each of
/// the five crowd groups that is in use.
pub fn crowd_man3d_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S2][0] = u64::MAX;
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(r32(c, S0).wrapping_add(CROWD_GROUP_ID));
        c.g[S3][0] = 4;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            if c.g[V0][0] != c.g[S2][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                CROWD_MAN3D_GROUP_UPDATE.call(c, 0x0034_410c)?;
            }
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(CROWD_GROUP_SIZE));
            c.cycles += 3;
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(CROWD_GROUP_SIZE));
            if (c.g[S3][0] as i64) >= 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_40f8);
                }
                continue;
            }
            break;
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

// ---------------------------------------------------------------------------
// PATH_ServiceQueues (0x3d1e80)

/// `PATH_RemoveEvent` (0x3d1ba8) and `PATH_ServiceQueue` (0x3d1d28).
const PATH_REMOVE_EVENT: Callee = Callee {
    addr: 0x003d_1ba8,
    f: ee::s14::PATH_RemoveEvent,
};
const PATH_SERVICE_QUEUE: Callee = Callee {
    addr: 0x003d_1d28,
    f: ee::s14::PATH_ServiceQueue,
};
/// The PATH state pointer and the PATH clock (ms), both 0x450000-relative.
const PATH_STATE_OFF: u32 = 0xbe0;
const PATH_TIME_OFF: u32 = 0xbc8;
/// PATH state: the event queue heads (16) and their count.
const PATH_QUEUES: u32 = 0xdc;
const PATH_QUEUE_COUNT: u32 = 0x91c;
const PATH_MAX_QUEUES: i64 = 16;
/// Queue head: start time, lifetime (0 = forever), packed action indices
/// (bits 16..21 = end, 22..27 = next).
const QUEUE_START: u32 = 0x0;
const QUEUE_LIFETIME: u32 = 0x4;
const QUEUE_INDICES: u32 = 0xc;
/// Resume points (dispatcher loop heads).
const SERVICE_QUEUES_ENTRY: u32 = 0x003d_1e80;
const SERVICE_QUEUES_NEXT: u32 = 0x003d_1f1c;
const SERVICE_QUEUES_QUEUE: u32 = 0x003d_1ec0;

/// The stages of PATH_ServiceQueues' loop (the original's block heads).
#[derive(Clone, Copy)]
enum ServiceStage {
    /// 0x3d1f20: past the last queue slot? (s0 = index)
    Check,
    /// 0x3d1ec0: one live queue (a0 = head, v0 = its packed indices).
    Queue,
    /// 0x3d1f1c: index + 1.
    Next,
}

/// `int PATH_ServiceQueues()` (0x3d1e80): runs every PATH event queue: a
/// queue with actions left (and not expired) is serviced, one that is done
/// or expired is removed (the later ones shift down); slots past the queue
/// count are cleared. Returns the number of actions run. Dispatcher-shaped:
/// an unwinding event at a back edge returns with `pc` at the loop head
/// (0x3d1f1c or 0x3d1ec0), where the trampoline re-enters it with the loop
/// state in s0..s5 (and a0/v0).
pub fn path_service_queues(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        let mut stage = match c.pc {
            SERVICE_QUEUES_ENTRY => {
                c.cycles += 15;
                Frame::open(c, 0x70);
                frame.save(c, S0, 0x60);
                frame.save(c, S1, 0x50);
                c.g[S0][0] = 0; // queue index
                frame.save(c, S2, 0x40);
                c.g[S1][0] = 0; // actions run
                frame.save(c, S3, 0x30);
                c.g[S2][0] = 1;
                frame.save(c, S4, 0x20);
                c.g[S3][0] = 0x45_0000;
                frame.save(c, S5, 0x10);
                c.g[S4][0] = 3;
                frame.save_ra(c, 0);
                c.g[S5][0] = 0x45_0000;
                ServiceStage::Check
            }
            SERVICE_QUEUES_NEXT => ServiceStage::Next,
            SERVICE_QUEUES_QUEUE => ServiceStage::Queue,
            pc => {
                c.jump_out(pc, SERVICE_QUEUES_ENTRY, 0x003d_1f80);
                return Ok(());
            }
        };
        loop {
            match stage {
                ServiceStage::Check => {
                    c.cycles += 3;
                    let index = c.g[S0][0] as i64;
                    c.g[V0][0] = (index < PATH_MAX_QUEUES) as u64;
                    let state = Addr(r32(c, S5).wrapping_sub(PATH_STATE_OFF)).lw(c);
                    c.g[V1][0] = state;
                    if index >= PATH_MAX_QUEUES {
                        break;
                    }
                    c.cycles += 4;
                    let count = Addr((state as u32).wrapping_add(PATH_QUEUE_COUNT)).lw(c);
                    let live = index < count as i64;
                    let slot = (state as u32).wrapping_add((index as u32) << 2);
                    c.g[V0][0] = sx(slot);
                    if !live {
                        // A slot past the count: clear it.
                        c.cycles += 3;
                        Addr(slot).off(PATH_QUEUES).sw(c, 0);
                        if crate::events::back_edge_unwinds(c, SERVICE_QUEUES_NEXT) {
                            return Err(crate::call::Unwound);
                        }
                        stage = ServiceStage::Next;
                        continue;
                    }
                    c.cycles += 4;
                    let head = Addr(slot).off(PATH_QUEUES).lw(c);
                    c.g[A0][0] = head;
                    if head == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(head as u32).off(QUEUE_INDICES).lw(c);
                    if crate::events::back_edge_unwinds(c, SERVICE_QUEUES_QUEUE) {
                        return Err(crate::call::Unwound);
                    }
                    stage = ServiceStage::Queue;
                }
                ServiceStage::Queue => {
                    // Remove (a1 != 0) when no actions are left, or when the
                    // lifetime ran out; else service.
                    c.cycles += 9;
                    let head = Addr(r32(c, A0));
                    let mut remove = c.g[S4][0];
                    let lifetime = head.off(QUEUE_LIFETIME).lw(c);
                    c.g[A2][0] = lifetime;
                    let indices = r32(c, V0);
                    let end = sx(indices >> 16) & 0x3f;
                    let next = sx(indices >> 22) & 0x3f;
                    c.g[V1][0] = end;
                    let pending = next < end;
                    c.g[V0][0] = pending as u64;
                    if pending {
                        remove = 0;
                    }
                    if lifetime != 0 {
                        c.cycles += 5;
                        let start = head.off(QUEUE_START).lw(c);
                        let now = Addr(r32(c, S3).wrapping_sub(PATH_TIME_OFF)).lw(c);
                        c.g[V1][0] = now;
                        let expiry = sx((start as u32).wrapping_add(lifetime as u32));
                        let expired = expiry < now;
                        c.g[V0][0] = expired as u64;
                        if expired {
                            remove = c.g[S2][0];
                        }
                    }
                    c.g[A1][0] = remove;
                    c.cycles += 2;
                    if remove != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        PATH_REMOVE_EVENT.call(c, 0x003d_1f08)?;
                        // The later queues moved down: stay on this index.
                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        PATH_SERVICE_QUEUE.call(c, 0x003d_1f18)?;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                    }
                    stage = ServiceStage::Next;
                }
                ServiceStage::Next => {
                    c.cycles += 1;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    stage = ServiceStage::Check;
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0037_25b0, vfx_tube_update_color_table, "VfxTube_UpdateColorTable"),
    (0x0012_2cd0, rider_is_on_ground_wrapper, "Rider_IsOnGround_Wrapper"),
    (0x002a_9f80, stream_buffer_update_volume, "StreamBuffer_UpdateVolume"),
    (0x0032_0c48, input_binding_eval_bool, "InputBinding_EvalBool"),
    (0x003b_86c0, snd_get_volume, "SND_GetVolume"),
    (0x0034_40c8, crowd_man3d_update, "CrowdMan3d_Update"),
    (0x003d_1e80, path_service_queues, "PATH_ServiceQueues"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// VfxTube_UpdateColorTable: colour unchanged (each channel equal up to
    /// the first differing one) or changed (the whole table rewritten).
    #[test]
    fn vfx_tube_update_color_table_matches() {
        check(ee::s12::f_003725b0__generated, vfx_tube_update_color_table, 200, &|c, seed| {
            let mut r = Rng(0x3725_b0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0037_25b0;
            let gp = GAME_GP;
            // Ints, small enough to be exact as floats.
            let ints = [r.next_u32() % 256, r.next_u32() % 256, r.next_u32() % 256];
            // How many leading channels already match (3 = no update).
            let matching = seed % 4;
            for i in 0..3u32 {
                c.bus.write32(gp + TUBE_COLOR_INT + 4 * i, ints[i as usize]);
                let last = if i < matching {
                    ints[i as usize] as f32
                } else {
                    ints[i as usize] as f32 + 1.0
                };
                c.bus.write32(gp + TUBE_COLOR_LAST + 4 * i, last.to_bits());
            }
        });
    }

    /// Rider_IsOnGround_Wrapper through the real `Rider_IsOnGround`: main
    /// state 0 (ground), 2 with/without the sub-state flag, or another state.
    #[test]
    fn rider_is_on_ground_wrapper_matches() {
        check(ee::s00::f_00122cd0__generated, rider_is_on_ground_wrapper, 200, &|c, seed| {
            let mut r = Rng(0x122c_d000 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0012_2cd0;
            let rider = DATA;
            let base = DATA + 0x2000;
            c.g[A0][0] = sx(rider);
            c.bus.write32(rider + 0x77c, base);
            let state = [0, 2, 2, 5][(seed % 4) as usize];
            c.bus.write32(base + 0xde0, state);
            c.bus.write32(base + 0x30, if seed % 8 < 4 { 0 } else { r.next_u32() | 1 });
        });
    }

    /// StreamBuffer_UpdateVolume: starved (stop and clear), no scale
    /// pointer, or a volume set through the real SND lock / SNDSTRM path
    /// (fake lock hooks) with products below 0, inside and above 0..127.
    #[test]
    fn stream_buffer_update_volume_matches() {
        check(ee::s08::f_002a9f80__generated, stream_buffer_update_volume, 240, &|c, seed| {
            let mut r = crate::difftest::fakes::machine(c, seed, 0x2a9f80, 0);
            c.pc = 0x002a_9f80;
            let sb = DATA + 0x100;
            let scale = DATA + 0x400;
            c.g[A0][0] = sx(sb);
            c.bus.write32(sb + SB_HANDLE, r.next_u32() % 8);
            c.bus.write8(sb + SB_VOLUME, r.next_u32() as u8);
            // Audio_GetFrameRate is 60: the limit is 120.
            let starved = match seed % 4 {
                0 => 121 + r.next_u32() % 100,
                1 => 120,
                _ => r.next_u32() % 120,
            };
            c.bus.write32(sb + SB_STARVED, starved);
            c.bus.write32(sb + SB_SCALE_PTR, if seed % 5 == 1 { 0 } else { scale });
            let f = match seed % 3 {
                0 => r.f32(4.0),
                1 => r.edge_f32(),
                _ => r.f32(1.0),
            };
            c.bus.write32(scale, f);
        });
    }

    /// InputBinding_EvalBool through the real InputMap_EvalBool and a VM
    /// program that stops at once (the camera tests' input map), with the
    /// stopped VM's value 0 or 1.
    #[test]
    fn input_binding_eval_bool_matches() {
        check(ee::s10::f_00320c48__generated, input_binding_eval_bool, 200, &|c, seed| {
            let mut r = Rng(0x0032_0c48 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0032_0c48;
            let binding = DATA + 0x1000;
            let map = DATA + 0x2000;
            c.g[A0][0] = binding as u64;
            c.g[A1][0] = (seed % 4) as u64; // the map's binding index
            c.bus.write32(binding, r.next_u32());
            c.bus.write32(binding + 4, map);
            crate::game::camera::camera::tests::input_map(c, map);
            // The stopped VM leaves `*(parser + 8)` in f0 (parser = the
            // map's frame below this function's 0x10 frame).
            let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
            c.bus.write32(crate::difftest::SP - 0x10 - 0x1d0 + 8, value.to_bits());
        });
    }

    /// SND_GetVolume through the real SND_HandleToChannel: negative handle,
    /// channel out of range, inactive channel, handle mismatch, or a
    /// playing channel whose volume is read.
    #[test]
    fn snd_get_volume_matches() {
        crate::difftest::check_lohi(ee::s13::f_003b86c0__generated, snd_get_volume, 240, &|c, seed| {
            let mut r = Rng(0x003b_86c0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x003b_86c0;
            // The SND state: channel count (+0x17e) and channel array (+0x1f0,
            // 0x8c bytes per channel: handle at +0, active byte at +0x69).
            let snd = 0x0050_a8e8;
            let channels = DATA + 0x1000;
            let params = DATA + 0x4000;
            c.bus.write16(snd + 0x17e, 8);
            c.bus.write32(snd + 0x1f0, channels);
            c.bus.write32(SND_CHANNEL_PARAMS, params);
            let ch = r.next_u32() % 10; // 8, 9: out of range
            let handle = (r.next_u32() & 0x7fff_ff00) | ch;
            c.g[A0][0] = match seed % 6 {
                0 => (r.next_u32() | 0x8000_0000) as i32 as i64 as u64,
                _ => handle as u64,
            };
            if ch < 8 {
                let at = channels + ch * 0x8c;
                c.bus.write8(at + 0x69, if seed % 6 == 1 { 0 } else { 1 });
                c.bus.write32(at, if seed % 6 == 2 { handle ^ 0x100 } else { handle });
                c.bus.write16(params + ch * 0x8c + 0x3a, r.next_u32() as u16);
            }
        });
    }

    /// CrowdMan3d_Update through the real CrowdMan3d_GroupUpdate: five
    /// groups, each unused (id -1) or with 0..2 members whose update
    /// method is a fake (which may unwind).
    #[test]
    fn crowd_man3d_update_matches() {
        crate::difftest::fakes::covered(ee::s11::f_003440c8__generated, crowd_man3d_update, 200, &|c, seed| {
            let mut r = crate::difftest::fakes::machine(c, seed, 0x3440c8, 6);
            c.pc = 0x0034_40c8;
            let man = DATA;
            c.g[A0][0] = sx(man);
            for g in 0..5u32 {
                let group = man + g * CROWD_GROUP_SIZE;
                let used = r.next_u32() % 4 != 0;
                c.bus.write32(group + CROWD_GROUP_ID, if used { g } else { u32::MAX });
                c.bus.write32(group + 0x10c, r.next_u32() % 3);
                c.bus.write32(group + 0x1dc, r.f32(5.0));
                c.bus.write32(group + 0x1e0, (10.0f32).to_bits());
                c.bus.write32(group + 0x1e4, r.f32(1.0));
                let items = DATA + 0x10000 + g * 0x1000;
                c.bus.write32(group + 0x1ec, items);
                for i in 0..2u32 {
                    let item = items + 0xd0 * i;
                    let vt = DATA + 0x20000 + g * 0x100 + i * 0x20;
                    c.bus.write32(item + 0x84, vt);
                    c.bus.write16(vt + 0x10, 0);
                    c.bus.write32(vt + 0x14, crate::difftest::fakes::FAKE + 0x10 * i);
                }
            }
        });
    }

    /// A PATH state with `count` queues for PATH_ServiceQueues: heads that
    /// are pending (actions left, already run so no PATH_ExecuteAction),
    /// done, expired or forever, a zero head that ends the scan, slots past
    /// the count; the removal callback (0x44f428) a fake or none. Queue
    /// heads live outside the state's action pool, so PATH_RemoveEvent
    /// frees nothing.
    fn path_queues_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let state = DATA + 0x1000;
        let now = 1000 + r.next_u32() % 1000;
        c.bus.write32(0x0045_0000 - PATH_STATE_OFF, state);
        c.bus.write32(0x0045_0000 - PATH_TIME_OFF, now);
        let callback = if seed % 2 == 0 { crate::difftest::fakes::FAKE } else { 0 };
        c.bus.write32(0x0045_0000 - 0xbd8, callback);
        let count = r.next_u32() % 17;
        c.bus.write32(state + PATH_QUEUE_COUNT, count);
        for i in 0..16u32 {
            let node = DATA + 0x10000 + i * 0x800;
            let head = if i >= count {
                r.next_u32() | 1 // cleared, never read
            } else if r.next_u32() % 12 == 0 {
                0
            } else {
                node
            };
            c.bus.write32(state + PATH_QUEUES + 4 * i, head);
            c.bus.write32(node + QUEUE_START, now.wrapping_sub(r.next_u32() % 300));
            let lifetime = if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() % 400 };
            c.bus.write32(node + QUEUE_LIFETIME, lifetime);
            c.bus.write32(node + 0x8, r.next_u32());
            let end = r.next_u32() % 5;
            let next = r.next_u32() % 5;
            let indices = (r.next_u32() & 0xf000_ffff) | end << 16 | next << 22;
            c.bus.write32(node + QUEUE_INDICES, indices);
            for k in 0..8u32 {
                let action = node + 0x10 + 16 * k;
                c.bus.write32(action + 4, 0);
                c.bus.write32(action + 8, r.next_u32() | 0x0400_0000);
            }
        }
    }

    crate::difftest::events::driven!(gen_path_service_queues, ee::s14::f_003d1e80__generated);
    crate::difftest::events::driven!(hand_path_service_queues, path_service_queues);

    /// PATH_ServiceQueues with the real PATH_ServiceQueue / PATH_RemoveEvent
    /// (fake callback, which may unwind), then with events delivered and
    /// unwinding at the loop heads (re-entered at 0x3d1f1c / 0x3d1ec0).
    #[test]
    fn path_service_queues_matches() {
        check(ee::s14::f_003d1e80__generated, path_service_queues, 240, &|c, seed| {
            let mut r = crate::difftest::fakes::machine(c, seed, 0x3d1e80, 3);
            c.pc = SERVICE_QUEUES_ENTRY;
            path_queues_fixture(c, &mut r, seed);
        });
        crate::difftest::events::check_events(
            gen_path_service_queues,
            hand_path_service_queues,
            120,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x3d1e81, 0);
                path_queues_fixture(c, &mut r, seed / 3);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, SERVICE_QUEUES_ENTRY);
                if mode != crate::difftest::events::Events::Off {
                    // Early enough for the short scans too.
                    c.next_event = c.cycles + 4 + (r.next_u32() % 40) as u64;
                }
                mode
            },
        );
    }
}
