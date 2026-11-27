//! Staging module oc992 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc992`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_85b8,
        ovscreenmap_on_widget_action,
        "OVScreenMap_OnWidgetAction",
    ),
    (0x0020_8658, ovscreenmap_post_input, "OVScreenMap_PostInput"),
    (0x0020_86a8, ovscreensession_init, "OVScreenSession_Init"),
    (
        0x0020_9fd0,
        ovscreensession_hide_indicators,
        "OVScreenSession_HideIndicators",
    ),
    (
        0x0020_a4e0,
        ovscreen_final_standings_recv_chat_message,
        "OVScreenFinalStandings_RecvChatMessage",
    ),
    (
        0x0020_a778,
        ovscreen_pda_template_init,
        "OVScreenPDATemplate_Init",
    ),
    (0x0020_ca10, gameui_open_pause_menu, "GameUI_OpenPauseMenu"),
    (
        0x0020_dbd0,
        ovscreen_yn_dialog_quit_game_update,
        "OVScreenYNDialogQuitGame_Update",
    ),
];

/// `obj->vtbl` method call of the generated bodies: `V0 = *(obj + 8)`,
/// `A0 = obj + *(short *)(V0 + 0x48)`, then `jalr *(V0 + 0x4c)` returning to
/// `ret`. The caller charges its own cycles and sets `A1`.
fn call_method(c: &mut Cpu, obj: usize, ret: u32) -> Flow {
    c.g[V0][0] = Addr((c.g[obj][0] as u32).wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `OVScreenMap_OnWidgetAction` (0x2085b8): the map screen's widget action.
/// Runs `EventSelect_OnWidgetAction` on the screen's event block (screen +0x9c).
/// When that reports 1 (handled) the screen's flags at +0x1c lose their 0x3f00
/// field and gain 0x780; otherwise the child at +0x10 gets a table call with
/// `+0x18` as its argument.
pub fn ovscreenmap_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
        Callee {
            addr: 0x0020_18a8,
            f: ee::s05::f_002018a8,
        }
        .call(c, 0x0020_85d0)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 10;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f190, 0x0020_85ec)?;
            let flags = (c.g[S0][0] as u32).wrapping_add(0x1c);
            c.g[V1][0] = Addr(flags).lw(c);
            c.g[V0][0] = !0x3f00u64;
            c.g[V1][0] &= c.g[V0][0];
            c.g[V1][0] |= 0x780;
            Addr(flags).sw(c, c.g[V1][0]);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenMap_PostInput` (0x208658): forwards the post-input event of the
/// map screen (screen +0x9c) to `EventSelect_PostInput`.
pub fn ovscreenmap_post_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c));
        Callee {
            addr: 0x0020_2770,
            f: ee::s05::f_00202770,
        }
        .call(c, 0x0020_8668)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenSession_Init` (0x2086a8): builds the map session's screen state
/// (the map picture, the player and map indicators) on the session `s1`.
/// Stores the picture handle at +0xd4 and the marker handle at +0xd0, clears
/// the list slots at +0xe0..+0x1a4 and runs the four setup calls at the end.
pub fn ovscreensession_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);

        // Block A: the picture handle from the table; a non-null handle gets a
        // call through the table.
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_add(0x17f8));
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_86cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_86e0)?;
        let t = c.g[V0][0] == 0;
        Addr((c.g[S1][0] as u32).wrapping_add(0xd4)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x0020_86f8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_86f8)?;
        }

        // Block B: the map-image setup (`cOVStateOverlay_Init`, then the table).
        c.cycles += 22;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_a380,
            f: ee::s05::f_0020a380,
        }
        .call(c, 0x0020_8700)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11c)).lw(c);
        call_ptr(c, 0x001a_3bf0, 0x0020_870c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[A0][0] = 0x004a_0000;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5508));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25a8));
        Addr((c.g[S1][0] as u32).wrapping_add(0xdc)).sw(c, 0);
        Addr((c.g[S1][0] as u32).wrapping_add(0xbc)).sw(c, 0);
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x1cc));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x1d0));
        Addr((c.g[S1][0] as u32).wrapping_add(0xc4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0020_8738)?;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xd4)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_8744)?;
        c.g[A2][0] = c.g[V0][0];
        let t = c.g[A2][0] == 0;
        Addr((c.g[S1][0] as u32).wrapping_add(0xd0)).sw(c, c.g[A2][0]);
        if !t {
            c.cycles += 6;
            c.g[A1][0] = 0;
            call_method(c, A2, 0x0020_8768)?;
        }

        // The list slots: seven pointer-sized cells cleared in lock step.
        c.cycles += 8;
        Addr((c.g[S1][0] as u32).wrapping_add(0xc0)).sw(c, 0);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1a4));
        Addr((c.g[S1][0] as u32).wrapping_add(0xe0)).sw(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x184));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x104));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xe4));
        c.g[A2][0] = 7;
        loop {
            c.cycles += 10;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            Addr(c.g[V1][0] as u32).sw(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            Addr(c.g[A0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            Addr(c.g[A1][0] as u32).sw(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            let t = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_8788);
            }
        }

        // Done: the four setup calls, each on the session.
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_8f10,
            f: ee::s05::f_00208f10,
        }
        .call(c, 0x0020_87b8)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_8f88,
            f: ee::s05::f_00208f88,
        }
        .call(c, 0x0020_87c0)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_96a8,
            f: ee::s05::f_002096a8,
        }
        .call(c, 0x0020_87c8)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0020_9970,
            f: ee::s05::f_00209970,
        }
        .call(c, 0x0020_87d0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenSession_HideIndicators` (0x209fd0): calls the hide method of the
/// session's child at +0xe0, then the hide method of each entry of the list
/// at +0xe4 (count at +0xc0; entries from +0xe4 and +0x104).
pub fn ovscreensession_hide_indicators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xe0)).lw(c);
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 5;
            call_method(c, A2, 0x0020_a008)?;
        }

        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc0)).lw(c);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !t {
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x104));
            loop {
                // The entry before the cursor (+0xe4 at the first step).
                c.cycles += 3;
                c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x20)).lw(c);
                let t = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !t {
                    c.cycles += 5;
                    call_method(c, A2, 0x0020_a038)?;
                }
                // The entry at the cursor.
                c.cycles += 3;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                let t = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !t {
                    c.cycles += 5;
                    call_method(c, A2, 0x0020_a058)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc0)).lw(c);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_a018);
                }
            }
        }

        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenFinalStandings_RecvChatMessage` (0x20a4e0): receives a chat line
/// on the final standings screen. While the chat's sound/popup state is set
/// (global 0x534b30) it hides the old popup, builds the new one from the
/// message text (+0x58) and the widget at +0x40, and lays it out.
pub fn ovscreen_final_standings_recv_chat_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A1][0] = 7;
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        frame.save(c, S2, 0x60);
        c.g[S6][0] = 0;
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
        call_ptr(c, 0x0014_dd58, 0x0020_a51c)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4b30)).lw(c);
        let t = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));

        'exit: {
            'b67c: {
                if t {
                    break 'b67c;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t {
                    break 'b67c;
                }
                c.cycles += 2;
                let t = c.g[A0][0] == 0;
                c.g[V0][0] = c.g[S6][0];
                if t {
                    break 'exit;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                if t {
                    break 'b67c;
                }
                c.cycles += 5;
                c.g[A2][0] = 0x41;
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0025_8160, 0x0020_a558)?;
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = 0x0047_0000;
                if t {
                    break 'b67c;
                }

                // The old popup is hidden (0x46e660 table entry) and its
                // widget at +0x40 is re-laid out when present.
                'b594: {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19a0));
                    call_ptr(c, 0x0031_7670, 0x0020_a568)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0020_a574)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if t {
                        break 'b594;
                    }
                    c.cycles += 5;
                    call_method(c, A2, 0x0020_a594)?;
                }

                // The chat line's popup is hidden the same way (0x46e7f8).
                'b5d4: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x0047_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1808));
                    call_ptr(c, 0x0031_7670, 0x0020_a5a0)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0020_a5ac)?;
                    c.g[A2][0] = c.g[V0][0];
                    if c.g[A2][0] == 0 {
                        c.g[A0][0] = 0x0047_0000;
                        break 'b5d4;
                    }
                    c.cycles += 7;
                    c.g[A1][0] = 0;
                    call_method(c, A2, 0x0020_a5d0)?;
                    c.g[A0][0] = 0x0047_0000;
                }

                // The new message: state flag, the text table, the layout.
                c.cycles += 42;
                c.g[S6][0] = 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17f8));
                call_ptr(c, 0x0031_7670, 0x0020_a5e0)?;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0020_a5ec)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x498));
                c.g[A0][0] = c.g[SP][0];
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc8)).lhu(c);
                c.g[S2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc4)).lhu(c);
                c.g[S3][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc6)).lhu(c);
                c.g[S4][0] = (c.g[S1][0] < 1) as u64;
                call_ptr(c, 0x0031_82d8, 0x0020_a60c)?;
                c.g[A1][0] = 0x004a_0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2710));
                call_ptr(c, 0x0031_8910, 0x0020_a61c)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0031_8910, 0x0020_a628)?;
                c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_19f8, 0x0020_a634)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S2][0];
                c.g[T1][0] = c.g[S3][0];
                c.g[T2][0] = c.g[S4][0];
                c.g[T3][0] = 1;
                c.g[A1][0] = 0x28;
                c.g[A2][0] = 4;
                call_ptr(c, 0x001a_83d8, 0x0020_a658)?;
                c.g[A1][0] = 1;
                call_method(c, S0, 0x0020_a670)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 2;
                call_ptr(c, 0x0031_8260, 0x0020_a67c)?;
                break 'b67c;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S6][0];
        }

        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenPDATemplate_Init` (0x20a778): fills the PDA template screen `s1`
/// (its picture handle at +0x40, its list at +0x10) and lays out its rows,
/// each row sized by the template's widget at +0xb0 and the division of the
/// world's counter by 0xe10 and 0xf.
pub fn ovscreen_pda_template_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x29;
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save(c, S0, 0x50);

        // Block A: the title picture; a non-null handle is shown.
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_add(0x1a10));
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        Addr((c.g[S1][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0020_a7a4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_a7b8)?;
        let t = c.g[V0][0] == 0;
        Addr((c.g[S1][0] as u32).wrapping_add(0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_a7d0)?;
        }

        // Block B: the subtitle picture.
        c.cycles += 11;
        c.g[A0][0] = 0x004a_0000;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2718));
        call_ptr(c, 0x0031_7670, 0x0020_a7e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_a7f4)?;
        let t = c.g[V0][0] == 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_c870, 0x0020_a808)?;
        }

        // Block C: the row list; each row's widget is resolved by the table and
        // laid out by its own method.
        c.cycles += 9;
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a20));
        call_ptr(c, 0x0031_7670, 0x0020_a814)?;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_a820)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] == 0;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if !t {
            c.cycles += 10;
            c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xb0)).lw(c);
            c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_a848)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            if !t {
                // Block 860: the row height.
                c.cycles += 3;
                c.g[V1][0] = 0xe10;
                c.cycles += 14;
                c.g[T0][0] = 0xf;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[T1][0] = (-10i64) as u64;
                c.g[A0][0] = c.g[SP][0];
                c.div(0, 2, 3);
                c.g[A3][0] = 0x43;
                c.g[V0][0] = c.lo[0];
                c.div(0, 2, 8);
                c.g[A2][0] = c.hi[0];
                c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                call_ptr(c, 0x002c_26d0, 0x0020_a88c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a_0e90, 0x0020_a898)?;
            }
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `GameUI_OpenPauseMenu` (0x20ca10): opens the pause menu's screens in
/// order: the game-state screen (6), the list screen (2 or 1 or the
/// selection-dependent screen), and the screens 3, 4 and 5 by the player's
/// state, leaving through the saved return address.
pub fn gameui_open_pause_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        'outer: {
            'p1: {
                call_ptr(c, 0x0014_dc80, 0x0020_ca20)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_dd58, 0x0020_ca2c)?;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[A1][0] = 7;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x78)).lw(c);
                call_ptr(c, 0x0014_dd58, 0x0020_ca3c)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4b30)).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t {
                    break 'p1;
                }
                'q: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    if t {
                        break 'q;
                    }
                    'r: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                        let t = c.g[V0][0] == 0;
                        if t {
                            break 'r;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x224)).lw(c);
                        let t = c.g[V0][0] == 0;
                        if t {
                            break 'r;
                        }
                        c.cycles += 3;
                        call_ptr(c, 0x0025_8c00, 0x0020_ca74)?;
                        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    }
                    c.cycles += 7;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0025_8790, 0x0020_ca80)?;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A0][0] = 0xe10;
                    c.g[V1][0] = 0x708;
                    Addr((c.g[V0][0] as u32).wrapping_add(0x64)).sw(c, c.g[V1][0]);
                    Addr((c.g[V0][0] as u32).wrapping_add(0x68)).sw(c, c.g[A0][0]);
                }
                c.cycles += 4;
                c.g[A0][0] = 6;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_ca9c)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }

            // Screen 4 when the list at +0xce8 reports a selection.
            'sblk: {
                c.cycles += 4;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
                call_ptr(c, 0x0030_b8c0, 0x0020_caac)?;
                let t = c.g[V0][0] == 0;
                c.g[V1][0] = 0x0053_0000;
                if t {
                    break 'sblk;
                }
                c.cycles += 4;
                c.g[A0][0] = 2;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_cabc)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }

            // Screens 0, 1 and 3 by the state of the record at 0x535bc8.
            'tblk: {
                'ublk: {
                    c.cycles += 5;
                    c.g[A2][0] = c.g[V1][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x49)).lb(c);
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    if t {
                        break 'ublk;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x48)).lb(c);
                    c.g[V0][0] ^= 4;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0xb;
                    if t {
                        break 'tblk;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 1;
                    Callee {
                        addr: 0x0020_ab50,
                        f: ee::s05::f_0020ab50,
                    }
                    .call(c, 0x0020_caf0)?;
                    frame.restore_ra(c, 0);
                    break 'outer;
                }
                c.cycles += 1;
                c.g[A1][0] = 0xb;
            }

            // Screen 4 when the state record's word at +0x214 differs.
            'wblk: {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x214)).lw(c);
                let t = c.g[V1][0] != c.g[A1][0];
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5bc8));
                if t {
                    break 'wblk;
                }
                c.cycles += 4;
                c.g[A0][0] = 4;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_cb14)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }

            // Screen 3 when the record's byte +0x48 is 4.
            'xblk: {
                c.cycles += 4;
                c.g[A0][0] = 4;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lb(c);
                let t = c.g[V1][0] != c.g[A0][0];
                if t {
                    break 'xblk;
                }
                c.cycles += 4;
                c.g[A0][0] = 3;
                Callee {
                    addr: 0x0020_ab50,
                    f: ee::s05::f_0020ab50,
                }
                .call(c, 0x0020_cb34)?;
                frame.restore_ra(c, 0);
                break 'outer;
            }
            c.cycles += 3;
            c.g[A0][0] = 5;
            Callee {
                addr: 0x0020_ab50,
                f: ee::s05::f_0020ab50,
            }
            .call(c, 0x0020_cb44)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        Addr((c.g[GP][0] as u32).wrapping_sub(0xc88)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenYNDialogQuitGame_Update` (0x20dbd0): per-frame update of the quit
