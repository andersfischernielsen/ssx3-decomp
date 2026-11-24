//! `src/main/all_in_one_main.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::debugfe::menu_item_dtor;
pub(crate) use crate::game::fe::debugfe::menu_page_dtor;
pub(crate) use crate::game::fe::debugfe::menu_sx_add;
pub(crate) use crate::game::fe::feasyncfile::fe_dtor_with_vtable;
pub(crate) use crate::game::fe::fepopup::oc508_at;
pub(crate) use crate::game::fe::festatebonusmaterial::FE_UI_STATE_DTOR;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_d580, ui_screen_flags_ctor, "UIScreenFlags_ctor"),
    (0x001d_c900, module_base_dtor, "ModuleBase_dtor"),
    (0x001d_c940, lobby_menu_base_dtor, "LobbyMenuBase_dtor"),
    (0x001d_cac0, fe_module_camera_dtor, "FEModuleCamera_dtor"),
    (
        0x001d_caf0,
        c_private_message_dtor_2,
        "cPrivateMessage_dtor_2",
    ),
    (
        0x001d_cd58,
        lobby_menu_message_page_dtor,
        "LobbyMenuMessagePage_dtor",
    ),
    (
        0x001d_d598,
        ui_screen_vt46d1a0_dtor,
        "UIScreenVt46d1a0_dtor",
    ),
    (
        0x001d_d5b8,
        ui_screen_vt46d1a0_ctor,
        "UIScreenVt46d1a0_ctor",
    ),
    (0x001d_d778, ui_input_mask_rebuild, "UIInputMask_Rebuild"),
    (0x001d_d800, ui_input_mask_set_bit, "UIInputMask_SetBit"),
    (
        0x001d_e9f8,
        c_fe_popup_option_reset_dtor,
        "cFEPopupOptionReset_dtor",
    ),
    (
        0x001d_ea20,
        cfe_popup_option_reset_init,
        "cFEPopupOptionReset_Init",
    ),
    (
        0x001d_eaa8,
        cfe_popup_option_reset_on_widget_action,
        "cFEPopupOptionReset_OnWidgetAction",
    ),
    (
        0x001d_eb58,
        cfe_popup_video_calibration_dtor,
        "cFEPopupVideoCalibration_dtor",
    ),
];

/// `UIScreenFlags* UIScreenFlags_ctor(UIScreenFlags* self)` (0x001dd580):
/// sets bit 0 and clears bit 1 of the flag halfword at +0 and returns `self`.
pub fn ui_screen_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] |= 0x1;
    c.g[V1][0] &= 0xfffd;
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = ra;
}

/// The `delete` of a gcc dtor (0x317e50).
pub(crate) const DELETE_THIS: u32 = 0x0031_7e50;

/// The shape of a gcc `dtor(this, flags)` that stores its vtable at
/// `this + field` and deletes `this` (0x317e50) when `flags & 1`.
pub(crate) fn vtable_dtor(c: &mut Cpu, vtable: u32, field: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    c.g[A1][0] &= 1;
    let delete = c.g[A1][0] != 0;
    let this = c.g[A0][0] as u32;
    c.sw(this.wrapping_add(field), c.g[V0][0]);
    if delete {
        c.cycles += 2;
        call_ptr(c, DELETE_THIS, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `ModuleBase_dtor` (0x1dc900).
pub fn module_base_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0046_d970, 0, 0x001d_c924));
}

/// `LobbyMenuBase_dtor` (0x1dc940).
pub fn lobby_menu_base_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0046_d948, 0, 0x001d_c964));
}

/// `FEModuleCamera_dtor` (0x1dcac0): the vtable is stored at +0x90.
pub fn fe_module_camera_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0045_b8b0, 0x90, 0x001d_cae4));
}

