//! Staging module oc1587 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1587`.

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
    (0x003e_5a78, debug_set_channel_flag, "Debug_SetChannelFlag"),
    (0x003e_5aa8, debug_set_channel_state, "Debug_SetChannelState"),
    (0x003e_5b60, debug_init_channels, "Debug_InitChannels"),
    (0x003e_5c78, debug_v_printf, "Debug_VPrintf"),
    (0x003d_4898, path_node_value, "func_003D4898"),
    (0x003d_49d0, path_nearest_entry, "func_003D49D0"),
    (0x003d_bae8, loc_string_table_remap_id, "LocStringTable_RemapId"),
    (0x003d_2df8, path_track_prepare, "PATH_TrackPrepare"),
    // @@OVR@@
];

// @@FN@@

/// `PRINT_init()` (`Debug_InitChannels`, 0x3e5b60). Once (the flag word at
/// 0x451008 is zero): marks the eight device entries (12 bytes each, at
/// 0x451250) and the 64 channel entries (8 bytes each, at 0x450e08) disabled,
/// clears the names of devices 4..7 and of channels 14..63, then enables
/// channel 2 and output 2.
pub fn debug_init_channels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[A0][0] = 0x450000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1008)).lw(c);
        let initialized = c.g[V0][0] != 0u64;
        let ra = c.g[RA][0];
        Addr(c.g[SP][0] as u32).sd(c, ra);
        if !initialized {
            c.cycles += 7;
            c.g[V0][0] = 0x1;
            c.g[V1][0] = 0x450000;
            let flag = (c.g[A0][0] as u32).wrapping_add(0x1008);
            let v = c.g[V0][0];
            Addr(flag).sw(c, v);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = 0xfffffffffffffffe;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
            // devices 0..7: enabled = 0
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                let enabled = (c.g[V1][0] as u32).wrapping_add(0x8);
                let v = c.g[V0][0];
                Addr(enabled).sw(c, v);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                let more = (c.g[V1][0] as i64) < (c.g[A1][0] as i64);
                c.g[V0][0] = more as u64;
                if more {
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_5b90);
                    }
                    continue;
                }
                break;
            }
            // channels 0..63: flags &= ~1 (enabled = 0)
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[A1][0] = 0xfffffffffffffffe;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe08));
            c.g[A0][0] = 0x3f;
            loop {
                c.cycles += 7;
                let flags = (c.g[V1][0] as u32).wrapping_add(0x4);
                c.g[V0][0] = Addr(flags).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                let v = c.g[V0][0];
                Addr(flags).sw(c, v);
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_5bc0);
                    }
                    continue;
                }
                break;
            }
            // devices 4..7: name = 0, func = 0
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1250));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
            Addr(c.g[V1][0] as u32).sw(c, 0u64);
            loop {
                c.cycles += 7;
                Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                let more = (c.g[V1][0] as i64) < (c.g[A0][0] as i64);
                c.g[V0][0] = more as u64;
                if more {
                    Addr(c.g[V1][0] as u32).sw(c, 0u64);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_5bf0);
                    }
                    continue;
                }
                break;
            }
            // channels 14..63: name = 0, real = 0
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[A1][0] = 0xfffffffffffffffd;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe08));
            c.g[A0][0] = 0x31;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70));
            loop {
                c.cycles += 7;
                let flags = (c.g[V1][0] as u32).wrapping_add(0x4);
                c.g[V0][0] = Addr(flags).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                Addr(c.g[V1][0] as u32).sw(c, 0u64);
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                let v = c.g[V0][0];
                Addr(flags).sw(c, v);
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_5c20);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 6;
            c.g[A0][0] = 0x2;
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x003e_5ef8, 0x003e_5c48)?;
            c.g[A0][0] = 0x2;
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x003e_5dc8, 0x003e_5c54)?;
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        ret(c);
        Ok(())
    })
}

