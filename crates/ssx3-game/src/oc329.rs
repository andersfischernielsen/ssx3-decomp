//! Staging module oc329 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc329`.

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
        0x0011_9b08,
        rider_metrix_on_wipeout,
        "RiderMetrix_OnWipeout",
    ),
    (
        0x0037_0d60,
        dynamic_emitter_reset_particles,
        "DynamicEmitter_ResetParticles",
    ),
    (
        0x002a_d940,
        bank_monitor_timer_kill,
        "BankMonitor_TimerKill",
    ),
    (0x0031_d790, csi_cubic_spline_free, "cCSICubicSpline_Free"),
    (
        0x0030_57c0,
        wscript_fn_add_halo_modifier,
        "WScriptFn_AddHaloModifier",
    ),
    (
        0x002b_0f48,
        speech_system_on_spch_event,
        "SpeechSystem_OnSpchEvent",
    ),
    (0x0035_0f60, restore_node_ctor, "RestoreNode_ctor"),
    (
        0x003c_e1b0,
        snd_channel_start_stream_voices,
        "SND_ChannelStartStreamVoices",
    ),
    (
        0x002f_f1c8,
        wscript_fn_add_rail_modifier,
        "WScriptFn_AddRailModifier",
    ),
    (0x003d_6ee8, spch_history_push, "SPCH_HistoryPush"),
];

// ---------------------------------------------------------------------------
// RiderMetrix_OnWipeout (0x119b08)

const WIPEOUT_SHOW_HUD_NUMBER: Callee = Callee {
    addr: 0x0011_7b88,
    f: ee::s00::f_00117b88,
};

const WIPEOUT_BANK_PENDING_SCORE: Callee = Callee {
    addr: 0x0011_a7a8,
    f: ee::s00::f_0011a7a8,
};

const WIPEOUT_ROUNDED_TRICKS: Callee = Callee {
    addr: 0x0011_7948,
    f: ee::s00::f_00117948,
};

const WIPEOUT_RESET_TRICK: Callee = Callee {
    addr: 0x0011_7838,
    f: ee::s00::f_00117838,
};

const WIPEOUT_RESET_COMBO: Callee = Callee {
    addr: 0x0011_75f8,
    f: ee::s00::f_001175f8,
};

/// `float RiderMetrix_OnWipeout(cRiderMetrix* m, int hudEffect)` (0x119b08):
/// count the wipeout — in the HUD effect counter `+0x12c` (and flash HUD
/// number 0x2d at 1.5x) when `hudEffect` is set, else in the per-jump
/// counter `+0x124`; add the pending trick points to `+0x1a0`; return
/// -0.25 (the wipeout penalty) when the rounded trick points are positive,
/// else 0; then reset the trick and the combo chain.
pub fn rider_metrix_on_wipeout(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let hud_effect = c.g[A1][0] != 0;
        frame.save_fpr(c, 20, 0x20);
        if hud_effect {
            c.cycles += 11;
            let count = Addr(r32(c, S0)).off(0x12c).lw(c);
            c.g[V0][0] = count;
            c.g[AT][0] = 0x3fc0_0000; // 1.5
            c.mtc1(1, 12);
            c.g[A1][0] = 0x2d;
            c.g[V0][0] = sx((count as u32).wrapping_add(1));
            c.g[A2][0] = 0;
            let at = r32(c, S0);
            Addr(at).off(0x12c).sw(c, c.g[V0][0]);
            c.g[A3][0] = 0;
            WIPEOUT_SHOW_HUD_NUMBER.call(c, 0x0011_9b44)?;
        } else {
            c.cycles += 3;
            let count = Addr(r32(c, S0)).off(0x124).lw(c);
            c.g[V0][0] = count;
            c.g[V0][0] = sx((count as u32).wrapping_add(1));
            let at = r32(c, S0);
            Addr(at).off(0x124).sw(c, c.g[V0][0]);
        }
        // The wipeout penalty: f20 stays 0 unless the trick points are
        // positive.
        c.cycles += 10;
        c.mtc1(0, 20);
        c.g[A0][0] = c.g[S0][0];
        WIPEOUT_BANK_PENDING_SCORE.call(c, 0x0011_9b64)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x1a0).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let at = r32(c, S0);
        Addr(at).off(0x1a0).sw(c, c.g[V1][0]);
        WIPEOUT_ROUNDED_TRICKS.call(c, 0x0011_9b78)?;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[AT][0] = 0xffff_ffff_be80_0000; // -0.25
            c.mtc1(1, 20);
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        WIPEOUT_RESET_TRICK.call(c, 0x0011_9b90)?;
        c.g[A0][0] = c.g[S0][0];
        WIPEOUT_RESET_COMBO.call(c, 0x0011_9b98)?;
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DynamicEmitter_ResetParticles (0x370d60)

/// `void DynamicEmitter_ResetParticles(DynamicEmitter* e)` (0x370d60):
/// clear the live counter `+0x17c` and the loop state `+0x1e0`, then write
/// the zero vector (`0x4ff120`) into each of the `+0x178` position and
/// velocity entries `vec4A` arrays at `+0x1a0` and `+0x1a4`.
pub fn dynamic_emitter_reset_particles(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[A3][0] = 0;
    c.g[V0][0] = Addr(r32(c, A2)).off(0x178).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    let at = r32(c, A2);
    Addr(at).off(0x17c).sw(c, 0);
    if !empty {
        c.cycles += 3;
        c.g[V0][0] = 0x0050_0000;
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0)); // 0x4ff120
        loop {
            c.cycles += 15;
            c.g[V1][0] = Addr(r32(c, A2)).off(0x1a0).lw(c);
            c.g[A1][0] = sx((c.g[A3][0] as u32) << 4);
            let zero = r32(c, T0);
            c.lq(2, zero);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let pos = r32(c, V1);
            c.sq(2, pos);
            c.g[A0][0] = Addr(r32(c, A2)).off(0x1a4).lw(c);
            let zero = r32(c, T0);
            c.lq(2, zero);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let vel = r32(c, A1);
            c.sq(2, vel);
            let at = r32(c, A2);
            Addr(at).off(0x17c).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, A2)).off(0x178).lw(c);
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            let at = r32(c, A2);
            Addr(at).off(0x1e0).sw(c, 0);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_0d80);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// BankMonitor_TimerKill (0x2ad940)

