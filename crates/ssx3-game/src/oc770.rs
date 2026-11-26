//! Staging module oc770 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc770`.

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
        0x001d_5488,
        mem_card_state_base_install_callbacks,
        "MemCardStateBase_InstallCallbacks",
    ),
    (0x0020_ec18, hud_meter_set_rect, "HUDMeter_SetRect"),
    (
        0x002b_6b50,
        cworld_sound_relocate_emitters,
        "cWorldSound_RelocateEmitters",
    ),
    (0x002e_5920, snow_fall_ctor, "SnowFall_ctor"),
    (
        0x0019_e588,
        fe_module_rider_preview_request_char,
        "FEModule_RiderPreviewRequestChar",
    ),
    (0x0036_91f8, graphics_man_ctor, "GraphicsMan_ctor"),
];

/// `cFEMemCard::InstallCallbacks(self, mode)` (0x1d5488): the memory-card
/// front-end state registers itself as the memory-card overlay manager's user.
///
/// Clears the state's flags, fetches the MC manager (`cSSXApp_getMCManager`),
/// stores the 40 `cFEMemCard_callback*` function pointers into the manager's
/// callback table (`fp + 0x00 .. 0xa8`), starts the manager (`ManagerGainState`),
/// sizes the read/write sets by `mode` (2: save game, 1: 0x80000 bytes,
/// else: save profile) and builds the save file name with `sprintf`.
/// The straight-line table setup is kept in the generated order (every
/// scratch register the code leaves behind is part of the call logs).
#[allow(unreachable_code)]
pub fn mem_card_state_base_install_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d583c: {
            'b_1d57d4: {
                c.cycles += 197;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x6b0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x630));
                c.g[S7][0] = c.g[A0][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x600), c.g[A1][0]);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x690));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x680));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x670));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x660));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x650));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x640));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x620));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x610), c.g[RA][0]);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x10c8), c.g[S7][0]);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1c0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x214), 0u64);
                call_ptr(c, 0x00227f80, 0x001d54d0)?;
                c.g[T3][0] = 0x1e0000;
                c.g[FP][0] = c.g[V0][0];
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x22b8));
                c.g[T9][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x68), c.g[T3][0]);
                c.g[T8][0] = 0x1d0000;
                c.g[T3][0] = 0x1e0000;
                c.g[S6][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x2288));
                c.g[S5][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x74), c.g[T3][0]);
                c.g[S4][0] = 0x1d0000;
                c.g[T3][0] = 0x1e0000;
                c.g[S3][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x2248));
                c.g[S2][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x84), c.g[T3][0]);
                c.g[A0][0] = 0x1d0000;
                c.g[T3][0] = 0x1d0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5f38));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x68e8));
                c.g[T7][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x44), c.g[T3][0]);
                c.g[T6][0] = 0x1e0000;
                c.g[T3][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7010));
                c.g[A0][0] = 0x1e0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x40), c.g[T3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2298));
                c.g[T3][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x70), c.g[A0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x5c28));
                c.g[A0][0] = 0x1e0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[T3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x22c8));
                c.g[T3][0] = 0x1e0000;
                c.g[T5][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x2258));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x88), c.g[T3][0]);
                c.g[V0][0] = 0x1d0000;
                c.g[T3][0] = 0x1d0000;
                c.g[V1][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7318));
                c.g[A1][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x48), c.g[T3][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x59a0));
                c.g[T3][0] = 0x1d0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5df0));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7420));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f40));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x50), c.g[T3][0]);
                c.g[T2][0] = 0x1e0000;
                c.g[T3][0] = 0x1d0000;
                c.g[T4][0] = 0x1e0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7548));
                c.g[S0][0] = 0x1e0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4c), c.g[T3][0]);
                c.g[S1][0] = 0x1e0000;
                c.g[T3][0] = 0x1d0000;
                c.g[A0][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7688));
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x22d8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x54), c.g[T3][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x22a8));
                c.g[T3][0] = 0x1d0000;
                c.sw(c.g[FP][0] as u32, c.g[V0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x72a8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x38), c.g[T3][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2278));
                c.g[T3][0] = c.g[T9][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x67c8));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2268));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x28), c.g[T3][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d78));
                c.g[T3][0] = c.g[T8][0];
                c.g[T1][0] = 0x1e0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x6ad0));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7ea0));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x24), c.g[T3][0]);
                c.g[T0][0] = 0x1e0000;
                c.g[T3][0] = c.g[S6][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x64), c.g[T4][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x6d88));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x6c), c.g[T2][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x3c), c.g[T3][0]);
                c.g[A3][0] = 0x1e0000;
                c.g[T3][0] = c.g[S5][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x77e8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x7c), c.g[S0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x58), c.g[T3][0]);
                c.g[A2][0] = 0x1e0000;
                c.g[T3][0] = c.g[S4][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x80), c.g[S1][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x6c28));
                c.g[A1][0] = 0x1d0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                c.g[V1][0] = 0x1d0000;
                c.g[T3][0] = c.g[S3][0];
                c.g[V0][0] = 0x1d0000;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x66b0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7c68));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[T3][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7c58));
                c.g[T3][0] = c.g[S2][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ca8));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7998));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7dc0));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x60), c.g[T3][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7b18));
                c.g[T3][0] = c.g[T7][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d70));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7ed0));
                c.g[T2][0] = 0x19;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
                c.g[A0][0] = c.g[FP][0];
                c.g[T3][0] = c.g[T6][0];
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7fe0));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x34), c.g[T3][0]);
                c.g[T3][0] = c.g[T5][0];
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7190));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8c), c.g[T3][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x90), c.g[T1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x94), c.g[T0][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x98), c.g[A3][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x9c), c.g[A2][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0x5c), c.g[A1][0]);
                c.sw((c.g[FP][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x19c), c.g[T2][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x600));
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1a8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1bc), c.g[V0][0]);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1ac), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1b0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1b4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1b8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1c4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1cc), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1d8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1dc), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1e0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1e4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1e8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1ec), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1f0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1f4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1f8), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1fc), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x200), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x204), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x208), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x20c), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x210), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x218), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x21c), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1a0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1a4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x220), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x224), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x228), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1d4), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1d0), 0u64);
                c.sw((c.g[S7][0] as u32).wrapping_add(0x1c8), 0u64);
                c.sh((c.g[S7][0] as u32).wrapping_add(0x9c), 0u64);
                call_ptr(c, 0x0023d5a8, 0x001d5778)?;
                call_ptr(c, 0x0014dc80, 0x001d5780)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x5;
                call_ptr(c, 0x0014dd58, 0x001d578c)?;
                c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x1bc));
                c.g[V0][0] = 0x2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_1d57d4;
                } else {
                    c.cycles += 14;
                    call_ptr(c, 0x00152948, 0x001d57a4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0023cc58, 0x001d57b4)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[FP][0];
                    call_ptr(c, 0x0023caa0, 0x001d57c0)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0023c8f0, 0x001d57cc)?;
                    let t = true;
                    c.g[A0][0] = c.g[FP][0];
                    if t {
                        break 'b_1d583c;
                    } else {
                        break 'b_1d57d4;
                    }
                }
            }
            'b_1d5808: {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];

                if t {
                    break 'b_1d5808;
                } else {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0x80000;
                    call_ptr(c, 0x0023cc58, 0x001d57e8)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0x80000;
                    call_ptr(c, 0x0023caa0, 0x001d57f4)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0023c8f0, 0x001d5800)?;
                    let t = true;
                    c.g[A0][0] = c.g[FP][0];
                    if t {
                        break 'b_1d583c;
                    } else {
                        break 'b_1d5808;
                    }
                }
            }
            c.cycles += 13;
            call_ptr(c, 0x00152ba8, 0x001d5810)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[FP][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0023cc58, 0x001d5820)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[FP][0];
            call_ptr(c, 0x0023caa0, 0x001d582c)?;
            c.g[A0][0] = c.g[FP][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0023c8f0, 0x001d5838)?;
            c.g[A0][0] = c.g[FP][0];
            break 'b_1d583c;
        }
        c.cycles += 30;
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xb4));
        c.g[A3][0] = 0x100;
        call_ptr(c, 0x00241d40, 0x001d584c)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14c8));
        call_ptr(c, 0x002c2540, 0x001d5860)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[A1][0] = 0x100;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00241aa0, 0x001d5870)?;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x11c));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x002c26d0, 0x001d5884)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x680));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x670));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x660));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x650));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x640));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x630));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x620));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x610));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `HUDMeter_SetRect(out, _, area)` (0x20ec18): derives the meter's draw
