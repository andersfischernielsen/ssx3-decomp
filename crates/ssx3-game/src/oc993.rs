//! Staging module oc993 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc993`.

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
        0x0020_dc68,
        ov_screen_replay_menu_init,
        "OVScreenReplayMenu_Init",
    ),
    (
        0x0020_e530,
        ov_screen_replay_menu_update,
        "OVScreenReplayMenu_Update",
    ),
    (
        0x0020_e9a0,
        ov_screen_trophy_on_widget_action,
        "OVScreenTrophy_OnWidgetAction",
    ),
    (
        0x0021_0b58,
        ov_screen_read_private_message_transition_in,
        "OVScreenReadPrivateMessage_TransitionIn",
    ),
    (
        0x0021_0bd8,
        ov_screen_read_private_message_set_message_text,
        "OVScreenReadPrivateMessage_SetMessageText",
    ),
    (
        0x0021_0df0,
        ov_screen_read_private_message_on_widget_action,
        "OVScreenReadPrivateMessage_OnWidgetAction",
    ),
    (
        0x0021_1088,
        ov_screen_profile_save_pda_ctor,
        "OVScreenProfileSavePDA_ctor",
    ),
    (
        0x0021_1270,
        ov_screen_profile_save_pda_reset_slot_list,
        "OVScreenProfileSavePDA_ResetSlotList",
    ),
];

// ---------------------------------------------------------------- OVScreenReplayMenu_Init

/// The replay screen object (`s1`) as an address.
fn replay_init_this(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// `GP - off` (the game's globals sit below `gp`).
fn replay_init_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `void cOVState_REPLAY::onCreateScreen(cOVState_REPLAY* this)` (0x20dc68).
/// Resolves the screen's two UI hash names (`0x471a40`, `0x471a40` + the
/// replay tables), creates the replay screen with `cUIEngine_addScreenByHashName`
/// and plays its frame, then sets up the camera name, hides or shows the
/// replay's objects by hash name, and sets the state words at +0x9c..+0xb4
/// from the world's replay record (`world+0x84`, `+0x28`, flag `+0x608`).
pub fn ov_screen_replay_menu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x470000;
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a40));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7670, 0x0020_dc8c)?;
        c.g[A0][0] = c.g[S0][0];
        let name_hash = c.g[V0][0];
        Addr(replay_init_this(c).wrapping_add(0xc)).sw(c, name_hash);
        c.g[S0][0] = Addr(replay_init_this(c).wrapping_add(0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_dc9c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_dcb0)?;
        let screen = c.g[V0][0];
        Addr(replay_init_this(c).wrapping_add(0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_dcc8)?;
        }

        // Camera name, then the objects that show and hide with the screen.
        c.cycles += 15;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0020_dcd0)?;
        Addr(replay_init_this(c).wrapping_add(0xa0)).sw(c, 0);
        c.g[A1][0] = 0x470000;
        Addr(replay_init_this(c).wrapping_add(0x9c)).sw(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ba0));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e8b8, 0x0020_dce8)?;
        c.g[A1][0] = 0x470000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bb0));
        call_ptr(c, 0x0039_e8b8, 0x0020_dcf8)?;
        let flag = Addr(replay_init_this(c).wrapping_add(0xa0)).lw(c);
        c.g[V0][0] = flag;
        c.g[A0][0] = c.g[S1][0];
        let shown_last = if flag == 0 {
            true
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x470000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bc0));
            call_ptr(c, 0x0039_e8b8, 0x0020_dd10)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bd0));
            call_ptr(c, 0x0039_e948, 0x0020_dd20)?;
            c.g[V0][0] = 0xf5;
            false
        };
        if shown_last {
            c.cycles += 8;
            c.g[A1][0] = 0x470000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bc0));
            call_ptr(c, 0x0039_e948, 0x0020_dd34)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bd0));
            call_ptr(c, 0x0039_e8b8, 0x0020_dd44)?;
            c.g[V0][0] = 0xf5;
        }

        // The replay state words.
        c.cycles += 13;
        c.g[V1][0] = 0x1e0;
        Addr(replay_init_this(c).wrapping_add(0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        let v0 = c.g[V0][0];
        Addr(replay_init_this(c).wrapping_add(0xac)).sw(c, v0);
        let v1 = c.g[V1][0];
        Addr(replay_init_this(c).wrapping_add(0xb0)).sw(c, v1);
        Addr(replay_init_this(c).wrapping_add(0xa4)).sw(c, v0);
        call_ptr(c, 0x0020_e6b0, 0x0020_dd64)?;
        c.g[V0][0] = Addr(replay_init_gp(c, 0x848)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x608)).lw(c);
        let no_flag = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if no_flag {
            c.cycles += 1;
            Addr(replay_init_this(c).wrapping_add(0xb4)).sw(c, 0);
        } else {
            c.cycles += 2;
            let v = c.g[V0][0];
            Addr(replay_init_this(c).wrapping_add(0xb4)).sw(c, v);
        }

        // The pending replay's object: its flags get bit 3 set.
        c.cycles += 13;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1be0));
        call_ptr(c, 0x0031_7670, 0x0020_dd94)?;
        c.g[A0][0] = Addr(replay_init_this(c).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_dda0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x90)).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0x8;
        let flags = c.g[V0][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0x90)).sw(c, flags);
        c.g[V1][0] = Addr(replay_init_this(c).wrapping_add(0xb4)).lw(c);
        let no_state = c.g[V1][0] == 0;
        frame.restore(c, S0, 0x20);
        if !no_state {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = 0x470000;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0039_b960, 0x0020_ddcc)?;
                c.g[A0][0] = 0x470000;
            }

            // Look up the second object and, if it exists, run its two
            // vtable methods (the replay's ticker and its end).
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1bf0));
            call_ptr(c, 0x0031_7670, 0x0020_ddd8)?;
            c.g[A0][0] = Addr(replay_init_this(c).wrapping_add(0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0020_dde4)?;
            c.g[S0][0] = c.g[V0][0];
            let no_object = c.g[S0][0] == 0;
            c.g[A1][0] = 0;
            if !no_object {
                c.cycles += 11;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0020_de04)?;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                let method2 = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method2, 0x0020_de1c)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }

        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- OVScreenReplayMenu_Update