/// `Debug_VPrintf(int channel, const char *fmt, va_list ap)` (0x3e5c78): if
/// the channel is enabled, formats `fmt` into a 0x2000-byte stack buffer
/// (`vsprintf`) and hands it to every enabled device that has a function
/// (`func(channel, buf)`). Initialises the channel tables first if needed.
pub fn debug_v_printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2040));
        c.g[V0][0] = 0x450000;
        let frame = c.g[SP][0] as u32;
        c.sq(S0, frame.wrapping_add(0x2030));
        c.sq(S1, frame.wrapping_add(0x2020));
        c.g[S0][0] = c.g[A1][0];
        c.sq(S2, frame.wrapping_add(0x2010));
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1008)).lw(c);
        c.g[S2][0] = c.g[A2][0];
        let initialized = c.g[V1][0] != 0u64;
        let ra = c.g[RA][0];
        Addr(frame.wrapping_add(0x2000)).sd(c, ra);
        if !initialized {
            c.cycles += 2;
            call_ptr(c, 0x003e_5b60, 0x003e_5cac)?;
        }
        // the channel's enabled bit
        c.cycles += 8;
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = sx((c.g[S1][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe08));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[V1][0] = c.g[V1][0] & 0x1;
        let enabled = c.g[V1][0] != 0u64;
        c.g[A2][0] = c.g[S2][0];
        if enabled {
            c.cycles += 7;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0041_86c8, 0x003e_5cd8)?;
            // every enabled device with a function gets (channel, buf)
            c.g[V1][0] = 0x450000;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                if c.g[V0][0] == 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
                    if c.g[V0][0] != 0u64 {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[SP][0];
                        let func = c.g[V0][0] as u32;
                        call_ptr(c, func, 0x003e_5d08)?;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_5ce8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 6;
        c.lq(S0, frame.wrapping_add(0x2030));
        c.lq(S1, frame.wrapping_add(0x2020));
        c.lq(S2, frame.wrapping_add(0x2010));
        c.g[RA][0] = Addr(frame.wrapping_add(0x2000)).ld(c);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2040));
        ret(c);
        Ok(())
    })
}

// @@FN@@

/// `PRINT_SetChannelState(int channel, int enable)` (0x3e5aa8). Channel 0
/// sets the enabled bit of all 64 channels, channel 1 only the channels whose
/// `real` bit (bit 1 of the flags word) is set, any other channel just its own
/// entry. The channel table is at 0x450e08 with 8-byte entries; the flags word
/// is at +4 (bit 0 enabled, bit 1 real).
pub fn debug_set_channel_state(c: &mut Cpu) {
    c.cycles += 2;
    let channel_nonzero = c.g[A0][0] != 0u64;
    c.g[A2][0] = c.g[A1][0];
    if channel_nonzero {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let not_one = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = 0x450000;
        if not_one {
            // a single channel
            c.cycles += 10;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe08));
            c.g[A1][0] = 0xfffffffffffffffe;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = (0u64 < c.g[A2][0]) as u64;
            let flags = (c.g[V1][0] as u32).wrapping_add(0x4);
            c.g[V0][0] = Addr(flags).lw(c);
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
            let v = c.g[V0][0];
            Addr(flags).sw(c, v);
            ret(c);
            return;
        }
        // channel 1: every channel that is real
        c.cycles += 4;
        c.g[A0][0] = (0u64 < c.g[A2][0]) as u64;
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe08));
        c.g[A1][0] = 0xfffffffffffffffe;
        c.g[A2][0] = 0x3f;
        loop {
            c.cycles += 5;
            let flags = (c.g[A3][0] as u32).wrapping_add(0x4);
            c.g[V1][0] = Addr(flags).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let not_real = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[V1][0] & c.g[A1][0];
            if !not_real {
                c.cycles += 2;
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                let v = c.g[V0][0];
                Addr(flags).sw(c, v);
            }
            c.cycles += 3;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003e_5b08);
                }
                continue;
            }
            c.cycles += 2;
            ret(c);
            return;
        }
    }
    // channel 0: every channel
    c.cycles += 6;
    c.g[V0][0] = 0x450000;
    c.g[A0][0] = (0u64 < c.g[A2][0]) as u64;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe08));
    c.g[A1][0] = 0xfffffffffffffffe;
    c.g[A2][0] = 0x3f;
    loop {
        c.cycles += 7;
        let flags = (c.g[V1][0] as u32).wrapping_add(0x4);
        c.g[V0][0] = Addr(flags).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        let v = c.g[V0][0];
        Addr(flags).sw(c, v);
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003e_5ac8);
            }
            continue;
        }
        c.cycles += 2;
        ret(c);
        return;
    }
}

