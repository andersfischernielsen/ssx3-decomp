//! Staging module oc1519 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1519`.

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
    (0x0039_4be0, ps2_bezier_man_v10_00394be0, "PS2BezierMan_v10"),
    (0x0039_4be8, ps2_bezier_man_v11_00394be8, "PS2BezierMan_v11"),
    (0x0039_4bf0, ps2_bezier_man_v12_00394bf0, "PS2BezierMan_v12"),
    (0x0039_4bf8, ps2_bezier_man_v13_00394bf8, "PS2BezierMan_v13"),
    (0x0039_4c00, ps2_bezier_man_v14_00394c00, "PS2BezierMan_v14"),
    (0x0039_4c08, ps2_bezier_man_v15_00394c08, "PS2BezierMan_v15"),
    (0x0039_4c10, ps2_bezier_man_v16_00394c10, "PS2BezierMan_v16"),
    (0x0039_68c8, vfx_tube_render_003968c8, "VfxTube_Render"),
    (0x0039_6918, path_arrow_update_00396918, "PathArrow_Update"),
    (0x0039_6920, path_arrow_render_00396920, "PathArrow_Render"),
    (0x0039_6958, detail_system_v05_00396958, "DetailSystem_v05"),
    (
        0x003a_4bd8,
        ui_object_handle_input_003a4bd8,
        "UIObject_HandleInput",
    ),
    (
        0x003a_4be0,
        ui_object_refresh_text_003a4be0,
        "UIObject_RefreshText",
    ),
    (0x003a_4be8, ui_object_render_003a4be8, "UIObject_Render"),
    (0x003a_4bf0, ui_object_v18_003a4bf0, "UIObject_v18"),
    (0x003a_4ea8, ui_icon_get_type_003a4ea8, "UIIcon_GetType"),
    (
        0x003a_50f8,
        c_ui_group_get_type_003a50f8,
        "cUIGroup_GetType",
    ),
    (0x003a_5330, ui_menu_get_type_003a5330, "UIMenu_GetType"),
    (0x003a_5500, ui_pair_get_type_003a5500, "UIPair_GetType"),
    (
        0x003a_56a0,
        ui_list_box_get_type_003a56a0,
        "UIListBox_GetType",
    ),
    (0x003a_56d8, ui_list_box_v23_003a56d8, "UIListBox_v23"),
    (0x003a_5aa8, ui_state_init_003a5aa8, "UIState_Init"),
    (
        0x003a_5ab0,
        ui_state_transition_in_003a5ab0,
        "UIState_TransitionIn",
    ),
    (
        0x003a_5ab8,
        ui_state_handle_input_003a5ab8,
        "UIState_HandleInput",
    ),
    (
        0x003a_5ac0,
        ui_state_transition_out_003a5ac0,
        "UIState_TransitionOut",
    ),
    (
        0x003a_5ac8,
        ui_state_on_deactivate_003a5ac8,
        "UIState_OnDeactivate",
    ),
    (0x003a_5ad0, ui_state_on_render_003a5ad0, "UIState_OnRender"),
    (
        0x003a_5ad8,
        ui_state_on_anim_event_003a5ad8,
        "UIState_OnAnimEvent",
    ),
    (
        0x003a_5b08,
        ui_state_on_child_event_003a5b08,
        "UIState_OnChildEvent",
    ),
    (
        0x003a_5b10,
        ui_state_on_widget_created_003a5b10,
        "UIState_OnWidgetCreated",
    ),
    (
        0x003a_5b18,
        ui_state_on_widget_action_003a5b18,
        "UIState_OnWidgetAction",
    ),
    (
        0x003a_5b20,
        ui_state_on_widget_notify_003a5b20,
        "UIState_OnWidgetNotify",
    ),
    (0x003a_5b28, ui_state_v21_003a5b28, "UIState_v21"),
    (0x003a_5b30, ui_state_v22_003a5b30, "UIState_v22"),
    (0x003a_5b38, ui_state_v23_003a5b38, "UIState_v23"),
    (
        0x003a_5b40,
        ui_state_on_net_event_003a5b40,
        "UIState_OnNetEvent",
    ),
    (0x003a_5df0, ui_text_get_type_003a5df0, "UIText_GetType"),
    (
        0x003a_6000,
        c_ui_scroll_get_type_003a6000,
        "cUIScroll_GetType",
    ),
    (
        0x003a_6168,
        c_ui_progress_get_type_003a6168,
        "cUIProgress_GetType",
    ),
    (
        0x003a_6220,
        c_ui_slider_get_type_003a6220,
        "cUISlider_GetType",
    ),
    (
        0x003a_6330,
        c_ui_text_scroll_get_type_003a6330,
        "cUITextScroll_GetType",
    ),
    (0x003a_6540, ui_vector_get_type_003a6540, "UIVector_GetType"),
    (
        0x003a_dc48,
        c_ps2_chunk_resolve_v08_003adc48,
        "cPS2ChunkResolve_v08",
    ),
    (
        0x003b_0fa0,
        ps2_sony_codec_internal_v05_003b0fa0,
        "PS2_SONY_CODEC_INTERNAL_v05",
    ),
    (
        0x003b_5470,
        snd_timer_null_callback_003b5470,
        "SND_TimerNullCallback",
    ),
];

