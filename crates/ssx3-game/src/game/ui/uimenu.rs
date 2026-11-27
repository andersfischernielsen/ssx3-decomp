//! `src/ui/uimenu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::ai::computer::lh_2;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::camera::cameracontroller::delete_owner;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::ui::uiengine::oc445_lbu;
pub(crate) use crate::game::ui::uitext::oc445_add;
pub(crate) use crate::game::ui::uitext::oc445_addu;
pub(crate) use crate::game::ui::uitext::oc445_lw;
pub(crate) use crate::game::ui::uitext::oc445_vcall;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_b6a0, ui_menu_render, "UIMenu_Render"),
    (0x0039_b760, uimenu_reset_scroll, "UIMenu_ResetScroll"),
    (
        0x0039_b7b0,
        ui_menu_find_first_selectable,
        "UIMenu_FindFirstSelectable",
    ),
    (0x0039_bad0, uipair_v04, "UIPair_v04"),
    (0x0039_bb50, ui_pair_set_visible, "UIPair_SetVisible"),
    (0x0039_bbd8, ui_pair_set_focus, "UIPair_SetFocus"),
    (0x0039_bd38, ui_pair_accept, "UIPair_Accept"),
    (0x0039_bd68, ui_pair_v03, "UIPair_v03"),
    (0x0039_be10, ui_pair_refresh_text, "UIPair_RefreshText"),
    (0x0039_be70, ui_pair_free_children, "UIPair_FreeChildren"),
    (0x0039_c130, ui_pair_render, "UIPair_Render"),
    (0x0039_c1c8, ui_pair_update_anim, "UIPair_UpdateAnim"),
    (0x0039_c240, c_ui_progress_dtor, "cUIProgress_dtor"),
    (0x0039_c2c8, cui_progress_load, "cUIProgress_Load"),
    (0x0039_c300, cui_progress_render, "cUIProgress_Render"),
    (
        0x0039_c398,
        c_ui_progress_update_anim,
        "cUIProgress_UpdateAnim",
    ),
    (0x0039_b960, ui_menu_set_selection, "UIMenu_SetSelection"),
    (0x0039_b000, ui_menu_on_input, "cUIMenu_onInput"),
    (0x0039_c428, cuiprogress_set_value, "cUIProgress_SetValue"),
    (
        0x0039_ad38,
        ui_menu_set_current_item,
        "UIMenu_SetCurrentItem",
    ),
    (0x0039_bab0, ui_pair_load, "UIPair_Load"),
    (0x0039_bed8, ui_pair_handle_input, "UIPair_HandleInput"),
    (0x0039_ae98, ui_menu_select_index, "UIMenu_SelectIndex"),
    (
        0x0039_b878,
        ui_menu_find_last_selectable,
        "UIMenu_FindLastSelectable",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_OBJECT_LOAD;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;

    pub(crate) use crate::game::fe::fepopup::callee::C0039C870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039FF78;

    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
}

/// `UIPair::Accept(self, node)` (0x39bd38): `node->v02(self)`.
pub fn ui_pair_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0039_bd5c, None))
}

/// `void cUIProgress_Load(cUIProgress* self)` (0x39c2c8): load the base menu
/// object, then set the bar's min (10) and max (100).
pub fn cui_progress_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_c2dc)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = 0x64;
        let s0 = c.g[S0][0];
        let v0 = c.g[V0][0];
        let v1 = c.g[V1][0];
        Addr(s0 as u32).off(0x84).sw(c, v0);
        Addr(s0 as u32).off(0x80).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIMenu_ResetScroll(UIMenu* m, int count)` (0x39b760): clear the
/// scroll cursor, remember the item count, flag the widget dirty (`+0x90 |=
/// 4`) and reset the first visible row.
pub fn uimenu_reset_scroll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sb(r32(c, S0).wrapping_add(0x95), 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x74));
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x90));
        c.sb(r32(c, S0).wrapping_add(0x96), c.g[A1][0]);
        c.g[V0][0] |= 4;
        c.sw(r32(c, S0).wrapping_add(0x90), c.g[V0][0]);
        callee::UI_LIST_COUNT.call(c, 0x0039_b78c)?;
        c.sb(r32(c, S0).wrapping_add(0x97), c.g[V0][0]);
        c.sb(r32(c, S0).wrapping_add(0x98), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_RefreshText(UIPair* pair)` (0x39be10): refresh the two text
/// widgets of the pair.
pub fn ui_pair_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let pair = c.g[A0][0];
        c.g[S0][0] = pair;
        c.g[A1][0] = Addr(at(c, S0, 0x78)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            c.g[V0][0] = Addr(at(c, A1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x80).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x84).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_be40)?;
            c.g[A1][0] = Addr(at(c, S0, 0x7c)).lw(c);
        } else {
            c.g[A1][0] = Addr(at(c, S0, 0x7c)).lw(c);
        }
        'second: {
            c.cycles += 2;
            let none = c.g[A1][0] == 0;
            frame.restore(c, S0, 0x10);
            if none {
                break 'second;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(at(c, A1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x80).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x84).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_be60)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_FreeChildren(UIPair* self)` (0x39be70): destroy the two child
/// objects (vtable slot 1 with argument 3) and clear the pointers.
pub fn ui_pair_free_children(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        let a_none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !a_none {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_bea0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_bea0)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        let b_none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !b_none {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_bec4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_bec4)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI pair / texture manager.

/// `func_0039C1C8` (0x39c1c8): `UIPair_UpdateAnim`. Updates the pair itself
/// (`UIObject_UpdateAnim`) and then each of its two children at `+0x78` and
/// `+0x7c` through `vtable[0x98]`. Signature: `void update_anim(this, flags)`.
pub fn ui_pair_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0] & 0xffff;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_c1e8);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_UPDATE_ANIM.call(c, 0x0039_c1e8)?;
        c.g[A2][0] = lw_r(c, S0, 0x78);
        let no_first = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_first {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, A2, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x98);
            c.g[V1][0] = lw_r(c, V0, 0x9c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0039_c208);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0039_c208)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw_r(c, S0, 0x7c);
        let no_second = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_second {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, A2, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x98);
            c.g[V1][0] = lw_r(c, V0, 0x9c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0039_c228);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0039_c228)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_v04(self, on)` (0x39bad0): store the pair's on flag
/// (`self+0x14`, bit 1) and notify the two child widgets through their
/// virtual `on` handlers.
pub fn uipair_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = 0xffff_ffff_ffff_fffd;
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[V1][0] = lw_at(c, p(r32(c, S0), 0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0x78));
        c.g[V1][0] &= c.g[A0][0];
        c.g[V1][0] |= c.g[V0][0];
        c.sw(p(r32(c, S0), 0x14), c.g[V1][0]);
        c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 8));
        let func = vslot(c, c.g[V0][0], 0x20, V1);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, func, 0x0039_bb1c)?;
        c.g[A2][0] = lw_at(c, p(r32(c, S0), 0x7c));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, p(c.g[A2][0] as u32, 8));
            let func = vslot(c, c.g[V0][0], 0x20, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0039_bb3c)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI objects.

/// `void UIPair_SetVisible(UIPair* self, int on)` (0x39bb50): set the pair's
/// "visible" bit and forward the visibility to both children.
pub fn ui_pair_set_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[A0][0] = 0xffff_ffff_ffff_ffbf;
        c.g[V1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 6);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        let null = c.g[A2][0] == 0;
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        if !null {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0039_bba0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0039_bba0)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !null {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0039_bbc0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0039_bbc0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_dtor(cUIProgress* self, int flags)` (0x39c240): release
/// the two child objects and run the base destructor.
pub fn c_ui_progress_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_4408);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x74)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 8, 0x0039_c284)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_c2a4)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_c2b0);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_c2b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_UpdateAnim(self, unsigned short ev)` (0x39c398): advance
/// the progress animator, then pass the event to the `+0x78` and `+0x74`
/// virtual objects.
pub fn c_ui_progress_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let _ev = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at_2(c, S0, 0xc));
        let no_anim = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0] & 0xffff;
        if !no_anim {
            c.cycles += 7;
            c.g[A2][0] = c.lbu(at_2(c, S0, 0x10));
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, callee::UI_ANIM_APPLY_2, 0x0039_c3c8)?;
            c.g[A0][0] = c.lw(at_2(c, S0, 0xc));
            c.g[A1][0] = c.lbu(at_2(c, S0, 0x10));
            call_ptr(c, callee::UI_ANIM_ADVANCE_2, 0x0039_c3d4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw(at_2(c, S0, 0x78));
        let no_a = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_a {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at_2(c, A2, 8));
            c.g[A0][0] = c.lh(at_2(c, V0, 0x98));
            c.g[V1][0] = c.lw(at_2(c, V0, 0x9c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x0039_c3f4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw(at_2(c, S0, 0x74));
        let no_b = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !no_b {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at_2(c, A2, 8));
            c.g[A0][0] = c.lh(at_2(c, V0, 0x98));
            c.g[V1][0] = c.lw(at_2(c, V0, 0x9c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let target = r32(c, V1);
            call_ptr(c, target, 0x0039_c414)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_Render(self, x, y)` (0x39c130): forward the pair's render to
/// the two child objects at_3 +0x78 and +0x7C, offset by the pair's own position.
pub fn ui_pair_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 21, 0x28);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(21, 12);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, S0, 0x78);
        let none = c.g[A1][0] == 0;
        c.mov_s(20, 13);
        if !none {
            c.cycles += 9;
            c.g[V1][0] = lw(c, A1, 0x8);
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x88).lh(c);
            c.add_s(12, 21, 12);
            c.add_s(13, 20, 13);
            c.g[V0][0] = lw(c, V1, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0039_c17c)?;
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, S0, 0x7c);
        if c.g[A1][0] != 0 {
            c.cycles += 10;
            c.g[V1][0] = lw(c, A1, 0x8);
            c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x88).lh(c);
            c.add_s(13, 20, 13);
            c.add_s(12, 21, 12);
            c.g[V0][0] = lw(c, V1, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0039_c1ac)?;
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_Render(self, x, y)` (0x39c300): render the progress pair's
/// two child objects when both are present.
pub fn cui_progress_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        c.g[A1][0] = lw(c, S0, 0x74);
        let mut draw = c.g[A1][0] != 0;
        if draw {
            c.cycles += 3;
            c.g[V0][0] = lw(c, S0, 0x78);
            draw = c.g[V0][0] != 0;
        }
        if draw {
            c.cycles += 20;
            c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = lw(c, A1, 0x8);
            c.add_s(20, 12, 20);
            c.add_s(21, 13, 21);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
            c.g[V1][0] = lw(c, V0, 0x8c);
            c.mov_s(12, 20);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.mov_s(13, 21);
            call_ptr(c, c.g[V1][0] as u32, 0x0039_c35c)?;
            c.g[A1][0] = lw(c, S0, 0x78);
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            c.g[V0][0] = lw(c, A1, 0x8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
            c.g[V1][0] = lw(c, V0, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x0039_c37c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* UIPair_v03(self, pred)` (0x39bd68): return `self` if the pair's
/// predicate matches, else the first matching child or next node.
pub fn ui_pair_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let pred = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, pred, 0x0039bd98)?;
        let hit = c.g[V0][0] != 0;
        let mut tail = false;
        if hit {
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
            let no_child = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if no_child {
                c.cycles += 1;
                c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                tail = true;
            } else {
                c.cycles += 7;
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
                let visit = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, visit, 0x0039bdc0)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
                } else {
                    c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                    tail = true;
                }
            }
        }
        if tail {
            c.cycles += 2;
            if c.g[A2][0] != 0 {
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
                let visit = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, visit, 0x0039bdf8)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
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

// ---------------------------------------------------------------------------
// `UIMenu_Render` (0x0039b6a0).
//
// `void UIMenu::Render(UIMenu* self, float x, float y)`: refresh the scroll
// range, then walk the item list, offsetting each item by the menu position
// and issuing its render call.

pub fn ui_menu_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_39b6dc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.g[S1][0] = c.g[A0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.mov_s(21, 12);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
            let t = c.g[A0][0] == 0u64;
            c.mov_s(20, 13);
            if t {
                break 'b_39b6dc;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x98));
                c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x97));
                c.g[RA][0] = sx(0x0039b6dc);
                c.g[A3][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                call_ptr(c, 0x0039da20, 0x0039b6dc)?;
                break 'b_39b6dc;
            }
        }
        'b_39b744: {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0039b6e4);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
            call_ptr(c, 0x00397840, 0x0039b6e4)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_39b744;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                'l_39b6f8: loop {
                    'b_39b730: {
                        c.cycles += 4;
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_39b730;
                        } else {
                            c.cycles += 10;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x44));
                            c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x48));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                            c.add_s(12, 12, 21);
                            c.add_s(13, 13, 20);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0039b72c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0039b72c)?;
                            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_39b730;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0039b738);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00397718, 0x0039b738)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0039b6f8);
                        }
                        continue 'l_39b6f8;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_39b744;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `int UIMenu_FindFirstSelectable(UIMenu *self, u32 skip)` (0x39b7b0): the