/// `void cOVState_REPLAY::onUpdate()` (0x20e530): the replay screen's frame.
/// Runs the base overlay update, advances the replay clock (`+0xa4`, step
/// `+0xa8`, limit `+0xac`/`+0xb0`) and shows or hides the two replay
/// objects by hash name when the clock reaches the limit, writes the
/// clock as a float into the object at `+0x40`, and then, with no replay
/// flag at `+0x9c`, shows or hides the replay tickers and the camera and
/// progress-bar updates.
pub fn ov_screen_replay_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0020_e900, 0x0020_e544)?;

        // The clock: advance by the step, wrap at the limit.
        c.g[V0][0] = Addr(replay_update_s0(c).wrapping_add(0xa4)).lw(c);
        c.g[V1][0] = Addr(replay_update_s0(c).wrapping_add(0xa8)).lw(c);
        c.g[A0][0] = Addr(replay_update_s0(c).wrapping_add(0xac)).lw(c);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let t = c.g[V0][0] == 0;
        let advanced = c.g[V1][0];
        Addr(replay_update_s0(c).wrapping_add(0xa4)).sw(c, advanced);
        let reset = if t {
            true
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(replay_update_s0(c).wrapping_add(0xb0)).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = Addr(replay_update_s0(c).wrapping_add(0xa4)).lw(c);
                false
            } else {
                true
            }
        };
        if reset {
            c.cycles += 3;
            let limit = c.g[A0][0];
            Addr(replay_update_s0(c).wrapping_add(0xa4)).sw(c, limit);
            Addr(replay_update_s0(c).wrapping_add(0xa8)).sw(c, 0);
            c.g[V1][0] = Addr(replay_update_s0(c).wrapping_add(0xa4)).lw(c);
        }

        // The reached-limit handling: show or hide the two replay objects.
        c.cycles += 3;
        c.g[V0][0] = Addr(replay_update_s0(c).wrapping_add(0xac)).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(replay_update_s0(c).wrapping_add(0xb0)).lw(c);
            c.cycles += 2;
            let differs = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = 0x470000;
            if differs {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c30));
                call_ptr(c, 0x0039_e8b8, 0x0020_e5e8)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bb0));
                call_ptr(c, 0x0039_e8b8, 0x0020_e5f8)?;
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c30));
                call_ptr(c, 0x0039_e8b8, 0x0020_e5c4)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bb0));
                call_ptr(c, 0x0039_e948, 0x0020_e5d4)?;
            }
            c.g[A0][0] = 0x4a0000;
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c30));
            call_ptr(c, 0x0039_e948, 0x0020_e598)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1bb0));
            call_ptr(c, 0x0039_e8b8, 0x0020_e5a8)?;
            c.g[A0][0] = 0x4a0000;
        }

        // The object at +0x40 gets the clock as a float at +0x48.
        c.cycles += 23;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2728));
        call_ptr(c, 0x0031_7670, 0x0020_e604)?;
        c.g[A0][0] = Addr(replay_update_s0(c).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_e610)?;
        c.lwc1(0, replay_update_s0(c).wrapping_add(0xa4));
        c.cvt_s_w(0, 0);
        c.ldl(V1, (c.g[V0][0] as u32).wrapping_add(0x4b));
        c.ldr(V1, (c.g[V0][0] as u32).wrapping_add(0x44));
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        c.sdl(V1, sp(c).wrapping_add(0x7));
        c.sdr(V1, sp(c));
        let tail_word = c.g[A0][0];
        Addr(sp(c).wrapping_add(0x8)).sw(c, tail_word);
        c.swc1(0, sp(c).wrapping_add(0x4));
        c.ldl(V1, sp(c).wrapping_add(0x7));
        c.ldr(V1, sp(c));
        c.g[A0][0] = Addr(sp(c).wrapping_add(0x8)).lw(c);
        c.sdl(V1, (c.g[V0][0] as u32).wrapping_add(0x4b));
        c.sdr(V1, (c.g[V0][0] as u32).wrapping_add(0x44));
        let last_word = c.g[A0][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).sw(c, last_word);

        // The replay flag at +0x9c: when clear, hide the ticker; when set,
        // show it and leave.
        c.g[V0][0] = Addr(replay_update_s0(c).wrapping_add(0x9c)).lw(c);
        let no_flag = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if no_flag {
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ba0));
            call_ptr(c, 0x0039_e8b8, 0x0020_e678)?;
            c.g[V0][0] = Addr(replay_update_s0(c).wrapping_add(0x9c)).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0x470000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ba0));
            call_ptr(c, 0x0039_e948, 0x0020_e664)?;
            c.g[V0][0] = Addr(replay_update_s0(c).wrapping_add(0x9c)).lw(c);
        }

        // The camera and the progress bar, after the world's replay check.
        c.cycles += 2;
        let flag_set = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(replay_update_gp(c, 0x848)).lw(c);
        if !flag_set {
            c.cycles += 3;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
            call_ptr(c, 0x0026_fa78, 0x0020_e690)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0020_e6b0, 0x0020_e698)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0020_e818, 0x0020_e6a0)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The replay screen object (`s0`) as an address.