/// `PRINT_SetDeviceState(int device, int enable)` (0x3e5a78): stores bit 0 of
/// `enable` into `PRINTdevicelist[device].enabled`. The device table is at
/// 0x451250 with 12-byte entries; the enabled word is at +8 of each entry.
pub fn debug_set_channel_flag(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0xc;
    c.g[V1][0] = 0x450000;
    c.mult(0, A0, A0, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
    c.g[A2][0] = 0xfffffffffffffffe;
    c.g[A1][0] = c.g[A1][0] & 0x1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let entry = (c.g[A0][0] as u32).wrapping_add(0x8);
    c.g[V0][0] = Addr(entry).lw(c);
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    let v = c.g[V0][0];
    Addr(entry).sw(c, v);
    ret(c);
}

/// `int func_003D4898(int node)` (0x3d4898): the value of node `node` of the
/// player path `D_0044F420[0]`. Zero for a negative or out-of-range node
/// (the path's node count is the u16 at +0x12 of its data); otherwise the
/// node's entry `v` (a short, through the node's index table at +0x38) is
/// positive, and the entry `v - 1` of the path's array (at +0x50, 8-byte
/// entries) is returned, else zero.
pub fn path_node_value(c: &mut Cpu) {
    c.cycles += 2;
    let negative = (c.g[A0][0] as i64) < 0;
    c.g[A3][0] = 0u64;
    if !negative {
        c.cycles += 7;
        c.g[V0][0] = 0x450000;
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0xbe0)).lw(c);
        c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x34)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x12)).lhu(c);
        let in_range = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
        c.g[V0][0] = in_range as u64;
        if in_range {
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
            c.cycles += 8;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lh(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lh(c);
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if positive {
                c.cycles += 4;
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x50)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A3][0];
            ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// Nearest-entry search (0x3d49d0), `(int n, int target, const E *tab)` in
/// `a0, a1, a2`: over the `n` four-byte entries `{ s8 a; s8 b; s16 v; }` it
/// finds the entry whose nearer of `a` and `b` lies closest to `target` (the
/// first one at the smallest distance below 0x7f) and returns its `v`; the
/// first entry if none is closer than 0x7f. -1 when `n <= 0`.
pub fn path_nearest_entry(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T3][0] = c.g[A1][0];
    let positive = (c.g[A0][0] as i64) > 0;
    c.g[T2][0] = c.g[A2][0];
    if positive {
        c.cycles += 2;
        c.g[T5][0] = 0x7f;
        c.cycles += 3;
        c.g[T1][0] = c.g[A0][0];
        c.g[T4][0] = 0xffffffffffffffff;
        c.g[T0][0] = c.g[T2][0];
        loop {
            c.cycles += 16;
            c.g[A1][0] = Addr(c.g[T0][0] as u32).lb(c);
            c.g[A0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x1)).lb(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T3][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T3][0] as u32));
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.g[A3][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
            c.g[A2][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.g[A3][0];
            }
            if c.g[V1][0] == 0 {
                c.g[A0][0] = c.g[A2][0];
            }
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.g[A0][0];
            }
            c.g[V0][0] = c.g[A1][0];
            c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
            let closer = c.g[V1][0] != 0u64;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
            if closer {
                c.cycles += 2;
                c.g[T5][0] = c.g[V0][0];
                c.g[T2][0] = c.g[T0][0];
            }
            c.cycles += 2;
            let more = c.g[T1][0] != 0u64;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003d_49f8);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x2)).lhu(c);
        ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0xffffffffffffffff;
        ret(c);
    }
}

