//! Staging module oc508 (wave wave229): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc508`.

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
    (0x001d_d440, lobby_menu_destroy_challenge_page, "LobbyMenu_DestroyChallengePage"),
    (0x001d_d4c0, lobby_menu_destroy_abort_page, "LobbyMenu_DestroyAbortPage"),
    (0x001d_d598, ui_screen_vt46d1a0_dtor, "UIScreenVt46d1a0_dtor"),
    (0x001d_d5b8, ui_screen_vt46d1a0_ctor, "UIScreenVt46d1a0_ctor"),
    (0x001d_d750, c_fe_state_dtor, "cFEState_dtor"),
    (0x001d_d778, ui_input_mask_rebuild, "UIInputMask_Rebuild"),
    (0x001d_d800, ui_input_mask_set_bit, "UIInputMask_SetBit"),
    (0x001d_d868, c_fe_state_cheat_char_select_dtor, "cFEStateCheatCharSelect_dtor"),
    (0x001d_d8c8, c_fe_state_rider_detail_dtor, "cFEStateRiderDetail_dtor"),
    (0x001d_d8f0, c_fe_state_uber_trick_dtor, "cFEStateUberTrick_dtor"),
    (0x001d_d950, ui_object_set_vec3_at_9c, "UIObject_SetVec3At9c"),
    (0x001d_dbd0, c_fe_popup_confirm_base_ctor_2, "cFEPopupConfirm_BaseCtor_2"),
    (0x001d_dc48, c_ui_state_buy_popup_ctor_2, "cUIStateBuyPopup_ctor_2"),
    (0x001d_dc90, c_ui_state_buy_popup_dtor, "cUIStateBuyPopup_dtor"),
    (
        0x001d_dcc8,
        c_ui_state_buy_popup_set_song_credit_params_thunk,
        "cUIStateBuyPopup_SetSongCreditParams_Thunk",
    ),
    (
        0x001d_dce8,
        c_ui_state_buy_popup_set_music_track_params_thunk,
        "cUIStateBuyPopup_SetMusicTrackParams_Thunk",
    ),
    (
        0x001d_dd08,
        c_ui_state_buy_popup_set_reward_item_params_thunk,
        "cUIStateBuyPopup_SetRewardItemParams_Thunk",
    ),
    (0x001d_de18, mem_card_state_base_ctor_3, "MemCardStateBase_ctor_3"),
    (0x001d_de78, c_fe_popup_confirm_ctor_2, "cFEPopupConfirm_ctor_2"),
    (0x001d_deb8, c_fe_popup_confirm_dtor, "cFEPopupConfirm_dtor"),
];

