//! Staging module oc782 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc782`.

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
        0x0018_77b0,
        fe_state_mem_card_start_profile_search,
        "cFEStateMemCard_StartProfileSearch",
    ),
    (
        0x0019_fa78,
        fe_module_rider_preview_release_bolt_textures,
        "FEModule_RiderPreviewReleaseBoltTextures",
    ),
    (
        0x0028_c2d0,
        ssx_audio_system_set_request_playlist,
        "SSXAudioSystem_SetRequestPlaylist",
    ),
    (
        0x001e_12f0,
        message_center_init_group_table,
        "MessageCenter_InitGroupTable",
    ),
    (0x002e_4370, c_transition_man_start, "cTransitionMan_Start"),
    (0x0028_b9b8, snd_bank_man_ctor, "SndBankMan_ctor"),
];

/// `void cFEStateMemCard_StartProfileSearch(void)` (0x1877b0, frame 0x90): the memory
/// card screen starts reading the profile file. Sets the state stage and the
/// instance's stage word to 5, asks the BE interface for interface 5, sets up
/// the memory card manager (profile read size and file type 1, file cursor and
/// slot from the profile file global, state 53), formats the file name and
/// product code into the instance's text buffer (+0x11c), marks the load
/// pressed and processing (+0x1a8), and plays the `@loadanim` animation on
/// the `!` screen object when both exist.
pub fn fe_state_mem_card_start_profile_search(c: &mut Cpu) {
    guest(c, |c| {
        'b_187908: {
            c.cycles += 71;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = 0x5;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c34), c.g[S1][0]);
            c.g[RA][0] = sx(0x001877d4);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), c.g[S1][0]);
            call_ptr(c, 0x0014dc80, 0x001877d4)?;
            c.g[A1][0] = 0x5;
            c.g[RA][0] = sx(0x001877e0);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x001877e0)?;
            c.g[RA][0] = sx(0x001877e8);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            call_ptr(c, 0x00227f80, 0x001877e8)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[S0][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1bc), 0u64);
            c.g[RA][0] = sx(0x001877fc);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c34), c.g[S1][0]);
            call_ptr(c, 0x00152ba8, 0x001877fc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00187808);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023caa0, 0x00187808)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00187814);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0023c8f0, 0x00187814)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c3c));
            c.g[A1][0] = 0x35;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1bc), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00187840);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00187840)?;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0x14;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00187854);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x00241d40, 0x00187854)?;
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00187868);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14c8));
            call_ptr(c, 0x002c2540, 0x00187868)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = 0x10;
            c.g[RA][0] = sx(0x00187878);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00241aa0, 0x00187878)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00187890);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11c));
            call_ptr(c, 0x002c26d0, 0x00187890)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[RA][0] = sx(0x0018789c);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x001d9308, 0x0018789c)?;
            c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[V0][0] = 0x1;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14c0));
            c.g[RA][0] = sx(0x001878b4);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001878b4)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001878c0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001878c0)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            if t {
                break 'b_187908;
            } else {
                c.cycles += 10;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2300));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x001878e0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                call_ptr(c, 0x00317670, 0x001878e0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001878ec);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x00397140, 0x001878ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if t {
                    break 'b_187908;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0x9;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00187908);
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x0039fcc8, 0x00187908)?;
                    break 'b_187908;
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void FEModule_RiderPreviewReleaseBoltTextures(FEModule* self, list)` (0x19fa78,
/// frame 0x90): when the preview's profile index (+4) and character id (+0)
/// are valid, walks the character's bolt part entries (0x38 bytes each, count
/// from the profile DB), and for each one whose bolt texture slot is
/// assigned and whose part type is not 6 releases the bolt file texture and
/// clears the slot to -1; a type 6 entry keeps its texture and clears the
/// done flag. Stores the flag at +0xcb8 (`mDoneLoadingTextures`).
pub fn fe_module_rider_preview_release_bolt_textures(c: &mut Cpu) {
    guest(c, |c| {
        'b_19fbb8: {
            'b_19fbb4: {
                c.cycles += 24;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x0019faac);
                c.g[S4][0] = c.g[A1][0];
                call_ptr(c, 0x0014dc80, 0x0019faac)?;
                c.g[A1][0] = 0x9;
                c.g[RA][0] = sx(0x0019fab8);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x0019fab8)?;
                c.g[RA][0] = sx(0x0019fac0);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014bdb8, 0x0019fac0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                let t = (c.g[A1][0] as i64) < 0;
                c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
                if t {
                    break 'b_19fbb4;
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                    if (c.g[A2][0] as i64) < 0 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                        break 'b_19fbb8;
                    } else {
                        'b_19fafc: {
                            c.cycles += 2;
                            if c.g[S4][0] != 0u64 {
                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                break 'b_19fafc;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x0019faf4);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x0014ad28, 0x0019faf4)?;
                                c.g[S4][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                break 'b_19fafc;
                            }
                        }
                        'b_19fbb0: {
                            c.cycles += 9;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x0019fb08);
                            c.g[S5][0] = 0x1;
                            call_ptr(c, 0x0014d988, 0x0019fb08)?;
                            c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x0019fb18);
                            c.g[S0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014d998, 0x0019fb18)?;
                            let t = (c.g[S0][0] as i64) <= 0;
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_19fbb0;
                            } else {
                                c.cycles += 2;
                                c.g[S3][0] = c.g[S0][0];
                                c.g[S7][0] = 0x6;
                                'l_19fb28: loop {
                                    'b_19fb54: {
                                        'b_19fb50: {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x288));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                            let t = (c.g[V0][0] as i64) < 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            if t {
                                                break 'b_19fb50;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                                let t = true;
                                                c.g[S0][0] = sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                if t {
                                                    break 'b_19fb54;
                                                } else {
                                                    break 'b_19fb50;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S0][0] = 0u64;
                                        break 'b_19fb54;
                                    }
                                    'b_19fba8: {
                                        c.cycles += 7;
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                        c.g[RA][0] = sx(0x0019fb64);
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        Callee {
                                            addr: 0x0019cce8,
                                            f: ee::s03::BoltFileIndexManager_GetTextureSlot,
                                        }
                                        .call(c, 0x0019fb64)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        if (c.g[A2][0] as i64) < 0 {
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                            break 'b_19fba8;
                                        } else {
                                            'b_19fba4: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lb((c.g[S1][0] as u32).wrapping_add(0x3));
                                                if c.g[V0][0] == c.g[S7][0] {
                                                    c.g[S5][0] = 0u64;
                                                    break 'b_19fba4;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                                    c.g[RA][0] = sx(0x0019fb88);
                                                    c.g[A0][0] = c.g[S6][0];
                                                    Callee {
                                                        addr: 0x0019dc20,
                                                        f: ee::s03::BoltFile_Release,
                                                    }
                                                    .call(c, 0x0019fb88)?;
                                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                                    c.g[A3][0] = 0xffffffffffffffff;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1788));
                                                    c.g[RA][0] = sx(0x0019fb9c);
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                    Callee { addr: 0x0019cd38, f: ee::s03::BoltFileIndexManager_SetTextureSlot }.call(c, 0x0019fb9c)?;
                                                    let t = true;
                                                    c.g[S3][0] =
                                                        sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                                    if t {
                                                        break 'b_19fba8;
                                                    } else {
                                                        break 'b_19fba4;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                            break 'b_19fba8;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S3][0] != 0u64;
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0019fb28);
                                        }
                                        continue 'l_19fb28;
                                    } else {
                                        break 'b_19fbb0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xcb8), c.g[S5][0]);
                        break 'b_19fbb4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_19fbb8;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_SetRequestPlaylist(SSXAudioSystem* sys, long key)`