/// `LocStringTable_RemapId(short key, const u8 *index)` (0x3dbae8): binary
/// search (`bsearch` with `compare_BAE8`, 4-byte entries) of the key in the
/// table at `index + 0x10`, whose entry count is the byte word at `index + 8`.
/// Returns the 16-bit value stored after the found key, or -1 if it is absent.
pub fn loc_string_table_remap_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        let sp = c.g[SP][0] as u32;
        let key = c.g[A0][0];
        Addr(sp).sh(c, key);
        c.g[T0][0] = 0x3e0000;
        let ra = c.g[RA][0];
        Addr(sp.wrapping_add(0x10)).sd(c, ra);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4538));
        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x3)).lbu(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2)).lbu(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1)).lbu(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
        c.g[A3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lbu(c);
        c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 8);
        c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = c.g[A2][0] | c.g[A3][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = 0x4;
        call_ptr(c, 0x0041_87e8, 0x003d_bb38)?;
        c.g[V1][0] = c.g[V0][0];
        let not_found = c.g[V1][0] == 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
        if not_found {
            c.cycles += 1;
            c.g[V0][0] = 0xffffffffffffffff;
        } else {
            c.cycles += 5;
            let value = c.g[V1][0] as u32;
            c.g[V0][0] = Addr(value.wrapping_add(0x1)).lbu(c);
            c.g[V1][0] = Addr(value).lbu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        }
        c.cycles += 3;
        let frame = c.g[SP][0] as u32;
        c.g[RA][0] = Addr(frame.wrapping_add(0x10)).ld(c);
        c.g[SP][0] = sx(frame.wrapping_add(0x20));
        ret(c);
        Ok(())
    })
}

/// What `PATH_TrackPrepare` does after choosing the request: call the
/// `func_002AE298` finisher, or return 0 at once (the selected request is
/// the one already in use).
enum TrackSelect {
    Finish,
    ReturnZero,
}

/// The request-table part of `PATH_TrackPrepare` (0x3d2df8): `reqTable =
/// PATH_FindTrackInfo(pathId)`, the entry `reqTable + 1 + idx`, and, when the
/// table asks for it and the voice's counters are behind, the type-1 search
/// for the request with the smallest `f0` (or the type-0 unload). Leaves A0 =
/// the voice's `f54` for the finisher, as the generated code does.
fn track_prepare_select(c: &mut Cpu) -> Result<TrackSelect, crate::call::Unwound> {
    c.cycles += 8;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003d_4a50, 0x003d_2e78)?;
    c.g[T0][0] = c.g[V0][0];
    c.g[V0][0] = 0xc;
    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
    c.mult(0, V0, S2, V0);
    let no_table = c.g[T0][0] == 0u64;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if no_table {
        c.cycles += 1;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x54)).lw(c);
        return Ok(TrackSelect::Finish);
    }
    c.cycles += 4;
    c.g[A1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x2)).lhu(c);
    c.g[V0][0] = 0x2;
    if c.g[A1][0] == c.g[V0][0] {
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x54)).lw(c);
        return Ok(TrackSelect::Finish);
    }
    // the counters: reqTable->f4 < voice->f104 + entry->f4, or the f8 pair
    c.cycles += 7;
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x104)).lw(c);
    c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
    c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
    let behind = c.g[V1][0] != 0u64;
    c.g[V0][0] = 0x1;
    if !behind {
        c.cycles += 7;
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x108)).lw(c);
        c.g[A0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
        if c.g[A0][0] == 0u64 {
            c.cycles += 1;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x54)).lw(c);
            return Ok(TrackSelect::Finish);
        }
    }
    // 'b_3d2f40 (the counters are behind): type 1 searches, type 0 unloads
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    c.g[A3][0] = 0x7fff_0000;
    if c.g[A1][0] != c.g[V0][0] {
        c.cycles += 2;
        if c.g[A1][0] != 0u64 {
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x54)).lw(c);
            return Ok(TrackSelect::Finish);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002a_e3a8, 0x003d_2f50)?;
        c.cycles += 1;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x54)).lw(c);
        return Ok(TrackSelect::Finish);
    }
    // type 1: the entry with the smallest f0 (the last one of equal f0's)
    c.cycles += 6;
    c.g[V0][0] = Addr(c.g[T0][0] as u32).lhu(c);
    c.g[A3][0] = c.g[A3][0] | 0xffff;
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
    let empty = c.g[V0][0] == 0u64;
    c.g[A0][0] = 0u64;
    if !empty {
        c.cycles += 2;
        c.g[T0][0] = c.g[V0][0];
        loop {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[V0][0] = (c.g[A3][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 3;
                c.g[A3][0] = c.g[V1][0];
                c.g[A1][0] = c.g[A0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[T0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003d_2f00);
                }
                continue;
            }
            break;
        }
    }
    c.cycles += 2;
    if c.g[A1][0] == c.g[S2][0] {
        c.g[V0][0] = 0;
        return Ok(TrackSelect::ReturnZero);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003d_3140, 0x003d_2f38)?;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x54)).lw(c);
    Ok(TrackSelect::Finish)
}