/// first enabled list entry after `skip`. A resumable loop (0x39b7e8).
pub fn ui_menu_find_first_selectable(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        let mut frame = Frame { size: 0x30 };
        loop {
            match pc {
                0x0039_b7b0 => {
                    c.cycles += 13;
                    frame = Frame::open(c, 0x30);
                    c.g[A1][0] &= 0xff;
                    frame.save(c, S0, 0x20);
                    c.g[V1][0] = c.g[A1][0] ^ 0xff;
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = 0;
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A0][0];
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[A1][0];
                    }
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x96)).lbu(c);
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xff;
                    if done {
                        pc = 0x0039_b864;
                    } else {
                        pc = 0x0039_b7e4;
                    }
                }
                0x0039_b7e4 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    pc = 0x0039_b7e8;
                }
                0x0039_b7e8 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_7870, 0x0039_b7f0)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x90)).lw(c);
                    c.g[V0][0] &= 0x4;
                    let no_flag = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    if no_flag {
                        pc = 0x0039_b838;
                    } else {
                        pc = 0x0039_b804;
                    }
                }
                0x0039_b804 => {
                    c.cycles += 11;
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x5c)).lw(c);
                    c.g[A2][0] = 1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = Addr(p_2(c.g[V0][0], 0xd0)).lw(c);
                    c.g[V1][0] = Addr(p_2(c.g[T0][0], 0x8)).lw(c);
                    c.g[A0][0] = Addr(p_2(c.g[V1][0], 0xa0)).lh(c);
                    c.g[V0][0] = Addr(p_2(c.g[V1][0], 0xa4)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0039_b828)?;
                    let no = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if no {
                        pc = 0x0039_b84c;
                    } else {
                        pc = 0x0039_b830;
                    }
                }
                0x0039_b830 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0039_b864;
                }
                0x0039_b838 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_2(c.g[V1][0], 0x14)).lw(c);
                    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 5) as u32);
                    c.g[V0][0] &= 1;
                    let no = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if no {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0039_b830;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0039_b830;
                    } else {
                        pc = 0x0039_b84c;
                    }
                }
                0x0039_b84c => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_2(c.g[S1][0], 0x96)).lbu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    if more {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0039_b7e8;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0039_b7e8;
                    } else {
                        pc = 0x0039_b860;
                    }
                }
                0x0039_b860 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xff;
                    pc = 0x0039_b864;
                }
                0x0039_b864 => {
                    c.cycles += 5;
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_b7b0, 0x0039_b878);
                    return Ok(());
                }
            }
        }
    });
}

/// `void func_0039BBD8(void* pair, int on)` (0x39bbd8): show/hide a UI pair.
/// Stores `on` in bit 4 of the flags at_4 `+0x14`, tells the entry at_4 `+0x78`
/// (`+8` vtable, slot 7), and, when there is a keyframer object at_4 `+0x7c`,
/// looks up its event for the wanted state, plays that frame on the screen at_4
/// `+0x5c` when one is found, and finally re-sets the keyframer and the owner
/// (slot 19 with 1 or 2).
pub fn ui_pair_set_focus(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_39bc28: {
        c.cycles += 15;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[A0][0] = 0xffffffffffffffef;
        c.g[V1][0] = lw_3(c, at_4(c, S0, 0x14));
        c.g[V0][0] = sx(r32(c, V0) << 4);
        c.g[A2][0] = lw_3(c, at_4(c, S0, 0x78));
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        sw(c, at_4(c, S0, 0x14), c.g[V1][0]);
        if t {
            break 'b_39bc28;
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_3(c, at_4(c, A2, 0x8));
            c.g[A0][0] = lh_2(c, at_4(c, V0, 0x38));
            c.g[V1][0] = lw_3(c, at_4(c, V0, 0x3c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bc28);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bc28).is_err() {
                return;
            }
            break 'b_39bc28;
        }
    }
    'b_39bd28: {
        c.cycles += 3;
        c.g[A0][0] = lw_3(c, at_4(c, S0, 0x7c));
        if c.g[A0][0] == 0u64 {
            frame.restore(c, S0, 0x20);
            break 'b_39bd28;
        } else {
            'b_39bcb4: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;

                if t {
                    break 'b_39bcb4;
                } else {
                    'b_39bc70: {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0039bc44);
                        c.g[A1][0] = 0u64;
                        if callee::C0039FF78.call(c, 0x0039bc44).is_err() {
                            return;
                        }
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        if c.g[A1][0] == c.g[V0][0] {
                            c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                            break 'b_39bc70;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = lw_3(c, at_4(c, S0, 0x7c));
                            c.g[A0][0] = lw_3(c, at_4(c, V0, 0x5c));
                            if c.g[A0][0] == 0u64 {
                                c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                                break 'b_39bc70;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0039bc6c);
                                c.g[A2][0] = 0x1;
                                if callee::C0039C870.call(c, 0x0039bc6c).is_err() {
                                    return;
                                }
                                c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                                break 'b_39bc70;
                            }
                        }
                    }
                    c.cycles += 17;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = lw_3(c, at_4(c, A2, 0x8));
                    c.g[A0][0] = lh_2(c, at_4(c, V0, 0x38));
                    c.g[V1][0] = lw_3(c, at_4(c, V0, 0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039bc88);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0039bc88).is_err() {
                        return;
                    }
                    c.g[A0][0] = lw_3(c, at_4(c, S0, 0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A1][0] = lw_3(c, at_4(c, S0, 0x7c));
                    c.g[A3][0] = lw_3(c, at_4(c, A0, 0xd0));
                    c.g[V0][0] = lw_3(c, at_4(c, A3, 0x8));
                    c.g[A0][0] = lh_2(c, at_4(c, V0, 0x98));
                    c.g[V1][0] = lw_3(c, at_4(c, V0, 0x9c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039bcac);
                    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0039bcac).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'b_39bd28;
                    } else {
                        break 'b_39bcb4;
                    }
                }
            }
            'b_39bce8: {
                c.cycles += 6;
                c.g[RA][0] = sx(0x0039bcbc);
                c.g[A1][0] = 0x1;
                if callee::C0039FF78.call(c, 0x0039bcbc).is_err() {
                    return;
                }
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                if c.g[A1][0] == c.g[V0][0] {
                    c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                    break 'b_39bce8;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = lw_3(c, at_4(c, S0, 0x7c));
                    c.g[A0][0] = lw_3(c, at_4(c, V0, 0x5c));
                    if c.g[A0][0] == 0u64 {
                        c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                        break 'b_39bce8;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0039bce4);
                        c.g[A2][0] = 0x1;
                        if callee::C0039C870.call(c, 0x0039bce4).is_err() {
                            return;
                        }
                        c.g[A2][0] = lw_3(c, at_4(c, S0, 0x7c));
                        break 'b_39bce8;
                    }
                }
            }
            c.cycles += 16;
            c.g[A1][0] = 0u64;
            c.g[V0][0] = lw_3(c, at_4(c, A2, 0x8));
            c.g[A0][0] = lh_2(c, at_4(c, V0, 0x38));
            c.g[V1][0] = lw_3(c, at_4(c, V0, 0x3c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bd00);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bd00).is_err() {
                return;
            }
            c.g[A0][0] = lw_3(c, at_4(c, S0, 0x5c));
            c.g[A2][0] = 0x2;
            c.g[A1][0] = lw_3(c, at_4(c, S0, 0x7c));
            c.g[A3][0] = lw_3(c, at_4(c, A0, 0xd0));
            c.g[V0][0] = lw_3(c, at_4(c, A3, 0x8));
            c.g[A0][0] = lh_2(c, at_4(c, V0, 0x98));
            c.g[V1][0] = lw_3(c, at_4(c, V0, 0x9c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bd24);
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bd24).is_err() {
                return;
            }
            frame.restore(c, S0, 0x20);
            break 'b_39bd28;
        }
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x30));
    c.pc = t;
    return;
}

pub(crate) mod oc445_callee {

    pub(crate) use crate::game::ui::uitext::oc445_callee::UI_LIST_GET_AT;

    pub(crate) use crate::game::ui::uitext::oc445_callee::UI_MENU_SET_CURRENT_ITEM;
}

// ---------------------------------------------------------------------------
// UIMenu_SetSelection.

/// `void UIMenu_SetSelection(UIMenu *menu, uint8 index)` (0x39b960): select
/// item `index`. When the menu does not scroll (attribute bit 4 clear, or
/// the screen manager's flag word bit 5 clear) only the stored index
/// (`+0xa4`) is set. Otherwise the current index (`+0x95`) is set, the
/// scroll window (`+0x98` offset, `+0x97` visible count) is moved to keep it
/// in view (notifying the manager's vtable slot 0xa0 with events 3/4 when
/// scrolling down, 2/4 when scrolling up), and the list item at
/// `index - offset` becomes the current item.
pub fn ui_menu_set_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = oc445_lw(c, S0, 0x14);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[V0][0] &= 1;
        let attr_clear = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0] & 0xff;
        let mut scrolls = false;
        if !attr_clear {
            c.cycles += 7;
            c.g[A3][0] = oc445_lw(c, S0, 0x5c);
            c.g[V1][0] = oc445_lw(c, A3, 0xd0);
            c.g[V0][0] = oc445_lw(c, V1, 0x1c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = oc445_lbu(c, S0, 0x98);
                scrolls = true;
            }
        }
        if !scrolls {
            c.cycles += 2;
            Addr(oc445_add(c, S0, 0xa4) as u32).sb(c, c.g[S1][0]);
        } else {
            'tail: {
                'above: {
                    c.cycles += 5;
                    c.g[V1][0] = oc445_lbu(c, S0, 0x97);
                    c.g[V0][0] = oc445_addu(c, A0, V1);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let inside_bottom = c.g[V0][0] != 0;
                    Addr(oc445_add(c, S0, 0x95) as u32).sb(c, c.g[S1][0]);
                    if inside_bottom {
                        break 'above;
                    }
                    // Below the window: scroll down so the item is the last
                    // visible one.
                    c.cycles += 24;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = oc445_add(c, V0, 0x1);
                    c.g[A2][0] = 0x3;
                    Addr(oc445_add(c, S0, 0x98) as u32).sb(c, c.g[V0][0]);
                    c.g[T0][0] = oc445_lw(c, A3, 0xd0);
                    c.g[A3][0] = c.g[V0][0] & 0xff;
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_b9f0)?;
                    c.g[A0][0] = oc445_lw(c, S0, 0x5c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A3][0] = oc445_lbu(c, S0, 0x98);
                    c.g[A2][0] = 0x4;
                    c.g[T0][0] = oc445_lw(c, A0, 0xd0);
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_ba18)?;
                    c.g[A1][0] = oc445_lbu(c, S0, 0x98);
                    break 'tail;
                }
                // The item is not below the window: scroll up when it is
                // above the offset.
                c.cycles += 3;
                c.g[V0][0] = (c.g[S1][0] < c.g[A0][0]) as u64;
                let not_above = c.g[V0][0] == 0;
                c.g[A2][0] = 0x2;
                if !not_above {
                    c.cycles += 19;
                    Addr(oc445_add(c, S0, 0x98) as u32).sb(c, c.g[S1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[T0][0] = oc445_lw(c, A3, 0xd0);
                    c.g[A3][0] = c.g[S1][0] & 0xff;
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_ba50)?;
                    c.g[A0][0] = oc445_lw(c, S0, 0x5c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A3][0] = oc445_lbu(c, S0, 0x98);
                    c.g[A2][0] = 0x4;
                    c.g[T0][0] = oc445_lw(c, A0, 0xd0);
                    c.g[V1][0] = oc445_lw(c, T0, 0x8);
                    oc445_vcall(c, V1, T0, 0xa0, V0, 0x0039_ba78)?;
                }
                c.cycles += 1;
                c.g[A1][0] = oc445_lbu(c, S0, 0x98);
            }
            c.cycles += 7;
            c.g[A0][0] = oc445_add(c, S0, 0x74);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            oc445_callee::UI_LIST_GET_AT.call(c, 0x0039_ba88)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 0x1;
            oc445_callee::UI_MENU_SET_CURRENT_ITEM.call(c, 0x0039_ba98)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// cUIThing `mAttr` (word).
pub(crate) const ONIN_ATTR: u32 = 0x14;

/// cUIMenu `mList` (the cList the items live in).
pub(crate) const ONIN_LIST: u32 = 0x74;