/// (0x28c2d0, frame 0x40): stores the requested radio playlist (+0x6098).
/// When the radio mode (+0x608c) is 1 or 3 and the playlist changed, first
/// calls `func_002B4070` on the music system at +0x118; then (modes 1 and 3
/// only, a changed playlist or not) while in a game with a music system
/// (+0x530) whose state is not 1..3 and (state 0x12d or no pending
/// request) restarts playback with `SelectRandomSong` and `PlayMusic`.
pub fn ssx_audio_system_set_request_playlist(c: &mut Cpu) {
    guest(c, |c| {
        'b_28c418: {
            'b_28c390: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x6098));
                let t = c.g[V0][0] == c.g[A1][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_28c390;
                } else {
                    'b_28c30c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.sd((c.g[S0][0] as u32).wrapping_add(0x6098), c.g[A1][0]);
                        if t {
                            break 'b_28c30c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_28c418;
                            } else {
                                break 'b_28c30c;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.g[RA][0] = sx(0x0028c318);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x002b4070, 0x0028c318)?;
                    c.g[RA][0] = sx(0x0028c320);
                    Callee {
                        addr: 0x0028b1b0,
                        f: ee::s07::SSXAudioSystem_IsInGame,
                    }
                    .call(c, 0x0028c320)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_28c418;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_28c418;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0028c33c);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x002b49e0, 0x0028c33c)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_28c418;
                            } else {
                                'b_28c368: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0028c358);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x002b4af0, 0x0028c358)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x12d;
                                    if t {
                                        break 'b_28c368;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S2][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_28c418;
                                        } else {
                                            break 'b_28c368;
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[RA][0] = sx(0x0028c370);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0028d488,
                                    f: ee::s07::SSXAudioSystem_SelectRandomSong,
                                }
                                .call(c, 0x0028c370)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x0028c388);
                                c.g[T0][0] = 0u64;
                                Callee {
                                    addr: 0x0028cf98,
                                    f: ee::s07::SSXAudioSystem_PlayMusic,
                                }
                                .call(c, 0x0028c388)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_28c418;
                                } else {
                                    break 'b_28c390;
                                }
                            }
                        }
                    }
                }
            }
            'b_28c3a4: {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x608c));
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x3;
                if t {
                    break 'b_28c3a4;
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_28c418;
                    } else {
                        break 'b_28c3a4;
                    }
                }
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x0028c3ac);
            Callee {
                addr: 0x0028b1b0,
                f: ee::s07::SSXAudioSystem_IsInGame,
            }
            .call(c, 0x0028c3ac)?;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_28c418;
            } else {
                'b_28c414: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x530));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    if t {
                        break 'b_28c414;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x0028c3c8);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x002b49e0, 0x0028c3c8)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_28c418;
                        } else {
                            'b_28c3f4: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x0028c3e4);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x002b4af0, 0x0028c3e4)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x12d;
                                if t {
                                    break 'b_28c3f4;
                                } else {
                                    c.cycles += 2;
                                    if c.g[S2][0] != c.g[V0][0] {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        break 'b_28c418;
                                    } else {
                                        break 'b_28c3f4;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x0028c3fc);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0028d488,
                                f: ee::s07::SSXAudioSystem_SelectRandomSong,
                            }
                            .call(c, 0x0028c3fc)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0xffffffffffffffff;
                            c.g[RA][0] = sx(0x0028c414);
                            c.g[T0][0] = 0u64;
                            Callee {
                                addr: 0x0028cf98,
                                f: ee::s07::SSXAudioSystem_PlayMusic,
                            }
                            .call(c, 0x0028c414)?;
                            break 'b_28c414;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_28c418;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void MessageCenter_InitGroupTable(void)` (0x1e12f0): fills the message
/// group table at 0x441630 (60 entries of 0x14 bytes): the per-group counts at
/// +8 (6, 11, 17, 7, ... 1, group 0 and the zero groups 0), the +4 word of
/// entry 0 cleared, then a prefix-sum loop that sets each next entry's +4
/// word to the previous entry's +4 plus its count at +8.
pub fn message_center_init_group_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 78;
        c.g[V0][0] = 0x440000;
        c.g[V1][0] = 0x6;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
        c.g[A1][0] = 0x3b;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[V1][0]);
        c.g[V1][0] = 0xb;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x94), c.g[V1][0]);
        c.g[V1][0] = 0x11;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
        c.g[V1][0] = 0x7;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), c.g[V1][0]);
        c.g[V1][0] = 0x6;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
        c.g[V1][0] = 0x8;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), c.g[V1][0]);
        c.g[V1][0] = 0x3;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xf8), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x120), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x134), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x148), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x15c), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x170), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x184), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x198), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), c.g[V1][0]);
        c.g[V1][0] = 0x4;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e8), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x210), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x224), c.g[V1][0]);
        c.g[V1][0] = 0x10;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x238), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x260), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x274), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x288), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), c.g[V1][0]);
        c.g[V1][0] = 0x2;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x328), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2b0), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2c4), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2d8), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2ec), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x300), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x314), c.g[V1][0]);
        c.g[V1][0] = 0x1;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x33c), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x350), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x364), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x378), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x38c), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3a0), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x418), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3b4), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x42c), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3c8), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x440), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3dc), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x454), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3f0), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x468), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x404), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x47c), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x490), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4a4), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4b8), c.g[V1][0]);
        'l_1e1428: loop {
            c.cycles += 7;
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x14));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x10));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[A0][0] as u32, c.g[V1][0]);
            let t = (c.g[A1][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001e1428);
                }
                continue 'l_1e1428;
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `int cTransitionMan_Start(cTransitionMan* self, initiator, transitionIn,
/// transitionOut, useScreenGrab, externalTime, float inTime, float holdTime,
/// float outTime)`
/// (0x2e4370): ends a transition already playing, stores the initiator and
/// external time pointer, and with `useScreenGrab` creates the screen grab
/// texture (256x256) through the graphics manager (when that fails the
/// transition objects are told to abort through their vtables and 0 is
/// returned); otherwise records the in/out transition objects and times,
/// marks the transition playing and returns 1.
pub fn c_transition_man_start(c: &mut Cpu) {
    guest(c, |c| {
        'b_2e43d0: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[T1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A2][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[T0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S4][0] = c.g[A3][0];
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S5][0] = c.g[A1][0];
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x78));
            c.mov_s(22, 14);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.mov_s(21, 13);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
            let t = c.g[V0][0] == 0u64;
            c.mov_s(20, 12);
            if t {
                break 'b_2e43d0;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002e43d0);
                Callee {
                    addr: 0x002e4578,
                    f: ee::s09::cTransitionMan_Stop,
                }
                .call(c, 0x002e43d0)?;
                break 'b_2e43d0;
            }
        }
        'b_2e4498: {
            'b_2e446c: {
                'b_2e4468: {
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[S5][0]);
                    let t = c.g[S3][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                    if t {
                        break 'b_2e4468;
                    } else {
                        'b_2e43e8: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                            if c.g[V0][0] != 0u64 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                                break 'b_2e43e8;
                            } else {
                                break 'b_2e43e8;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                        c.g[V0][0] = !(0u64 | c.g[V0][0]);
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x68));
                        if t {
                            break 'b_2e4468;
                        } else {
                            c.cycles += 12;
                            c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                            c.g[A2][0] = 0x100;
                            c.g[A3][0] = 0x100;
                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x1d8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1dc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002e4418);
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x002e4418)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_2e446c;
                            } else {
                                'b_2e4444: {
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[A1][0] = 0x3;
                                    if t {
                                        break 'b_2e4444;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x002e4444);
                                        c.g[A0][0] =
                                            sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x002e4444)?;
                                        break 'b_2e4444;
                                    }
                                }
                                'b_2e4460: {
                                    c.cycles += 2;
                                    let t = c.g[S4][0] == 0u64;
                                    c.g[A1][0] = 0x3;
                                    if t {
                                        break 'b_2e4460;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x002e4460);
                                        c.g[A0][0] =
                                            sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x002e4460)?;
                                        break 'b_2e4460;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_2e4498;
                                } else {
                                    break 'b_2e4468;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_2e446c;
            }
            'b_2e4494: {
                c.cycles += 9;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[S2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x54));
                c.swc1(21, (c.g[S0][0] as u32).wrapping_add(0x58));
                c.swc1(22, (c.g[S0][0] as u32).wrapping_add(0x5c));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                let t = c.g[S3][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[S3][0]);
                if t {
                    break 'b_2e4494;
                } else {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                    break 'b_2e4494;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_2e4498;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x78));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `cBankManager* SndBankMan_ctor(cBankManager* this, int embedded, int count, heap)`
/// (0x28b9b8, frame 0x90): with `embedded` set, builds the sound system
/// sub-object at +0xad0 (and points +0x1d8 at it); runs the load queue and
/// bank monitor ctors, sets the vtables (0x483ab8, the copy to the stack when
/// not embedded), stores `count` at +0xac8, allocates `count` 0x60 byte
/// `BankInstances` banks (constructing each) and publishes `this` in the
/// global at gp+0x524. Returns `this`.
pub fn snd_bank_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_28ba00: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A3][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A1][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[A2][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            let t = c.g[S1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_28ba00;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xad0));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0028ba00);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1d8), c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x0028ba00)?;
                break 'b_28ba00;
            }
        }
        'b_28ba6c: {
            c.cycles += 11;
            c.g[RA][0] = sx(0x0028ba08);
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x0028a058,
                f: ee::s07::SndLoadQueue_ctor,
            }
            .call(c, 0x0028ba08)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1d8));
            c.g[RA][0] = sx(0x0028ba18);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002acd08, 0x0028ba18)?;
            c.g[V1][0] = 0x480000;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1d8));
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3ab8));
            let t = c.g[S1][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
            if t {
                break 'b_28ba6c;
            } else {
                c.cycles += 16;
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x3ab8));
                c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1d8));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd(c.g[SP][0] as u32, c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1d8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xad0));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                break 'b_28ba6c;
            }
        }
        'b_28badc: {
            c.cycles += 21;
            c.g[A0][0] = 0x60;
            c.g[V0][0] = 0x480000;
            c.mult(0, 4, 20, 4);
            c.g[V1][0] = 0x480000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b48));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3ad8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0xac8), c.g[S4][0]);
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S2][0] as u32).wrapping_add(0xab0), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x29b8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x1d4), c.g[V1][0]);
            c.g[A0][0] = c.g[A0][0] | 0x10;
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x0028baa8);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0028baa8)?;
            c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0xacc));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V0][0] as u32, c.g[S4][0]);
            let t = c.g[S4][0] == 0u64;
            c.g[S0][0] = c.g[S3][0];
            if t {
                break 'b_28badc;
            } else {
                c.cycles += 2;
                c.g[S4][0] = 0xffffffffffffffff;
                'l_28bac8: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0028bad4);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x0028b248,
                        f: ee::s07::SndBank_ctor,
                    }
                    .call(c, 0x0028bad4)?;
                    let t = c.g[S1][0] != c.g[S4][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0028bac8);
                        }
                        continue 'l_28bac8;
                    } else {
                        break 'b_28badc;
                    }
                }
            }
        }
        c.cycles += 12;
        c.sw(c.g[S5][0] as u32, c.g[S3][0]);
        c.g[V0][0] = c.g[S2][0];
        c.sw((c.g[GP][0] as u32).wrapping_add(0x524), c.g[S2][0]);
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
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (clear of the fakes' call log at
    /// `DATA + 0x10_0000` and pointer range `DATA .. DATA + 0x4_0000`); the
    /// word after it is the per-seed mask that picks which result-altering
    /// fake calls return 0 / -1.
    const OC782_CTRL: u32 = DATA + 0x2c_0000;
    const OC782_MAGIC: u32 = 0x4f43_3738 + 2;
    const OC782_OBJ: u32 = DATA + 0x2e_0000;

    /// Gated fakes for the callees that have an override slot. `mode` 1: a
    /// call whose bit (`log index & 15`) is set in the mask word returns 0;
    /// mode 2: returns -1 (the fakes alone return non-zero pointers).
    macro_rules! oc782_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mode:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC782_CTRL) != OC782_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let n = c.bus.read32(fakes::LOG_COUNT);
                    fakes::fake_call(c, $addr);
                    if $mode != 0
                        && c.pc >> 16 != 0xdead
                        && c.bus.read32(OC782_CTRL + 4) >> (n & 15) & 1 != 0
                    {
                        c.g[V0][0] = if $mode == 1 { 0 } else { !0u64 };
                    }
                }
            )*
            fn oc782_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc782_fakes!(
        (
            oc782_fake_0014dc80,
            OC782_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            0
        ),
        (
            oc782_fake_0014dd58,
            OC782_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            0
        ),
        (
            oc782_fake_00152ba8,
            OC782_PREV_00152BA8,
            ee::s01::OVERRIDE_00152BA8,
            ee::s01::f_00152ba8__generated,
            0x0015_2ba8,
            0
        ),
        (
            oc782_fake_0014ad28,
            OC782_PREV_0014AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28,
            0
        ),
        (
            oc782_fake_0014bdb8,
            OC782_PREV_0014BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8,
            0
        ),
        (
            oc782_fake_0014d988,
            OC782_PREV_0014D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014_d988,
            0
        ),
        (
            oc782_fake_0014d998,
            OC782_PREV_0014D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014_d998,
            0
        ),
        (
            oc782_fake_0019cce8,
            OC782_PREV_0019CCE8,
            ee::s03::OVERRIDE_0019CCE8,
            ee::s03::f_0019cce8__generated,
            0x0019_cce8,
            2
        ),
        (
            oc782_fake_0019dc20,
            OC782_PREV_0019DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20,
            0
        ),
        (
            oc782_fake_0019cd38,
            OC782_PREV_0019CD38,
            ee::s03::OVERRIDE_0019CD38,
            ee::s03::f_0019cd38__generated,
            0x0019_cd38,
            0
        ),
        (
            oc782_fake_001d9308,
            OC782_PREV_001D9308,
            ee::s04::OVERRIDE_001D9308,
            ee::s04::f_001d9308__generated,
            0x001d_9308,
            0
        ),
        (
            oc782_fake_00227f80,
            OC782_PREV_00227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80,
            0
        ),
        (
            oc782_fake_0023c8f0,
            OC782_PREV_0023C8F0,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0,
            0
        ),
        (
            oc782_fake_0023caa0,
            OC782_PREV_0023CAA0,
            ee::s06::OVERRIDE_0023CAA0,
            ee::s06::f_0023caa0__generated,
            0x0023_caa0,
            0
        ),
        (
            oc782_fake_00241aa0,
            OC782_PREV_00241AA0,
            ee::s06::OVERRIDE_00241AA0,
            ee::s06::f_00241aa0__generated,
            0x0024_1aa0,
            0
        ),
        (
            oc782_fake_00241d40,
            OC782_PREV_00241D40,
            ee::s06::OVERRIDE_00241D40,
            ee::s06::f_00241d40__generated,
            0x0024_1d40,
            0
        ),
        (
            oc782_fake_002c2540,
            OC782_PREV_002C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540,
            0
        ),
        (
            oc782_fake_002c26d0,
            OC782_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0,
            0
        ),
        (
            oc782_fake_002b4070,
            OC782_PREV_002B4070,
            ee::s08::OVERRIDE_002B4070,
            ee::s08::f_002b4070__generated,
            0x002b_4070,
            0
        ),
        (
            oc782_fake_002b49e0,
            OC782_PREV_002B49E0,
            ee::s08::OVERRIDE_002B49E0,
            ee::s08::f_002b49e0__generated,
            0x002b_49e0,
            0
        ),
        (
            oc782_fake_002b4af0,
            OC782_PREV_002B4AF0,
            ee::s08::OVERRIDE_002B4AF0,
            ee::s08::f_002b4af0__generated,
            0x002b_4af0,
            0
        ),
        (
            oc782_fake_002ade88,
            OC782_PREV_002ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88,
            0
        ),
        (
            oc782_fake_002e4578,
            OC782_PREV_002E4578,
            ee::s09::OVERRIDE_002E4578,
            ee::s09::f_002e4578__generated,
            0x002e_4578,
            0
        ),
        (
            oc782_fake_00317670,
            OC782_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            0
        ),
        (
            oc782_fake_00317e30,
            OC782_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30,
            0
        ),
        (
            oc782_fake_00397140,
            OC782_PREV_00397140,
            ee::s13::OVERRIDE_00397140,
            ee::s13::f_00397140__generated,
            0x0039_7140,
            1
        ),
        (
            oc782_fake_0039d860,
            OC782_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            1
        ),
        (
            oc782_fake_0039fcc8,
            OC782_PREV_0039FCC8,
            ee::s13::OVERRIDE_0039FCC8,
            ee::s13::f_0039fcc8__generated,
            0x0039_fcc8,
            0
        ),
    );

    /// Scrambled machine with the fakes on, the gate word and per-seed mask,
    /// the BELibrary dummy (gp-0x1f04) and a world (gp-0x848) with the
    /// in-game flag word (+0x84) and a random +0x7c.
    fn oc782_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc782_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC782_CTRL, OC782_MAGIC);
        c.bus.write32(OC782_CTRL + 4, r.next_u32());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), OC782_OBJ + 0x8000);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x848), OC782_OBJ + 0x9000);
        c.bus
            .write32(OC782_OBJ + 0x9000 + 0x7c, r.next_u32() & 0xfff0);
        c.bus
            .write32(OC782_OBJ + 0x9000 + 0x84, [0, 1, 2][(seed % 3) as usize]);
        r
    }

    /// Forces the result of the `n`th fake call.
    fn oc782_force(c: &mut Cpu, n: u32, value: u32) {
        c.bus.write32(fakes::LOG_RESULTS + 4 * n, value);
    }

    /// `cFEStateMemCard_StartProfileSearch`: memory card manager (a forced
    /// pointer) with a vtable entry for the state call, fakes for the rest;
    /// the UI object and animation lookups return 0 for some calls (mask).
    #[test]
    fn fe_state_mem_card_start_profile_search_matches() {
        fakes::covered(
            ee::s02::f_001877b0__generated,
            fe_state_mem_card_start_profile_search,
            96,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782a, 17);
                let inst = OC782_OBJ + 0x1000;
                let mgr = OC782_OBJ + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x1c38), inst);
                c.bus.write32(GAME_GP.wrapping_sub(0x1c3c), r.next_u32());
                c.bus.write32(inst + 0x10, r.next_u32() & 0xfffff0);
                c.bus.write32(inst + 0x40, r.next_u32() & 0xfffff0);
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(mgr + 0x748, vt);
                oc782_force(c, 2, mgr);
            },
        );
    }

    /// `FEModule_RiderPreviewReleaseBoltTextures`: profile index / character
    /// id valid or negative, bolt part entry lists of 1..3 entries (types 6 and
    /// others, slots into the profile's bolt map), texture slots found or not
    /// (the slot lookup returns -1 for some calls).
    #[test]
    fn fe_module_rider_preview_release_bolt_textures_matches() {
        fakes::covered(
            ee::s03::f_0019fa78__generated,
            fe_module_rider_preview_release_bolt_textures,
            128,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782b, 14);
                let this = OC782_OBJ + 0x1000;
                let list = OC782_OBJ + 0x2000;
                let map = OC782_OBJ + 0x3000;
                let ents = OC782_OBJ + 0x4000;
                let neg = |r: &mut Rng| {
                    if r.next_u32() % 5 == 0 {
                        0xffff_fff0
                    } else {
                        r.next_u32() & 0xf
                    }
                };
                let profile = neg(&mut r);
                let chr = neg(&mut r);
                c.bus.write32(this, chr);
                c.bus.write32(this + 4, profile);
                c.bus.write32(this + 0xcb8, r.next_u32());
                c.bus.write32(list + 0x288, map);
                let count = 1 + r.next_u32() % 3;
                for i in 0..4u32 {
                    let e = ents + 0x38 * i;
                    c.bus
                        .write8(e + 3, if r.next_u32() % 2 == 0 { 6 } else { 2 });
                    c.bus.write16(e + 4, i as u16);
                    let v = if r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        (r.next_u32() % 4) as u16
                    };
                    c.bus.write16(map + 2 * i, v);
                }
                let with_list = seed % 4 != 0;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if with_list { sx(list) } else { 0 };
                if with_list {
                    oc782_force(c, 3, count);
                    oc782_force(c, 4, ents);
                } else {
                    oc782_force(c, 3, list);
                    oc782_force(c, 4, count);
                    oc782_force(c, 5, ents);
                }
            },
        );
    }

    /// `SSXAudioSystem_SetRequestPlaylist`: same / changed playlist, radio
    /// modes 0..5, in game or not, with or without the music system, state
    /// callee results in the stop range or not. The branch into
    /// `SelectRandomSong`/`PlayMusic` (unported, deep callees) is not
    /// reached: the faked state results never ask for it.
    #[test]
    fn ssx_audio_system_set_request_playlist_matches() {
        fakes::covered(
            ee::s07::f_0028c2d0__generated,
            ssx_audio_system_set_request_playlist,
            128,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782c, 3);
                let sys = OC782_OBJ + 0x1_0000;
                let key = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.bus
                    .write64(sys + 0x6098, if seed % 2 == 0 { key } else { !key });
                c.bus.write32(
                    sys + 0x608c,
                    [1, 3, 0, 2, 5, 1, 3, 4][(seed / 2 % 8) as usize],
                );
                c.bus.write32(
                    sys + 0x530,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(sys + 0x118, r.next_u32());
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = key;
                // The state call (first or second fake call) is in 1..3 for some seeds.
                let idx = if seed % 2 == 0 { 0 } else { 1 };
                if seed % 3 == 0 {
                    oc782_force(c, idx, 2);
                }
            },
        );
    }

    /// `MessageCenter_InitGroupTable`: the table is overwritten from random
    /// contents.
    #[test]
    fn message_center_init_group_table_matches() {
        check(
            ee::s04::f_001e12f0__generated,
            message_center_init_group_table,
            8,
            &|c, seed| {
                let mut r = Rng(0x782d_0000 ^ seed as u64);
                for i in 0..(0x4c8u32 / 4 + 4) {
                    c.bus.write32(0x0044_1630 + 4 * i, r.next_u32());
                }
                c.g[RA][0] = sx(crate::difftest::RA);
            },
        );
    }

    /// `cTransitionMan_Start`: a transition already playing (calls Stop) or
    /// not, grab texture present (id != -1) or not (creates it through the
    /// graphics manager's vtable, which never succeeds here, so the in/out
    /// transition objects are told to abort), with and without in/out objects,
    /// screen grab on or off.
    #[test]
    fn c_transition_man_start_matches() {
        fakes::covered(
            ee::s09::f_002e4370__generated,
            c_transition_man_start,
            128,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782e, 4);
                let this = OC782_OBJ + 0x1000;
                let gman = OC782_OBJ + 0x2000;
                c.bus.write32(
                    this + 0x44,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(this + 0x7c, r.next_u32());
                c.bus.write32(
                    this + 0x6c,
                    if seed / 2 % 2 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() & 0xfff
                    },
                );
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), gman);
                let vt = fakes::vtable(c, &mut r, &[0x1d8]);
                c.bus.write32(gman + 0x10d8, vt);
                let tin = if seed / 4 % 2 == 0 {
                    0
                } else {
                    OC782_OBJ + 0x4000
                };
                let tout = if seed / 8 % 2 == 0 {
                    0
                } else {
                    OC782_OBJ + 0x5000
                };
                let vin = fakes::vtable(c, &mut r, &[8]);
                let vout = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(OC782_OBJ + 0x4000, vin);
                c.bus.write32(OC782_OBJ + 0x5000, vout);
                c.g[A0][0] = sx(this);
                c.g[A2][0] = sx(tin);
                c.g[A3][0] = sx(tout);
                c.g[T0][0] = if seed / 16 % 3 == 0 { 0 } else { 1 };
            },
        );
    }

    /// `SndBankMan_ctor`: embedded or not, 0..4 banks. The load queue, bank
    /// monitor and bank ctors run for real; the allocator and the sound
    /// manager reference ctor are gated fakes (some unwind).
    #[test]
    fn snd_bank_man_ctor_matches() {
        fakes::covered(
            ee::s07::f_0028b9b8__generated,
            snd_bank_man_ctor,
            64,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782f, 8);
                let this = OC782_OBJ + 0x1_0000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = [0u64, 1, 2, 4][(seed / 2 % 4) as usize];
                c.g[A3][0] = sx(r.next_u32() & 0xff);
            },
        );
    }
}