const BANKKILL_TIMER_DISABLE: Callee = Callee {
    addr: 0x002a_d990,
    f: ee::s08::f_002ad990,
};

const BANKKILL_TIMER_SET_PERIOD: Callee = Callee {
    addr: 0x002a_d970,
    f: ee::s08::f_002ad970,
};

/// `void BankMonitor_TimerKill(BankMonitor* m)` (0x2ad940): disarm the timer
/// (`m+0` = 0) and set its period to -1 (`m+4` and `m+8`).
pub fn bank_monitor_timer_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        BANKKILL_TIMER_DISABLE.call(c, 0x002a_d954)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = u64::MAX;
        BANKKILL_TIMER_SET_PERIOD.call(c, 0x002a_d960)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cCSICubicSpline_Free (0x31d790)

const SPLINEFREE_MEM_FREE: Callee = Callee {
    addr: 0x0031_7e98,
    f: ee::s10::f_00317e98,
};

/// `void cCSICubicSpline_Free(cCSICubicSpline* s)` (0x31d790): free the point
/// array `s+8` when present, then clear its "allocated"/"solved" bits in the
/// word at `s+0` (masks 2 and 4).
pub fn csi_cubic_spline_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(0x8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            SPLINEFREE_MEM_FREE.call(c, 0x0031_d7b4)?;
            let at = r32(c, S0);
            Addr(at).off(0x8).sw(c, 0);
        }
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.cycles += 9;
        c.g[V1][0] = 0xffff_ffff_ffff_fffd; // ~2
        c.g[A0][0] = 0xffff_ffff_ffff_fffb; // ~4
        frame.restore_ra(c, 0);
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] &= c.g[A0][0];
        let at = r32(c, S0);
        Addr(at).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddHaloModifier (0x3057c0)

/// The halo-modifier parameter record built on every call (`0x4fbd68`), and
/// its static-init guard (`gp+0x2764`).
const HALO_TEMPLATE: u32 = 0x004f_bd68;

const HALO_TEMPLATE_GUARD: u32 = 0x2764;

/// The script-parameter type table the modifier loop reads: `0x440000`
/// plus this (`0x4466b0`).
pub(crate) const HALO_PARAM_TYPES_OFF: u32 = 0x66b0;