/// cUIMenu `mpScreen`.
pub(crate) const ONIN_SCREEN: u32 = 0x5c;

/// cUIMenu `mAttrMenu` (word): bit 3 is mDisableDirSound.
pub(crate) const ONIN_ATTR_MENU: u32 = 0x90;

/// cUIMenu `mInputMap[4]` (signed bytes at 0x91..0x94).
pub(crate) const ONIN_INPUT_MAP: u32 = 0x91;

/// cUIMenu `mScrollItemCurrent` (the selected row).
pub(crate) const ONIN_SCROLL_CUR: u32 = 0x95;

/// cUIMenu `mScrollItemOffset`.
pub(crate) const ONIN_SCROLL_OFFSET: u32 = 0x98;

/// cUIMenu `mpSelected` (the focused item).
pub(crate) const ONIN_SELECTED: u32 = 0xa0;

/// Keyframer-event lookup (`cUIThing_getKeyframerEvent`, 0x39ff78).
pub(crate) const ONIN_GET_EVENT_THREAD: Callee = Callee {
    addr: 0x0039_ff78,
    f: ee::s13::f_0039ff78,
};

/// `UIList_GetAt(list, index)` (0x397870).
pub(crate) const ONIN_LIST_GET_AT: Callee = Callee {
    addr: 0x0039_7870,
    f: ee::s13::f_00397870,
};

/// `UIMenu_FindNextSelectable` (0x39ab50).
pub(crate) const ONIN_FIND_NEXT: Callee = Callee {
    addr: 0x0039_ab50,
    f: ee::s13::f_0039ab50,
};

/// `UIMenu_FindPrevSelectable` (0x39ac48).
pub(crate) const ONIN_FIND_PREV: Callee = Callee {
    addr: 0x0039_ac48,
    f: ee::s13::f_0039ac48,
};

/// `UIMenu_SelectIndex(menu, index)` (0x39ae98).
pub(crate) const ONIN_SELECT_INDEX: Callee = Callee {
    addr: 0x0039_ae98,
    f: ee::s13::f_0039ae98,
};

/// `cUIScreen_playFrame(screen, event, 1)` (0x39c870).
pub(crate) const ONIN_PLAY_FRAME: Callee = Callee {
    addr: 0x0039_c870,
    f: ee::s13::f_0039c870,
};

/// `cUIThing_getState` (0x39ff50): the owner's state data.
pub(crate) const ONIN_GET_STATE_DATA: Callee = Callee {
    addr: 0x0039_ff50,
    f: ee::s13::f_0039ff50,
};

/// `UIObject_GetUserData` (0x3a0318).
pub(crate) const ONIN_GET_USER_DATA: Callee = Callee {
    addr: 0x003a_0318,
    f: ee::s13::f_003a0318,
};

/// `base + off` as a 32-bit guest address.
#[inline]
pub(crate) fn onin_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// A gcc virtual call: `A0 = lh off(vt)`, `func = lw off+4(vt)`, `A0 = obj +
/// A0`, `jalr func` returning to `ret`. `vt`, `obj` and `func` name registers.
pub(crate) fn onin_vcall(
    c: &mut Cpu,
    vt: usize,
    obj: usize,
    off: u32,
    func: usize,
    ret: u32,
) -> Flow {
    c.g[A0][0] = c.lh(onin_at(c.g[vt][0], off));
    c.g[func][0] = c.lw(onin_at(c.g[vt][0], off + 4));
    let target = c.g[func][0] as u32;
    c.g[A0][0] = sx(onin_at(c.g[obj][0], c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `int cUIMenu::onInput(cUIInput* in)` (0x39b000): the menu takes an input.
/// A hidden menu, one with no selected item, or one whose mAttr bit 4 is
/// clear ignores it (returns 0). The selected item gets the input first (its
/// `onInput` returning non-zero ends it with 1). Otherwise the direction
/// buttons (mInputMap; left/right for horizontal menus, mAttr bit 0) move the
/// selection, and the inputs 4-8 are reported to the item's widget event and
/// to the owner's focus handlers. Their results pick the item's sound; the
/// UI sound listener plays it. Returns 1 when the menu used the input.
pub fn ui_menu_on_input(c: &mut Cpu) {
    guest(c, ui_menu_on_input_body)
}

pub(crate) fn ui_menu_on_input_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x0039_b000);
    c.cycles += 12;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x20);
    frame.save(c, S3, 0x10);
    frame.save_ra(c, 0);

    'b_39b680: {
        'b_39b64c: {
            'b_39b618: {
                'b_39b610: {
                    c.g[V1][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] &= 1;
                    let hidden = c.g[V0][0] != 0;
                    c.g[S1][0] = c.g[A1][0];
                    if hidden {
                        break 'b_39b610;
                    }
                    c.cycles += 3;
                    c.g[A2][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                    let no_selection = c.g[A2][0] == 0;
                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
                    if no_selection {
                        break 'b_39b610;
                    }
                    c.cycles += 3;
                    c.g[V0][0] &= 1;
                    let inactive = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if inactive {
                        break 'b_39b680;
                    }
                    // The selected item gets the input first.
                    'b_39b08c: {
                        'b_39b088: {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw(onin_at(c.g[A2][0], ONIN_ATTR));
                            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                            c.g[V0][0] &= 1;
                            let not_enabled = c.g[V0][0] == 0;
                            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                            if not_enabled {
                                break 'b_39b088;
                            }
                            c.cycles += 3;
                            c.g[V0][0] &= 1;
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                break 'b_39b08c;
                            }
                            c.cycles += 8;
                            c.g[V1][0] = c.lw(onin_at(c.g[A2][0], 0x8));
                            c.g[A1][0] = c.g[S1][0];
                            onin_vcall(c, V1, A2, 0x78, V0, 0x0039_b080)?;
                            let used = c.g[V0][0] != 0;
                            c.g[V0][0] = 1;
                            if used {
                                break 'b_39b680;
                            }
                            break 'b_39b088;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                    }
                    // Input 3 / 1 / 2 / 0 are the direction buttons; the
                    // scroll index and the owner decide what they move.
                    'b_39b5f8: {
                        'b_39b5e8: {
                            'b_39b5e4: {
                                'b_39b18c: {
                                    'b_39b0f0: {
                                        'b_39b0c4: {
                                            c.cycles += 9;
                                            c.g[S3][0] = 0;
                                            c.g[A1][0] =
                                                c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 3));
                                            c.g[S2][0] =
                                                c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                            onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b0a8)?;
                                            let t = c.g[V0][0] == 0;
                                            c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                            if t {
                                                break 'b_39b0c4;
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                            c.g[V0][0] ^= 1;
                                            c.g[V0][0] &= 1;
                                            if c.g[V0][0] != 0 {
                                                break 'b_39b0f0;
                                            }
                                            break 'b_39b0c4;
                                        }
                                        c.cycles += 7;
                                        c.g[A1][0] = c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 1));
                                        onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b0d8)?;
                                        let t = c.g[V0][0] == 0;
                                        c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                        if t {
                                            break 'b_39b18c;
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                        c.g[V0][0] &= 1;
                                        if c.g[V0][0] == 0 {
                                            break 'b_39b18c;
                                        }
                                        break 'b_39b0f0;
                                    }
                                    'b_39b120: {
                                        c.cycles += 7;
                                        c.g[A1][0] = c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 3));
                                        onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b104)?;
                                        let t = c.g[V0][0] == 0;
                                        c.g[A1][0] = 0x7;
                                        if t {
                                            break 'b_39b120;
                                        }
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                        c.g[V0][0] ^= 1;
                                        c.g[V0][0] &= 1;
                                        if c.g[V0][0] != 0 {
                                            c.g[A1][0] = 0x9;
                                        }
                                        break 'b_39b120;
                                    }
                                    'b_39b14c: {
                                        c.cycles += 6;
                                        c.g[A0][0] = c.g[S0][0];
                                        ONIN_GET_EVENT_THREAD.call(c, 0x0039_b128)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[V0][0] = 0xffff;
                                        if c.g[A1][0] == c.g[V0][0] {
                                            break 'b_39b14c;
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                        if c.g[A0][0] == 0 {
                                            break 'b_39b14c;
                                        }
                                        c.cycles += 2;
                                        c.g[A2][0] = 0x1;
                                        ONIN_PLAY_FRAME.call(c, 0x0039_b14c)?;
                                        break 'b_39b14c;
                                    }
                                    // Button 3 (down, or right when horizontal) moves
                                    // the selection forward.
                                    c.cycles += 12;
                                    c.g[A0][0] = c.g[S0][0];
                                    ONIN_FIND_NEXT.call(c, 0x0039_b154)?;
                                    c.sb(onin_at(c.g[S0][0], ONIN_SCROLL_CUR), c.g[V0][0]);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    ONIN_SELECT_INDEX.call(c, 0x0039_b164)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR_MENU));
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                                    c.g[V0][0] &= 1;
                                    if c.g[V0][0] != 0 {
                                        break 'b_39b64c;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != 0;
                                    c.g[S3][0] = 1;
                                    if t {
                                        break 'b_39b64c;
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                    break 'b_39b5e4;
                                }
                                'b_39b5e0: {
                                    'b_39b284: {
                                        'b_39b1e8: {
                                            'b_39b1bc: {
                                                c.cycles += 7;
                                                c.g[A1][0] =
                                                    c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 2));
                                                onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b1a0)?;
                                                let t = c.g[V0][0] == 0;
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                if t {
                                                    break 'b_39b1bc;
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                                c.g[V0][0] ^= 1;
                                                c.g[V0][0] &= 1;
                                                if c.g[V0][0] != 0 {
                                                    break 'b_39b1e8;
                                                }
                                                break 'b_39b1bc;
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP));
                                            onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b1d0)?;
                                            let t = c.g[V0][0] == 0;
                                            c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                            if t {
                                                break 'b_39b284;
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                            c.g[V0][0] &= 1;
                                            if c.g[V0][0] == 0 {
                                                break 'b_39b284;
                                            }
                                            break 'b_39b1e8;
                                        }
                                        'b_39b218: {
                                            c.cycles += 7;
                                            c.g[A1][0] =
                                                c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 2));
                                            onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b1fc)?;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x6;
                                            if t {
                                                break 'b_39b218;
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                            c.g[V0][0] ^= 1;
                                            c.g[V0][0] &= 1;
                                            if c.g[V0][0] != 0 {
                                                c.g[A1][0] = 0x8;
                                            }
                                            break 'b_39b218;
                                        }
                                        'b_39b244: {
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S0][0];
                                            ONIN_GET_EVENT_THREAD.call(c, 0x0039_b220)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[V0][0] = 0xffff;
                                            if c.g[A1][0] == c.g[V0][0] {
                                                break 'b_39b244;
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                            if c.g[A0][0] == 0 {
                                                break 'b_39b244;
                                            }
                                            c.cycles += 2;
                                            c.g[A2][0] = 0x1;
                                            ONIN_PLAY_FRAME.call(c, 0x0039_b244)?;
                                            break 'b_39b244;
                                        }
                                        // Button 2 (up, or left when horizontal) moves
                                        // the selection back.
                                        c.cycles += 12;
                                        c.g[A0][0] = c.g[S0][0];
                                        ONIN_FIND_PREV.call(c, 0x0039_b24c)?;
                                        c.sb(onin_at(c.g[S0][0], ONIN_SCROLL_CUR), c.g[V0][0]);
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        ONIN_SELECT_INDEX.call(c, 0x0039_b25c)?;
                                        c.g[V1][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR_MENU));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                                        c.g[V0][0] &= 1;
                                        if c.g[V0][0] != 0 {
                                            break 'b_39b64c;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0;
                                        c.g[S3][0] = 1;
                                        if t {
                                            break 'b_39b5e0;
                                        }
                                        c.cycles += 2;
                                        break 'b_39b64c;
                                    }
                                    // Inputs 5 and 6 (focus move) and 7, 8 and 4
                                    // (the item's focus events): the owner's focus
                                    // handlers decide the sound.
                                    'b_39b5d4: {
                                        'b_39b358: {
                                            c.cycles += 6;
                                            onin_vcall(c, V1, S1, 0x28, V0, 0x0039_b294)?;
                                            if c.g[V0][0] == 0 {
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                break 'b_39b358;
                                            }
                                            'b_39b348: {
                                                'b_39b344: {
                                                    c.cycles += 31;
                                                    c.g[V0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                    c.g[A1][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_sub(c.g[A1][0] as u32));
                                                    ONIN_LIST_GET_AT.call(c, 0x0039_b2b0)?;
                                                    c.g[V1][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[S2][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A2][0] = 0x5;
                                                    c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                    c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                    onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b2d8)?;
                                                    c.g[V0][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A2][0] = 0x6;
                                                    c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                    c.g[A3][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                    c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_sub(c.g[A3][0] as u32));
                                                    onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b308)?;
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x100;
                                                    let t = c.g[V0][0] == 0;
                                                    c.g[A1][0] = 0x2;
                                                    if t {
                                                        break 'b_39b344;
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    ONIN_GET_EVENT_THREAD.call(c, 0x0039_b320)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0xffff;
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x1;
                                                    if t {
                                                        break 'b_39b348;
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] =
                                                        c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                    if c.g[A0][0] == 0 {
                                                        break 'b_39b348;
                                                    }
                                                    c.cycles += 2;
                                                    c.g[A2][0] = 0x1;
                                                    ONIN_PLAY_FRAME.call(c, 0x0039_b344)?;
                                                    break 'b_39b344;
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                break 'b_39b348;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_39b5d4;
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                            break 'b_39b5e8;
                                        }
                                        'b_39b42c: {
                                            c.cycles += 6;
                                            onin_vcall(c, V1, S1, 0x30, V0, 0x0039_b368)?;
                                            if c.g[V0][0] == 0 {
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                break 'b_39b42c;
                                            }
                                            'b_39b41c: {
                                                'b_39b418: {
                                                    c.cycles += 31;
                                                    c.g[V0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                    c.g[A1][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_sub(c.g[A1][0] as u32));
                                                    ONIN_LIST_GET_AT.call(c, 0x0039_b384)?;
                                                    c.g[V1][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[S2][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A2][0] = 0x6;
                                                    c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                    c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                    onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b3ac)?;
                                                    c.g[V0][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A2][0] = 0x7;
                                                    c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                    c.g[A3][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                    c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_sub(c.g[A3][0] as u32));
                                                    onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b3dc)?;
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x100;
                                                    let t = c.g[V0][0] == 0;
                                                    c.g[A1][0] = 0x3;
                                                    if t {
                                                        break 'b_39b418;
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    ONIN_GET_EVENT_THREAD.call(c, 0x0039_b3f4)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0xffff;
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x1;
                                                    if t {
                                                        break 'b_39b41c;
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] =
                                                        c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                    if c.g[A0][0] == 0 {
                                                        break 'b_39b41c;
                                                    }
                                                    c.cycles += 2;
                                                    c.g[A2][0] = 0x1;
                                                    ONIN_PLAY_FRAME.call(c, 0x0039_b418)?;
                                                    break 'b_39b418;
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                break 'b_39b41c;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_39b5d4;
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                            break 'b_39b5e8;
                                        }
                                        'b_39b500: {
                                            c.cycles += 6;
                                            onin_vcall(c, V1, S1, 0x40, V0, 0x0039_b43c)?;
                                            if c.g[V0][0] == 0 {
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                break 'b_39b500;
                                            }
                                            'b_39b4f0: {
                                                'b_39b4ec: {
                                                    c.cycles += 31;
                                                    c.g[V0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                    c.g[A1][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_sub(c.g[A1][0] as u32));
                                                    ONIN_LIST_GET_AT.call(c, 0x0039_b458)?;
                                                    c.g[V1][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[S2][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A2][0] = 0x8;
                                                    c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                    c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                    onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b480)?;
                                                    c.g[V0][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A2][0] = 0x9;
                                                    c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                    c.g[A3][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                    c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_sub(c.g[A3][0] as u32));
                                                    onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b4b0)?;
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x100;
                                                    let t = c.g[V0][0] == 0;
                                                    c.g[A1][0] = 0x5;
                                                    if t {
                                                        break 'b_39b4ec;
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    ONIN_GET_EVENT_THREAD.call(c, 0x0039_b4c8)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0xffff;
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x1;
                                                    if t {
                                                        break 'b_39b4f0;
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] =
                                                        c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                    if c.g[A0][0] == 0 {
                                                        break 'b_39b4f0;
                                                    }
                                                    c.cycles += 2;
                                                    c.g[A2][0] = 0x1;
                                                    ONIN_PLAY_FRAME.call(c, 0x0039_b4ec)?;
                                                    break 'b_39b4ec;
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                break 'b_39b4f0;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_39b5d4;
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                            break 'b_39b5e8;
                                        }
                                        c.cycles += 6;
                                        onin_vcall(c, V1, S1, 0x38, V0, 0x0039_b510)?;
                                        if c.g[V0][0] == 0 {
                                            c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                            break 'b_39b5f8;
                                        }
                                        'b_39b5c4: {
                                            'b_39b5c0: {
                                                c.cycles += 31;
                                                c.g[V0][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                c.g[A1][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_OFFSET));
                                                c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[A1][0] as u32));
                                                ONIN_LIST_GET_AT.call(c, 0x0039_b52c)?;
                                                c.g[V1][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                c.g[S2][0] = c.g[V0][0];
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A2][0] = 0x7;
                                                c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b554)?;
                                                c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[A0][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                c.g[A2][0] = 0x8;
                                                c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                c.g[A3][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_OFFSET));
                                                c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_sub(c.g[A3][0] as u32));
                                                onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b584)?;
                                                c.g[S1][0] = c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x100;
                                                let t = c.g[V0][0] == 0;
                                                c.g[A1][0] = 0x4;
                                                if t {
                                                    break 'b_39b5c0;
                                                }
                                                c.cycles += 6;
                                                c.g[A0][0] = c.g[S2][0];
                                                ONIN_GET_EVENT_THREAD.call(c, 0x0039_b59c)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_39b5c4;
                                                }
                                                c.cycles += 3;
                                                c.g[A0][0] = c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                if c.g[A0][0] == 0 {
                                                    break 'b_39b5c4;
                                                }
                                                c.cycles += 2;
                                                c.g[A2][0] = 0x1;
                                                ONIN_PLAY_FRAME.call(c, 0x0039_b5c0)?;
                                                break 'b_39b5c0;
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = c.g[S1][0] & 0x1;
                                            break 'b_39b5c4;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0;
                                        c.g[A1][0] = 0x1;
                                        if t {
                                            break 'b_39b5d4;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                        break 'b_39b5e8;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.g[S1][0] & 0x10;
                                    let t = c.g[V0][0] == 0;
                                    if t {
                                        break 'b_39b64c;
                                    }
                                    break 'b_39b5e0;
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                break 'b_39b5e4;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0;
                            break 'b_39b5e8;
                        }
                        // Input 9 (the item's user data, becomes the state's item).
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0039_b5f0);
                        ONIN_GET_USER_DATA.call(c, 0x0039_b5f0)?;
                        c.g[S3][0] = c.g[V0][0];
                        break 'b_39b64c;
                    }
                    c.cycles += 6;
                    onin_vcall(c, V1, S1, 0x20, V0, 0x0039_b608)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                        break 'b_39b618;
                    }
                    break 'b_39b610;
                }
                // Nothing used the input.
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'b_39b680;
            }
            // Input 4 (the item's focus event) reaches here with the scroll row in V0.
            c.cycles += 13;
            c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
            c.g[A1][0] = c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_OFFSET));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            ONIN_LIST_GET_AT.call(c, 0x0039_b628)?;
            c.g[V1][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 0x4;
            c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
            c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
            onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b64c)?;
            break 'b_39b64c;
        }
        // The UI sound listener (the owner's, else gUISoundNull) plays the result.
        'b_39b67c: {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            ONIN_GET_STATE_DATA.call(c, 0x0039_b654)?;
            c.g[A2][0] = c.lw(onin_at(c.g[V0][0], 0x14));
            c.g[V0][0] = sx(onin_at(c.g[GP][0], 0x2968));
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V0][0];
            }
            let no_listener = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S3][0];
            if no_listener {
                break 'b_39b67c;
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            onin_vcall(c, V0, A2, 0x10, V1, 0x0039_b67c)?;
            break 'b_39b67c;
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        break 'b_39b680;
    }

    c.cycles += 7;
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// cUIProgress field offsets.
pub(crate) const CUIPROGRESS_ATTR: u32 = 0x14;

