//! Staging module oc805 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc805`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026ae70, path_data_reload_paths, "PathData_ReloadPaths"),
    (0x00284c68, ssx_audio_system_ctor, "SSXAudioSystem_ctor"),
    (
        0x002862a8,
        ssx_audio_system_load_world_audio,
        "SSXAudioSystem_LoadWorldAudio",
    ),
    (
        0x00287700,
        ssx_audio_system_set_channel_volume,
        "SSXAudioSystem_SetChannelVolume",
    ),
];

/// `void PathData_ReloadPaths(PathData* self, char* file)`: frees the old path buffer (or loads the file through `PathData_LoadFile` when there was none), then loads the decompressed file, remembers the buffer at +0x48 and re-reads the int header, the AI paths and the track paths.
/// Hand port of the generated body (PathData_ReloadPaths, 0x0026ae70), every register, cycle and
/// call kept; any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn path_data_reload_paths(c: &mut Cpu) {
    if c.pc != 0x0026ae70 {
        return ee::s07::f_0026ae70__generated(c);
    }
    guest(c, |c| {
        'b_26aeb0: {
            'b_26aea0: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[S1][0] = c.g[A0][0];
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                let t = c.g[A0][0] == 0u64;
                c.g[S0][0] = c.g[A1][0];
                if t {
                    break 'b_26aea0;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0026ae98);
                    call_ptr(c, 0x00317e98, 0x0026ae98)?;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
                    if t {
                        break 'b_26aeb0;
                    } else {
                        break 'b_26aea0;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0026aeac);
            c.g[A1][0] = c.g[S0][0];
            Callee {
                addr: 0x0026ada8,
                f: ee::s07::PathData_LoadFile,
            }
            .call(c, 0x0026aeac)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
            break 'b_26aeb0;
        }
        c.cycles += 20;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0026aebc);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x003e22f0, 0x0026aebc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[S0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0026aed4);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        Callee {
            addr: 0x0026ba68,
            f: ee::s07::PathData_ReadInt,
        }
        .call(c, 0x0026aed4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0026aee0);
        c.g[A1][0] = c.g[S0][0];
        Callee {
            addr: 0x0026b410,
            f: ee::s07::PathData_ReadAIPaths,
        }
        .call(c, 0x0026aee0)?;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x0026aeec);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0026b310,
            f: ee::s07::PathData_ReadTrackPaths,
        }
        .call(c, 0x0026aeec)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `cSSXAudio::cSSXAudio(int, int, eSOUNDDEF_MODE)`: constructs the audio system (bank manager, stream array, controller, vtable patches when `a1` is set), clears the channel/volume state, sets the default channel volume, reads the playlist and volume config and loads the loading-screen bank.