/// `cLunoVar WScriptFn_AddHaloModifier(cLunoVar* out, int n, WsParam* params)`
/// (0x3057c0): copy the halo-modifier record template (`0x4fbd68`, built on
/// the first call), apply the `n` script parameters to the copy (integers
/// copied, floats converted for type-2 entries), resolve the record's id to
/// an object, and when that object is a move node with a live halo interface
/// add the halo modifier. The record (12 bytes) is returned in `*out`.
pub fn wscript_fn_add_halo_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(HALO_TEMPLATE_GUARD)).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !inited {
            // First call: build the template in place. The trailing byte is
            // passed through unchanged (the copy constructor's byte).
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4298));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[AT][0] = 0x3f80_0000; // 1.0
            c.mtc1(1, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A2][0] = Addr(HALO_TEMPLATE.wrapping_add(0x24)).lbu(c);
            c.g[A0][0] = u64::MAX;
            c.g[AT][0] = 0x42c8_0000; // 100.0
            c.mtc1(1, 1);
            c.g[A1][0] = 1;
            Addr(HALO_TEMPLATE).sw(c, c.g[V1][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x24)).sb(c, c.g[A2][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x8)).sw(c, c.g[A0][0]);
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x18));
            c.swc1(1, HALO_TEMPLATE.wrapping_add(0x1c));
            let guard = r32(c, GP).wrapping_add(HALO_TEMPLATE_GUARD);
            Addr(guard).sw(c, c.g[A1][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x4)).sw(c, 0);
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0xc));
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x10));
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x14));
            Addr(HALO_TEMPLATE.wrapping_add(0x20)).sw(c, 0);
            let spv = sp(c);
            c.sb(spv, c.g[A2][0]);
        }
        'b_3058c8: {
            // The stack copy of the record, id preset to -1.
            c.cycles += 10;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            let spv = sp(c);
            c.g[S1][0] = sx(spv.wrapping_add(0x10));
            c.g[V0][0] |= 0xffff;
            c.g[A2][0] = 0x28;
            c.g[A1][0] = sx(HALO_TEMPLATE);
            let buf = r32(c, S1);
            c.sw(buf, c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003e_6574, 0x0030_5868)?; // EA_MemCopy
            let no_params = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if no_params {
                break 'b_3058c8;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(HALO_PARAM_TYPES_OFF));
            c.g[A3][0] = 2;
            loop {
                // One script parameter: `idx` selects the record word, the
                // type table decides whether a value is converted to float.
                'b_3058bc: {
                    'b_3058b8: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A2)).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_3058b8;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_3058b8;
                        }
                        c.cycles += 4;
                        let val = r32(c, A2).wrapping_add(4);
                        c.lwc1(0, val);
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'b_3058bc;
                    }
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    break 'b_3058bc;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_3058c8;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_5880);
                }
            }
        }
        // Resolve the record's id to an object (v0), -1 meaning "the
        // currently controlled object".
        'b_305924: {
            {
                c.cycles += 4;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let not_minus_one = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x16c8)).lw(c);
                if !not_minus_one {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V0).wrapping_add(0x290)).lw(c);
                    break 'b_305924;
                }
            }
            {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let no_slot = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                if !no_slot {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if found {
                        break 'b_305924;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // A live move node: call its halo-interface test and, when it says
        // yes, add the modifier.
        'b_305964: {
            'b_305960: {
                c.cycles += 2;
                let no_obj = c.g[V0][0] == 0;
                c.g[V1][0] = 0x004d_0000;
                if no_obj {
                    break 'b_305960;
                }
                c.cycles += 3;
                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                let no_node = c.g[S0][0] == 0;
                c.g[V0][0] = c.g[S3][0];
                if no_node {
                    break 'b_305964;
                }
                {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_594c)?;
                    let no_interface = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !no_interface {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0035_5eb0, 0x0030_595c)?; // addHalo
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
                break 'b_305960;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
            break 'b_305964;
        }
        // Return the record by value: 12 bytes from the static result.
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let res = r32(c, A3);
        c.ldl(4, res.wrapping_add(7));
        c.ldr(4, res);
        c.g[A1][0] = Addr(res).off(0x8).lw(c);
        let out = r32(c, S3);
        c.sdl(4, out.wrapping_add(7));
        c.sdr(4, out);
        Addr(out).off(0x8).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_OnSpchEvent (0x2b0f48)

const SPCH_SPEECH_PLAY_EVENT: Callee = Callee {
    addr: 0x002b_04d8,
    f: ee::s08::f_002b04d8,
};

/// `int SpeechSystem_OnSpchEvent(SpeechSystem* self, SpeechEvent* ev)`
/// (0x2b0f48): hand the event to the first of the two speech instances
/// (`self+4`, instance pointers) that plays it (a non-negative result),
/// calling the instance's callback (`self+0xc`) with the result when the
/// instance index `self+0x10` matches. -1 when neither plays it.
pub fn speech_system_on_spch_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S2)).off(0x4).lw(c);
        'l_2b0f70: loop {
            'b_2b0fb8: {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    break 'b_2b0fb8;
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[S3][0];
                SPCH_SPEECH_PLAY_EVENT.call(c, 0x002b_0f8c)?;
                c.g[S0][0] = c.g[V0][0];
                if (c.g[S0][0] as i64) < 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    break 'b_2b0fb8;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S2)).off(0x10).lw(c);
                if c.g[V0][0] != c.g[S1][0] {
                    c.g[V0][0] = c.g[S0][0];
                    break 'l_2b0f70;
                }
                // The active instance's callback.
                c.cycles += 5;
                c.g[V0][0] = Addr(r32(c, S2)).off(0xc).lw(c);
                let callback = c.g[V0][0] as u32;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callback, 0x002b_0fb0)?;
                c.g[V0][0] = c.g[S0][0];
                break 'l_2b0f70;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break 'l_2b0f70;
            }
            c.g[V1][0] = Addr(r32(c, S2)).off(0x4).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002b_0f70);
            }
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
// RestoreNode_ctor (0x350f60)

const RESTORENODE_WORLD_OBJ_CTOR: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};

/// `RestoreNode* RestoreNode_ctor(RestoreNode* self, void* node)` (0x350f60):
/// construct the world object as kind 0x13 with `node`, install the
/// `RestoreNode` vtable (`0x491680`), then clear bits 5/6 of the node's
/// flags (+8) and set bit 2 (restore/dormant behaviour).
pub fn restore_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 1;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x13;
        RESTORENODE_WORLD_OBJ_CTOR.call(c, 0x0035_0f80)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A2][0] = Addr(r32(c, S0)).off(0x18).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1680));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        let at = r32(c, S0);
        Addr(at).off(0xc).sw(c, c.g[V1][0]);
        c.g[A0][0] |= 0xff9f; // ~0x60
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, A2)).off(0x8).lw(c);
        c.g[A1][0] |= 0xfffd; // ~2
        frame.restore_ra(c, 0);
        c.g[V1][0] &= c.g[A0][0];
        let node = r32(c, A2);
        Addr(node).off(0x8).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, S0)).off(0x18).lw(c);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = Addr(r32(c, A0)).off(0x8).lw(c);
        c.g[V1][0] &= c.g[A1][0];
        c.g[V1][0] |= 0x4;
        let node = r32(c, A0);
        Addr(node).off(0x8).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_ChannelStartStreamVoices (0x3ce1b0)

const SND_VOICE_START: Callee = Callee {
    addr: 0x003c_8158,
    f: ee::s14::f_003c8158,
};

const SND_VOICE_ACTIVATE: Callee = Callee {
    addr: 0x003c_83a0,
    f: ee::s14::f_003c83a0,
};