/// rectangles (`out`, 0x34 bytes of floats) from the layout rectangle `area`
/// (x, y, w, h floats at `a2`).
///
/// An inset (`f7`) comes from the active meter's size (`gp-0x98c` object:
/// the int at +0x14 as float times the float at +0x34, times the float at
/// `gp-0x54f4`), or the constant 30.0 when there is no meter (`gp-0x988` is
/// 0). The result is written to `out` together with a copy of `area`'s
/// 16 bytes at `out+0x20` and a scaled `out+0x2c` at `out+0x30` (scale float
/// at `gp-0x54f0`). The float maths runs on the guest FPU registers like the
/// generated body.
pub fn hud_meter_set_rect(c: &mut Cpu) {
    let gp = c.g[GP][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x988));
    let no_meter = c.g[V0][0] == 0;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    if !no_meter {
        c.cycles += 8;
        c.g[V0][0] = c.lw(gp.wrapping_sub(0x98c));
        c.lwc1(2, gp.wrapping_sub(0x54f4));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.cvt_s_w(0, 0);
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
        c.mul_s(0, 0, 1);
        c.mul_s(7, 0, 2);
    } else {
        c.cycles += 2;
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(1, 7);
    }
    c.cycles += 53;
    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = 0x4d0000;
    c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 5);
    c.add_s(0, 0, 2);
    c.lwc1(1, c.g[A2][0] as u32);
    c.mul_s(3, 3, 5);
    c.lwc1(4, (c.g[V0][0] as u32).wrapping_sub(0x7360));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x54f0));
    c.sub_s(0, 0, 7);
    c.add_s(1, 1, 3);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.swc1(1, c.g[SP][0] as u32);
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[SP][0] as u32);
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[A0][0] as u32);
    c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.add_s(1, 1, 0);
    c.sub_s(1, 1, 7);
    c.sub_s(1, 1, 4);
    c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.sub_s(3, 1, 3);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.lwc1(2, c.g[A2][0] as u32);
    c.sub_s(3, 3, 4);
    c.mul_s(0, 0, 5);
    c.mul_s(3, 3, 6);
    c.add_s(2, 2, 0);
    c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x1c));
    c.add_s(0, 0, 4);
    c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x14));
    c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[A2][0] as u32);
    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
    c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x27));
    c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x2f));
    c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x2c));
    c.mul_s(0, 0, 6);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ra;
}

/// Size of one emitter record in a world-sound blob.
pub(crate) const RELOCATE_EMITTER_STRIDE: u32 = 0x18;

