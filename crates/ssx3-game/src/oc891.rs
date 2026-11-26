//! Staging module oc891 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc891`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_b5d0,
        cfe_state_create_step2_fill_month_list,
        "cFEStateCreateStep2_FillMonthList",
    ),
    (
        0x001b_baa8,
        cfe_state_create_step2_validate_fields,
        "cFEStateCreateStep2_ValidateFields",
    ),
    (
        0x001b_bac8,
        cfe_state_create_step2_check_age,
        "cFEStateCreateStep2_CheckAge",
    ),
    (
        0x001b_bc40,
        cfe_state_create_step2_check_legal_accepted,
        "cFEStateCreateStep2_CheckLegalAccepted",
    ),
    (
        0x001b_bde0,
        cfe_state_create_duplicate_update,
        "cFEStateCreateDuplicate_Update",
    ),
    (
        0x001b_be00,
        cfe_state_create_duplicate_on_widget_created,
        "cFEStateCreateDuplicate_OnWidgetCreated",
    ),
];

/// `StrHash` (GetHashValue32 of a string at `a0`).
pub(crate) const FE_CS2_STR_HASH: u32 = 0x0031_7670;
/// `sprintf`.
pub(crate) const FE_CS2_SPRINTF: u32 = 0x0041_7828;
/// `cUIListBox_addEntryByStringID(box, hash, id)`.
pub(crate) const FE_CS2_LIST_ADD_BY_STRING_ID: u32 = 0x0039_a568;
/// `cUIListBox_getEntryByData(box, data, &out)`.
pub(crate) const FE_CS2_LIST_GET_BY_DATA: u32 = 0x0039_a768;
/// `cUIListBox_setCurrentEntryByIndex(box, index)`.
pub(crate) const FE_CS2_LIST_SET_CURRENT: u32 = 0x0039_a7a8;
/// `cUIText_setTextId(text, hash)`.
pub(crate) const FE_CS2_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;
/// `cUIText_setAsciiString(text, buf)`.
pub(crate) const FE_CS2_TEXT_SET_ASCII: u32 = 0x003a_0c50;
/// `cPopupInfo` config constructor and destructor, and its text setter.
pub(crate) const FE_CS2_POPUP_CFG_CTOR: u32 = 0x001c_55f8;
pub(crate) const FE_CS2_POPUP_CFG_DTOR: u32 = 0x001c_5750;
pub(crate) const FE_CS2_POPUP_CFG_SET_TEXT_ID: u32 = 0x001c_57c0;
/// `NetFEState_GetPopup(self)` and `NetFEState_ShowPopup(self, popup)`.
pub(crate) const FE_CS2_NETFE_GET_POPUP: u32 = 0x001a_9c50;
pub(crate) const FE_CS2_NETFE_SHOW_POPUP: u32 = 0x001a_9bc0;
/// `NetFEState_Update`.
pub(crate) const FE_CS2_NETFE_UPDATE: u32 = 0x001a_8770;

