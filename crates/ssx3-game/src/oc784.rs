//! Staging module oc784 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc784`.

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
    (0x001d_5df0, mem_card_ui_begin_load, "MemCardUI_BeginLoad"),
    (0x002a_cd08, bank_monitor_ctor, "BankMonitor_ctor"),
    (0x0039_bab0, ui_pair_load, "UIPair_Load"),
    (
        0x0028_9520,
        ssx_audio_system_load_playlist,
        "SSXAudioSystem_LoadPlaylist",
    ),
    (
        0x0020_d1d8,
        ov_screen_overlay_menu_init,
        "OVScreenOverlayMenu_Init",
    ),
    (
        0x002b_5a18,
        world_trigger_manager_load_trigger_info,
        "WorldTriggerManager_LoadTriggerInfo",
    ),
];

/// `void MemCardUI_BeginLoad(void)` (0x1d5df0; reference `cFEMemCard_readStart`).
///
/// Starts the memory-card read of the front-end save screen. Marks the
/// `cFEMemCard` instance (`gp-0x10c8`) as waiting (`+0x1b0 = 1`, `+0x1b4 = 0`),
/// points the card manager's file cursor (`+0xf8`) at its slot (`+0x1c4`) and
/// then reads by load type (`+0x1bc`): 0 profile (size
/// `getSaveProfileSize`), 1 replay (buffer `getReplayBuffer`, size
/// `cBE_getDataSize`, `+0x228 = 2`, remembers the type at `+0x200`), 2 save
/// game (size from `0x152948`); each reads via `cMCOverlayManager_readFile`.
/// Other types do nothing more.
pub fn mem_card_ui_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d5f24: {
            'b_1d5f20: {
                'b_1d5f1c: {
                    'b_1d5eec: {
                        'b_1d5ea4: {
                            c.cycles += 23;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            c.g[RA][0] = sx(0x001d5e0c);
                            call_ptr(c, 0x0014dc80, 0x001d5e0c)?;
                            c.g[A1][0] = 0x5;
                            c.g[RA][0] = sx(0x001d5e18);
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014dd58, 0x001d5e18)?;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[RA][0] = sx(0x001d5e24);
                            c.g[S0][0] = c.g[V0][0];
                            call_ptr(c, 0x00227f80, 0x001d5e24)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c4));
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x1b0), c.g[V1][0]);
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x1b4), 0u64);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                            c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1bc));
                            let t = c.g[S1][0] == c.g[V1][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_1d5ea4;
                            } else {
                                'b_1d5e64: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = 0x2;
                                        break 'b_1d5e64;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1d5eec;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1d5f24;
                                            } else {
                                                break 'b_1d5e64;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] != c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1d5f20;
                                } else {
                                    c.cycles += 14;
                                    c.g[RA][0] = sx(0x001d5e74);
                                    call_ptr(c, 0x00152948, 0x001d5e74)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                                    c.g[RA][0] = sx(0x001d5e84);
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x001d9948, 0x001d5e84)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A3][0] = c.g[S0][0];
                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d0));
                                    c.g[RA][0] = sx(0x001d5e9c);
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c4));
                                    call_ptr(c, 0x0023ca28, 0x001d5e9c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1d5f20;
                                    } else {
                                        break 'b_1d5ea4;
                                    }
                                }
                            }
                        }
                        c.cycles += 18;
                        c.g[V0][0] = 0x2;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001d5eb4);
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x228), c.g[V0][0]);
                        call_ptr(c, 0x00152688, 0x001d5eb4)?;
                        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1d4), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001d5ec4);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1d0), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x001d5ec4)?;
                        c.g[RA][0] = sx(0x001d5ecc);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014e048, 0x001d5ecc)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d0));
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                        c.g[RA][0] = sx(0x001d5ee0);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0023ca28, 0x001d5ee0)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                        let t = true;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x200), c.g[S1][0]);
                        if t {
                            break 'b_1d5f1c;
                        } else {
                            break 'b_1d5eec;
                        }
                    }
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001d5ef4);
                    call_ptr(c, 0x00152ba8, 0x001d5ef4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[RA][0] = sx(0x001d5f04);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001d9948, 0x001d5f04)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d0));
                    c.g[RA][0] = sx(0x001d5f1c);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c4));
                    call_ptr(c, 0x0023ca28, 0x001d5f1c)?;
                    break 'b_1d5f1c;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1d5f20;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1d5f24;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `BankMonitor* BankMonitor::BankMonitor(BankMonitor* this, int inCharge, void* heap)` (0x2acd08).
