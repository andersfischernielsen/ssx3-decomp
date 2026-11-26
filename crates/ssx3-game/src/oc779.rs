//! Staging module oc779 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc779`.

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
    (0x002a_a490, stream_array_ctor, "StreamArray_ctor"),
    (0x002a_69a0, crowd_audio_ctor, "CrowdAudio_ctor"),
    (0x0028_fd00, midi_banks_base_ctor, "MIDIBanks_BaseCtor"),
    (0x0039_c778, uiscreen_lookup_value, "UIScreen_LookupValue"),
];

// ---------------------------------------------------------------------------
// Irradiance_Load (0x0038acd0): already hand-written in `game/render/irradiance.rs`
// (deliberately not installed there: FILE_load waits on the IOP queue). Skipped.
// PATH_StopTracks (0x003d2a68): a `loop { match pc` block dispatcher (resumable
// at its back-edge heads) with no `__entries` twin. Skipped for the orchestrator's
// `port_dispatcher.py`.

/// `cStreamArray* StreamArray_ctor(cStreamArray* this, int inchrg, int banks,
/// int count, heap)` (0x2aa490, frame 0xc0): with `inchrg` set, builds the
/// sound manager reference and bank manager inside `this` (sub-object
/// pointers at +0x0 and +0x1ec) and runs their ctors; rewrites the sound
/// system and bank manager vtables (0x483998, 0x4839b8); when not in charge,
/// copies both vtables to the stack rebased to `this`; stores `count` at +4,
/// allocates the `count`-entry stream pointer array at +8 (named
/// `StreamArray`) and clears it. Returns `this`.
pub fn stream_array_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2aa4f4: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S1][0] = c.g[T0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S3][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            let t = c.g[S3][0] == 0u64;
            c.g[S4][0] = c.g[A3][0];
            if t {
                break 'b_2aa4f4;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.g[V1][0];
                c.g[A1][0] = c.g[S1][0];
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x002aa4e0);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1ec), c.g[V1][0]);
                call_ptr(c, 0x002ade88, 0x002aa4e0)?;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x002aa4f4);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0028b9b8, 0x002aa4f4)?;
                break 'b_2aa4f4;
            }
        }
        'b_2aa5cc: {
            c.cycles += 14;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3998));
            c.g[A0][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.g[A3][0] = 0x480000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x39b8));
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[S3][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2aa5cc;
            } else {
                c.cycles += 40;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x3998));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x39b8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[T1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                break 'b_2aa5cc;
            }
        }
        'b_2aa620: {
            c.cycles += 11;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = sx((c.g[S4][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3218));
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x002aa5e8);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x002aa5e8)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0u64;
            let t = (c.g[V1][0] as i64) <= 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            if t {
                break 'b_2aa620;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                'l_2aa600: loop {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x002aa600);
                        }
                        continue 'l_2aa600;
                    } else {
                        break 'b_2aa620;
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `cCrowdAudio* CrowdAudio_ctor(cCrowdAudio* this, int inchrg, int banks,
/// int count, heap)` (0x2a69a0, frame 0xb0): with `inchrg` set, builds the
/// sound manager reference and bank manager inside `this` (+0x118, +0x2f8),
/// runs the MIDI bank base ctor, rewrites the sub-object vtables (0x483818,
/// 0x483838) and, when not in charge, copies them to the stack rebased to
/// `this`; installs the crowd vtable (0x4837f8), clears the instance count
/// and array (+0x14, +0x18), runs `CrowdAudio_InitSampleList` over the three
/// sample lists of each of the three crowd entries (stride 0x54 from +0x1c)
/// and publishes `this` in the global at `gp+0x608`. Returns `this`.
pub fn crowd_audio_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a69fc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[T0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S2][0] = c.g[A2][0];
            c.g[S3][0] = c.g[A1][0];
            let t = c.g[S3][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            if t {
                break 'b_2a69fc;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x120));
                c.g[A0][0] = c.g[V1][0];
                c.g[A1][0] = c.g[S0][0];
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x002a69e8);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x2f8), c.g[V1][0]);
                call_ptr(c, 0x002ade88, 0x002a69e8)?;
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x002a69fc);
                c.g[A3][0] = c.g[S0][0];
                call_ptr(c, 0x0028b9b8, 0x002a69fc)?;
                break 'b_2a69fc;
            }
        }
        'b_2a6ae8: {
            c.cycles += 19;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x002a6a10);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002a8140, 0x002a6a10)?;
            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3818));
            c.g[A0][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.g[A3][0] = 0x480000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3838));
            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            let t = c.g[S3][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2a6ae8;
            } else {
                c.cycles += 40;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x3818));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3838));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[T1][0] = c.lw(c.g[S1][0] as u32);
                c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x118));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x120));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                break 'b_2a6ae8;
            }
        }
        c.cycles += 8;
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x37f8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[S2][0] = 0x2;
        c.g[S3][0] = 0xffffffffffffffff;
        'l_2a6b08: loop {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002a6b14);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            call_ptr(c, 0x002a4ba8, 0x002a6b14)?;
            c.g[RA][0] = sx(0x002a6b1c);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x002a4ba8, 0x002a6b1c)?;
            c.g[RA][0] = sx(0x002a6b24);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            call_ptr(c, 0x002a4ba8, 0x002a6b24)?;
            let t = c.g[S2][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x54));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002a6b08);
                }
                continue 'l_2a6b08;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.g[S1][0];
                c.sw((c.g[GP][0] as u32).wrapping_add(0x608), c.g[S1][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `cBankSys* MIDIBanks_BaseCtor(cBankSys* this, int inchrg, int banks,
/// int count, heap)` (0x28fd00, frame 0xb0): with `inchrg` set, builds the
/// sound manager reference and bank manager inside `this` (+0x10, +0x18) and
/// runs their ctors; rewrites the sound system and bank manager vtables
/// (0x483a60, 0x483a80); when not in charge, copies them to the stack
/// rebased to `this`; installs the bank system vtable (0x483a48), publishes
/// `this` at `gp+0x574` and allocates the 0x10-byte bank and patch tables
/// (+0x4, +0x8). Returns `this`.
pub fn midi_banks_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_28fd5c: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A3][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S2][0] = c.g[A2][0];
            c.g[S3][0] = c.g[A1][0];
            let t = c.g[S3][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            if t {
                break 'b_28fd5c;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = c.g[V1][0];
                c.g[A1][0] = c.g[S1][0];
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x0028fd48);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1f0), c.g[V1][0]);
                call_ptr(c, 0x002ade88, 0x0028fd48)?;
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0028fd5c);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0028b9b8, 0x0028fd5c)?;
                break 'b_28fd5c;
            }
        }
        'b_28fe34: {
            c.cycles += 14;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3a60));
            c.g[A0][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.g[A3][0] = 0x480000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3a80));
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[S3][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_28fe34;
            } else {
                c.cycles += 40;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x3a60));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3a80));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[T1][0] = c.lw(c.g[S0][0] as u32);
                c.g[A3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x18));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                break 'b_28fe34;
            }
        }
        c.cycles += 26;
        c.g[V0][0] = 0x480000;
        c.g[A1][0] = 0x4a0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a48));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3668));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x0028fe5c);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x574), c.g[S0][0]);
        call_ptr(c, 0x00317e30, 0x0028fe5c)?;
        c.g[A1][0] = 0x4a0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3670));
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x0028fe78);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x0028fe78)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `u16 UIScreen_LookupValue(UIScreen* screen, u32 key)` (0x39c778): walks
/// the screen's entry table (pointer at +0x34; first word is the entry count,
/// the entries follow, each `{key, size, value, ..}` with `size` the byte
/// stride) and returns the 16-bit value at +8 of the first entry whose key
/// matches, or 0xffff when none does.
pub fn uiscreen_lookup_value(c: &mut Cpu) {
    c.cycles += 5;
    let table = Addr(c.g[A0][0] as u32).off(0x34).lw(c);
    c.g[V0][0] = table;
    let count = Addr(table as u32).lw(c);
    c.g[V1][0] = count;
    c.g[A2][0] = 0;
    c.g[A0][0] = sx((table as u32).wrapping_add(0x4));
    if count != 0 {
        c.cycles += 1;
        c.g[A3][0] = c.g[V1][0];
        loop {
            c.cycles += 3;
            let key = Addr(c.g[A0][0] as u32).lw(c);
            c.g[V0][0] = key;
            if key != c.g[A1][0] {
                let size = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
                c.g[V1][0] = size;
                c.cycles += 4;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[A2][0] < c.g[A3][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(size as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_c790);
                    }
                    continue;
                }
                break;
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lhu(c);
                c.pc = t;
                return;
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0xffff;
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes and the fixture object, clear of the
    /// fakes' call log (`DATA + 0x10_0000`) and pointer range (`DATA ..
    /// DATA + 0x4_0000`).
    const OC779_CTRL: u32 = DATA + 0x2d_0000;
    const OC779_MAGIC: u32 = 0x4f43_3737 + 9;
    const OC779_OBJ: u32 = DATA + 0x2f_0000;

    /// Gated fakes for the callees that have an override slot.
    macro_rules! oc779_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC779_CTRL) != OC779_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc779_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc779_fakes!(
        (
            oc779_fake_002ade88,
            OC779_PREV_002ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc779_fake_00317e30,
            OC779_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc779_fake_002a4ba8,
            OC779_PREV_002A4BA8,
            ee::s08::OVERRIDE_002A4BA8,
            ee::s08::f_002a4ba8__generated,
            0x002a_4ba8
        ),
    );

    /// Fixture shared by the audio-system constructors: scrambled machine with
    /// fakes on, the gate word, the BELibrary dummy and world words, `this` in
    /// a0, the in-charge flag (a1), a random bank argument and heap, and, for
    /// the not-in-charge case, a bank-manager object at `this + 0x100` whose
    /// +0x1d8 field points at a scratch sub-object (the vtable fix-ups read
    /// and write through them).
    fn oc779_audio_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC779_CTRL, OC779_MAGIC);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), OC779_OBJ + 0x8000);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x848), OC779_OBJ + 0x9000);
        let bank_mgr = OC779_OBJ + 0x1000;
        c.bus.write32(OC779_OBJ, bank_mgr);
        c.bus.write32(bank_mgr + 0x1d8, OC779_OBJ + 0x2000);
        c.g[A0][0] = sx(OC779_OBJ);
        c.g[A1][0] = (seed % 2) as u64;
        c.g[A2][0] = sx(r.next_u32() & 0xff);
        c.g[A3][0] = sx(r.next_u32() & 0xff);
        c.g[T0][0] = sx(DATA + 0x20_0000 + (r.next_u32() & 0xf0));
        r
    }

    /// `StreamArray_ctor` for in-charge 0/1 and stream counts 0, 1, 3, 4 (the
    /// count is the `a3` argument), unwinding at random fake calls.
    #[test]
    fn stream_array_ctor_matches() {
        oc779_install_fakes();
        check(
            ee::s08::f_002aa490__generated,
            stream_array_ctor,
            64,
            &|c, seed| {
                oc779_audio_ctor_fixture(c, seed, 0x779a, 4);
                c.g[A3][0] = [0u64, 1, 3, 4][(seed / 2 % 4) as usize];
            },
        );
    }

    /// `CrowdAudio_ctor` for in-charge 0/1: the MIDI bank base ctor runs for
    /// real (no override slot); the sound manager, allocator and sample-list
    /// init are gated fakes (some unwind).
    #[test]
    fn crowd_audio_ctor_matches() {
        oc779_install_fakes();
        check(
            ee::s08::f_002a69a0__generated,
            crowd_audio_ctor,
            48,
            &|c, seed| {
                oc779_audio_ctor_fixture(c, seed, 0x779b, 12);
            },
        );
    }

    /// `MIDIBanks_BaseCtor` for in-charge 0/1: sound manager and the two
    /// table allocations are gated fakes (some unwind).
    #[test]
    fn midi_banks_base_ctor_matches() {
        oc779_install_fakes();
        check(
            ee::s07::f_0028fd00__generated,
            midi_banks_base_ctor,
            48,
            &|c, seed| {
                oc779_audio_ctor_fixture(c, seed, 0x779c, 4);
            },
        );
    }

    /// `UIScreen_LookupValue`: a table of 0..5 entries `{key, stride, value}`
    /// with varying strides; the key is found at some entry or missing.
    #[test]
    fn uiscreen_lookup_value_matches() {
        check(
            ee::s13::f_0039c778__generated,
            uiscreen_lookup_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x779d_0000 ^ seed as u64);
                scramble(c, &mut r);
                let screen = DATA + 0x100;
                let table = DATA + 0x400;
                let count = seed % 6;
                c.bus.write32(screen + 0x34, table);
                c.bus.write32(table, count);
                let mut p = table + 4;
                for i in 0..count {
                    let stride = 0xc + 4 * (r.next_u32() % 4);
                    c.bus.write32(p, 0x100 + i);
                    c.bus.write32(p + 4, stride);
                    c.bus.write32(p + 8, r.next_u32());
                    p += stride;
                }
                c.g[A0][0] = sx(screen);
                // Keys 0x100.. are present; 0x100 + count and a random one miss.
                c.g[A1][0] = match seed / 6 % 3 {
                    0 => sx(0x100 + (r.next_u32() % (count + 1))),
                    1 => sx(r.next_u32()),
                    _ => sx(0x100 + count.saturating_sub(1)),
                };
            },
        );
    }
}
