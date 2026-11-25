//! Staging module oc681 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc681`.

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
    (0x002250e0, oc681_luno_table_find_int, "cLunoTable_FindInt"),
    (0x002259a8, oc681_luno_table_read, "cLunoTable_Read"),
    (0x00226628, oc681_luno_vm_register_add, "LunoVMRegister_Add"),
    (
        0x0022cea8,
        oc681_section_man_request_event,
        "SectionMan_RequestEvent",
    ),
    (
        0x00230180,
        oc681_game_module_reset_race,
        "GameModule_ResetRace",
    ),
    (
        0x00230488,
        oc681_game_module_quit_to_load_screen,
        "GameModule_QuitToLoadScreen",
    ),
    (
        0x00230558,
        oc681_game_module_begin_replay_playback,
        "GameModule_BeginReplayPlayback",
    ),
    (
        0x00231ab8,
        oc681_game_module_can_restart_from_pause,
        "GameModule_CanRestartFromPause",
    ),
    (
        0x00235aa0,
        oc681_game_state_restart_race_enter,
        "GameStateRestartRace_Enter",
    ),
    (
        0x00236058,
        oc681_game_state_warp_to_event_enter,
        "GameStateWarpToEvent_Enter",
    ),
];

/// `LunoValue* cLunoTable_FindInt(cLunoTable* self, int key)` (0x2250e0): looks up
/// the integer `key` in the hash table: walks the chain at `key % modulus`,
/// comparing each node's index with an int-typed (3) script value, and returns
/// the node's value, or the nil value (0x4c9098) when absent.
pub fn oc681_luno_table_find_int(c: &mut Cpu) {
    guest(c, |c| {
        'b_22511c: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S3][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[A0][0] as u32));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
            c.divu(0, 19, 2);
            if c.g[V0][0] == 0u64 {
                break 'b_22511c;
            } else {
                break 'b_22511c;
            }
        }
        'b_22521c: {
            'b_225214: {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.hi[0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] == 0u64;
                c.g[S6][0] = 0x3;
                if t {
                    break 'b_225214;
                } else {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[SP][0];
                    c.g[S5][0] = 0x2;
                    c.g[S4][0] = 0x1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S6][0]);
                    'l_225148: loop {
                        'b_2251f8: {
                            'b_2251f4: {
                                'b_2251ec: {
                                    'b_225170: {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x00225150);
                                        c.g[A0][0] = c.g[SP][0];
                                        Callee {
                                            addr: 0x00226620,
                                            f: ee::s05::LunoVMRegister_AsId,
                                        }
                                        .call(c, 0x00225150)?;
                                        c.sw((c.g[V0][0] as u32), c.g[S3][0]);
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        let t = c.g[V1][0] != c.g[V0][0];

                                        if t {
                                            break 'b_225170;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                            let t = true;
                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                                            if t {
                                                break 'b_2251ec;
                                            } else {
                                                break 'b_225170;
                                            }
                                        }
                                    }
                                    'b_2251a8: {
                                        c.cycles += 2;
                                        if c.g[V0][0] != c.g[S5][0] {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_2251a8;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[S4][0] {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_2251a8;
                                            } else {
                                                'b_2251a0: {
                                                    c.cycles += 7;
                                                    c.lwc1(1, (c.g[SP][0] as u32));
                                                    c.cvt_s_w(1, 1);
                                                    c.lwc1(0, (c.g[S1][0] as u32));
                                                    c.c_eq_s(0, 1);
                                                    let t = c.fcc();
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_2251a0;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0u64;
                                                        break 'b_2251a0;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[S0][0] = c.g[V0][0];
                                                if t {
                                                    break 'b_2251f4;
                                                } else {
                                                    break 'b_2251a8;
                                                }
                                            }
                                        }
                                    }
                                    'b_2251d0: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[S4][0];

                                        if t {
                                            break 'b_2251d0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            let t = c.g[V0][0] != c.g[S5][0];

                                            if t {
                                                break 'b_2251d0;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, (c.g[S2][0] as u32));
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(3, 1);
                                                let t = true;
                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                if t {
                                                    break 'b_2251ec;
                                                } else {
                                                    break 'b_2251d0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[S6][0];
                                    c.g[S0][0] = 0u64;
                                    if t {
                                        break 'b_2251f4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        let t = c.g[V0][0] != c.g[S4][0];
                                        c.g[A0][0] = c.g[SP][0];
                                        if t {
                                            break 'b_2251f8;
                                        } else {
                                            c.cycles += 2;
                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                            break 'b_2251ec;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                                c.g[S0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                break 'b_2251f4;
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.g[SP][0];
                            break 'b_2251f8;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00225200);
                        c.g[A1][0] = 0x2;
                        Callee {
                            addr: 0x00225b90,
                            f: ee::s05::LunoVMRegister_dtor,
                        }
                        .call(c, 0x00225200)?;
                        let t = c.g[S0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_22521c;
                        } else {
                            c.cycles += 3;
                            c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            if c.g[S1][0] != 0u64 {
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S6][0]);
                                if event_due(c) {
                                    deliver_event_here(c, 0x00225148);
                                }
                                continue 'l_225148;
                            } else {
                                break 'b_225214;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x4d0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
            break 'b_22521c;
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void cLunoTable_Read(cLunoTable* self, Stream* s, void* base)` (0x2259a8):
/// deserialises a replay-frame table: destroys the old node array (virtual
/// deleters, then free), reads the 8-byte header and the entry count from the
/// stream, allocates a new node array and `set`s every (key, value) read back.
pub fn oc681_luno_table_read(c: &mut Cpu) {
    guest(c, |c| {
        'b_225a40: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = 0x480000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S2][0] = c.g[A1][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S3][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            let t = c.g[V1][0] == 0u64;
            c.g[S5][0] = c.g[A2][0];
            if t {
                break 'b_225a40;
            } else {
                'b_225a34: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    if c.g[V1][0] == c.g[S0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                        break 'b_225a34;
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                        'l_225a08: loop {
                            c.cycles += 10;
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00225a20);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00225a20)?;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                            let t = c.g[V1][0] != c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00225a08);
                                }
                                continue 'l_225a08;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                                break 'b_225a34;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x00225a40);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x00317e98, 0x00225a40)?;
                break 'b_225a40;
            }
        }
        'b_225acc: {
            c.cycles += 22;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
            c.g[A2][0] = 0x8;
            c.g[A1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00225a5c);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00225a5c)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6790));
            c.g[A2][0] = 0xffffffffa0000000;
            c.g[A3][0] = 0u64;
            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[S1][0] as u32) << 5);
            c.g[RA][0] = sx(0x00225a84);
            c.g[A0][0] = c.g[A0][0] | 0x10;
            call_ptr(c, 0x00317e30, 0x00225a84)?;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32), c.g[S1][0]);
            c.g[V0][0] = 0xffffffffffffffff;
            let t = c.g[S0][0] == c.g[V0][0];
            c.g[V1][0] = c.g[A0][0];
            if t {
                break 'b_225acc;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x480000;
                c.g[A1][0] = 0xffffffffffffffff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6770));
                'l_225aa8: loop {
                    c.cycles += 9;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                    let t = c.g[S0][0] != c.g[A1][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00225aa8);
                        }
                        continue 'l_225aa8;
                    } else {
                        break 'b_225acc;
                    }
                }
            }
        }
        'b_225b68: {
            c.cycles += 12;
            c.sw((c.g[S6][0] as u32), c.g[A0][0]);
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = 0x4;
            c.g[S1][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00225af0);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00225af0)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_225b68;
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.g[SP][0];
                'l_225b00: loop {
                    c.cycles += 26;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[SP][0] as u32), 0u64);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00225b18);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00225ca0,
                        f: ee::s05::LunoVMRegister_Read,
                    }
                    .call(c, 0x00225b18)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00225b30);
                    c.g[A2][0] = c.g[S5][0];
                    Callee {
                        addr: 0x00225ca0,
                        f: ee::s05::LunoVMRegister_Read,
                    }
                    .call(c, 0x00225b30)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00225b40);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00225338,
                        f: ee::s05::cLunoTable_Set,
                    }
                    .call(c, 0x00225b40)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00225b4c);
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x00225b90,
                        f: ee::s05::LunoVMRegister_dtor,
                    }
                    .call(c, 0x00225b4c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00225b58);
                    c.g[A1][0] = 0x2;
                    Callee {
                        addr: 0x00225b90,
                        f: ee::s05::LunoVMRegister_dtor,
                    }
                    .call(c, 0x00225b58)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00225b00);
                        }
                        continue 'l_225b00;
                    } else {
                        break 'b_225b68;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `LunoVMRegister* LunoVMRegister_Add(out, lhs, rhs)` (0x226628): adds two
/// script values. Same types add (int type 1, float 2, index 3); float + int and
/// int + float mix; any other pair gives the nil value. Returns `out`.
pub fn oc681_luno_vm_register_add(c: &mut Cpu) {
    guest(c, |c| {
        'b_226748: {
            'b_22672c: {
                'b_22671c: {
                    'b_2266ec: {
                        'b_2266e8: {
                            'b_2266d8: {
                                'b_2266ac: {
                                    c.cycles += 9;
                                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                    let t = c.g[A3][0] != c.g[V1][0];
                                    c.g[S1][0] = c.g[A0][0];
                                    if t {
                                        break 'b_2266ac;
                                    } else {
                                        'b_226668: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_226668;
                                            } else {
                                                c.cycles += 4;
                                                c.lwc1(20, (c.g[A1][0] as u32));
                                                c.lwc1(0, (c.g[A2][0] as u32));
                                                let t = true;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x8),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_2266d8;
                                                } else {
                                                    break 'b_226668;
                                                }
                                            }
                                        }
                                        'b_226684: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_226684;
                                            } else {
                                                c.cycles += 5;
                                                c.g[S0][0] = c.lw((c.g[A1][0] as u32));
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                                let t = true;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x8),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_22671c;
                                                } else {
                                                    break 'b_226684;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x4d0000;
                                        if t {
                                            break 'b_22672c;
                                        } else {
                                            c.cycles += 8;
                                            c.g[S0][0] = c.lw((c.g[A1][0] as u32));
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                            c.g[RA][0] = sx(0x002266a4);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            Callee {
                                                addr: 0x00226620,
                                                f: ee::s05::LunoVMRegister_AsId,
                                            }
                                            .call(c, 0x002266a4)?;
                                            let t = true;
                                            c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                                            if t {
                                                break 'b_226748;
                                            } else {
                                                break 'b_2266ac;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                    break 'b_2266e8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    if c.g[A3][0] != c.g[V0][0] {
                                        c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                        break 'b_2266ec;
                                    } else {
                                        c.cycles += 5;
                                        c.lwc1(0, (c.g[A2][0] as u32));
                                        c.cvt_s_w(0, 0);
                                        c.lwc1(20, (c.g[A1][0] as u32));
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[A0][0] = c.g[S1][0];
                                        break 'b_2266d8;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x002266e0);
                            c.add_s(20, 20, 0);
                            Callee {
                                addr: 0x00226618,
                                f: ee::s05::LunoVMRegister_AsFloat,
                            }
                            .call(c, 0x002266e0)?;
                            let t = true;
                            c.swc1(20, (c.g[V0][0] as u32));
                            if t {
                                break 'b_226748;
                            } else {
                                break 'b_2266e8;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0x1;
                        break 'b_2266ec;
                    }
                    c.cycles += 2;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4d0000;
                    if t {
                        break 'b_22672c;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = 0x2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x4d0000;
                        if t {
                            break 'b_22672c;
                        } else {
                            c.cycles += 6;
                            c.lwc1(0, (c.g[A2][0] as u32));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[S0][0] = c.lw((c.g[A1][0] as u32));
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                            break 'b_22671c;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x00226724);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                Callee {
                    addr: 0x00226610,
                    f: ee::s05::LunoVMRegister_AsInt,
                }
                .call(c, 0x00226724)?;
                let t = true;
                c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                if t {
                    break 'b_226748;
                } else {
                    break 'b_22672c;
                }
            }
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
            c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.ldr(3, (c.g[A2][0] as u32));
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x7));
            c.sdr(3, (c.g[S1][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            break 'b_226748;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void SectionMan_RequestEvent(cSectionMan* self, int idx, int mode)` (0x22cea8):
/// switches to the event location `idx`: unloads every BE section it does not
/// use (43-48 stay), records the location, loads its sections, the sky section
/// (49 when the location has none) and the transport section, then starts the
/// event switch for `mode` (7 maps through the location-mode table).
pub fn oc681_section_man_request_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_22cf04: {
            'b_22cf00: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = 0x7;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S3][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = c.g[A2][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_22cf00;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2820));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    let t = true;
                    c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if t {
                        break 'b_22cf04;
                    } else {
                        break 'b_22cf00;
                    }
                }
            }
            c.cycles += 1;
            c.g[FP][0] = c.g[A1][0];
            break 'b_22cf04;
        }
        c.cycles += 15;
        c.g[V1][0] = 0x28;
        c.g[V0][0] = 0x440000;
        c.mult(0, 3, 19, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2488));
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[S6][0] = 0x31;
        c.g[A1][0] = 0u64;
        c.g[S1][0] = 0x28;
        c.g[S7][0] = 0x4;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = ((c.g[A0][0] as i64) < 0i64) as u64;
        if c.g[V1][0] == 0 {
            c.g[S6][0] = c.g[A0][0];
        }
        c.g[A3][0] = 0x1;
        'l_22cf40: loop {
            c.cycles += 4;
            c.g[A2][0] = 0u64;
            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c));
            c.g[T1][0] = c.g[A1][0] ^ 0x2b;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            'l_22cf50: loop {
                'b_22cf80: {
                    c.cycles += 6;
                    c.mult(0, 4, 19, 17);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                    if t {
                        break 'b_22cf80;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        if c.g[A1][0] != c.g[V1][0] {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x0022cf50);
                            }
                            continue 'l_22cf50;
                        } else {
                            c.cycles += 1;
                            c.g[A3][0] = 0u64;
                            break 'b_22cf80;
                        }
                    }
                }
                'b_22cfa0: {
                    c.cycles += 5;
                    c.g[V0][0] = (c.g[S7][0] < c.g[T0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[T1][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[A3][0] == 0u64 {
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22cfa0;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0022cf9c);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0022d598,
                            f: ee::s05::SectionMan_RequestUnload,
                        }
                        .call(c, 0x0022cf9c)?;
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22cfa0;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 50i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A3][0] = 0x1;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0022cf40);
                    }
                    continue 'l_22cf40;
                } else {
                    'b_22d008: {
                        c.cycles += 11;
                        c.g[V0][0] = 0x28;
                        c.g[V1][0] = 0x440000;
                        c.mult(0, 5, 19, 2);
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2488));
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x1bc), c.g[S3][0]);
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x1c0), 0u64);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S1][0] = 0u64;
                        if t {
                            break 'b_22d008;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[S4][0] = c.g[A0][0];
                            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            'l_22cfe8: loop {
                                c.cycles += 8;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[RA][0] = sx(0x0022cff8);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                Callee {
                                    addr: 0x0022d3d8,
                                    f: ee::s05::SectionMan_RequestLoad,
                                }
                                .call(c, 0x0022cff8)?;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0022cfe8);
                                    }
                                    continue 'l_22cfe8;
                                } else {
                                    break 'b_22d008;
                                }
                            }
                        }
                    }
                    'b_22d020: {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[S6][0] ^ 0x31;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[S6][0];
                        if t {
                            break 'b_22d020;
                        } else {
                            c.cycles += 3;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x1a4), 0u64);
                            c.g[RA][0] = sx(0x0022d020);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x0022d3d8,
                                f: ee::s05::SectionMan_RequestLoad,
                            }
                            .call(c, 0x0022d020)?;
                            break 'b_22d020;
                        }
                    }
                    'b_22d04c: {
                        c.cycles += 8;
                        c.g[V0][0] = 0x28;
                        c.g[A0][0] = 0x440000;
                        c.mult(0, 2, 19, 2);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2488));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        let t = (c.g[A1][0] as i64) < 0;
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_22d04c;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0022d048);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x0022d3d8,
                                f: ee::s05::SectionMan_RequestLoad,
                            }
                            .call(c, 0x0022d048)?;
                            c.g[A0][0] = c.g[S2][0];
                            break 'b_22d04c;
                        }
                    }
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x0022d054);
                    c.g[A1][0] = c.g[FP][0];
                    Callee {
                        addr: 0x0022d640,
                        f: ee::s05::SectionMan_StartEventSwitch,
                    }
                    .call(c, 0x0022d054)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
            }
        }
    });
}

