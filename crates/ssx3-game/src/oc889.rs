//! Staging module oc889 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc889`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
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
        0x001b_a4e0,
        cfe_state_create_step2_update,
        "cFEStateCreateStep2_Update",
    ),
    (
        0x001b_9820,
        cfe_state_create_step1_on_keyboard_done,
        "cFEStateCreateStep1_OnKeyboardDone",
    ),
    (
        0x001b_a280,
        cfe_state_create_step1_submit_edits,
        "cFEStateCreateStep1_SubmitEdits",
    ),
    (
        0x001b_9950,
        cfe_state_create_step1_set_truncated_text,
        "cFEStateCreateStep1_SetTruncatedText",
    ),
];

/// `NetFEState::Update` (0x1a8770).
const NET_FE_STATE_UPDATE: Callee = Callee {
    addr: 0x001a_8770,
    f: ee::s03::f_001a8770,
};

/// `NetFEState::MakePasswordMask` (0x1a98b8).
const NET_MAKE_PASSWORD_MASK: Callee = Callee {
    addr: 0x001a_98b8,
    f: ee::s03::f_001a98b8,
};

/// `cFEStateCreateStep1::SetTruncatedText` (0x1b9950).
const SET_TRUNCATED_TEXT: Callee = Callee {
    addr: 0x001b_9950,
    f: ee::s03::f_001b9950,
};

/// `cFEStateCreateStep1::DetectChanges` (0x1ba358).
const DETECT_CHANGES: Callee = Callee {
    addr: 0x001b_a358,
    f: ee::s03::f_001ba358,
};

/// `cFEStateSelectPersona::ctor` (0x1ad130).
const SELECT_PERSONA_CTOR: Callee = Callee {
    addr: 0x001a_d130,
    f: ee::s03::f_001ad130,
};

/// `NetFEState::GotoNextState` (0x1a8a40).
const NET_GOTO_NEXT_STATE: Callee = Callee {
    addr: 0x001a_8a40,
    f: ee::s03::f_001a8a40,
};

/// `NetFEState::GetPopup` (0x1a9c50).
const NET_GET_POPUP: Callee = Callee {
    addr: 0x001a_9c50,
    f: ee::s03::f_001a9c50,
};

/// `NetFEState::ShowPopup` (0x1a9bc0).
const NET_SHOW_POPUP: Callee = Callee {
    addr: 0x001a_9bc0,
    f: ee::s03::f_001a9bc0,
};

/// `cFEStateCreateStep1` fields (`this` is the state, in s0 or s1).
const SELF_KEYBOARD: u32 = 0x69c;
const SELF_KEYBOARD_TEXT: u32 = 0x6a4;
const SELF_ACCOUNT: u32 = 0x6d0;
const SELF_MASKED_TEXT: u32 = 0x5c;
const SELF_POPUP_ARG: u32 = 0x10;

/// `cKeyboardPopup` fields.
const KEYBOARD_MISC: u32 = 0x18;
const KEYBOARD_EDIT_STRING: u32 = 0x74;

/// `cFEAccount` fields: the two password strings and the mail string.
const ACCOUNT_PASSWORD: u32 = 0x4;
const ACCOUNT_CONFIRM_PASSWORD: u32 = 0x8;
const ACCOUNT_MAIL: u32 = 0xc;

/// `cFEStateCreateStep2::Update(void)` (0x1ba4e0): forwards to the base
/// `NetFEState::Update`.
pub fn cfe_state_create_step2_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        NET_FE_STATE_UPDATE.call(c, 0x001b_a4f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The masked-display refresh shared by the two password cases: when the
/// keyboard has a text widget, rebuild the masked copy of the password from
/// the edit string and push it to the widget. `mask_ret` and `text_ret` are
/// the return addresses of the two calls in the case's own code.
fn keyboard_done_mask(c: &mut Cpu, this: u32, mask_ret: u32, text_ret: u32) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(this).off(SELF_KEYBOARD_TEXT).lw(c);
    let has_text = c.g[V0][0] != 0;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(KEYBOARD_EDIT_STRING));
    if has_text {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        NET_MAKE_PASSWORD_MASK.call(c, mask_ret)?;
        c.g[A0][0] = Addr(this).off(SELF_KEYBOARD_TEXT).lw(c);
        c.g[A1][0] = sx(this.wrapping_add(SELF_MASKED_TEXT));
        call_ptr(c, 0x003a_0c50, text_ret)?;
    }
    Ok(())
}