/// `cWorldSound_RelocateEmitters(self, blob)` (0x2b6b50): turns a loaded
/// world-sound blob's emitter records from blob-relative to absolute form and
/// registers them (`cWorldSound_InsertEmitters(self)`).
///
/// Returns 0 for a null blob or one whose first byte is set (already
/// relocated). Otherwise, for each of the `blob+0xc` records at `blob+0x10`
/// (24 bytes each): `+0x10` (offset) becomes `blob + offset`; `+0xc` is a
/// sound handle (low byte: bank, upper 24 bits: slot) that is resolved through
/// the sound manager (`*gp[0x16c8]` -> `+8` bank table -> `+0x1c` slot table)
/// to the sound object pointer, or 0 when the handle is -1 or unresolved; a
/// resolved sound gets its back-pointer (`+0x8c`) set to the record.
pub fn cworld_sound_relocate_emitters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[T5][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let blob = c.g[A1][0] as u32;
        let null_blob = blob == 0;
        c.g[V0][0] = 0;
        if !null_blob {
            c.cycles += 3;
            c.g[V0][0] = c.lb(blob);
            let already_relocated = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !already_relocated {
                c.cycles += 4;
                c.g[A2][0] = sx(blob.wrapping_add(0x10));
                c.g[A3][0] = c.lw(blob.wrapping_add(0xc));
                let no_records = (c.g[A3][0] as i64) <= 0;
                c.g[T0][0] = c.g[A2][0];
                if !no_records {
                    c.cycles += 4;
                    c.g[T3][0] = 0xffff_ffff_ffff_0000;
                    let gp = c.g[GP][0] as u32;
                    c.g[T4][0] = c.lw(gp.wrapping_add(0x16c8));
                    c.g[T3][0] |= 0xffff;
                    c.g[T2][0] = c.g[A3][0];
                    loop {
                        c.cycles += 6;
                        let rec = c.g[T0][0] as u32;
                        c.g[V0][0] = c.lw(rec.wrapping_add(0x10));
                        c.g[V1][0] = c.lw(rec.wrapping_add(0xc));
                        c.g[V0][0] = sx(blob.wrapping_add(c.g[V0][0] as u32));
                        c.g[T1][0] = c.g[V1][0];
                        let no_sound = c.g[V1][0] == c.g[T3][0];
                        c.sw(rec.wrapping_add(0x10), c.g[V0][0]);
                        if !no_sound {
                            c.cycles += 8;
                            let mgr = c.g[T4][0] as u32;
                            c.g[V0][0] = c.lw(mgr);
                            c.g[V1][0] &= 0xff;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                            let no_bank = c.g[V1][0] == 0;
                            c.g[V0][0] = sx((c.g[T1][0] as u32) >> 8);
                            let mut resolved = false;
                            if !no_bank {
                                c.cycles += 7;
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                resolved = c.g[V0][0] != 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            }
                            if !resolved {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            }
                            c.cycles += 2;
                            c.sw(rec.wrapping_add(0xc), c.g[V0][0]);
                        } else {
                            c.cycles += 1;
                            c.sw(rec.wrapping_add(0xc), 0);
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(rec.wrapping_add(0xc));
                        if c.g[V0][0] != 0 {
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), c.g[T0][0]);
                        }
                        c.cycles += 3;
                        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(1));
                        let more = c.g[T2][0] != 0;
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(RELOCATE_EMITTER_STRIDE));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x002b_6b90);
                        }
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[T5][0];
                call_ptr(c, 0x002b_6c90, 0x002b_6c14)?;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SnowFall::SnowFall(self)` (0x2e5920): builds the snowfall effect.
///
/// Sets the vtable (`+0xc`), runs the base constructor (0x354648), clears the
/// 12 box slots (`+0x10..+0x3c`), writes two (0, 0, 0, 1.0) vectors at
/// `+0x40`, then
/// allocates and constructs the snow boxes (`operator new(0xc0, name)` +
/// `SnowBox_ctor(box, nbox, kind, f12..f15)`): six at `+0x10..+0x24`, and
/// six more at `+0x28..+0x3c` when the graphics detail level
/// (`world+0x84 -> +0xc -> +0x7c`) is 2 or more. Returns `self`.
/// The calls keep the generated register traffic (the float arguments are
/// moved through `f20..f28`, whose values the caller sees restored).
#[allow(unreachable_code)]
pub fn snow_fall_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x78));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00354648, 0x002e595c)?;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7fd8));
        c.g[V1][0] = 0x1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x00319cc8, 0x002e5978)?;
        c.g[V1][0] = 0xb;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
        'l_2e5980: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002e5980);
                }
                continue 'l_2e5980;
            } else {
                c.cycles += 5;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = 0x1;
                'l_2e59b0: loop {
                    c.cycles += 9;
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                    c.lq(2, c.g[SP][0] as u32);
                    c.sq(2, c.g[V1][0] as u32);
                    let t = (c.g[A0][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002e59b0);
                        }
                        continue 'l_2e59b0;
                    } else {
                        'b_2e5cd0: {
                            c.cycles += 108;
                            c.g[V0][0] = 0x4a0000;
                            c.g[AT][0] = 0x40400000;
                            c.mtc1(1, 21);
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b20));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 22);
                            c.g[AT][0] = 0xffffffffc3480000;
                            c.mtc1(1, 20);
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = 0xc0;
                            c.g[A1][0] = c.g[S1][0];
                            c.lwc1(23, (c.g[GP][0] as u32).wrapping_sub(0x3a10));
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x002e5a0c)?;
                            c.g[AT][0] = 0x44fa0000;
                            c.mtc1(1, 24);
                            c.mov_s(13, 21);
                            c.mov_s(12, 23);
                            c.mov_s(14, 22);
                            c.mov_s(15, 20);
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.lwc1(28, (c.g[GP][0] as u32).wrapping_sub(0x3a0c));
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x002e4d88, 0x002e5a38)?;
                            c.g[AT][0] = 0x44160000;
                            c.mtc1(1, 27);
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = 0xc0;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                            call_ptr(c, 0x00317d70, 0x002e5a58)?;
                            c.lwc1(26, (c.g[GP][0] as u32).wrapping_sub(0x3a08));
                            c.mov_s(12, 23);
                            c.mov_s(13, 21);
                            c.mov_s(14, 22);
                            c.mov_s(15, 20);
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[AT][0] = 0xffffffffc2c80000;
                            c.mtc1(1, 25);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x002e4d88, 0x002e5a84)?;
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = 0xc0;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            call_ptr(c, 0x00317d70, 0x002e5a9c)?;
                            c.mov_s(12, 24);
                            c.mov_s(13, 21);
                            c.mov_s(14, 22);
                            c.mov_s(15, 20);
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x002e4d88, 0x002e5abc)?;
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = 0xc0;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            call_ptr(c, 0x00317d70, 0x002e5ad4)?;
                            c.mov_s(12, 28);
                            c.mov_s(13, 21);
                            c.mov_s(14, 22);
                            c.mov_s(15, 20);
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x002e4d88, 0x002e5af4)?;
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = 0xc0;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                            call_ptr(c, 0x00317d70, 0x002e5b0c)?;
                            c.mov_s(12, 24);
                            c.mov_s(13, 27);
                            c.mov_s(14, 26);
                            c.mov_s(15, 25);
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x002e4d88, 0x002e5b2c)?;
                            c.g[A0][0] = 0xc0;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                            call_ptr(c, 0x00317d70, 0x002e5b44)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0x1;
                            c.mov_s(12, 24);
                            c.mov_s(13, 27);
                            c.mov_s(14, 26);
                            c.mov_s(15, 25);
                            call_ptr(c, 0x002e4d88, 0x002e5b64)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x7c));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A3][0] = 0u64;
                            if t {
                                break 'b_2e5cd0;
                            } else {
                                c.cycles += 83;
                                c.g[A0][0] = 0xc0;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x002e5b94)?;
                                c.mov_s(12, 23);
                                c.mov_s(13, 21);
                                c.mov_s(14, 22);
                                c.mov_s(15, 20);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x002e4d88, 0x002e5bb4)?;
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = 0xc0;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                call_ptr(c, 0x00317d70, 0x002e5bcc)?;
                                c.mov_s(12, 23);
                                c.mov_s(13, 21);
                                c.mov_s(14, 22);
                                c.mov_s(15, 20);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x002e4d88, 0x002e5bec)?;
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = 0xc0;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                call_ptr(c, 0x00317d70, 0x002e5c04)?;
                                c.mov_s(12, 24);
                                c.mov_s(13, 21);
                                c.mov_s(14, 22);
                                c.mov_s(15, 20);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x002e4d88, 0x002e5c24)?;
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = 0xc0;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                call_ptr(c, 0x00317d70, 0x002e5c3c)?;
                                c.mov_s(12, 28);
                                c.mov_s(13, 21);
                                c.mov_s(14, 22);
                                c.mov_s(15, 20);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x002e4d88, 0x002e5c5c)?;
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = 0xc0;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A2][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                                call_ptr(c, 0x00317d70, 0x002e5c74)?;
                                c.mov_s(12, 24);
                                c.mov_s(13, 27);
                                c.mov_s(14, 26);
                                c.mov_s(15, 25);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x002e4d88, 0x002e5c94)?;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = 0xc0;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                                call_ptr(c, 0x00317d70, 0x002e5cac)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.mov_s(12, 24);
                                c.mov_s(13, 27);
                                c.mov_s(14, 26);
                                c.mov_s(15, 25);
                                c.g[A1][0] = 0x1;
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x002e4d88, 0x002e5ccc)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                                break 'b_2e5cd0;
                            }
                        }
                        c.cycles += 17;
                        call_ptr(c, 0x00319cc8, 0x002e5cd8)?;
                        c.g[V0][0] = c.g[S0][0];
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x78));
                        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x68));
                        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
                        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    })
}

/// `FEModule::RiderPreviewRequestChar(self, char_id, load_all)` (0x19e588):
/// records the requested character (`+0xc24`) and load-all flag (`+0xc28`).
/// If the preview already shows that character with a matching load-all
/// state (`+0x0`, `+0x10`) and its model is compiled (`+0xcb8`, `+0xcb4`),
/// the visibility fields are updated (`+0xcd4` = -1, `+0xcc8`/`+0xcc4`);
/// otherwise, when no render is pending (`+0xcd4` < 0), the request is queued:
/// the graphics manager's frame count + 4 becomes `+0xcd4`, the visible flag
/// (`+0xcc8`) is saved to `+0xcc4` and cleared.
pub fn fe_module_rider_preview_request_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0xc24), c.g[A1][0]);
        c.g[V0][0] = c.lw(this);
        let different = c.g[V0][0] != c.g[A1][0];
        c.sw(this.wrapping_add(0xc28), c.g[A2][0]);
        let load_all = c.g[A2][0] != 0;
        // Which tail runs: the "same character" tail (true; `v0` = the current
        // character id) or the pending-request tail (false; `v0` = +0xcd4).
        let same_char;
        if different {
            c.cycles += 1;
            c.g[V0][0] = c.lw(this.wrapping_add(0xcd4));
            same_char = false;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(this.wrapping_add(0x10));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                if !load_all {
                    c.g[V0][0] = c.lw(this);
                    same_char = true;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(this.wrapping_add(0xcd4));
                    same_char = false;
                }
            } else {
                // 2 + 2 + 2 (+ 1 when `load_all` is 0) cycles of the
                // generated three-way flag test.
                c.cycles += 2;
                if load_all {
                    c.g[V0][0] = c.lw(this);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(this);
                }
                same_char = true;
            }
        }
        if same_char {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let big_id = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !big_id {
                c.cycles += 3;
                c.g[V0][0] = c.lw(this.wrapping_add(0xcb8));
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(this.wrapping_add(0xcb4));
                    c.g[V1][0] = (0 < c.g[V0][0]) as u64;
                }
            }
            c.cycles += 2;
            let not_ready = c.g[V1][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !not_ready {
                c.cycles += 5;
                c.g[V1][0] = c.lw(this.wrapping_add(0xcc8));
                c.sw(this.wrapping_add(0xcd4), c.g[V0][0]);
                c.g[A0][0] = sx(this.wrapping_add(0xcc8));
                let shown = c.g[V1][0] != 0;
                c.g[A1][0] = 0;
                let mut set_visible = shown;
                if !shown {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(this.wrapping_add(0xcc4));
                    if c.g[V0][0] == 0 {
                        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    } else {
                        set_visible = true;
                    }
                }
                if set_visible {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                }
                c.cycles += 2;
                c.sw(this.wrapping_add(0xcc4), 0);
            }
        } else {
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 11;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_e65c)?;
            let this = c.g[S0][0] as u32;
            c.g[V1][0] = c.lw(this.wrapping_add(0xcc8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.sw(this.wrapping_add(0xcd4), c.g[V0][0]);
            c.sw(this.wrapping_add(0xcc4), c.g[V1][0]);
            c.sw(this.wrapping_add(0xcc8), 0);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GraphicsMan::GraphicsMan(self)` (0x3691f8): initialises the graphics