/// `cPrivateMessage_dtor_2` (0x1dcaf0): stores the vtable at +0xc, the
/// three members at +8, +4 and +0 are destroyed (0x318260 with 2), and the
/// object is deleted (0x317e50) when `flags & 1`.
pub fn c_private_message_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x2740));
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        c.sw(this.wrapping_add(0xc), c.g[V0][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb20)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb2c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb38)?;
        c.g[S0][0] &= 1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, 16, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, DELETE_THIS, 0x001d_cb4c)?;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenuMessagePage::~LobbyMenuMessagePage(this, flags)` (0x1dcd58): destroys
/// the message page's item at `+0x130`, then the base page.
pub fn lobby_menu_message_page_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        menu_item_dtor(c, menu_sx_add(s0, 0x130), 0x001d_cd7c)?;
        menu_page_dtor(c, s0, s1, 0x001d_cd88)?;
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

/// `cFEPopupOptionReset` destructor (0x1de9f8): vtable at `+8`, then
/// `UIState_dtor(this)`.
pub fn c_fe_popup_option_reset_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(c, 10, Target::Table(FE_UI_STATE_DTOR), 0x4c30, 0x001d_ea14)
    });
}

/// `void cFEPopupVideoCalibration::dtor(void* self)` (0x1deb58): stores the
/// class vtable word 0x46b300 at `self+8`, then calls the base destructor
/// (0x39e390) on `self`.
pub fn cfe_popup_video_calibration_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = 0x0046_b300;
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x0039_e390, 0x001d_eb74)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupOptionReset::onCreateScreen(void* self)` (0x1dea20): looks up
/// the hash of the option-reset string (0x45dd20) and stores it at `self+0xc`,
/// then fills the widget at `self+0x40` with the result of 0x397cc0 and, when
/// that is non-zero, calls 0x39c870 on it.
pub fn cfe_popup_option_reset_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0045_dd20;
        call_ptr(c, 0x0031_7670, 0x001d_ea3c)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A2][0] = c.g[V0][0];
        let hash = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, hash);
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001d_ea54)?;
        let widget = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, widget);
        if widget != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001d_ea6c)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `cFEPopupOptionReset::onWidgetEvent` (0x1deaa8) between its