/// `miscData == 11`: the password field. Refreshes the mask, then copies the
/// edit string into `mpAccount->mPassword` (`cBXString::operator=`).
fn keyboard_done_password(c: &mut Cpu, this: u32) -> Flow {
    keyboard_done_mask(c, this, 0x001b_988c, 0x001b_9898)?;
    c.cycles += 7;
    c.g[A1][0] = Addr(this).off(SELF_KEYBOARD).lw(c);
    c.g[A0][0] = Addr(this).off(SELF_ACCOUNT).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(KEYBOARD_EDIT_STRING));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(ACCOUNT_PASSWORD));
    call_ptr(c, 0x0031_84e8, 0x001b_98ac)?;
    Ok(())
}

/// `miscData == 12`: the confirm-password field, the same as the password
/// case at `mpAccount->mConfirmPassword`.
fn keyboard_done_confirm(c: &mut Cpu, this: u32) -> Flow {
    keyboard_done_mask(c, this, 0x001b_98c8, 0x001b_98d4)?;
    c.cycles += 7;
    c.g[A1][0] = Addr(this).off(SELF_KEYBOARD).lw(c);
    c.g[A0][0] = Addr(this).off(SELF_ACCOUNT).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(KEYBOARD_EDIT_STRING));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(ACCOUNT_CONFIRM_PASSWORD));
    call_ptr(c, 0x0031_84e8, 0x001b_98e8)?;
    Ok(())
}

/// `miscData == 10`: the text widget gets the edit string, then the account
/// itself (`mpAccount`, offset 0) takes it.
fn keyboard_done_account(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.g[A1][0] = Addr(this).off(SELF_KEYBOARD).lw(c);
    } else {
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(KEYBOARD_EDIT_STRING));
        call_ptr(c, 0x003a_0c50, 0x001b_9900)?;
        c.g[A1][0] = Addr(this).off(SELF_KEYBOARD).lw(c);
    }
    c.cycles += 5;
    c.g[A0][0] = Addr(this).off(SELF_ACCOUNT).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(KEYBOARD_EDIT_STRING));
    call_ptr(c, 0x0031_84e8, 0x001b_9910)?;
    Ok(())
}

/// `miscData == 13`: the mail field. Copies the edit string into
/// `mpAccount->mMail` and, when the mail is not empty, refreshes the text
/// widget through `SetTruncatedText`.
fn keyboard_done_mail(c: &mut Cpu, this: u32) -> Flow {
    c.cycles += 8;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(KEYBOARD_EDIT_STRING));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(ACCOUNT_MAIL));
    call_ptr(c, 0x0031_84e8, 0x001b_9924)?;
    c.g[V1][0] = Addr(this).off(SELF_ACCOUNT).lw(c);
    c.g[A2][0] = Addr(c.g[V1][0] as u32).off(ACCOUNT_MAIL).lw(c);
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_sub(0x8)).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S0][0];
    if !empty {
        c.cycles += 2;
        c.g[A1][0] = Addr(c.g[A0][0] as u32).off(SELF_KEYBOARD_TEXT).lw(c);
        SET_TRUNCATED_TEXT.call(c, 0x001b_9940)?;
    }
    c.cycles += 1;
    Ok(())
}