/// confirmation dialog. Updates the screen, then (while the global dialog
/// pointer at gp-0x204 is set and its flags allow it) refreshes the dialog's
/// text and its widget at +0x10.
pub fn ovscreen_yn_dialog_quit_game_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0020_e900,
            f: ee::s05::f_0020e900,
        }
        .call(c, 0x0020_dbe4)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        if c.g[V1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            'dc0c: {
                'dc08: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x68)).lw(c);
                    let t = c.g[V0][0] == 0;
                    c.g[A0][0] = 0;
                    if t {
                        break 'dc08;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x64)).lw(c);
                    let t = c.g[V0][0] != 0;
                    if t {
                        break 'dc0c;
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = 1;
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                'dc48: {
                    c.cycles += 10;
                    c.g[A0][0] = sx((0x0047_0000u32).wrapping_sub(0x1fb0));
                    call_ptr(c, 0x0031_7670, 0x0020_dc20)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_c778, 0x0020_dc2c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[A1][0] &= 0xffff;
                    if t {
                        break 'dc48;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[RA][0] = sx(0x0020_dc48);
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x0039_c870, 0x0020_dc48)?;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x0020_dc58)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC992_MAGIC` (otherwise the body they replaced runs).
    const OC992_CTRL: u32 = DATA + 0x0030_0000;
    const OC992_MAGIC: u32 = 0x0992_c0de;
    /// Forced results: one word per callee address (`v + 3`, so 0 is `3`).
    const OC992_MAIL: u32 = DATA + 0x0031_0000;
    /// The world (`gp - 0x848` points here).
    const OC992_W: u32 = DATA + 0x0020_0000;
    /// Fixture objects.
    const OC992_O: u32 = DATA + 0x0021_0000;
    const OC992_P: u32 = DATA + 0x0022_0000;
    const OC992_Q: u32 = DATA + 0x0023_0000;
    const OC992_R: u32 = DATA + 0x0023_1000;
    const OC992_S: u32 = DATA + 0x0023_2000;
    /// The class struct the `obj->vtbl` calls read (+0x48 short, +0x4c target).
    const OC992_CLS: u32 = DATA + 0x0024_0000;
    /// Dynamic call targets: outside the table, so the machine's miss hook answers.
    const OC992_TGT: u32 = fakes::FAKE + 0x40;

    fn oc992_mail(addr: u32) -> u32 {
        OC992_MAIL + (addr & 0xfffc)
    }

    /// Force the fake for `addr` to return `v` (0 forces zero).
    fn oc992_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc992_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs like `fakes::fake_call`, then returns the forced
    /// result unless it unwound.
    fn oc992_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc992_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 3) as u64 };
        }
    }

    macro_rules! oc992_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC992_CTRL) != OC992_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc992_fake_call(c, $addr);
                }
            )*
            fn oc992_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    // Direct and table callees of the eight bodies. Each slot is hooked once
    // per process; the fakes answer only in the fixtures of this module.
    oc992_fakes! {
        (oc992_fake_2018a8, OC992_PREV_2018A8, ee::s05::OVERRIDE_002018A8, ee::s05::f_002018a8__generated, 0x0020_18a8),
        (oc992_fake_202770, OC992_PREV_202770, ee::s05::OVERRIDE_00202770, ee::s05::f_00202770__generated, 0x0020_2770),
        (oc992_fake_39f190, OC992_PREV_39F190, ee::s13::OVERRIDE_0039F190, ee::s13::f_0039f190__generated, 0x0039_f190),
        (oc992_fake_317670, OC992_PREV_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (oc992_fake_397cc0, OC992_PREV_397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x0039_7cc0),
        (oc992_fake_39c870, OC992_PREV_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc992_fake_20a380, OC992_PREV_20A380, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated, 0x0020_a380),
        (oc992_fake_1a3bf0, OC992_PREV_1A3BF0, ee::s03::OVERRIDE_001A3BF0, ee::s03::f_001a3bf0__generated, 0x001a_3bf0),
        (oc992_fake_39d860, OC992_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc992_fake_208f10, OC992_PREV_208F10, ee::s05::OVERRIDE_00208F10, ee::s05::f_00208f10__generated, 0x0020_8f10),
        (oc992_fake_208f88, OC992_PREV_208F88, ee::s05::OVERRIDE_00208F88, ee::s05::f_00208f88__generated, 0x0020_8f88),
        (oc992_fake_2096a8, OC992_PREV_2096A8, ee::s05::OVERRIDE_002096A8, ee::s05::f_002096a8__generated, 0x0020_96a8),
        (oc992_fake_209970, OC992_PREV_209970, ee::s05::OVERRIDE_00209970, ee::s05::f_00209970__generated, 0x0020_9970),
        (oc992_fake_14dd58, OC992_PREV_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (oc992_fake_258160, OC992_PREV_258160, ee::s06::OVERRIDE_00258160, ee::s06::f_00258160__generated, 0x0025_8160),
        (oc992_fake_3a19f8, OC992_PREV_3A19F8, ee::s13::OVERRIDE_003A19F8, ee::s13::f_003a19f8__generated, 0x003a_19f8),
        (oc992_fake_1a83d8, OC992_PREV_1A83D8, ee::s03::OVERRIDE_001A83D8, ee::s03::f_001a83d8__generated, 0x001a_83d8),
        (oc992_fake_3182d8, OC992_PREV_3182D8, ee::s10::OVERRIDE_003182D8, ee::s10::f_003182d8__generated, 0x0031_82d8),
        (oc992_fake_318910, OC992_PREV_318910, ee::s10::OVERRIDE_00318910, ee::s10::f_00318910__generated, 0x0031_8910),
        (oc992_fake_318260, OC992_PREV_318260, ee::s10::OVERRIDE_00318260, ee::s10::f_00318260__generated, 0x0031_8260),
        (oc992_fake_20e900, OC992_PREV_20E900, ee::s05::OVERRIDE_0020E900, ee::s05::f_0020e900__generated, 0x0020_e900),
        (oc992_fake_39c778, OC992_PREV_39C778, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039_c778),
        (oc992_fake_2c26d0, OC992_PREV_2C26D0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0),
        (oc992_fake_3a0e90, OC992_PREV_3A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90),
        (oc992_fake_14dc80, OC992_PREV_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (oc992_fake_30b8c0, OC992_PREV_30B8C0, ee::s10::OVERRIDE_0030B8C0, ee::s10::f_0030b8c0__generated, 0x0030_b8c0),
        (oc992_fake_258c00, OC992_PREV_258C00, ee::s06::OVERRIDE_00258C00, ee::s06::f_00258c00__generated, 0x0025_8c00),
        (oc992_fake_258790, OC992_PREV_258790, ee::s06::OVERRIDE_00258790, ee::s06::f_00258790__generated, 0x0025_8790),
        (oc992_fake_20ab50, OC992_PREV_20AB50, ee::s05::OVERRIDE_0020AB50, ee::s05::f_0020ab50__generated, 0x0020_ab50),
    }

    /// Machine with the gate on, the world at `gp - 0x848`, and no forced
    /// results (every fake returns its default pointer).
    fn oc992_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC992_CTRL, OC992_MAGIC);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC992_W);
        r
    }

    /// A class struct for the method calls: short +0x48 and target +0x4c.
    fn oc992_class(c: &mut Cpu, k: u32) {
        c.bus.write16(OC992_CLS + 0x48, (0x10 * k) as u16);
        c.bus.write32(OC992_CLS + 0x4c, OC992_TGT + 0x10 * k);
    }

    #[test]
    fn ovscreenmap_on_widget_action_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_002085b8__generated,
            ovscreenmap_on_widget_action,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_2085);
                c.pc = 0x0020_85b8;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_O + 0x10, OC992_P);
                c.bus.write32(OC992_O + 0x1c, r.next_u32());
                // EventSelect_OnWidgetAction reports 0 or 1.
                oc992_force(c, 0x0020_18a8, seed % 2);
            },
        );
    }

    #[test]
    fn ovscreenmap_post_input_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_00208658__generated,
            ovscreenmap_post_input,
            200,
            &|c, seed| {
                oc992_base(c, seed, 0x0992_8658);
                c.pc = 0x0020_8658;
                c.g[A0][0] = sx(OC992_O);
            },
        );
    }

    #[test]
    fn ovscreensession_init_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_002086a8__generated,
            ovscreensession_init,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_86a8);
                c.pc = 0x0020_86a8;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_O + 0x10, OC992_P);
                // The picture handle and the widget from the table.
                oc992_force(c, 0x0039_7cc0, if seed % 2 == 0 { 0 } else { OC992_Q });
                oc992_force(c, 0x0039_d860, if seed % 3 == 0 { 0 } else { OC992_Q });
                c.bus.write32(OC992_Q + 8, OC992_CLS);
                oc992_class(c, seed % 4);
                c.bus.write32(OC992_W + 0x8c, r.next_u32());
            },
        );
    }

    #[test]
    fn ovscreensession_hide_indicators_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_00209fd0__generated,
            ovscreensession_hide_indicators,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_9fd0);
                c.pc = 0x0020_9fd0;
                c.g[A0][0] = sx(OC992_O);
                oc992_class(c, seed % 4);
                c.bus
                    .write32(OC992_O + 0xe0, if seed % 5 == 0 { 0 } else { OC992_P });
                c.bus.write32(OC992_P + 8, OC992_CLS);
                c.bus.write32(OC992_O + 0xc0, r.next_u32() % 4);
                for j in 0..12u32 {
                    let e = if r.next_u32() % 3 == 0 { 0 } else { OC992_P };
                    c.bus.write32(OC992_O + 0xe4 + 4 * j, e);
                }
            },
        );
    }

    #[test]
    fn ovscreen_final_standings_recv_chat_message_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020a4e0__generated,
            ovscreen_final_standings_recv_chat_message,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_a4e0);
                c.pc = 0x0020_a4e0;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_W + 0x78, OC992_P);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 7 == 0 { 0 } else { OC992_R },
                );
                c.bus
                    .write32(OC992_R, if seed % 5 == 0 { 0 } else { OC992_P });
                c.bus
                    .write32(0x0053_4b30, if seed % 4 == 0 { 0 } else { 1 });
                c.bus
                    .write32(OC992_O + 0x40, if seed % 6 == 0 { 0 } else { OC992_Q });
                c.bus.write32(OC992_O + 0x58, r.next_u32());
                c.bus.write32(OC992_Q + 8, OC992_CLS);
                oc992_class(c, seed % 4);
                oc992_force(c, 0x0025_8160, seed % 3);
                oc992_force(c, 0x0039_d860, if seed % 4 == 1 { 0 } else { OC992_Q });
                c.bus.write32(OC992_Q + 0xc4, r.next_u32() & 0xffff);
                c.bus.write32(OC992_Q + 0xc8, r.next_u32() & 0xffff);
            },
        );
    }

    #[test]
    fn ovscreen_pda_template_init_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020a778__generated,
            ovscreen_pda_template_init,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_a778);
                c.pc = 0x0020_a778;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_O + 0x10, OC992_P);
                oc992_force(c, 0x0039_7cc0, if seed % 2 == 0 { 0 } else { OC992_Q });
                oc992_force(c, 0x0039_d860, if seed % 3 == 0 { 0 } else { OC992_Q });
                c.bus.write32(OC992_Q + 0xb0, r.next_u32());
                c.bus.write32(OC992_Q + 8, OC992_CLS);
                // The row widget's method table (+0x8c of the world, +4, +0x20/+0x24).
                c.bus.write32(OC992_W + 0x8c, OC992_R);
                c.bus.write32(OC992_R + 4, OC992_S);
                c.bus.write16(OC992_S + 0x20, 0x10);
                c.bus.write32(OC992_S + 0x24, OC992_TGT);
                c.bus.write32(OC992_W + 0x1c, r.next_u32());
            },
        );
    }

    #[test]
    fn gameui_open_pause_menu_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020ca10__generated,
            gameui_open_pause_menu,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_ca10);
                c.pc = 0x0020_ca10;
                c.bus.write32(GAME_GP.wrapping_sub(0x204), OC992_O);
                c.bus
                    .write32(0x0053_4b30, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(OC992_W + 0x84, OC992_P);
                c.bus
                    .write32(OC992_P + 0x224, if seed % 2 == 0 { 0 } else { 1 });
                c.bus
                    .write32(OC992_P + 0x214, if seed % 4 == 0 { 0xb } else { 0x4 });
                c.bus.write32(GAME_GP.wrapping_add(0xce8), OC992_Q);
                c.bus.write8(0x0053_5bc8 + 0x49, (seed % 2) as u8);
                c.bus.write8(
                    0x0053_5bc8 + 0x48,
                    if seed % 5 == 0 {
                        4
                    } else {
                        r.next_u32() as u8 | 1
                    },
                );
            },
        );
    }

    #[test]
    fn ovscreen_yn_dialog_quit_game_update_matches() {
        oc992_install_fakes();
        check(
            ee::s05::f_0020dbd0__generated,
            ovscreen_yn_dialog_quit_game_update,
            200,
            &|c, seed| {
                let mut r = oc992_base(c, seed, 0x0992_dbd0);
                c.pc = 0x0020_dbd0;
                c.g[A0][0] = sx(OC992_O);
                c.bus.write32(OC992_O + 0x10, OC992_P);
                c.bus.write32(OC992_O + 0x40, OC992_Q);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 6 == 0 { 0 } else { OC992_R },
                );
                c.bus
                    .write32(OC992_R + 0x68, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(
                    OC992_R + 0x64,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                oc992_force(c, 0x0039_c778, if seed % 2 == 0 { 0xffff } else { 7 });
            },
        );
    }
}