///
/// In-charge construction builds the virtual `SoundManagerRef` base at
/// `this + 0x8f0`; then the monitor base (`BankMonitor_BaseCtor`), the
/// vtable word `0x483b88` for the virtual base (for a derived construction
/// a copy of it with the `this` adjustment fixed up is built on the stack),
/// the flags `+0x8dc = 0`, `+0x8e0 = +0x8e4 = 1`, and the 64 voice objects
/// (`+0x8ec`, `new[]` of 0xc0 bytes each with a ctor loop). Returns `this`.
pub fn bank_monitor_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2acd4c: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_2acd4c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8f0));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002acd4c);
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x002acd4c)?;
                break 'b_2acd4c;
            }
        }
        'b_2acdb0: {
            c.cycles += 9;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x002acd5c);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002ad9c8, 0x002acd5c)?;
            c.g[V1][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3b88));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
            if t {
                break 'b_2acdb0;
            } else {
                c.cycles += 16;
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x3b88));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd(c.g[SP][0] as u32, c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8f0));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_2acdb0;
            }
        }
        c.cycles += 20;
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b48));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8dc), 0u64);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8e4), c.g[V1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8e0), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3238));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8d8), c.g[V0][0]);
        c.g[A0][0] = 0x3010;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002acde4);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002acde4)?;
        c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8ec));
        c.g[V1][0] = 0x40;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[S0][0] = c.g[S3][0];
        c.g[S1][0] = 0x3f;
        c.g[S4][0] = 0xffffffffffffffff;
        'l_2ace00: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002ace0c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            call_ptr(c, 0x002ab9e0, 0x002ace0c)?;
            let t = c.g[S1][0] != c.g[S4][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002ace00);
                }
                continue 'l_2ace00;
            } else {
                c.cycles += 11;
                c.sw(c.g[S5][0] as u32, c.g[S3][0]);
                c.g[V0][0] = c.g[S2][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void UIPair::Load(UIPair* this)` (0x39bab0): just `UIObject_Load(this)`.
pub fn ui_pair_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039bac0);
        call_ptr(c, 0x0039fe00, 0x0039bac0)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `void SSXAudioSystem_LoadPlaylist(SSXAudioSystem* this, const char* name)` (0x289520).
///
/// Clears the music system (`this + 0x118`), parses the playlist file's
/// section `name` and adds every `SONG` key to it (`func_002B3EE8`), then
/// stores the playlist's index among the 20 names (`+0x60a4`, 16 bytes each;
/// 20 when not found) at `+0x6238`, finishes the music system, copies its
/// active map (`+0x510 -> +0x518`) and destroys the parser.
pub fn ssx_audio_system_load_playlist(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8b0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x850), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002b3eb0, 0x00289550)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x46c));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x002a77f8, 0x00289560)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x002a7c68, 0x0028956c)?;
        let not_found = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x14;
        // 0x2895d8 is the "name search" stage; the song scan runs only when
        // the section was found.
        if !not_found {
            // 0x289574
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002a7a20, 0x0028957c)?;
            c.g[S3][0] = c.g[S0][0];
            // 0x2895c8: loop head; the body (0x289588) runs while !IsEOF.
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7f90, 0x002895d0)?;
                let eof = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x14;
                if eof {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00289588);
                }
                // 0x289588: next key name; `SONG` keys assign the song.
                c.cycles += 9;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7da0, 0x00289590)?;
                c.g[V1][0] = 0x4a0000;
                c.g[A1][0] = c.g[V0][0];
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3608));
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0041aa88, 0x002895a4)?;
                let not_song = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                if !not_song {
                    // 0x2895ac
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x002a7ff8, 0x002895bc)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x002b3ee8, 0x002895c8)?;
                }
            }
        }
        // 0x2895d8: playlist index = 20 ("not found"), then search the names.
        c.cycles += 4;
        c.g[S0][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x6238), c.g[V0][0]);
        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        loop {
            // 0x2895e8
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 20i64) as u64;
            let done = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x14;
            if done {
                break;
            }
            // 0x2895f4
            c.cycles += 7;
            c.g[A1][0] = c.g[S4][0];
            c.mult(0, 4, 16, 4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60a4));
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x004165a8, 0x00289608)?;
            if c.g[V0][0] != 0u64 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x002895e8);
                }
                continue;
            }
            // 0x289610: match: remember the index.
            c.cycles += 1;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x6238), c.g[S0][0]);
            break;
        }
        // 0x289614
        c.cycles += 15;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x002b4060, 0x0028961c)?;
        c.g[V0][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x510));
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        c.sd((c.g[S2][0] as u32).wrapping_add(0x518), c.g[V0][0]);
        call_ptr(c, 0x002a7890, 0x00289630)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x850));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.pc = t;
        return Ok(());
    })
}

/// `void cOVStateOverlayMenu::init(cOVState* this)` (0x20d1d8).
///
/// Creates the overlay menu screen (`onCreateScreen`), marks the
/// `overlayMenu` list object (flag `|= 8` at `+0x90`), then for the five
/// entries `overlayOpt%d`: entry 0 gets the label of option `D_004A2860[0]`
/// (`setUnicodeStringByID`, index stored at `+0x18`), entries 1..4 are hidden
/// (`setVisible(0)`) and made static (`setStatic(1)`) through their vtable.
pub fn ov_screen_overlay_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S4][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x890));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x0020d210);
        c.g[S6][0] = 0x470000;
        call_ptr(c, 0x0020a380, 0x0020d210)?;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x0020d21c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1af0));
        call_ptr(c, 0x00317670, 0x0020d21c)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x0020d228);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x0020d228)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
        c.g[A0][0] = 0x440000;
        c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d10));
        c.g[V1][0] = c.g[V1][0] | 0x8;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), c.g[V1][0]);
        'l_20d240: loop {
            'b_20d2d0: {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020d250);
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1b40));
                call_ptr(c, 0x00417828, 0x0020d250)?;
                c.g[RA][0] = sx(0x0020d258);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x0020d258)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d264);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d264)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S1][0] == 0u64 {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    break 'b_20d2d0;
                } else {
                    'b_20d2cc: {
                        'b_20d2a0: {
                            c.cycles += 2;
                            let t = (c.g[S2][0] as i64) > 0;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_20d2a0;
                            } else {
                                c.cycles += 10;
                                c.g[S0][0] = c.lw(c.g[S4][0] as u32);
                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                c.g[RA][0] = sx(0x0020d28c);
                                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                call_ptr(c, 0x00317670, 0x0020d28c)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0020d298);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x003a1000, 0x0020d298)?;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                                if t {
                                    break 'b_20d2cc;
                                } else {
                                    break 'b_20d2a0;
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0020d2b4);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0020d2b4)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0020d2cc);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0020d2cc)?;
                        break 'b_20d2cc;
                    }
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    break 'b_20d2d0;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0020d240);
                }
                continue 'l_20d240;
            } else {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `int WorldTriggerManager::LoadTriggerInfo(WorldTriggerManager* this)` (0x2b5a18).
///
/// Loads the trigger file when it exists: keeps the pack (`+0x338`), returns 0
/// when its format flag is set, otherwise allocates `n + 2` 8-byte info
/// entries (`+0x340` count, `+0x33c` array) and fills entries 2.. with each
/// record's type and address, stepping over the record by its type (1: 0x18,
/// 2: 0x38, 3: 0x30, 4: 0x10 bytes). Without the file everything is cleared.
/// Returns 1.
pub fn world_trigger_manager_load_trigger_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_2b5b2c: {
            'b_2b5b28: {
                'b_2b5b1c: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x768));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x002b5a38);
                    call_ptr(c, 0x00317f38, 0x002b5a38)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x768));
                    if t {
                        break 'b_2b5b1c;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x002b5a48);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x003e22f0, 0x002b5a48)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x338), c.g[S1][0]);
                        c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_2b5b2c;
                        } else {
                            c.cycles += 15;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x480000;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x35c0));
                            c.g[A0][0] = sx((c.g[V0][0] as u32) << 3);
                            c.g[RA][0] = sx(0x002b5a80);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x340), c.g[V0][0]);
                            call_ptr(c, 0x00317e30, 0x002b5a80)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x340));
                            c.g[A3][0] = 0x2;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x33c), c.g[V0][0]);
                            c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                            if t {
                                break 'b_2b5b28;
                            } else {
                                c.cycles += 4;
                                c.g[T2][0] = 0x1;
                                c.g[T1][0] = 0x2;
                                c.g[T0][0] = 0x3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x33c));
                                'l_2b5aa8: loop {
                                    'b_2b5b00: {
                                        'b_2b5ad8: {
                                            c.cycles += 10;
                                            c.g[A0][0] = sx((c.g[A3][0] as u32) << 3);
                                            c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[V1][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x33c));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                            let t = c.g[V1][0] != c.g[T2][0];

                                            if t {
                                                break 'b_2b5ad8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x18));
                                                if t {
                                                    break 'b_2b5b00;
                                                } else {
                                                    break 'b_2b5ad8;
                                                }
                                            }
                                        }
                                        'b_2b5ae8: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[T1][0];

                                            if t {
                                                break 'b_2b5ae8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x38));
                                                if t {
                                                    break 'b_2b5b00;
                                                } else {
                                                    break 'b_2b5ae8;
                                                }
                                            }
                                        }
                                        'b_2b5af8: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[T0][0];
                                            c.g[V1][0] = c.g[V1][0] ^ 0x4;
                                            if t {
                                                break 'b_2b5af8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_2b5b00;
                                                } else {
                                                    break 'b_2b5af8;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                                        if c.g[V1][0] == 0 {
                                            c.g[A2][0] = c.g[V0][0];
                                        }
                                        break 'b_2b5b00;
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x340));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x33c));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002b5aa8);
                                        }
                                        continue 'l_2b5aa8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_2b5b2c;
                                        } else {
                                            break 'b_2b5b1c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x33c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x338), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x340), 0u64);
                break 'b_2b5b28;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_2b5b2c;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, put_words, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc784's callee fakes in a machine's RAM.
    pub(crate) const OC784_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC784_MAGIC: u32 = 0x0784_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc784_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// The "first n calls return 1" counter of the fake for `addr`.
    pub(crate) fn oc784_count(addr: u32) -> u32 {
        DATA + 0x30_4000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc784_arm(c: &mut Cpu) {
        c.bus.write32(OC784_CTRL, OC784_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc784_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc784_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc784_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc784_mail(addr), 1);
    }

    /// The first `n` calls of the fake for `addr` return 1.
    pub(crate) fn oc784_first_ones(c: &mut Cpu, addr: u32, n: u32) {
        c.bus.write32(oc784_count(addr), n);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (or 1 while its counter runs) unless it
    /// unwound.
    pub(crate) fn oc784_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc784_mail(addr));
        let count = c.bus.read32(oc784_count(addr));
        if count > 0 {
            c.bus.write32(oc784_count(addr), count - 1);
        }
        fakes::fake_call(c, addr);
        if c.pc == ra {
            if count > 0 {
                c.g[V0][0] = 1;
            } else if mail != 0 {
                c.g[V0][0] = match mail {
                    1 => 0,
                    m => (m - 3) as u64,
                };
            }
        }
    }

    macro_rules! oc784_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC784_CTRL) != OC784_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc784_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc784_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc784_fakes!(
        (
            oc784_fk_14dc80,
            OC784_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc784_fk_14dd58,
            OC784_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc784_fk_227f80,
            OC784_PREV_00227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc784_fk_152948,
            OC784_PREV_00152948,
            ee::s01::OVERRIDE_00152948,
            ee::s01::f_00152948__generated,
            0x0015_2948
        ),
        (
            oc784_fk_1d9948,
            OC784_PREV_001D9948,
            ee::s04::OVERRIDE_001D9948,
            ee::s04::f_001d9948__generated,
            0x001d_9948
        ),
        (
            oc784_fk_23ca28,
            OC784_PREV_0023CA28,
            ee::s06::OVERRIDE_0023CA28,
            ee::s06::f_0023ca28__generated,
            0x0023_ca28
        ),
        (
            oc784_fk_152688,
            OC784_PREV_00152688,
            ee::s01::OVERRIDE_00152688,
            ee::s01::f_00152688__generated,
            0x0015_2688
        ),
        (
            oc784_fk_14e048,
            OC784_PREV_0014E048,
            ee::s01::OVERRIDE_0014E048,
            ee::s01::f_0014e048__generated,
            0x0014_e048
        ),
        (
            oc784_fk_152ba8,
            OC784_PREV_00152BA8,
            ee::s01::OVERRIDE_00152BA8,
            ee::s01::f_00152ba8__generated,
            0x0015_2ba8
        ),
        (
            oc784_fk_2ade88,
            OC784_PREV_002ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc784_fk_317e30,
            OC784_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc784_fk_2ab9e0,
            OC784_PREV_002AB9E0,
            ee::s08::OVERRIDE_002AB9E0,
            ee::s08::f_002ab9e0__generated,
            0x002a_b9e0
        ),
        (
            oc784_fk_2ad848,
            OC784_PREV_002AD848,
            ee::s08::OVERRIDE_002AD848,
            ee::s08::f_002ad848__generated,
            0x002a_d848
        ),
        (
            oc784_fk_39fe00,
            OC784_PREV_0039FE00,
            ee::s13::OVERRIDE_0039FE00,
            ee::s13::f_0039fe00__generated,
            0x0039_fe00
        ),
        (
            oc784_fk_2b3eb0,
            OC784_PREV_002B3EB0,
            ee::s08::OVERRIDE_002B3EB0,
            ee::s08::f_002b3eb0__generated,
            0x002b_3eb0
        ),
        (
            oc784_fk_2a77f8,
            OC784_PREV_002A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8
        ),
        (
            oc784_fk_2a7c68,
            OC784_PREV_002A7C68,
            ee::s08::OVERRIDE_002A7C68,
            ee::s08::f_002a7c68__generated,
            0x002a_7c68
        ),
        (
            oc784_fk_2a7a20,
            OC784_PREV_002A7A20,
            ee::s08::OVERRIDE_002A7A20,
            ee::s08::f_002a7a20__generated,
            0x002a_7a20
        ),
        (
            oc784_fk_41aa88,
            OC784_PREV_0041AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88
        ),
        (
            oc784_fk_2a7ff8,
            OC784_PREV_002A7FF8,
            ee::s08::OVERRIDE_002A7FF8,
            ee::s08::f_002a7ff8__generated,
            0x002a_7ff8
        ),
        (
            oc784_fk_2b3ee8,
            OC784_PREV_002B3EE8,
            ee::s08::OVERRIDE_002B3EE8,
            ee::s08::f_002b3ee8__generated,
            0x002b_3ee8
        ),
        (
            oc784_fk_2a7f90,
            OC784_PREV_002A7F90,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90
        ),
        (
            oc784_fk_4165a8,
            OC784_PREV_004165A8,
            ee::s15::OVERRIDE_004165A8,
            ee::s15::f_004165a8__generated,
            0x0041_65a8
        ),
        (
            oc784_fk_2a7890,
            OC784_PREV_002A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890
        ),
        (
            oc784_fk_20a380,
            OC784_PREV_0020A380,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020_a380
        ),
        (
            oc784_fk_317670,
            OC784_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc784_fk_39d860,
            OC784_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc784_fk_417828,
            OC784_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc784_fk_3a1000,
            OC784_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc784_fk_317f38,
            OC784_PREV_00317F38,
            ee::s10::OVERRIDE_00317F38,
            ee::s10::f_00317f38__generated,
            0x0031_7f38
        ),
        (
            oc784_fk_3e22f0,
            OC784_PREV_003E22F0,
            ee::s14::OVERRIDE_003E22F0,
            ee::s14::f_003e22f0__generated,
            0x003e_22f0
        ),
    );

    pub(crate) const OC784_OBJ: u32 = DATA + 0x1_0000;
    pub(crate) const OC784_APP: u32 = DATA + 0x1_8000;
    pub(crate) const OC784_FILE: u32 = DATA + 0x2_0000;
    pub(crate) const OC784_NEW: u32 = DATA + 0x3_0000;

    /// `MemCardUI_BeginLoad`: the load type (0, 1, 2 and others) and the
    /// unwinds of its up to eight calls.
    #[test]
    fn mem_card_ui_begin_load_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s04::f_001d5df0__generated,
            mem_card_ui_begin_load,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784a, 7);
                oc784_arm(c);
                c.bus.write32(GAME_GP - 0x848, OC784_APP);
                c.bus.write32(GAME_GP - 0x10c8, OC784_OBJ);
                let mode = [0u32, 1, 2, 3, 0xffff_ffff][(seed % 5) as usize];
                c.bus.write32(OC784_OBJ + 0x1bc, mode);
                c.bus.write32(OC784_OBJ + 0x1c4, r.next_u32());
                c.bus.write32(OC784_OBJ + 0x1d0, r.next_u32());
            },
        );
    }

    /// `BankMonitor_ctor`: in-charge and derived construction. The base
    /// constructor (0x2ad9c8, no override slot) and `SndPlayParams_ctor` run
    /// for real; the other callees are faked.
    #[test]
    fn bank_monitor_ctor_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s08::f_002acd08__generated,
            bank_monitor_ctor,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784b, 4);
                oc784_arm(c);
                c.g[A0][0] = sx(OC784_OBJ);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = sx(DATA + 0x4_0000);
                // The derived form reads the virtual base pointer at `this + 0`.
                c.bus
                    .write32(OC784_OBJ, OC784_OBJ + 0x900 + (r.next_u32() & 0xf0));
                c.bus.write32(OC784_NEW, 0);
                oc784_force(c, 0x0031_7e30, OC784_NEW);
            },
        );
    }

    /// `UIPair_Load`: a single call to `UIObject_Load`.
    #[test]
    fn ui_pair_load_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s13::f_0039bab0__generated,
            ui_pair_load,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x784c, 1);
                oc784_arm(c);
            },
        );
    }

    /// `SSXAudioSystem_LoadPlaylist`: section found or not, how many names
    /// mismatch before the match (or none matches), and the unwinds. The song
    /// scan loop (`IsEOF` false) needs the real `IniParser_ReadToken`
    /// (0x2a7da0, no override slot), so `IsEOF` always reports end of file.
    #[test]
    fn ssx_audio_system_load_playlist_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s07::f_00289520__generated,
            ssx_audio_system_load_playlist,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784d, 10);
                oc784_arm(c);
                c.pc = 0x0028_9520;
                c.g[A0][0] = sx(OC784_OBJ);
                c.g[A1][0] = sx(OC784_FILE);
                c.bus.write32(GAME_GP + 0x46c, r.next_u32());
                c.bus.write32(OC784_OBJ + 0x510, r.next_u32());
                c.bus.write32(OC784_OBJ + 0x514, r.next_u32());
                if seed % 3 == 0 {
                    oc784_force_zero(c, 0x002a_7c68);
                } else {
                    oc784_force(c, 0x002a_7c68, 5);
                }
                oc784_force(c, 0x002a_7f90, 5);
                oc784_first_ones(c, 0x0041_65a8, (seed / 3) % 22);
                if (seed / 66) % 2 == 0 {
                    oc784_force_zero(c, 0x0041_65a8);
                }
            },
        );
    }

    /// `OVScreenOverlayMenu_Init`: the overlay list and the five option
    /// widgets (each with a vtable of faked `setVisible`/`setStatic`), or no
    /// widgets at all (every lookup returns 0).
    #[test]
    fn ov_screen_overlay_menu_init_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s05::f_0020d1d8__generated,
            ov_screen_overlay_menu_init,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784e, 20);
                oc784_arm(c);
                c.g[A0][0] = sx(OC784_OBJ);
                c.bus.write32(OC784_OBJ + 0x40, r.next_u32());
                let vt = fakes::vtable(c, &mut r, &[0x40, 0x48]);
                c.bus.write32(OC784_FILE + 8, vt);
                c.bus.write32(OC784_FILE + 0x90, r.next_u32());
                c.bus.write32(OC784_FILE + 0x18, r.next_u32());
                if seed % 4 == 0 {
                    oc784_force_zero(c, 0x0039_d860);
                } else {
                    oc784_force(c, 0x0039_d860, OC784_FILE);
                }
            },
        );
    }

    /// `WorldTriggerManager_LoadTriggerInfo`: no trigger file, a file with the
    /// format flag set, and files with 0..4 records of random types.
    #[test]
    fn world_trigger_manager_load_trigger_info_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s08::f_002b5a18__generated,
            world_trigger_manager_load_trigger_info,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784f, 3);
                oc784_arm(c);
                c.g[A0][0] = sx(OC784_OBJ);
                c.bus.write32(GAME_GP + 0x768, r.next_u32());
                if seed % 5 == 0 {
                    oc784_force_zero(c, 0x0031_7f38);
                } else {
                    oc784_force(c, 0x0031_7f38, 1);
                }
                oc784_force(c, 0x003e_22f0, OC784_FILE);
                oc784_force(c, 0x0031_7e30, OC784_NEW);
                for w in 0..0x80u32 {
                    c.bus.write32(OC784_FILE + 4 * w, r.next_u32() % 5);
                }
                c.bus.write8(OC784_FILE, if seed % 7 == 1 { 1 } else { 0 });
                c.bus
                    .write32(OC784_FILE + 8, [0u32, 1, 2, 3, 4, 6][(seed % 6) as usize]);
            },
        );
    }
}