/// The cases after the two password checks, in the order the generated
/// dispatch tests them: `misc` below 12 (only 10 acts), then 12, then 13.
fn keyboard_done_dispatch(c: &mut Cpu, this: u32, misc: u64) -> Flow {
    c.cycles += 2;
    let high = c.g[V0][0] == 0;
    c.g[V0][0] = 0xa;
    if !high {
        c.cycles += 2;
        if misc == 10 {
            c.g[A0][0] = Addr(this).off(SELF_KEYBOARD_TEXT).lw(c);
            return keyboard_done_account(c, this);
        }
        c.cycles += 2;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = 0xc;
    let is_confirm = misc == c.g[V0][0];
    c.g[V0][0] = 0xd;
    if is_confirm {
        return keyboard_done_confirm(c, this);
    }
    c.cycles += 2;
    if misc == c.g[V0][0] {
        c.g[A0][0] = Addr(this).off(SELF_ACCOUNT).lw(c);
        return keyboard_done_mail(c, this);
    }
    c.cycles += 2;
    Ok(())
}

/// `cFEStateCreateStep1::OnKeyboardDone(this)` (0x1b9820): when the keyboard
/// closes, stores its edit string into the account field its `miscData`
/// selects (11 password, 12 confirm, 10 account, 13 mail).
pub fn cfe_state_create_step1_on_keyboard_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xb;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = Addr(this).off(SELF_KEYBOARD).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(KEYBOARD_MISC).lw(c);
        let misc = c.g[V1][0];
        let is_password = misc == c.g[V0][0];
        c.g[V0][0] = (misc < 0xc) as u64;
        if is_password {
            keyboard_done_password(c, this)?;
        } else {
            keyboard_done_dispatch(c, this, misc)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep1::SubmitEdits(this)` (0x1ba280): runs the change check.
/// When a field changed it posts the popup; otherwise it selects the persona
/// and moves on to the next state.
pub fn cfe_state_create_step1_submit_edits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let this = c.g[S1][0] as u32;
        DETECT_CHANGES.call(c, 0x001b_a298)?;
        c.g[V1][0] = Addr(this).off(SELF_ACCOUNT).lw(c);
        let account = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(account).off(0x4c).lw(c);
        let mut changed = c.g[V0][0] != 0;
        if !changed {
            c.cycles += 3;
            c.g[V0][0] = Addr(account).off(0x44).lw(c);
            changed = c.g[V0][0] != 0;
            if !changed {
                c.cycles += 3;
                c.g[V0][0] = Addr(account).off(0x48).lw(c);
                changed = c.g[V0][0] != 0;
                c.g[A1][0] = 0x460000;
            }
        }
        if changed {
            submit_edits_popup(c, frame, this)?;
        } else {
            submit_edits_persona(c, frame, this)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The no-change branch: select the persona for the account and go on to the
/// next state.
fn submit_edits_persona(c: &mut Cpu, frame: Frame, this: u32) -> Flow {
    c.cycles += 13;
    c.g[A0][0] = 0x6d8;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d68));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001b_a2d4)?;
    c.g[A1][0] = Addr(this).off(SELF_POPUP_ARG).lw(c);
    c.g[A0][0] = c.g[V0][0];
    SELECT_PERSONA_CTOR.call(c, 0x001b_a2e0)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    NET_GOTO_NEXT_STATE.call(c, 0x001b_a2ec)?;
    frame.restore(c, S0, 0x20);
    Ok(())
}

/// The changed branch: fetch the popup, clear its results, show it with the
/// account's message, then the account's own handler.
fn submit_edits_popup(c: &mut Cpu, frame: Frame, this: u32) -> Flow {
    c.cycles += 20;
    c.g[A0][0] = c.g[S1][0];
    NET_GET_POPUP.call(c, 0x001b_a2fc)?;
    c.g[S0][0] = c.g[V0][0];
    let popup = c.g[S0][0] as u32;
    c.g[A0][0] = 0x460000;
    Addr(popup).off(0xbc).sw(c, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x49d8));
    call_ptr(c, 0x0031_7670, 0x001b_a310)?;
    c.g[V1][0] = 0x3;
    c.g[A2][0] = 0x81d;
    Addr(popup).off(0x164).sw(c, 0);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S1][0];
    let v0 = c.g[V0][0];
    Addr(popup).off(0xc4).sw(c, v0);
    let v1 = c.g[V1][0];
    Addr(popup).off(0xc0).sw(c, v1);
    let a2 = c.g[A2][0];
    Addr(popup).off(0x150).sw(c, a2);
    NET_SHOW_POPUP.call(c, 0x001b_a334)?;
    c.g[A1][0] = Addr(this).off(SELF_ACCOUNT).lw(c);
    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
    call_ptr(c, 0x0025_bf18, 0x001b_a340)?;
    frame.restore(c, S0, 0x20);
    Ok(())
}

/// The float register `r` as an `FpuF32`.
fn truncated_fpr(c: &Cpu, r: usize) -> crate::ps2f::FpuF32 {
    crate::ps2f::FpuF32(c.f[r])
}

/// `cFEStateCreateStep1::SetTruncatedText(this, text, email)` (0x1b9950):
/// fits the mail address into the text widget. It copies `email` to a frame
/// buffer and, while the measured width is at least the field width, cuts
/// characters off its end; then it shows the copy (with "..." appended when
/// it was cut) or the whole address.
pub fn cfe_state_create_step1_set_truncated_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 70;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x100);
        frame.save(c, S1, 0xf0);
        frame.save(c, S2, 0xe0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0xd0);
        c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
        frame.save(c, S4, 0xc0);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[S6][0] = 0;
        let text = c.g[S1][0] as u32;
        c.g[V1][0] = Addr(text).off(0x8).lw(c);
        // The 8 bytes at text+0x50 go to the frame's buffer at sp+0, the word
        // at text+0x58 to sp+8.
        c.g[V0][0] = Addr(text).off(0x50).read_u64_unaligned(c);
        c.g[A0][0] = Addr(text).off(0x58).lw(c);
        let buf = sp(c);
        let v0 = c.g[V0][0];
        Addr(buf).write_u64_unaligned(c, v0);
        let a0 = c.g[A0][0];
        Addr(buf.wrapping_add(8)).sw(c, a0);
        let v1 = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(v1).off(0xa0).lh(c);
        c.g[V0][0] = Addr(v1).off(0xa4).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        // The widget's method (a vtable slot) takes the text as `this`.
        call_ptr(c, target, 0x001b_99b0)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003a_04f0, 0x001b_99b8)?;
        // Width of the font scaled by the text's measure factors.
        let font = c.g[V0][0] as u32;
        let fv = Addr(sp(c)).read_fpu(c);
        abi::set_fpr(c, 2, fv);
        c.g[A0][0] = c.g[S3][0];
        let fv = Addr(sp(c).wrapping_add(4)).read_fpu(c);
        abi::set_fpr(c, 3, fv);
        let fv = Addr(font.wrapping_add(0x30)).read_fpu(c);
        abi::set_fpr(c, 0, fv);
        let fv = Addr(font.wrapping_add(0x34)).read_fpu(c);
        abi::set_fpr(c, 1, fv);
        let f0 = truncated_fpr(c, 0) * truncated_fpr(c, 2);
        let f1 = truncated_fpr(c, 1) * truncated_fpr(c, 3);
        abi::set_fpr(c, 0, f0);
        abi::set_fpr(c, 1, f1);
        Addr(font.wrapping_add(0x38)).write_fpu(c, f0);
        Addr(font.wrapping_add(0x3c)).write_fpu(c, f1);
        call_ptr(c, 0x0041_6810, 0x001b_99e0)?;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x20));
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        call_ptr(c, 0x0041_66f4, 0x001b_99f4)?;
        c.g[S4][0] = c.g[S0][0];
        c.g[V1][0] = 0x4a_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bd0));
        call_ptr(c, 0x003a_04f0, 0x001b_9a08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = sx(sp(c).wrapping_add(0x80));
        let font = c.g[A0][0] as u32;
        let fv = Addr(font.wrapping_add(0x38)).read_fpu(c);
        abi::set_fpr(c, 12, fv);
        c.g[A1][0] = c.g[S5][0];
        let fv = Addr(font.wrapping_add(0x3c)).read_fpu(c);
        abi::set_fpr(c, 13, fv);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_1fb0, 0x001b_9a24)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003a_04f0, 0x001b_9a2c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        let font = c.g[A0][0] as u32;
        let fv = Addr(font.wrapping_add(0x38)).read_fpu(c);
        abi::set_fpr(c, 12, fv);
        c.g[A2][0] = sx(sp(c).wrapping_add(0x70));
        let fv = Addr(font.wrapping_add(0x3c)).read_fpu(c);
        abi::set_fpr(c, 13, fv);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_1fb0, 0x001b_9a48)?;
        let fv = Addr(sp(c).wrapping_add(0x10)).read_fpu(c);
        abi::set_fpr(c, 1, fv);
        let fv = Addr(sp(c).wrapping_add(0x88)).read_fpu(c);
        abi::set_fpr(c, 0, fv);
        let fv = Addr(sp(c).wrapping_add(0x78)).read_fpu(c);
        abi::set_fpr(c, 2, fv);
        let width = truncated_fpr(c, 1) - truncated_fpr(c, 0);
        abi::set_fpr(c, 1, width);
        let (w, lim) = (truncated_fpr(c, 1), truncated_fpr(c, 2));
        let mut too_wide = abi::c_lt_s(c, w, lim);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        if too_wide {
            c.cycles += 2;
            c.g[S6][0] = 1;
            // Cut one character per pass until the text fits.
            while too_wide {
                c.cycles += 19;
                c.g[A0][0] = c.g[S1][0];
                let cut = c.g[V0][0] as u32;
                Addr(cut).sb(c, 0);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x003a_04f0, 0x001b_9a80)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                let font = c.g[A0][0] as u32;
                let fv = Addr(font.wrapping_add(0x38)).read_fpu(c);
                abi::set_fpr(c, 12, fv);
                c.g[A2][0] = sx(sp(c).wrapping_add(0x70));
                let fv = Addr(font.wrapping_add(0x3c)).read_fpu(c);
                abi::set_fpr(c, 13, fv);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0039_1fb0, 0x001b_9a9c)?;
                let fv = Addr(sp(c).wrapping_add(0x10)).read_fpu(c);
                abi::set_fpr(c, 1, fv);
                let fv = Addr(sp(c).wrapping_add(0x88)).read_fpu(c);
                abi::set_fpr(c, 0, fv);
                let fv = Addr(sp(c).wrapping_add(0x78)).read_fpu(c);
                abi::set_fpr(c, 2, fv);
                let width = truncated_fpr(c, 1) - truncated_fpr(c, 0);
                abi::set_fpr(c, 1, width);
                let (w, lim) = (truncated_fpr(c, 1), truncated_fpr(c, 2));
                too_wide = abi::c_lt_s(c, w, lim);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                if too_wide && event_due(c) {
                    deliver_event_here(c, 0x001b_9a70);
                }
            }
        }
        // Show the cut text ("..." appended) or the whole address.
        c.cycles += 2;
        let whole = c.g[S6][0] == 0;
        c.g[A1][0] = c.g[S5][0];
        if whole {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x003a_0c50, 0x001b_9aec)?;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0041_62d0, 0x001b_9acc)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x003a_0c50, 0x001b_9ad8)?;
        }
        frame.restore(c, S0, 0x100);
        c.cycles += 9;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word in the machine's RAM: the fakes only run where it is set.
    const OC889_GATE: u32 = DATA + 0x20_0000;
    const OC889_MAGIC: u32 = 0x0889_0889;

    macro_rules! oc889_fake {
        ($fake:ident, $prev:ident, $gen:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC889_GATE) != OC889_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc889_fake!(
        oc889_fake_update,
        OC889_PREV_UPDATE,
        ee::s03::f_001a8770,
        0x001a_8770
    );
    oc889_fake!(
        oc889_fake_mask,
        OC889_PREV_MASK,
        ee::s03::f_001a98b8,
        0x001a_98b8
    );
    oc889_fake!(
        oc889_fake_detect,
        OC889_PREV_DETECT,
        ee::s03::f_001ba358,
        0x001b_a358
    );
    oc889_fake!(
        oc889_fake_persona,
        OC889_PREV_PERSONA,
        ee::s03::f_001ad130,
        0x001a_d130
    );
    oc889_fake!(
        oc889_fake_show,
        OC889_PREV_SHOW,
        ee::s03::f_001a9bc0,
        0x001a_9bc0
    );
    oc889_fake!(
        oc889_fake_str,
        OC889_PREV_STR,
        ee::s10::f_003184e8,
        0x0031_84e8
    );
    oc889_fake!(
        oc889_fake_settext,
        OC889_PREV_SETTEXT,
        ee::s13::f_003a0c50,
        0x003a_0c50
    );
    oc889_fake!(
        oc889_fake_alloc,
        OC889_PREV_ALLOC,
        ee::s10::f_00317670,
        0x0031_7670
    );
    oc889_fake!(
        oc889_fake_alloc2,
        OC889_PREV_ALLOC2,
        ee::s10::f_00317d70,
        0x0031_7d70
    );
    oc889_fake!(
        oc889_fake_ui,
        OC889_PREV_UI,
        ee::s06::f_0025bf18,
        0x0025_bf18
    );
    oc889_fake!(
        oc889_fake_font,
        OC889_PREV_FONT,
        ee::s13::f_003a04f0,
        0x003a_04f0
    );

    /// `Font_MeasureText` (0x391fb0): each call stores its call count as the
    /// measured width at the caller's sp+0x10, so the shortening loop of
    /// `SetTruncatedText` ends after a pass or two.
    fn oc889_fake_measure(c: &mut Cpu) {
        if c.bus.read32(OC889_GATE) != OC889_MAGIC {
            return fakes::fall_back(c, &OC889_PREV_MEASURE, ee::s12::f_00391fb0);
        }
        let n = c.bus.read32(OC889_GATE + 4) + 1;
        c.bus.write32(OC889_GATE + 4, n);
        c.bus
            .write32(sp(c).wrapping_add(0x10), (n as f32).to_bits());
        fakes::fake_call(c, 0x0039_1fb0);
    }

    static OC889_PREV_MEASURE: AtomicUsize = AtomicUsize::new(0);

    /// Hooks the fakes into their override slots (once per process).
    fn oc889_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8770),
            oc889_fake_update,
            &OC889_PREV_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A98B8),
            oc889_fake_mask,
            &OC889_PREV_MASK,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001BA358),
            oc889_fake_detect,
            &OC889_PREV_DETECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001AD130),
            oc889_fake_persona,
            &OC889_PREV_PERSONA,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A9BC0),
            oc889_fake_show,
            &OC889_PREV_SHOW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003184E8),
            oc889_fake_str,
            &OC889_PREV_STR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc889_fake_settext,
            &OC889_PREV_SETTEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc889_fake_alloc,
            &OC889_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc889_fake_alloc2,
            &OC889_PREV_ALLOC2,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025BF18),
            oc889_fake_ui,
            &OC889_PREV_UI,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A04F0),
            oc889_fake_font,
            &OC889_PREV_FONT,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391FB0),
            oc889_fake_measure,
            &OC889_PREV_MEASURE,
        );
    }

    /// The scrambled machine with the gate set (its fakes run here only).
    fn oc889_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        machine(c, seed, salt, calls);
        c.bus.write32(OC889_GATE, OC889_MAGIC);
    }

    #[test]
    fn cfe_state_create_step1_set_truncated_text_matches() {
        oc889_install_fakes();
        check(
            ee::s03::f_001b9950__generated,
            cfe_state_create_step1_set_truncated_text,
            8,
            &|c, seed| {
                oc889_machine(c, seed, 0x001b_9950, 3);
                c.bus.write32(OC889_GATE + 4, 0);
                let this = DATA + 0x1000;
                let text = DATA + 0x2000;
                let email = DATA + 0x3000;
                let widget = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = text as u64;
                c.g[A2][0] = email as u64;
                for (i, b) in b"abcdefgh\0".iter().enumerate() {
                    c.bus.write8(email + i as u32, *b);
                }
                c.bus.write32(text + 0x8, widget);
                c.bus.write16(widget + 0xa0, 0);
                c.bus.write32(widget + 0xa4, fakes::FAKE);
                c.bus.write32(text + 0x50, seed.wrapping_mul(0x0101_0101));
                c.bus.write32(text + 0x54, 0x1234_5678);
                c.bus.write32(text + 0x58, 0x2468_ace0 ^ seed);
                // The frame's float slots: f0 = 0, f2 = 0.5 (no cut) or 3.0 (one cut).
                c.bus.write32(crate::difftest::SP - 0x88, 0.0f32.to_bits());
                let limit = if seed % 2 == 0 { 0.5f32 } else { 3.0f32 };
                c.bus.write32(crate::difftest::SP - 0x98, limit.to_bits());
                c.pc = 0x001b_9950;
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_update_matches() {
        oc889_install_fakes();
        check(
            ee::s03::f_001ba4e0__generated,
            cfe_state_create_step2_update,
            6,
            &|c, seed| {
                oc889_machine(c, seed, 0x001b_a4e0, 2);
                c.pc = 0x001b_a4e0;
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_on_keyboard_done_matches() {
        oc889_install_fakes();
        check(
            ee::s03::f_001b9820__generated,
            cfe_state_create_step1_on_keyboard_done,
            14,
            &|c, seed| {
                oc889_machine(c, seed, 0x001b_9820, 4);
                let this = DATA + 0x1000;
                let kb = DATA + 0x2000;
                let account = DATA + 0x4000;
                let mail = DATA + 0x5010;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + SELF_KEYBOARD, kb);
                c.bus.write32(
                    this + SELF_KEYBOARD_TEXT,
                    if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
                );
                c.bus.write32(this + SELF_ACCOUNT, account);
                c.bus.write32(account + ACCOUNT_MAIL, mail);
                // An empty mail skips SetTruncatedText (no override slot yet).
                c.bus.write32(mail - 8, 0);
                let misc = [10, 11, 12, 13, 3, 9, 15][(seed % 7) as usize];
                c.bus.write32(kb + KEYBOARD_MISC, misc);
                c.pc = 0x001b_9820;
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_submit_edits_matches() {
        oc889_install_fakes();
        check(
            ee::s03::f_001ba280__generated,
            cfe_state_create_step1_submit_edits,
            8,
            &|c, seed| {
                oc889_machine(c, seed, 0x001b_a280, 4);
                let this = DATA + 0x1000;
                let account = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + SELF_ACCOUNT, account);
                c.bus.write32(this + SELF_POPUP_ARG, DATA + 0x3000);
                c.bus.write32(account + 0x4c, (seed % 4 == 0) as u32);
                c.bus.write32(account + 0x44, (seed % 4 == 1) as u32);
                c.bus.write32(account + 0x48, (seed % 4 == 2) as u32);
                c.pc = 0x001b_a280;
            },
        );
    }
}