/// `self->field` / `text->field` at `base + off`.
#[inline(always)]
pub(crate) fn fe_cs2_at(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `cFEStateCreateStep2_FillMonthList(self, box, ...)` (0x001bb5d0): fill the
/// month list box with the twelve month names (ids 1 to 12), select the entry
/// whose data is 1 (its index is the byte at `sp`), select index 1 again, and
/// make the box visible through its vtable (`setVisible(1)`, slot 0x48).
pub fn cfe_state_create_step2_fill_month_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 114;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];

        // The box's flags: `mAttr = (mAttr & ~1) | 0x80`, and the out byte is 0.
        c.g[V0][0] = fe_cs2_at(c.g[S0][0], 0x14).lw(c);
        Addr(sp(c)).sb(c, 0);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x80;
        let flags = c.g[V0][0];
        fe_cs2_at(c.g[S0][0], 0x14).sw(c, flags);

        // `AddEntry(box, GetHashValue32(kT_CMN<Month>), i + 1)` for each month.
        for month in 0..12u32 {
            let hash_ret = 0x001b_b604 + 0x1c * month;
            c.g[A0][0] = sx(0x0046_4b30 + 0x10 * month);
            call_ptr(c, FE_CS2_STR_HASH, hash_ret)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = (month + 1) as u64;
            call_ptr(c, FE_CS2_LIST_ADD_BY_STRING_ID, hash_ret + 0x10)?;
        }

        // `GetEntryByData(box, 1, &out)`, then select the entry it found.
        c.g[A2][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, FE_CS2_LIST_GET_BY_DATA, 0x001b_b758)?;
        c.g[A1][0] = Addr(sp(c)).lbu(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, FE_CS2_LIST_SET_CURRENT, 0x001b_b764)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, FE_CS2_LIST_SET_CURRENT, 0x001b_b770)?;

        // `box->setVisible(1)` through the vtable at `box->vt` (slot 0x48).
        c.g[V0][0] = fe_cs2_at(c.g[S0][0], 0x8).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = fe_cs2_at(c.g[V0][0], 0x48).lh(c);
        c.g[V1][0] = fe_cs2_at(c.g[V0][0], 0x4c).lw(c);
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x001b_b788)?;

        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateStep2_ValidateFields(self)` (0x001bbaa8): the form is valid
/// when the legal-age text was accepted (`CheckLegalAccepted` returns nonzero).
pub fn cfe_state_create_step2_validate_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, FE_CS2_CHECK_LEGAL_ACCEPTED, 0x001b_bab8)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// Entry of `cFEStateCreateStep2_CheckLegalAccepted` (0x001bbc40), called by
/// `ValidateFields`.
pub(crate) const FE_CS2_CHECK_LEGAL_ACCEPTED: u32 = 0x001b_bc40;

/// `cFEStateCreateStep2_CheckAge(self, age)` (0x001bbac8): the age check of the
/// create-account form. Under 13 shows the under-age popup (code 2061), 13 to
/// 17 shows the parent-email popup (code 2081) and fills in the parent mail,
/// and 18 and up returns 1 with nothing shown. A shown popup returns 0.
pub fn cfe_state_create_step2_check_age(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S4, 0xe0);
        frame.save(c, S0, 0x120);
        c.g[S4][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x110);
        frame.save(c, S2, 0x100);
        frame.save(c, S3, 0xf0);
        frame.save_ra(c, 0xd0);

        // The popup config on the stack, constructed (and destroyed below).
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, FE_CS2_POPUP_CFG_CTOR, 0x001b_baf4)?;

        if fe_cs2_age_popup(c)? {
            // Show the popup built in `fe_cs2_age_popup` (code in v0), then
            // destroy the config.
            c.cycles += 8;
            c.g[A0][0] = c.g[S4][0];
            let code = c.g[V0][0];
            fe_cs2_at(c.g[S0][0], 0x150).sw(c, code);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, FE_CS2_NETFE_SHOW_POPUP, 0x001b_bc10)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            call_ptr(c, FE_CS2_POPUP_CFG_DTOR, 0x001b_bc1c)?;
            c.g[V0][0] = 0;
        }

        c.cycles += 8;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore(c, S3, 0xf0);
        frame.restore(c, S4, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// The age branches of `CheckAge`. Returns `true` when a popup was built and
/// its code is in `v0` (the caller shows it), `false` for an adult (`v0 = 1`,
/// config already destroyed).
fn fe_cs2_age_popup(c: &mut Cpu) -> Result<bool, Unwound> {
    // `v1 = age < 13` is the branch test; the compiled code leaves it in v1.
    c.g[V1][0] = ((c.g[S0][0] as i64) < 13) as u64;
    if (c.g[S0][0] as i64) < 13 {
        // Under 13: meType 1 with the under-age message and the OK button.
        c.cycles += 21;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, FE_CS2_NETFE_GET_POPUP, 0x001b_bb08)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx(0x0046_4bf0);
        c.g[V0][0] = 1;
        let mete_type = c.g[V0][0];
        fe_cs2_at(c.g[S0][0], 0xbc).sw(c, mete_type);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
        call_ptr(c, FE_CS2_STR_HASH, 0x001b_bb24)?;
        c.g[V1][0] = 3;
        let event_type = c.g[V1][0];
        fe_cs2_at(c.g[S0][0], 0xc0).sw(c, event_type);
        c.g[A0][0] = sx(0x0046_1618);
        let message = c.g[V0][0];
        fe_cs2_at(c.g[S0][0], 0xc4).sw(c, message);
        call_ptr(c, FE_CS2_STR_HASH, 0x001b_bb3c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        call_ptr(c, FE_CS2_POPUP_CFG_SET_TEXT_ID, 0x001b_bb4c)?;
        c.g[V0][0] = 0x80d;
        return Ok(true);
    }

    c.g[V0][0] = ((c.g[S0][0] as i64) < 18) as u64;
    c.cycles += 2;
    c.g[A0][0] = c.g[S4][0];
    if c.g[V0][0] == 0 {
        // 18 and up: nothing to show.
        c.cycles += 5;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, FE_CS2_POPUP_CFG_DTOR, 0x001b_bb68)?;
        c.g[V0][0] = 1;
        return Ok(false);
    }

    // 13 to 17: the parent-email popup, with the entry box prefilled.
    c.cycles += 36;
    c.g[S2][0] = 3;
    call_ptr(c, FE_CS2_NETFE_GET_POPUP, 0x001b_bb78)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[S1][0] = sx(0x0046_4c08);
    c.g[V1][0] = 0x32;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let max_chars = c.g[V1][0];
    fe_cs2_at(c.g[S0][0], 0x180).sw(c, max_chars);
    c.g[A0][0] = c.g[S1][0];
    let mete_type = c.g[V0][0];
    fe_cs2_at(c.g[S0][0], 0xbc).sw(c, mete_type);
    call_ptr(c, FE_CS2_STR_HASH, 0x001b_bb9c)?;
    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
    let title_type = c.g[S2][0];
    fe_cs2_at(c.g[S0][0], 0xc8).sw(c, title_type);
    c.g[A0][0] = c.g[S1][0];
    let title = c.g[V0][0];
    fe_cs2_at(c.g[S0][0], 0xcc).sw(c, title);
    call_ptr(c, FE_CS2_STR_HASH, 0x001b_bbb0)?;
    let entry_title = c.g[V0][0];
    fe_cs2_at(c.g[S0][0], 0xd4).sw(c, entry_title);
    c.g[A1][0] = 2;
    let entry_title_type = c.g[S2][0];
    fe_cs2_at(c.g[S0][0], 0xd0).sw(c, entry_title_type);
    c.g[A0][0] = sx(0x0046_4c20);
    c.g[V0][0] = fe_cs2_at(c.g[S4][0], 0x6d0).lw(c);
    c.g[V1][0] = fe_cs2_at(c.g[V0][0], 0x10).lw(c);
    let box_type = c.g[A1][0];
    fe_cs2_at(c.g[S0][0], 0xd8).sw(c, box_type);
    let parent_mail = c.g[V1][0];
    fe_cs2_at(c.g[S0][0], 0xdc).sw(c, parent_mail);
    call_ptr(c, FE_CS2_STR_HASH, 0x001b_bbd8)?;
    let message_type = c.g[S2][0];
    fe_cs2_at(c.g[S0][0], 0xc0).sw(c, message_type);
    c.g[A0][0] = sx(0x0046_1618);
    let message = c.g[V0][0];
    fe_cs2_at(c.g[S0][0], 0xc4).sw(c, message);
    call_ptr(c, FE_CS2_STR_HASH, 0x001b_bbec)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 0;
    call_ptr(c, FE_CS2_POPUP_CFG_SET_TEXT_ID, 0x001b_bbfc)?;
    c.g[V0][0] = 0x821;
    Ok(true)
}

/// `cFEStateCreateStep2_CheckLegalAccepted(self)` (0x001bbc40): true when the
/// account's legal text was accepted. Otherwise shows the "accept legal" popup
/// (code 2061) and returns 0.
pub fn cfe_state_create_step2_check_legal_accepted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S2, 0xe0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x100);
        frame.save(c, S1, 0xf0);
        frame.save_ra(c, 0xd0);

        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, FE_CS2_POPUP_CFG_CTOR, 0x001b_bc60)?;

        // `self->mpAccount->mLegalAccepted`.
        c.g[V1][0] = fe_cs2_at(c.g[S2][0], 0x6d0).lw(c);
        c.g[V0][0] = fe_cs2_at(c.g[V1][0], 0x40).lw(c);
        let not_accepted = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[SP][0];

        if !not_accepted {
            c.cycles += 4;
            c.g[A1][0] = 2;
            call_ptr(c, FE_CS2_POPUP_CFG_DTOR, 0x001b_bc78)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 28;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, FE_CS2_NETFE_GET_POPUP, 0x001b_bc88)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_4c38);
            c.g[V0][0] = 1;
            let mete_type = c.g[V0][0];
            fe_cs2_at(c.g[S0][0], 0xbc).sw(c, mete_type);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
            call_ptr(c, FE_CS2_STR_HASH, 0x001b_bca4)?;
            c.g[V1][0] = 3;
            let message_type = c.g[V1][0];
            fe_cs2_at(c.g[S0][0], 0xc0).sw(c, message_type);
            c.g[A0][0] = sx(0x0046_1618);
            let message = c.g[V0][0];
            fe_cs2_at(c.g[S0][0], 0xc4).sw(c, message);
            call_ptr(c, FE_CS2_STR_HASH, 0x001b_bcbc)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            call_ptr(c, FE_CS2_POPUP_CFG_SET_TEXT_ID, 0x001b_bccc)?;
            c.g[V0][0] = 0x80d;
            c.g[A0][0] = c.g[S2][0];
            let code = c.g[V0][0];
            fe_cs2_at(c.g[S0][0], 0x150).sw(c, code);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, FE_CS2_NETFE_SHOW_POPUP, 0x001b_bce0)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            call_ptr(c, FE_CS2_POPUP_CFG_DTOR, 0x001b_bcec)?;
            c.g[V0][0] = 0;
        }

        c.cycles += 6;
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S0, 0x100);
        frame.restore(c, S1, 0xf0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateDuplicate_Update(self)` (0x001bbde0): the duplicate-account