/// prologue and epilogue. Returns `Ok` on every normal exit (the caller runs
/// the epilogue); `S0` is restored on each path, as the generated code does.
pub(crate) fn cfe_popup_option_reset_on_widget_body(c: &mut Cpu, frame: Frame) -> Flow {
    if c.g[S1][0] == 0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = 5;
    let other_event = c.g[A2][0] != c.g[V0][0];
    frame.restore(c, S0, 0x30);
    if other_event {
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = 0x004a_0000;
    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14e8));
    call_ptr(c, 0x0031_7670, 0x001d_eae4)?;
    if c.g[S0][0] == c.g[V0][0] {
        // The stored id matches the first string: pop the UI state stack.
        c.cycles += 8;
        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = 1;
        let flag = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(0x48)).sb(c, flag);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f190, 0x001d_eb04)?;
        frame.restore(c, S0, 0x30);
        return Ok(());
    }
    c.g[A0][0] = 0x004a_0000;
    c.cycles += 5;
    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f0));
    call_ptr(c, 0x0031_7670, 0x001d_eb18)?;
    let mismatch = c.g[S0][0] != c.g[V0][0];
    frame.restore(c, S0, 0x30);
    if mismatch {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lw(c);
    c.g[A1][0] = 1;
    Addr((c.g[S2][0] as u32).wrapping_add(0x48)).sb(c, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f190, 0x001d_eb34)?;
    c.cycles += 1;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// `void cFEPopupOptionReset::onWidgetEvent(void* self, int event, int id)`
/// (0x1deaa8). Compares the word at `id+0x38` with the hashes of two option
/// strings (0x4a14e8, 0x4a14f0); a match pops the UI state stack at
/// `self+0x10` (0x39f190) and sets the byte at `self+0x48`.
pub fn cfe_popup_option_reset_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        cfe_popup_option_reset_on_widget_body(c, frame)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use ssx3_ee as ee;

    use crate::difftest::fakes::machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    /// Flag halfword with every combination of bits 0 and 1 and random
    /// upper bits.
    #[test]
    fn ui_screen_flags_ctor_matches() {
        check(
            ee::s04::f_001dd580__generated,
            ui_screen_flags_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dd580, 0);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                c.bus.write32(OC431_BASE + 0x100, r.next_u32());
                c.pc = 0x001d_d580;
            },
        );
    }

    use crate::difftest::fakes::covered;

    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::debugfe::tests::oc505_install_fakes;
    pub(crate) use crate::game::fe::debugfe::tests::oc505_machine;

    pub(crate) use crate::game::fe::debugfe::tests::OC506_THIS;

    pub(crate) use crate::game::fe::debugfe::tests::oc506_arm;
    pub(crate) use crate::game::fe::debugfe::tests::oc506_hook_all;

    #[test]
    fn module_base_dtor_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc900__generated,
            module_base_dtor,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dc9_00, 1);
                c.pc = 0x001d_c900;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_base_dtor_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc940__generated,
            lobby_menu_base_dtor,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dc9_40, 1);
                c.pc = 0x001d_c940;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn fe_module_camera_dtor_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dcac0__generated,
            fe_module_camera_dtor,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_c0, 1);
                c.pc = 0x001d_cac0;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_private_message_dtor_2_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dcaf0__generated,
            c_private_message_dtor_2,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_f0, 4);
                c.pc = 0x001d_caf0;
                c.g[A0][0] = (DATA + 0x500) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_message_page_dtor_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcd58__generated,
            lobby_menu_message_page_dtor,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcd58, 3);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;

    use crate::difftest::Rng;

    use crate::difftest::fresh;
    pub(crate) use crate::game::fe::fepopup::tests::oc508_setup;
    pub(crate) use crate::game::fe::festatebonusmaterial::tests::ui_state_case;

    /// The engine chain `**(object + 0x10)`: the object's word at +0x10
    /// points at `DATA + 0x2000`, whose word is the engine at `DATA + 0x3000`.
    pub(crate) fn oc508_engine_chain(c: &mut Cpu) {
        c.bus.write32(DATA + 0x1010, DATA + 0x2000);
        c.bus.write32(DATA + 0x2000, DATA + 0x3000);
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
        check(
            ee::s04::f_001dd800__generated,
            ui_input_mask_set_bit,
            200,
            &|c, seed| {
                let mut r = oc508_setup(c, seed, 0x1dd800);
                oc508_engine_chain(c);
                c.g[A1][0] = sx((seed % 4) as u32);
                c.g[A2][0] = sx(r.next_u32() % 8);
            },
        );
    }

    #[test]
    fn c_fe_popup_option_reset_dtor_matches() {
        ui_state_case(
            ee::s04::f_001de9f8__generated,
            c_fe_popup_option_reset_dtor,
            0x001d_e9f8,
        );
    }

    /// Scrambled machine with the test object at `DATA` in `A0`.
    pub(crate) fn object_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA);
        r
    }

    /// The string hash the generated code computes for `addr` on empty RAM.
    pub(crate) fn string_hash(addr: u32) -> u64 {
        let mut m = fresh();
        m.g[A0][0] = addr as u64;
        m.g[RA][0] = 0x0012_3450;
        m.pc = 0x0031_7670;
        ee::s10::f_00317670(&mut m);
        m.g[V0][0]
    }

    #[test]
    fn cfe_popup_video_calibration_dtor_matches() {
        check(
            ee::s04::f_001deb58__generated,
            cfe_popup_video_calibration_dtor,
            40,
            &|c, seed| {
                let _ = object_machine(c, seed, 0xeb58);
            },
        );
    }

    #[test]
    fn cfe_popup_option_reset_init_matches() {
        check(
            ee::s04::f_001dea20__generated,
            cfe_popup_option_reset_init,
            60,
            &|c, seed| {
                let mut r = object_machine(c, seed, 0xea20);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x2000 + 0x40, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn cfe_popup_option_reset_on_widget_action_matches() {
        let h1 = string_hash(0x004a_14e8) as u32;
        let h2 = string_hash(0x004a_14f0) as u32;
        check(
            ee::s04::f_001deaa8__generated,
            cfe_popup_option_reset_on_widget_action,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0xeaa8, 0);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(DATA) };
                c.g[A2][0] = if seed % 3 == 0 {
                    5
                } else {
                    r.next_u32() as u64
                };
                c.g[A0][0] = sx(DATA + 0x100);
                let stored = match seed % 4 {
                    0 => h1,
                    1 => h2,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x38, stored);
                c.bus.write32(DATA + 0x110, DATA + 0x2000);
            },
        );
    }
}