/// `PS2BezierMan_v10` (0x0039_4be0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v10_00394be0(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x354), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v11` (0x0039_4be8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v11_00394be8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x344), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v12` (0x0039_4bf0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v12_00394bf0(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x34c), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v13` (0x0039_4bf8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v13_00394bf8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x350), c.g[A1][0]);
    ret(c);
}

/// `PS2BezierMan_v14` (0x0039_4c00): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v14_00394c00(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x344));
    ret(c);
}

/// `PS2BezierMan_v15` (0x0039_4c08): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v15_00394c08(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34c));
    ret(c);
}

/// `PS2BezierMan_v16` (0x0039_4c10): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_v16_00394c10(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x350));
    ret(c);
}

/// `VfxTube_Render` (0x0039_68c8): straight-line leaf. Mechanical port of the generated body.
pub fn vfx_tube_render_003968c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PathArrow_Update` (0x0039_6918): straight-line leaf. Mechanical port of the generated body.
pub fn path_arrow_update_00396918(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PathArrow_Render` (0x0039_6920): straight-line leaf. Mechanical port of the generated body.
pub fn path_arrow_render_00396920(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DetailSystem_v05` (0x0039_6958): straight-line leaf. Mechanical port of the generated body.
pub fn detail_system_v05_00396958(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIObject_HandleInput` (0x003a_4bd8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_object_handle_input_003a4bd8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIObject_RefreshText` (0x003a_4be0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_object_refresh_text_003a4be0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIObject_Render` (0x003a_4be8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_object_render_003a4be8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIObject_v18` (0x003a_4bf0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_object_v18_003a4bf0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIIcon_GetType` (0x003a_4ea8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_icon_get_type_003a4ea8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x11;
    ret(c);
}

/// `cUIGroup_GetType` (0x003a_50f8): straight-line leaf. Mechanical port of the generated body.
pub fn c_ui_group_get_type_003a50f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x10;
    ret(c);
}

/// `UIMenu_GetType` (0x003a_5330): straight-line leaf. Mechanical port of the generated body.
pub fn ui_menu_get_type_003a5330(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x13;
    ret(c);
}

/// `UIPair_GetType` (0x003a_5500): straight-line leaf. Mechanical port of the generated body.
pub fn ui_pair_get_type_003a5500(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x15;
    ret(c);
}

/// `UIListBox_GetType` (0x003a_56a0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_list_box_get_type_003a56a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x12;
    ret(c);
}

/// `UIListBox_v23` (0x003a_56d8): straight-line leaf: 1 load(s), 1 store(s). Mechanical port of the generated body.
pub fn ui_list_box_v23_003a56d8(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x74));
    c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
    c.g[V1][0] = 0xfffffffffffffffb;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
    c.sw((c.g[A0][0] as u32).wrapping_add(0x74), c.g[V0][0]);
    ret(c);
}