/// manager object: its vtable (`+0x10d8`), the global instance pointer
/// (`gp+0x2a90`), the -1-filled 16-bit index tables (`+0xee8..`, `+0xed8..`,
/// `+0xf00..`), a 16-byte register image at `0x501420` (cleared with
/// `memset(.., 0, 16)`, then edited bit by bit and copied to `+0xe88..`),
/// and copies a 0x158-byte default block with `memcpy`. Returns `self`.
/// Straight-line register code, kept in the generated order (the nested
/// loops are the compiler's unrolled table fills).
#[allow(unreachable_code)]
pub fn graphics_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.g[V0][0] = 0x490000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3950));
        c.g[S2][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf50));
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf00));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = 0xffffffffffffffff;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = 0xffffffffffffffff;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0xee8));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_add(0xed8));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0xef0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.g[FP][0] = 0x500000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe88));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10d8), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.g[V0][0] = 0x3;
        'l_369268: loop {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            c.g[A1][0] = 0u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            'l_369278: loop {
                c.cycles += 7;
                c.sh(c.g[V1][0] as u32, c.g[S3][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00369278);
                    }
                    continue 'l_369278;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x10;
                    call_ptr(c, 0x00416210, 0x003692a0)?;
                    c.g[V0][0] = c.g[S1][0];
                    let t = c.g[V0][0] != c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00369268);
                        }
                        continue 'l_369268;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.g[S5][0];
                        c.g[A0][0] = 0u64;
                        c.g[A1][0] = 0xffffffffffffffff;
                        'l_3692b8: loop {
                            c.cycles += 7;
                            c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003692b8);
                                }
                                continue 'l_3692b8;
                            } else {
                                c.cycles += 9;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x10;
                                call_ptr(c, 0x00416210, 0x003692e4)?;
                                c.g[A3][0] = 0xffffffffffffffff;
                                c.g[T0][0] = 0xffffffffffffffff;
                                c.g[V0][0] = c.g[S7][0];
                                c.g[V1][0] = 0x3;
                                'l_3692f8: loop {
                                    c.cycles += 4;
                                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[A0][0] = 0u64;
                                    'l_369308: loop {
                                        c.cycles += 7;
                                        c.sh(c.g[V1][0] as u32, c.g[A3][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00369308);
                                            }
                                            continue 'l_369308;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V1][0] = c.g[A2][0];
                                            let t = c.g[V1][0] != c.g[T0][0];
                                            c.g[V0][0] = c.g[A1][0];
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003692f8);
                                                }
                                                continue 'l_3692f8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                                c.g[A0][0] = 0u64;
                                                c.g[A1][0] = 0xffffffffffffffff;
                                                'l_369340: loop {
                                                    c.cycles += 7;
                                                    c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                    c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00369340);
                                                        }
                                                        continue 'l_369340;
                                                    } else {
                                                        c.cycles += 11;
                                                        c.g[V0][0] = 0x1;
                                                        c.g[V1][0] = 0xffffffffffffffff;
                                                        c.g[V0][0] = 0x1;
                                                        c.g[V1][0] = 0xffffffffffffffff;
                                                        c.sw(
                                                            (c.g[S2][0] as u32).wrapping_add(0xf44),
                                                            0u64,
                                                        );
                                                        c.g[V0][0] = 0x4a0000;
                                                        c.sw(
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_add(0x2a90),
                                                            c.g[S2][0],
                                                        );
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5a10));
                                                        c.g[V1][0] = 0u64;
                                                        c.g[A1][0] = 0xffffffffffffffff;
                                                        'l_369388: loop {
                                                            c.cycles += 7;
                                                            c.sh(c.g[A0][0] as u32, c.g[A1][0]);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] =
                                                                (c.g[V1][0] < 0x2u64) as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x2));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00369388,
                                                                    );
                                                                }
                                                                continue 'l_369388;
                                                            } else {
                                                                c.cycles += 51;
                                                                c.g[S0][0] = sx((c.g[FP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1420));
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[A2][0] = 0x10;
                                                                call_ptr(
                                                                    c, 0x00416210, 0x003693b8,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[V0][0] = 0xfffffffffe7f0000;
                                                                c.g[V0][0] = c.g[V0][0] | 0xffff;
                                                                c.g[A1][0] = 0x800000;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[V0][0];
                                                                c.g[A0][0] = 0xffffffffffbf0000;
                                                                c.g[A0][0] = c.g[A0][0] | 0xffff;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] | c.g[A1][0];
                                                                c.g[V0][0] = 0xffffffffffcf0000;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[A0][0];
                                                                c.g[V0][0] = c.g[V0][0] | 0xffff;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[V0][0];
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[FP][0] as u32)
                                                                        .wrapping_add(0x1420));
                                                                c.g[A3][0] = 0x300000;
                                                                c.g[A0][0] = 0xfffffffffffffc1f;
                                                                c.g[V0][0] = 0xfffffffffff00000;
                                                                c.g[T1][0] = 0xfffffffffffffffc;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] | c.g[A3][0];
                                                                c.g[A1][0] =
                                                                    c.g[A1][0] & c.g[A0][0];
                                                                c.g[V0][0] = c.g[V0][0] | 0xfff;
                                                                c.g[A2][0] =
                                                                    c.g[A2][0] & c.g[T1][0];
                                                                c.g[A0][0] = 0xfffffffffffff3ff;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[V0][0];
                                                                c.g[A2][0] =
                                                                    c.g[A2][0] & c.g[A0][0];
                                                                c.g[A1][0] = c.g[A1][0] | 0x80;
                                                                c.g[V0][0] = 0xffffffffffffffe0;
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[T1][0];
                                                                c.g[A0][0] = 0xfffffffffffffc3f;
                                                                c.g[A1][0] =
                                                                    c.g[A1][0] & c.g[V0][0];
                                                                c.g[A3][0] = 0xffffffffc0000000;
                                                                c.g[A2][0] =
                                                                    c.g[A2][0] & c.g[A0][0];
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    c.g[A1][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[T0][0] =
                                                                    c.g[T0][0] & c.g[A3][0];
                                                                c.sw(
                                                                    (c.g[FP][0] as u32)
                                                                        .wrapping_add(0x1420),
                                                                    c.g[A2][0],
                                                                );
                                                                c.g[T4][0] = c.g[S0][0];
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                    c.g[T0][0],
                                                                );
                                                                c.g[T3][0] = c.g[FP][0];
                                                                c.g[T7][0] = 0x5;
                                                                c.g[T6][0] = 0x1f;
                                                                c.g[T5][0] = 0xff;
                                                                c.g[T2][0] = 0u64;
                                                                c.g[T1][0] = 0xc;
                                                                c.g[T0][0] = 0x2;
                                                                'l_369470: loop {
                                                                    c.cycles += 18;
                                                                    c.g[A3][0] = sx((c.g[T6][0]
                                                                        as u32)
                                                                        << ((c.g[T0][0] as u32)
                                                                            & 31));
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[T4][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V1][0] = sx((c.g[T7][0]
                                                                        as u32)
                                                                        << ((c.g[T0][0] as u32)
                                                                            & 31));
                                                                    c.g[V0][0] =
                                                                        !(0u64 | c.g[A3][0]);
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[T3][0] as u32)
                                                                            .wrapping_add(0x1420));
                                                                    c.g[A0][0] = sx((c.g[T5][0]
                                                                        as u32)
                                                                        << ((c.g[T1][0] as u32)
                                                                            & 31));
                                                                    c.g[A1][0] =
                                                                        c.g[A1][0] & c.g[V0][0];
                                                                    c.g[V1][0] =
                                                                        c.g[V1][0] & c.g[A3][0];
                                                                    c.g[A0][0] =
                                                                        !(0u64 | c.g[A0][0]);
                                                                    c.g[A1][0] =
                                                                        c.g[A1][0] | c.g[V1][0];
                                                                    c.g[A2][0] =
                                                                        c.g[A2][0] & c.g[A0][0];
                                                                    c.g[T2][0] = sx((c.g[T2][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.sw(
                                                                        (c.g[T4][0] as u32)
                                                                            .wrapping_add(0x4),
                                                                        c.g[A1][0],
                                                                    );
                                                                    c.g[T1][0] = sx((c.g[T1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8));
                                                                    c.sw(
                                                                        (c.g[T3][0] as u32)
                                                                            .wrapping_add(0x1420),
                                                                        c.g[A2][0],
                                                                    );
                                                                    c.g[V0][0] = (c.g[T2][0]
                                                                        < 0x2u64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[T0][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x5));
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x00369470,
                                                                            );
                                                                        }
                                                                        continue 'l_369470;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[A1][0] = 0u64;
                                                                        c.g[A2][0] = 0x500000;
                                                                        c.g[A3][0] = 0x3;
                                                                        c.g[A0][0] = 0x2;
                                                                        'l_3694c8: loop {
                                                                            c.cycles += 9;
                                                                            c.g[V0][0] =
                                                                                sx((c.g[A3][0]
                                                                                    as u32)
                                                                                    << ((c.g[A0][0]
                                                                                        as u32)
                                                                                        & 31));
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [A2][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1420,
                                                                                ));
                                                                            c.g[V0][0] = !(0u64
                                                                                | c.g[V0][0]);
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                & c.g[V0][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x2,
                                                                                    ));
                                                                            c.g[V0][0] = (c.g[A1]
                                                                                [0]
                                                                                < 0x2u64)
                                                                                as u64;
                                                                            let t =
                                                                                c.g[V0][0] != 0u64;
                                                                            c.sw(
                                                                                (c.g[A2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1420,
                                                                                    ),
                                                                                c.g[V1][0],
                                                                            );
                                                                            if t {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x003694c8);
                                                                                }
                                                                                continue 'l_3694c8;
                                                                            } else {
                                                                                c.cycles += 11;
                                                                                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1420));
                                                                                c.g[V0][0] =
                                                                                    0x4a0000;
                                                                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a10));
                                                                                c.lwl(5, (c.g[A0][0] as u32).wrapping_add(0x3));
                                                                                c.lwr(
                                                                                    5,
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                );
                                                                                c.swl(5, (c.g[V1][0] as u32).wrapping_add(0x13));
                                                                                c.swr(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                c.g[S0][0] =
                                                                                    c.g[FP][0];
                                                                                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe88));
                                                                                c.g[V1][0] = 0x3;
                                                                                'l_369518: loop {
                                                                                    c.cycles += 14;
                                                                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1420));
                                                                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                                                    c.ldr(
                                                                                        5,
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                                                                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                                                                                    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                                                                                    c.sdr(
                                                                                        5,
                                                                                        c.g[V0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                                                                                    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                                                    let t = (c.g[V1]
                                                                                        [0]
                                                                                        as i64)
                                                                                        >= 0;
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x00369518);
                                                                                        }
                                                                                        continue 'l_369518;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            33;
                                                                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V0][0] = 0x4a0000;
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0xe84), c.g[A1][0]);
                                                                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5960));
                                                                                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                                                        c.ldr(6, c.g[A0][0] as u32);
                                                                                        c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x7));
                                                                                        c.sdr(6, c.g[S2][0] as u32);
                                                                                        c.g[A1][0] = 0xffffffffffff0000;
                                                                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                        c.g[A1][0] = c.g[A1][0] | 0xffff;
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                                                                                        c.g[A2]
                                                                                            [0] =
                                                                                            0x158;
                                                                                        call_ptr(c, 0x003e6448, 0x00369588)?;
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        c.g[V0]
                                                                                            [0] = c
                                                                                            .g[S2]
                                                                                            [0];
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x270), c.g[V1][0]);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x26c), 0u64);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x274), 0u64);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0xf48), 0u64);
                                                                                        c.sw((c.g[S2][0] as u32).wrapping_add(0xf4c), 0u64);
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                                                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                                                                                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                                                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                                                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                        let t = c.g
                                                                                            [RA][0]
                                                                                            as u32;
                                                                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                                                                                        c.pc = t;
                                                                                        return Ok(
                                                                                            (),
                                                                                        );
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
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Magic word at `OC770_CTRL`: while a machine holds it, the fakes of this
    /// module's callees are live (otherwise they run the body they replaced).
    const OC770_CTRL: u32 = DATA + 0x30_0000;
    const OC770_MAGIC: u32 = 0x4f43_3770;

    macro_rules! oc770_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC770_CTRL) != OC770_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc770_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc770_fakes!(
        (
            oc770_fake_2b6c90,
            OC770_PREV_2B6C90,
            ee::s08::OVERRIDE_002B6C90,
            ee::s08::f_002b6c90__generated,
            0x002b_6c90
        ),
        (
            oc770_fake_227f80,
            OC770_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc770_fake_23d5a8,
            OC770_PREV_23D5A8,
            ee::s06::OVERRIDE_0023D5A8,
            ee::s06::f_0023d5a8__generated,
            0x0023_d5a8
        ),
        (
            oc770_fake_14dc80,
            OC770_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc770_fake_14dd58,
            OC770_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc770_fake_152948,
            OC770_PREV_152948,
            ee::s01::OVERRIDE_00152948,
            ee::s01::f_00152948__generated,
            0x0015_2948
        ),
        (
            oc770_fake_23cc58,
            OC770_PREV_23CC58,
            ee::s06::OVERRIDE_0023CC58,
            ee::s06::f_0023cc58__generated,
            0x0023_cc58
        ),
        (
            oc770_fake_23caa0,
            OC770_PREV_23CAA0,
            ee::s06::OVERRIDE_0023CAA0,
            ee::s06::f_0023caa0__generated,
            0x0023_caa0
        ),
        (
            oc770_fake_23c8f0,
            OC770_PREV_23C8F0,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0
        ),
        (
            oc770_fake_152ba8,
            OC770_PREV_152BA8,
            ee::s01::OVERRIDE_00152BA8,
            ee::s01::f_00152ba8__generated,
            0x0015_2ba8
        ),
        (
            oc770_fake_241d40,
            OC770_PREV_241D40,
            ee::s06::OVERRIDE_00241D40,
            ee::s06::f_00241d40__generated,
            0x0024_1d40
        ),
        (
            oc770_fake_2c2540,
            OC770_PREV_2C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540
        ),
        (
            oc770_fake_241aa0,
            OC770_PREV_241AA0,
            ee::s06::OVERRIDE_00241AA0,
            ee::s06::f_00241aa0__generated,
            0x0024_1aa0
        ),
        (
            oc770_fake_2c26d0,
            OC770_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc770_fake_354648,
            OC770_PREV_354648,
            ee::s11::OVERRIDE_00354648,
            ee::s11::f_00354648__generated,
            0x0035_4648
        ),
        (
            oc770_fake_319cc8,
            OC770_PREV_319CC8,
            ee::s10::OVERRIDE_00319CC8,
            ee::s10::f_00319cc8__generated,
            0x0031_9cc8
        ),
        (
            oc770_fake_317d70,
            OC770_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc770_fake_2e4d88,
            OC770_PREV_2E4D88,
            ee::s09::OVERRIDE_002E4D88,
            ee::s09::f_002e4d88__generated,
            0x002e_4d88
        ),
        (
            oc770_fake_416210,
            OC770_PREV_416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x0041_6210
        ),
        (
            oc770_fake_3e6448,
            OC770_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    /// Fake-callee machine for this module's fixtures: scrambled registers,
    /// the miss hooks, a game-like gp, the fakes live, and a fake that
    /// unwinds at a random call among the first `calls`.
    fn oc770_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc770_install_fakes();
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC770_CTRL, OC770_MAGIC);
        r
    }

    /// A float word for `HUDMeter_SetRect`'s inputs: edge cases and ordinary values.
    fn oc770_float(r: &mut Rng) -> u32 {
        if r.next_u32() % 3 == 0 {
            r.edge_f32()
        } else {
            r.f32(300.0)
        }
    }

    /// `HUDMeter_SetRect`: with and without an active meter (`gp-0x988`).
    #[test]
    fn hud_meter_set_rect_matches() {
        const OUT: u32 = DATA + 0x100;
        const AREA: u32 = DATA + 0x200;
        const METER: u32 = DATA + 0x300;
        check(
            ee::s05::f_0020ec18__generated,
            hud_meter_set_rect,
            300,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7701, 0);
                let gp = c.g[GP][0] as u32;
                c.pc = 0x0020_ec18;
                c.g[A0][0] = OUT as u64;
                c.g[A2][0] = AREA as u64;
                let meter = if seed % 3 == 0 { 0 } else { METER };
                c.bus.write32(gp - 0x988, meter);
                c.bus.write32(gp - 0x98c, METER);
                c.bus.write32(METER + 0x14, r.next_u32() % 5000);
                let w = oc770_float(&mut r);
                c.bus.write32(METER + 0x34, w);
                for off in [0x54f4u32, 0x54f0] {
                    let w = oc770_float(&mut r);
                    c.bus.write32(gp - off, w);
                }
                let w = oc770_float(&mut r);
                c.bus.write32(0x004d_0000 - 0x7360, w);
                for i in 0..4 {
                    let w = oc770_float(&mut r);
                    c.bus.write32(AREA + 4 * i, w);
                }
                c.bus.write32(OUT + 0x2c, r.next_u32());
            },
        );
    }

    /// `cWorldSound_RelocateEmitters`: null / already relocated blobs, 0..4
    /// records with unresolved, -1 and resolvable sound handles, and fake
    /// `InsertEmitters` calls that unwind.
    #[test]
    fn cworld_sound_relocate_emitters_matches() {
        const BLOB: u32 = DATA + 0x4000;
        const REGISTRY: u32 = DATA + 0x1000;
        const WORLD: u32 = DATA + 0x1100;
        const BANKS: u32 = DATA + 0x1200;
        const BANK: u32 = DATA + 0x1300;
        const SLOTS: u32 = DATA + 0x2000;
        const SOUNDS: u32 = DATA + 0x6000;
        check(
            ee::s08::f_002b6b50__generated,
            cworld_sound_relocate_emitters,
            400,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7702, 1);
                let gp = c.g[GP][0] as u32;
                c.pc = 0x002b_6b50;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = if seed % 13 == 0 { 0 } else { BLOB as u64 };
                c.bus.write8(BLOB, (seed % 7 == 1) as u8);
                let count = (r.next_u32() % 6) as i32 - 1;
                c.bus.write32(BLOB + 0xc, count as u32);
                c.bus.write32(gp + 0x16c8, REGISTRY);
                c.bus.write32(REGISTRY, WORLD);
                c.bus.write32(WORLD + 8, BANKS);
                for bank in 0..3u32 {
                    let p = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        BANK + bank * 0x40
                    };
                    c.bus.write32(BANKS + bank * 4, p);
                    c.bus
                        .write32(BANK + bank * 0x40 + 0x1c, SLOTS + bank * 0x100);
                    for slot in 0..4u32 {
                        let w = if r.next_u32() % 3 == 0 {
                            r.next_u32() & 0xff
                        } else {
                            (((SOUNDS + 0x40 * (bank * 4 + slot)) / 4) << 8) | 5
                        };
                        c.bus.write32(SLOTS + bank * 0x100 + slot * 4, w);
                    }
                }
                for i in 0..4u32 {
                    let rec = BLOB + 0x10 + RELOCATE_EMITTER_STRIDE * i;
                    let handle = if r.next_u32() % 4 == 0 {
                        0xffff_ffff
                    } else {
                        ((r.next_u32() % 5) << 8) | (r.next_u32() % 4)
                    };
                    c.bus.write32(rec + 0xc, handle);
                    c.bus.write32(rec + 0x10, r.next_u32() & 0xfff);
                }
            },
        );
    }

    /// `FEModule_RiderPreviewRequestChar`: every flag combination, ready and
    /// not-ready models, queued requests through the fake frame-count call.
    #[test]
    fn fe_module_rider_preview_request_char_matches() {
        const THIS: u32 = DATA + 0x2000;
        const GRMAN: u32 = DATA + 0x6000;
        const VT: u32 = DATA + 0x8000;
        check(
            ee::s03::f_0019e588__generated,
            fe_module_rider_preview_request_char,
            600,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7703, 1);
                let gp = c.g[GP][0] as u32;
                c.pc = 0x0019_e588;
                let req = r.next_u32() % 12;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = req as u64;
                c.g[A2][0] = (r.next_u32() % 3 == 0) as u64;
                let cur = if r.next_u32() % 3 == 0 { req + 1 } else { req };
                c.bus.write32(THIS, cur);
                c.bus.write32(THIS + 0x10, r.next_u32() % 2);
                c.bus.write32(THIS + 0xcb8, r.next_u32() % 2);
                let pick = |r: &mut Rng, v: [u32; 3]| v[(r.next_u32() % 3) as usize];
                c.bus
                    .write32(THIS + 0xcb4, pick(&mut r, [0, 5, 0xffff_fffd]));
                c.bus.write32(THIS + 0xcc8, pick(&mut r, [0, 1, 7]));
                c.bus.write32(THIS + 0xcc4, pick(&mut r, [0, 1, 3]));
                c.bus
                    .write32(THIS + 0xcd4, pick(&mut r, [0xffff_ffff, 0, 5]));
                c.bus.write32(gp - 0x854, GRMAN);
                c.bus.write32(GRMAN + 0x10d8, VT);
                c.bus.write16(VT + 0x390, (r.next_u32() & 0xfffc) as u16);
                c.bus.write32(VT + 0x394, fakes::FAKE);
            },
        );
    }

    /// `MemCardStateBase_InstallCallbacks`: the three sizing modes, fake
    /// callees, and unwinds at every call.
    #[test]
    fn mem_card_state_base_install_callbacks_matches() {
        const THIS: u32 = DATA + 0x2_0000;
        check(
            ee::s04::f_001d5488__generated,
            mem_card_state_base_install_callbacks,
            300,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7704, 19);
                c.pc = 0x001d_5488;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = (seed % 4) as u64;
                for off in (0x1a0..0x230).step_by(4) {
                    let w = r.next_u32();
                    c.bus.write32(THIS + off, w);
                }
            },
        );
    }

    /// `SnowFall_ctor`: both detail levels (`world+0x84 -> +0xc -> +0x7c` below
    /// and at/above 2), the float constants from gp, and unwinds at every call.
    #[test]
    fn snow_fall_ctor_matches() {
        const THIS: u32 = DATA + 0x2_0000;
        const WORLD: u32 = DATA + 0x1000;
        const GFX: u32 = DATA + 0x1100;
        const CFG: u32 = DATA + 0x1200;
        check(
            ee::s09::f_002e5920__generated,
            snow_fall_ctor,
            300,
            &|c, seed| {
                let mut r = oc770_machine(c, seed, 0x7705, 27);
                let gp = c.g[GP][0] as u32;
                c.pc = 0x002e_5920;
                c.g[A0][0] = THIS as u64;
                c.bus.write32(gp - 0x848, WORLD);
                c.bus.write32(WORLD + 0x84, GFX);
                c.bus.write32(GFX + 0xc, CFG);
                let level = [0u32, 1, 2, 3, 0xffff_ffff][(seed % 5) as usize];
                c.bus.write32(CFG + 0x7c, level);
                for off in [0x3a10u32, 0x3a0c, 0x3a08] {
                    let w = oc770_float(&mut r);
                    c.bus.write32(gp - off, w);
                }
            },
        );
    }

    /// `GraphicsMan_ctor`: fake `memset`/`memcpy`, unwinds at every call.
    #[test]
    fn graphics_man_ctor_matches() {
        const THIS: u32 = DATA + 0x3_0000;
        check(
            ee::s12::f_003691f8__generated,
            graphics_man_ctor,
            200,
            &|c, seed| {
                let _r = oc770_machine(c, seed, 0x7706, 4);
                c.pc = 0x0036_91f8;
                c.g[A0][0] = THIS as u64;
            },
        );
    }
}
