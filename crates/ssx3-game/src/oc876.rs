//! Staging module oc876 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc876`.

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
        0x001a_dc30,
        cfe_state_select_persona_on_submit_result,
        "cFEStateSelectPersona_OnSubmitResult",
    ),
    (
        0x001a_de88,
        cfe_state_challenge_popup_init,
        "cFEStateChallengePopup_Init",
    ),
    (
        0x001a_e0e0,
        cfe_state_challenge_popup_update,
        "cFEStateChallengePopup_Update",
    ),
];

/// The "filt" (profanity), "inam" (name length) and "dupl" (duplicate) error
/// codes of `online_ErrorEvent::mError`, as the generated code builds them.
const SUBMIT_CODE_FILT: u64 = 0x6669_6c74;
const SUBMIT_CODE_INAM: u64 = 0x696e_616d;
const SUBMIT_CODE_DUPL: u64 = 0x6475_706c;

/// Which branch of the submit-result switch ran.
#[derive(Clone, Copy, PartialEq)]
enum SubmitPath {
    Filt,
    Inam,
    Dupl,
    Default,
}

/// `void cFEStateSelectPersona_OnSubmitResult(self, msg)` (0x1adc30): the
/// persona submit result. The error code picks the popup: "filt" shows the
/// profanity popup (persona or plain, by `mbExistingPersona` at +0x6d0),
/// "inam" the persona-name-length popup, "dupl" creates the duplicate state;
/// anything else goes to `NetFEState_HandleNetEvent`.
pub fn cfe_state_select_persona_on_submit_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        c.g[V0][0] = SUBMIT_CODE_FILT;
        let msg = r32(c, A2);
        c.g[V1][0] = Addr(msg.wrapping_add(8)).lw(c);
        c.g[S2][0] = c.g[A0][0];
        let code = c.g[V1][0];

        let path = if code == SUBMIT_CODE_FILT {
            SubmitPath::Filt
        } else {
            c.cycles += 3;
            if SUBMIT_CODE_FILT < code {
                c.cycles += 3;
                if code == SUBMIT_CODE_INAM {
                    SubmitPath::Inam
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = Addr(msg).lw(c);
                    SubmitPath::Default
                }
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x46_0000;
                if code == SUBMIT_CODE_DUPL {
                    SubmitPath::Dupl
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = Addr(msg).lw(c);
                    SubmitPath::Default
                }
            }
        };

        match path {
            SubmitPath::Filt => {
                c.cycles += 8;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a_9c50, 0x001a_dd30)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 1;
                let v = c.g[V1][0];
                Addr(r32(c, S0).wrapping_add(0xbc)).sw(c, v);
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6d0)).lw(c);
                let existing = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xbc));
                if existing {
                    c.cycles += 5;
                    c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x20c8));
                    call_ptr(c, 0x0031_7670, 0x001a_dd6c)?;
                    c.g[V1][0] = 3;
                    c.g[A0][0] = 0x83d;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x30a8));
                    call_ptr(c, 0x0031_7670, 0x001a_dd54)?;
                    c.g[V1][0] = 3;
                    c.g[A0][0] = 0x818;
                }
                c.cycles += 11;
                let v = c.g[V0][0];
                Addr(r32(c, S0).wrapping_add(0xc4)).sw(c, v);
                let v = c.g[V1][0];
                Addr(r32(c, S0).wrapping_add(0xc0)).sw(c, v);
                let v = c.g[A0][0];
                Addr(r32(c, S0).wrapping_add(0x150)).sw(c, v);
                c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x1618));
                call_ptr(c, 0x0031_7670, 0x001a_dd8c)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x001c_57c0, 0x001a_dd9c)?;
                c.g[A0][0] = c.g[S2][0];
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001a_9bc0, 0x001a_dda8)?;
            }
            SubmitPath::Inam => {
                c.cycles += 23;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a_9c50, 0x001a_dcd4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x15f8));
                c.g[V0][0] = 1;
                let v = c.g[V0][0];
                Addr(r32(c, S0).wrapping_add(0xbc)).sw(c, v);
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xbc));
                call_ptr(c, 0x0031_7670, 0x001a_dcf0)?;
                c.g[V1][0] = 3;
                c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x1618));
                let v = c.g[V1][0];
                Addr(r32(c, S0).wrapping_add(0xc0)).sw(c, v);
                let v = c.g[V0][0];
                Addr(r32(c, S0).wrapping_add(0xc4)).sw(c, v);
                call_ptr(c, 0x0031_7670, 0x001a_dd08)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x001c_57c0, 0x001a_dd18)?;
                c.g[V0][0] = 0x818;
                c.g[A0][0] = c.g[S2][0];
                let v = c.g[V0][0];
                Addr(r32(c, S0).wrapping_add(0x150)).sw(c, v);
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001a_9bc0, 0x001a_dda8)?;
            }
            SubmitPath::Dupl => {
                c.cycles += 14;
                c.g[A0][0] = 0x6e0;
                c.g[A1][0] = sx(0x46_0000u32.wrapping_add(0x3090));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001a_dca8)?;
                c.g[A1][0] = Addr(r32(c, S2).wrapping_add(0x10)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x001b_bd08, 0x001a_dcb8)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x001a_8a40, 0x001a_dcc4)?;
            }
            SubmitPath::Default => {
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a_8e40, 0x001a_ddb8)?;
            }
        }

        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengePopup_Init(self)` (0x1ade88): the popup's setup.
/// Calls the state's hook through the vtable at `this+8`, fetches the text
/// from the message table, fills the challenge text and the set options,
/// clears the one-shot flag at +0x6d4 (or sets up the default popup text when
/// the table is empty), and at the end shows the disconnected popup when the
/// global object at `gp-0xc8` has no first word.
pub fn cfe_state_challenge_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x100)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x104)).lw(c);
        let hook = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, hook, 0x001a_deb0)?;
        c.g[A0][0] = 0x46_0000;
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x30c8));
        call_ptr(c, 0x0031_7670, 0x001a_dec0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001a_ded4)?;
        let zero = c.g[V0][0] == 0;
        let v = c.g[V0][0];
        Addr(r32(c, S1).wrapping_add(0x40)).sw(c, v);

        // `zero` and the one-shot-flag path both reach the common exit with
        // one more cycle; the other branch reloads v0 and falls through.
        let mut exit_plus_one = zero;
        if !zero {
            c.cycles += 32;
            c.g[A2][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001a_deec)?;
            call_ptr(c, 0x0014_dc80, 0x001a_def4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x001a_df00)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x1d9c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0014_4d98, 0x001a_df10)?;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(8)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
            let hook2 = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, hook2, 0x001a_df24)?;
            c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_f728, 0x001a_df34)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_f688, 0x001a_df3c)?;
            c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
            c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x98)).lw(c);
            call_ptr(c, 0x0025_e6d0, 0x001a_df48)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
            call_ptr(c, 0x001a_f930, 0x001a_df50)?;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x6d4)).lw(c);
            if c.g[V0][0] != 0 {
                Addr(r32(c, S1).wrapping_add(0x6d4)).sw(c, 0);
                exit_plus_one = true;
            } else {
                c.cycles += 11;
                c.g[A0][0] = sx(0x46_0000u32.wrapping_add(0x30e0));
                call_ptr(c, 0x0031_7670, 0x001a_df68)?;
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001a_df74)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 6;
                call_ptr(c, 0x0039_b960, 0x001a_df80)?;
            }
        }

        if exit_plus_one {
            c.cycles += 1;
        }
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        let has_first_word = c.g[V1][0] != 0;
        frame.restore(c, S0, 0x20);
        if !has_first_word {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_9930, 0x001a_dfa0)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengePopup_Update(self)` (0x1ae0e0): forwards to