/// `UIState_Init` (0x003a_5aa8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_init_003a5aa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIState_TransitionIn` (0x003a_5ab0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_transition_in_003a5ab0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIState_HandleInput` (0x003a_5ab8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_handle_input_003a5ab8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIState_TransitionOut` (0x003a_5ac0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_transition_out_003a5ac0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIState_OnDeactivate` (0x003a_5ac8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_deactivate_003a5ac8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIState_OnRender` (0x003a_5ad0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_render_003a5ad0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIState_OnAnimEvent` (0x003a_5ad8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_anim_event_003a5ad8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIState_OnChildEvent` (0x003a_5b08): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_child_event_003a5b08(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIState_OnWidgetCreated` (0x003a_5b10): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_widget_created_003a5b10(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIState_OnWidgetAction` (0x003a_5b18): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_widget_action_003a5b18(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UIState_OnWidgetNotify` (0x003a_5b20): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_widget_notify_003a5b20(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    ret(c);
}

/// `UIState_v21` (0x003a_5b28): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_v21_003a5b28(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    ret(c);
}

/// `UIState_v22` (0x003a_5b30): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_v22_003a5b30(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    ret(c);
}

/// `UIState_v23` (0x003a_5b38): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_v23_003a5b38(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIState_OnNetEvent` (0x003a_5b40): straight-line leaf. Mechanical port of the generated body.
pub fn ui_state_on_net_event_003a5b40(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIText_GetType` (0x003a_5df0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_text_get_type_003a5df0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x17;
    ret(c);
}

/// `cUIScroll_GetType` (0x003a_6000): straight-line leaf. Mechanical port of the generated body.
pub fn c_ui_scroll_get_type_003a6000(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x21;
    ret(c);
}

/// `cUIProgress_GetType` (0x003a_6168): straight-line leaf. Mechanical port of the generated body.
pub fn c_ui_progress_get_type_003a6168(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x19;
    ret(c);
}

/// `cUISlider_GetType` (0x003a_6220): straight-line leaf. Mechanical port of the generated body.
pub fn c_ui_slider_get_type_003a6220(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x16;
    ret(c);
}

/// `cUITextScroll_GetType` (0x003a_6330): straight-line leaf. Mechanical port of the generated body.
pub fn c_ui_text_scroll_get_type_003a6330(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x20;
    ret(c);
}

/// `UIVector_GetType` (0x003a_6540): straight-line leaf. Mechanical port of the generated body.
pub fn ui_vector_get_type_003a6540(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x18;
    ret(c);
}

/// `cPS2ChunkResolve_v08` (0x003a_dc48): straight-line leaf. Mechanical port of the generated body.
pub fn c_ps2_chunk_resolve_v08_003adc48(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL_v05` (0x003b_0fa0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_sony_codec_internal_v05_003b0fa0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `SND_TimerNullCallback` (0x003b_5470): straight-line leaf. Mechanical port of the generated body.
pub fn snd_timer_null_callback_003b5470(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, Rng};

    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    fn leaf_val(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => (r.next_u32() % 64) as u64,
            4 => sx(r.next_u32()),
            5 => r.next_u32() as u64,
            6 => 0x8000_0000,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        }
    }

    /// Fill `size` bytes at `start` with zero / small / random words, or, when
    /// `chase` is set, with pointers (16-byte aligned) into `ptr_lo..ptr_lo + 0x200`.
    #[allow(dead_code)]
    fn leaf_fill(c: &mut Cpu, r: &mut Rng, start: u32, size: u32, chase: bool, ptr_lo: u32) {
        for i in 0..size / 4 {
            let w = if chase {
                ptr_lo + 16 * (r.next_u32() % 32)
            } else {
                match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() % 16,
                    _ => r.next_u32(),
                }
            };
            c.bus.write32(start + 4 * i, w);
        }
    }

    /// `PS2BezierMan_v10` (0x0039_4be0) against the generated body.
    #[test]
    fn ps2_bezier_man_v10_00394be0_matches() {
        check(
            ee::s13::f_00394be0__generated,
            ps2_bezier_man_v10_00394be0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394be0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v11` (0x0039_4be8) against the generated body.
    #[test]
    fn ps2_bezier_man_v11_00394be8_matches() {
        check(
            ee::s13::f_00394be8__generated,
            ps2_bezier_man_v11_00394be8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394be8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v12` (0x0039_4bf0) against the generated body.
    #[test]
    fn ps2_bezier_man_v12_00394bf0_matches() {
        check(
            ee::s13::f_00394bf0__generated,
            ps2_bezier_man_v12_00394bf0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bf0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v13` (0x0039_4bf8) against the generated body.
    #[test]
    fn ps2_bezier_man_v13_00394bf8_matches() {
        check(
            ee::s13::f_00394bf8__generated,
            ps2_bezier_man_v13_00394bf8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bf8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v14` (0x0039_4c00) against the generated body.
    #[test]
    fn ps2_bezier_man_v14_00394c00_matches() {
        check(
            ee::s13::f_00394c00__generated,
            ps2_bezier_man_v14_00394c00,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394c00, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v15` (0x0039_4c08) against the generated body.
    #[test]
    fn ps2_bezier_man_v15_00394c08_matches() {
        check(
            ee::s13::f_00394c08__generated,
            ps2_bezier_man_v15_00394c08,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394c08, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_v16` (0x0039_4c10) against the generated body.
    #[test]
    fn ps2_bezier_man_v16_00394c10_matches() {
        check(
            ee::s13::f_00394c10__generated,
            ps2_bezier_man_v16_00394c10,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394c10, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `VfxTube_Render` (0x0039_68c8) against the generated body.
    #[test]
    fn vfx_tube_render_003968c8_matches() {
        check(
            ee::s13::f_003968c8__generated,
            vfx_tube_render_003968c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3968c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PathArrow_Update` (0x0039_6918) against the generated body.
    #[test]
    fn path_arrow_update_00396918_matches() {
        check(
            ee::s13::f_00396918__generated,
            path_arrow_update_00396918,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x396918, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PathArrow_Render` (0x0039_6920) against the generated body.
    #[test]
    fn path_arrow_render_00396920_matches() {
        check(
            ee::s13::f_00396920__generated,
            path_arrow_render_00396920,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x396920, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DetailSystem_v05` (0x0039_6958) against the generated body.
    #[test]
    fn detail_system_v05_00396958_matches() {
        check(
            ee::s13::f_00396958__generated,
            detail_system_v05_00396958,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x396958, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIObject_HandleInput` (0x003a_4bd8) against the generated body.
    #[test]
    fn ui_object_handle_input_003a4bd8_matches() {
        check(
            ee::s13::f_003a4bd8__generated,
            ui_object_handle_input_003a4bd8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a4bd8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIObject_RefreshText` (0x003a_4be0) against the generated body.
    #[test]
    fn ui_object_refresh_text_003a4be0_matches() {
        check(
            ee::s13::f_003a4be0__generated,
            ui_object_refresh_text_003a4be0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a4be0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIObject_Render` (0x003a_4be8) against the generated body.
    #[test]
    fn ui_object_render_003a4be8_matches() {
        check(
            ee::s13::f_003a4be8__generated,
            ui_object_render_003a4be8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a4be8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIObject_v18` (0x003a_4bf0) against the generated body.
    #[test]
    fn ui_object_v18_003a4bf0_matches() {
        check(
            ee::s13::f_003a4bf0__generated,
            ui_object_v18_003a4bf0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a4bf0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIIcon_GetType` (0x003a_4ea8) against the generated body.
    #[test]
    fn ui_icon_get_type_003a4ea8_matches() {
        check(
            ee::s13::f_003a4ea8__generated,
            ui_icon_get_type_003a4ea8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a4ea8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cUIGroup_GetType` (0x003a_50f8) against the generated body.
    #[test]
    fn c_ui_group_get_type_003a50f8_matches() {
        check(
            ee::s13::f_003a50f8__generated,
            c_ui_group_get_type_003a50f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a50f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIMenu_GetType` (0x003a_5330) against the generated body.
    #[test]
    fn ui_menu_get_type_003a5330_matches() {
        check(
            ee::s13::f_003a5330__generated,
            ui_menu_get_type_003a5330,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5330, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIPair_GetType` (0x003a_5500) against the generated body.
    #[test]
    fn ui_pair_get_type_003a5500_matches() {
        check(
            ee::s13::f_003a5500__generated,
            ui_pair_get_type_003a5500,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5500, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIListBox_GetType` (0x003a_56a0) against the generated body.
    #[test]
    fn ui_list_box_get_type_003a56a0_matches() {
        check(
            ee::s13::f_003a56a0__generated,
            ui_list_box_get_type_003a56a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a56a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIListBox_v23` (0x003a_56d8) against the generated body.
    #[test]
    fn ui_list_box_v23_003a56d8_matches() {
        check(
            ee::s13::f_003a56d8__generated,
            ui_list_box_v23_003a56d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a56d8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_Init` (0x003a_5aa8) against the generated body.
    #[test]
    fn ui_state_init_003a5aa8_matches() {
        check(
            ee::s13::f_003a5aa8__generated,
            ui_state_init_003a5aa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5aa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_TransitionIn` (0x003a_5ab0) against the generated body.
    #[test]
    fn ui_state_transition_in_003a5ab0_matches() {
        check(
            ee::s13::f_003a5ab0__generated,
            ui_state_transition_in_003a5ab0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5ab0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_HandleInput` (0x003a_5ab8) against the generated body.
    #[test]
    fn ui_state_handle_input_003a5ab8_matches() {
        check(
            ee::s13::f_003a5ab8__generated,
            ui_state_handle_input_003a5ab8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5ab8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_TransitionOut` (0x003a_5ac0) against the generated body.
    #[test]
    fn ui_state_transition_out_003a5ac0_matches() {
        check(
            ee::s13::f_003a5ac0__generated,
            ui_state_transition_out_003a5ac0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5ac0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnDeactivate` (0x003a_5ac8) against the generated body.
    #[test]
    fn ui_state_on_deactivate_003a5ac8_matches() {
        check(
            ee::s13::f_003a5ac8__generated,
            ui_state_on_deactivate_003a5ac8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5ac8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnRender` (0x003a_5ad0) against the generated body.
    #[test]
    fn ui_state_on_render_003a5ad0_matches() {
        check(
            ee::s13::f_003a5ad0__generated,
            ui_state_on_render_003a5ad0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5ad0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnAnimEvent` (0x003a_5ad8) against the generated body.
    #[test]
    fn ui_state_on_anim_event_003a5ad8_matches() {
        check(
            ee::s13::f_003a5ad8__generated,
            ui_state_on_anim_event_003a5ad8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5ad8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnChildEvent` (0x003a_5b08) against the generated body.
    #[test]
    fn ui_state_on_child_event_003a5b08_matches() {
        check(
            ee::s13::f_003a5b08__generated,
            ui_state_on_child_event_003a5b08,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b08, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnWidgetCreated` (0x003a_5b10) against the generated body.
    #[test]
    fn ui_state_on_widget_created_003a5b10_matches() {
        check(
            ee::s13::f_003a5b10__generated,
            ui_state_on_widget_created_003a5b10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnWidgetAction` (0x003a_5b18) against the generated body.
    #[test]
    fn ui_state_on_widget_action_003a5b18_matches() {
        check(
            ee::s13::f_003a5b18__generated,
            ui_state_on_widget_action_003a5b18,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b18, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnWidgetNotify` (0x003a_5b20) against the generated body.
    #[test]
    fn ui_state_on_widget_notify_003a5b20_matches() {
        check(
            ee::s13::f_003a5b20__generated,
            ui_state_on_widget_notify_003a5b20,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b20, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_v21` (0x003a_5b28) against the generated body.
    #[test]
    fn ui_state_v21_003a5b28_matches() {
        check(
            ee::s13::f_003a5b28__generated,
            ui_state_v21_003a5b28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_v22` (0x003a_5b30) against the generated body.
    #[test]
    fn ui_state_v22_003a5b30_matches() {
        check(
            ee::s13::f_003a5b30__generated,
            ui_state_v22_003a5b30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_v23` (0x003a_5b38) against the generated body.
    #[test]
    fn ui_state_v23_003a5b38_matches() {
        check(
            ee::s13::f_003a5b38__generated,
            ui_state_v23_003a5b38,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b38, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIState_OnNetEvent` (0x003a_5b40) against the generated body.
    #[test]
    fn ui_state_on_net_event_003a5b40_matches() {
        check(
            ee::s13::f_003a5b40__generated,
            ui_state_on_net_event_003a5b40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5b40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIText_GetType` (0x003a_5df0) against the generated body.
    #[test]
    fn ui_text_get_type_003a5df0_matches() {
        check(
            ee::s13::f_003a5df0__generated,
            ui_text_get_type_003a5df0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a5df0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cUIScroll_GetType` (0x003a_6000) against the generated body.
    #[test]
    fn c_ui_scroll_get_type_003a6000_matches() {
        check(
            ee::s13::f_003a6000__generated,
            c_ui_scroll_get_type_003a6000,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a6000, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cUIProgress_GetType` (0x003a_6168) against the generated body.
    #[test]
    fn c_ui_progress_get_type_003a6168_matches() {
        check(
            ee::s13::f_003a6168__generated,
            c_ui_progress_get_type_003a6168,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a6168, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cUISlider_GetType` (0x003a_6220) against the generated body.
    #[test]
    fn c_ui_slider_get_type_003a6220_matches() {
        check(
            ee::s13::f_003a6220__generated,
            c_ui_slider_get_type_003a6220,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a6220, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cUITextScroll_GetType` (0x003a_6330) against the generated body.
    #[test]
    fn c_ui_text_scroll_get_type_003a6330_matches() {
        check(
            ee::s13::f_003a6330__generated,
            c_ui_text_scroll_get_type_003a6330,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a6330, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIVector_GetType` (0x003a_6540) against the generated body.
    #[test]
    fn ui_vector_get_type_003a6540_matches() {
        check(
            ee::s13::f_003a6540__generated,
            ui_vector_get_type_003a6540,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a6540, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cPS2ChunkResolve_v08` (0x003a_dc48) against the generated body.
    #[test]
    fn c_ps2_chunk_resolve_v08_003adc48_matches() {
        check(
            ee::s13::f_003adc48__generated,
            c_ps2_chunk_resolve_v08_003adc48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3adc48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2_SONY_CODEC_INTERNAL_v05` (0x003b_0fa0) against the generated body.
    #[test]
    fn ps2_sony_codec_internal_v05_003b0fa0_matches() {
        check(
            ee::s13::f_003b0fa0__generated,
            ps2_sony_codec_internal_v05_003b0fa0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b0fa0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SND_TimerNullCallback` (0x003b_5470) against the generated body.
    #[test]
    fn snd_timer_null_callback_003b5470_matches() {
        check(
            ee::s13::f_003b5470__generated,
            snd_timer_null_callback_003b5470,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3b5470, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