/// Hand port of the generated body (SSXAudioSystem_ctor, 0x00284c68), every register, cycle and
/// call kept; any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn ssx_audio_system_ctor(c: &mut Cpu) {
    if c.pc != 0x00284c68 {
        return ee::s07::f_00284c68__generated(c);
    }
    guest(c, |c| {
        'b_284d00: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.g[S2][0] = c.g[A3][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S3][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S4][0] = c.g[T0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
            let t = c.g[S3][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            if t {
                break 'b_284d00;
            } else {
                c.cycles += 22;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c98));
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c90));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7768));
                c.g[A0][0] = c.g[A2][0];
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                c.g[A1][0] = c.g[S4][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x7768), c.g[V1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6e70), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x118), c.g[V0][0]);
                c.g[RA][0] = sx(0x00284cd4);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x5560), c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x00284cd4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7768));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00284ce8);
                c.g[A3][0] = c.g[S4][0];
                Callee {
                    addr: 0x0028b9b8,
                    f: ee::s07::SndBankMan_ctor,
                }
                .call(c, 0x00284ce8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5560));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x00284d00);
                c.g[T0][0] = c.g[S4][0];
                call_ptr(c, 0x002aa490, 0x00284d00)?;
                break 'b_284d00;
            }
        }
        'b_284ea0: {
            c.cycles += 23;
            c.g[A3][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x00284d18);
            c.g[T0][0] = c.g[S4][0];
            call_ptr(c, 0x002ab200, 0x00284d18)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3708));
            c.g[A3][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[T8][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3648));
            c.g[S2][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[T9][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3728));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[T8][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[S3][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T9][0]);
            if t {
                break 'b_284ea0;
            } else {
                c.cycles += 81;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x3708));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[T5][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3648));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A1][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[T4][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[T3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw(c.g[T2][0] as u32);
                c.g[T1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[T0][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c90));
                c.g[A3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A2][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x38));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T5][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[V0][0] = c.lw(c.g[T2][0] as u32);
                c.g[A3][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x30));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T6][0]);
                c.g[A2][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x8));
                c.g[T7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw(c.g[T7][0] as u32);
                c.g[A0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c98));
                c.g[V0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[T6][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x3728));
                c.g[T0][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T5][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T4][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x18));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T3][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T2][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x28));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T1][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A2][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T6][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T5][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T1][0]);
                c.g[V0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw(c.g[T7][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x1d4), c.g[S0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                break 'b_284ea0;
            }
        }
        c.cycles += 72;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V1][0] = 0x480000;
        c.g[T0][0] = 0x480000;
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0x1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3688));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x36d8));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x36b8));
        c.g[T3][0] = 0xc8;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5728), 0u64);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5734), 0u64);
        c.g[A3][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5740), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2820));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5744), 0u64);
        c.g[A0][0] = 0x320;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x574c), 0u64);
        c.g[A2][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5750), 0u64);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60a4));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5754), 0u64);
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c70));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5758), 0u64);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c78));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5768), 0u64);
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6030));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x576c), 0u64);
        c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x579c));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5770), 0u64);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5774), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x577c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5784), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5788), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x578c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5818), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5820), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5828), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5984), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5988), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5558), c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x571c), c.g[T0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x5730));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5794), c.g[T2][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x581c), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x597c), c.g[T3][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x572c), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5764), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5780), c.g[T2][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5790), c.g[T2][0]);
        c.g[RA][0] = sx(0x00284f80);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5798), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x00284f80)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5980), c.g[V0][0]);
        c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6394));
        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x63e4));
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = 0x1d;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fdc), c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fc0), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fc8), c.g[V1][0]);
        c.g[V0][0] = 0x2;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fd4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fd8), 0u64);
        'l_284fc0: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0xffffffffffffffff;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00284fc0);
                }
                continue 'l_284fc0;
            } else {
                c.cycles += 37;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A2][0] = 0x1;
                c.g[V1][0] = 0x17;
                c.g[V0][0] = 0x3;
                c.g[A1][0] = 0x3c;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6284), c.g[V1][0]);
                c.mov_s(1, 0);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x63d4), c.g[V0][0]);
                c.g[V1][0] = c.g[A3][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c54), c.g[A1][0]);
                c.g[V0][0] = c.g[T0][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x646c), c.g[A0][0]);
                c.g[A1][0] = 0xa;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c88), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62b0), c.g[S4][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x603c), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6040), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x608c), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6090), 0u64);
                c.sd((c.g[S1][0] as u32).wrapping_add(0x6098), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6254), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x627c), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6294), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6298), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62a4), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62a8), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62b4), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62b8), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x63c0), 0u64);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x63d0));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x63d8));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x63dc), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6474), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c58), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c60), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6250), 0u64);
                'l_285070: loop {
                    c.cycles += 7;
                    c.swc1(1, c.g[V0][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.swc1(1, c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = (c.g[A1][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00285070);
                        }
                        continue 'l_285070;
                    } else {
                        c.cycles += 27;
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4660));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0028509c);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x0028509c)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x6470), 0u64);
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x002850ac);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00288f60,
                            f: ee::s07::SSXAudioSystem_SetDebugMode,
                        }
                        .call(c, 0x002850ac)?;
                        c.g[RA][0] = sx(0x002850b4);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00289470,
                            f: ee::s07::SSXAudioSystem_ReadPlaylistNames,
                        }
                        .call(c, 0x002850b4)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002850c0);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00289520,
                            f: ee::s07::SSXAudioSystem_LoadPlaylist,
                        }
                        .call(c, 0x002850c0)?;
                        c.g[RA][0] = sx(0x002850c8);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00287fc8,
                            f: ee::s07::SSXAudioSystem_LoadVolumeConfig,
                        }
                        .call(c, 0x002850c8)?;
                        c.g[RA][0] = sx(0x002850d0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00297f70, 0x002850d0)?;
                        c.g[A0][0] = 0x290000;
                        c.g[RA][0] = sx(0x002850dc);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x69b0));
                        call_ptr(c, 0x003b5a28, 0x002850dc)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fb0), 0u64);
                        c.g[A1][0] = 0xffffffffffffffff;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fb4), 0u64);
                        c.g[V1][0] = c.g[S3][0];
                        c.g[V0][0] = c.g[S5][0];
                        c.g[A0][0] = 0x1;
                        'l_2850f8: loop {
                            c.cycles += 7;
                            c.sw(c.g[V0][0] as u32, 0u64);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                            c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                            let t = (c.g[A0][0] as i64) >= 0;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002850f8);
                                }
                                continue 'l_2850f8;
                            } else {
                                c.cycles += 7;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5fe0), 0u64);
                                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5fe4), 0u64);
                                c.g[A0][0] = 0xffffffffffffffff;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5fec), 0u64);
                                c.g[V1][0] = 0x2;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5ff0), 0u64);
                                'l_285130: loop {
                                    c.cycles += 7;
                                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[V1][0] as i64) >= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00285130);
                                        }
                                        continue 'l_285130;
                                    } else {
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x00285154);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x002948a0,
                                            f: ee::s07::SSXAudioSystem_ClearRiderAirFlags,
                                        }
                                        .call(c, 0x00285154)?;
                                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x58));
                                        c.g[V1][0] = 0x16;
                                        'l_285160: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, 0u64);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00285160);
                                                }
                                                continue 'l_285160;
                                            } else {
                                                c.cycles += 36;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x00285188);
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x002a4590, 0x00285188)?;
                                                c.g[S0][0] = 0x4a0000;
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x00285198);
                                                c.g[A0][0] = c.g[S1][0];
                                                call_ptr(c, 0x0029ed90, 0x00285198)?;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x3568));
                                                c.g[RA][0] = sx(0x002851a4);
                                                c.g[A0][0] = c.g[S2][0];
                                                call_ptr(c, 0x002b0ce0, 0x002851a4)?;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = 0x4;
                                                c.g[RA][0] = sx(0x002851b4);
                                                c.g[A2][0] = 0u64;
                                                Callee {
                                                    addr: 0x00287968,
                                                    f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
                                                }
                                                .call(c, 0x002851b4)?;
                                                c.g[T0][0] = c.g[V0][0];
                                                c.g[A3][0] = c.g[S0][0];
                                                c.g[T1][0] = 0x1;
                                                c.g[T2][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x002851d4);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x002b0e60, 0x002851d4)?;
                                                c.g[RA][0] = sx(0x002851dc);
                                                c.g[A0][0] = c.g[S1][0];
                                                Callee { addr: 0x0028f700, f: ee::s07::SSXAudioSystem_LoadLoadingScreenBank }.call(c, 0x002851dc)?;
                                                c.g[V0][0] = c.g[S1][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
                                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                                c.g[RA][0] =
                                                    c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                                                let t = c.g[RA][0] as u32;
                                                c.g[SP][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x130));
                                                c.pc = t;
                                                return Ok(());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `void SSXAudioSystem_LoadWorldAudio(SSXAudioSystem* self)`: parses the audio ini (GLOBAL basepath, then the section named like the world extension) and loads the MAIN, BOARD, SPUBOARD, CROWD, AUX, TRICKY, MOUNTAIN, TRANSPORT and LAND banks, then sets the bank memory, loads the crowd audio and the grunt bank.