/// `void GameModule_ResetRace(cGame* self)` (0x230180): resets the race for a
/// restart (playback, buckets, AI, avalanche, views, world, replay, scripts,
/// camera, crowds, particles, HUD state event 7, audio) and sets game state 6.
pub fn oc681_game_module_reset_race(c: &mut Cpu) {
    guest(c, |c| {
        'b_230274: {
            c.cycles += 55;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[RA][0] = sx(0x002301a0);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            call_ptr(c, 0x0026ca90, 0x002301a0)?;
            c.g[RA][0] = sx(0x002301a8);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            call_ptr(c, 0x00309030, 0x002301a8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002301b4);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x00354c98, 0x002301b4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002301c0);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x00355118, 0x002301c0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002301cc);
            c.g[A1][0] = 0x8;
            call_ptr(c, 0x00355118, 0x002301cc)?;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xcc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002301e8);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x002301e8)?;
            c.g[RA][0] = sx(0x002301f0);
            call_ptr(c, 0x002d9b40, 0x002301f0)?;
            c.g[RA][0] = sx(0x002301f8);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            call_ptr(c, 0x0022e7c8, 0x002301f8)?;
            c.g[RA][0] = sx(0x00230200);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x003a6800, 0x00230200)?;
            c.g[RA][0] = sx(0x00230208);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f228, 0x00230208)?;
            c.g[RA][0] = sx(0x00230210);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            call_ptr(c, 0x002789e0, 0x00230210)?;
            c.g[RA][0] = sx(0x00230218);
            call_ptr(c, 0x0015db58, 0x00230218)?;
            c.g[RA][0] = sx(0x00230220);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            call_ptr(c, 0x00229498, 0x00230220)?;
            c.g[RA][0] = sx(0x00230228);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            call_ptr(c, 0x00343bc0, 0x00230228)?;
            c.g[RA][0] = sx(0x00230230);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x00357b38, 0x00230230)?;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ab0));
            c.g[RA][0] = sx(0x00230244);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x00317670, 0x00230244)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00230250);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039f9d8, 0x00230250)?;
            c.g[A3][0] = c.g[V0][0];
            let t = c.g[A3][0] == 0u64;
            c.g[A1][0] = 0x7;
            if t {
                break 'b_230274;
            } else {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00230274);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00230274)?;
                break 'b_230274;
            }
        }
        c.cycles += 13;
        c.g[RA][0] = sx(0x0023027c);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x00309f18, 0x0023027c)?;
        c.g[RA][0] = sx(0x00230284);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        call_ptr(c, 0x00308c60, 0x00230284)?;
        c.g[RA][0] = sx(0x0023028c);
        call_ptr(c, 0x0028b180, 0x0023028c)?;
        c.g[RA][0] = sx(0x00230294);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00287108, 0x00230294)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameModule_QuitToLoadScreen(cGame* self, int toFrontEnd)` (0x230488):
/// requests state 6, quits the comm race and voip when present, then makes a
/// `cPreGameLoadScreen` (`toFrontEnd != 0`) or `cPreFELoadScreen` the next module.
pub fn oc681_game_module_quit_to_load_screen(c: &mut Cpu) {
    guest(c, |c| {
        'b_2304d4: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x002304a8);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x002706f0, 0x002304a8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x6;
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x002304bc);
            c.g[A3][0] = 0u64;
            Callee {
                addr: 0x00231250,
                f: ee::s06::GameModule_RequestState,
            }
            .call(c, 0x002304bc)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                break 'b_2304d4;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002304d0);
                Callee {
                    addr: 0x00258ae0,
                    f: ee::s06::cGameComm_QuitRace,
                }
                .call(c, 0x002304d0)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                break 'b_2304d4;
            }
        }
        'b_2304e4: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2304e4;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002304e4);
                call_ptr(c, 0x00266df8, 0x002304e4)?;
                break 'b_2304e4;
            }
        }
        'b_230548: {
            'b_23051c: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;
                c.g[A2][0] = 0x100;
                if t {
                    break 'b_23051c;
                } else {
                    c.cycles += 12;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4890));
                    c.g[RA][0] = sx(0x00230500);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x00230500)?;
                    c.g[RA][0] = sx(0x00230508);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x00232720,
                        f: ee::s06::cPreGameLoadScreen_ctor,
                    }
                    .call(c, 0x00230508)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x00230514);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00316ef8, 0x00230514)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_230548;
                    } else {
                        break 'b_23051c;
                    }
                }
            }
            c.cycles += 11;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4878));
            c.g[RA][0] = sx(0x00230530);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x00230530)?;
            c.g[RA][0] = sx(0x00230538);
            c.g[A0][0] = c.g[V0][0];
            Callee {
                addr: 0x00232738,
                f: ee::s06::cPreFELoadScreen_ctor,
            }
            .call(c, 0x00230538)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[RA][0] = sx(0x00230544);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x00316ef8, 0x00230544)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_230548;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameModule_BeginReplayPlayback(cGame* self)` (0x230558): pushes pause