/// `int SND_ChannelStartStreamVoices(SND_Channel* ch, int entryOff,
/// void* stream, int rate, int a3, int t0, int t1, int t2, int t3)`
/// (0x3ce1b0): start the stream voices of the channel entry
/// `*(soundState+0x1f0) + entryOff`. Nothing happens when the entry's flags
/// (+0x24) say "done" (bit 3) or "not streaming" (bit 2 missing); otherwise
/// each of the `+0x23` voices is started with its rate, the seven channel
/// parameters are applied, and each voice is activated.
pub fn snd_channel_start_stream_voices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = 0x8c;
        frame.save(c, S1, 0xc0);
        c.mult(0, A1, A1, V0); // entryOff * sizeof(entry)
        frame.save(c, S4, 0x90);
        c.g[V1][0] = 0x0051_0000;
        frame.save(c, S7, 0x60);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5718));
        frame.save(c, FP, 0x50);
        c.g[S7][0] = c.g[T2][0];
        frame.save(c, S0, 0xd0);
        c.g[FP][0] = c.g[A2][0];
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x40);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1f0).lw(c);
        let spv = sp(c);
        Addr(spv).off(0x30).sw(c, c.g[A3][0]);
        Addr(spv).off(0x34).sw(c, c.g[T0][0]);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        Addr(spv).off(0x38).sw(c, c.g[T1][0]);
        c.g[A0][0] = Addr(r32(c, S1)).off(0x24).lhu(c);
        c.g[V0][0] = c.g[A0][0] & 0x8;
        let done = c.g[V0][0] != 0;
        c.g[S4][0] = c.g[T3][0];
        'b_3ce348: {
            'b_3ce344: {
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] & 0x4;
                    let not_streaming = c.g[V0][0] == 0;
                    let spv = sp(c);
                    c.lq(16, spv.wrapping_add(0xd0));
                    if not_streaming {
                        break 'b_3ce348;
                    }
                    'b_3ce2b0: {
                        c.cycles += 3;
                        c.g[T2][0] = Addr(r32(c, S1)).off(0x23).lbu(c);
                        let no_voices = c.g[T2][0] == 0;
                        c.g[S2][0] = 0;
                        if no_voices {
                            break 'b_3ce2b0;
                        }
                        c.cycles += 6;
                        c.g[S6][0] = c.g[V1][0];
                        c.g[S5][0] = u64::MAX;
                        c.g[S3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = Addr(r32(c, S4)).off(0x7).lbu(c);
                        loop {
                            c.cycles += 26;
                            c.g[T0][0] = 0xffff_ffff_ffff_0000;
                            c.g[V1][0] = Addr(r32(c, S0)).lh(c);
                            c.g[A2][0] = 0;
                            let spv = sp(c);
                            c.sw(spv.wrapping_add(0x28), c.g[S2][0]);
                            c.g[A3][0] = 0;
                            let spv = sp(c);
                            c.sw(spv.wrapping_add(0x20), c.g[V0][0]);
                            c.g[T0][0] |= 0xffff;
                            c.g[V0][0] = Addr(r32(c, S6)).off(0x2f).lbu(c);
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                            c.g[A0][0] = Addr(r32(c, S6)).off(0x30).lbu(c);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                            let spv = sp(c);
                            c.sw(spv, c.g[S5][0]);
                            c.sw(spv.wrapping_add(0x8), c.g[S5][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            let spv = sp(c);
                            c.sw(spv.wrapping_add(0x10), 0);
                            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                            c.sw(spv.wrapping_add(0x18), 0);
                            c.g[T1][0] = Addr(r32(c, S3)).lw(c);
                            c.g[A1][0] = Addr(r32(c, S7)).off(0x3).lbu(c);
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[T3][0] = Addr(r32(c, S1)).off(0x14).lw(c);
                            SND_VOICE_START.call(c, 0x003c_e2a0)?;
                            c.g[T2][0] = Addr(r32(c, S1)).off(0x23).lbu(c);
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[T2][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break 'b_3ce2b0;
                            }
                            c.g[V0][0] = Addr(r32(c, S4)).off(0x7).lbu(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x003c_e248);
                            }
                        }
                    }
                    c.cycles += 23;
                    c.g[A0][0] = Addr(r32(c, S1)).off(0x4).lh(c);
                    c.g[S2][0] = 0;
                    call_ptr(c, 0x003c_25e0, 0x003c_e2bc)?; // pitch
                    c.g[A0][0] = Addr(r32(c, S1)).off(0x4).lh(c);
                    call_ptr(c, 0x003c_38e0, 0x003c_e2c4)?; // pan
                    c.g[A0][0] = Addr(r32(c, S1)).off(0x4).lh(c);
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x003c_27e0, 0x003c_e2d0)?; // send
                    c.g[A0][0] = Addr(r32(c, S1)).off(0x4).lh(c);
                    c.g[A1][0] = c.g[FP][0];
                    call_ptr(c, 0x003c_3010, 0x003c_e2dc)?; // time stretch
                    let spv = sp(c);
                    c.g[A1][0] = Addr(spv).off(0x30).lw(c);
                    c.g[A0][0] = Addr(r32(c, S1)).off(0x4).lh(c);
                    call_ptr(c, 0x003c_30c8, 0x003c_e2e8)?; // distortion
                    let spv = sp(c);
                    c.g[A1][0] = Addr(spv).off(0x34).lw(c);
                    c.g[A0][0] = Addr(r32(c, S1)).off(0x4).lh(c);
                    call_ptr(c, 0x003c_3178, 0x003c_e2f4)?; // lowpass
                    c.g[A0][0] = Addr(r32(c, S1)).off(0x4).lh(c);
                    let spv = sp(c);
                    c.g[A1][0] = Addr(spv).off(0x38).lw(c);
                    call_ptr(c, 0x003c_3250, 0x003c_e300)?; // highpass
                    c.g[V0][0] = Addr(r32(c, S1)).off(0x23).lbu(c);
                    let no_voices = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x0051_0000;
                    if no_voices {
                        break 'b_3ce344;
                    }
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                    c.g[V0][0] = Addr(r32(c, S3)).off(0x2f).lbu(c);
                    loop {
                        c.cycles += 11;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                        c.g[A0][0] = Addr(r32(c, S3)).off(0x30).lbu(c);
                        c.g[V1][0] = Addr(r32(c, S0)).lh(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                        SND_VOICE_ACTIVATE.call(c, 0x003c_e334)?;
                        c.g[V0][0] = Addr(r32(c, S1)).off(0x23).lbu(c);
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break 'b_3ce344;
                        }
                        c.g[V0][0] = Addr(r32(c, S3)).off(0x2f).lbu(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_e318);
                        }
                    }
                }
            }
            c.cycles += 1;
            let spv = sp(c);
            c.lq(16, spv.wrapping_add(0xd0));
        }
        c.cycles += 12;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddRailModifier (0x2ff1c8)