/// Hand port of the generated body (SSXAudioSystem_LoadWorldAudio, 0x002862a8), every register, cycle and
/// call kept; any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn ssx_audio_system_load_world_audio(c: &mut Cpu) {
    if c.pc != 0x002862a8 {
        return ee::s07::f_002862a8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002862a8;
        loop {
            match pc {
                0x002862a8 => {
                    c.cycles += 43;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xad0));
                    c.g[V0][0] = 0x4a0000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xac0));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35a0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa80));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa60));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xab0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xaa0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa90));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa70));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa50));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xa40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xa30), c.g[RA][0]);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x35a0));
                    c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.g[RA][0] = sx(0x002862f8);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00288d18,
                        f: ee::s07::SSXAudioSystem_SetupChannelGains,
                    }
                    .call(c, 0x002862f8)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0028630c);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    Callee {
                        addr: 0x0028a1e8,
                        f: ee::s07::SndLoadQueue_AllocAsyncBuffer,
                    }
                    .call(c, 0x0028630c)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x35a8));
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x5878), 0u64);
                    c.g[A2][0] = 0x100;
                    c.sb((c.g[S4][0] as u32).wrapping_add(0x5838), c.g[V1][0]);
                    c.g[S6][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00286334);
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x464));
                    call_ptr(c, 0x002a77f8, 0x00286334)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3570));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0028634c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[V1][0]);
                    call_ptr(c, 0x002a7bf8, 0x0028634c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    if t {
                        pc = 0x002863a0;
                        continue;
                    }
                    pc = 0x00286354;
                    continue;
                }
                0x00286354 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00286390;
                        continue;
                    }
                    pc = 0x0028635c;
                    continue;
                }
                0x0028635c => {
                    c.cycles += 1;
                    pc = 0x00286360;
                    continue;
                }
                0x00286360 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00286368);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7da0, 0x00286368)?;
                    c.g[V1][0] = 0x480000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28c0));
                    c.g[RA][0] = sx(0x0028637c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x0028637c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x00286390;
                        continue;
                    }
                    pc = 0x00286384;
                    continue;
                }
                0x00286384 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286390);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x002a7ff8, 0x00286390)?;
                    pc = 0x00286390;
                    continue;
                }
                0x00286390 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00286398);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7f90, 0x00286398)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    if t {
                        if back_edge_unwinds(c, 0x00286360) {
                            return Ok(());
                        }
                        pc = 0x00286360;
                        continue;
                    }
                    pc = 0x002863a0;
                    continue;
                }
                0x002863a0 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa20), c.g[V1][0]);
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x950));
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x9b0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                    c.g[FP][0] = 0x4a0000;
                    let t = true;
                    c.g[S7][0] = 0x4a0000;
                    if t {
                        pc = 0x002866ec;
                        continue;
                    }
                    pc = 0x002863c0;
                    continue;
                }
                0x002863c0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002863c8);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x004165a8, 0x002863c8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa20));
                    if t {
                        pc = 0x002866ec;
                        continue;
                    }
                    pc = 0x002863d0;
                    continue;
                }
                0x002863d0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x002863d8);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x004166f4, 0x002863d8)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa20));
                    c.g[RA][0] = sx(0x002863e4);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x587c));
                    call_ptr(c, 0x004166f4, 0x002863e4)?;
                    let t = true;
                    c.g[S5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa20));
                    if t {
                        pc = 0x002866dc;
                        continue;
                    }
                    pc = 0x002863ec;
                    continue;
                }
                0x002863ec => {
                    c.cycles += 1;
                    pc = 0x002863f0;
                    continue;
                }
                0x002863f0 => {
                    c.cycles += 15;
                    c.g[RA][0] = sx(0x002863f8);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7da0, 0x002863f8)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[V0][0] = 0x480000;
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286410);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28c0));
                    call_ptr(c, 0x002a7ff8, 0x00286410)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3580));
                    c.g[RA][0] = sx(0x00286424);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286424)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286460;
                        continue;
                    }
                    pc = 0x0028642c;
                    continue;
                }
                0x0028642c => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0028643c);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x0028643c)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286454);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286454)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286460;
                    continue;
                }
                0x00286460 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                    c.g[RA][0] = sx(0x00286470);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286470)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002864ac;
                        continue;
                    }
                    pc = 0x00286478;
                    continue;
                }
                0x00286478 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286488);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286488)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002864a0);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002864a0)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x002864ac;
                    continue;
                }
                0x002864ac => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28d0));
                    c.g[RA][0] = sx(0x002864bc);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002864bc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002864f8;
                        continue;
                    }
                    pc = 0x002864c4;
                    continue;
                }
                0x002864c4 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x002864d4);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x002864d4)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002864ec);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002864ec)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x10;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x002864f8;
                    continue;
                }
                0x002864f8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b8));
                    c.g[RA][0] = sx(0x00286508);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286508)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286544;
                        continue;
                    }
                    pc = 0x00286510;
                    continue;
                }
                0x00286510 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286520);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286520)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286538);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286538)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x5;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286544;
                    continue;
                }
                0x00286544 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35c0));
                    c.g[RA][0] = sx(0x00286554);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286554)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286590;
                        continue;
                    }
                    pc = 0x0028655c;
                    continue;
                }
                0x0028655c => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0028656c);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x0028656c)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286584);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286584)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x6;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286590;
                    continue;
                }
                0x00286590 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35c8));
                    c.g[RA][0] = sx(0x002865a0);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002865a0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002865dc;
                        continue;
                    }
                    pc = 0x002865a8;
                    continue;
                }
                0x002865a8 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x002865b8);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x002865b8)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002865d0);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002865d0)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x7;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x002865dc;
                    continue;
                }
                0x002865dc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28e0));
                    c.g[RA][0] = sx(0x002865ec);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002865ec)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00286628;
                        continue;
                    }
                    pc = 0x002865f4;
                    continue;
                }
                0x002865f4 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286604);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286604)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x0028661c);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x0028661c)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286628;
                    continue;
                }
                0x00286628 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28f0));
                    c.g[RA][0] = sx(0x00286638);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286638)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286688;
                        continue;
                    }
                    pc = 0x00286640;
                    continue;
                }
                0x00286640 => {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286650);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286650)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286668);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286668)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0x4;
                    pc = 0x00286670;
                    continue;
                }
                0x00286670 => {
                    c.cycles += 6;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00286680);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0028bc58,
                        f: ee::s07::SndBankMan_LoadBank,
                    }
                    .call(c, 0x00286680)?;
                    let t = true;

                    if t {
                        pc = 0x002866dc;
                        continue;
                    }
                    pc = 0x00286688;
                    continue;
                }
                0x00286688 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35d0));
                    c.g[RA][0] = sx(0x00286698);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286698)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x002866dc;
                        continue;
                    }
                    pc = 0x002866a0;
                    continue;
                }
                0x002866a0 => {
                    c.cycles += 15;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x002866ac);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x002866ac)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002866c4);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002866c4)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0xa;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x002866dc);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0028bc58,
                        f: ee::s07::SndBankMan_LoadBank,
                    }
                    .call(c, 0x002866dc)?;
                    pc = 0x002866dc;
                    continue;
                }
                0x002866dc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002866e4);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7f90, 0x002866e4)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x002863f0) {
                            return Ok(());
                        }
                        pc = 0x002863f0;
                        continue;
                    }
                    pc = 0x002866ec;
                    continue;
                }
                0x002866ec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002866f4);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7b08, 0x002866f4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x002863c0) {
                            return Ok(());
                        }
                        pc = 0x002863c0;
                        continue;
                    }
                    pc = 0x002866fc;
                    continue;
                }
                0x002866fc => {
                    c.cycles += 27;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = 0x10000;
                    c.g[A1][0] = 0x3;
                    c.g[A2][0] = c.g[A2][0] | 0x9000;
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x0028671c);
                    c.g[T0][0] = 0u64;
                    Callee {
                        addr: 0x0028bda8,
                        f: ee::s07::SndBankMan_SetBankMemory,
                    }
                    .call(c, 0x0028671c)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = 0x60000;
                    c.g[A1][0] = 0xc;
                    c.g[A2][0] = c.g[A2][0] | 0xa400;
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x0028673c);
                    c.g[T0][0] = 0u64;
                    Callee {
                        addr: 0x0028bda8,
                        f: ee::s07::SndBankMan_SetBankMemory,
                    }
                    .call(c, 0x0028673c)?;
                    c.g[RA][0] = sx(0x00286744);
                    call_ptr(c, 0x0014dc80, 0x00286744)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00286750);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00286750)?;
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = 0x2;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                    let t = c.g[V0][0] != c.g[A0][0];
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x468));
                    if t {
                        pc = 0x00286784;
                        continue;
                    }
                    pc = 0x00286768;
                    continue;
                }
                0x00286768 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x2;
                    c.g[RA][0] = sx(0x0028677c);
                    c.g[T0][0] = 0x5;
                    call_ptr(c, 0x002a6d78, 0x0028677c)?;
                    let t = true;
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        pc = 0x0028679c;
                        continue;
                    }
                    pc = 0x00286784;
                    continue;
                }
                0x00286784 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x00286798);
                    c.g[T0][0] = 0x5;
                    call_ptr(c, 0x002a6d78, 0x00286798)?;
                    c.g[A0][0] = c.g[S4][0];
                    pc = 0x0028679c;
                    continue;
                }
                0x0028679c => {
                    c.cycles += 19;
                    c.g[RA][0] = sx(0x002867a4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0029f3f8, 0x002867a4)?;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.g[S6][0];
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x627c), c.g[V0][0]);
                    c.g[RA][0] = sx(0x002867b8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002a7890, 0x002867b8)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xac0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xab0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xaa0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa90));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa80));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa70));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa60));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa50));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xa40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xad0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002862a8, 0x002867e8);
                    return Ok(());
                }
            }
        }
    });
}