fn replay_update_s0(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

/// `GP - off`.
fn replay_update_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

// ---------------------------------------------------------------- OVScreenTrophy_OnWidgetAction

/// `void cOVStateTrophy::onWidgetEvent(cOVStateTrophy* self, Event* ev,
/// int kind)` (0x20e9a0). Kind 5 calls the item's virtual method at
/// `+0x5c` with the event's (1.0, 1.0, 0, 0) vector, and, when the item is
/// the active one (`+0x18 == 1`), the list's current item's virtual method
/// at `+0x24`. Kind 6 calls the list's item method at `+0x2c` and, for a
/// non-null result, pushes a UI state (`cUIStateStack_push`). Every
/// path ends by popping the frame; kinds other than 5 and 6 pop at once.
pub fn ov_screen_trophy_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let no_event = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        'done: {
            'cleanup: {
                if no_event {
                    break 'cleanup;
                }
                'slot: {
                    'kind_is_five: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let is_five = c.g[A2][0] == c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if is_five {
                            break 'kind_is_five;
                        }
                        c.cycles += 2;
                        if c.g[A2][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                            break 'slot;
                        }
                        c.cycles += 2;
                        frame.restore(c, S0, 0x30);
                        break 'done;
                    }

                    // Kind 5: the item's method with the event's vector.
                    'pick: {
                        c.cycles += 16;
                        c.g[V1][0] = Addr(trophy_s0(c).wrapping_add(0x8)).lw(c);
                        c.g[A1][0] = c.g[SP][0];
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
                        c.swc1(0, sp(c).wrapping_add(0x4));
                        c.swc1(0, sp(c));
                        Addr(sp(c).wrapping_add(0x8)).sw(c, 0);
                        Addr(sp(c).wrapping_add(0xc)).sw(c, 0);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                        let method = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(trophy_s0(c).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method, 0x0020_ea08)?;
                        c.g[V1][0] = Addr(trophy_s0(c).wrapping_add(0x18)).lw(c);
                        c.g[V0][0] = 0x1;
                        let not_active = c.g[V1][0] != c.g[V0][0];
                        c.g[A1][0] = 0;
                        if not_active {
                            break 'pick;
                        }
                        c.cycles += 10;
                        c.g[V0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0x1;
                        c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x4)).lw(c);
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                        let method2 = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, method2, 0x0020_ea3c)?;
                        c.g[A1][0] = c.g[V0][0];
                    }
                    c.cycles += 5;
                    c.g[A0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0020_ea4c)?;
                    frame.restore(c, S0, 0x30);
                    break 'done;
                }

                // Kind 6: the list's current item's method.
                c.cycles += 10;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = Addr(trophy_s0(c).wrapping_add(0x18)).lw(c);
                c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
                let method3 = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method3, 0x0020_ea74)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !none {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(trophy_s1(c).wrapping_add(0x10)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0020_ea88)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The trophy screen (`s1`) as an address.
fn trophy_s1(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// The widget event (`s0`) as an address.
fn trophy_s0(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

// ---------------------------------------------------------------- OVScreenReadPrivateMessage_TransitionIn

/// `void OVScreenReadPrivateMessage_TransitionIn(screen, message)`
/// (0x210b58). With a message, it takes the game comm's queued chat line
/// (`cGameComm_getQueuedChat`, kind 0x41, into `screen+0x9c`) and shows
/// it with `OVScreenReadPrivateMessage_SetMessageText`; with no line it
/// opens the keyboard for a reply (`OVScreenReadPrivateMessage_OpenKeyboard`,
/// the 0x4a2560 prompt, kind 0x3d). Without a message it only pops the frame.
/// Returns 1.
pub fn ov_screen_read_private_message_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_message = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        'done: {
            'after_text: {
                if no_message {
                    break 'after_text;
                }
                'queued: {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0021_0b84);
                    c.g[A2][0] = 0x41;
                    call_ptr(c, 0x0025_8160, 0x0021_0b84)?;
                    let no_line = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if no_line {
                        break 'queued;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0021_0bd8, 0x0021_0b98)?;
                    frame.restore(c, S0, 0x20);
                    break 'done;
                }
                // No queued line: open the keyboard for a reply.
                c.cycles += 8;
                c.g[A2][0] = 0x4a0000;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2560));
                c.g[A1][0] = 0;
                c.g[T0][0] = 0;
                c.g[A3][0] = c.g[A2][0];
                c.g[T1][0] = 0;
                c.g[T2][0] = 0x3d;
                call_ptr(c, 0x0021_0f40, 0x0021_0bc0)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- OVScreenReadPrivateMessage_SetMessageText

/// `void OVScreenReadPrivateMessage_SetMessageText(screen, name, a)`
/// (0x210bd8). Finds the screen's text widget (`screen+0x40`, UI hash
/// `0x4717f8`), and when it exists writes the message text into its
/// scroller: a string built from the name (`0x534fb8` / `0x534fc8` prefix,
/// the `0x4a2710` separator, and the name in `s2`), added with
/// `cUITextScroll_addAsciiString`. With `a == 0` the string uses the
/// shorter prefix pair. Finishes with `func_001A83D8` (the widget's
/// layout) on the widget's fields at +0xc4..+0xc8.
pub fn ov_screen_read_private_message_set_message_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = 0x470000;
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17f8));
        call_ptr(c, 0x0031_7670, 0x0021_0c10)?;
        c.g[A0][0] = Addr(text_set_s0(c).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0021_0c1c)?;
        c.g[S0][0] = c.g[V0][0];
        let no_widget = c.g[S0][0] == 0;
        c.g[V0][0] = Addr(text_set_gp(c, 0x848)).lw(c);
        'done: {
            if no_widget {
                break 'done;
            }
            'text_block: {
                'with_pair: {
                    c.cycles += 8;
                    c.g[A1][0] = 0x7;
                    c.g[S5][0] = Addr(text_set_s0(c).wrapping_add(0xc8)).lhu(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
                    c.g[S3][0] = Addr(text_set_s0(c).wrapping_add(0xc4)).lhu(c);
                    c.g[S4][0] = Addr(text_set_s0(c).wrapping_add(0xc6)).lhu(c);
                    call_ptr(c, 0x0014_dd58, 0x0021_0c40)?;
                    let no_flag = c.g[S1][0] == 0;
                    c.g[A1][0] = 0x530000;
                    if no_flag {
                        break 'with_pair;
                    }
                    c.cycles += 19;
                    c.g[A1][0] = 0x530000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fb8));
                    call_ptr(c, 0x0031_82d8, 0x0021_0c58)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2710));
                    call_ptr(c, 0x0031_8910, 0x0021_0c68)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_8910, 0x0021_0c74)?;
                    c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a_19f8, 0x0021_0c80)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0031_8260, 0x0021_0c8c)?;
                    c.g[A0][0] = c.g[S0][0];
                    break 'text_block;
                }
                // Without the flag: the short prefix.
                c.cycles += 17;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fc8));
                call_ptr(c, 0x0031_82d8, 0x0021_0ca0)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2710));
                call_ptr(c, 0x0031_8910, 0x0021_0cb0)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0031_8910, 0x0021_0cbc)?;
                c.g[A1][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_19f8, 0x0021_0cc8)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0031_8260, 0x0021_0cd4)?;
                c.g[A0][0] = c.g[S0][0];
            }
            // Layout of the widget from its fields.
            c.cycles += 8;
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[T1][0] = c.g[S4][0];
            c.g[A1][0] = 0x1e;
            c.g[A2][0] = 0x8;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0x1;
            call_ptr(c, 0x001a_83d8, 0x0021_0cf8)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The widget event's screen (`s0` before the lookup) as an address.
