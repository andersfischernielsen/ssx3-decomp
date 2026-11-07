//! Staging wave 212 (oc311): hand-written replacements for the hottest
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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_5f60,
        world_trigger_manager_play_trigger_sound,
        "WorldTriggerManager_PlayTriggerSound",
    ),
    (
        0x002f_ae38,
        world_obj_factory_attach_modifier,
        "WorldObjFactory_AttachModifier",
    ),
    (0x0034_aef8, flag_save, "Flag_Save"),
    (0x0037_12b8, dynamic_emitter_save, "DynamicEmitter_Save"),
    (
        0x002a_d5f0,
        bank_monitor_stop_handle,
        "BankMonitor_StopHandle",
    ),
    (
        0x0029_10e0,
        ssx_audio_system_start_ambience_loop,
        "SSXAudioSystem_StartAmbienceLoop",
    ),
    (0x003c_9420, snd_dec_utk_init_mem, "SND_DecUtk_InitMem"),
    (0x0035_78a8, particle_ctor_2, "Particle_ctor_2"),
];
// ---------------------------------------------------------------------------
// 0x002b5f60 WorldTriggerManager_PlayTriggerSound

/// `WorldTriggerManager.triggers` (+0x33c): 8-byte entries `{type, info}`;
/// type 3 is a trigger that never plays.
pub(crate) const WTM_PLAY_TRIGGERS: u32 = 0x33c;
/// `WorldTriggerManager.lastEvent` (+0x328): the last event that played.
pub(crate) const WTM_PLAY_LAST_EVENT: u32 = 0x328;
/// `WorldTriggerManager.triggerCount` (+0x340).
pub(crate) const WTM_PLAY_TRIGGER_COUNT: u32 = 0x340;
/// The trigger type that is never played.
const WTM_PLAY_TYPE_MUTED: u64 = 3;

/// One `queue.table[count] = value` store of the inlined push (the
/// original's register pair; `s0` = monitor, `s1` = queue).
pub(crate) fn wtm_play_store(
    c: &mut Cpu,
    regs: (usize, usize),
    table: u32,
    shift: u32,
    extra: u32,
    value: u64,
) {
    crate::game::sound::soundsys::boost_empty_store_entry(c, regs, table, shift, extra, value);
}