/// `void SSXAudioSystem_SetChannelVolume(SSXAudioSystem* self, int channel, float volume)`: clamps the volume to [0, 1], scales it per channel (jump table, constants at gp+0x414..), stores volume and gain in the channel record at +0x62bc and for channel 0/1/4 updates the music player, the master volume or the per-bank gains.
/// Hand port of the generated body (SSXAudioSystem_SetChannelVolume, 0x00287700), every register, cycle and
/// call kept; any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn ssx_audio_system_set_channel_volume(c: &mut Cpu) {
    if c.pc != 0x00287700 {
        return ee::s07::f_00287700__generated(c);
    }
    guest(c, |c| {
        'b_2877e0: {
            'b_2877dc: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 4);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62bc));
                c.mtc1(0, 1);
                c.g[S1][0] = c.g[A0][0];
                c.min_s(12, 12, 0);
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = (c.g[S2][0] < 0xbu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.max_s(12, 12, 1);
                if t {
                    break 'b_2877dc;
                } else {
                    'b_2877d0: {
                        'b_2877cc: {
                            'b_2877c4: {
                                'b_2877bc: {
                                    'b_2877b4: {
                                        'b_2877ac: {
                                            'b_2877a4: {
                                                'b_28779c: {
                                                    'b_287794: {
                                                        'b_28778c: {
                                                            'b_28776c: {
                                                                c.cycles += 7;
                                                                c.g[V0][0] = 0x480000;
                                                                c.g[V1][0] =
                                                                    sx((c.g[S2][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2900));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x00287760 => {
                                                                        c.cycles += 3;
                                                                        c.swc1(
                                                                            12,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        let t = true;
                                                                        c.swc1(
                                                                            12,
                                                                            c.g[S0][0] as u32,
                                                                        );
                                                                        if t {
                                                                            break 'b_2877dc;
                                                                        } else {
                                                                            break 'b_28776c;
                                                                        }
                                                                    }
                                                                    0x0028776c => {
                                                                        break 'b_28776c;
                                                                    }
                                                                    0x0028778c => {
                                                                        break 'b_28778c;
                                                                    }
                                                                    0x00287794 => {
                                                                        break 'b_287794;
                                                                    }
                                                                    0x0028779c => {
                                                                        break 'b_28779c;
                                                                    }
                                                                    0x002877a4 => {
                                                                        break 'b_2877a4;
                                                                    }
                                                                    0x002877ac => {
                                                                        break 'b_2877ac;
                                                                    }
                                                                    0x002877b4 => {
                                                                        break 'b_2877b4;
                                                                    }
                                                                    0x002877bc => {
                                                                        break 'b_2877bc;
                                                                    }
                                                                    0x002877c4 => {
                                                                        break 'b_2877c4;
                                                                    }
                                                                    0x002877cc => {
                                                                        break 'b_2877cc;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x00287700,
                                                                            0x00287920,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 8;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x418),
                                                            );
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x118));
                                                            c.swc1(12, c.g[S0][0] as u32);
                                                            c.mul_s(0, 12, 0);
                                                            c.g[RA][0] = sx(0x00287784);
                                                            c.swc1(
                                                                0,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            call_ptr(c, 0x002b4388, 0x00287784)?;
                                                            let t = true;
                                                            c.g[V0][0] =
                                                                sx((c.g[S2][0] as u32) << 2);
                                                            if t {
                                                                break 'b_2877e0;
                                                            } else {
                                                                break 'b_28778c;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32).wrapping_add(0x420),
                                                        );
                                                        if t {
                                                            break 'b_2877d0;
                                                        } else {
                                                            break 'b_287794;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_add(0x424),
                                                    );
                                                    if t {
                                                        break 'b_2877d0;
                                                    } else {
                                                        break 'b_28779c;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x41c));
                                                if t {
                                                    break 'b_2877d0;
                                                } else {
                                                    break 'b_2877a4;
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x414));
                                            if t {
                                                break 'b_2877d0;
                                            } else {
                                                break 'b_2877ac;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x428));
                                        if t {
                                            break 'b_2877d0;
                                        } else {
                                            break 'b_2877b4;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x42c));
                                    if t {
                                        break 'b_2877d0;
                                    } else {
                                        break 'b_2877bc;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x430));
                                if t {
                                    break 'b_2877d0;
                                } else {
                                    break 'b_2877c4;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x434));
                            if t {
                                break 'b_2877d0;
                            } else {
                                break 'b_2877cc;
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x438));
                        break 'b_2877d0;
                    }
                    c.cycles += 3;
                    c.swc1(12, c.g[S0][0] as u32);
                    c.mul_s(0, 12, 0);
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    break 'b_2877dc;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
            break 'b_2877e0;
        }
        'b_287910: {
            'b_28790c: {
                'b_287870: {
                    'b_287860: {
                        c.cycles += 11;
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = 0x1;
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x6394));
                        c.g[V0][0] = c.g[V1][0];
                        c.mul_s(1, 1, 0);
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x8));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x63e4));
                        c.mul_s(1, 1, 0);
                        let t = c.g[S2][0] == c.g[A0][0];
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_287860;
                        } else {
                            'b_287838: {
                                'b_287828: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = 0x4;
                                        break 'b_287828;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S2][0] == 0u64;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_287838;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_287910;
                                            } else {
                                                break 'b_287828;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S2][0] == c.g[V0][0];
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x43c));
                                if t {
                                    break 'b_287870;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_28790c;
                                    } else {
                                        break 'b_287838;
                                    }
                                }
                            }
                            c.cycles += 10;
                            c.g[AT][0] = 0x42fe0000;
                            c.mtc1(1, 0);
                            c.mul_s(0, 1, 0);
                            c.cvt_w_s(1, 0);
                            c.mfc1(4, 1);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                            c.g[RA][0] = sx(0x00287858);
                            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                            call_ptr(c, 0x003b8160, 0x00287858)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_28790c;
                            } else {
                                break 'b_287860;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00287868);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x002b4388, 0x00287868)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_28790c;
                    } else {
                        break 'b_287870;
                    }
                }
                c.cycles += 39;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x440));
                c.mul_s(0, 1, 0);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_add(0x444));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_add(0x448));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_add(0x44c));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x636c));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_add(0x450));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_add(0x454));
                c.mul_s(1, 1, 2);
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_add(0x458));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x45c));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_add(0x460));
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6370));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 3);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x6374));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 4);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6378));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 5);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x637c));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 6);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6380));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 7);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x6384));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 8);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6388));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x638c));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 9);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6390));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_28790c;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_287910;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fresh, put_words, run_generated, Rng, DATA, GAME_GP};
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;

    /// Gate word of the oc805 fakes: they act only on machines that hold the magic here.
    pub(crate) const OC805_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC805_MAGIC: u32 = 0x0c80_5a5a;
    /// Result queues of the fakes: one 0x80-byte record per callee at `OC805_Q + 0x80 * index`
    /// (word 0: calls made, word 1: code after the queue ran out, words 2..: queued codes).
    pub(crate) const OC805_Q: u32 = DATA + 0x20_1000;
    pub(crate) const OC805_QLEN: u32 = 24;

    /// Every callee of the four functions (their override slots are faked).
    pub(crate) const OC805_CALLEES: &[u32] = &[
        0x0026ada8, 0x0026b310, 0x0026b410, 0x0026ba68, 0x00317e98, 0x003e22f0, 0x00287700,
        0x00287968, 0x00287fc8, 0x00288f60, 0x00289470, 0x00289520, 0x0028b9b8, 0x0028f700,
        0x002948a0, 0x00297f70, 0x0029ed90, 0x002a4590, 0x002aa490, 0x002ab200, 0x002ade88,
        0x002b0ce0, 0x002b0e60, 0x00317e30, 0x003b5a28, 0x0014dc80, 0x0014dd58, 0x00288d18,
        0x0028a1e8, 0x0028bc58, 0x0028bda8, 0x0029f3f8, 0x002a6d78, 0x002a77f8, 0x002a7890,
        0x002a7b08, 0x002a7bf8, 0x002a7da0, 0x002a7f90, 0x002a7ff8, 0x004165a8, 0x004166f4,
        0x00417828, 0x0041aa88, 0x002b4388, 0x003b8160,
    ];

    /// A queue code: 0 = the fake's default pointer, else the forced `v0` plus one.
    pub(crate) fn oc805_code(v: u32) -> u32 {
        v.wrapping_add(1)
    }

    /// Queue the results of the fake for `addr`; `after` is used once the queue ran out.
    pub(crate) fn oc805_queue(c: &mut Cpu, addr: u32, codes: &[u32], after: u32) {
        let i = OC805_CALLEES.iter().position(|&a| a == addr).unwrap() as u32;
        let q = OC805_Q + 0x80 * i;
        c.bus.write32(q, 0);
        c.bus.write32(q + 4, after);
        for (n, &code) in codes.iter().enumerate() {
            c.bus.write32(q + 8 + 4 * n as u32, code);
        }
    }

    /// Random queue: each entry is the forced value `zero` or the default pointer.
    pub(crate) fn oc805_random_queue(
        c: &mut Cpu,
        r: &mut Rng,
        addr: u32,
        len: u32,
        zero_in: u32,
        after: u32,
    ) {
        let codes: Vec<u32> = (0..len)
            .map(|_| {
                if r.next_u32() % zero_in == 0 {
                    oc805_code(0)
                } else {
                    0
                }
            })
            .collect();
        oc805_queue(c, addr, &codes, after);
    }

    fn oc805_unhooked(_c: &mut Cpu) {
        unreachable!("oc805 fake called before it was hooked");
    }

    /// The fake callee for `A`: logs the call like `fakes::fake_call` and returns the queued
    /// result (or the default pointer); machines without the gate word run the old body.
    pub(crate) fn oc805_fake<const A: u32>(c: &mut Cpu) {
        if c.bus.read32(OC805_CTRL) != OC805_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc805_fake::<A>), oc805_unhooked);
        }
        let i = OC805_CALLEES.iter().position(|&a| a == A).unwrap() as u32;
        let q = OC805_Q + 0x80 * i;
        let n = c.bus.read32(q);
        c.bus.write32(q, n + 1);
        let code = if n < OC805_QLEN {
            c.bus.read32(q + 8 + 4 * n)
        } else {
            c.bus.read32(q + 4)
        };
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, A);
        if c.pc == ra && code != 0 {
            c.g[V0][0] = code.wrapping_sub(1) as u64;
        }
    }

    macro_rules! oc805_hooks {
        ($($slot:path => $addr:literal),* $(,)?) => {
            pub(crate) fn oc805_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), oc805_fake::<$addr>, fakes::prev_cell(oc805_fake::<$addr>)); )*
            }
        };
    }

    oc805_hooks! {
        ee::s07::OVERRIDE_0026ADA8 => 0x0026ada8,
        ee::s07::OVERRIDE_0026B310 => 0x0026b310,
        ee::s07::OVERRIDE_0026B410 => 0x0026b410,
        ee::s07::OVERRIDE_0026BA68 => 0x0026ba68,
        ee::s10::OVERRIDE_00317E98 => 0x00317e98,
        ee::s14::OVERRIDE_003E22F0 => 0x003e22f0,
        ee::s07::OVERRIDE_00287700 => 0x00287700,
        ee::s07::OVERRIDE_00287968 => 0x00287968,
        ee::s07::OVERRIDE_00287FC8 => 0x00287fc8,
        ee::s07::OVERRIDE_00288F60 => 0x00288f60,
        ee::s07::OVERRIDE_00289470 => 0x00289470,
        ee::s07::OVERRIDE_00289520 => 0x00289520,
        ee::s07::OVERRIDE_0028B9B8 => 0x0028b9b8,
        ee::s07::OVERRIDE_0028F700 => 0x0028f700,
        ee::s07::OVERRIDE_002948A0 => 0x002948a0,
        ee::s08::OVERRIDE_00297F70 => 0x00297f70,
        ee::s08::OVERRIDE_0029ED90 => 0x0029ed90,
        ee::s08::OVERRIDE_002A4590 => 0x002a4590,
        ee::s08::OVERRIDE_002AA490 => 0x002aa490,
        ee::s08::OVERRIDE_002AB200 => 0x002ab200,
        ee::s08::OVERRIDE_002ADE88 => 0x002ade88,
        ee::s08::OVERRIDE_002B0CE0 => 0x002b0ce0,
        ee::s08::OVERRIDE_002B0E60 => 0x002b0e60,
        ee::s10::OVERRIDE_00317E30 => 0x00317e30,
        ee::s13::OVERRIDE_003B5A28 => 0x003b5a28,
        ee::s01::OVERRIDE_0014DC80 => 0x0014dc80,
        ee::s01::OVERRIDE_0014DD58 => 0x0014dd58,
        ee::s07::OVERRIDE_00288D18 => 0x00288d18,
        ee::s07::OVERRIDE_0028A1E8 => 0x0028a1e8,
        ee::s07::OVERRIDE_0028BC58 => 0x0028bc58,
        ee::s07::OVERRIDE_0028BDA8 => 0x0028bda8,
        ee::s08::OVERRIDE_0029F3F8 => 0x0029f3f8,
        ee::s08::OVERRIDE_002A6D78 => 0x002a6d78,
        ee::s08::OVERRIDE_002A77F8 => 0x002a77f8,
        ee::s08::OVERRIDE_002A7890 => 0x002a7890,
        ee::s08::OVERRIDE_002A7B08 => 0x002a7b08,
        ee::s08::OVERRIDE_002A7BF8 => 0x002a7bf8,
        ee::s08::OVERRIDE_002A7DA0 => 0x002a7da0,
        ee::s08::OVERRIDE_002A7F90 => 0x002a7f90,
        ee::s08::OVERRIDE_002A7FF8 => 0x002a7ff8,
        ee::s15::OVERRIDE_004165A8 => 0x004165a8,
        ee::s15::OVERRIDE_004166F4 => 0x004166f4,
        ee::s15::OVERRIDE_00417828 => 0x00417828,
        ee::s15::OVERRIDE_0041AA88 => 0x0041aa88,
        ee::s08::OVERRIDE_002B4388 => 0x002b4388,
        ee::s13::OVERRIDE_003B8160 => 0x003b8160,
    }

    /// Scrambled machine with the fake hooks, the gate word set and `pc` at the entry.
    fn oc805_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc805_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC805_CTRL, OC805_MAGIC);
        c.pc = entry;
        r
    }

    /// Distinct total cycle counts of the generated body over the seeds (one per path taken).
    fn oc805_paths(generated: fn(&mut Cpu), seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    fn reload_paths_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc805_machine(c, seed, 0xae70, 6, 0x0026_ae70);
        let obj = DATA + 0x1000;
        c.g[A0][0] = obj as u64;
        c.g[A1][0] = (DATA + 0x2000) as u64;
        // +0x48: the loaded path buffer (set: freed first, unset: LoadFile).
        let buf = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
        c.bus.write32(obj + 0x48, buf);
        let _ = r.next_u32();
    }

    /// `PathData_ReloadPaths`: both the free path and the load path, with unwinds in every callee.
    #[test]
    fn path_data_reload_paths_matches() {
        check(
            ee::s07::f_0026ae70__generated,
            path_data_reload_paths,
            200,
            &reload_paths_setup,
        );
        assert!(oc805_paths(ee::s07::f_0026ae70__generated, 200, &reload_paths_setup) >= 2);
    }

    fn ctor_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc805_machine(c, seed, 0x4c68, 26, 0x0028_4c68);
        let this = DATA + 0x100;
        c.g[A0][0] = this as u64;
        // a1: construct the sub-systems (1) or only fix up the existing vtable (0).
        c.g[A1][0] = (seed % 2) as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
        c.g[T0][0] = r.next_u32() as u64;
        // The controller pointers the a1 == 0 path reads: +0x118 -> P -> Q, Q+0x1d8 -> R.
        let (p, q, rr) = (DATA + 0x2_0000, DATA + 0x2_0100, DATA + 0x2_0400);
        c.bus.write32(this + 0x118, p);
        c.bus.write32(p, q);
        c.bus.write32(q + 0x1d8, rr);
    }

    /// `SSXAudioSystem_ctor`: with and without the sub-system construction, with unwinds.
    #[test]
    fn ssx_audio_system_ctor_matches() {
        check(
            ee::s07::f_00284c68__generated,
            ssx_audio_system_ctor,
            200,
            &ctor_setup,
        );
        assert!(oc805_paths(ee::s07::f_00284c68__generated, 200, &ctor_setup) >= 2);
    }

    fn load_world_audio_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc805_machine(c, seed, 0x62a8, 70, 0x0028_62a8);
        let this = DATA + 0x100;
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x118, DATA + 0x2_0000);
        // cBENewRaceInterface.cGamePath == 2 or not.
        c.bus
            .write8(0x0053_5bc8 + 0x49, if seed % 3 == 0 { 2 } else { 1 });
        // The ini parser: FindSection, sections (strcmp 0 = the world section), keys
        // (strcasecmp 0 = this key), section ends. Past their queues the section scan stops
        // and the key loops end.
        oc805_random_queue(c, &mut r, 0x002a_7bf8, 2, 3, 0);
        oc805_random_queue(c, &mut r, 0x002a_7b08, 4, 3, oc805_code(0));
        oc805_random_queue(c, &mut r, 0x0041_65a8, 4, 2, 0);
        oc805_random_queue(c, &mut r, 0x0041_aa88, 24, 3, 0);
        // AtSectionEnd: a forced 0 keeps scanning keys, the default pointer (nonzero) ends
        // the section; past the queue it always ends.
        let ends: Vec<u32> = (0..24)
            .map(|_| {
                if r.next_u32() % 4 == 0 {
                    0
                } else {
                    oc805_code(0)
                }
            })
            .collect();
        oc805_queue(c, 0x002a_7f90, &ends, 0);
    }

    /// `SSXAudioSystem_LoadWorldAudio`: the ini scan with random section and key matches.
    #[test]
    fn ssx_audio_system_load_world_audio_matches() {
        check(
            ee::s07::f_002862a8__generated,
            ssx_audio_system_load_world_audio,
            300,
            &load_world_audio_setup,
        );
        assert!(oc805_paths(ee::s07::f_002862a8__generated, 300, &load_world_audio_setup) >= 6);
    }

    fn set_channel_volume_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc805_machine(c, seed, 0x7700, 3, 0x0028_7700);
        let this = DATA + 0x1_0000;
        c.g[A0][0] = this as u64;
        // Channels 0..=10 (the jump table), 11 and 12 (out of range), and -1.
        let ch = seed % 14;
        c.g[A1][0] = if ch == 13 { u64::MAX } else { ch as u64 };
        c.f[12] = if seed % 5 == 0 {
            r.edge_f32()
        } else {
            r.f32(1.5)
        };
        // The jump table at 0x482900 and the per-channel scale constants at gp + 0x414.
        let arms = [
            0x0028_7760u32,
            0x0028_776c,
            0x0028_778c,
            0x0028_7794,
            0x0028_779c,
            0x0028_77a4,
            0x0028_77ac,
            0x0028_77b4,
            0x0028_77bc,
            0x0028_77c4,
            0x0028_77cc,
        ];
        put_words(c, 0x0048_2900, &arms);
        let consts: Vec<u32> = (0..0x14).map(|_| r.f32(2.0)).collect();
        put_words(c, GAME_GP + 0x414, &consts);
        // The channel records (+0x62bc, 16 bytes each) and the two gain tables (+0x6394, +0x63e4).
        let rec: Vec<u32> = (0..0x60).map(|_| r.f32(2.0)).collect();
        put_words(c, this + 0x6280, &rec);
        let gains: Vec<u32> = (0..0x40).map(|_| r.f32(2.0)).collect();
        put_words(c, this + 0x6360, &gains);
    }

    /// `SSXAudioSystem_SetChannelVolume`: every channel arm of the jump table and the range check.
    #[test]
    fn ssx_audio_system_set_channel_volume_matches() {
        check(
            ee::s07::f_00287700__generated,
            ssx_audio_system_set_channel_volume,
            280,
            &set_channel_volume_setup,
        );
        assert!(
            oc805_paths(
                ee::s07::f_00287700__generated,
                280,
                &set_channel_volume_setup
            ) >= 8
        );
    }
}