/// The guest address `c.g[reg] + off` (the generated
/// `(c.g[reg][0] as u32).wrapping_add(off)`).
pub(crate) fn oc508_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `void LobbyMenu_DestroyChallengePage(LobbyMenu* page, int flags)`
/// (0x1dd440): destroy the six menu items of the challenge-response page
/// (`cMenuItem` at `page + 0x138 .. 0x1c8`, flags 2), then the page menu
/// itself at `page` with the caller's flags.
pub fn lobby_menu_destroy_challenge_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        const ITEMS: [(u32, u32); 6] = [
            (0x1c8, 0x001d_d464),
            (0x1a4, 0x001d_d470),
            (0x18c, 0x001d_d47c),
            (0x170, 0x001d_d488),
            (0x154, 0x001d_d494),
            (0x138, 0x001d_d4a0),
        ];
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for &(off, back) in ITEMS.iter() {
            c.g[A0][0] = sx(oc508_at(c, S0, off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, back)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x001d_d4ac)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_DestroyAbortPage(LobbyMenu* page, int flags)` (0x1dd4c0):
/// destroy the abort page's menu item (`page + 0x134`, flags 2), then the
/// page menu at `page` with the caller's flags.
pub fn lobby_menu_destroy_abort_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(oc508_at(c, S0, 0x134));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002c_a280, 0x001d_d4e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x001d_d4f0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIScreenVt46d1a0::~UIScreenVt46d1a0(this)` (0x1dd598): the base
/// screen's destructor (0x39c558), with `a0` passed through.
pub fn ui_screen_vt46d1a0_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0039_c558, 0x001d_d5a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIScreenVt46d1a0::UIScreenVt46d1a0(this)` (0x1dd5b8): the keyboard
/// screen's constructor. Builds the base screen (flags at `this + 0x14`,
/// then the list at `+0x18`, the UI thing at `+0x40` and a second list at
/// `+0xb4`), and sets the vtables: `0x494670` while the base runs, the
/// screen's own `0x46d1a0` at the end (`+0x8`), and `0x4949e8` at `+0x48`.
pub fn ui_screen_vt46d1a0_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(0x0049_4670);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(oc508_at(c, S0, 0x4), c.g[S0][0]);
        c.g[A0][0] = sx(oc508_at(c, S0, 0x14));
        c.sw(oc508_at(c, S0, 0x0), c.g[S0][0]);
        c.sw(oc508_at(c, S0, 0xc), 0);
        c.sw(oc508_at(c, S0, 0x10), 0);
        c.sw(oc508_at(c, S0, 0x8), c.g[V0][0]);
        call_ptr(c, 0x001d_d580, 0x001d_d5ec)?;
        c.g[A0][0] = sx(oc508_at(c, S0, 0x18));
        call_ptr(c, 0x0039_77e8, 0x001d_d5f4)?;
        c.sw(oc508_at(c, S0, 0x34), 0);
        c.g[A1][0] = 0;
        c.sw(oc508_at(c, S0, 0x38), 0);
        c.g[A2][0] = 0;
        c.sw(oc508_at(c, S0, 0x3c), 0);
        c.g[A0][0] = sx(oc508_at(c, S0, 0x40));
        call_ptr(c, 0x0039_fb30, 0x001d_d610)?;
        c.g[V1][0] = sx(0x0049_0000 + 0x49e8);
        c.g[A0][0] = sx(oc508_at(c, S0, 0xb4));
        c.sw(oc508_at(c, S0, 0x48), c.g[V1][0]);
        call_ptr(c, 0x0039_77e8, 0x001d_d624)?;
        c.g[V1][0] = sx(0x0047_0000 - 0x2e60);
        c.g[V0][0] = c.g[S0][0];
        c.sw(oc508_at(c, S0, 0xd0), 0);
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The destructor shape of the FE states and popups: `this->vtable` (the
/// word at +8) is set to `vt`, then the base destructor `base(this)` is
/// called, returning to `ret`.
pub(crate) fn oc508_vtable_dtor(c: &mut Cpu, vt: u64, base: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = vt;
    c.sw(oc508_at(c, V1, 0x8), c.g[V0][0]);
    call_ptr(c, base, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEState::~cFEState(this)` (0x1dd750): vtable 0x46d0d0, then the base
/// state destructor (0x39e390).
pub fn c_fe_state_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_d76c)
    })
}

/// `cFEStateCheatCharSelect::~cFEStateCheatCharSelect(this)` (0x1dd868).
pub fn c_fe_state_cheat_char_select_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_d884)
    })
}

/// `cFEStateRiderDetail::~cFEStateRiderDetail(this)` (0x1dd8c8).
pub fn c_fe_state_rider_detail_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_d8e4)
    })
}

/// `cFEStateUberTrick::~cFEStateUberTrick(this)` (0x1dd8f0).
pub fn c_fe_state_uber_trick_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_d90c)
    })
}