/// `int WorldTriggerManager_PlayTriggerSound(WorldTriggerManager* m, void*
/// event, int id, Rider* rider, int volume)` (0x2b5f60): play world trigger
/// `id`. Muted triggers (type 3) return -1. A new `event` different from the
/// last one first marks its flagged triggers as played. The trigger's info
/// (`WorldTriggerManager_GetTriggerInfo`) gives the bank and sample; it is
/// pushed on the audio system's bank-monitor queue (defaults: pan 100, 90,
/// 50, priority 127), with `volume` clamped to 0..127, the channel-7 gain,
/// and for a rider its position source and listener; then
/// `Audio_PlayPushedSound`. Returns the play handle or -1.
pub fn world_trigger_manager_play_trigger_sound(c: &mut Cpu) {
    guest(c, |c| {
        const GET_INSTANCE: u32 = 0x0028_b180; // SSXAudioSystem_GetInstance
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0]; // event
        c.g[S1][0] = c.g[A0][0]; // manager
        c.g[S2][0] = c.g[A2][0]; // id
        c.g[S6][0] = c.g[A3][0]; // rider
        c.g[S7][0] = c.g[T0][0]; // volume
        let mgr = c.g[S1][0] as u32;
        c.g[V1][0] = sx((c.g[S2][0] as u32) << 3);
        c.g[V0][0] = c.lw(mgr.wrapping_add(WTM_PLAY_TRIGGERS));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = WTM_PLAY_TYPE_MUTED;

        // 0: v0 = -1 and restore s0 (muted, no info); 1: restore s0 (v0
        // already -1, the info's third word set); 2: played.
        let exit: u8 = 'body: {
            if c.g[A0][0] == WTM_PLAY_TYPE_MUTED {
                break 'body 0;
            }
            // A new event: mark its flagged triggers played.
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            if c.g[S0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(mgr.wrapping_add(WTM_PLAY_LAST_EVENT));
                if c.g[S0][0] != c.g[V0][0] {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002b_6740,
                        f: ee::s08::WorldTriggerManager_AnyTriggerFlagged,
                    }
                    .call(c, 0x002b_5fd0)?;
                    let flagged = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S1][0];
                    if flagged {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x002b_6868,
                            f: ee::s08::WorldTriggerManager_MarkPlayed,
                        }
                        .call(c, 0x002b_5fe0)?;
                    }
                    c.cycles += 2;
                    c.sw(mgr.wrapping_add(WTM_PLAY_LAST_EVENT), c.g[S0][0]);
                    c.g[A0][0] = c.g[S1][0];
                } else {
                    c.sw(mgr.wrapping_add(WTM_PLAY_LAST_EVENT), c.g[S0][0]);
                }
            }

            // GetTriggerInfo(m, id, &bank, &sample, &extra, 1, 0).
            c.cycles += 9;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = c.g[SP][0] | 0x4;
            c.g[T0][0] = c.g[SP][0] | 0x8;
            c.g[T1][0] = 0x1;
            c.g[T2][0] = 0;
            Callee {
                addr: 0x002b_63d0,
                f: ee::s08::WorldTriggerManager_GetTriggerInfo,
            }
            .call(c, 0x002b_6004)?;
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x8));
            if !found {
                break 'body 0;
            }
            c.cycles += 2;
            let extra = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if extra {
                break 'body 1;
            }

            // Push a new entry on the monitor's queue.
            c.cycles += 159;
            c.g[S3][0] = 0x18; // record size
            call_ptr(c, GET_INSTANCE, 0x002b_601c)?;
            c.g[S2][0] = c.g[V0][0]; // sys
            c.g[S4][0] = c.lw(sp(c)); // bank
            c.g[S0][0] = c.lw(c.g[S2][0] as u32); // monitor
            c.g[A1][0] = 0;
            c.g[S5][0] = c.lw(sp(c).wrapping_add(0x4)); // sample
            c.g[A2][0] = 0x8;
            let monitor = c.g[S0][0] as u32;
            c.g[V0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[S1][0] = sx(monitor.wrapping_add(0x1d8)); // queue
            let queue = c.g[S1][0] as u32;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(monitor.wrapping_add(0x1f4), c.g[V0][0]);
            // queue.records[count] = the 24-byte template (monitor + 0x1dc).
            c.g[V1][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[V0][0] = c.lw(queue.wrapping_add(0x20));
            c.mult(0, V1, V1, S3);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(2, monitor.wrapping_add(0x1e3));
            c.ldr(2, monitor.wrapping_add(0x1dc));
            c.ldl(4, monitor.wrapping_add(0x1eb));
            c.ldr(4, monitor.wrapping_add(0x1e4));
            c.ldl(7, monitor.wrapping_add(0x1f3));
            c.ldr(7, monitor.wrapping_add(0x1ec));
            let record = c.g[V1][0] as u32;
            c.sdl(2, record.wrapping_add(0x7));
            c.sdr(2, record);
            c.sdl(4, record.wrapping_add(0xf));
            c.sdr(4, record.wrapping_add(0x8));
            c.sdl(7, record.wrapping_add(0x17));
            c.sdr(7, record.wrapping_add(0x10));
            // monitor.handles[count] = 0 (inline at +0x1fc).
            c.g[V0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0);
            wtm_play_store(c, (V1, V0), 0x34, 2, 0, 0);
            // table 0x38[count] = the record's first byte (sign-extended).
            c.g[A3][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[A0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[T0][0] = c.lw(queue.wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, A0, A0, S3);
            c.g[V0][0] = c.lw(queue.wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            wtm_play_store(c, (V0, V1), 0x3c, 2, 0, 0);
            // memset(&queue.slots[count] (+0x40, 8 bytes), 0, 8).
            c.g[A0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx(queue.wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x0041_6210, 0x002b_60f8)?;

            // The entry's parameters.
            c.g[A0][0] = 0x64;
            c.g[A1][0] = 0x5a;
            c.g[A2][0] = 0x32;
            c.g[A3][0] = 0x7f;
            c.g[T0][0] = 0x1;
            wtm_play_store(c, (V1, V0), 0x60, 2, 0, 0);
            wtm_play_store(c, (V0, V1), 0x64, 2, 0, 0);
            wtm_play_store(c, (V1, V0), 0x68, 2, 0, 0);
            wtm_play_store(c, (V0, V1), 0x6c, 2, 0, 0);
            wtm_play_store(c, (V1, V0), 0x70, 4, 0x4, 0);
            wtm_play_store(c, (V0, V1), 0x70, 4, 0x0, 0x64);
            wtm_play_store(c, (V1, V0), 0x70, 4, 0x8, 0x5a);
            wtm_play_store(c, (V0, V1), 0x70, 4, 0xc, 0x32);
            wtm_play_store(c, (V1, V0), 0x74, 2, 0, 0);
            wtm_play_store(c, (V0, V1), 0x80, 2, 0, 0);
            wtm_play_store(c, (V1, V0), 0x84, 2, 0, 0);
            wtm_play_store(c, (V0, V1), 0x78, 2, 0, 0x7f);
            wtm_play_store(c, (V1, V0), 0x7c, 2, 0, 0x1);

            // sys->pushBank[count] = bank; sys->pushSample[count] = sample.
            let sys = c.g[S2][0] as u32;
            c.g[A0][0] = c.lw(sys);
            c.g[V1][0] = c.lw(sys.wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S4][0]);
            c.g[V1][0] = c.lw(sys);
            c.g[A0][0] = c.lw(sys.wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S5][0]);

            // queue.volume[count] = volume; record.volume = clamp(volume, 0, 127).
            call_ptr(c, GET_INSTANCE, 0x002b_6244)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.lw(c.g[V1][0] as u32);
            let monitor = c.g[A1][0] as u32;
            c.g[V0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[A2][0] = sx(monitor.wrapping_add(0x1d8));
            let queue = c.g[A2][0] as u32;
            c.g[V1][0] = c.lw(queue.wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S7][0]);
            c.g[A0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[V1][0] = c.lw(queue.wrapping_add(0x1c));
            c.g[A3][0] = c.lw(queue.wrapping_add(0x38));
            c.mult(0, A0, A0, S3);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A1][0] = c.lw(queue.wrapping_add(0x20));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            let volume_at = |c: &mut Cpu| -> u32 {
                let i = c.lw(queue.wrapping_add(0x1c)) as u32;
                (i << 2).wrapping_add(c.g[A3][0] as u32)
            };
            'clamped: {
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    let at = volume_at(c);
                    c.g[V0][0] = sx(at);
                    c.g[V1][0] = c.lw(at);
                    let negative = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = 0;
                    if negative {
                        break 'clamped;
                    }
                }
                c.cycles += 7;
                let at = volume_at(c);
                c.g[V0][0] = sx(at);
                c.g[V1][0] = c.lw(at);
                c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0x7f;
                } else {
                    c.cycles += 4;
                    let at = volume_at(c);
                    c.g[V0][0] = c.lbu(at);
                }
            }
            c.cycles += 15;
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);

            // queue.gain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 7, 0).
            call_ptr(c, GET_INSTANCE, 0x002b_62dc)?;
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
            call_ptr(c, GET_INSTANCE, 0x002b_62e4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x7;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0028_7968, 0x002b_62f4)?;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x214));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);

            if c.g[S6][0] != 0 {
                // A rider: the position source (rider + 0x110), its
                // listener, and the "positional" flag.
                c.cycles += 30;
                call_ptr(c, GET_INSTANCE, 0x002b_6318)?;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x110));
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A1][0]);
                call_ptr(c, GET_INSTANCE, 0x002b_6338)?;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
                call_ptr(c, GET_INSTANCE, 0x002b_6340)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S6][0];
                call_ptr(c, 0x0028_8a20, 0x002b_634c)?; // GetRiderListenerIndex
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x23c));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                call_ptr(c, GET_INSTANCE, 0x002b_6368)?;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = 0x1;
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw(c.g[V0][0] as u32, c.g[A1][0]);
            }
            // return Audio_PlayPushedSound(sys).
            c.cycles += 6;
            call_ptr(c, GET_INSTANCE, 0x002b_6390)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0029_06b8, 0x002b_6398)?;
            frame.restore(c, S0, 0x90);
            2
        };
        match exit {
            0 => {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                frame.restore(c, S0, 0x90);
            }
            1 => {
                c.cycles += 1;
                frame.restore(c, S0, 0x90);
            }
            _ => {}
        }
        c.cycles += 10;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002fae38 WorldObjFactory_AttachModifier

