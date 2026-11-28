//! `src/main/all_in_one_main.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
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
    (0x001d_b718, std_list_clear, "StdList_Clear"),
    (0x001d_b778, std_vector_insert_aux, "StdVector_InsertAux"),
    (
        0x001d_b918,
        std_vector_insert_aux_2,
        "StdVector_InsertAux_2",
    ),
    (
        0x001d_ea80,
        cfe_popup_option_reset_on_widget_menu,
        "cFEPopupOptionReset_onWidgetMenu",
    ),
    (
        0x001d_eb50,
        c_fe_popup_option_reset_v23_001deb50,
        "cFEPopupOptionReset_v23",
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

/// `cFEPopupOptionReset_v23` (0x001d_eb50): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_fe_popup_option_reset_v23_001deb50(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// The object's `this` (`s1`) as an address.
#[inline(always)]
pub(crate) fn this_of(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// `StdList_Clear(list)` (0x1db718): deletes every node after the sentinel
/// head (`list + 4`), each through the node deleter (0x317e50) with its
/// `next` read first, then points the sentinel at itself (`next` and
/// `prev`). A back-edge event check at the loop head 0x1db740.
pub fn std_list_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[S0][0] != c.g[V0][0] {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
                call_ptr(c, 0x0031_7e50, 0x001d_b748)?;
                c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_b740);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
                frame.restore(c, S0, 0x20);
                break;
            }
        } else {
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 7;
        let head = c.g[V0][0];
        Addr(head as u32).sw(c, head);
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
        frame.restore(c, S1, 0x10);
        let v1 = c.g[V1][0];
        Addr((v1 as u32).wrapping_add(4)).sw(c, v1);
        frame.close(c);
        Ok(())
    });
}

/// Return addresses of the calls in `StdVector_InsertAux` (the generated
/// body's `c.g[RA][0] = sx(0x...)` lines), per function copy.
#[derive(Clone, Copy)]
pub(crate) struct InsertRets {
    /// `memmove` of the tail, fast path (0x41610c).
    fast_move: u32,
    /// `EA_MemAllocNamed` for a one-element block (0x317d70).
    alloc_one: u32,
    /// `operator_new_named` for a grown block (0x317e30).
    alloc_many: u32,
    /// `memmove` of the head into the new block (0x41610c).
    copy_head: u32,
    /// `memmove` of the tail behind the new element (0x41610c).
    copy_tail: u32,
    /// Free of the old one-element block (0x317e50).
    free_one: u32,
    /// `EA_MemFree` of the old block (0x317e98).
    free_many: u32,
}

/// `StdVector_InsertAux(vec, pos, x)` (0x1db778): `_M_insert_aux` for a
/// vector of pointers (`mpStart`, `mpFinish`, `mpEndOfStorage` at +4, +8,
/// +0xc). With room behind the finish it shifts the tail up by one; a full
/// vector grows to twice its size (one element when empty) and moves both
/// halves into the new block.
pub fn std_vector_insert_aux(c: &mut Cpu) {
    guest(c, |c| {
        let rets = InsertRets {
            fast_move: 0x001d_b7ec,
            alloc_one: 0x001d_b834,
            alloc_many: 0x001d_b854,
            copy_head: 0x001d_b878,
            copy_tail: 0x001d_b8ac,
            free_one: 0x001d_b8d4,
            free_many: 0x001d_b8e4,
        };
        ptr_vector_insert_aux(c, &rets)
    });
}

/// `StdVector_InsertAux` for the second vector type (0x1db918): the same
/// body at its own addresses.
pub fn std_vector_insert_aux_2(c: &mut Cpu) {
    guest(c, |c| {
        let rets = InsertRets {
            fast_move: 0x001d_b98c,
            alloc_one: 0x001d_b9d4,
            alloc_many: 0x001d_b9f4,
            copy_head: 0x001d_ba18,
            copy_tail: 0x001d_ba4c,
            free_one: 0x001d_ba74,
            free_many: 0x001d_ba84,
        };
        ptr_vector_insert_aux(c, &rets)
    });
}