fn text_set_s0(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

/// `GP - off`.
fn text_set_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

// ---------------------------------------------------------------- OVScreenReadPrivateMessage_OnWidgetAction

/// `void OVScreenReadPrivateMessage_OnWidgetAction(screen, ev, kind)`
/// (0x210df0), the read-message screen's widget handler. Does nothing
/// without the screen's widget (`screen+0x40`). Kind 6 clears the
/// widget's status bits (`+0x1c`, to `0x780`) after the item's method
/// call; kind 5 opens the reply keyboard (or, with a reply already in
/// `+0xe0`, leaves); kinds 0..6 other than those hand the event to
/// `cOVState_onWidgetEvent`; kind 13 sends the queued reply (`strlen` of
/// the reply text at `+0xe0 + 0x74`, `cGameComm_sendChat`) and shows it.
pub fn ov_screen_read_private_message_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        'out: {
            'body: {
                c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0x40)).lw(c);
                let no_widget = c.g[V0][0] == 0;
                c.g[V0][0] = 0x6;
                if no_widget {
                    break 'body;
                }
                'widget: {
                    'pick: {
                        'state: {
                            'kind_six: {
                                c.cycles += 2;
                                let is_six = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x7) as u64;
                                if is_six {
                                    break 'kind_six;
                                }
                                'kind_five: {
                                    c.cycles += 2;
                                    let below = c.g[V0][0] == 0;
                                    c.g[V0][0] = 0x5;
                                    if below {
                                        break 'kind_five;
                                    }
                                    c.cycles += 2;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                                        break 'state;
                                    }
                                    c.cycles += 2;
                                    break 'widget;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0xd;
                                let is_thirteen = c.g[A2][0] == c.g[V0][0];
                                c.g[S0][0] = sx(read_msg_s1(c).wrapping_add(0x9c));
                                if is_thirteen {
                                    break 'pick;
                                }
                                c.cycles += 2;
                                break 'widget;
                            }

                            // Kind 6: clear the widget's status bits.
                            c.cycles += 3;
                            c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                            let has_reply = c.g[V0][0] != 0;
                            frame.restore(c, S0, 0x20);
                            if has_reply {
                                break 'out;
                            }
                            c.cycles += 10;
                            c.g[A0][0] = Addr(read_msg_s1(c).wrapping_add(0x10)).lw(c);
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039_f190, 0x0021_0e60)?;
                            c.g[V1][0] = Addr(read_msg_s1(c).wrapping_add(0x1c)).lw(c);
                            c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                            c.g[V1][0] = c.g[V1][0] | 0x780;
                            let status = c.g[V1][0];
                            Addr(read_msg_s1(c).wrapping_add(0x1c)).sw(c, status);
                            break 'body;
                        }

                        // Kind 5: open the reply keyboard unless a reply is queued.
                        c.cycles += 2;
                        let has_reply = c.g[V0][0] != 0;
                        frame.restore(c, S0, 0x20);
                        if has_reply {
                            break 'out;
                        }
                        c.cycles += 16;
                        c.g[A2][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2560));
                        c.g[A1][0] = 0;
                        c.g[A3][0] = c.g[A2][0];
                        c.g[T0][0] = 0;
                        c.g[T1][0] = 0;
                        c.g[T2][0] = 0x3d;
                        call_ptr(c, 0x0021_0f40, 0x0021_0ea4)?;
                        c.g[A0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                        c.g[A1][0] = 0x4b;
                        c.g[A2][0] = 0x1;
                        c.g[A3][0] = 0x2;
                        call_ptr(c, 0x001c_e3c8, 0x0021_0eb8)?;
                        frame.restore(c, S0, 0x20);
                        break 'out;
                    }

                    // Kind 13: the queued chat line, if any, is shown as the reply.
                    c.cycles += 6;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x41;
                    call_ptr(c, 0x0025_8160, 0x0021_0ed0)?;
                    let no_line = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S0][0];
                    if !no_line {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = 0;
                        call_ptr(c, 0x0021_0bd8, 0x0021_0ee4)?;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = Addr(read_msg_s1(c).wrapping_add(0xe0)).lw(c);
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x74));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_6810, 0x0021_0ef4)?;
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x20);
                        break 'out;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0025_8c50, 0x0021_0f08)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0021_0bd8, 0x0021_0f18)?;
                    frame.restore(c, S0, 0x20);
                    break 'out;
                }

                // Every other kind: the generic overlay widget handler.
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0020_a6a8, 0x0021_0f28)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The read-message screen (`s1`) as an address.
fn read_msg_s1(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

// ---------------------------------------------------------------- OVScreenProfileSavePDA_ctor

/// `cOVState_PROFILE::cOVState_PROFILE(this, mgr, type, a3)` (0x211088),
/// the profile save screen's constructor. Runs the base memory-card
/// constructor, stores the screen's vtable and sizes, then, for a nonzero
/// `type`, scans the bits of `type >> 1` for the slot count (the first
/// set bit above bit 0, at most nine). Looks up the BE interface and
/// counts the profile slots of the game's save list, storing the slot
/// index at `+0x1cc` when it finds the matching one. Returns `this`.
pub fn ov_screen_profile_save_pda_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A2][0] & 0xff;
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A3][0];
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S4][0] = 0;
        call_ptr(c, 0x001d_53b0, 0x0021_10c0)?;
        c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
        c.g[V1][0] = 0x24;
        c.g[V0][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
        let size = c.g[V1][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0xc)).sw(c, size);
        let vtable = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        c.g[V1][0] = 0x1;
        let mgr = c.g[S0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0x278)).sw(c, mgr);
        c.g[S5][0] = 0x530000;
        Addr((c.g[S2][0] as u32).wrapping_add(0x1cc)).sw(c, 0);
        Addr((c.g[S2][0] as u32).wrapping_add(0x22c)).sw(c, 0);
        let no_type = c.g[S1][0] == 0;
        Addr((c.g[S2][0] as u32).wrapping_add(0x230)).sw(c, 0);
        'scan_bits: {
            if no_type {
                break 'scan_bits;
            }
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            'bit_loop: loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 9i64) as u64;
                let in_range = c.g[V0][0] != 0;
                c.g[V0][0] = sx(((c.g[S1][0] as u32) as i32 >> ((c.g[V1][0] as u32) & 31)) as u32);
                if !in_range {
                    break 'scan_bits;
                }
                c.cycles += 3;
                c.g[S1][0] = c.g[V0][0] & 0xff;
                if c.g[S1][0] != 0 {
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_10f8);
                    }
                    continue 'bit_loop;
                }
                c.cycles += 1;
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                break 'scan_bits;
            }
        }

        // The BE interface and the save list's slot count.
        c.cycles += 17;
        c.g[S1][0] = 0x1;
        call_ptr(c, 0x0014_dc80, 0x0021_111c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0021_1128)?;
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0021_1134)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0021_1140)?;
        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5bc8));
        c.g[A0][0] = 0x2;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x49)).lb(c);
        c.g[V0][0] ^= 0x2;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[A0][0];
        }
        'scan: loop {
            'slot_done: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if !more {
                    break 'slot_done;
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0014_7318, 0x0021_116c)?;
                if c.g[V0][0] != c.g[S4][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_1158);
                    }
                    continue 'scan;
                }
                c.cycles += 1;
                let slot = c.g[S0][0];
                Addr((c.g[S2][0] as u32).wrapping_add(0x1cc)).sw(c, slot);
            }
            break 'scan;
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------- OVScreenProfileSavePDA_ResetSlotList