/// The rail-modifier record template (`0x4fba28`), its static-init guard
/// (`gp+0x2540`) and its parameter type table (`0x446470`).
const RAIL_TEMPLATE: u32 = 0x004f_ba28;

const RAIL_TEMPLATE_GUARD: u32 = 0x2540;

pub(crate) const RAIL_PARAM_TYPES_OFF: u32 = 0x6470;

/// `cLunoVar WScriptFn_AddRailModifier(cLunoVar* out, int n, WsParam* params)`
/// (0x2ff1c8): same shape as `WScriptFn_AddHaloModifier` for the rail
/// modifier: copy the 16-byte template `0x4fba28` (built on the first call),
/// apply the parameters, resolve the object from the record's first word and
/// — only when the second word (the rail id) is not -1 — add the rail
/// modifier to the object's move node.
pub fn wscript_fn_add_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(RAIL_TEMPLATE_GUARD)).lw(c);
        frame.save(c, S0, 0x60);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !inited {
            c.cycles += 12;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x45d8));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = Addr(RAIL_TEMPLATE.wrapping_add(0xc)).lbu(c);
            c.g[V0][0] |= 0xffff;
            Addr(RAIL_TEMPLATE).sw(c, c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            Addr(RAIL_TEMPLATE.wrapping_add(0xc)).sb(c, c.g[A2][0]);
            c.g[A1][0] = 1;
            Addr(RAIL_TEMPLATE.wrapping_add(0x4)).sw(c, c.g[V0][0]);
            Addr(RAIL_TEMPLATE.wrapping_add(0x8)).sw(c, c.g[A0][0]);
            let guard = r32(c, GP).wrapping_add(RAIL_TEMPLATE_GUARD);
            Addr(guard).sw(c, c.g[A1][0]);
            let spv = sp(c);
            c.sb(spv, c.g[A2][0]);
        }
        'b_2ff2b0: {
            c.cycles += 11;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            let spv = sp(c);
            c.g[S1][0] = sx(spv.wrapping_add(0x10));
            c.g[V0][0] |= 0xffff;
            c.g[A2][0] = 0x10;
            c.g[A1][0] = sx(RAIL_TEMPLATE);
            c.sw(spv.wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw(spv.wrapping_add(0x10), c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x002f_f24c)?; // EA_MemCopy
            let no_params = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if no_params {
                break 'b_2ff2b0;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(RAIL_PARAM_TYPES_OFF));
            c.g[A3][0] = 2;
            loop {
                'b_2ff2a4: {
                    'b_2ff2a0: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A2)).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_2ff2a0;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_2ff2a0;
                        }
                        c.cycles += 4;
                        let val = r32(c, A2).wrapping_add(4);
                        c.lwc1(0, val);
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'b_2ff2a4;
                    }
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    break 'b_2ff2a4;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_2ff2b0;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_f268);
                }
            }
        }
        // Resolve the record's id (word 0) to an object; the result is v1.
        'b_2ff310: {
            {
                c.cycles += 4;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let not_minus_one = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x16c8)).lw(c);
                if !not_minus_one {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x290)).lw(c);
                    break 'b_2ff310;
                }
            }
            'b_2ff30c: {
                {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    let no_slot = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if !no_slot {
                        c.cycles += 7;
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let found = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if found {
                            break 'b_2ff30c;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // The rail id (word 1) must be set, the node alive and its rail
        // interface present.
        'b_2ff354: {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S1)).off(0x4).lw(c);
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0x004d_0000;
                break 'b_2ff354;
            }
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
            let no_node = c.g[S0][0] == 0;
            c.g[V1][0] = 0x004d_0000;
            if no_node {
                break 'b_2ff354;
            }
            {
                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002f_f340)?;
                let no_interface = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_interface {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0035_5e38, 0x002f_f350)?; // addRail
                }
            }
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
            break 'b_2ff354;
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let res = r32(c, A3);
        c.ldl(4, res.wrapping_add(7));
        c.ldr(4, res);
        c.g[A1][0] = Addr(res).off(0x8).lw(c);
        let out = r32(c, S3);
        c.sdl(4, out.wrapping_add(7));
        c.sdr(4, out);
        Addr(out).off(0x8).sw(c, c.g[A1][0]);
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
// SPCH_HistoryPush (0x3d6ee8)