/// screen's update is the network state's update.
pub fn cfe_state_create_duplicate_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, FE_CS2_NETFE_UPDATE, 0x001b_bdf0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCreateDuplicate_OnWidgetCreated(self, text)` (0x001bbe00): one of
/// the duplicate-rider widgets was created; give its text the right label (a
/// fixed caption string, or one of the four weekday names).
pub fn cfe_state_create_duplicate_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x40);
        frame.save(c, S0, 0x80);

        let early = fe_cs2_widget_caption(c)?;
        if !early {
            // Fell through to the weekday loop's end.
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x80);

        c.cycles += 6;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// What the caption search does after the title and menu-name tests.
enum CaptionNext {
    /// The fixed caption is in `a0`: hash it and set the text id.
    Caption,
    /// Go to the weekday loop.
    Loop,
}

/// The caption choice of `OnWidgetCreated`. Returns `true` when the fixed
/// caption was set (the function then returns at once), `false` when the
/// weekday loop ran.
fn fe_cs2_widget_caption(c: &mut Cpu) -> Result<bool, Unwound> {
    // "SelectNew": the widget only carries its state in misc data 4.
    c.g[A0][0] = sx(0x0046_4c68);
    c.g[S0][0] = fe_cs2_at(c.g[S2][0], 0x38).lw(c);
    call_ptr(c, FE_CS2_STR_HASH, 0x001b_be30)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 3;
        c.g[V0][0] = 4;
        let misc = c.g[V0][0];
        fe_cs2_at(c.g[S2][0], 0x18).sw(c, misc);
        return Ok(false);
    }

    c.g[V0][0] = fe_cs2_at(c.g[S1][0], 0x6d0).lw(c);
    c.cycles += 2;
    let next = if c.g[V0][0] != 0 {
        c.g[S1][0] = 0;
        CaptionNext::Loop
    } else {
        fe_cs2_widget_title(c)?
    };
    match next {
        CaptionNext::Caption => {
            c.cycles += 7;
            call_ptr(c, FE_CS2_STR_HASH, 0x001b_bea0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, FE_CS2_TEXT_SET_TEXT_ID, 0x001b_beac)?;
            return Ok(true);
        }
        CaptionNext::Loop => {}
    }

    // The weekday loop: "SelectNamepct_d" with 0 to 3 (a name per weekday).
    c.cycles += 1;
    c.g[S3][0] = 0x0046_0000;
    loop {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 4) as u64;
        let done = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[SP][0];
        if done {
            break;
        }
        c.cycles += 8;
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4cb8));
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, FE_CS2_SPRINTF, 0x001b_bed4)?;
        c.g[S0][0] = fe_cs2_at(c.g[S2][0], 0x38).lw(c);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, FE_CS2_STR_HASH, 0x001b_bee0)?;
        if c.g[S0][0] != c.g[V0][0] {
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x001b_bebc);
            }
            continue;
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, FE_CS2_TEXT_SET_ASCII, 0x001b_bef4)?;
        let index = c.g[S1][0];
        fe_cs2_at(c.g[S2][0], 0x18).sw(c, index);
        break;
    }
    Ok(false)
}

/// The title and menu-name captions for widgets of a duplicate screen with no
/// account (`mbAccount == 0`). Sets `a0` to the caption that matches, or goes
/// to the weekday loop.
fn fe_cs2_widget_title(c: &mut Cpu) -> Result<CaptionNext, Unwound> {
    // "TextTitle": the duplicate-user title.
    c.cycles += 6;
    c.g[A0][0] = sx(0x0046_1520);
    c.g[S0][0] = fe_cs2_at(c.g[S2][0], 0x38).lw(c);
    call_ptr(c, FE_CS2_STR_HASH, 0x001b_be5c)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 3;
        c.g[A0][0] = sx(0x0046_4c78);
        return Ok(CaptionNext::Caption);
    }

    c.g[V0][0] = fe_cs2_at(c.g[S1][0], 0x6d0).lw(c);
    c.cycles += 2;
    let has_account = c.g[V0][0] != 0;
    c.g[S1][0] = 0;
    if has_account {
        return Ok(CaptionNext::Loop);
    }

    // "StaticMenuName": the suggested-user caption.
    c.cycles += 6;
    c.g[A0][0] = sx(0x0046_4c90);
    c.g[S0][0] = fe_cs2_at(c.g[S2][0], 0x38).lw(c);
    call_ptr(c, FE_CS2_STR_HASH, 0x001b_be88)?;
    let mismatch = c.g[S0][0] != c.g[V0][0];
    c.g[S1][0] = 0;
    if mismatch {
        c.cycles += 1;
        return Ok(CaptionNext::Loop);
    }
    c.cycles += 2;
    c.g[A0][0] = sx(0x0046_4ca0);
    Ok(CaptionNext::Caption)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC891_MAGIC` in the machine's RAM (otherwise the body they replaced runs).
    const OC891_CTRL: u32 = DATA + 0x000f_0000;
    const OC891_MAGIC: u32 = 0x4f38_3931;
    /// The object (`self`) the account screens read, its popup and account.
    const OC891_SELF: u32 = DATA + 0x4000;
    const OC891_POPUP: u32 = DATA + 0x5000;
    const OC891_ACCOUNT: u32 = DATA + 0x6000;

    pub(crate) fn oc891_gate(c: &mut Cpu) {
        c.bus.write32(OC891_CTRL, OC891_MAGIC);
    }

    /// A gated fake for a callee: while the gate is on it logs the call and
    /// returns (the forced results of the setup apply); otherwise it runs the
    /// body it replaced.
    macro_rules! oc891_fake {
        ($fake:ident, $target:expr, $gen:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC891_CTRL) != OC891_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc891_fake!(
        oc891_fake_str_hash,
        FE_CS2_STR_HASH,
        ee::s10::f_00317670__generated
    );
    oc891_fake!(
        oc891_fake_sprintf,
        FE_CS2_SPRINTF,
        ee::s15::f_00417828__generated
    );
    oc891_fake!(
        oc891_fake_list_add,
        FE_CS2_LIST_ADD_BY_STRING_ID,
        ee::s13::f_0039a568__generated
    );
    oc891_fake!(
        oc891_fake_list_get,
        FE_CS2_LIST_GET_BY_DATA,
        ee::s13::f_0039a768__generated
    );
    oc891_fake!(
        oc891_fake_list_set,
        FE_CS2_LIST_SET_CURRENT,
        ee::s13::f_0039a7a8__generated
    );
    oc891_fake!(
        oc891_fake_text_id,
        FE_CS2_TEXT_SET_TEXT_ID,
        ee::s13::f_003a1000__generated
    );
    oc891_fake!(
        oc891_fake_text_ascii,
        FE_CS2_TEXT_SET_ASCII,
        ee::s13::f_003a0c50__generated
    );
    oc891_fake!(
        oc891_fake_popup_ctor,
        FE_CS2_POPUP_CFG_CTOR,
        ee::s03::f_001c55f8__generated
    );
    oc891_fake!(
        oc891_fake_popup_dtor,
        FE_CS2_POPUP_CFG_DTOR,
        ee::s03::f_001c5750__generated
    );
    oc891_fake!(
        oc891_fake_popup_text,
        FE_CS2_POPUP_CFG_SET_TEXT_ID,
        ee::s03::f_001c57c0__generated
    );
    oc891_fake!(
        oc891_fake_show_popup,
        FE_CS2_NETFE_SHOW_POPUP,
        ee::s03::f_001a9bc0__generated
    );
    oc891_fake!(
        oc891_fake_netfe_update,
        FE_CS2_NETFE_UPDATE,
        ee::s03::f_001a8770__generated
    );

    /// Hooks the gated fakes into their override slots (once per process).
    pub(crate) fn oc891_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc891_fake_str_hash,
            fakes::prev_cell(oc891_fake_str_hash),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc891_fake_sprintf,
            fakes::prev_cell(oc891_fake_sprintf),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039A568),
            oc891_fake_list_add,
            fakes::prev_cell(oc891_fake_list_add),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039A768),
            oc891_fake_list_get,
            fakes::prev_cell(oc891_fake_list_get),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039A7A8),
            oc891_fake_list_set,
            fakes::prev_cell(oc891_fake_list_set),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc891_fake_text_id,
            fakes::prev_cell(oc891_fake_text_id),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc891_fake_text_ascii,
            fakes::prev_cell(oc891_fake_text_ascii),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C55F8),
            oc891_fake_popup_ctor,
            fakes::prev_cell(oc891_fake_popup_ctor),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C5750),
            oc891_fake_popup_dtor,
            fakes::prev_cell(oc891_fake_popup_dtor),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C57C0),
            oc891_fake_popup_text,
            fakes::prev_cell(oc891_fake_popup_text),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A9BC0),
            oc891_fake_show_popup,
            fakes::prev_cell(oc891_fake_show_popup),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8770),
            oc891_fake_netfe_update,
            fakes::prev_cell(oc891_fake_netfe_update),
        );
    }

    /// The screen object with its popup already built (`GetPopup`'s fast
    /// path returns `self->popup` at +0x6a0) and an account with the legal
    /// flag and parent mail set.
    pub(crate) fn oc891_account(c: &mut Cpu, r: &mut Rng, legal: u32) {
        c.bus.write32(OC891_SELF + 0x6a0, OC891_POPUP);
        c.bus.write32(OC891_SELF + 0x6d0, OC891_ACCOUNT);
        c.bus.write32(OC891_ACCOUNT + 0x40, legal);
        c.bus.write32(OC891_ACCOUNT + 0x10, r.next_u32());
    }

    #[test]
    fn cfe_state_create_step2_fill_month_list_matches() {
        oc891_install();
        check(
            ee::s03::f_001bb5d0__generated,
            cfe_state_create_step2_fill_month_list,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1b_b5d0, 0);
                oc891_gate(c);
                let list = DATA + 0x1000;
                let vt = DATA + 0x2000;
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(list);
                c.bus.write32(list + 0x14, r.next_u32());
                c.bus.write32(list + 0x8, vt);
                c.bus.write16(vt + 0x48, (r.next_u32() & 0xfff8) as u16);
                // The `setVisible` slot calls the fake (outside the table).
                c.bus.write32(vt + 0x4c, fakes::FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_validate_fields_matches() {
        oc891_install();
        check(
            ee::s03::f_001bbaa8__generated,
            cfe_state_create_step2_validate_fields,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1b_baa8, 0);
                oc891_gate(c);
                oc891_account(c, &mut r, seed % 2);
                c.g[A0][0] = sx(OC891_SELF);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_check_age_matches() {
        oc891_install();
        check(
            ee::s03::f_001bbac8__generated,
            cfe_state_create_step2_check_age,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1b_bac8, 0);
                oc891_gate(c);
                oc891_account(c, &mut r, 0);
                c.g[A0][0] = sx(OC891_SELF);
                // Under 13, 13 to 17, 18 and up, and negative ages.
                let ages = [-3i32, 0, 12, 13, 17, 18, 40];
                c.g[A1][0] = sx(ages[(seed % 7) as usize] as u32);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_check_legal_accepted_matches() {
        oc891_install();
        check(
            ee::s03::f_001bbc40__generated,
            cfe_state_create_step2_check_legal_accepted,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1b_bc40, 0);
                oc891_gate(c);
                oc891_account(c, &mut r, seed % 2);
                c.g[A0][0] = sx(OC891_SELF);
            },
        );
    }

    #[test]
    fn cfe_state_create_duplicate_update_matches() {
        oc891_install();
        check(
            ee::s03::f_001bbde0__generated,
            cfe_state_create_duplicate_update,
            50,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x1b_bde0, 0);
                oc891_gate(c);
                c.g[A0][0] = sx(OC891_SELF);
            },
        );
    }

    #[test]
    fn cfe_state_create_duplicate_on_widget_created_matches() {
        oc891_install();
        check(
            ee::s03::f_001bbe00__generated,
            cfe_state_create_duplicate_on_widget_created,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1b_be00, 0);
                oc891_gate(c);
                let text = DATA + 0x7000;
                c.g[A0][0] = sx(OC891_SELF);
                c.g[A1][0] = sx(text);
                // Forced hash results pick the branch: 0 SelectNew, 1 the
                // title, 2 the loop's first name.
                let want = 0x1234_5678u32;
                let mode = seed % 4;
                let account = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    OC891_ACCOUNT
                };
                let (has_account, slot) = match mode {
                    0 => (account, u32::MAX),
                    1 => (account, 0),
                    2 => (0, 1),
                    _ => (OC891_ACCOUNT, 2),
                };
                c.bus.write32(OC891_SELF + 0x6d0, has_account);
                if slot == u32::MAX {
                    c.bus.write32(text + 0x38, r.next_u32() | 1);
                } else {
                    c.bus.write32(text + 0x38, want);
                    c.bus.write32(fakes::LOG_RESULTS + 4 * slot, want);
                }
            },
        );
    }
}