/// `void UIInputMask_Rebuild(UIInputMask* mask, int count)` (0x1dd778): the
/// byte at `mask + 0x15` becomes the AND, over the `count` players (the
/// signed byte of `a1`, at most 0 for none), of the complements of
/// `FEModule_GetPlayerPadMask(engine, i)`, starting from 0xff. The engine is
/// `**(mask + 0x10)`. A VBlank event at the loop's back edge is delivered
/// here and the loop goes on (the result is not checked).
pub fn ui_input_mask_rebuild(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = 0xff;
        frame.save(c, S3, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc508_at(c, S4, 0x10));
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[S3][0] = c.lw(c.g[V0][0] as u32);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 9;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 24);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x001a_1cd0, 0x001d_d7cc)?;
                c.g[V0][0] = !c.g[V0][0];
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S2][0] &= c.g[V0][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_d7b8);
                }
            }
        }
        c.cycles += 9;
        c.sb(oc508_at(c, S4, 0x15), c.g[S2][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIInputMask_SetBit(UIInputMask* mask, int player, int controller)`
/// (0x1dd800): `FEModule_SetPlayerPadMask(engine, player, (1 << controller)
/// & 0xff)`, with the engine at `**(mask + 0x10)`.
pub fn ui_input_mask_set_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
        c.g[V0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        c.g[V1][0] = c.lw(oc508_at(c, A0, 0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A2][0] as u32) & 31));
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.g[A2][0] = c.g[V0][0] & 0xff;
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        call_ptr(c, 0x001a_1cb8, 0x001d_d830)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIObject_SetVec3At9c(UIObject* self, const Vec3* v)` (0x1dd950):
/// copy the 12-byte vector at `v` to `self + 0x9c`, as an unaligned 8-byte
/// move plus one word, through a stack temporary.
pub fn ui_object_set_vec3_at_9c(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x10);
    c.ldl(2, oc508_at(c, A1, 0x7));
    c.ldr(2, oc508_at(c, A1, 0x0));
    c.g[V1][0] = c.lw(oc508_at(c, A1, 0x8));
    c.sdl(2, oc508_at(c, SP, 0x7));
    c.sdr(2, oc508_at(c, SP, 0x0));
    c.sw(oc508_at(c, SP, 0x8), c.g[V1][0]);
    c.ldl(2, oc508_at(c, SP, 0x7));
    c.ldr(2, oc508_at(c, SP, 0x0));
    c.g[V1][0] = c.lw(oc508_at(c, SP, 0x8));
    c.sdl(2, oc508_at(c, A0, 0xa3));
    c.sdr(2, oc508_at(c, A0, 0x9c));
    c.sw(oc508_at(c, A0, 0xa4), c.g[V1][0]);
    frame.close(c);
}

/// `cFEPopupConfirm` base-constructor variant (0x1ddbd0): the base UI state
/// (0x39e318) with no arguments, the vtable 0x46cbd8 at +8, the member at
/// +0xbc set up (0x1c55f8), then the words at +0x2b0 and +0x2b4 take the
/// game word at gp+0xda0. Returns `this`.
pub fn c_fe_popup_confirm_base_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x001d_dbec)?;
        c.g[V1][0] = sx(0x0047_0000 - 0x3428);
        c.g[A0][0] = sx(oc508_at(c, S0, 0xbc));
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        call_ptr(c, 0x001c_55f8, 0x001d_dc00)?;
        c.g[V1][0] = c.lw(oc508_at(c, GP, 0xda0));
        c.g[V0][0] = c.g[S0][0];
        c.sw(oc508_at(c, S0, 0x2b4), c.g[V1][0]);
        c.sw(oc508_at(c, S0, 0x2b0), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup::cUIStateBuyPopup(this)` variant (0x1ddc48): the base
/// UI state (0x39e318), the vtable 0x46cb08 at +8, then the parameter block
/// at +0x48 is reset (0x1ca8a0). Returns `this`.
pub fn c_ui_state_buy_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x001d_dc64)?;
        c.g[V1][0] = sx(0x0047_0000 - 0x34f8);
        c.g[A0][0] = sx(oc508_at(c, S0, 0x48));
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        call_ptr(c, 0x001c_a8a0, 0x001d_dc78)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIStateBuyPopup::~cUIStateBuyPopup(this)` (0x1ddc90).
pub fn c_ui_state_buy_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_dcac)
    })
}

/// The parameter thunks of the buy popup (+0x48 is the block the
/// `cBuyPopupInfo` setter fills): `target(this + 0x48)`, returning to `ret`.
pub(crate) fn oc508_params_thunk(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(oc508_at(c, A0, 0x48));
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cUIStateBuyPopup::SetSongCreditParams` thunk (0x1ddcc8).
pub fn c_ui_state_buy_popup_set_song_credit_params_thunk(c: &mut Cpu) {
    guest(c, |c| oc508_params_thunk(c, 0x001c_a900, 0x001d_dcd8))
}

/// `cUIStateBuyPopup::SetMusicTrackParams` thunk (0x1ddce8).
pub fn c_ui_state_buy_popup_set_music_track_params_thunk(c: &mut Cpu) {
    guest(c, |c| oc508_params_thunk(c, 0x001c_a980, 0x001d_dcf8))
}

/// `cUIStateBuyPopup::SetRewardItemParams` thunk (0x1ddd08).
pub fn c_ui_state_buy_popup_set_reward_item_params_thunk(c: &mut Cpu) {
    guest(c, |c| oc508_params_thunk(c, 0x001c_aa18, 0x001d_dd18))
}

/// `MemCardStateBase` constructor variant (0x1dde18): the base UI state
/// (0x39e2a0), then the fields at +0x48, +0x4c, +0x54 and +0x98 are cleared,
/// the vtable is written at +8 (0x474e08, then 0x46c980 at the end), and the
/// halfword at 0x4a1390 is stored at +0x58. Returns `this`.
pub fn mem_card_state_base_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x001d_de30)?;
        c.sw(oc508_at(c, S0, 0x98), 0);
        c.g[V1][0] = sx(0x0047_0000 + 0x4e08);
        c.sw(oc508_at(c, S0, 0x48), 0);
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(oc508_at(c, S0, 0x4c), 0);
        c.sw(oc508_at(c, S0, 0x54), 0);
        c.g[V1][0] = sx(0x0047_0000 - 0x3680);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu(oc508_at(c, A1, 0x1390));
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        c.sh(oc508_at(c, S0, 0x58), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm::cFEPopupConfirm(this)` variant (0x1dde78): the base
/// popup (0x1c58e8) with no arguments, the vtable 0x46c890 at +8. Returns
/// `this`.
pub fn c_fe_popup_confirm_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001c_58e8, 0x001d_de94)?;
        c.g[V1][0] = sx(0x0047_0000 - 0x3770);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.sw(oc508_at(c, S0, 0x8), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm::~cFEPopupConfirm(this)` (0x1ddeb8): vtable 0x46c890,
/// then the base popup destructor (0x1c5a90).
pub fn c_fe_popup_confirm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x3770), 0x001c_5a90, 0x001d_ded4)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// Random registers, the game's gp, and the object (`this`) at
    /// `DATA + 0x1000` in a0. The object's memory stays zero.
    pub(crate) fn oc508_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = sx(DATA + 0x1000);
        r
    }

    /// The engine chain `**(object + 0x10)`: the object's word at +0x10
    /// points at `DATA + 0x2000`, whose word is the engine at `DATA + 0x3000`.
    pub(crate) fn oc508_engine_chain(c: &mut Cpu) {
        c.bus.write32(DATA + 0x1010, DATA + 0x2000);
        c.bus.write32(DATA + 0x2000, DATA + 0x3000);
    }

    #[test]
    fn lobby_menu_destroy_challenge_page_matches() {
        check(
            ee::s04::f_001dd440__generated,
            lobby_menu_destroy_challenge_page,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd440);
                c.g[A1][0] = if seed % 2 == 0 { 2 } else { 0 };
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_abort_page_matches() {
        check(
            ee::s04::f_001dd4c0__generated,
            lobby_menu_destroy_abort_page,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd4c0);
                c.g[A1][0] = if seed % 2 == 0 { 2 } else { 0 };
            },
        );
    }

    #[test]
    fn ui_screen_vt46d1a0_dtor_matches() {
        check(
            ee::s04::f_001dd598__generated,
            ui_screen_vt46d1a0_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd598);
            },
        );
    }

    #[test]
    fn ui_screen_vt46d1a0_ctor_matches() {
        check(
            ee::s04::f_001dd5b8__generated,
            ui_screen_vt46d1a0_ctor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd5b8);
            },
        );
    }

    #[test]
    fn c_fe_state_dtor_matches() {
        check(ee::s04::f_001dd750__generated, c_fe_state_dtor, 200, &|c, seed| {
            oc508_setup(c, seed, 0x1dd750);
        });
    }

    #[test]
    fn ui_input_mask_rebuild_events_match() {
        check_events(
            ee::s04::f_001dd778__generated,
            ui_input_mask_rebuild,
            120,
            &|c, seed| {
                let mut r = oc508_setup(c, seed, 0x1dd778);
                let mode = schedule(c, &mut r, seed, 0x001d_d778);
                c.g[A0][0] = sx(DATA + 0x1000);
                oc508_engine_chain(c);
                // The count is the signed byte of a1: 0 and -1 skip the loop.
                c.g[A1][0] = [0u64, 1, 3, 0xff, 2][(seed % 5) as usize];
                mode
            },
        );
    }

    #[test]
    fn ui_input_mask_set_bit_matches() {
        check(ee::s04::f_001dd800__generated, ui_input_mask_set_bit, 200, &|c, seed| {
            let mut r = oc508_setup(c, seed, 0x1dd800);
            oc508_engine_chain(c);
            c.g[A1][0] = sx((seed % 4) as u32);
            c.g[A2][0] = sx(r.next_u32() % 8);
        });
    }

    #[test]
    fn c_fe_state_cheat_char_select_dtor_matches() {
        check(
            ee::s04::f_001dd868__generated,
            c_fe_state_cheat_char_select_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd868);
            },
        );
    }

    #[test]
    fn c_fe_state_rider_detail_dtor_matches() {
        check(
            ee::s04::f_001dd8c8__generated,
            c_fe_state_rider_detail_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd8c8);
            },
        );
    }

    #[test]
    fn c_fe_state_uber_trick_dtor_matches() {
        check(
            ee::s04::f_001dd8f0__generated,
            c_fe_state_uber_trick_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd8f0);
            },
        );
    }

    #[test]
    fn ui_object_set_vec3_at_9c_matches() {
        check(
            ee::s04::f_001dd950__generated,
            ui_object_set_vec3_at_9c,
            200,
            &|c, seed| {
                let mut r = oc508_setup(c, seed, 0x1dd950);
                fill(c, &mut r, DATA + 0x2000, 4);
                c.g[A1][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_base_ctor_2_matches() {
        check(
            ee::s04::f_001ddbd0__generated,
            c_fe_popup_confirm_base_ctor_2,
            200,
            &|c, seed| {
                let mut r = oc508_setup(c, seed, 0x1ddbd0);
                c.bus.write32(GAME_GP + 0xda0, r.next_u32());
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_ctor_2_matches() {
        check(
            ee::s04::f_001ddc48__generated,
            c_ui_state_buy_popup_ctor_2,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddc48);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_dtor_matches() {
        check(
            ee::s04::f_001ddc90__generated,
            c_ui_state_buy_popup_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddc90);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_song_credit_params_thunk_matches() {
        check(
            ee::s04::f_001ddcc8__generated,
            c_ui_state_buy_popup_set_song_credit_params_thunk,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddcc8);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_music_track_params_thunk_matches() {
        check(
            ee::s04::f_001ddce8__generated,
            c_ui_state_buy_popup_set_music_track_params_thunk,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddce8);
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_reward_item_params_thunk_matches() {
        check(
            ee::s04::f_001ddd08__generated,
            c_ui_state_buy_popup_set_reward_item_params_thunk,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddd08);
            },
        );
    }

    #[test]
    fn mem_card_state_base_ctor_3_matches() {
        check(
            ee::s04::f_001dde18__generated,
            mem_card_state_base_ctor_3,
            200,
            &|c, seed| {
                let mut r = oc508_setup(c, seed, 0x1dde18);
                c.bus.write16(0x004a_1390, r.next_u32() as u16);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_ctor_2_matches() {
        check(
            ee::s04::f_001dde78__generated,
            c_fe_popup_confirm_ctor_2,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dde78);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_dtor_matches() {
        check(
            ee::s04::f_001ddeb8__generated,
            c_fe_popup_confirm_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1ddeb8);
            },
        );
    }
}