/// `Node.vtable` (+0xc).
pub(crate) const WOF_ATTACH_VTABLE: u32 = 0xc;
/// `Node.object` (+0x18): the world object a modifier attaches to.
pub(crate) const WOF_ATTACH_OBJECT: u32 = 0x18;
/// Vtable slots (`{i16 delta, fn}` pairs): `IsType(type)` (+0x50),
/// `Refresh()` (+0x128), the deleting destructor (+0x8).
pub(crate) const WOF_ATTACH_VT_IS_TYPE: u32 = 0x50;
pub(crate) const WOF_ATTACH_VT_REFRESH: u32 = 0x128;
pub(crate) const WOF_ATTACH_VT_DELETE: u32 = 0x8;

/// `node->vtable->slot(node + delta, a1)` with the original's registers:
/// `rv` holds the vtable, `rf` the method.
fn wof_attach_vcall(c: &mut Cpu, rv: usize, rf: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let node = c.g[S0][0] as u32;
    c.g[rv][0] = c.lw(node.wrapping_add(WOF_ATTACH_VTABLE));
    let vt = c.g[rv][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[rf][0] = c.lw(vt.wrapping_add(slot + 4));
    let target = c.g[rf][0] as u32;
    c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `int WorldObjFactory_AttachModifier(Node* node, int type, void* data)`
/// (0x2fae38): a node that already is a type 9, 6 or 22 modifier keeps
/// going (1); one that already is `type` is refreshed (1). Otherwise types
/// 13 (particle), 11 (mesh animation), 0 (debounce) and 22 (dead fade)
/// allocate their modifier on the node's object (whose +0xc word is
/// cleared), built from `data` (-1); any other type deletes the node (0).
pub fn world_obj_factory_attach_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0]; // node
        c.g[S2][0] = c.g[A1][0]; // type
        c.g[S3][0] = c.g[A2][0]; // data
        'done: {
            // Already a type 9, 6 or 22 modifier: nothing to do.
            for (i, (kind, ret)) in [
                (0x9u64, 0x002f_ae74u32),
                (0x6, 0x002f_ae94),
                (0x16, 0x002f_aeb4),
            ]
            .into_iter()
            .enumerate()
            {
                if i > 0 {
                    c.cycles += 8;
                }
                c.g[A1][0] = kind;
                wof_attach_vcall(c, V1, V0, WOF_ATTACH_VT_IS_TYPE, ret)?;
                let is = c.g[V0][0] != 0;
                c.g[V0][0] = 0x1;
                if is {
                    break 'done;
                }
            }
            // Already `type`: refresh it.
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            wof_attach_vcall(c, V1, V0, WOF_ATTACH_VT_IS_TYPE, 0x002f_aed4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                wof_attach_vcall(c, V0, V1, WOF_ATTACH_VT_REFRESH, 0x002f_aef0)?;
                c.g[V0][0] = 0x1;
                break 'done;
            }
            c.g[V0][0] = 0xb;

            // The modifier to build: (alloc size, name, ret, ctor, ctor ret).
            c.cycles += 2;
            let ty = c.g[S2][0];
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(WOF_ATTACH_OBJECT));
            let object = c.g[S1][0] as u32;
            let kind: Option<u8> = if ty == 0xb {
                Some(11)
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((ty as i64) < 12) as u64;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if ty == 0 {
                        c.sw(object.wrapping_add(0xc), 0);
                        Some(0)
                    } else {
                        c.cycles += 2;
                        None
                    }
                } else {
                    c.g[V0][0] = 0xd;
                    c.cycles += 2;
                    if ty == 0xd {
                        c.sw(object.wrapping_add(0xc), 0);
                        Some(13)
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x16;
                        if ty == 0x16 {
                            c.sw(object.wrapping_add(0xc), 0);
                            Some(22)
                        } else {
                            c.cycles += 2;
                            None
                        }
                    }
                }
            };
            let Some(kind) = kind else {
                // Unknown type: delete the node.
                c.cycles += 2;
                c.g[A1][0] = 0x3;
                if c.g[S0][0] != 0 {
                    c.cycles += 5;
                    wof_attach_vcall(c, V0, V1, WOF_ATTACH_VT_DELETE, 0x002f_b040)?;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            };
            // (size, name, alloc ret, ctor, ctor ret)
            let (size, name, alloc_ret, ctor, ctor_ret) = match kind {
                13 => (
                    0x2c,
                    0x0049_0000 - 0x69e8,
                    0x002f_af50,
                    0x0035_78a8,
                    0x002f_af68,
                ),
                11 => (
                    0x6c0,
                    0x0049_0000 - 0x69d8,
                    0x002f_af8c,
                    0x0035_1b40,
                    0x002f_afac,
                ),
                0 => (
                    0x34,
                    0x0049_0000 - 0x69c8,
                    0x002f_afcc,
                    0x0034_2c08,
                    0x002f_afe4,
                ),
                _ => (
                    0x3c,
                    0x0049_0000 - 0x69b8,
                    0x002f_b004,
                    0x0035_0750,
                    0x002f_b01c,
                ),
            };
            if kind == 11 {
                c.cycles += 17;
                c.sw(object.wrapping_add(0xc), 0);
            } else {
                c.cycles += 14;
            }
            // EA_MemAllocNamed(size, name, 0x20000000, 0)
            c.g[A1][0] = sx(name);
            c.g[A0][0] = size;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            Callee {
                addr: 0x0031_7d70,
                f: ee::s10::EA_MemAllocNamed,
            }
            .call(c, alloc_ret)?;
            // ctor(mem, 1, object, data, [the world's +0x294 for meshes,] node)
            let node = c.g[S0][0];
            if kind == 11 {
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x294));
                c.g[T1][0] = node;
            } else {
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = node;
            }
            c.g[A1][0] = 0x1;
            call_ptr(c, ctor, ctor_ret)?;
            c.g[V0][0] = u64::MAX;
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
// 0x0034aef8 Flag_Save

/// `void Flag_Save(Flag* self, Stream* stream)` (0x34aef8): a flag saves
/// only its world-object state (`WorldObj_Save`).
pub fn flag_save(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee {
            addr: 0x0034_fe90,
            f: ee::s11::WorldObj_Save,
        }),
        0x0034_af08,
    );
}