/// The frame and the full/room split of `_M_insert_aux`.
pub(crate) fn ptr_vector_insert_aux(c: &mut Cpu, rets: &InsertRets) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S1, 0x50);
    frame.save(c, S3, 0x30);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S4, 0x20);
    c.g[S3][0] = c.g[A1][0];
    frame.save(c, S0, 0x60);
    frame.save(c, S2, 0x40);
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
    c.g[V0][0] = Addr(this_of(c).wrapping_add(0xc)).lw(c);
    let full = c.g[V1][0] == c.g[V0][0];
    c.g[S4][0] = c.g[A2][0];
    if full {
        ptr_vector_grow(c, rets)?;
    } else {
        ptr_vector_room(c, rets)?;
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The room path: store the last pointer one slot up, then move the
/// `[pos, finish - 1)` run up by one and write `x` at `pos`.
pub(crate) fn ptr_vector_room(c: &mut Cpu, rets: &InsertRets) -> Flow {
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        c.g[A0][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(4)).lw(c);
        let last = c.g[V0][0];
        Addr(c.g[V1][0] as u32).sw(c, last);
        c.g[A0][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
    }
    c.cycles += 12;
    c.g[A1][0] = c.g[S3][0];
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
    let new_finish = c.g[V0][0];
    Addr(this_of(c).wrapping_add(8)).sw(c, new_finish);
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[S0][0] = Addr(c.g[S4][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
    call_ptr(c, 0x0041_610c, rets.fast_move)?;
    let x = c.g[S0][0];
    Addr(c.g[S3][0] as u32).sw(c, x);
    Ok(())
}

/// The full path: allocate the new block (twice the element count, or one
/// element when empty), copy the head, the new element and the tail into
/// it, free the old block and install the three new pointers.
pub(crate) fn ptr_vector_grow(c: &mut Cpu, rets: &InsertRets) -> Flow {
    'grow: {
        'single: {
            c.cycles += 9;
            c.g[V0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
            c.g[S0][0] = 1;
            c.g[A0][0] = 1;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V1][0];
            }
            let not_one = c.g[S0][0] != c.g[A0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            if not_one {
                break 'single;
            }
            c.cycles += 9;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3218));
            c.g[A0][0] = 4;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            c.g[S5][0] = 4;
            call_ptr(c, 0x0031_7d70, rets.alloc_one)?;
            c.g[S2][0] = c.g[V0][0];
            break 'grow;
        }
        c.cycles += 8;
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3218));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, rets.alloc_many)?;
        c.g[S5][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
    }

    // The head [start, pos) into the new block.
    c.cycles += 9;
    c.g[A1][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    call_ptr(c, 0x0041_610c, rets.copy_head)?;
    c.g[V1][0] = c.g[S0][0];
    if c.g[S0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(c.g[S4][0] as u32).lw(c);
        let x = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, x);
    }

    // The tail [pos, finish) behind the new element, then the old block.
    'tail: {
        {
            c.cycles += 14;
            c.g[A2][0] = Addr(this_of(c).wrapping_add(8)).lw(c);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, 0x0041_610c, rets.copy_tail)?;
            c.g[A0][0] = Addr(this_of(c).wrapping_add(4)).lw(c);
            c.g[V0][0] = Addr(this_of(c).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            let no_old_block = c.g[A0][0] == 0;
            c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            if !no_old_block {
                'release: {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    if c.g[V1][0] != c.g[V0][0] {
                        break 'release;
                    }
                    c.cycles += 4;
                    call_ptr(c, 0x0031_7e50, rets.free_one)?;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    break 'tail;
                }
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, rets.free_many)?;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
    }
    c.cycles += 3;
    let finish = c.g[S3][0];
    Addr(this_of(c).wrapping_add(8)).sw(c, finish);
    let end = c.g[V0][0];
    Addr(this_of(c).wrapping_add(0xc)).sw(c, end);
    let start = c.g[S2][0];
    Addr(this_of(c).wrapping_add(4)).sw(c, start);
    Ok(())
}