pub(crate) const CUIPROGRESS_BACKGROUND: u32 = 0x74;

pub(crate) const CUIPROGRESS_KNOB: u32 = 0x78;

pub(crate) const CUIPROGRESS_VALUE: u32 = 0x7c;

pub(crate) const CUIPROGRESS_NUM_VALUES: u32 = 0x80;

/// `cUIProgress_SetValue` (0x39c428): stores the value, reads the background
/// widget's size (vtable slot 0xa0, or the zero vector `0x4ff0d8` copy when
/// there is no background) and scales its size
/// by value / numValues (x when attr bit 0 is clear, y when set), then hands that size to the knob widget (vtable
/// slot 0xa8). No knob or no numValues: nothing is set.
pub fn cuiprogress_set_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0050_0000;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let a1 = c.g[A1][0];
        c.sw(this.wrapping_add(CUIPROGRESS_VALUE), a1);
        c.g[A2][0] = c.lw(this.wrapping_add(CUIPROGRESS_BACKGROUND)) as u64;
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
        let zero = c.g[A3][0] as u32;
        c.ldl(3, zero.wrapping_add(0x7));
        c.ldr(3, zero);
        c.g[A0][0] = c.lw(zero.wrapping_add(0x8));
        let buf = sp(c);
        c.sdl(3, buf.wrapping_add(0x7));
        c.sdr(3, buf);
        let a0 = c.g[A0][0];
        c.sw(buf.wrapping_add(0x8), a0);
        c.g[A1][0] = c.g[SP][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let bg = c.g[A2][0] as u32;
            c.g[V0][0] = c.lw(bg.wrapping_add(0x8));
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = c.lh(vt.wrapping_add(0xa0));
            c.g[V1][0] = c.lw(vt.wrapping_add(0xa4));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(bg.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_c47c)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CUIPROGRESS_KNOB));
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CUIPROGRESS_NUM_VALUES));
            if c.g[V1][0] == 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                let this = c.g[S0][0] as u32;
                c.g[V0][0] = c.lw(this.wrapping_add(CUIPROGRESS_ATTR));
                c.g[V0][0] ^= 0x1;
                c.g[V0][0] &= 0x1;
                let y_axis = c.g[V0][0] == 0;
                let buf = sp(c);
                c.lwc1(1, buf);
                if !y_axis {
                    c.cycles += 8;
                    c.lwc1(0, this.wrapping_add(CUIPROGRESS_VALUE));
                    c.cvt_s_w(0, 0);
                    c.mtc1(3, 2);
                    c.cvt_s_w(2, 2);
                    c.div_s(0, 0, 2);
                    c.mul_s(1, 1, 0);
                    c.swc1(1, buf);
                } else {
                    c.cycles += 8;
                    c.lwc1(0, this.wrapping_add(CUIPROGRESS_VALUE));
                    c.cvt_s_w(0, 0);
                    c.lwc1(1, buf.wrapping_add(0x4));
                    c.mtc1(3, 2);
                    c.cvt_s_w(2, 2);
                    c.div_s(0, 0, 2);
                    c.mul_s(1, 1, 0);
                    c.swc1(1, buf.wrapping_add(0x4));
                }
                c.cycles += 8;
                c.g[A2][0] = c.lw(this.wrapping_add(CUIPROGRESS_KNOB)) as u64;
                c.g[A1][0] = c.g[SP][0];
                let knob = c.g[A2][0] as u32;
                c.g[V0][0] = c.lw(knob.wrapping_add(0x8));
                let vt = c.g[V0][0] as u32;
                c.g[A0][0] = c.lh(vt.wrapping_add(0xa8));
                c.g[V1][0] = c.lw(vt.wrapping_add(0xac));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(knob.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_c504)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void UIMenu::setCurrentItem(UIMenu* menu, UIObject* item, int replaceThread)` (0x39ad38).
///
/// Makes `item` the menu's current item (`+0xa0`; `+0xa4 = 0xff`). When the
/// old item is flagged (`+0x14` bit 4) its event thread is started and two
/// virtual calls (leave) are made; then the new item gets the virtual calls
/// (enter) and its event thread is started, or replaces the screen's thread.
pub fn ui_menu_set_current_item(c: &mut Cpu) {
    guest(c, |c| {
        'b_39adf0: {
            'b_39adec: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = 0xff;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.sb((c.g[S0][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                let t = c.g[A0][0] == 0u64;
                c.g[S2][0] = c.g[A2][0];
                if t {
                    break 'b_39adec;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
                        break 'b_39adf0;
                    } else {
                        'b_39adb0: {
                            c.cycles += 6;
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x0039ff78,
                                f: ee::s13::UIObject_GetEventThread,
                            }
                            .call(c, 0x0039ad84)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V0][0] = 0xffff;
                            if c.g[A1][0] == c.g[V0][0] {
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                break 'b_39adb0;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                if c.g[A0][0] == 0u64 {
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                    break 'b_39adb0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A2][0] = 0x1;
                                    Callee {
                                        addr: 0x0039c870,
                                        f: ee::s13::UIScreen_StartThread,
                                    }
                                    .call(c, 0x0039adac)?;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                    break 'b_39adb0;
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0039adc8)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[A2][0] = 0x2;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0039adec)?;
                        break 'b_39adec;
                    }
                }
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[S1][0]);
            break 'b_39adf0;
        }
        'b_39ae80: {
            c.cycles += 23;
            c.g[A2][0] = 0x1;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0039ae14)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0039ae30)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x0039ff78,
                f: ee::s13::UIObject_GetEventThread,
            }
            .call(c, 0x0039ae3c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            if c.g[A1][0] == c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_39ae80;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[A0][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_39ae80;
                } else {
                    'b_39ae74: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;

                        if t {
                            break 'b_39ae74;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x0039c870,
                                f: ee::s13::UIScreen_StartThread,
                            }
                            .call(c, 0x0039ae6c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_39ae80;
                            } else {
                                break 'b_39ae74;
                            }
                        }
                    }
                    c.cycles += 3;
                    Callee {
                        addr: 0x0039c820,
                        f: ee::s13::UIScreen_ReplaceThread,
                    }
                    .call(c, 0x0039ae7c)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_39ae80;
                }
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
    })
}

pub(crate) const PAIR_GET_STATE_DATA: Callee = Callee {
    addr: 0x0039_ff50,
    f: ee::s13::f_0039ff50,
};

pub(crate) const PAIR_GET_EVENT_THREAD: Callee = Callee {
    addr: 0x0039_ff78,
    f: ee::s13::f_0039ff78,
};

pub(crate) const PAIR_START_THREAD: Callee = Callee {
    addr: 0x0039_c870,
    f: ee::s13::f_0039c870,
};

pub(crate) const PAIR_GET_USER_DATA: Callee = Callee {
    addr: 0x003a_0318,
    f: ee::s13::f_003a0318,
};

/// `in->vt[off](in + delta, a1 = pair)` on the input in s0 (a1 set by the
/// caller); `ret` is the return address of the call.
pub(crate) fn pair_input_vcall(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(off));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(off + 4));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// The focus-controller object (`pair->screen (+0x5c) + 0xd0`) virtual call
/// `vt[off](obj + delta, pair, a2)` with the generated's register order:
/// `a2` is set by the caller; `a1 = pair` (s1) stays.
pub(crate) fn pair_focus_vcall(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// A direction case of `UIPair_HandleInput` (previous 5 / next 6): notify the
/// focus controller (`a2 = dir`), move the focus (`a2 = dir - 5`), play the
/// keyframer event (`a1 = event slot`) if the move changed focus (0x100),
/// then fetch the move's sound (`GetUserData`, 1 if bit 0, 0 if bit 4) and
/// tell the sound listener (`s2`) about it. Leaves `v0 = 1`.
pub(crate) fn pair_direction_case(c: &mut Cpu, dir: u64, rets: [u32; 5]) -> Flow {
    c.cycles += 21;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
    c.g[A2][0] = dir;
    pair_focus_vcall(c, 0x98, rets[0])?;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = dir - 5;
    // (the original loads a3 from v0 here; `pair_focus_vcall` reads v1)
    c.g[V1][0] = c.g[V0][0];
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, rets[1])?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] & 0x100;
    let unchanged = c.g[V0][0] == 0;
    c.g[A1][0] = dir - 3;
    let mut skip_plus1 = false;
    if !unchanged {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        PAIR_GET_EVENT_THREAD.call(c, rets[2])?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = 0xffff;
        let no_event = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0] & 0x1;
        if no_event {
            skip_plus1 = true;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
            c.g[A2][0] = 0x1;
            PAIR_START_THREAD.call(c, rets[3])?;
        }
    }
    if !skip_plus1 {
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0] & 0x1;
    }
    // Which sound: bit 0 -> 1, else bit 4 -> 0, else none.
    c.cycles += 2;
    let bit0_clear = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S1][0];
    let mut fetch = false;
    if !bit0_clear {
        c.cycles += 2;
        c.g[A1][0] = 0x1;
        fetch = true;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[S0][0] & 0x10;
        let bit4_clear = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !bit4_clear {
            c.cycles += 1;
            c.g[A1][0] = 0;
            fetch = true;
        }
    }
    if fetch {
        c.cycles += 3;
        PAIR_GET_USER_DATA.call(c, 0x0039_c074)?;
        c.g[S3][0] = c.g[V0][0];
    }
    // The listener's vt[2] (`+0x10`) with the sound.
    c.cycles += 2;
    let no_listener = c.g[S2][0] == 0;
    c.g[A1][0] = c.g[S3][0];
    if no_listener {
        c.cycles += 1;
        c.g[V0][0] = 0x1;
    } else {
        c.cycles += 7;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, rets[4])?;
        c.g[V0][0] = 0x1;
    }
    Ok(())
}

/// `bool UIPair_HandleInput(UIPair* this, const UIInput* in)` (0x39bed8)
/// (reference: `cUIPair::onInput`).
///
/// A pair of widgets with one focus. Previous (input 5) and next (input 6)
/// move the focus ([`pair_direction_case`]); input 4 only notifies the focus
/// controller; anything else goes to the child widget (`+0x7c`), if any.
/// Returns 1 when the pair handled the input.
pub fn ui_pair_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
        frame.save_ra(c, 0);
        c.g[S3][0] = 0;
        PAIR_GET_STATE_DATA.call(c, 0x0039_bf04)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        if c.g[A1][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0039_bf20)?;
        let is_prev = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[S1][0];
        let handled_dir = if is_prev {
            pair_direction_case(
                c,
                5,
                [
                    0x0039_bf48,
                    0x0039_bf6c,
                    0x0039_bf84,
                    0x0039_bfa0,
                    0x0039_c094,
                ],
            )?;
            true
        } else {
            c.cycles += 7;
            pair_input_vcall(c, 0x30, 0x0039_bfc8)?;
            let is_next = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if is_next {
                pair_direction_case(
                    c,
                    6,
                    [
                        0x0039_bff0,
                        0x0039_c014,
                        0x0039_c02c,
                        0x0039_c048,
                        0x0039_c094,
                    ],
                )?;
                true
            } else {
                false
            }
        };
        if handled_dir {
            c.cycles += 1;
        } else {
            c.cycles += 7;
            pair_input_vcall(c, 0x20, 0x0039_c0b0)?;
            let is_focus = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if is_focus {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                c.g[A2][0] = 0x4;
                pair_focus_vcall(c, 0x98, 0x0039_c0d8)?;
                c.g[V0][0] = 0x1;
                c.cycles += 1;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                let no_child = c.g[A2][0] == 0;
                c.g[A1][0] = c.g[S0][0];
                if no_child {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.cycles += 1;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0039_c100)?;
                    frame.restore(c, 16, 0x40);
                    return pair_epilogue(c, frame);
                }
            }
        }
        frame.restore(c, 16, 0x40);
        pair_epilogue(c, frame)
    })
}

/// The shared epilogue of `ui_pair_handle_input` (after `lq s0`).
pub(crate) fn pair_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void UIPair::Load(UIPair* this)` (0x39bab0): just `UIObject_Load(this)`.
pub fn ui_pair_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039bac0);
        call_ptr(c, 0x0039fe00, 0x0039bac0)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `bool UIMenu::onInputChangeItem(UIMenu* this, u8 old)` (0x39ae98): when the