/// `NetFEState_Update` with the same `self`.
pub fn cfe_state_challenge_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_8770, 0x001a_e0f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// Not registered: the real callees run away on test data (EAFile_AllocRequest
// spin at 0x3de768); kept until the fixture gets an allocator.
#[allow(dead_code)]
/// `void cFEStateChallengePopup_ApplySettings(self)` (0x1ae680): pushes the
/// popup's setting choices to the engine: the event-option flags are written
/// into a global, the four list boxes are updated, and the hook at the
/// world's vtable is called with the result. Does nothing when `self+0x40`
/// (the setting-list object) is empty.
pub fn cfe_state_challenge_popup_apply_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        if !empty {
            c.cycles += 58;
            c.g[A1][0] = 7;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x78)).lw(c);
            call_ptr(c, 0x0014_dd58, 0x001a_e6ac)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6dc)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_a738, 0x001a_e6b8)?;
            c.g[V0][0] = (c.g[V0][0] != 0) as u64;
            c.g[V1][0] = 0x53_0000;
            let v = c.g[V0][0];
            Addr(r32(c, V1).wrapping_add(0x4b34)).sw(c, v);
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
            let hook = r32(c, V1);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, hook, 0x001a_e6d8)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e8)).lw(c);
            call_ptr(c, 0x0039_a738, 0x001a_e6e0)?;
            c.g[S0][0] = 0x53_0000;
            c.g[V0][0] &= 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x8b8));
            c.g[A0][0] = 0xffff_ffff_ffff_ffbf;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 6);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V1][0] |= c.g[V0][0];
            let v = c.g[V1][0];
            Addr(r32(c, S0).wrapping_add(0x18)).sw(c, v);
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e4)).lw(c);
            call_ptr(c, 0x0039_a738, 0x001a_e70c)?;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
            c.g[V0][0] &= 1;
            c.g[A0][0] = 0xffff_ffff_ffff_feff;
            c.g[V0][0] = sx(r32(c, V0) << 8);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V1][0] |= c.g[V0][0];
            let v = c.g[V1][0];
            Addr(r32(c, S0).wrapping_add(0x18)).sw(c, v);
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6ec)).lw(c);
            call_ptr(c, 0x0039_a738, 0x001a_e730)?;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
            c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
            c.g[V0][0] &= 1;
            c.g[V1][0] &= c.g[A0][0];
            c.g[V0][0] = sx(r32(c, V0) << 7);
            c.g[V1][0] |= c.g[V0][0];
            let v = c.g[V1][0];
            Addr(r32(c, S0).wrapping_add(0x18)).sw(c, v);
            call_ptr(c, 0x0014_dc80, 0x001a_e750)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x001a_e75c)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e0)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_a738, 0x001a_e768)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4d98, 0x001a_e774)?;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(8)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
            let hook2 = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, hook2, 0x001a_e788)?;
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// Not registered: the real callees run away on test data (EAFile_AllocRequest
// spin at 0x3de768); kept until the fixture gets an allocator.
#[allow(dead_code)]
/// `void cFEStateChallengePopup_InitSettingLists(self, all)` (0x1af098): sets
/// the four list boxes (power-ups, multipliers, point icons, CPU riders) to
/// the current event options. With `all` set they all go to their defaults.
/// The CPU riders follow the event's AI count, or a global when there is no
/// game event.
pub fn cfe_state_challenge_popup_init_setting_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x001a_f0b8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001a_f0c4)?;
        c.g[S2][0] = c.g[V0][0];

        if c.g[S0][0] != 0 {
            // `all`: every list to its default entry.
            c.cycles += 11;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e4)).lw(c);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0039_a7a8, 0x001a_f0d8)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e8)).lw(c);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0039_a7a8, 0x001a_f0e4)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6ec)).lw(c);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0039_a7a8, 0x001a_f0f0)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e0)).lw(c);
        } else {
            // Power-ups, multipliers, point icons: the option bit when the
            // option object is set, else entry 1.
            set_power_ups_list(c)?;
            set_multipliers_list(c)?;
            set_point_icons_list(c)?;

            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x5c12)).lb(c);
            c.g[A1][0] = 0;
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x1d9c)).lbu(c);
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e0)).lw(c);
                call_ptr(c, 0x0039_a7a8, 0x001a_f1ec)?;
                frame.restore(c, S0, 0x30);
                c.cycles += 5;
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0014_4d98, 0x001a_f1b8)?;
            c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(8)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
            let hook = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
            call_ptr(c, hook, 0x001a_f1cc)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e0)).lw(c);
        }

        // CPU riders: entry 0.
        c.cycles += 4;
        c.g[A1][0] = 0;
        call_ptr(c, 0x0039_a7a8, 0x001a_f1d8)?;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The power-ups list box: entry from bit 8 of the first option word when