/// `cFEPopupOptionReset_onWidgetMenu` (0x1dea80): maps the menu key in `a2`
/// (the third argument) to a result. Keys below 7 and keys of 10 or more give
/// 0x101; keys 7, 8 and 9 give 0x100. The key is compared as an unsigned
/// 64-bit value, so a negative key counts as large. `a0` is left as the
/// "key below 7" flag when the key is below 10.
pub fn cfe_popup_option_reset_on_widget_menu(c: &mut Cpu) {
    c.cycles += 3;
    let below_ten = (c.g[A2][0] < 10) as u64;
    c.g[V1][0] = below_ten;
    c.g[V0][0] = 0x101;
    if below_ten != 0 {
        c.cycles += 4;
        let below_seven = (c.g[A2][0] < 7) as u64;
        c.g[A0][0] = below_seven;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = 0x100;
        if below_seven != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    ret(c);
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

    use crate::difftest::fakes;

    pub(crate) use crate::game::fe::fepopupmisc::tests::leaf_fill;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `cFEPopupOptionReset_v23` (0x001d_eb50) against the generated body.
    #[test]
    fn c_fe_popup_option_reset_v23_001deb50_matches() {
        check(
            ee::s04::f_001deb50__generated,
            c_fe_popup_option_reset_v23_001deb50,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1deb50, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// The gate word of this module's fakes, in the test machine's RAM.
    pub(crate) const GATE: u32 = DATA + 0x0020_1540;

    pub(crate) const GATE_ON: u32 = 0x1540_c0de;

    /// The V0 the fake callees return (read from `RESULT_CELL`).
    pub(crate) const RESULT_CELL: u32 = DATA + 0x0020_1550;

    /// A block the fake allocator hands out (inside the fixture's data area).
    pub(crate) const NEW_BLOCK: u32 = DATA + 0x0008_0000;

    pub(crate) fn gate_on(c: &mut Cpu) {
        c.bus.write32(GATE, GATE_ON);
        c.bus.write32(RESULT_CELL, NEW_BLOCK);
    }

    /// A gated fake of one callee: with the gate on it returns at once with
    /// V0 = `RESULT_CELL` (allocators, getters, and void callees alike);
    /// with the gate off it runs the body the slot held before.
    macro_rules! oc1540_fake {
        ($fake:ident, $prev:ident, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(GATE) != GATE_ON {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                c.g[V0][0] = c.bus.read32(RESULT_CELL) as u64;
                ret(c);
            }
        };
    }

    oc1540_fake!(
        oc1540_fake_alloc_one,
        OC1540_PREV_ALLOC_ONE,
        ee::s10::f_00317d70__generated
    );

    oc1540_fake!(
        oc1540_fake_alloc_many,
        OC1540_PREV_ALLOC_MANY,
        ee::s10::f_00317e30__generated
    );

    oc1540_fake!(
        oc1540_fake_free_one,
        OC1540_PREV_FREE_ONE,
        ee::s10::f_00317e50__generated
    );

    oc1540_fake!(
        oc1540_fake_free_many,
        OC1540_PREV_FREE_MANY,
        ee::s10::f_00317e98__generated
    );

    oc1540_fake!(
        oc1540_fake_base_ctor,
        OC1540_PREV_BASE_CTOR,
        ee::s13::f_0039e2a0__generated
    );

    oc1540_fake!(
        oc1540_fake_base_ctor_4,
        OC1540_PREV_BASE_CTOR_4,
        ee::s13::f_0039e318__generated
    );

    oc1540_fake!(
        oc1540_fake_memcard,
        OC1540_PREV_MEMCARD,
        ee::s04::f_001d53b0__generated
    );

    oc1540_fake!(
        oc1540_fake_be,
        OC1540_PREV_BE,
        ee::s01::f_0014dc80__generated
    );

    oc1540_fake!(
        oc1540_fake_iface,
        OC1540_PREV_IFACE,
        ee::s01::f_0014dd58__generated
    );

    oc1540_fake!(
        oc1540_fake_collectibles,
        OC1540_PREV_COLLECTIBLES,
        ee::s01::f_001589b0__generated
    );

    oc1540_fake!(
        oc1540_fake_big,
        OC1540_PREV_BIG,
        ee::s01::f_00158a50__generated
    );

    oc1540_fake!(
        oc1540_fake_earnings,
        OC1540_PREV_EARNINGS,
        ee::s01::f_00158af0__generated
    );

    oc1540_fake!(
        oc1540_fake_track,
        OC1540_PREV_TRACK,
        ee::s01::f_00158910__generated
    );

    /// Hooks the fakes above into their slots (once per process).
    pub(crate) fn install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc1540_fake_alloc_one,
            &OC1540_PREV_ALLOC_ONE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1540_fake_alloc_many,
            &OC1540_PREV_ALLOC_MANY,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1540_fake_free_one,
            &OC1540_PREV_FREE_ONE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1540_fake_free_many,
            &OC1540_PREV_FREE_MANY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1540_fake_base_ctor,
            &OC1540_PREV_BASE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E318),
            oc1540_fake_base_ctor_4,
            &OC1540_PREV_BASE_CTOR_4,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D53B0),
            oc1540_fake_memcard,
            &OC1540_PREV_MEMCARD,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc1540_fake_be,
            &OC1540_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc1540_fake_iface,
            &OC1540_PREV_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001589B0),
            oc1540_fake_collectibles,
            &OC1540_PREV_COLLECTIBLES,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00158A50),
            oc1540_fake_big,
            &OC1540_PREV_BIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00158AF0),
            oc1540_fake_earnings,
            &OC1540_PREV_EARNINGS,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00158910),
            oc1540_fake_track,
            &OC1540_PREV_TRACK,
        );
    }

    /// A vector of pointers for `StdVector_InsertAux`, in the four shapes the
    /// function branches on: room behind the finish (seed % 4 == 0), a full
    /// vector of one element, a full vector of three, an empty vector.
    pub(crate) fn vector_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1db778, 0);
        gate_on(c);
        let vec = DATA + 0x1000;
        let data = DATA + 0x2000;
        let fast = seed % 4 == 0;
        let (start, finish, end, slots) = match seed % 4 {
            0 => {
                let k = 1 + r.next_u32() % 3;
                (data, data + 4 * k, data + 4 * (k + 2), k)
            }
            1 => (data, data + 4, data + 4, 1),
            2 => (data, data + 12, data + 12, 3),
            _ => (0, 0, 0, 0),
        };
        // Room: the insert point is an element (up to the last one); full:
        // any point up to the finish.
        let span = if fast { slots } else { slots + 1 };
        let pos = start + 4 * ((seed / 4) % span);
        c.bus.write32(vec + 4, start);
        c.bus.write32(vec + 8, finish);
        c.bus.write32(vec + 0xc, end);
        for i in 0..slots {
            c.bus.write32(data + 4 * i, r.next_u32());
        }
        let x = DATA + 0x3000;
        c.bus.write32(x, r.next_u32());
        c.g[A0][0] = sx(vec);
        c.g[A1][0] = sx(pos);
        c.g[A2][0] = sx(x);
    }

    #[test]
    fn std_list_clear_matches() {
        install_fakes();
        check(
            ee::s04::f_001db718__generated,
            std_list_clear,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x1db718, 0);
                gate_on(c);
                // The list object at +4 holds the sentinel; 0..3 nodes follow it.
                let list = DATA + 0x1000;
                let head = DATA + 0x2000;
                c.bus.write32(list + 4, head);
                let mut prev = head;
                for i in 0..seed % 4 {
                    let node = DATA + 0x2100 + 0x40 * i;
                    c.bus.write32(prev, node);
                    prev = node;
                }
                c.bus.write32(prev, head);
                c.g[A0][0] = sx(list);
            },
        );
    }

    #[test]
    fn std_vector_insert_aux_matches() {
        install_fakes();
        check(
            ee::s04::f_001db778__generated,
            std_vector_insert_aux,
            200,
            &|c, seed| vector_setup(c, seed),
        );
    }

    #[test]
    fn std_vector_insert_aux_2_matches() {
        install_fakes();
        check(
            ee::s04::f_001db918__generated,
            std_vector_insert_aux_2,
            200,
            &|c, seed| vector_setup(c, seed),
        );
    }

    #[test]
    fn cfe_popup_option_reset_on_widget_menu_matches() {
        check(
            ee::s04::f_001dea80__generated,
            cfe_popup_option_reset_on_widget_menu,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x001d_ea80, 0);
                // Keys below 7, 7..9, 10 and up, negative, and a high-word key.
                c.g[A2][0] = match seed % 7 {
                    0 => 0,
                    1 => 6,
                    2 => 7,
                    3 => 9,
                    4 => 10,
                    5 => sx(0xffff_ffff),
                    _ => (1u64 << 40) | 3,
                };
            },
        );
    }
}