// ---------------------------------------------------------------------------
// 0x003712b8 DynamicEmitter_Save

/// `DynamicEmitter` fields saved: the time (+0), the rate (+0x28, floats),
/// two counters (+0x170, +0x174) and the state (+0x17c).
pub(crate) const DYN_EMITTER_SAVE_RATE: u32 = 0x28;
pub(crate) const DYN_EMITTER_SAVE_COUNT_A: u32 = 0x170;
pub(crate) const DYN_EMITTER_SAVE_COUNT_B: u32 = 0x174;
pub(crate) const DYN_EMITTER_SAVE_STATE: u32 = 0x17c;

/// `void DynamicEmitter_Save(DynamicEmitter* self, Stream* stream)`
/// (0x3712b8): write a 20-byte record `{time, rate, countB, countA, state}`
/// with the stream's write method (`stream->vtable` slot +0x8).
pub fn dynamic_emitter_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0x20);
        let rec = sp(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x14;
        let this = c.g[A0][0] as u32;
        c.g[T0][0] = c.lw(this.wrapping_add(DYN_EMITTER_SAVE_STATE));
        c.lwc1(1, this);
        c.lwc1(0, this.wrapping_add(DYN_EMITTER_SAVE_RATE));
        c.g[V1][0] = c.lw(this.wrapping_add(DYN_EMITTER_SAVE_COUNT_B));
        c.g[V0][0] = c.lw(this.wrapping_add(DYN_EMITTER_SAVE_COUNT_A));
        c.swc1(1, rec);
        c.swc1(0, rec.wrapping_add(0x4));
        c.sw(rec.wrapping_add(0x8), c.g[V1][0]);
        c.sw(rec.wrapping_add(0x10), c.g[T0][0]);
        c.sw(rec.wrapping_add(0xc), c.g[V0][0]);
        // stream->Write(&rec, 0x14)
        let stream = c.g[A3][0] as u32;
        c.g[V0][0] = c.lw(stream);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = c.lh(vt.wrapping_add(0x8));
        c.g[V1][0] = c.lw(vt.wrapping_add(0xc));
        c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
        let write = c.g[V1][0] as u32;
        call_ptr(c, write, 0x0037_1308)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002ad5f0 BankMonitor_StopHandle

/// `void BankMonitor_StopHandle(BankMonitor* bm, int id, int kind, float
/// fade)` (0x2ad5f0): find the playing instance `(kind, id)`; fade it out
/// over `fade` (`BankMonitorInstance_FadeVolume(inst, -1, 1, fade)`) or,
/// for no fade, stop it at once (`BankMonitorInstance_Reset`).
pub fn bank_monitor_stop_handle(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_fpr(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.f[20] = c.f[12]; // fade
        Callee {
            addr: 0x002a_d550,
            f: ee::s08::BankMonitor_FindInstance,
        }
        .call(c, 0x002a_d604)?;
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.f[0] = 0;
            let no_fade = FpuF32(c.f[20]).eq_s(FpuF32(c.f[0]));
            crate::abi::set_fcc(c, no_fade);
            c.g[A0][0] = c.g[V0][0];
            if !no_fade {
                c.cycles += 6;
                c.f[12] = c.f[20];
                c.g[A1][0] = u64::MAX;
                c.g[A2][0] = 0x1;
                Callee {
                    addr: 0x002a_cb80,
                    f: ee::s08::BankMonitorInstance_FadeVolume,
                }
                .call(c, 0x002a_d630)?;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x002a_bb38,
                    f: ee::s08::BankMonitorInstance_Reset,
                }
                .call(c, 0x002a_d640)?;
            }
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002910e0 SSXAudioSystem_StartAmbienceLoop

/// `SSXAudioSystem.ambienceGain` (+0x5830, the gain word the loop plays
/// with) and `.ambienceStarted` (+0x5834).
pub(crate) const AMBIENCE_LOOP_GAIN: u32 = 0x5830;
pub(crate) const AMBIENCE_LOOP_STARTED: u32 = 0x5834;
/// The 8-byte key the ambience loop is pushed with (`.data` 0x4a3680).
const AMBIENCE_LOOP_KEY: u32 = 0x004a_3680;

/// `void SSXAudioSystem_StartAmbienceLoop(SSXAudioSystem* sys)` (0x2910e0):
/// once: push sample 0x20 of bank 0 on the bank monitor's queue (pan 100,
/// 90, 50, priority 127, the system's ambience gain word, the ambience
/// key), play it (`Audio_PlayPushedSound`) and mark it started.
pub fn ssx_audio_system_start_ambience_loop(c: &mut Cpu) {
    use crate::game::sound::soundsys::boost_empty_store_entry as ambience_store;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A0][0]; // sys
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x10);
        let sys = c.g[S4][0] as u32;
        c.g[V0][0] = c.lw(sys.wrapping_add(AMBIENCE_LOOP_STARTED));
        frame.restore(c, S0, 0x60);
        if c.g[V0][0] == 0 {
            c.cycles += 172;
            // New queue entry: count += 1, the 24-byte template record.
            c.g[S0][0] = c.lw(sys); // monitor
            let monitor = c.g[S0][0] as u32;
            c.g[T1][0] = 0x18;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[S1][0] = sx(monitor.wrapping_add(0x1d8)); // queue
            let queue = c.g[S1][0] as u32;
            c.g[S2][0] = 0x1;
            c.g[S3][0] = 0x20; // sample
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(monitor.wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[V0][0] = c.lw(queue.wrapping_add(0x20));
            c.mult(0, V1, V1, T1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(2, monitor.wrapping_add(0x1e3));
            c.ldr(2, monitor.wrapping_add(0x1dc));
            c.ldl(4, monitor.wrapping_add(0x1eb));
            c.ldr(4, monitor.wrapping_add(0x1e4));
            c.ldl(7, monitor.wrapping_add(0x1f3));
            c.ldr(7, monitor.wrapping_add(0x1ec));
            let record = c.g[V1][0] as u32;
            c.sdl(2, record.wrapping_add(0x7));
            c.sdr(2, record);
            c.sdl(4, record.wrapping_add(0xf));
            c.sdr(4, record.wrapping_add(0x8));
            c.sdl(7, record.wrapping_add(0x17));
            c.sdr(7, record.wrapping_add(0x10));
            c.g[V0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0);
            ambience_store(c, (V1, V0), 0x34, 2, 0, 0);
            c.g[A3][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[A0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[T0][0] = c.lw(queue.wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, A0, A0, T1);
            c.g[V0][0] = c.lw(queue.wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            ambience_store(c, (V0, V1), 0x3c, 2, 0, 0);
            // memset(&queue.slots[count], 0, 8)
            c.g[A0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx(queue.wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x0041_6210, 0x0029_11e8)?;

            // The entry's parameters.
            c.g[A1][0] = 0x64;
            c.g[A2][0] = 0x5a;
            c.g[A3][0] = 0x32;
            c.g[T0][0] = 0x7f;
            c.g[T1][0] = sx(sys.wrapping_add(AMBIENCE_LOOP_GAIN));
            c.g[T2][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S4][0];
            ambience_store(c, (V1, V0), 0x60, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x64, 2, 0, 0);
            ambience_store(c, (V1, V0), 0x68, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x6c, 2, 0, 0);
            ambience_store(c, (V1, V0), 0x70, 4, 0x4, 0);
            ambience_store(c, (V0, V1), 0x70, 4, 0x0, 0x64);
            ambience_store(c, (V1, V0), 0x70, 4, 0x8, 0x5a);
            ambience_store(c, (V0, V1), 0x70, 4, 0xc, 0x32);
            ambience_store(c, (V1, V0), 0x74, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x80, 2, 0, 0);
            ambience_store(c, (V1, V0), 0x84, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x78, 2, 0, 0x7f);
            ambience_store(c, (V1, V0), 0x7c, 2, 0, 0x1);

            // sys->pushBank[count] = 0; sys->pushSample[count] = 0x20.
            c.g[A1][0] = c.lw(sys);
            c.g[V1][0] = c.lw(sys.wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0);
            c.g[V1][0] = c.lw(sys);
            c.g[A1][0] = c.lw(sys.wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S3][0]);
            // Through (*sys->sndRoot): gain[count] = &sys->ambienceGain,
            // table 0x7c[count] = 0, slots[count] = the ambience key.
            for (table, value) in [(0x214u32, c.g[T1][0]), (0x254, 0)] {
                c.g[V1][0] = c.lw(sys.wrapping_add(0x118));
                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                let mon = c.g[A1][0] as u32;
                c.g[V0][0] = c.lw(mon.wrapping_add(0x1f4));
                c.g[V1][0] = c.lw(mon.wrapping_add(table));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V0][0] as u32, value);
            }
            c.g[V1][0] = c.lw(sys.wrapping_add(0x118));
            c.g[A1][0] = c.lw(c.g[V1][0] as u32);
            let mon = c.g[A1][0] as u32;
            c.g[A3][0] = sx(AMBIENCE_LOOP_KEY);
            let tmp = sp(c);
            c.ldl(2, AMBIENCE_LOOP_KEY.wrapping_add(0x7));
            c.ldr(2, AMBIENCE_LOOP_KEY);
            c.sdl(2, tmp.wrapping_add(0x7));
            c.sdr(2, tmp);
            c.g[V1][0] = sx(mon.wrapping_add(0x1d8));
            c.g[V0][0] = c.lw(mon.wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let slot = c.g[V1][0] as u32;
            c.ldl(8, tmp.wrapping_add(0x7));
            c.ldr(8, tmp);
            c.sdl(8, slot.wrapping_add(0x47));
            c.sdr(8, slot.wrapping_add(0x40));
            Callee {
                addr: 0x0029_06b8,
                f: ee::s07::Audio_PlayPushedSound,
            }
            .call(c, 0x0029_13b4)?;
            c.sw(sys.wrapping_add(AMBIENCE_LOOP_STARTED), c.g[S2][0]);
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003c9420 SND_DecUtk_InitMem

/// The UTK decoder object: callbacks (+0, +4), sample count (+0x1c), a
/// cleared word (+0x20), mode (+0x24), the UTK state (+0x28).
pub(crate) const DEC_UTK_COUNT: u32 = 0x1c;
pub(crate) const DEC_UTK_MODE: u32 = 0x24;
pub(crate) const DEC_UTK_STATE: u32 = 0x28;
/// The configuration: input (+0), count (+0xc), mode (+0x24), and the
/// callback slot the decoder fills (+0x28).
pub(crate) const DEC_UTK_CFG_COUNT: u32 = 0xc;

/// `void SND_DecUtk_InitMem(DecUtk* self, DecUtkCfg* cfg)` (0x3c9420): set
/// up a UTK (MicroTalk) decoder in memory: its callbacks, count and mode
/// from `cfg`; allocate (`SND_UtkAlloc(0xd68)`) and reset the UTK state,
/// set its "running" flag, mode 0 for decoder modes below 3 else 1, and
/// point it at the input (`SND_UtkSetInput(state, cfg->input, count * 4,
/// count)`).
pub fn snd_dec_utk_init_mem(c: &mut Cpu) {
    guest(c, |c| {
        const ALLOC: Callee = Callee {
            addr: 0x003c_d1a8,
            f: ee::s14::SND_UtkAlloc,
        };
        const RESET: Callee = Callee {
            addr: 0x003c_ddb8,
            f: ee::s14::SND_UtkReset,
        };
        const GET_STATE: Callee = Callee {
            addr: 0x003c_e078,
            f: ee::s14::SND_UtkGetState,
        };
        const SET_STATE: Callee = Callee {
            addr: 0x003c_e0b8,
            f: ee::s14::SND_UtkSetState,
        };
        const SET_MODE: Callee = Callee {
            addr: 0x003c_e0d8,
            f: ee::s14::SND_UtkSetMode,
        };
        const SET_INPUT: Callee = Callee {
            addr: 0x003c_dde8,
            f: ee::s14::SND_UtkSetInput,
        };
        c.cycles += 44;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let this = c.g[S0][0] as u32;
        let cfg = c.g[S1][0] as u32;
        c.g[V0][0] = 0x003c_9360; // decode callback
        c.sw(this, c.g[V0][0]);
        c.g[A1][0] = 0x003c_9358; // the cfg's callback
        c.g[V1][0] = 0x003c_93f8; // second callback
        c.g[A2][0] = c.lw(cfg.wrapping_add(DEC_UTK_CFG_COUNT));
        c.sw(this.wrapping_add(0x20), 0);
        c.g[A0][0] = 0xd68;
        c.sw(this.wrapping_add(DEC_UTK_COUNT), c.g[A2][0]);
        c.g[V0][0] = c.lw(cfg.wrapping_add(0x24));
        c.sw(this.wrapping_add(DEC_UTK_MODE), c.g[V0][0]);
        c.sw(cfg.wrapping_add(0x28), c.g[A1][0]);
        c.sw(this.wrapping_add(0x4), c.g[V1][0]);
        ALLOC.call(c, 0x003c_9478)?;
        c.g[A0][0] = c.g[V0][0];
        RESET.call(c, 0x003c_9480)?;
        // state = GetState(utk); state.running = 1; SetState(utk, &state, 0).
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
        c.sw(this.wrapping_add(DEC_UTK_STATE), c.g[V0][0]);
        GET_STATE.call(c, 0x003c_9490)?;
        let s = sp(c);
        c.g[A0][0] = c.lw(this.wrapping_add(DEC_UTK_STATE));
        c.g[V0][0] = 0x1;
        c.ldl(3, s.wrapping_add(0x17));
        c.ldr(3, s.wrapping_add(0x10));
        c.g[A1][0] = c.lw(s.wrapping_add(0x18));
        c.sdl(3, s.wrapping_add(0x7));
        c.sdr(3, s);
        c.sw(s.wrapping_add(0x8), c.g[A1][0]);
        c.sw(s.wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0;
        SET_STATE.call(c, 0x003c_94c0)?;
        c.g[V0][0] = c.lw(this.wrapping_add(DEC_UTK_MODE));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
        c.g[A0][0] = c.lw(this.wrapping_add(DEC_UTK_STATE));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = 0;
            SET_MODE.call(c, 0x003c_94d8)?;
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x1;
            SET_MODE.call(c, 0x003c_94e8)?;
        }
        c.g[A2][0] = c.lw(this.wrapping_add(DEC_UTK_COUNT));
        c.cycles += 10;
        c.g[A0][0] = c.lw(this.wrapping_add(DEC_UTK_STATE));
        c.g[A1][0] = c.lw(cfg);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        SET_INPUT.call(c, 0x003c_9500)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003578a8 Particle_ctor_2

/// The `Particle` modifier object's vtable (0x48ee60).
pub(crate) const PARTICLE_CTOR_VTABLE: u32 = 0x0049_0000 - 0x11a0;

/// `Particle* Particle_ctor_2(Particle* self, int list, WorldObj* object,
/// void* data, Node* node)` (0x3578a8): a modifier object of type 13 on
/// `object` (`ModObj_ctor`) with the particle vtable; it takes over the
/// node's object (`ModObj_TakeOverObject`) and adds a particle modifier
/// built from `data` (`ModObj_AddParticleModifier`). Returns `self`.
pub fn particle_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0xd; // the particle modifier type
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A3][0]; // data
        c.g[S1][0] = c.g[T0][0]; // node
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[V0][0]; // object
        Callee {
            addr: 0x0035_5280,
            f: ee::s11::ModObj_ctor,
        }
        .call(c, 0x0035_78d8)?;
        c.g[V1][0] = sx(PARTICLE_CTOR_VTABLE);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        Callee {
            addr: 0x0035_5f10,
            f: ee::s11::ModObj_TakeOverObject,
        }
        .call(c, 0x0035_78f0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0035_5cb0,
            f: ee::s11::ModObj_AddParticleModifier,
        }
        .call(c, 0x0035_78fc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `WorldTriggerManager_PlayTriggerSound`: muted (type 3), special
    /// (type 0: the real GetSpecialTriggerInfo's no-info path) and playable
    /// (type 1) triggers, ids inside and past the count; no event, the last
    /// event again, or a new one whose trigger ids (real
    /// cWorldSound_GetTriggerIds on an empty list, ids taken from the stack
    /// slots) are flagged or not (the real MarkPlayed); volumes below 0,
    /// inside, at and above 127; with or without a rider (the real
    /// GetRiderListenerIndex, in game or not). The push runs on the audio
    /// test world (real memset, gain lookup and `Audio_PlayPushedSound` with
    /// no bank). Not reached: a non-zero third info word (only trigger 0x4d,
    /// whose special path plays speech).
    #[test]
    fn world_trigger_manager_play_trigger_sound_matches() {
        use crate::difftest::{check, GAME_GP, SP as STACK_TOP};
        use crate::game::sound::soundsys::tests::audio;
        const MGR: u32 = audio::SYS + 0x8_0000;
        const TRIGGERS: u32 = MGR + 0x1000;
        const INFOS: u32 = MGR + 0x2000;
        const EVENT: u32 = MGR + 0x4000;
        const RIDER: u32 = MGR + 0x6000;
        check(
            ee::s08::f_002b5f60__generated,
            world_trigger_manager_play_trigger_sound,
            200,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x2b_5f60, 0x002b_5f60);
                c.bus.write32(GAME_GP + 0x410, audio::SYS); // SSXAudioSystem_GetInstance
                                                            // The manager: six triggers, nothing played yet.
                for i in 0..0x100 {
                    c.bus.write32(MGR + 4 * i, 0);
                }
                c.bus.write32(MGR + WTM_PLAY_TRIGGERS, TRIGGERS);
                c.bus.write32(MGR + WTM_PLAY_TRIGGER_COUNT, 6);
                c.bus.write32(MGR + WTM_PLAY_LAST_EVENT, EVENT + 0x100);
                c.bus.write32(MGR + 0x32c, 0x7777); // last played id
                for i in 0..8u32 {
                    let ty = match (seed + i) % 7 {
                        0 => 3,
                        1 => 0,
                        _ => 1,
                    };
                    let info = INFOS + 0x40 * i;
                    c.bus.write32(TRIGGERS + 8 * i, ty);
                    c.bus.write32(TRIGGERS + 8 * i + 4, info);
                    c.bus.write32(info + 4, (r.next_u32() % 2 == 0) as u32); // flag
                    c.bus.write32(info + 0x10, r.next_u32() % 4); // bank (none loaded)
                    c.bus.write32(info + 0x14, r.next_u32() % 256); // sample
                }
                // The event's trigger list (empty) and the stack slots
                // AnyTriggerFlagged reads the ids from.
                c.bus.write32(EVENT + 0x8c, EVENT + 0x200);
                c.bus.write32(EVENT + 0x200 + 0x10, EVENT + 0x300);
                c.bus.write32(EVENT + 0x300, 0);
                c.bus.write32(EVENT + 0x304, 0);
                let ids = STACK_TOP - 0xe0;
                for k in 0..4 {
                    c.bus.write32(ids + 4 * k, 0);
                }
                c.bus.write32(ids + 4 * (seed % 4), r.next_u32() % 8);
                // The rider: a fixed listener.
                c.bus.write32(RIDER + 0x874, 1);
                c.bus.write32(RIDER + 0x87c, 1);
                c.bus.write32(RIDER + 0x870, 3);
                // Arguments.
                c.g[A0][0] = sx(MGR);
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => sx(EVENT + 0x100),
                    _ => sx(EVENT),
                };
                c.g[A2][0] = (if seed % 5 == 4 {
                    r.next_u32() % 8
                } else {
                    r.next_u32() % 6
                }) as u64;
                c.g[A3][0] = if seed % 3 == 0 { 0 } else { sx(RIDER) };
                c.g[T0][0] = sx(match seed % 6 {
                    0 => (r.next_u32() % 100).wrapping_neg(),
                    1 => 127 + r.next_u32() % 100,
                    2 => 127,
                    3 => 0,
                    _ => r.next_u32() % 127,
                });
            },
        );
    }
    /// `WorldObjFactory_AttachModifier`: a node that already is a type 9, 6
    /// or 22 modifier, or already `type` (refresh: a fake), and for other
    /// nodes types 13 (particle) and 0 (debounce: the real allocator and
    /// constructors on a test heap; the node's "which transform" method
    /// returns the object's own matrix, so ModObj_TakeOverObject deletes the
    /// node through a fake) and unknown types (deleted through a fake, some
    /// unwinding). `IsType` is the real leaf 0x2afa58 (`*field == type`).
    /// Not reached: types 11 (mesh animation) and 22 (dead fade), whose
    /// constructors walk model data.
    #[test]
    fn world_obj_factory_attach_modifier_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{elf_image, DATA, GAME_GP};
        const NODE: u32 = DATA + 0x1000;
        const NODE_KIND: u32 = NODE + 0x40;
        const VT: u32 = DATA + 0x2000;
        const OBJECT: u32 = DATA + 0x3000;
        const BUF: u32 = DATA + 0x4000;
        const BELIB: u32 = DATA + 0x5000;
        covered(
            ee::s10::f_002fae38__generated,
            world_obj_factory_attach_modifier,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_ae38, 2);
                elf_image(c);
                crate::game::bx::bxstring::tests::heap(c);
                for a in (NODE..BELIB + 0x100).step_by(4) {
                    c.bus.write32(a, 0);
                }
                c.bus.write32(GAME_GP - 0x1f04, BELIB);
                // The node: vtable, kind field, its object.
                c.bus.write32(NODE + WOF_ATTACH_VTABLE, VT);
                c.bus.write32(NODE + WOF_ATTACH_OBJECT, OBJECT);
                c.bus.write16(VT + WOF_ATTACH_VT_IS_TYPE, 0x40);
                c.bus.write32(VT + WOF_ATTACH_VT_IS_TYPE + 4, 0x002a_fa58);
                c.bus.write32(VT + WOF_ATTACH_VT_REFRESH + 4, FAKE);
                c.bus.write32(VT + WOF_ATTACH_VT_DELETE + 4, FAKE + 0x10);
                c.bus.write32(VT + 0x84, 0x0014_06c0); // no particle modifiers
                c.bus.write32(VT + 0xc4, FAKE + 0x20); // its transform
                c.bus.write32(LOG_RESULTS, OBJECT + 0x10);
                c.bus.write32(OBJECT + 0x8, 0x100);
                c.bus.write32(OBJECT + 0xc, r.next_u32());
                let ty: u32 = match seed % 8 {
                    0 | 1 => 13,
                    2 | 3 => 0,
                    4 => [1, 5, 12, 14, 30][(seed / 8 % 5) as usize],
                    5 => (r.next_u32() % 40).wrapping_neg(),
                    _ => match r.next_u32() % 24 {
                        // Not the mesh-animation / dead-fade constructors.
                        t @ (11 | 22) => t + 1,
                        t => t,
                    },
                };
                let kind = match seed % 11 {
                    0 => 9,
                    1 => 6,
                    2 => 22,
                    3 => ty,
                    _ => 100,
                };
                c.bus.write32(NODE_KIND, kind);
                c.pc = 0x002f_ae38;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[A0][0] = sx(NODE);
                c.g[A1][0] = sx(ty);
                c.g[A2][0] = sx(BUF);
            },
        );
    }
    /// `Flag_Save`: the real `WorldObj_Save` / `SyncProcess_Save` on a
    /// random object; the stream's write methods are fakes, some unwinding
    /// (fixture as in `mod_obj_save_matches`).
    #[test]
    fn flag_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{fill, DATA};
        covered(ee::s11::f_0034aef8__generated, flag_save, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_aef8, 4);
            c.pc = 0x0034_aef8;
            let (this, stream) = (DATA + 0x100, DATA + 0x400);
            fill(c, &mut r, this, 0x10);
            let vt = vtable(c, &mut r, &[8, 0x30]);
            c.bus.write32(stream, vt);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
        });
    }

    /// `DynamicEmitter_Save`: random fields (floats as bit patterns); the
    /// stream's write method is a fake that sometimes unwinds.
    #[test]
    fn dynamic_emitter_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        covered(
            ee::s12::f_003712b8__generated,
            dynamic_emitter_save,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0037_12b8, 1);
                c.pc = 0x0037_12b8;
                let (this, stream) = (DATA + 0x100, DATA + 0x400);
                c.bus.write32(this, r.edge_f32());
                c.bus.write32(this + DYN_EMITTER_SAVE_RATE, r.f32(100.0));
                c.bus.write32(this + DYN_EMITTER_SAVE_COUNT_A, r.next_u32());
                c.bus.write32(this + DYN_EMITTER_SAVE_COUNT_B, r.next_u32());
                c.bus
                    .write32(this + DYN_EMITTER_SAVE_STATE, r.next_u32() % 4);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }
    /// `BankMonitor_StopHandle`: the real `BankMonitor_FindInstance` over
    /// 64 instances (no match, a match among decoys with the wrong id), a
    /// zero (also -0.0) or non-zero fade: the real `FadeVolume` / `Reset` on
    /// instance states (4.., negative) that need no SND call.
    #[test]
    fn bank_monitor_stop_handle_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const MONITOR: u32 = DATA + 0x100;
        const INSTANCES: u32 = DATA + 0x1000;
        check(
            ee::s08::f_002ad5f0__generated,
            bank_monitor_stop_handle,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_d5f0, 0);
                c.pc = 0x002a_d5f0;
                c.bus.write32(MONITOR + 0x8ec, INSTANCES);
                for a in (INSTANCES..INSTANCES + 64 * 0xc0).step_by(4) {
                    c.bus.write32(a, 0);
                }
                let kind = if seed % 5 == 0 {
                    (1 + r.next_u32() % 8).wrapping_neg()
                } else {
                    4 + r.next_u32() % 6
                };
                let id = r.next_u32() % 50;
                // Decoys: same kind, other id.
                for _ in 0..3 {
                    let k = r.next_u32() % 64;
                    c.bus.write32(INSTANCES + 0xc0 * k, kind);
                    c.bus.write32(INSTANCES + 0xc0 * k + 4, id + 1);
                }
                if seed % 4 != 0 {
                    let k = r.next_u32() % 64;
                    c.bus.write32(INSTANCES + 0xc0 * k, kind);
                    c.bus.write32(INSTANCES + 0xc0 * k + 4, id);
                    c.bus.write32(INSTANCES + 0xc0 * k + 0x54, r.next_u32() % 2);
                }
                c.g[A0][0] = sx(MONITOR);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = sx(kind);
                c.f[12] = match seed % 6 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => r.f32(5.0),
                };
            },
        );
    }
    /// `SSXAudioSystem_StartAmbienceLoop`: already started, or the push
    /// (0..2 entries queued before) and the real `Audio_PlayPushedSound`
    /// with no bank loaded, on the audio test world.
    #[test]
    fn ssx_audio_system_start_ambience_loop_matches() {
        use crate::difftest::check;
        use crate::game::sound::soundsys::tests::audio;
        check(
            ee::s07::f_002910e0__generated,
            ssx_audio_system_start_ambience_loop,
            120,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_10e0, 0x0029_10e0);
                c.bus.write32(
                    audio::SYS + AMBIENCE_LOOP_STARTED,
                    if seed % 4 == 0 {
                        1 + r.next_u32() % 3
                    } else {
                        0
                    },
                );
                c.bus.write32(audio::SYS + AMBIENCE_LOOP_GAIN, r.f32(1.0));
            },
        );
    }
    /// `SND_DecUtk_InitMem`: the allocation hook (0x517604) is a fake that
    /// returns the test's UTK state block (or unwinds); the real
    /// `SND_UtkReset` / `GetState` / `SetState` / `SetMode` / `SetInput`
    /// run, for decoder modes below and from 3 and inputs with and without
    /// the 0xee marker byte.
    #[test]
    fn snd_dec_utk_init_mem_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{fill, DATA};
        const THIS: u32 = DATA + 0x100;
        const CFG: u32 = DATA + 0x200;
        const INPUT: u32 = DATA + 0x400;
        const UTK: u32 = DATA + 0x2000;
        covered(
            ee::s14::f_003c9420__generated,
            snd_dec_utk_init_mem,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_9420, 1);
                c.pc = 0x003c_9420;
                c.bus.write32(0x0051_7604, FAKE); // SND_UtkAlloc's allocator hook
                c.bus.write32(LOG_RESULTS, UTK);
                fill(c, &mut r, THIS, 0x10);
                fill(c, &mut r, CFG, 0x10);
                fill(c, &mut r, UTK + 0xd40, 0x0a);
                c.bus.write32(UTK + 4, r.next_u32());
                c.bus.write32(CFG, INPUT);
                c.bus.write32(CFG + DEC_UTK_CFG_COUNT, r.next_u32() % 0x400);
                c.bus.write32(
                    CFG + 0x24,
                    if seed % 7 == 0 {
                        (r.next_u32() % 4).wrapping_neg()
                    } else {
                        r.next_u32() % 6
                    },
                );
                c.bus.write8(
                    INPUT,
                    if seed % 3 == 0 {
                        0xee
                    } else {
                        r.next_u32() as u8
                    },
                );
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(CFG);
            },
        );
    }
    /// `Particle_ctor_2`: a fresh block from a test heap on a test object
    /// (no "hidden" flag work: object flag 0x100 set); the real `ModObj_ctor`
    /// chain, `ModObj_TakeOverObject` (no node, or a node whose transform
    /// method, a fake, returns the object's own matrix so it is deleted
    /// through a fake, some unwinding) and `ModObj_AddParticleModifier`
    /// (real allocator and ParticleModifier constructor).
    #[test]
    fn particle_ctor_2_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{elf_image, fill, DATA, GAME_GP};
        const SELF: u32 = DATA + 0x100;
        const NODE: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x2000;
        const OBJECT: u32 = DATA + 0x3000;
        const BUF: u32 = DATA + 0x4000;
        const BELIB: u32 = DATA + 0x5000;
        covered(ee::s11::f_003578a8__generated, particle_ctor_2, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x35_78a8, 2);
            elf_image(c);
            crate::game::bx::bxstring::tests::heap(c);
            for a in (SELF..BELIB + 0x100).step_by(4) {
                c.bus.write32(a, 0);
            }
            c.bus.write32(GAME_GP - 0x1f04, BELIB);
            fill(c, &mut r, BUF, 0x10);
            c.bus.write32(NODE + 0xc, VT);
            c.bus.write32(VT + 0x84, 0x0014_06c0); // no particle modifiers
            c.bus.write32(VT + 0xc4, FAKE); // its transform
            c.bus.write32(VT + 0xc, FAKE + 0x10); // deleting destructor
            c.bus.write32(LOG_RESULTS, OBJECT + 0x10);
            c.bus.write32(OBJECT + 0x8, 0x100);
            c.pc = 0x0035_78a8;
            c.g[RA][0] = crate::difftest::RA as u64;
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = 1;
            c.g[A2][0] = sx(OBJECT);
            c.g[A3][0] = sx(BUF);
            c.g[T0][0] = if seed % 5 == 0 { 0 } else { sx(NODE) };
        });
    }
}
