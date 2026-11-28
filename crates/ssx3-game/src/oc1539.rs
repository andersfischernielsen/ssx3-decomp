//! Staging module oc1539 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1539`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_5ff0,
        cfe_state_audio_options_ctor,
        "cFEStateAudioOptions_ctor",
    ),
    (
        0x0019_6378,
        cfe_state_audio_options_on_gain_focus,
        "cFEStateAudioOptions_OnActivate",
    ),
    (
        0x001a_5768,
        cfe_state_net_messenger_find_name,
        "cFEStateNetMessenger_FindName",
    ),
    (
        0x001b_0d30,
        cfe_state_challenge_received_popup_show_accept_decline,
        "cFEStateChallengeReceivedPopup_ShowAcceptDecline",
    ),
    (
        0x001b_6080,
        cfe_state_net_config_on_widget_list_box,
        "cFEStateNetConfig_onWidgetListBox",
    ),
    (
        0x001b_ba18,
        cfe_state_create_step2_is_email_format_valid,
        "cFEStateCreateStep2_IsEmailFormatValid",
    ),
    (
        0x001b_dca0,
        cfe_state_lobby_menu_on_widget_pair,
        "cFEStateLobbyMenu_onWidgetPair",
    ),
    (
        0x001c_0218,
        cfe_state_online_main_menu_on_child_event_ignore_confirm,
        "cFEStateOnlineMainMenu_OnChildEventIgnoreConfirm",
    ),
];

/// `obj->method(a1)` on the object in `a2`, the generated `vtable + off`
/// pattern: `v0` = the object's vtable (`obj+8`), `a0 = obj + delta` and
/// `v1` = the method, both read from the vtable slot at `off` (`lh` delta,
/// `lw` method), then the call returning to `ret`.
fn audio_options_vcall_a2(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(8));
    let vt = c.g[V0][0];
    let slot = VSlot::read(c, vt, off);
    c.g[A0][0] = slot.this(c.g[A2][0]);
    c.g[V1][0] = slot.func;
    call_ptr(c, slot.func as u32, ret)
}