/// The found-track part of `PATH_TrackPrepare` (after `PATH_SelectMap` said
/// yes): finds the voice, selects the request, calls the finisher, and clears
/// the two init words. Returns the function's `v0`.
fn track_prepare_found(c: &mut Cpu) -> Result<u64, crate::call::Unwound> {
    c.cycles += 5;
    c.g[RA][0] = sx(0x003d_2e64);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003d_5290, 0x003d_2e64)?;
    c.g[S0][0] = c.g[V0][0];
    let found = c.g[S0][0] != 0u64;
    c.g[V0][0] = 0x450000;
    if found {
        match track_prepare_select(c)? {
            TrackSelect::ReturnZero => return Ok(0),
            TrackSelect::Finish => {
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S4][0];
                call_ptr(c, 0x002a_e298, 0x003d_2f64)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x450000;
            }
        }
    }
    c.cycles += 3;
    Addr((c.g[S5][0] as u32).wrapping_sub(0xbd0)).sw(c, 0u64);
    Addr((c.g[V0][0] as u32).wrapping_sub(0xbe0)).sw(c, 0u64);
    c.g[V0][0] = c.g[S3][0];
    Ok(c.g[V0][0])
}

/// `PATH_TrackPrepare(int pathId, int idx, void *userHandle)` (0x3d2df8):
/// when the path system is set up (`0x44f42c` set, `0x44f430` clear) and
/// `PATH_SelectMap(-1)` says so, prepares the track for `pathId`: finds it,
/// selects the request `idx` (possibly unloading another bank or switching
/// to the nearest request), and calls `func_002AE298` for the voice. Returns
/// that result, 0, or -18 when the system is not ready.
pub fn path_track_prepare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.g[V0][0] = 0x450000;
        let frame = c.g[SP][0] as u32;
        c.sq(S1, frame.wrapping_add(0x50));
        c.sq(S2, frame.wrapping_add(0x40));
        c.g[S1][0] = c.g[A0][0];
        c.sq(S3, frame.wrapping_add(0x30));
        c.g[S2][0] = c.g[A1][0];
        c.sq(S4, frame.wrapping_add(0x20));
        c.g[S3][0] = 0u64;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0xbd4)).lbu(c);
        c.g[S4][0] = c.g[A2][0];
        c.sq(S0, frame.wrapping_add(0x60));
        c.sq(S5, frame.wrapping_add(0x10));
        let not_set_up = c.g[V1][0] == 0u64;
        let ra = c.g[RA][0];
        Addr(frame).sd(c, ra);
        let v0: u64 = if not_set_up {
            c.cycles += 2;
            0xffff_ffff_ffff_ffee
        } else {
            c.cycles += 4;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0xbd0)).lw(c);
            let busy = c.g[V0][0] != 0u64;
            c.g[S5][0] = c.g[V1][0];
            if busy {
                c.cycles += 2;
                0xffff_ffff_ffff_ffee
            } else {
                c.cycles += 4;
                c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                call_ptr(c, 0x003d_5450, 0x003d_2e4c)?;
                if c.g[V0][0] != 0u64 {
                    track_prepare_found(c)?
                } else {
                    c.cycles += 2;
                    0xffff_ffff_ffff_ffee
                }
            }
        };
        c.g[V0][0] = v0;
        c.cycles += 9;
        c.lq(S0, frame.wrapping_add(0x60));
        c.lq(S1, frame.wrapping_add(0x50));
        c.lq(S2, frame.wrapping_add(0x40));
        c.lq(S3, frame.wrapping_add(0x30));
        c.lq(S4, frame.wrapping_add(0x20));
        c.lq(S5, frame.wrapping_add(0x10));
        c.g[RA][0] = Addr(frame).ld(c);
        c.g[SP][0] = sx(frame.wrapping_add(0x70));
        ret(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA};

    #[test]
    fn debug_set_channel_flag_matches() {
        check(ee::s14::f_003e5a78__generated, debug_set_channel_flag, 300, &|c, seed| {
            let mut r = Rng(0x003e_5a78 ^ seed as u64);
            scramble(c, &mut r);
            let device = seed % 8;
            c.g[A0][0] = sx(device);
            c.g[A1][0] = sx(r.next_u32());
            c.bus.write32(0x0045_1250 + 12 * device + 8, r.next_u32());
        });
    }

    #[test]
    fn debug_set_channel_state_matches() {
        check(ee::s14::f_003e5aa8__generated, debug_set_channel_state, 300, &|c, seed| {
            let mut r = Rng(0x003e_5aa8 ^ seed as u64);
            scramble(c, &mut r);
            // channel 0 (all), 1 (the real ones), or one of the others
            let channel = match seed % 4 {
                0 => 0,
                1 => 1,
                _ => 2 + r.next_u32() % 62,
            };
            c.g[A0][0] = sx(channel);
            c.g[A1][0] = sx(if seed % 5 == 0 { 0 } else { r.next_u32() });
            for i in 0..64u32 {
                c.bus.write32(0x0045_0e08 + 8 * i + 4, r.next_u32());
            }
        });
    }

    #[test]
    fn debug_init_channels_matches() {
        check(ee::s14::f_003e5b60__generated, debug_init_channels, 300, &|c, seed| {
            let mut r = Rng(0x003e_5b60 ^ seed as u64);
            scramble(c, &mut r);
            // the device and channel tables hold junk
            fill(c, &mut r, 0x0045_0e00, 0x180);
            // the initialised flag: zero on most seeds
            let flag = if seed % 3 == 0 { r.next_u32() | 1 } else { 0 };
            c.bus.write32(0x0045_1008, flag);
        });
    }

    #[test]
    fn debug_v_printf_matches() {
        check(ee::s14::f_003e5c78__generated, debug_v_printf, 300, &|c, seed| {
            let mut r = Rng(0x003e_5c78 ^ seed as u64);
            scramble(c, &mut r);
            // the libc formatter and the game's tables
            elf_image(c);
            fill(c, &mut r, 0x0045_0e00, 0x180);
            // the initialised flag: zero on some seeds (the init runs)
            let flag = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(0x0045_1008, flag);
            // the channel: enabled on two seeds out of three
            let channel = r.next_u32() % 64;
            let enabled = (seed % 3 != 0) as u32;
            c.bus.write32(0x0045_0e08 + 8 * channel + 4, (r.next_u32() & !1) | enabled);
            // devices: some enabled, some with a function (a leaf that returns)
            for i in 0..8u32 {
                let entry = 0x0045_1250 + 12 * i;
                let on = r.next_u32() & 1;
                c.bus.write32(entry + 8, (r.next_u32() & !1) | on);
                let func = if (seed + i) % 2 == 0 { 0 } else { 0x0014_06c0 };
                c.bus.write32(entry + 4, func);
            }
            // the format "dbg" (no conversions) and an empty va_list
            c.bus.write32(DATA + 0x3000, 0x0067_6264);
            c.g[A0][0] = sx(channel);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = sx(DATA + 0x3100);
        });
    }

    #[test]
    fn path_node_value_matches() {
        check(ee::s14::f_003d4898__generated, path_node_value, 300, &|c, seed| {
            let mut r = Rng(0x003d_4898 ^ seed as u64);
            scramble(c, &mut r);
            let player = DATA + 0x4000;
            let pd = DATA + 0x5000;
            let idx = DATA + 0x6000;
            let arr = DATA + 0x7000;
            c.bus.write32(0x0044_f420, player);
            c.bus.write32(player + 0x34, pd);
            c.bus.write32(player + 0x38, idx);
            c.bus.write32(player + 0x50, arr);
            c.bus.write16(pd + 0x12, (r.next_u32() % 6) as u16);
            for k in 0..8u32 {
                c.bus.write16(idx + 2 * k, (r.next_u32() % 8) as u16);
                // the entry's v: -1 .. 5
                let v = (r.next_u32() % 7) as i32 - 1;
                c.bus.write16(pd + 4 * k, v as u16);
            }
            fill(c, &mut r, arr, 8);
            // node: negative on some seeds, else in or past the node count
            let node = if seed % 7 == 0 { u32::MAX } else { r.next_u32() % 8 };
            c.g[A0][0] = sx(node);
        });
    }

    #[test]
    fn path_nearest_entry_matches() {
        check(ee::s14::f_003d49d0__generated, path_nearest_entry, 300, &|c, seed| {
            let mut r = Rng(0x003d_49d0 ^ seed as u64);
            scramble(c, &mut r);
            // n: zero and negative on some seeds
            let n: i32 = match seed % 7 {
                0 => 0,
                1 => -1,
                _ => 1 + (r.next_u32() % 6) as i32,
            };
            let tab = DATA + 0x3000;
            for i in 0..6u32 {
                let a = (r.next_u32() % 41) as i32 - 20;
                let b = (r.next_u32() % 41) as i32 - 20;
                let v = r.next_u32() & 0xffff;
                let word = (a as u8 as u32) | ((b as u8 as u32) << 8) | (v << 16);
                c.bus.write32(tab + 4 * i, word);
            }
            let target = (r.next_u32() % 41) as i32 - 20;
            c.g[A0][0] = sx(n as u32);
            c.g[A1][0] = sx(target as u32);
            c.g[A2][0] = sx(tab);
        });
    }

    #[test]
    fn loc_string_table_remap_id_matches() {
        check(ee::s14::f_003dbae8__generated, loc_string_table_remap_id, 300, &|c, seed| {
            let mut r = Rng(0x003d_bae8 ^ seed as u64);
            scramble(c, &mut r);
            let index = DATA + 0x3000;
            // entry count (0..6) in the byte word at +8, sorted 4-byte entries
            // at +0x10: key in the low half, value in the high half
            let count = r.next_u32() % 7;
            c.bus.write32(index + 8, count);
            for i in 0..count {
                let value = r.next_u32() & 0xffff;
                c.bus.write32(index + 0x10 + 4 * i, 10 * i | (value << 16));
            }
            // a key that is present on most seeds
            let key = if seed % 5 == 0 {
                r.next_u32() & 0xffff
            } else {
                10 * (r.next_u32() % 7)
            };
            c.g[A0][0] = sx(key);
            c.g[A1][0] = sx(index);
        });
    }

    use std::ptr::addr_of_mut;

    use crate::difftest::fakes;

    // Gated stand-ins for the callees of PATH_TrackPrepare: each returns its
    // mailbox word in v0 and goes back to ra. Off the gate they run the body
    // they replaced.
    const OC1587_GATE: u32 = DATA + 0x3800;
    const OC1587_MAGIC: u32 = 0x1587_0a5a;
    const OC1587_MBOX: u32 = DATA + 0x3810;

    fn oc1587_stub(c: &mut Cpu, k: u32) {
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = c.bus.read32(OC1587_MBOX + 4 * k) as u64;
        c.pc = ra;
    }

    fn oc1587_fake_select(c: &mut Cpu) {
        if c.bus.read32(OC1587_GATE) != OC1587_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1587_fake_select),
                ee::s14::f_003d5450__generated,
            );
        }
        oc1587_stub(c, 0);
    }

    fn oc1587_fake_find(c: &mut Cpu) {
        if c.bus.read32(OC1587_GATE) != OC1587_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1587_fake_find),
                ee::s14::f_003d5290__generated,
            );
        }
        oc1587_stub(c, 1);
    }

    fn oc1587_fake_info(c: &mut Cpu) {
        if c.bus.read32(OC1587_GATE) != OC1587_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1587_fake_info),
                ee::s14::f_003d4a50__generated,
            );
        }
        oc1587_stub(c, 2);
    }

    fn oc1587_fake_finish(c: &mut Cpu) {
        if c.bus.read32(OC1587_GATE) != OC1587_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1587_fake_finish),
                ee::s08::f_002ae298__generated,
            );
        }
        oc1587_stub(c, 3);
    }

    fn oc1587_fake_unlock(c: &mut Cpu) {
        if c.bus.read32(OC1587_GATE) != OC1587_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1587_fake_unlock),
                ee::s08::f_002ae3a8__generated,
            );
        }
        oc1587_stub(c, 4);
    }

    fn oc1587_fake_unload(c: &mut Cpu) {
        if c.bus.read32(OC1587_GATE) != OC1587_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1587_fake_unload),
                ee::s14::f_003d3140__generated,
            );
        }
        oc1587_stub(c, 5);
    }

    fn oc1587_install() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D5450),
            oc1587_fake_select,
            fakes::prev_cell(oc1587_fake_select),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D5290),
            oc1587_fake_find,
            fakes::prev_cell(oc1587_fake_find),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D4A50),
            oc1587_fake_info,
            fakes::prev_cell(oc1587_fake_info),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AE298),
            oc1587_fake_finish,
            fakes::prev_cell(oc1587_fake_finish),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AE3A8),
            oc1587_fake_unlock,
            fakes::prev_cell(oc1587_fake_unlock),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D3140),
            oc1587_fake_unload,
            fakes::prev_cell(oc1587_fake_unload),
        );
    }

    /// `PATH_TrackPrepare`: the set-up and busy states, a voice whose counters
    /// are and are not behind, request tables of every type (including empty
    /// ones), the type-1 search with and without a closer request, and the
    /// found and not-found tracks.
    #[test]
    fn path_track_prepare_matches() {
        oc1587_install();
        check(ee::s14::f_003d2df8__generated, path_track_prepare, 300, &|c, seed| {
            let mut r = Rng(0x003d_2df8 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write32(OC1587_GATE, OC1587_MAGIC);
            c.bus.write8(0x0044_f42c, (seed % 6 != 0) as u8);
            c.bus.write32(0x0044_f430, if seed % 5 == 0 { 1 } else { 0 });
            // the mailbox: select, find, info, finish, unlock, unload
            let pv = DATA + 0x9000;
            let rt = DATA + 0x9400;
            c.bus.write32(OC1587_MBOX, (seed % 4 != 0) as u32);
            c.bus.write32(OC1587_MBOX + 4, if seed % 7 == 3 { 0 } else { pv });
            c.bus.write32(OC1587_MBOX + 8, if seed % 9 == 4 { 0 } else { rt });
            c.bus.write32(OC1587_MBOX + 12, r.next_u32());
            c.bus.write32(OC1587_MBOX + 16, 0);
            c.bus.write32(OC1587_MBOX + 20, 0);
            // the voice: f54 and f5c are arbitrary, the counters small
            c.bus.write32(pv + 0x54, r.next_u32());
            c.bus.write32(pv + 0x5c, r.next_u32());
            c.bus.write32(pv + 0x104, r.next_u32() % 8);
            c.bus.write32(pv + 0x108, r.next_u32() % 8);
            // the request table: count, type, two counters, four entries
            c.bus.write16(rt, (r.next_u32() % 4) as u16);
            c.bus.write16(rt + 2, (r.next_u32() % 3) as u16);
            c.bus.write32(rt + 4, r.next_u32() % 8);
            c.bus.write32(rt + 8, r.next_u32() % 8);
            for i in 0..4u32 {
                let e = rt + 0xc + 12 * i;
                c.bus.write32(e, (r.next_u32() % 40).wrapping_sub(10));
                c.bus.write32(e + 4, r.next_u32() % 8);
                c.bus.write32(e + 8, r.next_u32() % 8);
            }
            c.g[A0][0] = sx(r.next_u32() % 5);
            c.g[A1][0] = sx(r.next_u32() % 3);
            c.g[A2][0] = sx(r.next_u32());
        });
    }

    // @@NEXT@@
}