/// `void SPCH_HistoryPush(u8* hist, u8 value)` (0x3d6ee8): push `value` into
/// the speech history ring. The header at `hist+6` is the ring length, `+4`
/// its flag/length byte, `+5` the wrap byte; the data starts at
/// `hist + ((wrap * ((flags & 0xf) + 2) + 0xf) & 0x3ffc) + (flags & 0xf) * 4`
/// plus a skip count when bit 7 of the flags is set. The byte at the write
/// position is the countdown of occupied entries; the value goes one past
/// it, and the count wraps to 0 at the ring length.
pub fn spch_history_push(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[T1][0] = Addr(r32(c, A3)).off(0x6).lbu(c);
    let empty = c.g[T1][0] == 0;
    c.g[T2][0] = 0;
    if !empty {
        {
            c.cycles += 12;
            c.g[A0][0] = Addr(r32(c, A3)).off(0x4).lbu(c);
            c.g[A2][0] = Addr(r32(c, A3)).off(0x5).lbu(c);
            c.g[V1][0] = c.g[A0][0] & 0xf;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
            c.g[A0][0] &= 0x80;
            c.mult(0, V0, A2, V0);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
            c.g[V0][0] &= 0x3ffc;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let no_skip = c.g[A0][0] == 0;
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if !no_skip {
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7));
                c.g[V1][0] = u64::MAX;
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xe));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            }
        }
        {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
            c.g[V0][0] = (c.g[V1][0] < c.g[T1][0]) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if !full {
                c.cycles += 1;
                c.sb(r32(c, V0).wrapping_add(1), c.g[A1][0]);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(r32(c, A3)).off(0x6).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V1][0] &= 0xff;
        c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.sb(r32(c, A0), c.g[V1][0]);
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{fake, machine};
    use crate::difftest::{check, DATA, GAME_GP};

    /// Gated slot replacements (the suite runs tests in parallel): a fake
    /// runs only while this magic is in the machine's RAM, else the generated
    /// body (the slot's default: the tests do not install the ports).
    const OC329_CTRL: u32 = DATA + 0x20_0000;
    const OC329_MAGIC: u32 = 0x0c32_9f01;

    type GuestBody = fn(&mut Cpu);

    /// Installs a gated fake in a callee's override slot and puts the
    /// previous body back on drop.
    struct Oc329Slot {
        slot: *mut GuestBody,
        prev: GuestBody,
    }

    impl Oc329Slot {
        fn install(slot: *mut GuestBody, fake: GuestBody) -> Oc329Slot {
            unsafe {
                let prev = *slot;
                *slot = fake;
                Oc329Slot { slot, prev }
            }
        }
    }

    impl Drop for Oc329Slot {
        fn drop(&mut self) {
            unsafe {
                *self.slot = self.prev;
            }
        }
    }

    // -- RiderMetrix_OnWipeout -------------------------------------------

    #[test]
    fn rider_metrix_on_wipeout_matches() {
        check(
            ee::s00::f_00119b08__generated,
            rider_metrix_on_wipeout,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x11_9b08, 0);
                c.pc = 0x0011_9b08;
                let m = DATA + 0x100;
                // No HUD message table: the HUD helpers take their null
                // path, whatever the score fields hold.
                c.bus.write32(m + 0x1b0, 0);
                c.bus.write32(m + 0x124, seed % 5);
                c.bus.write32(m + 0x12c, seed % 7);
                c.bus
                    .write32(m + 0x1a0, if seed % 4 == 0 { 0 } else { 1000 });
                c.bus.write32(m + 0x84, if seed % 3 == 0 { 0 } else { 250 });
                c.bus.write32(m + 0xa0, if seed % 3 == 1 { 0 } else { 40 });
                // Trick scale 1.0, points 0 or 10: the rounded trick points
                // straddle zero, so the -0.25 penalty branch happens.
                c.bus.write32(m + 0x14, 0x3f80_0000);
                c.bus
                    .write32(m + 0x1c4, if seed % 2 == 0 { 0 } else { 10.0f32.to_bits() });
                c.bus.write32(m + 0x1c, 0);
                c.bus.write32(GAME_GP - 0x7b08, 0x3f80_0000); // 1.0
                c.bus.write32(GAME_GP - 0x7b0c, 0x3f80_0000);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    // -- DynamicEmitter_ResetParticles -----------------------------------

    #[test]
    fn dynamic_emitter_reset_particles_matches() {
        check(
            ee::s12::f_00370d60__generated,
            dynamic_emitter_reset_particles,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37_0d60, 0);
                c.pc = 0x0037_0d60;
                let e = DATA + 0x400;
                c.bus.write32(e + 0x178, seed % 5);
                c.bus.write32(e + 0x17c, 3);
                c.bus.write32(e + 0x1a0, DATA + 0x2000);
                c.bus.write32(e + 0x1a4, DATA + 0x3000);
                // The zero vector the loop copies from 0x4ff120.
                for i in 0..4u32 {
                    c.bus.write32(0x004f_f120 + 4 * i, r.next_u32());
                }
                for i in 0..0x40u32 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
                    c.bus.write32(DATA + 0x3000 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(e);
            },
        );
    }

    // -- BankMonitor_TimerKill -------------------------------------------

    #[test]
    fn bank_monitor_timer_kill_matches() {
        check(
            ee::s08::f_002ad940__generated,
            bank_monitor_timer_kill,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_d940, 0);
                c.pc = 0x002a_d940;
                let m = DATA + 0x300;
                for i in 0..4u32 {
                    c.bus.write32(m + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    // -- cCSICubicSpline_Free --------------------------------------------

    fn oc329_fake_ea_mem_free(c: &mut Cpu) {
        if c.bus.read32(OC329_CTRL) != OC329_MAGIC {
            return ee::s10::f_00317e98__generated(c);
        }
        fake(c, 0x0031_7e98, c.g[RA][0] as u32);
    }

    #[test]
    fn csi_cubic_spline_free_matches() {
        let _slot = Oc329Slot::install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc329_fake_ea_mem_free,
        );
        check(
            ee::s10::f_0031d790__generated,
            csi_cubic_spline_free,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x31_d790, 0);
                c.pc = 0x0031_d790;
                c.bus.write32(OC329_CTRL, OC329_MAGIC);
                let s = DATA + 0x200;
                c.bus.write32(s, seed.wrapping_mul(0x9e37_79b9));
                // The point array is absent in half the seeds.
                c.bus
                    .write32(s + 8, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
                c.g[A0][0] = sx(s);
            },
        );
    }

    // -- WScriptFn_AddHaloModifier / AddRailModifier ----------------------

    /// The record template guard, the id tables at `gp+0x16c8` / `gp+0xce8`,
    /// the object → node → vtable chain, and the script parameters both
    /// modifier ports consume. `types` is the parameter type table.
    /// Modes: 0 the record keeps id -1, 1 the id resolves to nothing, 2 the
    /// object has no node, 3 the node's halo/rail interface answers `0`.
    fn modifier_fixture(c: &mut Cpu, seed: u32, types: u32) {
        let params = DATA + 0x1000;
        let node = DATA + 0x3000;
        let vt = DATA + 0x3100;
        let obj = DATA + 0x3200;
        let holder = DATA + 0x3400;
        let slots = DATA + 0x3500;
        let second = DATA + 0x3600;
        let id = (2u32 << 8) | 1;
        let mode = (seed / 2) % 4;
        let inited = (seed >> 1) & 1;
        let n = if mode == 0 { (seed / 8) % 2 } else { 3 };

        c.bus.write32(GAME_GP + 0x2764, inited);
        c.bus.write32(GAME_GP + 0x2540, inited);
        // Type table: index 0 integer, index 1 float conversion, index 2
        // matching integer.
        for (i, t) in [0u32, 2, 2, 0, 0, 0, 0, 0].iter().enumerate() {
            c.bus.write32(types + 4 * i as u32, *t);
        }
        if n > 0 {
            if mode == 0 {
                // A parameter that leaves the record's id alone.
                c.bus.write32(params, 3);
                c.bus.write32(params + 4, 0x4142_0000);
                c.bus.write32(params + 0xc, 0);
            } else {
                c.bus.write32(params, 0); // idx 0: the object id
                c.bus.write32(params + 4, id);
                c.bus.write32(params + 0xc, 3);
            }
        }
        if n > 1 {
            c.bus.write32(params + 0x10, 1); // idx 1: a float
            c.bus.write32(params + 0x14, 0x4120_0000);
            c.bus.write32(params + 0x1c, 3);
        }
        if n > 2 {
            c.bus.write32(params + 0x20, 2); // idx 2: an integer
            c.bus.write32(params + 0x24, 0x1122_3344);
            c.bus.write32(params + 0x2c, 2);
        }
        // The "current object" for id -1: *(*(gp+0xce8)+0x290).
        let p = DATA + 0x3700;
        c.bus.write32(GAME_GP + 0xce8, p);
        c.bus.write32(p + 0x290, DATA + 0x3780);
        // The id table: *D_004A47B8 -> holder -> slots -> slot -> second.
        c.bus.write32(GAME_GP + 0x16c8, holder);
        c.bus.write32(holder, 0);
        c.bus.write32(holder + 8, slots);
        c.bus.write32(slots, 0);
        c.bus
            .write32(slots + 4, if mode >= 2 { slots + 0x50 } else { 0 });
        c.bus.write32(slots + 0x50 + 0x1c, second);
        c.bus.write32(second + 2 * 4, (obj >> 2) << 8);
        c.bus.write32(obj + 0xc, if mode >= 3 { node } else { 0 });
        c.bus.write32(node + 0xc, vt);
        c.bus.write16(vt + 0x80, 0);
        // A real leaf that answers 0, so the add-modifier call is skipped.
        c.bus.write32(vt + 0x84, 0x0014_06c0);
        c.g[A0][0] = sx(DATA + 0x200);
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(params);
    }

    #[test]
    fn wscript_fn_add_halo_modifier_matches() {
        check(
            ee::s10::f_003057c0__generated,
            wscript_fn_add_halo_modifier,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x30_57c0, 0);
                c.pc = 0x0030_57c0;
                modifier_fixture(c, seed, 0x0044_0000 + HALO_PARAM_TYPES_OFF);
            },
        );
    }

    #[test]
    fn wscript_fn_add_rail_modifier_matches() {
        check(
            ee::s10::f_002ff1c8__generated,
            wscript_fn_add_rail_modifier,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x2f_f1c8, 0);
                c.pc = 0x002f_f1c8;
                modifier_fixture(c, seed, 0x0044_0000 + RAIL_PARAM_TYPES_OFF);
            },
        );
    }

    // -- SpeechSystem_OnSpchEvent ----------------------------------------

    fn oc329_fake_speech_play_event(c: &mut Cpu) {
        if c.bus.read32(OC329_CTRL) != OC329_MAGIC || c.bus.read32(OC329_CTRL + 4) == 0 {
            return ee::s08::f_002b04d8__generated(c);
        }
        fake(c, 0x002b_04d8, c.g[RA][0] as u32);
    }

    #[test]
    fn speech_system_on_spch_event_matches() {
        let _slot = Oc329Slot::install(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002B04D8),
            oc329_fake_speech_play_event,
        );
        check(
            ee::s08::f_002b0f48__generated,
            speech_system_on_spch_event,
            160,
            &|c, seed| {
                let _r = machine(c, seed, 0x2b_0f48, 0);
                c.pc = 0x002b_0f48;
                c.bus.write32(OC329_CTRL, OC329_MAGIC);
                let sys = DATA + 0x200;
                let insts = DATA + 0x400;
                let ev = DATA + 0x800;
                // The real callee finds no bank (id 0x1234 in an empty bank
                // table) and answers -1.
                let use_fake = (seed % 3 != 0) as u32;
                c.bus.write32(OC329_CTRL + 4, use_fake);
                c.bus.write32(
                    crate::difftest::fakes::LOG_RESULTS,
                    if use_fake == 1 { 5 } else { 0 },
                );
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS + 4, 7);
                // Instance 0 and 1: present or absent.
                for i in 0..2u32 {
                    let inst = insts + 0x40 * i;
                    let present = (seed >> i) & 1 != 0;
                    c.bus
                        .write32(sys + 4 + 4 * i, if present { inst } else { 0 });
                    c.bus.write32(inst + 0xf0, 0); // no banks
                    c.bus.write32(inst + 0xf4, DATA + 0x1000);
                }
                // The active instance index and the callback (a leaf that
                // returns 0).
                c.bus.write32(sys + 0xc, 0x0014_06c0);
                c.bus.write32(sys + 0x10, (seed % 3) as u32);
                c.bus.write32(ev, 0x1234);
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(ev);
            },
        );
    }

    // -- RestoreNode_ctor -------------------------------------------------

    fn oc329_fake_world_obj_ctor(c: &mut Cpu) {
        if c.bus.read32(OC329_CTRL) != OC329_MAGIC {
            return ee::s11::f_0034fb00__generated(c);
        }
        fake(c, 0x0034_fb00, c.g[RA][0] as u32);
    }

    #[test]
    fn restore_node_ctor_matches() {
        let _slot = Oc329Slot::install(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc329_fake_world_obj_ctor,
        );
        check(ee::s11::f_00350f60__generated, restore_node_ctor, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x35_0f60, 0);
            c.pc = 0x0035_0f60;
            c.bus.write32(OC329_CTRL, OC329_MAGIC);
            let node = DATA + 0x200;
            let selfp = DATA + 0x100;
            c.bus.write32(node + 8, r.next_u32());
            c.bus.write32(selfp + 0x18, node);
            c.g[A0][0] = sx(selfp);
            c.g[A1][0] = sx(node);
        });
    }

    // -- SND_ChannelStartStreamVoices -------------------------------------

    fn oc329_fake_voice_start(c: &mut Cpu) {
        if c.bus.read32(OC329_CTRL) != OC329_MAGIC {
            return ee::s14::f_003c8158__generated(c);
        }
        fake(c, 0x003c_8158, c.g[RA][0] as u32);
    }

    fn oc329_fake_voice_activate(c: &mut Cpu) {
        if c.bus.read32(OC329_CTRL) != OC329_MAGIC {
            return ee::s14::f_003c83a0__generated(c);
        }
        fake(c, 0x003c_83a0, c.g[RA][0] as u32);
    }

    #[test]
    fn snd_channel_start_stream_voices_matches() {
        let _start = Oc329Slot::install(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003C8158),
            oc329_fake_voice_start,
        );
        let _activate = Oc329Slot::install(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003C83A0),
            oc329_fake_voice_activate,
        );
        check(
            ee::s14::f_003ce1b0__generated,
            snd_channel_start_stream_voices,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_e1b0, 0);
                c.pc = 0x003c_e1b0;
                c.bus.write32(OC329_CTRL, OC329_MAGIC);
                let state = 0x0050_a8e8;
                let entries = DATA + 0x4000;
                let entry = entries;
                let aux = DATA + 0x5000;
                let voices = DATA + 0x5100;
                let t2p = DATA + 0x5200;
                c.bus.write32(state + 0x1f0, entries);
                // Entry flags (+0x24, lhu): bit 3 skips, bit 2 missing skips.
                let flags: u32 = match seed % 3 {
                    0 => 8,
                    1 => 0,
                    _ => 4,
                };
                c.bus.write16(entry + 0x24, flags as u16);
                c.bus
                    .write8(entry + 0x23, if seed % 5 == 0 { 0 } else { 2 });
                // The voice id list (+4, lh each) and the entry's +0x14.
                c.bus.write16(entry + 4, 0);
                c.bus.write16(entry + 6, 0);
                c.bus.write32(entry + 0x14, r.next_u32());
                // The auxiliary struct the loop reads.
                c.bus.write32(aux + 0x18, voices);
                c.bus.write32(voices, r.next_u32());
                c.bus.write32(voices + 4, r.next_u32());
                c.bus.write8(aux + 7, (seed % 4) as u8);
                c.bus.write8(t2p + 3, (seed % 3) as u8);
                c.bus.write8(state + 0x2f, 2);
                c.bus.write8(state + 0x30, 3);
                c.g[A0][0] = sx(DATA + 0x6000);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x1111;
                c.g[A3][0] = 0x2222;
                c.g[T0][0] = 0x3333;
                c.g[T1][0] = 0x4444;
                c.g[T2][0] = sx(t2p);
                c.g[T3][0] = sx(aux);
            },
        );
    }

    // -- SPCH_HistoryPush -------------------------------------------------

    #[test]
    fn spch_history_push_matches() {
        check(ee::s14::f_003d6ee8__generated, spch_history_push, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x3d_6ee8, 0);
            c.pc = 0x003d_6ee8;
            let hist = DATA + 0x8000;
            for i in 0..0x1400u32 {
                c.bus.write32(hist + 4 * i, r.next_u32());
            }
            // flags (+4): length nibble (0..15) and the skip bit; wrap (+5);
            // ring length (+6).
            let flags = (seed % 16) | if seed % 4 == 0 { 0x80 } else { 0 };
            c.bus.write8(hist + 4, flags as u8);
            c.bus.write8(hist + 5, (seed % 37) as u8);
            c.bus.write8(hist + 6, (seed % 9) as u8);
            c.g[A0][0] = sx(hist);
            c.g[A1][0] = (seed % 251) as u64;
        });
    }
}