/// `void cFEStateAudioOptions::cFEStateAudioOptions(cUIEngine* engine,
/// char userNum, bool inStore)` (0x195FF0, `this` in a0): the base
/// `cUIState` constructor, then the audio-options fields. The user number
/// (sign-extended char) goes to +0x44, the controller mask to +0x15 when the
/// player pointer at +0x10 is set, and the four words at +0x4c..+0x58 are
/// cleared by a counted loop. Two texture-bank lookups and the group numbers
/// (+0x5c..+0x7c) follow. Returns `this`.
pub fn cfe_state_audio_options_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        // cUIState(this, engine): a0 and a1 still hold the arguments.
        call_ptr(c, 0x0039_e2a0, 0x0019_6018)?;
        let this = r32(c, S1);
        c.g[V0][0] = 0x10;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bb8));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sb(this.wrapping_add(0x44), c.g[S0][0]);
        c.g[V0][0] = c.lw(this.wrapping_add(0x10));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        if c.g[A0][0] == 0 {
            c.sw(this.wrapping_add(0x48), 0u64);
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x001a_1cd0, 0x0019_6048)?;
            c.sb(this.wrapping_add(0x15), c.g[V0][0]);
            c.sw(this.wrapping_add(0x48), 0u64);
        }
        c.cycles += 6;
        c.g[V1][0] = 3;
        c.sw(this.wrapping_add(0x64), 0u64);
        c.g[V0][0] = sx(this.wrapping_add(0x58));
        c.sw(this.wrapping_add(0x68), 0u64);
        c.sw(this.wrapping_add(0x6c), 0u64);
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_6068);
            }
        }
        c.cycles += 48;
        c.sw(this.wrapping_add(0x74), c.g[S2][0]);
        c.g[A0][0] = 0x0046_0000;
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25b8));
        call_ptr(c, 0x0031_7670, 0x0019_6098)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_8380, 0x0019_60a4)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[S0][0] = c.lw(this.wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25c8));
        c.sw(this.wrapping_add(0x5c), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0019_60b8)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_8380, 0x0019_60c4)?;
        c.sw(this.wrapping_add(0x60), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0019_60cc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0019_60d8)?;
        c.g[A1][0] = c.lb(this.wrapping_add(0x44));
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_7488, 0x0019_60e4)?;
        c.sw(this.wrapping_add(0x70), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0019_60ec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0019_60f8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lb(this.wrapping_add(0x44));
        c.g[A2][0] = c.lw(this.wrapping_add(0x70));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0015_8700, 0x0019_610c)?;
        c.g[A1][0] = c.lb(this.wrapping_add(0x44));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.lw(this.wrapping_add(0x70));
        c.sw(this.wrapping_add(0x78), c.g[V0][0]);
        call_ptr(c, 0x0015_87b8, 0x0019_6120)?;
        c.g[V0][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
        frame.restore(c, S0, 0x30);
        c.sw(this.wrapping_add(0x7c), c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S2, 0x10);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateAudioOptions::onGainFocus(cUIState* prev)` (0x196378): while
/// the option objects are not built (+0x7c clear), each child's virtual
/// method gets its argument (the `vtable + 0x48`, `+0xd0`, `+0x30` and
/// `+0x58` slots), and a child at +0x74 skips the last pair. Then the base
/// `cFEState_OnGainFocus(prev)`.
pub fn cfe_state_audio_options_on_gain_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        let this = r32(c, S1);
        c.g[V0][0] = c.lw(this.wrapping_add(0x7c));
        let created = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A1][0];
        if created {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 82;
            c.g[A2][0] = c.lw(this.wrapping_add(0x68));
            c.g[A1][0] = 1;
            audio_options_vcall_a2(c, 0x48, 0x0019_63b8)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x50));
            c.g[A1][0] = 1;
            audio_options_vcall_a2(c, 0x48, 0x0019_63d4)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x64));
            c.g[A1][0] = 1;
            audio_options_vcall_a2(c, 0x48, 0x0019_63f0)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x6c));
            c.g[A1][0] = 1;
            audio_options_vcall_a2(c, 0x48, 0x0019_640c)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x58));
            c.g[A1][0] = 1;
            audio_options_vcall_a2(c, 0x48, 0x0019_6428)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x68));
            c.g[A1][0] = 1;
            audio_options_vcall_a2(c, 0xd0, 0x0019_6444)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x68));
            c.g[A1][0] = 0;
            audio_options_vcall_a2(c, 0x30, 0x0019_6460)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x50));
            c.g[S0][0] = 0x004c_66d8;
            c.g[A1][0] = c.g[S0][0];
            audio_options_vcall_a2(c, 0x58, 0x0019_6484)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x6c));
            c.g[A1][0] = 1;
            audio_options_vcall_a2(c, 0xd0, 0x0019_64a0)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x6c));
            c.g[A1][0] = 0;
            audio_options_vcall_a2(c, 0x30, 0x0019_64bc)?;
            c.g[A2][0] = c.lw(this.wrapping_add(0x58));
            c.g[A1][0] = c.g[S0][0];
            audio_options_vcall_a2(c, 0x58, 0x0019_64d8)?;
            c.g[V0][0] = c.lw(this.wrapping_add(0x74));
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[S1][0];
            } else {
                c.cycles += 14;
                c.g[A2][0] = c.lw(this.wrapping_add(0x64));
                c.g[A1][0] = 1;
                audio_options_vcall_a2(c, 0xd0, 0x0019_6500)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x64));
                c.g[A1][0] = 0;
                audio_options_vcall_a2(c, 0x30, 0x0019_651c)?;
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
            }
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0018_6518, 0x0019_6528)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateNetMessenger::FindName(const char* name)` (0x1A5768): the
/// index of the first entry of the list at +0x6e8..+0x6ec whose name
/// (`strcasecmp` of the entry's first word) matches, or -1.
pub fn cfe_state_net_messenger_find_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S1);
        c.g[V0][0] = c.lw(this.wrapping_add(0x6ec));
        c.g[V1][0] = c.lw(this.wrapping_add(0x6e8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        let mut found = false;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = c.lw(this.wrapping_add(0x6e8));
            loop {
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[A1][0] = c.g[S2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                call_ptr(c, 0x0041_aa88, 0x001a_57b8)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    found = true;
                    break;
                }
                c.g[V0][0] = c.lw(this.wrapping_add(0x6ec));
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V1][0] = c.lw(this.wrapping_add(0x6e8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(this.wrapping_add(0x6e8));
                if event_due(c) {
                    deliver_event_here(c, 0x001a_57a0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengeReceivedPopup::ShowAcceptDecline(void)` (0x1B0D30):
/// clear the popup's result word (+0x6d4), then, when the screen at +0x40 is
/// set, look up three widgets by hash (the `0x4635f8`, `0x463468` and
/// `0x463618` strings) and call the virtual method of each one found, with
/// the accept/decline sounds as arguments.
pub fn cfe_state_challenge_received_popup_show_accept_decline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = r32(c, S1);
        c.sw(this.wrapping_add(0x6d4), 0u64);
        c.g[V0][0] = c.lw(this.wrapping_add(0x40));
        let no_screen = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_screen {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x35f8));
            call_ptr(c, 0x0031_7670, 0x001b_0d5c)?;
            c.g[A0][0] = c.lw(this.wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001b_0d68)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            if c.g[S0][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001b_0d88)?;
            }
            c.cycles += 9;
            c.g[A0][0] = 0x0046_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3468));
            call_ptr(c, 0x0031_7670, 0x001b_0d94)?;
            c.g[A0][0] = c.lw(this.wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001b_0da0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            if c.g[S0][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001b_0dc0)?;
            }
            c.cycles += 9;
            c.g[A0][0] = 0x0046_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3618));
            call_ptr(c, 0x0031_7670, 0x001b_0dcc)?;
            c.g[A0][0] = c.lw(this.wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001b_0dd8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x0046_0000;
            if c.g[S0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x37b0));
                call_ptr(c, 0x0031_7670, 0x001b_0dec)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x001b_0df8)?;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateNetConfig::onWidgetListBox(void* widget, int id, int kind)`
/// (0x1B6080): answer 0x101 (not handled) for `kind >= 2`; otherwise 1 when
/// the list-box count at +0x6d4 is above 1, else 0.
pub fn cfe_state_net_config_on_widget_list_box(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = (c.g[A2][0] < 0x2) as u64;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] != 0 {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6d4));
        c.g[A0][0] = 0x1;
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[A0][0] == 0 {
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `bool cFEStateCreateStep2::IsEmailFormatValid(const char* email)`
/// (0x1BBA18): the address has a non-empty part before one `@` and a `.`
/// after it. The scan keeps the first `.`-after-`@` position and the `@` flag
/// (`s2`); the result is `s0 != 0 && s2 != 0`. The string is read by `strlen`
/// first.
pub fn cfe_state_create_step2_is_email_format_valid(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[A0][0] = c.g[S1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0041_6810, 0x001b_ba3c)?;
        c.g[S0][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        let longer = (c.g[A0][0] as i64) > 0;
        c.g[V1][0] = 0;
        if longer {
            c.cycles += 3;
            c.g[A2][0] = 0x40;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[V0][0] as u32);
                let differs = c.g[V0][0] != c.g[A2][0];
                c.g[V0][0] ^= 0x2e;
                if differs {
                    c.cycles += 1;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[A1][0];
                    }
                } else {
                    c.cycles += 2;
                    c.g[S2][0] = 0x1;
                }
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001b_ba58);
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        if c.g[S2][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = (0u64 < c.g[S0][0]) as u64;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLobbyMenu::onWidgetPair(int id, int kind)` (0x1BDCA0): with
/// the first pair (`a2 == 1`) or a non-zero `a2`, the result is 0x101 at
/// once. Otherwise scan the four-word list at +0x718 for `a1` (at most four
/// entries); on a match, `v1` is the byte at +0x318 of the entry's pointer
/// (the list at +0x708). Always returns 0x101.
pub fn cfe_state_lobby_menu_on_widget_pair(c: &mut Cpu) {
    'body: {
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[A2][0] == c.g[V0][0] {
            break 'body;
        }
        c.cycles += 2;
        let second_pair = c.g[A2][0] != 0;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x718));
        if second_pair {
            break 'body;
        }
        c.cycles += 3;
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x708));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let differs = c.g[A1][0] != c.g[V0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if differs {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                if !more {
                    break 'body;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001b_dcc0);
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x318));
                break 'body;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    ret(c);
}

/// `void cFEStateOnlineMainMenu::OnChildEventIgnoreConfirm(self, popup, kind)`
/// (0x1C0218): every popup event except kind 0xf (the confirm) goes to the
/// base `cFEStateNet::onPopupEvent`.
pub fn cfe_state_online_main_menu_on_child_event_ignore_confirm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let confirm = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if !confirm {
            c.cycles += 2;
            call_ptr(c, 0x001a_97b8, 0x001c_0230)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, machine_rt, FAKE};
    use crate::difftest::{check, DATA};

    // Fixture objects, all in DATA+0x280000 and up: clear of the fake log,
    // the fakes' returned pointers and the allocator's regions.

    /// The object under test (`this`).
    pub(crate) const OC1539_OBJ: u32 = DATA + 0x28_0000;
    /// The vtable the child objects point at.
    pub(crate) const OC1539_VT: u32 = DATA + 0x2a_0000;
    /// The `prev` state handed to the gain-focus body.
    pub(crate) const OC1539_PREV: u32 = DATA + 0x2c_0000;
    /// A string the string functions read.
    pub(crate) const OC1539_STR: u32 = DATA + 0x2b_0000;

    pub(crate) fn oc1539_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A NUL-terminated string at `a`.
    pub(crate) fn oc1539_str(c: &mut Cpu, a: u32, s: &[u8]) {
        for (k, b) in s.iter().enumerate() {
            c.bus.write8(a + k as u32, *b);
        }
        c.bus.write8(a + s.len() as u32, 0);
    }

    /// Vtable slots: delta 0 at `vt+off` and the fake call at `vt+off+4`.
    pub(crate) fn oc1539_vtable(c: &mut Cpu, vt: u32, slots: &[u32]) {
        for &off in slots {
            c.bus.write16(vt + off, 0);
            c.bus.write32(vt + off + 4, FAKE);
        }
    }

    #[test]
    fn cfe_state_audio_options_ctor_matches() {
        check(
            ee::s02::f_00195ff0__generated,
            cfe_state_audio_options_ctor,
            120,
            &|c, seed| {
                machine_rt(c, seed, 0x0019_5ff0, 6);
                let this = OC1539_OBJ;
                let player = this + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (this + 0x1000) as u64;
                c.g[A2][0] = ((seed as u64) << 40) | ((seed as u64 * 37) % 256);
                c.g[A3][0] = (seed % 2) as u64;
                oc1539_w(c, this + 0x10, player);
                oc1539_w(c, player, if seed % 3 == 0 { 0 } else { this + 0x3000 });
            },
        );
    }

    #[test]
    fn cfe_state_audio_options_on_gain_focus_matches() {
        check(
            ee::s02::f_00196378__generated,
            cfe_state_audio_options_on_gain_focus,
            160,
            &|c, seed| {
                machine_rt(c, seed, 0x0019_6378, 4);
                let this = OC1539_OBJ;
                let child = this + 0x2000;
                oc1539_vtable(c, OC1539_VT, &[0x30, 0x48, 0x58, 0xd0]);
                oc1539_w(c, child + 8, OC1539_VT);
                for off in [0x50u32, 0x58, 0x64, 0x68, 0x6c] {
                    oc1539_w(c, this + off, child);
                }
                oc1539_w(c, this + 0x74, if seed % 3 == 0 { 0 } else { 1 });
                oc1539_w(c, this + 0x7c, if seed % 4 == 0 { 0 } else { 1 });
                c.g[A0][0] = this as u64;
                c.g[A1][0] = OC1539_PREV as u64;
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_find_name_matches() {
        const NAMES: [&[u8]; 4] = [b"Alpha", b"Bravo", b"Charlie", b"Delta"];
        check(
            ee::s03::f_001a5768__generated,
            cfe_state_net_messenger_find_name,
            120,
            &|c, seed| {
                machine(c, seed, 0x001a_5768, 0);
                let this = OC1539_OBJ;
                let begin = this + 0x1000;
                let count = seed % 5;
                oc1539_w(c, this + 0x6e8, begin);
                oc1539_w(c, this + 0x6ec, begin + 4 * count);
                for i in 0..count {
                    let elem = this + 0x2000 + 0x40 * i;
                    let name = this + 0x3000 + 0x20 * i;
                    oc1539_w(c, begin + 4 * i, elem);
                    oc1539_w(c, elem, name);
                    oc1539_str(c, name, NAMES[i as usize]);
                }
                let needle: &[u8] = if seed % 2 == 0 && count > 0 {
                    NAMES[(seed % count) as usize]
                } else {
                    b"Zulu"
                };
                oc1539_str(c, OC1539_STR, needle);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = OC1539_STR as u64;
            },
        );
    }

    #[test]
    fn cfe_state_challenge_received_popup_show_accept_decline_matches() {
        check(
            ee::s03::f_001b0d30__generated,
            cfe_state_challenge_received_popup_show_accept_decline,
            120,
            &|c, seed| {
                machine_rt(c, seed, 0x001b_0d30, 4);
                let this = OC1539_OBJ;
                let screen = this + 0x2000;
                oc1539_w(c, this + 0x40, if seed % 4 == 0 { 0 } else { screen });
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn cfe_state_net_config_on_widget_list_box_matches() {
        check(
            ee::s03::f_001b6080__generated,
            cfe_state_net_config_on_widget_list_box,
            40,
            &|c, seed| {
                machine(c, seed, 0x001b_6080, 0);
                let this = OC1539_OBJ;
                c.g[A0][0] = this as u64;
                c.g[A2][0] = (seed % 4) as u64;
                oc1539_w(c, this + 0x6d4, seed % 3);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_is_email_format_valid_matches() {
        const CASES: [&[u8]; 7] = [b"a@b.c", b"a.b@c", b"@x", b"", b"abc", b"a.@b", b"x.y@z.w"];
        check(
            ee::s03::f_001bba18__generated,
            cfe_state_create_step2_is_email_format_valid,
            140,
            &|c, seed| {
                machine_rt(c, seed, 0x001b_ba18, 2);
                oc1539_str(c, OC1539_STR, CASES[(seed % 7) as usize]);
                c.g[A1][0] = OC1539_STR as u64;
            },
        );
    }

    #[test]
    fn cfe_state_lobby_menu_on_widget_pair_matches() {
        check(
            ee::s03::f_001bdca0__generated,
            cfe_state_lobby_menu_on_widget_pair,
            120,
            &|c, seed| {
                machine(c, seed, 0x001b_dca0, 0);
                let this = OC1539_OBJ;
                let pair = this + 0x2000;
                for k in 0..4u32 {
                    oc1539_w(c, this + 0x718 + 4 * k, 10 + k);
                    oc1539_w(c, this + 0x708 + 4 * k, pair);
                }
                c.bus.write8(pair + 0x318, (seed % 256) as u8);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (10 + seed % 5) as u64;
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn cfe_state_online_main_menu_on_child_event_ignore_confirm_matches() {
        check(
            ee::s03::f_001c0218__generated,
            cfe_state_online_main_menu_on_child_event_ignore_confirm,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001c_0218, 2);
                c.g[A0][0] = OC1539_OBJ as u64;
                c.g[A1][0] = (OC1539_OBJ + 0x1000) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0xf } else { (seed % 16) as u64 };
            },
        );
    }
}