/// current index (`+0x95`) differs from `old`, scrolls the visible window
/// (`+0x97` visible rows, `+0x98` first row) so the current entry shows, calls
/// the owner's virtual hooks, makes the entry the current item
/// (`UIMenu_SetCurrentItem`) and returns true. Returns false when `old` is
/// already current.
pub fn ui_menu_select_index(c: &mut Cpu) {
    if c.pc != 0x0039ae98 {
        return ee::s13::f_0039ae98__generated(c);
    }
    guest(c, |c| {
        'b_39afec: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A1][0] = c.g[A1][0] & 0xff;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 0u64;
            if t {
                break 'b_39afec;
            } else {
                'b_39afa4: {
                    'b_39af30: {
                        c.cycles += 5;
                        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        if (c.g[S1][0] as i64) >= 0 {
                            c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x97));
                            break 'b_39af30;
                        } else {
                            c.cycles += 24;
                            c.sb((c.g[S0][0] as u32).wrapping_add(0x98), c.g[V1][0]);
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[A2][0] = 0x2;
                            c.g[A3][0] = c.g[V1][0] & 0xff;
                            c.g[S1][0] = 0u64;
                            c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0039af00);
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0039af00)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                            c.g[A2][0] = 0x4;
                            c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0039af28);
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0039af28)?;
                            let t = true;
                            c.g[A1][0] = c.g[S1][0];
                            if t {
                                break 'b_39afa4;
                            } else {
                                break 'b_39af30;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        break 'b_39afa4;
                    } else {
                        c.cycles += 26;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x98), c.g[V0][0]);
                        c.g[A2][0] = 0x3;
                        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x97));
                        c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                        c.g[A3][0] = c.g[V0][0] & 0xff;
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0039af78);
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0039af78)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                        c.g[A2][0] = 0x4;
                        c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0039afa0);
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0039afa0)?;
                        c.g[A1][0] = c.g[S1][0];
                        break 'b_39afa4;
                    }
                }
                c.cycles += 18;
                c.g[RA][0] = sx(0x0039afac);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                Callee {
                    addr: 0x00397870,
                    f: ee::s13::UIList_GetAt,
                }
                .call(c, 0x0039afac)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0039afc0);
                c.g[A2][0] = c.g[A2][0] & 0x1;
                Callee {
                    addr: 0x0039ad38,
                    f: ee::s13::UIMenu_SetCurrentItem,
                }
                .call(c, 0x0039afc0)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x98));
                c.g[A2][0] = 0x5;
                c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0039afe8);
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0039afe8)?;
                c.g[V0][0] = 0x1;
                break 'b_39afec;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `UIMenu::getPrev`-style search (0x39b878), the backward twin of