/// `void cOVState_PROFILE::displayOff(this)` (0x211270): clears the save
/// screen's slot list. First, when the screen's profile record (`+0x8`)
/// has a valid handler (`+0x154`), the card is checked through the memory
/// card manager; when that is also clear, the card's info is refreshed
/// (`OVScreenProfileSavePDA_RefreshCardInfo`) and the function returns.
/// Otherwise the slots `1..=6` are cleared one by one: each slot name is
/// formatted (`sprintf` of the `0x471d68` pattern with the slot number),
/// its UI object is looked up by hash, and its two methods are called.
pub fn ov_screen_profile_save_pda_reset_slot_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A0][0];
        'exit_all: {
            'refresh: {
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x150)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x154)).lw(c);
                let handler = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, handler, 0x0021_12a0)?;
                let no_handler = c.g[V0][0] == 0;
                c.g[S1][0] = 0x1;
                if no_handler {
                    break 'refresh;
                }
                c.cycles += 10;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                call_ptr(c, 0x0022_7f80, 0x0021_12b0)?;
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x434)).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1a8)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1ac)).lw(c);
                let check = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, check, 0x0021_12c8)?;
                let card_clear = c.g[V0][0] == 0;
                c.g[S1][0] = 0x1;
                if card_clear {
                    break 'refresh;
                }
                c.cycles += 10;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x3;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x118)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x11c)).lw(c);
                let refresh_fn = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, refresh_fn, 0x0021_12e8)?;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0021_1380, 0x0021_12f0)?;
                frame.restore(c, S0, 0x80);
                break 'exit_all;
            }

            // Clear the slots 1..=6 (the loop's counter is `s1`).
            c.cycles += 2;
            c.g[S3][0] = 0x470000;
            c.g[A0][0] = c.g[SP][0];
            'slots: loop {
                c.cycles += 25;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1d68));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0041_7828, 0x0021_1310)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x0021_1318)?;
                c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0021_1324)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc8)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xcc)).lw(c);
                let clear_fn = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, clear_fn, 0x0021_1340)?;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                let hide_fn = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, hide_fn, 0x0021_1358)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 7i64) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[SP][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_1300);
                    }
                    continue 'slots;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x80);
                break 'exit_all;
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, DATA, GAME_GP};

    #[test]
    fn ov_screen_replay_menu_init_matches() {
        check(
            ee::s05::f_0020dc68__generated,
            ov_screen_replay_menu_init,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0020_dc68, 4);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x40);
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
                let flag = if seed % 2 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA + 0x2200 + 0x608, flag);
            },
        );
    }

    #[test]
    fn ov_screen_replay_menu_update_matches() {
        check(
            ee::s05::f_0020e530__generated,
            ov_screen_replay_menu_update,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0020_e530, 4);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x40);
                // The clock, step and limits: small values so both sides of
                // every compare occur across the seeds.
                c.bus.write32(this + 0xa4, r.next_u32() % 100);
                c.bus.write32(this + 0xa8, r.next_u32() % 50);
                c.bus.write32(this + 0xac, r.next_u32() % 200);
                c.bus.write32(this + 0xb0, r.next_u32() % 200);
                let object = DATA + 0x1400;
                c.bus.write32(this + 0x40, object);
                fill(c, &mut r, object, 0x20);
                let flag = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(this + 0x9c, flag);
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x28, DATA + 0x2200);
            },
        );
    }

    #[test]
    fn ov_screen_trophy_on_widget_action_matches() {
        check(
            ee::s05::f_0020e9a0__generated,
            ov_screen_trophy_on_widget_action,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_e9a0, 4);
                let screen = DATA + 0x1000;
                let event = DATA + 0x1100;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { sx(event) };
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
                fill(c, &mut r, screen, 0x10);
                fill(c, &mut r, event, 0x10);
                // The screen's list: item word at +0x10 points at the item
                // pointer block (item pointer, then its vtable).
                let items = DATA + 0x1400;
                let item = DATA + 0x1500;
                let vt = DATA + 0x1600;
                c.bus.write32(screen + 0x10, items);
                c.bus.write32(items, item);
                c.bus.write32(item + 0x4, vt);
                fill(c, &mut r, vt, 0x10);
                fill(c, &mut r, item, 0x10);
                // The event's object: vtable at +0x8 with method at +0x5c.
                let object = DATA + 0x1200;
                let object_vt = DATA + 0x1300;
                c.bus.write32(event + 0x8, object);
                c.bus.write32(object + 0x4, object_vt);
                c.bus
                    .write32(event + 0x18, if seed % 2 == 0 { 1 } else { r.next_u32() });
                fill(c, &mut r, object_vt, 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_transition_in_matches() {
        check(
            ee::s05::f_00210b58__generated,
            ov_screen_read_private_message_transition_in,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_0b58, 4);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x1100) };
                fill(c, &mut r, screen, 0x40);
                c.bus.write32(GAME_GP.wrapping_sub(0x204), DATA + 0x1300);
                fill(c, &mut r, DATA + 0x1300, 0x20);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_set_message_text_matches() {
        check(
            ee::s05::f_00210bd8__generated,
            ov_screen_read_private_message_set_message_text,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_0bd8, 3);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { 1 };
                fill(c, &mut r, screen, 0x40);
                let widget = DATA + 0x1200;
                c.bus.write32(screen + 0x40, widget);
                fill(c, &mut r, widget, 0x40);
                c.bus.write32(DATA + 0x1100, 0x0043_4241);
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill(c, &mut r, world, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_on_widget_action_matches() {
        check(
            ee::s05::f_00210df0__generated,
            ov_screen_read_private_message_on_widget_action,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_0df0, 4);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = sx(DATA + 0x1100);
                c.g[A2][0] = (seed % 9) as u64;
                fill(c, &mut r, screen, 0x40);
                // The widget at +0x40 is absent for some seeds.
                let widget = if seed % 5 == 0 { 0 } else { DATA + 0x1200 };
                c.bus.write32(screen + 0x40, widget);
                fill(c, &mut r, DATA + 0x1200, 0x40);
                // The reply at +0xe0: absent or present.
                let reply = if seed % 2 == 0 { 0 } else { DATA + 0x1300 };
                c.bus.write32(screen + 0xe0, reply);
                fill(c, &mut r, DATA + 0x1300, 0x40);
                c.bus.write32(DATA + 0x1300 + 0x74, 0x0043_4241);
                c.bus.write32(GAME_GP.wrapping_sub(0x204), DATA + 0x1400);
                fill(c, &mut r, DATA + 0x1400, 0x20);
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_ctor_matches() {
        check(
            ee::s05::f_00211088__generated,
            ov_screen_profile_save_pda_ctor,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_1088, 4);
                let this = DATA + 0x1000;
                let mgr = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x3000);
                // The type word: zero, or a value whose bits give 1..8 slots.
                c.g[A2][0] = [0u64, 2, 4, 6, 8, 12, 16, 0xff][(seed % 8) as usize];
                c.g[A3][0] = sx(mgr);
                fill(c, &mut r, this, 0x80);
                fill(c, &mut r, DATA + 0x3000, 0x20);
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_reset_slot_list_matches() {
        check(
            ee::s05::f_00211270__generated,
            ov_screen_profile_save_pda_reset_slot_list,
            200,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0021_1270, 4);
                crate::difftest::elf_image(c);
                let screen = DATA + 0x1000;
                c.g[A0][0] = sx(screen);
                let profile = DATA + 0x1200;
                c.bus.write32(screen + 0x8, profile);
                let vt = DATA + 0x1300;
                let object = DATA + 0x1400;
                c.bus.write32(screen + 0x40, object);
                c.bus.write32(object + 0x8, vt);
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill(c, &mut r, world, 0x40);
            },
        );
    }
}