/// state 8 on the pause stack, copies the pause flags, starts the replay
/// and activates the menu system.
pub fn oc681_game_module_begin_replay_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = 0x530000;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
        c.g[A1][0] = 0x440000;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2910));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A2][0] = 0x8;
        c.g[A1][0] = 0u64;
        c.sw((c.g[V0][0] as u32), c.g[A2][0]);
        c.g[A3][0] = 0u64;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A0][0]);
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[RA][0] = sx(0x002305ac);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[T0][0]);
        call_ptr(c, 0x0026f8a0, 0x002305ac)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
        c.g[RA][0] = sx(0x002305b8);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002cc0b8, 0x002305b8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `bool GameModule_CanRestartFromPause(cGame* self)` (0x231ab8): false while the
/// pending state is 5, 14, 1 or 10; otherwise the script engine's "pause menu
/// allowed" answer combined with the same test on the current state.
pub fn oc681_game_module_can_restart_from_pause(c: &mut Cpu) {
    guest(c, |c| {
        'b_231b3c: {
            'b_231b04: {
                'b_231b00: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x210));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        break 'b_231b00;
                    } else {
                        'b_231af8: {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0xe;
                            if t {
                                break 'b_231af8;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_231af8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0xa;
                                    if t {
                                        break 'b_231af8;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                        if t {
                                            break 'b_231b04;
                                        } else {
                                            break 'b_231af8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_231b3c;
                        } else {
                            break 'b_231b00;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                break 'b_231b04;
            }
            'b_231b38: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00231b0c);
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x00278da0, 0x00231b0c)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x5;
                if t {
                    break 'b_231b38;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = 0xe;
                    if t {
                        break 'b_231b38;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A0][0] == c.g[V0][0];
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'b_231b38;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A0][0] == c.g[V1][0];
                            c.g[V0][0] = c.g[A0][0] ^ 0xa;
                            if t {
                                break 'b_231b38;
                            } else {
                                c.cycles += 2;
                                c.g[S0][0] = c.g[V1][0];
                                if c.g[V0][0] == 0 {
                                    c.g[S0][0] = 0u64;
                                }
                                break 'b_231b38;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            break 'b_231b3c;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateRestartRace_Enter(GameState* self)` (0x235aa0): resets the game
/// mode and the race, then either (mode 5/6 or a flagged race) sets state 3, or
/// re-adds the computer riders and restarts and sets state 0.
pub fn oc681_game_state_restart_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_235c98: {
            'b_235b64: {
                'b_235b5c: {
                    'b_235b58: {
                        c.cycles += 40;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S7][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[RA][0] = sx(0x00235ad8);
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014de68, 0x00235ad8)?;
                        c.g[RA][0] = sx(0x00235ae0);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                        call_ptr(c, 0x0030b7f8, 0x00235ae0)?;
                        c.g[RA][0] = sx(0x00235ae8);
                        call_ptr(c, 0x0014dc80, 0x00235ae8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00235af4);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x00235af4)?;
                        c.g[RA][0] = sx(0x00235afc);
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dc80, 0x00235afc)?;
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x00235b08);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00235b08)?;
                        c.g[S5][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[RA][0] = sx(0x00235b1c);
                        c.g[A0][0] = c.g[S6][0];
                        Callee {
                            addr: 0x002382d8,
                            f: ee::s06::GameModeMan_ResetMode,
                        }
                        .call(c, 0x00235b1c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[RA][0] = sx(0x00235b28);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        Callee {
                            addr: 0x00230180,
                            f: ee::s06::GameModule_ResetRace,
                        }
                        .call(c, 0x00235b28)?;
                        c.g[V0][0] = 0x530000;
                        c.g[V1][0] = 0x5;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                        c.g[A0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == c.g[V1][0];
                        c.g[FP][0] = c.g[V0][0];
                        if t {
                            break 'b_235b58;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x6;
                            let t = c.g[A0][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_235b5c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x49));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                if t {
                                    break 'b_235b64;
                                } else {
                                    break 'b_235b58;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x3;
                    break 'b_235b5c;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_235c98;
                } else {
                    break 'b_235b64;
                }
            }
            'b_235be4: {
                c.cycles += 16;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[S4][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                c.g[RA][0] = sx(0x00235b88);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x00144d98, 0x00235b88)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00235b9c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00235b9c)?;
                let t = (c.g[S3][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
                if t {
                    break 'b_235be4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    'l_235bb0: loop {
                        c.cycles += 13;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00235bc0);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00147338, 0x00235bc0)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00235bd8);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x001473d0, 0x00235bd8)?;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00235bb0);
                            }
                            continue 'l_235bb0;
                        } else {
                            break 'b_235be4;
                        }
                    }
                }
            }
            'b_235c28: {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00235bf8);
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00235bf8)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[RA][0] = sx(0x00235c08);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012ab20, 0x00235c08)?;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x00235c24);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012ac48, 0x00235c24)?;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                }
            }
            'b_235c50: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_235c50;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x00235c4c);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0027a860, 0x00235c4c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    break 'b_235c50;
                }
            }
            c.cycles += 18;
            c.g[RA][0] = sx(0x00235c58);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002790a0, 0x00235c58)?;
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x98));
            c.g[V0][0] = 0x2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = 0x3;
            c.g[RA][0] = sx(0x00235c70);
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.g[V0][0];
            }
            call_ptr(c, 0x0027aaf8, 0x00235c70)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[RA][0] = sx(0x00235c7c);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00278f38, 0x00235c7c)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[RA][0] = sx(0x00235c90);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
            call_ptr(c, 0x001f36c0, 0x00235c90)?;
            c.sw((c.g[S7][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S7][0] as u32).wrapping_add(0x14), 0u64);
            break 'b_235c98;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateWarpToEvent_Enter(GameState* self)` (0x236058): fails the active
/// mission, starts game mode 0xc, resets the race, puts the rider at the start
/// point and starts a white fade transition.
pub fn oc681_game_state_warp_to_event_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 79;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[RA][0] = sx(0x0023607c);
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
        call_ptr(c, 0x0030b7f8, 0x0023607c)?;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00236090);
        c.g[A1][0] = 0xc;
        Callee {
            addr: 0x00238160,
            f: ee::s06::GameModeMan_StartMode,
        }
        .call(c, 0x00236090)?;
        c.g[RA][0] = sx(0x00236098);
        call_ptr(c, 0x0014dc80, 0x00236098)?;
        c.g[A1][0] = 0xa;
        c.g[RA][0] = sx(0x002360a4);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014dd58, 0x002360a4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002360b4);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        Callee {
            addr: 0x00230180,
            f: ee::s06::GameModule_ResetRace,
        }
        .call(c, 0x002360b4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A2][0] = 0x2;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x002360d4);
        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0011de60, 0x002360d4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x002360f0);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0011df18, 0x002360f0)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x00236100);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        Callee {
            addr: 0x00230698,
            f: ee::s06::GameModule_GetTransitionMan,
        }
        .call(c, 0x00236100)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023610c);
        c.g[A0][0] = 0x14;
        call_ptr(c, 0x002e4ce8, 0x0023610c)?;
        c.g[S0][0] = c.g[V0][0];
        c.swc1(20, (c.g[SP][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x00236128);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
        call_ptr(c, 0x002e4d70, 0x00236128)?;
        c.g[V1][0] = 0x490000;
        c.mtc1(0, 12);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7f40));
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[S0][0] as u32), c.g[V1][0]);
        c.g[A3][0] = c.g[S0][0];
        c.mov_s(14, 20);
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[SP][0] as u32));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0xb));
        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x13));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.mov_s(13, 12);
        c.g[T0][0] = 0u64;
        c.g[RA][0] = sx(0x0023617c);
        c.g[T1][0] = 0u64;
        call_ptr(c, 0x002e4370, 0x0023617c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, fake, machine};
    use crate::difftest::{DATA, GAME_GP};
    use ssx3_ee_rt::funcs::{register_functions, FnEntry};
    use std::ptr::addr_of_mut;

    /// Set in the child process that runs one of these tests: the function
    /// table is replaced by [`OC681_STUBS`] (every other call is a logged
    /// fake) and the direct callees' override slots hold fakes. Both are
    /// process-wide, so they never run in the shared test process.
    const OC681_CHILD_ENV: &str = "SSX3_OC681_CHILD";

    /// Fixture area, clear of the fakes' pointer results (`DATA..DATA+0xfffff`)
    /// and their log (`DATA+0x100000`).
    const OC681_BASE: u32 = DATA + 0x20_0000;
    /// Return values for the two stubbed callees.
    const OC681_RET_FIND: u32 = OC681_BASE + 0x8f00;
    const OC681_RET_QUERY: u32 = OC681_BASE + 0x8f04;
    /// `SSXApp`, its game object, the AI object and their vtables.
    const OC681_APP: u32 = OC681_BASE + 0x1000;
    const OC681_GAME: u32 = OC681_BASE + 0x2000;
    const OC681_AI: u32 = OC681_BASE + 0x3000;
    const OC681_AI_VT: u32 = OC681_BASE + 0x4000;
    const OC681_MODE_MAN: u32 = OC681_BASE + 0x5000;
    const OC681_HELD: u32 = OC681_BASE + 0x8000;
    const OC681_HELD_VT: u32 = OC681_BASE + 0x8100;
    /// The new-race record (`cBENewRaceInterface_newRace`).
    const OC681_NEW_RACE: u32 = 0x0053_5bc8;

    /// Runs `run` in a child process (this test, by name, with the env set).
    macro_rules! oc681_child {
        ($test:ident, $run:path) => {{
            if std::env::var_os(OC681_CHILD_ENV).is_some() {
                $run();
            } else {
                let name = format!(
                    "{}::{}",
                    module_path!().split_once("::").map_or("", |p| p.1),
                    stringify!($test)
                );
                oc681_spawn(&name);
            }
        }};
    }

    fn oc681_spawn(name: &str) {
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name, "--exact", "--test-threads=1", "--nocapture"])
            .env(OC681_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// A stub that returns the word the fixture left at `at`, after logging.
    fn oc681_stub_value(c: &mut Cpu, target: u32, at: u32) {
        let ret = c.g[RA][0] as u32;
        fake(c, target, ret);
        if c.pc == ret {
            c.g[V0][0] = c.bus.read32(at) as u64;
        }
    }
    fn oc681_stub_find(c: &mut Cpu) {
        oc681_stub_value(c, 0x0039_f9d8, OC681_RET_FIND)
    }
    fn oc681_stub_query(c: &mut Cpu) {
        oc681_stub_value(c, 0x0027_8da0, OC681_RET_QUERY)
    }
    static OC681_STUBS: [FnEntry; 2] = [
        FnEntry {
            start: 0x0027_8da0,
            end: 0x0027_8da4,
            f: oc681_stub_query,
        },
        FnEntry {
            start: 0x0039_f9d8,
            end: 0x0039_f9dc,
            f: oc681_stub_find,
        },
    ];

    /// In the child only: every call but the two stubs is a logged fake.
    /// (`fresh()` registers the real table again for each machine.)
    fn oc681_use_stubs() {
        if std::env::var_os(OC681_CHILD_ENV).is_some() {
            register_functions(&OC681_STUBS);
        }
    }

    macro_rules! oc681_slot_fake {
        ($name:ident, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                fakes::fake_call(c, $addr)
            }
        };
    }
    fn oc681_hook(slot: *mut fn(&mut Cpu), f: fn(&mut Cpu)) {
        fakes::hook(slot, f, fakes::prev_cell(f));
    }
    oc681_slot_fake!(oc681_fk_as_id, 0x0022_6620);
    oc681_slot_fake!(oc681_fk_as_float, 0x0022_6618);
    oc681_slot_fake!(oc681_fk_as_int, 0x0022_6610);
    oc681_slot_fake!(oc681_fk_dtor, 0x0022_5b90);
    oc681_slot_fake!(oc681_fk_read, 0x0022_5ca0);
    oc681_slot_fake!(oc681_fk_set, 0x0022_5338);
    oc681_slot_fake!(oc681_fk_unload, 0x0022_d598);
    oc681_slot_fake!(oc681_fk_load, 0x0022_d3d8);
    oc681_slot_fake!(oc681_fk_event_switch, 0x0022_d640);
    oc681_slot_fake!(oc681_fk_request_state, 0x0023_1250);
    oc681_slot_fake!(oc681_fk_pre_game, 0x0023_2720);
    oc681_slot_fake!(oc681_fk_pre_fe, 0x0023_2738);
    oc681_slot_fake!(oc681_fk_reset_mode, 0x0023_82d8);
    oc681_slot_fake!(oc681_fk_start_mode, 0x0023_8160);
    oc681_slot_fake!(oc681_fk_transition_man, 0x0023_0698);

    fn oc681_hook_luno() {
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226620), oc681_fk_as_id);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226618), oc681_fk_as_float);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226610), oc681_fk_as_int);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225B90), oc681_fk_dtor);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225CA0), oc681_fk_read);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225338), oc681_fk_set);
    }

    /// Writes a gcc vtable slot `{delta, fn}` at `vt + off`.
    fn oc681_vslot(c: &mut Cpu, vt: u32, off: u32, delta: u32, f: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, f);
    }

    /// The objects every game-module function reaches: `SSXApp` at
    /// `gp-0x848` with its game (+0x84), mode manager (+0xc0), and the game's
    /// AI object (+0xc) whose vtable slot 0x28 is a fake method.
    fn oc681_world(c: &mut Cpu, flags: u32) {
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x848), OC681_APP);
        c.bus.write32(OC681_APP + 0x84, OC681_GAME);
        c.bus.write32(OC681_APP + 0xc0, OC681_MODE_MAN);
        c.bus.write32(OC681_GAME + 0xc, OC681_AI);
        c.bus.write32(OC681_AI + 0xcc, OC681_AI_VT);
        oc681_vslot(c, OC681_AI_VT, 0x28, 0, fakes::FAKE);
        c.bus.write32(OC681_HELD + 8, OC681_HELD_VT);
        oc681_vslot(c, OC681_HELD_VT, 0xc0, 0, fakes::FAKE + 0x10);
        c.bus
            .write32(OC681_RET_FIND, if flags & 1 == 0 { 0 } else { OC681_HELD });
        c.bus.write32(OC681_RET_QUERY, flags >> 1 & 1);
    }

    /// `cLunoTable_FindInt(self, key)`: finds the node whose index equals the
    /// integer `key` in the hash chain at `key % modulus` (nil when none).
    /// Fixture: random chains of 0-3 nodes of every index type; the key
    /// register on the stack is reloaded as integer or float. `AsId` and the
    /// destructor are fakes.
    fn oc681_luno_table_find_int_run() {
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00226620), oc681_fk_as_id);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_00225B90), oc681_fk_dtor);
        covered(
            ee::s05::f_002250e0__generated,
            oc681_luno_table_find_int,
            240,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_50e0, 4);
                c.pc = 0x0022_50e0;
                let (this, table, nodes) =
                    (OC681_BASE + 0x100, OC681_BASE + 0x200, OC681_BASE + 0x1000);
                let key = r.next_u32() % 7;
                let modulus = 1 + r.next_u32() % 4;
                c.bus.write32(this, table);
                c.bus.write32(table + 0xc, modulus);
                c.bus.write32(table + 0x10, nodes);
                for i in 0..6u32 {
                    let n = nodes + 0x20 * i;
                    let idx_type = r.next_u32() % 5;
                    let idx_word = match r.next_u32() % 3 {
                        0 => key,
                        1 => (key as f32).to_bits(),
                        _ => r.next_u32(),
                    };
                    c.bus.write32(n, idx_word);
                    c.bus.write32(n + 8, idx_type);
                    c.bus.write32(n + 0xc, r.next_u32());
                    // The head nodes of the chains mostly hold values.
                    let val_type = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    };
                    c.bus.write32(n + 0x14, val_type);
                    let next = if i < 5 && r.next_u32() % 2 == 0 {
                        nodes + 0x20 * (i + 1 + r.next_u32() % (5 - i))
                    } else {
                        0
                    };
                    c.bus
                        .write32(n + 0x18, if next > nodes + 0x20 * 5 { 0 } else { next });
                }
                let stack_key = match r.next_u32() % 3 {
                    0 => key,
                    1 => (key as f32).to_bits(),
                    _ => r.next_u32(),
                };
                c.bus.write32(crate::difftest::SP - 0x90, stack_key);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = key as u64;
            },
        );
    }

    #[test]
    fn oc681_luno_table_find_int_matches() {
        oc681_child!(
            oc681_luno_table_find_int_matches,
            oc681_luno_table_find_int_run
        );
    }

    /// `cLunoTable_Read(self, stream, base)`: destroys the old nodes, reads
    /// the 8-byte table header and the entry count through the stream's
    /// vtable, allocates a node array and sets every (key, value) read back.
    /// Fixture: 0-2 old nodes with virtual deleters (or none), moduli -1..2,
    /// entry counts -1..2; the stream, allocator, register reader and table
    /// set are fakes.
    fn oc681_luno_table_read_run() {
        oc681_hook_luno();
        covered(
            ee::s05::f_002259a8__generated,
            oc681_luno_table_read,
            240,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_59a8, 14);
                c.pc = 0x0022_59a8;
                let (this, table, old, node_vt) = (
                    OC681_BASE + 0x100,
                    OC681_BASE + 0x200,
                    OC681_BASE + 0x1000,
                    OC681_BASE + 0x3000,
                );
                let (stream, stream_vt) = (OC681_BASE + 0x400, OC681_BASE + 0x500);
                c.bus.write32(this, table);
                let have_old = r.next_u32() % 4 != 0;
                c.bus.write32(table + 0x10, if have_old { old } else { 0 });
                c.bus.write32(old - 0x10, r.next_u32() % 3);
                for i in 0..3u32 {
                    c.bus.write32(old + 0x20 * i + 0x1c, node_vt);
                }
                oc681_vslot(
                    c,
                    node_vt,
                    8,
                    (r.next_u32() & 0x7c) & !3,
                    fakes::FAKE + 0x20,
                );
                c.bus.write32(
                    table + 0xc,
                    [u32::MAX, 0, 1, 2][(r.next_u32() % 4) as usize],
                );
                c.bus.write32(stream, stream_vt);
                oc681_vslot(
                    c,
                    stream_vt,
                    0x10,
                    (r.next_u32() & 0x3c) & !3,
                    fakes::FAKE + 0x10,
                );
                c.bus.write32(
                    crate::difftest::SP - 0x90,
                    [u32::MAX, 0, 1, 2][(r.next_u32() % 4) as usize],
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    #[test]
    fn oc681_luno_table_read_matches() {
        oc681_child!(oc681_luno_table_read_matches, oc681_luno_table_read_run);
    }

    /// `LunoVMRegister_Add(out, lhs, rhs)`: adds two script values: same
    /// types add (int, float, index; others give nil), float + int and
    /// int + float mix, everything else is nil. Fixture: every pair of types
    /// 0..4 with integer and float words; the `As*` accessors are fakes.
    fn oc681_luno_vm_register_add_run() {
        oc681_hook_luno();
        covered(
            ee::s05::f_00226628__generated,
            oc681_luno_vm_register_add,
            250,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_6628, 2);
                c.pc = 0x0022_6628;
                let (out, lhs, rhs) = (OC681_BASE + 0x100, OC681_BASE + 0x200, OC681_BASE + 0x300);
                let lt = seed % 5;
                let rt = if seed % 2 == 0 { lt } else { (seed / 5) % 5 };
                let word = |r: &mut crate::difftest::Rng| match r.next_u32() % 3 {
                    0 => r.next_u32() % 100,
                    1 => r.edge_f32(),
                    _ => r.next_u32(),
                };
                let (lw, rw) = (word(&mut r), word(&mut r));
                c.bus.write32(lhs, lw);
                c.bus.write32(lhs + 8, lt);
                c.bus.write32(rhs, rw);
                c.bus.write32(rhs + 8, rt);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(lhs);
                c.g[A2][0] = sx(rhs);
            },
        );
    }

    #[test]
    fn oc681_luno_vm_register_add_matches() {
        oc681_child!(
            oc681_luno_vm_register_add_matches,
            oc681_luno_vm_register_add_run
        );
    }

    /// `SectionMan_RequestEvent(self, idx, mode)`: unloads every BE section
    /// the location `idx` does not use (not the always-resident 43-48),
    /// records the location, loads its sections, the sky and the transport
    /// section, then starts the event switch for `mode` (mode 7 maps through
    /// the location-mode table). Fixture: random locations in the static
    /// location map (0-6 sections, sky/transport present or -1).
    fn oc681_section_man_request_event_run() {
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_0022D598), oc681_fk_unload);
        oc681_hook(addr_of_mut!(ee::s05::OVERRIDE_0022D3D8), oc681_fk_load);
        oc681_hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D640),
            oc681_fk_event_switch,
        );
        covered(
            ee::s05::f_0022cea8__generated,
            oc681_section_man_request_event,
            120,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0022_cea8, 30);
                c.pc = 0x0022_cea8;
                let idx = seed % 4;
                let entry = 0x0044_2488 + 0x28 * idx;
                c.bus.write32(entry + 4, r.next_u32() % 7);
                let sky = [u32::MAX, 5, 0x31, 44][(r.next_u32() % 4) as usize];
                c.bus.write32(entry + 8, sky);
                c.bus
                    .write32(entry + 0xc, [u32::MAX, 7, 0][(r.next_u32() % 3) as usize]);
                for j in 0..6u32 {
                    let id = match r.next_u32() % 4 {
                        0 => 43 + r.next_u32() % 6,
                        _ => r.next_u32() % 50,
                    };
                    c.bus.write32(entry + 0x10 + 4 * j, id);
                }
                // `mLocationModeMap[idx].mode`.
                c.bus.write32(0x0044_2820 + 8 * idx + 4, r.next_u32() % 16);
                c.g[A0][0] = sx(OC681_BASE + 0x100);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    7
                } else {
                    (r.next_u32() % 16) as u64
                };
            },
        );
    }

    #[test]
    fn oc681_section_man_request_event_matches() {
        oc681_child!(
            oc681_section_man_request_event_matches,
            oc681_section_man_request_event_run
        );
    }

    /// `GameModule_ResetRace(self)`: resets the race for a restart: playback,
    /// buckets, AI (virtual call), avalanche, views, world, replay, script
    /// engine, camera, crowds, particles, the HUD state (when found: virtual
    /// call), scripts, audio, then comm/voip and the load indicator.
    /// Every callee is a logged fake; the HUD lookup returns an object or 0.
    fn oc681_game_module_reset_race_run() {
        covered(
            ee::s06::f_00230180__generated,
            oc681_game_module_reset_race,
            120,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_0180, 30);
                oc681_world(c, seed);
                c.pc = 0x0023_0180;
                for off in [0x10u32, 0x28, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x48, 0x94] {
                    c.bus.write32(OC681_GAME + off, r.next_u32());
                }
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_module_reset_race_matches() {
        oc681_child!(
            oc681_game_module_reset_race_matches,
            oc681_game_module_reset_race_run
        );
    }

    /// `GameModule_QuitToLoadScreen(self, toFrontEnd)`: requests state 6,
    /// leaves the comm session and voip, then makes a pre-game or a pre-FE
    /// load screen the next module. Not covered: a live game comm
    /// (`gp-0x204 != 0`) runs the unported, unbounded `cGameComm_QuitRace`
    /// on test data, so the fixture keeps it at 0.
    fn oc681_game_module_quit_to_load_screen_run() {
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_00231250),
            oc681_fk_request_state,
        );
        oc681_hook(addr_of_mut!(ee::s06::OVERRIDE_00232720), oc681_fk_pre_game);
        oc681_hook(addr_of_mut!(ee::s06::OVERRIDE_00232738), oc681_fk_pre_fe);
        covered(
            ee::s06::f_00230488__generated,
            oc681_game_module_quit_to_load_screen,
            100,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_0488, 9);
                oc681_world(c, seed);
                c.pc = 0x0023_0488;
                c.bus.write32(GAME_GP.wrapping_sub(0x204), 0);
                c.bus.write32(
                    GAME_GP + 0x304,
                    if seed & 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(OC681_GAME + 0x28, r.next_u32());
                c.g[A0][0] = sx(OC681_GAME);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn oc681_game_module_quit_to_load_screen_matches() {
        oc681_child!(
            oc681_game_module_quit_to_load_screen_matches,
            oc681_game_module_quit_to_load_screen_run
        );
    }

    /// `GameModule_BeginReplayPlayback(self)`: pushes pause state 8 on the
    /// pause stack (index at `gp-0x69c`), copies the flags word, starts the
    /// replay and activates the menu system.
    fn oc681_game_module_begin_replay_playback_run() {
        covered(
            ee::s06::f_00230558__generated,
            oc681_game_module_begin_replay_playback,
            100,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_0558, 2);
                c.pc = 0x0023_0558;
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), r.next_u32() % 4);
                c.bus.write32(0x0044_2910, r.next_u32());
                c.bus.write32(OC681_GAME + 0x28, r.next_u32());
                c.bus.write32(OC681_GAME + 0xa4, r.next_u32());
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_module_begin_replay_playback_matches() {
        oc681_child!(
            oc681_game_module_begin_replay_playback_matches,
            oc681_game_module_begin_replay_playback_run
        );
    }

    /// `bool GameModule_CanRestartFromPause(self)`: false in the pending
    /// states 5, 14, 1, 10; otherwise whether the script engine allows the
    /// pause menu and the current state is not one of those. Fixture: both
    /// state words over the listed states, the script query returning 0/1.
    fn oc681_game_module_can_restart_from_pause_run() {
        covered(
            ee::s06::f_00231ab8__generated,
            oc681_game_module_can_restart_from_pause,
            160,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_1ab8, 1);
                oc681_world(c, seed >> 3);
                c.pc = 0x0023_1ab8;
                let states = [0u32, 5, 14, 1, 10, 2, 7];
                c.bus
                    .write32(OC681_GAME + 0x210, states[(seed % 7) as usize]);
                c.bus
                    .write32(OC681_GAME + 0x214, states[(r.next_u32() % 7) as usize]);
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_module_can_restart_from_pause_matches() {
        oc681_child!(
            oc681_game_module_can_restart_from_pause_matches,
            oc681_game_module_can_restart_from_pause_run
        );
    }

    /// `GameStateRestartRace_Enter(self)`: resets the game mode and race,
    /// then (for a heat that is neither mode 5/6 nor flagged) re-adds the AI
    /// riders, restarts the replay/camera and sets the state to 0, or sets
    /// state 3 straight away. Every callee but `GameModeMan_ResetMode` and
    /// `GameModule_ResetRace` (both direct) is a logged fake.
    fn oc681_game_state_restart_race_enter_run() {
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_002382D8),
            oc681_fk_reset_mode,
        );
        covered(
            ee::s06::f_00235aa0__generated,
            oc681_game_state_restart_race_enter,
            160,
            &|c, seed| {
                oc681_use_stubs();
                let mut r = machine(c, seed, 0x0023_5aa0, 40);
                oc681_world(c, seed);
                c.pc = 0x0023_5aa0;
                let modes = [5u8, 6, 3, 0];
                c.bus
                    .write8(OC681_NEW_RACE + 0x48, modes[(seed % 4) as usize]);
                c.bus.write8(OC681_NEW_RACE + 0x49, ((seed >> 2) & 1) as u8);
                // The AI count (+0x10 - +0x14) and the per-AI stride.
                let n = r.next_u32() % 5;
                c.bus.write32(OC681_MODE_MAN + 0x10, n + 1);
                c.bus.write32(OC681_MODE_MAN + 0x14, 1);
                c.bus.write32(OC681_MODE_MAN + 0x98, r.next_u32() % 2);
                for i in 0..8u32 {
                    c.bus.write32(OC681_MODE_MAN + 0x18 + 4 * i, r.next_u32());
                    c.bus.write32(OC681_MODE_MAN + 0x40 + 4 * i, r.next_u32());
                }
                let (ai_man, ai_man2) = (OC681_BASE + 0x7000, OC681_BASE + 0x7100);
                c.bus.write32(GAME_GP.wrapping_sub(0x488), ai_man);
                c.bus.write32(ai_man + 0xc, ai_man2);
                c.bus.write32(ai_man2 + 0x7c, 1 + r.next_u32() % 2);
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_state_restart_race_enter_matches() {
        oc681_child!(
            oc681_game_state_restart_race_enter_matches,
            oc681_game_state_restart_race_enter_run
        );
    }

    /// `GameStateWarpToEvent_Enter(self)`: fails the active mission, starts
    /// game mode 0xc, partially restarts the race, puts the rider at the
    /// start point and starts a white fade-in transition (a new
    /// `cFade_TObj` with colour 1.0 x4). Direct callees `StartMode`,
    /// `ResetRace`, `GetTransitionMan` and the rest are fakes.
    fn oc681_game_state_warp_to_event_enter_run() {
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_00238160),
            oc681_fk_start_mode,
        );
        oc681_hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            oc681_fk_transition_man,
        );
        covered(
            ee::s06::f_00236058__generated,
            oc681_game_state_warp_to_event_enter,
            80,
            &|c, seed| {
                oc681_use_stubs();
                let _r = machine(c, seed, 0x0023_6058, 24);
                oc681_world(c, seed);
                c.pc = 0x0023_6058;
                c.g[A0][0] = sx(OC681_GAME);
            },
        );
    }

    #[test]
    fn oc681_game_state_warp_to_event_enter_matches() {
        oc681_child!(
            oc681_game_state_warp_to_event_enter_matches,
            oc681_game_state_warp_to_event_enter_run
        );
    }
}