/// `ui_menu_find_next_selectable`: from the given index (or `count - 1` when it
/// is past the end) back towards 0, the first selectable entry (flags bit 5
/// clear, or the owner's virtual hook when `+0x90` bit 2 is set). `v0` gets
/// its index, or 0xff when none qualifies.
pub fn ui_menu_find_last_selectable(c: &mut Cpu) {
    if c.pc != 0x0039b878 {
        return ee::s13::f_0039b878__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0039b878;
        loop {
            match pc {
                0x0039b878 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[A1][0] = c.g[A1][0] & 0xff;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    c.g[V0][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = c.g[A1][0];
                    if t {
                        pc = 0x0039b8b4;
                        continue;
                    }
                    pc = 0x0039b8a8;
                    continue;
                }
                0x0039b8a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.g[A1][0] = c.g[V0][0] & 0xff;
                    c.g[S0][0] = c.g[A1][0];
                    pc = 0x0039b8b4;
                    continue;
                }
                0x0039b8b4 => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 16);
                    if t {
                        pc = 0x0039b93c;
                        continue;
                    }
                    pc = 0x0039b8bc;
                    continue;
                }
                0x0039b8bc => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S3][0] = 0xffffffffffff0000;
                    pc = 0x0039b8c8;
                    continue;
                }
                0x0039b8c8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[RA][0] = sx(0x0039b8d4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00397870,
                        f: ee::s13::UIList_GetAt,
                    }
                    .call(c, 0x0039b8d4)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0039b91c;
                        continue;
                    }
                    pc = 0x0039b8e8;
                    continue;
                }
                0x0039b8e8 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0039b90c);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0039b90c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        pc = 0x0039b930;
                        continue;
                    }
                    pc = 0x0039b914;
                    continue;
                }
                0x0039b914 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0] & 0xff;
                    if t {
                        pc = 0x0039b940;
                        continue;
                    }
                    pc = 0x0039b91c;
                    continue;
                }
                0x0039b91c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x0039b914) {
                            return Ok(());
                        }
                        pc = 0x0039b914;
                        continue;
                    }
                    pc = 0x0039b930;
                    continue;
                }
                0x0039b930 => {
                    c.cycles += 3;
                    c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    let t = (c.g[S0][0] as i64) >= 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x0039b8c8) {
                            return Ok(());
                        }
                        pc = 0x0039b8c8;
                        continue;
                    }
                    pc = 0x0039b93c;
                    continue;
                }
                0x0039b93c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xff;
                    pc = 0x0039b940;
                    continue;
                }
                0x0039b940 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039b878, 0x0039b95c);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::tests::oc784_install_fakes;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::virtual_setup;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::be::beintplayer::tests::r_u32;
    pub(crate) use crate::game::camera::cameracontroller::tests::fake_child;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::w8;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::CHILD;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;

    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_5;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fereal::tests::fake_slot;
    pub(crate) use crate::game::fe::fereal::tests::SELF2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_2;

    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_6;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_4;

    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn ui_pair_accept_matches() {
        check(
            ee::s13::f_0039bd38__generated,
            ui_pair_accept,
            SEEDS,
            &|c, seed| {
                virtual_setup(c, seed, 0x39bd_38, 0x10);
            },
        );
    }

    #[test]
    fn cui_progress_load_matches() {
        check(
            ee::s13::f_0039c2c8__generated,
            cui_progress_load,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39c2_c8, 0);
                let obj = DATA + 0x400;
                let desc = DATA + 0x600;
                fill(c, &mut r, obj, 0x40);
                fill(c, &mut r, desc, 0x20);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(desc);
                c.bus.write32(obj + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x48);
            },
        );
    }

    #[test]
    fn uimenu_reset_scroll_matches() {
        check(
            ee::s13::f_0039b760__generated,
            uimenu_reset_scroll,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_b760, 0);
                let m = DATA + 0x1000;
                c.bus.write32(m + 0x90, r.next_u32());
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (r.next_u32() % 0x100) as u64;
            },
        );
    }

    #[test]
    fn ui_pair_refresh_text_matches() {
        check(
            ssx3_ee::s13::f_0039be10__generated,
            ui_pair_refresh_text,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x39be10, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x80, 0);
                let first = if seed % 3 == 0 { 0 } else { OBJ2 };
                let second = if seed % 4 == 0 { 0 } else { OBJ3 };
                w(c, OBJ + 0x78, first);
                w(c, OBJ + 0x7c, second);
                w(c, OBJ2 + 0x8, VT);
                w(c, OBJ3 + 0x8, VT);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ui_pair_free_children_matches() {
        check(
            ee::s13::f_0039be70__generated,
            ui_pair_free_children,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_be70, 2);
                c.g[A0][0] = sx(DATA);
                let a = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
                let b = if seed % 4 == 0 { 0 } else { DATA + 0x300 };
                if a != 0 {
                    fake_slot(c, &mut r, a, 0x8, 0x8);
                }
                if b != 0 {
                    fake_slot(c, &mut r, b, 0x8, 0x8);
                }
                c.bus.write32(DATA + 0x78, a);
                c.bus.write32(DATA + 0x7c, b);
            },
        );
    }

    #[test]
    fn ui_pair_update_anim_matches() {
        check(
            ee::s13::f_0039c1c8__generated,
            ui_pair_update_anim,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x39c1c8, 4);
                be_library_2(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = seed as u64;
                if seed % 2 == 0 {
                    w_2(c, this + 0x78, 0);
                } else {
                    // the pair child's vtable is the word after its next links
                    let a = DATA + 0x400;
                    w_2(c, this + 0x78, a);
                    w_2(c, a + 8, DATA + 0x2000);
                    slot_2(c, DATA + 0x2000, 0x98, 0, FAKE);
                }
                if seed % 3 == 0 {
                    w_2(c, this + 0x7c, 0);
                } else {
                    let b = DATA + 0x500;
                    w_2(c, this + 0x7c, b);
                    w_2(c, b + 8, DATA + 0x2000);
                    slot_2(c, DATA + 0x2000, 0x98, 0, FAKE);
                }
            },
        );
    }

    #[test]
    fn uipair_v04_matches() {
        check(
            ee::s13::f_0039bad0__generated,
            uipair_v04,
            SEEDS_3,
            &|c, seed| {
                base(c, seed, 0xbad0, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 3) as u64;
                c.bus.write32(s + 0x14, r_u32(seed, 1));
                let a = DATA + 0x200;
                let b = DATA + 0x240;
                c.bus.write32(s + 0x78, a);
                c.bus.write32(s + 0x7c, if seed % 3 == 0 { 0 } else { b });
                let vta = DATA + 0x400;
                slot_3(c, vta, 0x20, 0, FAKE);
                c.bus.write32(a + 8, vta);
                let vtb = DATA + 0x440;
                slot_3(c, vtb, 0x20, 0, FAKE + 0x10);
                c.bus.write32(b + 8, vtb);
            },
        );
    }

    #[test]
    fn c_ui_progress_dtor_matches() {
        check(
            ssx3_ee::s13::f_0039c240__generated,
            c_ui_progress_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39c240, 3);
                be_library_3(c);
                world(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                if seed % 3 != 0 {
                    fake_child(c, obj, 0x74, 8, 0);
                }
                if seed % 4 != 0 {
                    fake_child(c, obj, 0x78, 8, 1);
                }
                let _ = r.next_u32();
            },
        );
    }

    // ------------------------------------------------------------------
    // UI objects.

    #[test]
    fn ui_pair_set_visible_matches() {
        check(
            ssx3_ee::s13::f_0039bb50__generated,
            ui_pair_set_visible,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39bb50, 3);
                be_library_3(c);
                world(c);
                let pair = DATA + 0x100;
                c.g[A0][0] = pair as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w_3(c, pair + 0x14, if seed % 2 == 0 { 0x40 } else { 0 });
                w_3(c, pair + 0x78, 0);
                w_3(c, pair + 0x7c, 0);
                if seed % 3 != 0 {
                    let a = DATA + 0x300;
                    let vt = DATA + 0x500;
                    w_3(c, pair + 0x78, a);
                    w_3(c, a + 8, vt);
                    slot_4(c, vt, 0x48, 0, FAKE);
                    slot_4(c, vt, 0x4c, 0, FAKE + 0x10);
                }
                if seed % 4 != 0 {
                    let b = DATA + 0x400;
                    let vt = DATA + 0x600;
                    w_3(c, pair + 0x7c, b);
                    w_3(c, b + 8, vt);
                    slot_4(c, vt, 0x48, 0, FAKE + 0x20);
                    slot_4(c, vt, 0x4c, 0, FAKE + 0x30);
                }
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_ui_progress_update_anim_matches() {
        check(
            ee::s13::f_0039c398__generated,
            c_ui_progress_update_anim,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x39c3, 0);
                env(c);
                let this = DATA + 0x400;
                let vt = DATA + 0x1000;
                slot_5(c, vt, 0x98, EMPTY);
                w32(c, this + 0xc, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                w8(c, this + 0x10, seed % 8);
                w32(
                    c,
                    this + 0x78,
                    if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
                );
                w32(
                    c,
                    this + 0x74,
                    if seed % 5 == 0 { 0 } else { DATA + 0x3100 },
                );
                w32(c, DATA + 0x3000 + 8, vt);
                w32(c, DATA + 0x3100 + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 0x1000) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI render / texture manager.

    #[test]
    fn ui_pair_render_matches() {
        check(
            ee::s13::f_0039c130__generated,
            ui_pair_render,
            100,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x39c130, 4);
                let obj = DATA + 0x1000;
                let a = DATA + 0x2000;
                let b = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                w_4(c, obj + 0x78, if seed % 4 == 0 { 0 } else { a });
                w_4(c, obj + 0x7c, if seed % 3 == 0 { 0 } else { b });
                w_4(c, obj + 0x44, r.edge_f32());
                w_4(c, obj + 0x48, r.edge_f32());
                w_4(c, a + 0x8, vt);
                w_4(c, b + 0x8, vt);
                slot_6(c, vt, 0x88, 8, FAKE);
            },
        );
    }

    #[test]
    fn cui_progress_render_matches() {
        check(
            ee::s13::f_0039c300__generated,
            cui_progress_render,
            100,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x39c300, 4);
                let obj = DATA + 0x1000;
                let a = DATA + 0x2000;
                let b = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                w_4(c, obj + 0x74, if seed % 4 == 0 { 0 } else { a });
                w_4(c, obj + 0x78, if seed % 3 == 0 { 0 } else { b });
                w_4(c, obj + 0x44, r.edge_f32());
                w_4(c, obj + 0x48, r.edge_f32());
                w_4(c, a + 0x8, vt);
                w_4(c, b + 0x8, vt);
                slot_6(c, vt, 0x88, 8, FAKE);
            },
        );
    }

    #[test]
    fn ui_pair_v03_matches() {
        check(
            ee::s13::f_0039bd68__generated,
            ui_pair_v03,
            10,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x39bd68, 3);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2_2 as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x14]);
                if seed % 2 == 1 {
                    // UIList_First on a zeroed list returns 0: take the child
                    // path instead of the predicate-hit one.
                    w_5(c, vt + 0x10, 0);
                    w_5(c, vt + 0x14, 0x0039_7840);
                }
                w_5(c, OBJ2_2, vt);
                w_5(c, OBJ_2 + 0x78, if seed % 4 == 0 { 0 } else { CHILD });
                w_5(c, OBJ_2 + 0x7c, if seed % 3 == 0 { 0 } else { SELF2 });
                let vt9 = vtable(c, &mut r, &[0x18, 0x1c]);
                w_5(c, CHILD + 8, vt9);
                let vt10 = vtable(c, &mut r, &[0x18, 0x1c]);
                w_5(c, SELF2 + 8, vt10);
            },
        );
    }

    #[test]
    fn ui_menu_render_matches() {
        strict(
            ee::s13::f_0039b6a0__generated,
            ui_menu_render,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39b6, 2);
                let this = DATA;
                c.bus.write32(this + 0x9c, 0);
                let list = this + 0x74;
                let (node0, node1, sent, obj) = (
                    DATA + 0x400u32,
                    DATA + 0x500u32,
                    DATA + 0x600u32,
                    DATA + 0x700u32,
                );
                if seed % 4 == 0 {
                    c.bus.write32(list + 4, list);
                } else {
                    c.bus.write32(list + 4, node0);
                    c.bus.write32(node0 + 4, node1);
                    c.bus.write32(node0 + 0x14, 0x40);
                    c.bus.write32(node0 + 8, obj);
                    c.bus.write32(node1 + 4, sent);
                    c.bus.write32(node1 + 0x14, 0);
                    c.bus.write32(sent + 4, sent);
                    c.bus.write16(obj + 0x88, 0);
                    c.bus.write32(obj + 0x8c, FAKE);
                }
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ui_menu_find_first_selectable_matches() {
        check(
            ee::s13::f_0039b7b0__generated,
            ui_menu_find_first_selectable,
            60,
            &|c, seed| {
                let _r = base_4(c, seed, 0x39b7);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 6) as u64;
                c.bus.write8(DATA + 0x96, (seed % 5) as u8);
                c.bus.write32(DATA + 0x74, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x90, if seed % 3 == 0 { 4 } else { 0 });
                c.bus.write32(DATA + 0x5c, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0xd0, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 8, DATA + 0x2200);
                c.bus.write16(DATA + 0x2200 + 0xa0, 0);
                c.bus.write32(DATA + 0x2200 + 0xa4, FAKE);
                c.bus.write32(DATA + 0x1000 + 0x14, (seed % 2) as u32);
            },
        );
    }

    pub(crate) const PAIR: u32 = DATA + 0x1_0000;

    pub(crate) const PAIR_A: u32 = DATA + 0x1_1000;

    pub(crate) const PAIR_B: u32 = DATA + 0x1_2000;

    pub(crate) const PAIR_SCREEN: u32 = DATA + 0x1_3000;

    #[test]
    fn ui_pair_set_focus_matches() {
        check_all(
            ee::s13::f_0039bbd8__generated,
            ui_pair_set_focus,
            30,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x39bb_d8, 10);
                c.g[A0][0] = sx(PAIR);
                c.g[A1][0] = sx(seed % 2);
                c.bus.write32(PAIR + 0x14, r.next_u32());
                if seed % 2 == 0 {
                    c.bus.write32(PAIR + 0x78, PAIR_A);
                    let vt1 = vtable(c, &mut r, &[0x38, 0x3c]);
                    c.bus.write32(PAIR_A + 8, vt1);
                }
                if seed % 3 != 0 {
                    c.bus.write32(PAIR + 0x7c, PAIR_B);
                    let vt2 = vtable(c, &mut r, &[0x38, 0x3c]);
                    c.bus.write32(PAIR_B + 8, vt2);
                    if seed % 2 == 0 {
                        c.bus.write32(PAIR_B + 0x5c, PAIR_SCREEN);
                        c.bus.write32(PAIR + 0x5c, PAIR_SCREEN);
                        c.bus.write32(PAIR_SCREEN + 0xd0, PAIR_A);
                        let vt3 = vtable(c, &mut r, &[0x98, 0x9c]);
                        c.bus.write32(PAIR_A + 8, vt3);
                    }
                }
            },
        );
    }

    pub(crate) use crate::game::ealib::seg_2dcae8::tests::OC445_GET_AT;
    pub(crate) use crate::game::ui::uiengine::tests::OC445_OBJ2;
    pub(crate) use crate::game::ui::uitext::tests::oc445_gate;
    pub(crate) use crate::game::ui::uitext::tests::oc445_vtable;

    pub(crate) use crate::game::ui::uitext::tests::OC445_OBJ;

    pub(crate) use crate::game::ui::uitext::tests::OC445_VT;

    pub(crate) const OC445_MGR: u32 = DATA + 0x3000;

    pub(crate) const OC445_ITEM: u32 = DATA + 0x5000;

    pub(crate) const OC445_VT_ITEM: u32 = DATA + 0x5800;

    pub(crate) const OC445_CUR: u32 = DATA + 0x6000;

    /// `UIMenu_SetSelection`: non-scrolling menus (attribute or manager
    /// flag clear), selections inside the window, below it and above it.
    #[test]
    fn ui_menu_set_selection_matches() {
        fakes::covered(
            ee::s13::f_0039b960__generated,
            ui_menu_set_selection,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39b960, 8);
                oc445_gate(c);
                oc445_vtable(c, &mut r, OC445_VT, &[0x98, 0xa0]);
                oc445_vtable(c, &mut r, OC445_VT_ITEM, &[0x38]);
                c.bus.write32(OC445_OBJ2 + 0xd0, OC445_MGR);
                c.bus.write32(OC445_MGR + 8, OC445_VT);
                // Manager flag word bit 5 (the menu scrolls).
                let mgr_flag = if seed % 7 == 0 { 0 } else { 1 << 5 };
                c.bus
                    .write32(OC445_MGR + 0x1c, mgr_flag | (r.next_u32() & !(1 << 5)));
                // Menu attribute bit 4.
                let attr = if seed % 5 == 0 { 0 } else { 1 << 4 };
                c.bus
                    .write32(OC445_OBJ + 0x14, attr | (r.next_u32() & !(1 << 4)));
                c.bus.write32(OC445_OBJ + 0x5c, OC445_OBJ2);
                c.bus.write8(OC445_OBJ + 0x95, r.next_u32() as u8);
                c.bus.write8(OC445_OBJ + 0x97, 1 + (r.next_u32() % 5) as u8);
                c.bus.write8(OC445_OBJ + 0x98, (r.next_u32() % 6) as u8);
                // The current item (0 or a real object).
                let cur = if seed % 3 == 0 { OC445_CUR } else { 0 };
                c.bus.write32(OC445_OBJ + 0xa0, cur);
                c.bus.write32(OC445_CUR + 8, OC445_VT_ITEM);
                c.bus.write32(OC445_CUR + 0x14, r.next_u32());
                c.bus.write32(OC445_ITEM + 8, OC445_VT_ITEM);
                c.bus.write32(OC445_GET_AT, OC445_ITEM);
                c.g[A0][0] = sx(OC445_OBJ);
                let idx = r.next_u32() % 14;
                c.g[A1][0] =
                    (r.next_u32() as u64) << 32 | (r.next_u32() as u64 & !0xff) | idx as u64;
                c.pc = 0x0039_b960;
            },
        );
    }

    use crate::difftest::fakes::covered;

    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;

    /// The menu object (its cUIThing mAttr at 0x14, mAttrMenu at 0x90).
    pub(crate) const SELF: u32 = DATA + 0x1000;

    /// The menu's screen; its owner at +0xd0 has the focus handlers.
    pub(crate) const SCREEN: u32 = DATA + 0x2000;

    pub(crate) const OWNER: u32 = DATA + 0x3000;

    pub(crate) const VT_OWNER: u32 = DATA + 0x3800;

    /// The input event (its vtable's slots 2 and 4-8 are the button tests).
    pub(crate) const INPUT: u32 = DATA + 0x4000;

    pub(crate) const VT_INPUT: u32 = DATA + 0x4800;

    /// The selected item (its vtable's slot at 0x78 is `onInput`).
    pub(crate) const CUR: u32 = DATA + 0x5000;

    pub(crate) const VT_CUR: u32 = DATA + 0x5800;

    /// The item `UIList_GetAt` returns.
    pub(crate) const ITEM: u32 = DATA + 0x6000;

    pub(crate) const VT_ITEM: u32 = DATA + 0x6800;

    /// The UI sound listener (`gUISoundNull`, a vtable pointer in GP+0x2968).
    pub(crate) const VT_SOUND: u32 = DATA + 0x7000;

    pub(crate) const SOUND_NULL: u32 = GAME_GP + 0x2968;

    /// A real leaf that returns 0 (a vtable slot that answers "not used").
    pub(crate) const ZERO_LEAF: u32 = 0x0014_06c0;

    /// One vtable slot at `off`: a random delta, and either the zero leaf or
    /// a fake method (the hashed result of the fake call).
    pub(crate) fn onin_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, zero: bool, n: u32) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + off, delta as u16);
        let f = if zero {
            ZERO_LEAF
        } else {
            fakes::FAKE + 0x10 * n
        };
        c.bus.write32(vt + off + 4, f);
    }

    pub(crate) fn onin_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x39_b000, 6);
        oc445_gate(c);
        // Forced results of the first fake calls (0: hashed).
        let forced = [0u32, 1, 0x101, 0x11, 0x10, 0x100][seed as usize % 6];
        for n in 0..8u32 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, forced);
        }
        // The menu: hidden on seed % 7 == 0, inactive (mAttr bit 4 clear) on
        // seed % 5 == 0; no selected item on seed % 6 == 0.
        let mut attr = r.next_u32() & !0x30;
        if seed % 7 == 0 {
            attr |= 1 << 5;
        }
        if seed % 5 != 0 {
            attr |= 1 << 4;
        }
        c.bus.write32(SELF + ONIN_ATTR, attr);
        let menu_word = if seed % 4 == 0 {
            r.next_u32() & !8
        } else {
            r.next_u32() | 8
        };
        c.bus.write32(SELF + ONIN_ATTR_MENU, menu_word);
        for k in 0..4 {
            c.bus.write8(SELF + ONIN_INPUT_MAP + k, r.next_u32() as u8);
        }
        c.bus.write8(SELF + ONIN_SCROLL_CUR, r.next_u32() as u8);
        c.bus.write8(SELF + 0x97, 1 + (r.next_u32() % 5) as u8);
        c.bus
            .write8(SELF + ONIN_SCROLL_OFFSET, (r.next_u32() % 6) as u8);
        c.bus
            .write32(SELF + ONIN_SCREEN, if seed % 3 == 0 { 0 } else { SCREEN });
        c.bus
            .write32(SELF + ONIN_SELECTED, if seed % 6 == 0 { 0 } else { CUR });
        // The screen's owner and its focus handlers (slots 19 and 20).
        c.bus.write32(SCREEN + 0xd0, OWNER);
        c.bus.write32(OWNER + 8, VT_OWNER);
        onin_slot(c, &mut r, VT_OWNER, 0x98, seed % 2 == 0, 1);
        onin_slot(c, &mut r, VT_OWNER, 0xa0, seed % 3 == 0, 2);
        // The input: slot 2 (button) and slots 4-8 (events), zero or fake
        // by bit i of the seed.
        c.bus.write32(INPUT + 8, VT_INPUT);
        for (i, off) in [0x10u32, 0x20, 0x28, 0x30, 0x38, 0x40]
            .into_iter()
            .enumerate()
        {
            let zero = (seed >> i) & 1 == 0;
            onin_slot(c, &mut r, VT_INPUT, off, zero, 3 + i as u32);
        }
        // The selected item: onInput returns 0 or 1 (fake).
        c.bus.write32(CUR + 8, VT_CUR);
        c.bus.write32(CUR + ONIN_ATTR, r.next_u32());
        onin_slot(c, &mut r, VT_CUR, 0x78, seed % 2 == 0, 9);
        // What UIList_GetAt returns (the oc445 fake), and its vtable.
        c.bus.write32(ITEM + 8, VT_ITEM);
        c.bus.write32(OC445_GET_AT, ITEM);
        // The sound listener (gUISoundNull's vtable: slot 2 plays).
        c.bus.write32(SOUND_NULL, VT_SOUND);
        onin_slot(c, &mut r, VT_SOUND, 0x10, seed % 4 == 0, 10);
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = sx(INPUT);
        c.pc = 0x0039_b000;
    }

    #[test]
    fn ui_menu_on_input_matches() {
        check(
            ee::s13::f_0039b000__generated,
            ui_menu_on_input,
            300,
            &onin_setup,
        );
    }

    /// SetValue with and without background and knob, both bar directions,
    /// zero and non-zero numValues; the widget calls are fakes (some unwind).
    #[test]
    fn cuiprogress_set_value_matches() {
        covered(
            ee::s13::f_0039c428__generated,
            cuiprogress_set_value,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39c428, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 50) as u64;
                let bg = if seed % 5 == 0 { 0 } else { DATA + 0x2000 };
                let knob = if seed % 7 == 0 { 0 } else { DATA + 0x3000 };
                c.bus.write32(this + CUIPROGRESS_ATTR, r.next_u32());
                c.bus.write32(this + CUIPROGRESS_BACKGROUND, bg);
                c.bus.write32(this + CUIPROGRESS_KNOB, knob);
                c.bus.write32(
                    this + CUIPROGRESS_NUM_VALUES,
                    if seed % 3 == 0 { 0 } else { r.next_u32() % 40 },
                );
                let vt = fakes::vtable(c, &mut r, &[0xa0, 0xa8]);
                c.bus.write32(DATA + 0x2008, vt);
                c.bus.write32(DATA + 0x3008, vt);
                // The zero vector the size buffer is initialised from.
                let zero = 0x0050_0000u32 - 0xf28;
                for i in 0..3 {
                    c.bus.write32(zero + 4 * i, r.f32(100.0));
                }
            },
        );
    }

    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc784_arm;

    pub(crate) use crate::game::fe::ovstatehud::tests::oc754_arm;
    pub(crate) use crate::game::fe::ovstatehud::tests::oc754_force;
    pub(crate) use crate::game::fe::ovstatehud::tests::oc754_force_zero;
    pub(crate) use crate::game::fe::ovstatehud::tests::oc754_mail;
    pub(crate) use crate::game::fe::ovstatehud::tests::OC754_CTRL;
    pub(crate) use crate::game::fe::ovstatehud::tests::OC754_MAGIC;
    use std::sync::atomic::AtomicUsize;

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc754_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc754_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc754_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC754_CTRL) != OC754_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc754_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc754_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc754_fakes!(
        (
            oc754_fk_4165a8,
            OC754_PREV_4165A8,
            ee::s15::OVERRIDE_004165A8,
            ee::s15::f_004165a8__generated,
            0x0041_65a8
        ),
        (
            oc754_fk_4166f4,
            OC754_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc754_fk_417828,
            OC754_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc754_fk_28bce8,
            OC754_PREV_28BCE8,
            ee::s07::OVERRIDE_0028BCE8,
            ee::s07::f_0028bce8__generated,
            0x0028_bce8
        ),
        (
            oc754_fk_28bd10,
            OC754_PREV_28BD10,
            ee::s07::OVERRIDE_0028BD10,
            ee::s07::f_0028bd10__generated,
            0x0028_bd10
        ),
        (
            oc754_fk_28b180,
            OC754_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc754_fk_39ff78,
            OC754_PREV_39FF78,
            ee::s13::OVERRIDE_0039FF78,
            ee::s13::f_0039ff78__generated,
            0x0039_ff78
        ),
        (
            oc754_fk_39c870,
            OC754_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc754_fk_39c7c0,
            OC754_PREV_39C7C0,
            ee::s13::OVERRIDE_0039C7C0,
            ee::s13::f_0039c7c0__generated,
            0x0039_c7c0
        ),
        (
            oc754_fk_2c2508,
            OC754_PREV_2C2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c_2508
        ),
        (
            oc754_fk_2b40f0,
            OC754_PREV_2B40F0,
            ee::s08::OVERRIDE_002B40F0,
            ee::s08::f_002b40f0__generated,
            0x002b_40f0
        ),
        (
            oc754_fk_317670,
            OC754_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc754_fk_317e30,
            OC754_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc754_fk_11b698,
            OC754_PREV_11B698,
            ee::s00::OVERRIDE_0011B698,
            ee::s00::f_0011b698__generated,
            0x0011_b698
        ),
        (
            oc754_fk_2dcf28,
            OC754_PREV_2DCF28,
            ee::s09::OVERRIDE_002DCF28,
            ee::s09::f_002dcf28__generated,
            0x002d_cf28
        ),
        (
            oc754_fk_2e8560,
            OC754_PREV_2E8560,
            ee::s09::OVERRIDE_002E8560,
            ee::s09::f_002e8560__generated,
            0x002e_8560
        ),
        (
            oc754_fk_3e6448,
            OC754_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    pub(crate) const OC754_MENU: u32 = DATA + 0x1_0000;

    pub(crate) const OC754_OWNER: u32 = DATA + 0x2_0000;

    pub(crate) const OC754_HELPER: u32 = DATA + 0x3_0000;

    pub(crate) const OC754_CUR: u32 = DATA + 0x4_0000;

    pub(crate) const OC754_ITEM: u32 = DATA + 0x5_0000;

    pub(crate) const OC754_VT: u32 = DATA + 0x6_0000;

    /// `UIMenu_SetCurrentItem`: no/flagged/unflagged old item, event thread
    /// -1 or not, items with and without a thread object (`+0x5c`), start or
    /// replace the thread, and the unwinds of its calls.
    #[test]
    fn ui_menu_set_current_item_matches() {
        oc754_install_fakes();
        fakes::covered(
            ee::s13::f_0039ad38__generated,
            ui_menu_set_current_item,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754e, 10);
                oc754_arm(c);
                c.g[A0][0] = sx(OC754_MENU);
                c.g[A1][0] = sx(OC754_ITEM);
                c.g[A2][0] = ((seed / 32) % 2) as u64;
                let cur = if seed % 4 == 0 { 0 } else { OC754_CUR };
                c.bus.write32(OC754_MENU + 0xa0, cur);
                c.bus.write32(OC754_MENU + 0x5c, OC754_OWNER);
                c.bus.write32(OC754_OWNER + 0xd0, OC754_HELPER);
                c.bus.write32(OC754_HELPER + 8, OC754_VT);
                c.bus.write32(OC754_CUR + 8, OC754_VT);
                c.bus.write32(OC754_ITEM + 8, OC754_VT);
                for (slot, i) in [(0x38u32, 0u32), (0x98, 1)] {
                    c.bus
                        .write16(OC754_VT + slot, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(OC754_VT + slot + 4, fakes::FAKE + 0x10 * i);
                }
                let flag4 = if (seed / 4) % 2 == 0 { 0x10 } else { 0 };
                c.bus
                    .write32(OC754_CUR + 0x14, (r.next_u32() & !0x10) | flag4);
                c.bus.write32(
                    OC754_CUR + 0x5c,
                    if (seed / 8) % 2 == 0 { 0 } else { 0x1234 },
                );
                c.bus.write32(
                    OC754_ITEM + 0x5c,
                    if (seed / 16) % 2 == 0 { 0 } else { 0x5678 },
                );
                if seed % 3 == 0 {
                    oc754_force(c, 0x0039_ff78, 0xffff);
                }
                // ReplaceThread (real, unported) finds no primary thread to destroy.
                oc754_force_zero(c, 0x0039_c7c0);
            },
        );
    }

    pub(crate) const OC776_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC776_MAGIC: u32 = 0x0776_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc776_mail(addr: u32) -> u32 {
        OC776_CTRL + 0x1000 + (addr & 0xffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc776_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc776_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc776_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc776_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    /// Fake methods that return false (a real-looking 0) or the fixture's move result.
    pub(crate) const OC776_FALSE_FN: u32 = fakes::FAKE + 0x100;

    pub(crate) const OC776_MOVE_FN: u32 = fakes::FAKE + 0x110;

    pub(crate) fn oc776_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let move_result = c.bus.read32(OC776_CTRL + 0x10);
        fakes::fake_call(c, target);
        if c.pc == ra {
            if target == OC776_FALSE_FN {
                c.g[V0][0] = 0;
            } else if target == OC776_MOVE_FN {
                c.g[V0][0] = move_result as u64;
            }
        }
    }

    /// Plain gated fakes (logged calls) of the UI state/menu/text callees.
    macro_rules! oc776_plain_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC776_CTRL) != OC776_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc776_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc776_install_plain_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc776_plain_fakes!(
        (
            oc776_fk_14dc80,
            OC776_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc776_fk_14dd58,
            OC776_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc776_fk_14ad28,
            OC776_PREV_0014AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc776_fk_14bdb8,
            OC776_PREV_0014BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc776_fk_14d998,
            OC776_PREV_0014D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014_d998
        ),
        (
            oc776_fk_14d988,
            OC776_PREV_0014D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014_d988
        ),
        (
            oc776_fk_317d70,
            OC776_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc776_fk_2f1890,
            OC776_PREV_002F1890,
            ee::s09::OVERRIDE_002F1890,
            ee::s09::f_002f1890__generated,
            0x002f_1890
        ),
        (
            oc776_fk_2eca68,
            OC776_PREV_002ECA68,
            ee::s09::OVERRIDE_002ECA68,
            ee::s09::f_002eca68__generated,
            0x002e_ca68
        ),
        (
            oc776_fk_30d4b8,
            OC776_PREV_0030D4B8,
            ee::s10::OVERRIDE_0030D4B8,
            ee::s10::f_0030d4b8__generated,
            0x0030_d4b8
        ),
        (
            oc776_fk_30d840,
            OC776_PREV_0030D840,
            ee::s10::OVERRIDE_0030D840,
            ee::s10::f_0030d840__generated,
            0x0030_d840
        ),
        (
            oc776_fk_2f19f0,
            OC776_PREV_002F19F0,
            ee::s09::OVERRIDE_002F19F0,
            ee::s09::f_002f19f0__generated,
            0x002f_19f0
        ),
        (
            oc776_fk_19eba0,
            OC776_PREV_0019EBA0,
            ee::s03::OVERRIDE_0019EBA0,
            ee::s03::f_0019eba0__generated,
            0x0019_eba0
        ),
        (
            oc776_fk_39ff50,
            OC776_PREV_39FF50,
            ee::s13::OVERRIDE_0039FF50,
            ee::s13::f_0039ff50__generated,
            0x0039_ff50
        ),
        (
            oc776_fk_39ff78,
            OC776_PREV_39FF78,
            ee::s13::OVERRIDE_0039FF78,
            ee::s13::f_0039ff78__generated,
            0x0039_ff78
        ),
        (
            oc776_fk_39c870,
            OC776_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc776_fk_3a0318,
            OC776_PREV_3A0318,
            ee::s13::OVERRIDE_003A0318,
            ee::s13::f_003a0318__generated,
            0x003a_0318
        ),
        (
            oc776_fk_39b960,
            OC776_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc776_fk_39e948,
            OC776_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc776_fk_39e8b8,
            OC776_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc776_fk_3a1000,
            OC776_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
    );

    /// `UIPair_HandleInput`: input 5 (previous), 6 (next), 4 (focus only) and
    /// anything else (with and without a child widget); move results with
    /// and without the changed (0x100), accepted (1) and refused (0x10) bits;
    /// event frame present or not; default or the owner's sound listener;
    /// and the unwinds of its calls.
    #[test]
    fn ui_pair_handle_input_matches() {
        oc776_install_plain_fakes();
        let moves: [u32; 7] = [0, 1, 0x10, 0x100, 0x101, 0x110, 0x111];
        fakes::covered(
            ee::s13::f_0039bed8__generated,
            ui_pair_handle_input,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7764, 9);
                c.hooks.miss = oc776_miss;
                c.pc = 0x0039_bed8;
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                let (this, input, screen, focus, focus_vt) = (
                    DATA + 0x1_0000,
                    DATA + 0x2_0000,
                    DATA + 0x3_0000,
                    DATA + 0x3_1000,
                    DATA + 0x3_2000,
                );
                let (input_vt, state, listener, listener_vt) = (
                    DATA + 0x2_1000,
                    DATA + 0x4_0000,
                    DATA + 0x4_1000,
                    DATA + 0x4_2000,
                );
                let (child, child_vt) = (DATA + 0x5_0000, DATA + 0x5_1000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(input);
                let scenario = seed % 6;
                let slot = |c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, f: u32| {
                    c.bus
                        .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(vt + off + 4, f);
                };
                let pick = |on: bool, i: u32| {
                    if on {
                        fakes::FAKE + 0x10 * i
                    } else {
                        OC776_FALSE_FN
                    }
                };
                c.bus.write32(input + 8, input_vt);
                slot(
                    c,
                    &mut r,
                    input_vt,
                    0x28,
                    pick(matches!(scenario, 2 | 5), 1),
                );
                slot(c, &mut r, input_vt, 0x30, pick(scenario == 3, 2));
                slot(c, &mut r, input_vt, 0x20, pick(scenario == 4, 3));
                c.bus.write32(this + 0x5c, screen);
                c.bus
                    .write32(this + 0x7c, if scenario == 1 { 0 } else { child });
                c.bus.write32(screen + 0xd0, focus);
                c.bus.write32(focus + 8, focus_vt);
                slot(c, &mut r, focus_vt, 0x98, fakes::FAKE + 0x40);
                slot(c, &mut r, focus_vt, 0xa8, OC776_MOVE_FN);
                c.bus.write32(child + 8, child_vt);
                slot(c, &mut r, child_vt, 0x78, fakes::FAKE + 0x50);
                c.bus.write32(listener + 0, listener_vt);
                c.bus
                    .write32(crate::difftest::GAME_GP + 0x2968, listener_vt);
                slot(c, &mut r, listener_vt, 0x10, fakes::FAKE + 0x60);
                c.bus
                    .write32(state + 0x14, if (seed / 6) % 2 == 0 { listener } else { 0 });
                oc776_force(c, 0x0039_ff50, state);
                let event = if (seed / 12) % 2 == 0 { 0x1234 } else { 0xffff };
                oc776_force(c, 0x0039_ff78, event);
                c.bus
                    .write32(OC776_CTRL + 0x10, moves[((seed / 24) % 7) as usize]);
            },
        );
    }

    /// `UIPair_Load`: a single call to `UIObject_Load`.
    #[test]
    fn ui_pair_load_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s13::f_0039bab0__generated,
            ui_pair_load,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x784c, 1);
                oc784_arm(c);
            },
        );
    }

    use crate::difftest::fakes::machine_rt;
    pub(crate) use crate::game::ui::uilistbox::tests::nav_setup;
    pub(crate) use crate::game::ui::uilistbox::tests::nav_wire;

    pub(crate) use crate::game::ui::uilistbox::tests::NAV_MENU;

    use std::ptr::addr_of_mut;

    /// Gate word of the faked `UIMenu_SetCurrentItem` (in this machine's RAM).
    pub(crate) const OC1073_CTRL: u32 = DATA + 0x34_0000;

    pub(crate) const OC1073_MAGIC: u32 = 0x1073_c0de;

    pub(crate) static OC1073_PREV_SET_CURRENT: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake for `UIMenu_SetCurrentItem` (0x39ad38): logs the call when
    /// the gate is armed, otherwise runs the body it replaced.
    pub(crate) fn oc1073_fake_set_current(c: &mut Cpu) {
        if c.bus.read32(OC1073_CTRL) != OC1073_MAGIC {
            return fakes::fall_back(c, &OC1073_PREV_SET_CURRENT, ee::s13::f_0039ad38__generated);
        }
        fakes::fake_call(c, 0x0039_ad38);
    }

    #[test]
    fn ui_menu_find_last_selectable_matches() {
        check(
            ee::s13::f_0039b878__generated,
            ui_menu_find_last_selectable,
            300,
            &|c, seed| {
                let mut r = nav_setup(c, seed, 0x39b8_78);
                // The start index: in range on odd seeds, past the end otherwise.
                let start = if seed % 2 == 1 {
                    r.next_u32() % 6
                } else {
                    r.next_u32() % 8
                };
                c.g[A1][0] = (start | (r.next_u32() & !0xff)) as u64;
                c.pc = 0x0039_b878;
            },
        );
    }

    #[test]
    fn ui_menu_select_index_matches() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039AD38),
            oc1073_fake_set_current,
            &OC1073_PREV_SET_CURRENT,
        );
        check(
            ee::s13::f_0039ae98__generated,
            ui_menu_select_index,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x39ae_98, 0);
                c.bus.write32(OC1073_CTRL, OC1073_MAGIC);
                nav_wire(c, &mut r, 8);
                let cur = r.next_u32() % 8;
                let limit = r.next_u32() % 8;
                let old = r.next_u32() % 8;
                c.bus.write8(NAV_MENU + 0x95, cur as u8);
                c.bus.write8(NAV_MENU + 0x97, limit as u8);
                c.bus.write8(NAV_MENU + 0x98, old as u8);
                // The argument is the old index: equal to the current one on
                // every fourth seed (no change), with junk in the high bits.
                let index = if seed % 4 == 0 { cur } else { r.next_u32() % 8 };
                c.g[A0][0] = sx(NAV_MENU);
                c.g[A1][0] = (index | (r.next_u32() & !0xff)) as u64;
                c.pc = 0x0039_ae98;
            },
        );
    }
}