/// `mpEventOptions[1]` is set, else entry 1.
#[allow(dead_code)]
fn set_power_ups_list(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x6d8)).lw(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
    let unset = c.g[V1][0] == 0;
    c.g[V0][0] = 0x53_0000;
    if unset {
        c.cycles += 4;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e4)).lw(c);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f130)?;
    } else {
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e4)).lw(c);
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x8d0)).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 8) as u32);
        c.g[A1][0] &= 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f11c)?;
    }
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x6d8)).lw(c);
    Ok(())
}

/// The multipliers list box: as [`set_power_ups_list`], bit 6 and option 3.
#[allow(dead_code)]
fn set_multipliers_list(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
    let unset = c.g[V1][0] == 0;
    c.g[V0][0] = 0x53_0000;
    if unset {
        c.cycles += 4;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e8)).lw(c);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f168)?;
    } else {
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6e8)).lw(c);
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x8d0)).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 6) as u32);
        c.g[A1][0] &= 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f154)?;
    }
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x6d8)).lw(c);
    Ok(())
}

/// The point-icons list box: as [`set_power_ups_list`], bit 7 and option 2.
#[allow(dead_code)]
fn set_point_icons_list(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(8)).lw(c);
    let unset = c.g[V1][0] == 0;
    c.g[V0][0] = 0x53_0000;
    if unset {
        c.cycles += 4;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6ec)).lw(c);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f1a0)?;
        c.g[V0][0] = 0x53_0000;
    } else {
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x6ec)).lw(c);
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x8d0)).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 7) as u32);
        c.g[A1][0] &= 1;
        call_ptr(c, 0x0039_a7a8, 0x001a_f18c)?;
        c.g[V0][0] = 0x53_0000;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};

    #[test]
    fn cfe_state_select_persona_on_submit_result_matches() {
        check(
            ssx3_ee::s03::f_001adc30__generated,
            cfe_state_select_persona_on_submit_result,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x1adc30, 0);
                let this = DATA + 0x2000;
                let msg = DATA + 0x3000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = msg as u64;
                c.pc = 0x001a_dc30;
                let codes = [
                    SUBMIT_CODE_FILT as u32,
                    SUBMIT_CODE_INAM as u32,
                    SUBMIT_CODE_DUPL as u32,
                    0x1234_5678,
                ];
                c.bus.write32(msg + 8, codes[(seed % 4) as usize]);
                c.bus.write32(msg, seed);
                c.bus.write32(this + 0x6d0, (seed / 4) % 2);
                c.bus.write32(this + 0x10, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn cfe_state_challenge_popup_init_matches() {
        check(
            ssx3_ee::s03::f_001ade88__generated,
            cfe_state_challenge_popup_init,
            30,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x1ade88, 0);
                let this = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = this as u64;
                c.pc = 0x001a_de88;
                c.bus.write32(this + 8, vt);
                c.bus.write16(vt + 0x100, 0x20);
                c.bus.write32(vt + 0x104, 0x0077_7000);
                c.bus.write32(this + 0x10, DATA + 0x4000);
                c.bus
                    .write32(this + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x5000 });
                c.bus
                    .write32(this + 0x6d4, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x6000);
                c.bus.write32(DATA + 0x6000, seed % 2);
                c.bus.write32(DATA + 0x6000 + 0x98, DATA + 0x7000);
            },
        );
    }

    #[test]
    fn cfe_state_challenge_popup_update_matches() {
        check(
            ssx3_ee::s03::f_001ae0e0__generated,
            cfe_state_challenge_popup_update,
            20,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x1ae0e0, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.pc = 0x001a_e0e0;
            },
        );
    }
}
