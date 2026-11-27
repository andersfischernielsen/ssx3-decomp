//! `src/util/menu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::set_reg;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::gp as gp_2;
pub(crate) use crate::game::ai::computer::lh_2;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::fepopup::at as at_5;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sdsp;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::replay::replaycache::destroy_if_flag;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::visualfx::boardtrailfx::gp;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002c_fb58,
        menu_sys_group_item_measure,
        "MenuSysGroupItem_Measure",
    ),
    (0x002c_a258, menu_sys_item_ctor, "MenuSysItem_ctor"),
    (0x002c_a280, menu_sys_item_dtor, "MenuSysItem_dtor"),
    (
        0x002c_a2b0,
        menu_sys_item_handle_input,
        "MenuSysItem_HandleInput",
    ),
    (
        0x002c_a320,
        menu_sys_item_is_selectable,
        "MenuSysItem_IsSelectable",
    ),
    (
        0x002c_a378,
        menu_sys_item_is_visible,
        "MenuSysItem_IsVisible",
    ),
    (
        0x002c_a388,
        menu_sys_item_set_visible,
        "MenuSysItem_SetVisible",
    ),
    (
        0x002c_a3b0,
        menu_sys_item_is_disabled,
        "MenuSysItem_IsDisabled",
    ),
    (
        0x002c_a3e8,
        menu_sys_item_get_input_code,
        "MenuSysItem_GetInputCode",
    ),
    (0x002c_aa58, menu_sys_page_ctor, "MenuSysPage_ctor"),
    (0x002c_aa80, menu_sys_page_dtor, "MenuSysPage_dtor"),
    (
        0x002c_aab0,
        menu_sys_page_validate_selection,
        "MenuSysPage_ValidateSelection",
    ),
    (
        0x002c_ab08,
        menu_sys_page_select_next,
        "MenuSysPage_SelectNext",
    ),
    (
        0x002c_abe0,
        menu_sys_page_select_prev,
        "MenuSysPage_SelectPrev",
    ),
    (
        0x002c_acc0,
        menusyspage_handle_input,
        "MenuSysPage_HandleInput",
    ),
    (
        0x002c_b180,
        menu_sys_page_begin_draw,
        "MenuSysPage_BeginDraw",
    ),
    (0x002c_b2f8, menu_sys_page_end_draw, "MenuSysPage_EndDraw"),
    (0x002c_b350, menu_sys_build_quad, "MenuSys_BuildQuad"),
    (
        0x002c_b880,
        menu_sys_page_set_row_color,
        "MenuSysPage_SetRowColor",
    ),
    (
        0x002c_bcc8,
        menusyspage_insert_item,
        "MenuSysPage_InsertItem",
    ),
    (
        0x002c_be10,
        menu_sys_page_remove_all_items,
        "MenuSysPage_RemoveAllItems",
    ),
    (
        0x002c_be68,
        menu_sys_page_set_selection,
        "MenuSysPage_SetSelection",
    ),
    (
        0x002c_bf08,
        menu_sys_page_get_selected_index,
        "MenuSysPage_GetSelectedIndex",
    ),
    (
        0x002c_bf30,
        menu_sys_page_get_selected_item,
        "MenuSysPage_GetSelectedItem",
    ),
    (
        0x002c_bf60,
        menu_sys_page_on_activate,
        "MenuSysPage_OnActivate",
    ),
    (
        0x002c_bfd0,
        menu_sys_page_on_deactivate,
        "MenuSysPage_OnDeactivate",
    ),
    (0x002c_c048, menu_sys_dtor, "MenuSys_dtor"),
    (0x002c_c070, menu_sys_reset, "MenuSys_Reset"),
    (0x002c_c0c0, menu_sys_update, "MenuSys_Update"),
    (0x002c_c350, menu_sys_draw, "MenuSys_Draw"),
    (0x002c_c3b8, menu_sys_push_page, "MenuSys_PushPage"),
    (0x002c_c460, menu_sys_pop_page, "MenuSys_PopPage"),
    (
        0x002c_c578,
        menu_sys_bool_item_handle_input,
        "MenuSysBoolItem_HandleInput",
    ),
    (0x002c_c758, menu_sys_bool_item_draw, "MenuSysBoolItem_Draw"),
    (0x002c_cc38, menu_sys_enum_item_draw, "MenuSysEnumItem_Draw"),
    (
        0x002c_ccc8,
        menu_sys_submenu_item_ctor,
        "MenuSysSubmenuItem_ctor",
    ),
    (
        0x002c_cd20,
        menu_sys_submenu_item_handle_input,
        "MenuSysSubmenuItem_HandleInput",
    ),
    (
        0x002c_cd90,
        menu_sys_submenu_item_measure,
        "MenuSysSubmenuItem_Measure",
    ),
    (0x002c_cdf0, menu_sys_back_item_ctor, "MenuSysBackItem_ctor"),
    (
        0x002c_ce38,
        menu_sys_back_item_handle_input,
        "MenuSysBackItem_HandleInput",
    ),
    (
        0x002c_cea8,
        menu_sys_back_item_measure,
        "MenuSysBackItem_Measure",
    ),
    (
        0x002c_cf08,
        menu_sys_action_item_ctor,
        "MenuSysActionItem_ctor",
    ),
    (
        0x002c_cf98,
        menu_sys_action_item_ctor_3,
        "MenuSysActionItem_ctor_3",
    ),
    (
        0x002c_d008,
        menusysactionitem_handle_input,
        "MenuSysActionItem_HandleInput",
    ),
    (
        0x002c_d090,
        menu_sys_action_item_measure,
        "MenuSysActionItem_Measure",
    ),
    (
        0x002c_d0e8,
        menu_sys_action_item_draw,
        "MenuSysActionItem_Draw",
    ),
    (
        0x002c_d1d0,
        menu_sys_callback_item_handle_input,
        "MenuSysCallbackItem_HandleInput",
    ),
    (
        0x002c_d240,
        menu_sys_callback_item_measure,
        "MenuSysCallbackItem_Measure",
    ),
    (
        0x002c_d2f0,
        menu_sys_label_item_measure,
        "MenuSysLabelItem_Measure",
    ),
    (
        0x002c_d3a0,
        menu_sys_spacer_item_measure,
        "MenuSysSpacerItem_Measure",
    ),
    (
        0x002c_d578,
        menu_sys_int8_item_reset_value,
        "MenuSysInt8Item_ResetValue",
    ),
    (
        0x002c_d7b8,
        menu_sys_int8_item_measure,
        "MenuSysInt8Item_Measure",
    ),
    (0x002c_d7f0, menu_sys_int8_item_draw, "MenuSysInt8Item_Draw"),
    (
        0x002c_d8f8,
        menu_sys_int_item_ctor_3,
        "MenuSysIntItem_ctor_3",
    ),
    (
        0x002c_d9b8,
        menu_sys_int_item_reset_value,
        "MenuSysIntItem_ResetValue",
    ),
    (
        0x002c_dbf0,
        menu_sys_int_item_measure,
        "MenuSysIntItem_Measure",
    ),
    (0x002c_dc28, menu_sys_int_item_draw, "MenuSysIntItem_Draw"),
    (
        0x002c_dce8,
        menu_sys_float_item_ctor_2,
        "MenuSysFloatItem_ctor_2",
    ),
    (
        0x002c_df78,
        menu_sys_float_item_reset_value,
        "MenuSysFloatItem_ResetValue",
    ),
    (
        0x002c_dfa0,
        menu_sys_float_item_measure,
        "MenuSysFloatItem_Measure",
    ),
    (
        0x002c_e100,
        menu_sys_float_item_draw,
        "MenuSysFloatItem_Draw",
    ),
    (
        0x002c_e368,
        menu_sys_float_range_item_measure,
        "MenuSysFloatRangeItem_Measure",
    ),
    (
        0x002c_e3a0,
        menu_sys_float_range_item_draw,
        "MenuSysFloatRangeItem_Draw",
    ),
    (
        0x002c_e418,
        menu_sys_text_edit_item_ctor,
        "MenuSysTextEditItem_ctor",
    ),
    (
        0x002c_ed20,
        menu_sys_color_channel_item_ctor,
        "MenuSysColorChannelItem_ctor",
    ),
    (0x002c_f8d8, menu_sys_group_item_v08, "MenuSysGroupItem_v08"),
    (0x002c_fb28, menu_sys_group_item_v10, "MenuSysGroupItem_v10"),
    (
        0x002c_fd28,
        menu_sys_group_item_draw,
        "MenuSysGroupItem_Draw",
    ),
    (
        0x002c_fe78,
        menu_sys_group_item_on_focus,
        "MenuSysGroupItem_OnFocus",
    ),
    (
        0x002c_ff28,
        menu_sys_color_swatch_item_ctor,
        "MenuSysColorSwatchItem_ctor",
    ),
    (
        0x002c_ff88,
        menu_sys_color_swatch_item_measure,
        "MenuSysColorSwatchItem_Measure",
    ),
    (
        0x002d_0450,
        menu_sys_vector_label_item_measure,
        "MenuSysVectorLabelItem_Measure",
    ),
    (
        0x002d_0500,
        menu_sys_vector_label_item_draw,
        "MenuSysVectorLabelItem_Draw",
    ),
    (
        0x002d_08e0,
        menu_sys_point2d_item_on_focus,
        "MenuSysPoint2DItem_OnFocus",
    ),
    (
        0x002d_0d48,
        menu_sys_point2d_item_measure,
        "MenuSysPoint2DItem_Measure",
    ),
    (
        0x002d_19b8,
        wscript_man_obj_trigger_hook,
        "WScriptMan_ObjTriggerHook",
    ),
    (
        0x002d_1a30,
        wscript_man_try_run_obj_script3,
        "WScriptMan_TryRunObjScript3",
    ),
    (
        0x002d_1ac8,
        wscript_man_run_obj_script5_hook,
        "WScriptMan_RunObjScript5Hook",
    ),
    (0x002d_1af0, world_get_rider_count, "World_GetRiderCount"),
    (0x002d_1b08, world_get_rider, "World_GetRider"),
    (0x002d_1b30, mod_obj_call_slot7, "ModObj_CallSlot7"),
    (
        0x002d_1b58,
        world_get_rider_position,
        "World_GetRiderPosition",
    ),
    (
        0x002d_1c98,
        world_get_ai_frame_count,
        "World_GetAIFrameCount",
    ),
    (0x002d_1cb0, world_get_flag_manager, "World_GetFlagManager"),
    (0x002d_1ba0, world_get_peak_number, "World_GetPeakNumber"),
    (
        0x002d_19e8,
        wscript_man_try_run_kill_script,
        "WScriptMan_TryRunKillScript",
    ),
    (0x002d_1bf0, world_add_focus_object, "World_AddFocusObject"),
    (
        0x002d_1a78,
        wscriptman_run_obj_script0_hook,
        "WScriptMan_RunObjScript0Hook",
    ),
    (0x002c_a368, menu_sys_item_on_focus, "MenuSysItem_OnFocus"),
    (0x002c_a370, menu_sys_item_on_blur, "MenuSysItem_OnBlur"),
    (
        0x002c_a408,
        menu_sys_item_get_left_axis,
        "MenuSysItem_GetLeftAxis",
    ),
    (
        0x002c_a428,
        menu_sys_item_get_right_axis,
        "MenuSysItem_GetRightAxis",
    ),
    (
        0x002c_a448,
        menu_sys_item_get_up_axis,
        "MenuSysItem_GetUpAxis",
    ),
    (
        0x002c_a468,
        menu_sys_item_get_down_axis,
        "MenuSysItem_GetDownAxis",
    ),
    (
        0x002c_a488,
        menu_sys_item_is_slow_mod_held,
        "MenuSysItem_IsSlowModHeld",
    ),
    (
        0x002c_a4a8,
        menu_sys_item_is_fine_mod_held,
        "MenuSysItem_IsFineModHeld",
    ),
    (
        0x002c_a4c8,
        menu_sys_item_measure_row,
        "MenuSysItem_MeasureRow",
    ),
    (
        0x002c_a768,
        menu_sys_item_draw_row_text,
        "MenuSysItem_DrawRowText",
    ),
    (0x002c_a988, menu_sys_item_draw_row, "MenuSysItem_DrawRow"),
    (0x002c_ad48, menu_sys_page_on_action, "MenuSysPage_OnAction"),
    (0x002c_ad50, menu_sys_page_v04, "MenuSysPage_v04"),
    (0x002c_ad58, menu_sys_page_draw, "MenuSysPage_Draw"),
    (
        0x002c_b498,
        menu_sys_page_draw_frame,
        "MenuSysPage_DrawFrame",
    ),
    (
        0x002c_b8c8,
        menu_sys_page_draw_scroll_arrow_down,
        "MenuSysPage_DrawScrollArrowDown",
    ),
    (
        0x002c_bac8,
        menu_sys_page_draw_scroll_arrow_up,
        "MenuSysPage_DrawScrollArrowUp",
    ),
    (
        0x002c_bd50,
        menu_sys_page_remove_item,
        "MenuSysPage_RemoveItem",
    ),
    (
        0x002c_be20,
        menu_sys_page_sort_items,
        "MenuSysPage_SortItems",
    ),
    (0x002c_c018, menu_sys_ctor, "MenuSys_ctor"),
    (0x002c_c0b8, menu_sys_set_active, "MenuSys_SetActive"),
    (0x002c_c248, menu_sys_get_input_code, "MenuSys_GetInputCode"),
    (0x002c_c250, menu_sys_get_key_code, "MenuSys_GetKeyCode"),
    (0x002c_c258, menu_sys_get_key_char, "MenuSys_GetKeyChar"),
    (0x002c_c260, menu_sys_get_left_axis, "MenuSys_GetLeftAxis"),
    (0x002c_c280, menu_sys_get_right_axis, "MenuSys_GetRightAxis"),
    (0x002c_c2a0, menu_sys_get_up_axis, "MenuSys_GetUpAxis"),
    (0x002c_c2c0, menu_sys_get_down_axis, "MenuSys_GetDownAxis"),
    (
        0x002c_c2e0,
        menu_sys_is_slow_mod_held,
        "MenuSys_IsSlowModHeld",
    ),
    (
        0x002c_c318,
        menu_sys_is_fine_mod_held,
        "MenuSys_IsFineModHeld",
    ),
    (0x002c_c500, menu_sys_bool_item_ctor, "MenuSysBoolItem_ctor"),
    (
        0x002c_c648,
        menu_sys_bool_item_measure,
        "MenuSysBoolItem_Measure",
    ),
    (0x002c_c7b0, menu_sys_enum_item_ctor, "MenuSysEnumItem_ctor"),
    (
        0x002c_c7f8,
        menu_sys_enum_item_ctor_2,
        "MenuSysEnumItem_ctor_2",
    ),
    (
        0x002c_c838,
        menu_sys_enum_item_ctor_3,
        "MenuSysEnumItem_ctor_3",
    ),
    (
        0x002c_c8f0,
        menu_sys_enum_item_handle_input,
        "MenuSysEnumItem_HandleInput",
    ),
    (
        0x002c_cb18,
        menu_sys_enum_item_measure,
        "MenuSysEnumItem_Measure",
    ),
    (
        0x002c_cdc8,
        menu_sys_submenu_item_draw,
        "MenuSysSubmenuItem_Draw",
    ),
    (0x002c_cee0, menu_sys_back_item_draw, "MenuSysBackItem_Draw"),
    (
        0x002c_cf50,
        menu_sys_action_item_ctor_2,
        "MenuSysActionItem_ctor_2",
    ),
    (
        0x002c_d128,
        menu_sys_callback_item_ctor,
        "MenuSysCallbackItem_ctor",
    ),
    (
        0x002c_d168,
        menu_sys_callback_item_ctor_2,
        "MenuSysCallbackItem_ctor_2",
    ),
    (
        0x002c_d278,
        menu_sys_callback_item_draw,
        "MenuSysCallbackItem_Draw",
    ),
    (
        0x002c_d2a0,
        menu_sys_label_item_ctor,
        "MenuSysLabelItem_ctor",
    ),
    (
        0x002c_d2e8,
        menu_sys_label_item_is_selectable,
        "MenuSysLabelItem_IsSelectable",
    ),
    (
        0x002c_d328,
        menu_sys_label_item_draw,
        "MenuSysLabelItem_Draw",
    ),
    (
        0x002c_d350,
        menu_sys_spacer_item_ctor,
        "MenuSysSpacerItem_ctor",
    ),
    (
        0x002c_d398,
        menu_sys_spacer_item_is_selectable,
        "MenuSysSpacerItem_IsSelectable",
    ),
    (
        0x002c_d3f0,
        menu_sys_spacer_item_draw,
        "MenuSysSpacerItem_Draw",
    ),
    (0x002c_d3f8, menu_sys_int8_item_ctor, "MenuSysInt8Item_ctor"),
    (
        0x002c_d450,
        menu_sys_int8_item_ctor_2,
        "MenuSysInt8Item_ctor_2",
    ),
    (
        0x002c_d4a0,
        menu_sys_int8_item_ctor_3,
        "MenuSysInt8Item_ctor_3",
    ),
    (
        0x002c_d5a0,
        menu_sys_int8_item_handle_input,
        "MenuSysInt8Item_HandleInput",
    ),
    (0x002c_d850, menu_sys_int_item_ctor, "MenuSysIntItem_ctor"),
    (
        0x002c_d8a8,
        menu_sys_int_item_ctor_2,
        "MenuSysIntItem_ctor_2",
    ),
    (
        0x002c_d9d8,
        menu_sys_int_item_handle_input,
        "MenuSysIntItem_HandleInput",
    ),
    (
        0x002c_dc88,
        menu_sys_float_item_ctor,
        "MenuSysFloatItem_ctor",
    ),
    (
        0x002c_dd78,
        menu_sys_float_item_handle_input,
        "MenuSysFloatItem_HandleInput",
    ),
    (0x002c_dfd8, menu_sys_format_float, "MenuSys_FormatFloat"),
    (
        0x002c_e170,
        menu_sys_float_range_item_ctor,
        "MenuSysFloatRangeItem_ctor",
    ),
    (
        0x002c_e1f0,
        menu_sys_float_range_item_handle_input,
        "MenuSysFloatRangeItem_HandleInput",
    ),
    (
        0x002c_e488,
        menu_sys_text_edit_item_handle_input,
        "MenuSysTextEditItem_HandleInput",
    ),
    (
        0x002c_e820,
        menu_sys_text_edit_item_measure,
        "MenuSysTextEditItem_Measure",
    ),
    (
        0x002c_e910,
        menu_sys_text_edit_item_draw,
        "MenuSysTextEditItem_Draw",
    ),
    (
        0x002c_ed78,
        menu_sys_color_channel_item_handle_input,
        "MenuSysColorChannelItem_HandleInput",
    ),
    (
        0x002c_eee8,
        menu_sys_color_channel_item_measure,
        "MenuSysColorChannelItem_Measure",
    ),
    (
        0x002c_f1c8,
        menu_sys_color_channel_item_draw,
        "MenuSysColorChannelItem_Draw",
    ),
    (
        0x002c_f788,
        menu_sys_group_item_page_forward_v03,
        "MenuSysGroupItem_PageForwardV03",
    ),
    (
        0x002c_f7b8,
        menu_sys_group_item_page_forward_v04,
        "MenuSysGroupItem_PageForwardV04",
    ),
    (
        0x002c_f7e8,
        menu_sys_group_item_ctor,
        "MenuSysGroupItem_ctor",
    ),
    (
        0x002c_f860,
        menu_sys_group_item_ctor_2,
        "MenuSysGroupItem_ctor_2",
    ),
    (
        0x002c_f908,
        menu_sys_group_item_add_item,
        "MenuSysGroupItem_AddItem",
    ),
    (
        0x002c_f928,
        menu_sys_group_item_remove_item,
        "MenuSysGroupItem_RemoveItem",
    ),
    (
        0x002c_f948,
        menu_sys_group_item_reset,
        "MenuSysGroupItem_Reset",
    ),
    (
        0x002c_f988,
        menu_sys_group_item_set_header_item,
        "MenuSysGroupItem_SetHeaderItem",
    ),
    (
        0x002c_fa08,
        menu_sys_group_item_handle_input,
        "MenuSysGroupItem_HandleInput",
    ),
    (0x002c_faf8, menu_sys_group_item_v09, "MenuSysGroupItem_v09"),
    (
        0x002c_fef8,
        menu_sys_group_item_on_blur,
        "MenuSysGroupItem_OnBlur",
    ),
    (
        0x002c_ff80,
        menu_sys_color_swatch_item_is_selectable,
        "MenuSysColorSwatchItem_IsSelectable",
    ),
    (
        0x002d_0008,
        menu_sys_color_swatch_item_draw,
        "MenuSysColorSwatchItem_Draw",
    ),
    (
        0x002d_03f0,
        menu_sys_vector_label_item_ctor,
        "MenuSysVectorLabelItem_ctor",
    ),
    (
        0x002d_0448,
        menu_sys_vector_label_item_is_selectable,
        "MenuSysVectorLabelItem_IsSelectable",
    ),
    (
        0x002d_0808,
        menu_sys_point2_d_item_ctor,
        "MenuSysPoint2DItem_ctor",
    ),
    (
        0x002d_0908,
        menu_sys_point2d_item_handle_input,
        "MenuSysPoint2DItem_HandleInput",
    ),
    (
        0x002d_0ef8,
        menu_sys_point2d_item_draw,
        "MenuSysPoint2DItem_Draw",
    ),
    (
        0x002d_39f8,
        menu_sys_item_reset_value,
        "MenuSysItem_ResetValue",
    ),
    (0x002d_3a68, menu_sys_bool_item_dtor, "MenuSysBoolItem_dtor"),
    (0x002d_3a98, menu_sys_enum_item_dtor, "MenuSysEnumItem_dtor"),
    (
        0x002d_3ae0,
        menu_sys_submenu_item_dtor,
        "MenuSysSubmenuItem_dtor",
    ),
    (0x002d_3b10, menu_sys_back_item_dtor, "MenuSysBackItem_dtor"),
    (
        0x002d_3b38,
        menu_sys_action_item_dtor,
        "MenuSysActionItem_dtor",
    ),
    (
        0x002d_3b80,
        menu_sys_callback_item_dtor,
        "MenuSysCallbackItem_dtor",
    ),
    (
        0x002d_3bb0,
        menu_sys_label_item_dtor,
        "MenuSysLabelItem_dtor",
    ),
    (
        0x002d_3be0,
        menu_sys_spacer_item_dtor,
        "MenuSysSpacerItem_dtor",
    ),
    (0x002d_3c00, menu_sys_int8_item_dtor, "MenuSysInt8Item_dtor"),
    (0x002d_3c30, menu_sys_int_item_dtor, "MenuSysIntItem_dtor"),
    (
        0x002d_3c60,
        menu_sys_float_item_dtor,
        "MenuSysFloatItem_dtor",
    ),
    (
        0x002d_3ce0,
        menu_sys_float_range_item_dtor,
        "MenuSysFloatRangeItem_dtor",
    ),
    (
        0x002d_3d20,
        menu_sys_text_edit_item_dtor,
        "MenuSysTextEditItem_dtor",
    ),
    (
        0x002d_3d48,
        menu_sys_text_edit_item_set_text,
        "MenuSysTextEditItem_SetText",
    ),
    (
        0x002d_3d70,
        menu_sys_color_channel_item_dtor,
        "MenuSysColorChannelItem_dtor",
    ),
    (
        0x002d_3d90,
        menu_sys_group_item_page_dtor,
        "MenuSysGroupItem_PageDtor",
    ),
    (
        0x002d_3db0,
        menu_sys_group_item_dtor,
        "MenuSysGroupItem_dtor",
    ),
    (
        0x002d_3df8,
        menu_sys_color_swatch_item_dtor,
        "MenuSysColorSwatchItem_dtor",
    ),
    (
        0x002d_3ea8,
        menu_sys_vector_label_item_dtor,
        "MenuSysVectorLabelItem_dtor",
    ),
    (
        0x002d_3f50,
        menu_sys_point2_d_item_dtor,
        "MenuSysPoint2DItem_dtor",
    ),
    (
        0x002d_4008,
        menu_sys_page_sort_compare,
        "MenuSysPage_SortCompare",
    ),
    (
        0x002d_4080,
        menu_sys_debug_printf_4080,
        "MenuSys_DebugPrintf_4080",
    ),
];

/// Same-shard direct callees used above.
pub(crate) mod callee {

    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MENU_SYS_PAGE_SELECT_NEXT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SPRINTF_2 as SPRINTF;

    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2;

    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;

    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;

    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;

    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;

    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_4;

    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED as MENU_SYS_ITEM_IS_DISABLED_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION as MENU_SYS_PAGE_VALIDATE_SELECTION_2;

    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS as MENU_SYS_ITEM_ON_FOCUS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;

    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW_5;

    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;

    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::misc::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_6;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;

    pub const MENU_SYS_BUILD_QUAD: Callee = Callee {
        addr: 0x002c_b350,
        f: ee::s09::MenuSys_BuildQuad,
    };

    pub const MENU_SYS_PAGE_GET_SELECTED_ITEM: Callee = Callee {
        addr: 0x002c_bf30,
        f: ee::s09::MenuSysPage_GetSelectedItem,
    };

    pub const MENU_SYS_FORMAT_FLOAT: Callee = Callee {
        addr: 0x002c_dfd8,
        f: ee::s09::MenuSys_FormatFloat,
    };

    pub const MENU_SYS_ITEM_HANDLE_INPUT_2: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::MenuSysItem_HandleInput,
    };

    pub const MENU_SYS_ITEM_IS_VISIBLE_2: Callee = Callee {
        addr: 0x002c_a378,
        f: ee::s09::MenuSysItem_IsVisible,
    };

    pub const MENU_SYS_ITEM_IS_DISABLED_4: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::MenuSysItem_IsDisabled,
    };

    pub const MENU_SYS_ITEM_GET_INPUT_CODE_2: Callee = Callee {
        addr: 0x002c_a3e8,
        f: ee::s09::MenuSysItem_GetInputCode,
    };

    pub const MENU_SYS_ITEM_GET_LEFT_AXIS: Callee = Callee {
        addr: 0x002c_a408,
        f: ee::s09::MenuSysItem_GetLeftAxis,
    };

    pub const MENU_SYS_ITEM_GET_RIGHT_AXIS: Callee = Callee {
        addr: 0x002c_a428,
        f: ee::s09::MenuSysItem_GetRightAxis,
    };

    pub const MENU_SYS_ITEM_GET_UP_AXIS: Callee = Callee {
        addr: 0x002c_a448,
        f: ee::s09::MenuSysItem_GetUpAxis,
    };

    pub const MENU_SYS_ITEM_GET_DOWN_AXIS: Callee = Callee {
        addr: 0x002c_a468,
        f: ee::s09::MenuSysItem_GetDownAxis,
    };

    pub const MENU_SYS_PAGE_BEGIN_DRAW: Callee = Callee {
        addr: 0x002c_b180,
        f: ee::s09::MenuSysPage_BeginDraw,
    };

    pub const MENU_SYS_PAGE_END_DRAW: Callee = Callee {
        addr: 0x002c_b2f8,
        f: ee::s09::MenuSysPage_EndDraw,
    };

    pub const MENU_SYS_PAGE_DRAW_SCROLL_ARROW_DOWN: Callee = Callee {
        addr: 0x002c_b8c8,
        f: ee::s09::MenuSysPage_DrawScrollArrowDown,
    };

    pub const MENU_SYS_PAGE_DRAW_SCROLL_ARROW_UP: Callee = Callee {
        addr: 0x002c_bac8,
        f: ee::s09::MenuSysPage_DrawScrollArrowUp,
    };

    pub const MENU_SYS_GET_KEY_CODE: Callee = Callee {
        addr: 0x002c_c250,
        f: ee::s09::MenuSys_GetKeyCode,
    };

    pub const MENU_SYS_GET_KEY_CHAR: Callee = Callee {
        addr: 0x002c_c258,
        f: ee::s09::MenuSys_GetKeyChar,
    };

    pub const DEBUG_MENU_ADD_WORLD_IF_MISSING: Callee = Callee {
        addr: 0x0017_a830,
        f: ee::s02::DebugMenu_AddWorldIfMissing,
    };

    pub const MENU_SYS_ITEM_HANDLE_INPUT_3: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::MenuSysItem_HandleInput,
    };

    pub const MENU_SYS_ITEM_IS_DISABLED_5: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::MenuSysItem_IsDisabled,
    };

    pub const MENU_SYS_ITEM_GET_INPUT_CODE_3: Callee = Callee {
        addr: 0x002c_a3e8,
        f: ee::s09::MenuSysItem_GetInputCode,
    };

    pub const MENU_SYS_ITEM_IS_SLOW_MOD_HELD: Callee = Callee {
        addr: 0x002c_a488,
        f: ee::s09::MenuSysItem_IsSlowModHeld,
    };

    pub const MENU_SYS_ITEM_IS_FINE_MOD_HELD: Callee = Callee {
        addr: 0x002c_a4a8,
        f: ee::s09::MenuSysItem_IsFineModHeld,
    };

    pub const MENU_SYS_ITEM_DTOR: Callee = Callee {
        addr: 0x002c_a280,
        f: ee::s09::f_002ca280,
    };

    pub const MENU_SYS_PAGE_DTOR: Callee = Callee {
        addr: 0x002c_aa80,
        f: ee::s09::f_002caa80,
    };

    pub const MENU_SYS_ITEM_CTOR_5: Callee = Callee {
        addr: 0x002c_a258,
        f: ee::s09::f_002ca258,
    };

    pub const MENU_SYS_PAGE_CTOR: Callee = Callee {
        addr: 0x002c_aa58,
        f: ee::s09::f_002caa58,
    };

    pub const MENU_SYS_RESET: Callee = Callee {
        addr: 0x002c_c070,
        f: ee::s09::f_002cc070,
    };

    pub const MENU_SYS_LABEL_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_d2a0,
        f: ee::s09::f_002cd2a0,
    };

    pub const MENU_SYS_SPACER_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_d350,
        f: ee::s09::f_002cd350,
    };

    pub const MENU_SYS_BACK_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_cdf0,
        f: ee::s09::f_002ccdf0,
    };

    pub const MENU_SYS_PAGE_INSERT_ITEM: Callee = Callee {
        addr: 0x002c_bcc8,
        f: ee::s09::f_002cbcc8,
    };

    pub const MENU_SYS_GROUP_ITEM_ADD_ITEM: Callee = Callee {
        addr: 0x002c_f908,
        f: ee::s09::f_002cf908,
    };

    pub const MENU_SYS_GROUP_ITEM_CTOR_2: Callee = Callee {
        addr: 0x002c_f860,
        f: ee::s09::f_002cf860,
    };

    pub const MENU_SYS_COLOR_CHANNEL_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_ed20,
        f: ee::s09::f_002ced20,
    };

    pub const MENU_SYS_COLOR_SWATCH_ITEM_CTOR: Callee = Callee {
        addr: 0x002c_ff28,
        f: ee::s09::f_002cff28,
    };

    pub const MENU_SYS_FLOAT_ITEM_CTOR_2: Callee = Callee {
        addr: 0x002c_dce8,
        f: ee::s09::f_002cdce8,
    };

    pub const MENU_SYS_VECTOR_LABEL_ITEM_CTOR: Callee = Callee {
        addr: 0x002d_03f0,
        f: ee::s09::f_002d03f0,
    };

    pub const ITEM_ON_BLUR: Callee = Callee {
        addr: 0x002c_a370,
        f: ee::s09::f_002ca370,
    };

    pub const ITEM_GET_LEFT_AXIS: Callee = Callee {
        addr: 0x002c_c260,
        f: ee::s09::f_002cc260,
    };

    pub const ITEM_GET_RIGHT_AXIS: Callee = Callee {
        addr: 0x002c_c280,
        f: ee::s09::f_002cc280,
    };

    pub const ITEM_GET_UP_AXIS: Callee = Callee {
        addr: 0x002c_c2a0,
        f: ee::s09::f_002cc2a0,
    };

    pub const ITEM_GET_DOWN_AXIS: Callee = Callee {
        addr: 0x002c_c2c0,
        f: ee::s09::f_002cc2c0,
    };

    pub const ITEM_IS_SLOW_MOD_HELD: Callee = Callee {
        addr: 0x002c_c2e0,
        f: ee::s09::f_002cc2e0,
    };

    pub const ITEM_IS_FINE_MOD_HELD: Callee = Callee {
        addr: 0x002c_c318,
        f: ee::s09::f_002cc318,
    };

    pub const PAGE_ON_DEACTIVATE: Callee = Callee {
        addr: 0x002c_bfd0,
        f: ee::s09::f_002cbfd0,
    };

    pub const INPUT_BINDING_EVAL_FLOAT: Callee = Callee {
        addr: 0x0032_0bf0,
        f: ee::s10::f_00320bf0,
    };

    pub const MENU_SYS_ITEM_HANDLE_INPUT_4: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::f_002ca2b0,
    };

    pub const MENU_SYS_ITEM_IS_DISABLED_6: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::f_002ca3b0,
    };

    pub const MENU_SYS_ITEM_GET_LEFT_AXIS_2: Callee = Callee {
        addr: 0x002c_a408,
        f: ee::s09::f_002ca408,
    };

    pub const MENU_SYS_ITEM_GET_RIGHT_AXIS_2: Callee = Callee {
        addr: 0x002c_a428,
        f: ee::s09::f_002ca428,
    };

    pub const MENU_SYS_ITEM_DRAW_ROW_7: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::f_002ca988,
    };

    pub const MENU_SYS_PAGE_GET_SELECTED_ITEM_2: Callee = Callee {
        addr: 0x002c_bf30,
        f: ee::s09::f_002cbf30,
    };

    pub const MENU_SYS_ITEM_MEASURE_ROW_2: Callee = Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::f_002ca4c8,
    };

    pub const MENU_SYS_PAGE_VALIDATE_SELECTION_3: Callee = Callee {
        addr: 0x002c_aab0,
        f: ee::s09::f_002caab0,
    };

    pub const MENU_SYS_PAGE_REMOVE_ALL_ITEMS: Callee = Callee {
        addr: 0x002c_be10,
        f: ee::s09::f_002cbe10,
    };

    pub const MENU_SYS_PAGE_SELECT_NEXT_3: Callee = Callee {
        addr: 0x002c_ab08,
        f: ee::s09::f_002cab08,
    };

    pub const MENU_SYS_PAGE_SELECT_PREV_2: Callee = Callee {
        addr: 0x002c_abe0,
        f: ee::s09::f_002cabe0,
    };

    pub const MENU_SYS_PAGE_SET_SELECTION_2: Callee = Callee {
        addr: 0x002c_be68,
        f: ee::s09::f_002cbe68,
    };

    pub const MENU_SYS_PAGE_REMOVE_ITEM: Callee = Callee {
        addr: 0x002c_bd50,
        f: ee::s09::f_002cbd50,
    };

    pub const MENU_SYS_GROUP_ITEM_HANDLE_INPUT_2: Callee = Callee {
        addr: 0x002c_fa08,
        f: ee::s09::f_002cfa08,
    };

    pub const MENU_SYS_ITEM_DRAW_ROW_TEXT: Callee = Callee {
        addr: 0x002c_a768,
        f: ee::s09::f_002ca768,
    };
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {}

/// `gp`-relative globals used here.
pub(crate) mod g {}

pub(crate) mod addr {

    pub(crate) use crate::game::bx::bxstringctor::addr::SPRINTF;
}

/// `gp-0x848`: the `World` singleton pointer (global data filled in at
/// startup).
pub(crate) const WORLD_GLOBAL: u32 = 0x848;

/// `World+0x84`: the World's AI manager.
pub(crate) const WORLD_AI_MANAGER: u32 = 0x84;

/// `World+0x84+0x70`: the World's flag manager.
pub(crate) const FLAG_MANAGER: u32 = 0x70;

/// `AIManager+0xc`: the AI currently being processed.
pub(crate) const AI_MANAGER_CURRENT: u32 = 0xc;

/// `AI+0x8`: the AI's frame counter.
pub(crate) const AI_FRAME_COUNT: u32 = 0x8;

// ---------------------------------------------------------------- world

/// `int World_GetAIFrameCount(void)` (0x2d1c98): the current AI's frame
/// counter. `World` at `gp-0x848`, its AI manager at `+0x84`, the current AI
/// at `+0xc`, and the count at `+0x8`.
pub fn world_get_ai_frame_count(c: &mut Cpu) {
    c.cycles += 5;
    let gp = Addr(c.g[GP][0] as u32);
    let world = gp.off(0u32.wrapping_sub(WORLD_GLOBAL)).lw(c);
    c.g[V0][0] = world;
    let ai_manager = Addr(world as u32).off(WORLD_AI_MANAGER).lw(c);
    c.g[V1][0] = ai_manager;
    let ai = Addr(ai_manager as u32).off(AI_MANAGER_CURRENT).lw(c);
    c.g[A0][0] = ai;
    c.g[V0][0] = Addr(ai as u32).off(AI_FRAME_COUNT).lw(c);
    abi::ret(c);
}

/// `FlagManager *World_GetFlagManager(void)` (0x2d1cb0): the World's flag
/// manager (`World+0x84` → `+0x70`).
pub fn world_get_flag_manager(c: &mut Cpu) {
    c.cycles += 4;
    let gp = Addr(c.g[GP][0] as u32);
    let world = gp.off(0u32.wrapping_sub(WORLD_GLOBAL)).lw(c);
    c.g[V0][0] = world;
    let manager = Addr(world as u32).off(WORLD_AI_MANAGER).lw(c);
    c.g[V1][0] = manager;
    c.g[V0][0] = Addr(manager as u32).off(FLAG_MANAGER).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Menu system.

/// `MenuSysItem* MenuSysItem_ctor(MenuSysItem* item, void* data)` (0x2ca258):
/// set the vtable at +0x10, flags at +4, name at +8 and clear +0xc.
pub fn menu_sys_item_ctor(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x480000;
    c.g[V1][0] = 0x1;
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x6f28));
    let item = Addr(abi::arg(c, 0));
    item.off(0x4).sw(c, c.g[V1][0]);
    item.off(0x10).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    item.off(0x8).sw(c, c.g[A1][0]);
    item.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `bool MenuSysItem_IsVisible(MenuSysItem* item)` (0x2ca378): the visible bit.
pub fn menu_sys_item_is_visible(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    c.g[V0][0] &= 0x1;
    abi::ret(c);
}

/// `void MenuSysItem_SetVisible(MenuSysItem* item, bool visible)` (0x2ca388):
/// set or clear the visible bit.
pub fn menu_sys_item_set_visible(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    if c.g[A1][0] == 0 {
        c.cycles += 4;
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        c.g[V0][0] &= c.g[V1][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] |= 0x1;
    }
    Addr(abi::arg(c, 0)).off(0x4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `bool MenuSysItem_IsDisabled(MenuSysItem* item)` (0x2ca3b0): the disabled bit.
pub fn menu_sys_item_is_disabled(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    c.g[V0][0] &= 0x8;
    abi::ret(c);
}

/// `MenuSysPage* MenuSysPage_ctor(MenuSysPage* page)` (0x2caa58): set the
/// vtable at +0x12c and clear the four state words.
pub fn menu_sys_page_ctor(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = 0x480000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x480000u32.wrapping_add(0x6ed0));
    let page = Addr(abi::arg(c, 0));
    page.sw(c, 0);
    page.off(0x12c).sw(c, c.g[V1][0]);
    page.off(0x4).sw(c, 0);
    page.off(0x8).sw(c, 0);
    page.off(0x124).sw(c, 0);
    abi::ret(c);
}

/// `void MenuSysPage_RemoveAllItems(MenuSysPage* page)` (0x2cbe10): clear the
/// item list head/tail.
pub fn menu_sys_page_remove_all_items(c: &mut Cpu) {
    c.cycles += 3;
    let page = Addr(abi::arg(c, 0));
    page.sw(c, 0);
    page.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void MenuSys_Reset(MenuSys* menu)` (0x2cc070): reset the menu state.
pub fn menu_sys_reset(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0xb;
    let menu = Addr(abi::arg(c, 0));
    menu.off(0x50).sw(c, c.g[V1][0]);
    menu.off(0x48).sw(c, c.g[V0][0]);
    menu.off(0x58).sw(c, 0);
    menu.off(0x54).sw(c, 0);
    menu.sw(c, 0);
    menu.off(0x64).sw(c, 0);
    menu.off(0x4c).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World.

/// `int World_GetRiderCount(void)` (0x2d1af0): follow the world singleton to
/// the rider manager and return its count at +0x78.
pub fn world_get_rider_count(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let man = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = man;
    let sub = Addr(man as u32).off(0xc).lw(c);
    c.g[A0][0] = sub;
    c.g[V0][0] = Addr(sub as u32).off(0x78).lw(c);
    abi::ret(c);
}

/// `Rider* World_GetRider(World* w, int index)` (0x2d1b08): look up the rider
/// pointer table entry, returning null when the entry is absent.
pub fn world_get_rider(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c0));
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script hooks.

/// `int WScriptMan_RunObjScript5Hook(int id)` (0x2d1ac8): run object script 5
/// on the current object script manager (`gp+0xce8`).
pub fn wscript_man_run_obj_script5_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0030_a688, 0x002d_1ae0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysItem_GetInputCode(MenuSysItem **this)` (0x002ca3e8): the input
/// code of the menu system at `(*this) + 0x124`.
pub fn menu_sys_item_get_input_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x124)).lw(c);
        call_ptr(c, callee::MENU_SYS_GET_INPUT_CODE, 0x002c_a3fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSys_dtor(MenuSys *this, int flags)` (0x2cc048).
pub fn menu_sys_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002c_c060))
}

/// `int MenuSysPage_GetSelectedIndex(MenuSysPage* self)` (0x2cbf08): validate
/// the page's selection, then read the selected index (+0x4).
pub fn menu_sys_page_get_selected_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION.call(c, 0x002c_bf1c)?;
        c.g[V0][0] = Addr(at(c.g[S0][0] as u32, 4)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MenuSysPoint2DItem_OnFocus(MenuSysPoint2DItem* self)` (0x2d08e0):
/// run the base item's focus handler and clear the item's focus flag (+0x40).
pub fn menu_sys_point2d_item_on_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_ON_FOCUS.call(c, 0x002d_08f4)?;
        Addr(at(c.g[S0][0] as u32, 0x40)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Table and vtable dispatchers.

/// `void ModObj_CallSlot7(ModObj* obj)` (0x2d1b30): call the object's vtable
/// slot 7 (delta at `+0x38`, method at `+0x3c`) with the adjusted `this`.
pub fn mod_obj_call_slot7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        let vt = c.g[V1][0] as u32;
        c.g[A1][0] = Addr(at(vt, 0x38)).lh(c);
        c.g[V0][0] = Addr(at(vt, 0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x002d_1b4c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysItem_dtor` (0x2ca280): vtable at `+0x10`.
pub fn menu_sys_item_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x10, 0x0048_6f28, 0x002c_a2a4);
}

/// `MenuSysPage_dtor` (0x2caa80): vtable at `+0x12c`.
pub fn menu_sys_page_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x12c, 0x0048_6ed0, 0x002c_aaa4);
}

// ---------------------------------------------------------------------------
// Menus.

/// `MenuSysPage_GetSelectedItem(MenuSysPage* page)` (0x2cbf30): select the
/// page's current item first, then return `page->items[selected]` (the word
/// at `page + index*4 + 0xc`).
pub fn menu_sys_page_get_selected_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_bf08, 0x002c_bf44)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_v08(MenuSysGroupItem* item)` (0x2cf8d8): notify the item
/// that its selected page changed (virtual slot `+0x40`/`+0x44` of the
/// selected item's object).
pub fn menu_sys_group_item_v08(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x002c_bf30, 0x002c_f8e8)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x40).lh(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x44).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_f8fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_v10(MenuSysGroupItem* item)` (0x2cfb28): the same for the
/// object at `(*item)->page` (slot `+0x20`/`+0x24` of the vtable at `+0x12c`).
pub fn menu_sys_group_item_v10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x12c).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_fb4c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script system.

/// `WScriptMan_ObjTriggerHook(this, a, b, c)` (0x2d19b8): forward an
/// object-trigger event to the script manager singleton at `gp+0xce8`.
pub fn wscript_man_obj_trigger_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
        c.g[A1][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V1][0];
        call_ptr(c, 0x0030_a060, 0x002d_19dc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu item measurement.

/// `MenuSysItem* Measure(MenuSysItem* self, sItemLayout* layout)`
/// (0x2ccd90 / 0x2ccea8 / 0x2cd2f0): measure the row with the layout's row
/// count and return `self`.
pub(crate) fn menu_item_measure(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x20);
    c.g[A2][0] = 0;
    frame.save(c, S0, 0x10);
    c.g[T0][0] = 0;
    frame.save_ra(c, 0);
    let item = c.g[A0][0];
    c.g[S0][0] = item;
    c.g[A3][0] = Addr(abi::arg(c, 1)).off(0x14).lw(c);
    callee::MENU_ITEM_MEASURE_ROW.call(c, ret)?;
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysSubmenuItem_Measure` (0x2ccd90).
pub fn menu_sys_submenu_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_cdb0))
}

/// `MenuSysBackItem_Measure` (0x2ccea8).
pub fn menu_sys_back_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_cec8))
}

/// `MenuSysLabelItem_Measure` (0x2cd2f0).
pub fn menu_sys_label_item_measure(c: &mut Cpu) {
    guest(c, |c| menu_item_measure(c, 0x002c_d310))
}

// ---------------------------------------------------------------------------
// MenuSys*Item_Measure trampolines.
//
// Each is the same 7-instruction wrapper: `t0` receives the static row layout
// table, `a2` the item's value/count field at `+0x14`, `a3` zero, then
// `MenuSysItem_MeasureRow(row, item)` lays the row out; the item is returned.

/// `void* menu_measure(this, item, table, ret)`.
pub(crate) fn menu_measure(c: &mut Cpu, table: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[T0][0] = sx(table);
        frame.save_ra(c, 0);
        let row = c.g[A0][0];
        c.g[S0][0] = row;
        c.g[A3][0] = 0;
        c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
        callee::MEASURE_ROW.call(c, ret)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* MenuSysCallbackItem_Measure(this, item)` (0x2cd240): lay the item
/// out with the callback-item descriptor table.
pub fn menu_sys_callback_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66c8, 0x002c_d264);
}

/// `void* MenuSysInt8Item_Measure(this, item)` (0x2cd7b8): lay the item out
/// with the integer-item descriptor table.
pub fn menu_sys_int8_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66e0, 0x002c_d7dc);
}

/// `void* MenuSysIntItem_Measure(this, item)` (0x2cdbf0): lay the item out
/// with the integer-item descriptor table.
pub fn menu_sys_int_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66e0, 0x002c_dc14);
}

/// `void* MenuSysFloatItem_Measure(this, item)` (0x2cdfa0): lay the item out
/// with the float-item descriptor table.
pub fn menu_sys_float_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66f0, 0x002c_dfc4);
}

/// `void* MenuSysFloatRangeItem_Measure(this, item)` (0x2ce368): lay the item
/// out with the float-item descriptor table.
pub fn menu_sys_float_range_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66f0, 0x002c_e38c);
}

/// `void MenuSysActionItem_Draw(Item* self, u32 arg)` (0x2cd0e8): draw the
/// row with the item's icon (`+0x18`) or its label (`+0x14`) as the fourth
/// argument.
pub fn menu_sys_action_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[T0][0] = Addr(self_at).off(0x18).lw(c);
        if c.g[T0][0] != 0 {
            c.cycles += 5;
            c.g[A2][0] = Addr(self_at).off(0x14).lw(c);
            c.g[A3][0] = 0;
            callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_d108)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[A3][0] = Addr(self_at).off(0x14).lw(c);
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_d11c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void World_GetRiderPosition(int i)` (0x2d1b58): call the rider slot `i`
/// object's (`race->riders[i]`) slot-5 method with its `+0x6c0` sub-object.
pub fn world_get_rider_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(gp_2(c, -0x848));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::arg(c, 0) << 2);
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x84));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh(r32(c, V1).wrapping_add(0x28));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2c));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, method, 0x002d_1b90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSysItem_IsSelectable(self)` (0x2ca320): visible
/// (`menu_item_is_visible`) and not disabled (`menu_item_is_disabled`).
pub fn menu_sys_item_is_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        callee::MENU_SYS_ITEM_IS_VISIBLE.call(c, 0x002c_a33c)?;
        let hidden = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !hidden {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_ITEM_IS_DISABLED.call(c, 0x002c_a34c)?;
            c.g[S1][0] = (c.g[V0][0] < 1) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_SetRowColor(self, row, on)` (0x2cb880): forward to the
/// row widget's colour method (`self+0x12c` is the vtable; entry 8's delta at
/// `+0x40`, its method at `+0x44`), passing the "on" or "off" colour global.
pub fn menu_sys_page_set_row_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[V1][0] = Addr(self_.wrapping_add(0x12c)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(vtable.wrapping_add(0x40)).lh(c);
        c.g[A3][0] = sx(vtable.wrapping_add(0x44));
        let on = c.g[A2][0] != 0;
        c.g[A0][0] = sx(self_.wrapping_add(abi::r32(c, V0)));
        if on {
            c.cycles += 3;
            c.g[V0][0] = 0x4d_0000;
            c.g[A2][0] = sx(0x4d_0000u32.wrapping_add(0x5380));
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x4d_0000;
            c.g[A2][0] = sx(0x4d_0000u32.wrapping_add(0x5390));
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(abi::r32(c, A3)).lw(c);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x002c_b8bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_OnDeactivate(self)` (0x2cbfd0): deactivate the selected
/// row (`self->items[self->selected]`, its vtable's entry 7) and re-arm the
/// wrap-around flag (`+0x124 = 0`).
pub fn menu_sys_page_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(self_.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        let slot = c.g[V0][0] as u32;
        c.g[V0][0] = sx(self_.wrapping_add(slot));
        let item_slot = c.g[V0][0] as u32;
        c.g[A1][0] = Addr(item_slot.wrapping_add(0xc)).lw(c);
        let item = c.g[A1][0] as u32;
        c.g[V1][0] = Addr(item.wrapping_add(0x10)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vtable.wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr(vtable.wrapping_add(0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(item.wrapping_add(abi::r32(c, A0)));
        call_ptr(c, target, 0x002c_c004)?;
        Addr(self_.wrapping_add(0x124)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysBackItem* MenuSysBackItem_ctor(self, text)` (0x2ccdf0): a menu
/// item (id -1) that keeps `text` at `+0x14` and uses the back-item vtable.
pub fn menu_sys_back_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_ce10)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        Addr(self_.wrapping_add(0x14)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x6d90));
        c.g[V0][0] = c.g[S0][0];
        Addr(self_.wrapping_add(0x10)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysActionItem* MenuSysActionItem_ctor(self)` (0x2ccf08): a menu item
/// (id -1) that uses the action-item vtable and starts with no action, label
/// or child page.
pub fn menu_sys_action_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_cf20)?;
        let self_ = abi::r32(c, S0);
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x6d40));
        Addr(self_.wrapping_add(0x14)).sw(c, 0);
        Addr(self_.wrapping_add(0x10)).sw(c, c.g[V1][0]);
        Addr(self_.wrapping_add(0x1c)).sw(c, 0);
        Addr(self_.wrapping_add(0x20)).sw(c, 0);
        Addr(self_.wrapping_add(0x18)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int WScriptMan_TryRunObjScript3(id)` (0x2d1a30): when the script manager
/// (`gp_3+0xce8`) knows an object script for `id`, run it; returns 1 when it
/// ran.
pub fn wscript_man_try_run_obj_script3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp_3(c, 0xce8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0030_a2e8, 0x002d_1a4c)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = Addr(gp_3(c, 0xce8)).lw(c);
        if !found {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0030_a548, 0x002d_1a64)?;
        c.g[V0][0] = 1;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_ValidateSelection(self)` (0x2caab0): when the selected
/// item is not selectable, move to the next one.
pub fn menu_sys_page_validate_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        c.g[A1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).off(0x10).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x18).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x002c_aae4);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_aae4)?;
        if c.g[V0][0] != 0 {
            // no-op: return
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002c_aaf4);
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_PAGE_SELECT_NEXT.call(c, 0x002c_aaf4)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_EndDraw(void)` (0x2cb2f8): end the current menu's draw,
/// end its draw list and undo the 0x14 draw offset.
pub fn menu_sys_page_end_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002c_b318);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_b318)?;
        c.g[A1][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0xb0).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0xb4).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002c_b330);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_b330)?;
        c.g[A0][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A0)).off(0xe84).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        Addr(p(r32(c, A0), 0xe84)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MenuSysActionItem_Measure(self, item)` (0x2cd090): measure an
/// action item's row with its icon or its text width.
pub fn menu_sys_action_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = Addr(r32(c, A1)).off(0x18).lw(c);
        let has_icon = c.g[T0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if has_icon {
            c.cycles += 5;
            c.g[A2][0] = Addr(r32(c, A1)).off(0x14).lw(c);
            c.g[RA][0] = sx(0x002c_d0b4);
            c.g[A3][0] = 0;
            callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002c_d0b4)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 6;
            c.g[A3][0] = Addr(r32(c, A1)).off(0x14).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x002c_d0d0);
            c.g[T0][0] = 0;
            callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002c_d0d0)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menus (s09).

/// `void MenuSysBoolItem_Draw(MenuSysBoolItem* self, int a1)` (0x2cc758):
/// draw a two-state menu row, picking the on/off variant from the item's
/// image and state fields.
pub fn menu_sys_bool_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[T1][0] = Addr(this).off(0x18).lw(c);
        if c.g[T1][0] == 0 {
            c.g[A3][0] = Addr(this).off(0x14).lw(c);
        } else {
            c.cycles += 3;
            c.g[T0][0] = Addr(this).off(0x1c).lw(c);
            let no_state = c.g[T0][0] == 0;
            c.g[A3][0] = 0;
            if !no_state {
                c.cycles += 7;
                c.g[V1][0] = Addr(this).off(0x20).lw(c);
                c.g[A2][0] = Addr(this).off(0x14).lw(c);
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = c.g[T1][0];
                }
                callee::MENU_SYS_ITEM_DRAW_ROW_2.call(c, 0x002c_c78c)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            c.g[A3][0] = Addr(this).off(0x14).lw(c);
        }
        c.cycles += 4;
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002c_c7a4);
        c.g[T0][0] = 0;
        callee::MENU_SYS_ITEM_DRAW_ROW_2.call(c, 0x002c_c7a4)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// A menu item constructor: `MenuSysItem_ctor(this, -1)`, then the class
/// vtable and its two data words. Shared by the three below.
pub(crate) fn menu_item_ctor_body(
    c: &mut Cpu,
    this: u64,
    vtable_delta: u32,
    first: u64,
    second: u64,
    ret: u32,
) -> crate::call::Flow {
    callee::MENU_SYS_ITEM_CTOR_2.call(c, ret)?;
    c.g[V1][0] = 0x0048_0000;
    Addr(this as u32).off(0x14).sw(c, first);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(vtable_delta));
    Addr(this as u32).off(0x18).sw(c, second);
    Addr(this as u32).off(0x10).sw(c, c.g[V1][0]);
    c.g[V0][0] = this;
    Ok(())
}

/// `MenuSysSubmenuItem* MenuSysSubmenuItem_ctor(this, a1, a2)` (0x2cccc8).
pub fn menu_sys_submenu_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6de0, first, second, 0x002c_ccf0)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysColorChannelItem* MenuSysColorChannelItem_ctor(this, a1, a2)`
/// (0x2ced20).
pub fn menu_sys_color_channel_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6a70, first, second, 0x002c_ed48)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysColorSwatchItem* MenuSysColorSwatchItem_ctor(this, a1, a2)`
/// (0x2cff28).
pub fn menu_sys_color_swatch_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        let (this, first, second) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
        menu_item_ctor_body(c, this, 0x6968, first, second, 0x002c_ff50)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu items.

/// `s2CD3A0Vec MenuSysSpacerItem_Measure(self, item)` (0x2cd3a0): the spacer's
/// height (the int at `item + 0x14`) as a vec4 `{0, 0, 0, (float)h}`.
pub fn menu_sys_spacer_item_measure(c: &mut Cpu) {
    c.cycles += 19;
    c.mtc1(0, 0);
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x10);
    c.g[SP][0] = sx(sp);
    c.g[V0][0] = c.g[A0][0];
    let out = c.g[A0][0] as u32;
    let height = Addr(abi::arg(c, 1)).off(0x14).0;
    c.swc1(0, sp);
    c.swc1(0, sp.wrapping_add(0x8));
    c.swc1(0, sp.wrapping_add(0x4));
    c.lwc1(1, height);
    c.cvt_s_w(1, 1);
    c.swc1(1, sp.wrapping_add(0xc));
    // The original copies the four words of the local vector with
    // ldl/ldr + sdl/sdr pairs; keep the exact (possibly misaligned) ops.
    c.ldl(3, sp.wrapping_add(7));
    c.ldr(3, sp);
    c.ldl(5, sp.wrapping_add(0xf));
    c.ldr(5, sp.wrapping_add(8));
    c.sdl(3, out.wrapping_add(7));
    c.sdr(3, out);
    c.sdl(5, out.wrapping_add(0xf));
    c.sdr(5, out.wrapping_add(8));
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

/// `void MenuSysInt8Item_ResetValue(self)` (0x2cd578): store the item's
/// signed-byte value (`+0x1c`) through the pointer at `+0x18`, or 0 when it is
/// not positive.
pub fn menu_sys_int8_item_reset_value(c: &mut Cpu) {
    c.cycles += 3;
    let item = Addr(abi::arg(c, 0));
    c.g[V0][0] = item.off(0x1c).lb(c);
    let positive = (c.g[V0][0] as i64) > 0;
    c.g[V1][0] = item.off(0x1c).lbu(c);
    if positive {
        c.cycles += 3;
        c.g[V0][0] = item.off(0x18).lw(c);
        let dst = c.g[V0][0] as u32;
        let v = c.g[V1][0];
        Addr(dst).sb(c, v);
    } else {
        c.cycles += 3;
        c.g[V0][0] = item.off(0x18).lw(c);
        let dst = c.g[V0][0] as u32;
        Addr(dst).sb(c, 0);
    }
    abi::ret(c);
}

/// `void MenuSysIntItem_ResetValue(self)` (0x2cd9b8): store the item's int
/// value (`+0x1c`) through the pointer at `+0x18`, or 0 when it is not
/// positive.
pub fn menu_sys_int_item_reset_value(c: &mut Cpu) {
    c.cycles += 3;
    let item = Addr(abi::arg(c, 0));
    c.g[V1][0] = item.off(0x1c).lw(c);
    let positive = (c.g[V1][0] as i64) > 0;
    c.g[V0][0] = item.off(0x18).lw(c);
    let dst = c.g[V0][0] as u32;
    if positive {
        c.cycles += 2;
        let v = c.g[V1][0];
        Addr(dst).sw(c, v);
    } else {
        c.cycles += 2;
        Addr(dst).sw(c, 0);
    }
    abi::ret(c);
}

/// `void MenuSysFloatItem_ResetValue(self)` (0x2cdf78): store the item's
/// float value (`+0x1c`) through the pointer at `+0x18`, or +0.0 when it is
/// not positive.
pub fn menu_sys_float_item_reset_value(c: &mut Cpu) {
    c.cycles += 6;
    let item = Addr(abi::arg(c, 0));
    c.mtc1(0, 1);
    c.lwc1(0, item.off(0x1c).0);
    c.c_le_s(0, 1);
    let positive = !c.fcc();
    c.g[V0][0] = item.off(0x18).lw(c);
    let dst = c.g[V0][0] as u32;
    if positive {
        c.cycles += 2;
        c.swc1(0, dst);
    } else {
        c.cycles += 2;
        c.swc1(1, dst);
    }
    abi::ret(c);
}

/// `cMenuItem* MenuSysActionItem_ctor_3(cMenuItem* self, char* text, int a2,
/// int a3)` (0x2CCF98): the base constructor plus the action item's vtable and
/// two fields.
pub fn menu_sys_action_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        callee::MENU_SYS_ITEM_CTOR_3.call(c, 0x002c_cfbc)?;
        c.g[V1][0] = 0x480000;
        c.sw(ea(c.g[S0][0], 0x14), c.g[S1][0]);
        c.g[V1][0] = sx(ea(c.g[V1][0], 0x6d40));
        c.sw(ea(c.g[S0][0], 0x1c), c.g[S2][0]);
        c.sw(ea(c.g[S0][0], 0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(ea(c.g[S0][0], 0x20), 0);
        c.sw(ea(c.g[S0][0], 0x18), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysInt8Item_Draw(MenuSysInt8Item* self, void* gfx)` (0x2CD7F0):
/// format the byte value at `*(self+0x18)` into a stack buffer, then draw the
/// row.
pub fn menu_sys_int8_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x18));
        c.g[A1][0] = sx(ea(c.g[A1][0], 0x3970));
        c.g[A2][0] = c.lb(c.g[V0][0] as u32);
        call_ptr(c, addr::SPRINTF, 0x002c_d820)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(ea(c.g[A0][0], 0x14));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW_3.call(c, 0x002c_d838)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysIntItem_Draw(MenuSysIntItem* self, void* gfx)` (0x2CDC28):
/// the same for the 32-bit value at `*(self+0x18)`.
pub fn menu_sys_int_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x18));
        c.g[A1][0] = sx(ea(c.g[A1][0], 0x3970));
        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, addr::SPRINTF, 0x002c_dc58)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.lw(ea(c.g[A0][0], 0x14));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW_3.call(c, 0x002c_dc70)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSys_Draw(item)` 0x2cc350: compute the item's scaled size into its
/// object (`+0x38`/`+0x3c`) and forward to the draw method of the item's
/// selected object.
pub fn menu_sys_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        c.lwc1(1, gp_4(c, -0x3ff4));
        frame.save_ra(c, 0x10);
        let obj = lw_at(c, this, 0x58) as u32;
        c.g[V1][0] = obj as u64;
        c.swc1(1, p(sp(c), 4));
        c.swc1(1, sp(c));
        c.lwc1(0, p(obj, 0x30));
        c.lwc1(2, p(obj, 0x34));
        c.mul_s(0, 0, 1);
        c.swc1(0, p(obj, 0x38));
        c.lwc1(1, p(sp(c), 4));
        c.mul_s(2, 2, 1);
        c.swc1(2, p(obj, 0x3c));
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, V0)));
        c.g[A1][0] = Addr(p(r32(c, A0), 4)).lw(c);
        let target = Addr(p(r32(c, A1), 0x12c)).lw(c) as u32;
        c.g[V0][0] = target as u64;
        c.g[A0][0] = Addr(p(target, 0x28)).lh(c);
        c.g[V1][0] = lw_at(c, target, 0x2c);
        let func = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, func, 0x002c_c3ac)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system.

/// The shared handler of the back and callback menu items: run the base
/// input handler, then (when enabled and not already consumed) pop the page
/// for the "back" code, or run the item's callback.
pub(crate) fn menu_item_handle_input(c: &mut Cpu, entry: u32, callback: bool) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, 0x002c_a2b0, entry + 0x18)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002c_a3b0, entry + 0x24)?;
    let disabled = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[S1][0];
    let mut restored_early = false;
    if !disabled {
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            frame.restore(c, S0, 0x20);
            restored_early = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_a3e8, entry + 0x3c)?;
            c.g[V1][0] = 6;
            let not_back = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = c.g[S1][0];
            if !not_back {
                c.cycles += 4;
                if callback {
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
                    let target = c.g[V0][0] as u32;
                    call_ptr(c, target, entry + 0x54)?;
                    c.g[V0][0] = c.g[S1][0];
                } else {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x124)).lw(c);
                    call_ptr(c, 0x002c_c460, entry + 0x54)?;
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
    }
    if !restored_early {
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysBackItem::HandleInput(self)` (0x2cce38): pop the current page when
/// the "back" input arrives.
pub fn menu_sys_back_item_handle_input(c: &mut Cpu) {
    guest(c, |c| menu_item_handle_input(c, 0x002c_ce38, false))
}

/// `MenuSysCallbackItem::HandleInput(self)` (0x2cd1d0): call the item's
/// callback when the "back" input arrives.
pub fn menu_sys_callback_item_handle_input(c: &mut Cpu) {
    guest(c, |c| menu_item_handle_input(c, 0x002c_d1d0, true))
}

/// `MenuSysTextEditItem* MenuSysTextEditItem_ctor(self, a1, name)`
/// (0x2ce418): base menu item plus the text-edit vtable and the name copy.
pub fn menu_sys_text_edit_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = u64::MAX;
        call_ptr(c, 0x002c_a258, 0x002c_e440)?;
        c.g[V1][0] = 0x480000;
        Addr(p(c.g[S0][0] as u32, 0x14)).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ac0));
        Addr(p(c.g[S0][0] as u32, 0x28)).sw(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x10)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x18));
        c.g[A2][0] = 0xf;
        call_ptr(c, 0x0041_6ce0, 0x002c_e464)?;
        c.g[V0][0] = c.g[S0][0];
        Addr(p(c.g[S0][0] as u32, 0x27)).sb(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysItem_HandleInput(MenuSysItem* item)` (0x2ca2b0): map the
/// current input code to an item action (8: 1, 7: 2, 0: 4, 1: 3; otherwise 0).
pub fn menu_sys_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_ITEM_GET_INPUT_CODE.call(c, 0x002c_a2c0)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let code_is_1 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let result: u64;
        if code_is_1 {
            // b_2ca310 -> +1, v0 = 3 -> b_2ca314
            c.cycles += 1;
            result = 3;
        } else if c.g[V0][0] == 0 {
            // code >= 2: fall into b_2ca300 with v0 = 7.
            c.cycles += 2;
            c.g[V0][0] = 7;
            c.cycles += 2;
            let is_7 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 8;
            if is_7 {
                c.cycles += 2;
                result = 2;
            } else {
                c.cycles += 2;
                let not_8 = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if not_8 {
                    result = 0;
                } else {
                    c.cycles += 2;
                    result = 1;
                }
            }
        } else {
            // code == 0: b_2ca308 -> v0 = 4
            c.cycles += 2;
            c.g[V0][0] = 7;
            c.cycles += 2;
            c.cycles += 2;
            result = 4;
        }
        c.g[V0][0] = result;
        c.cycles += 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysPage_OnActivate(MenuSysPage* page, int wrap)` (0x2cbf60):
/// store the wrap flag, clamp the selection to the last item, activate the
/// selected item (virtual `+0x34` with argument 2) and validate the selection.
pub fn menu_sys_page_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p_2(r32(c, S0), 0x124)).sw(c, c.g[A1][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(p_2(r32(c, S0), 4)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
        }
        c.cycles += 15;
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = abi::addu(c.g[S0][0], c.g[V0][0]);
        c.g[A2][0] = Addr(p_2(c.g[V0][0] as u32, 0xc)).lw(c);
        c.g[V1][0] = Addr(p_2(r32(c, A2), 0x10)).lw(c);
        c.g[A0][0] = Addr(p_2(c.g[V1][0] as u32, 0x30)).lh(c);
        c.g[V0][0] = Addr(p_2(c.g[V1][0] as u32, 0x34)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        call::call_ptr(c, target, 0x002c_bfb8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_PAGE_VALIDATE_SELECTION.call(c, 0x002c_bfc0)?;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `int MenuSysSubmenuItem_HandleInput(MenuSysSubmenuItem* item)`
/// (0x2ccd20): run the base item input, and when the item is enabled, produced
/// no action and the input code is 6 (confirm), push the submenu page named
/// at `item+0x18`.
pub fn menu_sys_submenu_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_ITEM_HANDLE_INPUT.call(c, 0x002c_cd38)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_ITEM_IS_DISABLED.call(c, 0x002c_cd44)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        let mut restore_16 = false;
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                restore_16 = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_ITEM_GET_INPUT_CODE.call(c, 0x002c_cd5c)?;
                c.g[V1][0] = 6;
                let not_confirm = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[S1][0];
                if !not_confirm {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A1][0] = Addr(p_2(r32(c, S0), 0x18)).lw(c);
                    c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 0x124)).lw(c);
                    callee::MENU_PUSH_PAGE.call(c, 0x002c_cd78)?;
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        if !restore_16 {
            c.cycles += 1;
        }
        lqsp(c, 16, 0x20);
        c.cycles += 4;
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysFloatItem_Draw(MenuSysFloatItem* item, int a1)` (0x2ce100):
/// format `*item->value * item->scale + item->bias` into a stack string, draw
/// the row, and destroy the temporary string.
pub fn menu_sys_float_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let _frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sdsp(c, 0x10, c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p_2(r32(c, S0), 0x18)).lw(c);
        c.lwc1(1, p_2(r32(c, S0), 0x24));
        c.lwc1(12, c.g[V0][0] as u32);
        c.lwc1(0, p_2(r32(c, S0), 0x28));
        c.mul_s(12, 12, 1);
        c.add_s(12, 12, 0);
        callee::MENU_FORMAT_FLOAT.call(c, 0x002c_e138)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = Addr(c.g[SP][0] as u32).lw(c);
        c.g[A2][0] = Addr(p_2(r32(c, A0), 0x14)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::MENU_ITEM_DRAW_ROW.call(c, 0x002c_e150)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call::call_ptr(c, 0x0031_8260, 0x002c_e15c)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        c.g[RA][0] = Addr(p_2(c.g[SP][0] as u32, 0x10)).ld(c);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `void MenuSysFloatRangeItem_Draw(self, row)` (0x2ce3a0): format the item's
/// float (`*(self+0x18)` scaled by 180/pi) into a 0x70-byte buffer and draw
/// the row.
pub fn menu_sys_float_range_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0xb0));
        c.lwc1(0, gpo(c, -0x3fb4));
        c.sq(16, sp_2(c).wrapping_add(0xa0));
        c.sq(17, sp_2(c).wrapping_add(0x90));
        c.g[S0][0] = 0x4a_0000;
        c.sq(18, sp_2(c).wrapping_add(0x80));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp_2(c).wrapping_add(0x70), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = sx(low(c, S0).wrapping_add(0x39c8));
        c.g[V0][0] = lw_r(c, S1, 0x18);
        c.lwc1(12, low(c, V0));
        c.mul_s(12, 12, 0);
        call_ptr(c, callee::CVT_FLOAT_TO_INT, 0x002c_e3d8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::SPRINTF, 0x002c_e3e8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = lw_r(c, A0, 0x14);
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_DRAW_ROW_4.call(c, 0x002c_e400)?;
        c.lq(16, sp_2(c).wrapping_add(0xa0));
        c.lq(17, sp_2(c).wrapping_add(0x90));
        c.lq(18, sp_2(c).wrapping_add(0x80));
        c.g[RA][0] = Addr(sp_2(c).wrapping_add(0x70)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysGroupItem_OnFocus(self, mode)` (0x002cfe78): select the
/// group's page (0 = first, 1 = last) and activate it.
pub fn menu_sys_group_item_on_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_ON_FOCUS_2.call(c, 0x002c_fe94)?;
        let mut select: Option<u64> = None;
        let s0 = c.g[S0][0];
        c.g[V0][0] = 0x1;
        if s0 == 0 {
            c.cycles += 4;
            c.g[S0][0] = sx(word(c, S1).wrapping_add(0x18));
            select = Some(0);
        } else {
            c.cycles += 2;
            c.g[A0][0] = sx(word(c, S1).wrapping_add(0x18));
            if s0 == 1 {
                c.cycles += 3;
                c.g[S0][0] = sx(word(c, S1).wrapping_add(0x18));
                select = Some(0xffffffffffffffff);
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(at_2(c, S1, 0)).lw(c);
            }
        }
        if let Some(arg) = select {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = arg;
            callee::MENU_SYS_PAGE_SET_SELECTION.call(c, 0x002c_fed0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = Addr(at_2(c, S1, 0)).lw(c);
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x002c_fee0);
        c.g[A1][0] = Addr(at_2(c, V0, 0x124)).lw(c);
        callee::MENU_SYS_PAGE_ON_ACTIVATE.call(c, 0x002c_fee0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysIntItem* MenuSysIntItem_ctor_3(MenuSysIntItem* p, int a1, int a2,
/// int a3, int t0, int t1)` (0x002cd8f8): base menu item, class vtable, then
/// the extra fields.
pub fn menu_sys_int_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
        c.sq(S0, sp(c).wrapping_add(0x50));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S1, sp(c).wrapping_add(0x40));
        c.sq(S2, sp(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[T1][0];
        c.sq(S3, sp(c).wrapping_add(0x20));
        c.g[S2][0] = c.g[A2][0];
        c.sq(S4, sp(c).wrapping_add(0x10));
        c.g[S3][0] = c.g[A3][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S4][0] = c.g[T0][0];
        call_ptr(c, 0x002c_a258, 0x002c_d92c)?; // MenuSysItem_ctor
        c.g[V1][0] = sx(0x0048_6bb0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
        c.lq(S0, sp(c).wrapping_add(0x50));
        c.lq(S1, sp(c).wrapping_add(0x40));
        c.lq(S2, sp(c).wrapping_add(0x30));
        c.lq(S3, sp(c).wrapping_add(0x20));
        c.lq(S4, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
        abi::ret(c);
        Ok(())
    })
}

/// `void MenuSysColorSwatchItem_Measure(MenuSysColorSwatchItem* item,
/// MenuSysItem* row)` (0x002cff88): measure the base row, pad its height by 9
/// and floor it at 29, then store the 16-byte extent into the item.
pub fn menu_sys_color_swatch_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A3][0] = 0;
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[T0][0] = 0;
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x002c_a4c8, 0x002c_ffac)?; // MenuSysItem_MeasureRow
        c.lwc1(2, sp(c).wrapping_add(8));
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4210_0000;
        c.mtc1(1, 3);
        c.lwc1(1, sp(c).wrapping_add(0xc));
        c.add_s(2, 2, 3);
        c.g[AT][0] = 0x41e8_0000;
        c.mtc1(1, 0);
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.max_s(1, 1, 0);
        c.swc1(2, sp(c).wrapping_add(8));
        c.swc1(1, sp(c).wrapping_add(0xc));
        c.ldl(3, sp(c).wrapping_add(7));
        c.ldr(3, sp(c));
        c.ldl(4, sp(c).wrapping_add(0xf));
        c.ldr(4, sp(c).wrapping_add(8));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(7));
        c.sdr(3, c.g[S0][0] as u32);
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(8));
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void MenuSysPage_HandleInput(page)` (0x2cacc0): let the selected item
/// handle its input, then move the selection for the "next"/"previous" codes
/// (3 and 4); returns 0 for the unhandled case.
pub fn menusyspage_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION_2.call(c, 0x002c_acd4)?;
        let base = r32(c, S0);
        c.g[V0][0] = lw_at_2(c, p(base, 4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = lw_at_2(c, p(c.g[V0][0] as u32, 0xc));
        c.g[V1][0] = lw_at_2(c, p(c.g[A1][0] as u32, 0x10));
        let func = vslot(c, c.g[V1][0], 0x10, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x002c_acf8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 3;
        let next = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 4;
        let mut handled = true;
        if next {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_PAGE_SELECT_NEXT_2.call(c, 0x002c_ad20)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            let prev = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if prev {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_PAGE_SELECT_PREV.call(c, 0x002c_ad30)?;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 2;
                frame.restore(c, 16, 0x10);
                handled = false;
            }
        }
        if handled {
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_InsertItem(page, item, index)` (0x2cbcc8): insert the
/// item into the page's item array at `index`, shifting the rest up, and
/// keep the item count and the selected index consistent.
pub fn menusyspage_insert_item(c: &mut Cpu) {
    c.cycles += 2;
    let item = c.g[A0][0];
    let page = c.g[A1][0];
    let mut index = c.g[A2][0];
    let given = (index as i64) >= 0;
    c.sw(page as u32, item);
    if !given {
        c.cycles += 1;
        index = lw_at_2(c, item as u32);
        c.g[A2][0] = index;
    }
    c.g[A2][0] = index;
    c.cycles += 5;
    let count = lw_at_2(c, item as u32);
    c.g[T0][0] = count;
    c.g[T1][0] = sx((item as u32).wrapping_add(0xc));
    c.g[V0][0] = ((index as i64) < (count as i64)) as u64;
    let fits = c.g[V0][0] != 0;
    c.g[T2][0] = sx((index as u32) << 2);
    if fits {
        c.cycles += 2;
        c.g[V0][0] = sx((count as u32) << 2);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
        loop {
            c.cycles += 7;
            c.g[V0][0] = lw_at_2(c, (c.g[A3][0] as u32).wrapping_sub(4));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = ((index as i64) < (c.g[T0][0] as i64)) as u64;
            c.sw(c.g[A3][0] as u32, c.g[V0][0]);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(4));
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_bcf0);
            }
        }
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
    c.g[V1][0] = lw_at_2(c, p(item as u32, 0x124));
    if c.g[V1][0] == 0 {
        c.g[V0][0] = lw_at_2(c, item as u32);
    } else {
        c.cycles += 4;
        c.g[V1][0] = lw_at_2(c, p(item as u32, 4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (index as i64)) as u64;
        let after = c.g[V0][0] != 0;
        c.g[V0][0] = lw_at_2(c, item as u32);
        if !after {
            c.cycles += 2;
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if !empty {
                c.cycles += 1;
                c.sw(p(item as u32, 4), c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = lw_at_2(c, item as u32);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(item as u32, c.g[V0][0]);
    abi::ret(c);
}

/// `void MenuSysActionItem_HandleInput(item)` (0x2cd008): the action item's
/// input: run the item input, then the action when it was triggered.
pub fn menusysactionitem_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_HANDLE_INPUT.call(c, 0x002c_d020)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED_2.call(c, 0x002c_d02c)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        let mut skip_one = false;
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                frame.restore(c, 16, 0x20);
                skip_one = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_d044)?;
                c.g[V1][0] = 6;
                let other = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[S1][0];
                if other {
                } else {
                    c.cycles += 10;
                    c.g[A3][0] = lw_at_2(c, c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = lw_at_2(c, p(c.g[A1][0] as u32, 0x1c));
                    c.g[V1][0] = lw_at_2(c, p(c.g[A3][0] as u32, 0x12c));
                    let func = vslot(c, c.g[V1][0], 0x18, V0);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, func, 0x002c_d070)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu items.

/// The entry point and the loop head (where an event resumes) of
/// [`menu_sys_enum_item_draw`].
pub(crate) const ENUM_DRAW_ENTRY: u32 = 0x002c_cc38;

pub(crate) const ENUM_DRAW_HEAD: u32 = 0x002c_cc58;

/// `void MenuSysEnumItem_Draw(MenuSysEnumItem* self, void* a1)` (0x2ccc38):
/// walk the item's enum table and draw the row whose value matches, or the
/// item's own last row when there is no match.
///
/// The original is a `match pc` dispatcher: its loop head is a resume point
/// (an event delivered at the back edge returns and the trampoline re-enters
/// the function there).
pub fn menu_sys_enum_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENUM_DRAW_ENTRY => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[T1][0] = c.g[A1][0];
                    c.g[A3][0] = 0;
                    c.g[T2][0] = 0x0048_0000;
                    pc = 0x002c_cc94;
                }
                0x002c_cc54 => {
                    c.cycles += 1;
                    pc = ENUM_DRAW_HEAD;
                }
                ENUM_DRAW_HEAD => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, A3) << 3);
                    c.g[V1][0] = Addr(p(r32(c, A2), 0x20)).lw(c);
                    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = Addr(p(r32(c, T0), 4)).lw(c);
                    let mismatch = c.g[A0][0] != c.g[V0][0];
                    c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                    pc = if mismatch { 0x002c_cc94 } else { 0x002c_cc74 };
                }
                0x002c_cc74 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A2][0];
                    c.g[T0][0] = Addr(r32(c, T0)).lw(c);
                    c.g[A2][0] = Addr(p(r32(c, A0), 0x14)).lw(c);
                    c.g[A1][0] = c.g[T1][0];
                    c.g[A3][0] = 0;
                    callee::MENU_SYS_ITEM_DRAW_ROW_5.call(c, 0x002c_cc8c)?;
                    c.g[RA][0] = Addr(r32(c, SP)).ld(c);
                    pc = 0x002c_ccbc;
                }
                0x002c_cc94 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
                    if c.g[A3][0] != c.g[V0][0] {
                        c.g[A1][0] = Addr(p(r32(c, A2), 0x1c)).lw(c);
                        if crate::events::back_edge_unwinds(c, ENUM_DRAW_HEAD) {
                            return Ok(());
                        }
                        pc = ENUM_DRAW_HEAD;
                    } else {
                        pc = 0x002c_cca0;
                    }
                }
                0x002c_cca0 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[A2][0];
                    c.g[A1][0] = c.g[T1][0];
                    c.g[A2][0] = Addr(p(r32(c, A0), 0x14)).lw(c);
                    c.g[T0][0] = sx(r32(c, T2).wrapping_add(0x66b8));
                    c.g[A3][0] = 0;
                    callee::MENU_SYS_ITEM_DRAW_ROW_5.call(c, 0x002c_ccb8)?;
                    c.g[RA][0] = Addr(r32(c, SP)).ld(c);
                    pc = 0x002c_ccbc;
                }
                0x002c_ccbc => {
                    c.cycles += 2;
                    let ra = r32(c, RA);
                    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENUM_DRAW_ENTRY, 0x002c_ccc4);
                    return Ok(());
                }
            }
        }
    })
}

/// `cFloatMenuItem* cFloatMenuItem_ctor(cFloatMenuItem* self, void* text,
/// int a2, int a3, float f1, float f2)` (0x2cdce8): base constructor, then the
/// float item's fields, vtable and unit scale.
pub fn menu_sys_float_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        c.swc1(21, p(r32(c, SP), 0x48));
        c.g[S2][0] = c.g[A2][0];
        c.swc1(20, p(r32(c, SP), 0x40));
        c.mov_s(21, 13);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        callee::MENU_SYS_ITEM_CTOR_4.call(c, 0x002c_dd1c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V1][0] = 0x0048_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x6b60));
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[S1][0]);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        c.swc1(20, p(r32(c, S0), 0x1c));
        c.swc1(21, p(r32(c, S0), 0x20));
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[V1][0]);
        c.swc1(0, p(r32(c, S0), 0x24));
        Addr(p(r32(c, S0), 0x30)).sw(c, 0);
        Addr(p(r32(c, S0), 0x2c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x28)).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.lwc1(21, p(r32(c, SP), 0x48));
        c.lwc1(20, p(r32(c, SP), 0x40));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu pages.

/// `void MenuSysPage_SetSelection(MenuSysPage* page, int index)` (0x2cbe68):
/// normalise a negative index by the item count, and when the index changed,
/// run the old item's deselect method, store the new index, run the new item's
/// select method and validate the page.
pub fn menu_sys_page_set_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        let non_negative = (c.g[S1][0] as i64) >= 0;
        frame.save_ra(c, 0);
        if !non_negative {
            c.cycles += 2;
            c.g[V0][0] = lw(c, c.g[S2][0] as u32);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, p(c.g[S2][0] as u32, 4));
        let same = c.g[S1][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !same {
            c.cycles += 20;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = lw(c, c.g[V0][0] as u32);
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 0x10));
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x38)).lh(c);
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x3c));
            c.g[V0][0] = sx(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x002c_bebc)?;
            sw(c, p(c.g[S2][0] as u32, 4), c.g[S1][0]);
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = 2;
            c.g[A2][0] = lw(c, c.g[S0][0] as u32);
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x30)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x34));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x002c_bee4)?;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_aab0, 0x002c_beec)?; // MenuSysPage_ValidateSelection
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MenuSys_PopPage(self)` (0x2cc460): dispose the top page and, if one
/// remains, activate its parent; otherwise clear the menu's active flag.
pub fn menu_sys_pop_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this.wrapping_add(4));
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(this).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = lw_2(c, A1, 0x12c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
        c.g[V1][0] = lw_2(c, V0, 0x3c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_c4a4)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !empty {
            c.cycles += 12;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = lw_2(c, A3, 0x12c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[A2][0] = lw_2(c, V1, 0x34);
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002c_c4d4)?;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::MENU_SYS_SET_ACTIVE.call(c, 0x002c_c4ec)?;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: push a page item.

/// `void MenuSys_PushPage(MenuSys* menu, item)` (0x2cc3b8): notify the item
/// below the page, append the item to the page's array, notify it with the new
/// index and mark the menu active.
pub fn menu_sys_push_page(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 8;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_3(c, c.g[S0][0] as u32);
        let t = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !t {
            c.cycles += 9;
            c.g[V0][0] = sx(at_4(c.g[V0][0], 0xffff_ffff));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = lw_3(c, at_4(c.g[V0][0], 4));
            c.g[V1][0] = lw_3(c, at_4(c.g[A1][0], 0x12c));
            c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0x38));
            c.g[V0][0] = lw_3(c, at_4(c.g[V1][0], 0x3c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x002c_c3fc);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002c_c3fc)?;
        }
        c.cycles += 25;
        c.g[A0][0] = lw_3(c, c.g[S0][0] as u32);
        c.g[A2][0] = sx(at_4(c.g[S0][0], 4));
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx(at_4(c.g[A0][0], 1));
        Addr(c.g[V0][0] as u32).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(at_4(c.g[A0][0], 0xffff_ffff));
        Addr(c.g[S0][0] as u32).sw(c, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A3][0] = lw_3(c, c.g[A2][0] as u32);
        c.g[V0][0] = lw_3(c, at_4(c.g[A3][0], 0x12c));
        c.g[A0][0] = lh(c, at_4(c.g[V0][0], 0x30));
        c.g[V1][0] = lw_3(c, at_4(c.g[V0][0], 0x34));
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x002c_c440);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_c440)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x002c_c44c);
        c.g[A1][0] = 1;
        callee::MENU_SYS_SET_ACTIVE_2.call(c, 0x002c_c44c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: vector-label item draw.

/// `void MenuSysVectorLabelItem_Draw(item, ctx)` (0x2d0500): convert the
/// label's position to three doubles for `sprintf`, then draw the row.
pub fn menu_sys_vector_label_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xe0);
        c.cycles += 42;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = sx(0x0048_0000);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0xa0);
        c.g[S0][0] = sx(0x0048_6720);
        frame.save(c, S4, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S4][0] = c.g[A1][0];
        c.g[V0][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[RA][0] = sx(0x002d_0538);
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x0041_3af8, 0x002d_0538)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0548);
        c.lwc1(12, at_4(c.g[V1][0], 4));
        call_ptr(c, 0x0041_3af8, 0x002d_0548)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0558);
        c.lwc1(12, at_4(c.g[V1][0], 8));
        call_ptr(c, 0x0041_3af8, 0x002d_0558)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[RA][0] = sx(0x002d_0570);
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x002d_0570)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = lw_3(c, at_4(c.g[A0][0], 0x18));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x002d_0588);
        c.g[T0][0] = sx(frame.at(c, 0));
        callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002d_0588)?;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: vector-label item measure.

/// `void MenuSysVectorLabelItem_Measure(item, ctx)` (0x2d0450): format the
/// label's position and measure the row.
pub fn menu_sys_vector_label_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xe0);
        c.cycles += 43;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = sx(0x0048_0000);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0xa0);
        c.g[S0][0] = sx(0x0048_6720);
        frame.save(c, S4, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S4][0] = c.g[A0][0];
        c.g[V0][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[RA][0] = sx(0x002d_0488);
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x0041_3af8, 0x002d_0488)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0498);
        c.lwc1(12, at_4(c.g[V1][0], 4));
        call_ptr(c, 0x0041_3af8, 0x002d_0498)?;
        c.g[V1][0] = lw_3(c, at_4(c.g[S1][0], 0x14));
        c.g[S2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_04a8);
        c.lwc1(12, at_4(c.g[V1][0], 8));
        call_ptr(c, 0x0041_3af8, 0x002d_04a8)?;
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x002d_04c0);
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x002d_04c0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = lw_3(c, at_4(c.g[A1][0], 0x18));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x002d_04d8);
        c.g[T0][0] = sx(frame.at(c, 0));
        callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002d_04d8)?;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysBoolItem_HandleInput(MenuSysBoolItem *self)` (0x2cc578):
/// toggles the item's bool when the handled input is confirm (2/3) or the
/// item's own code is 6 (and the item is enabled), then notifies the owner.
pub fn menu_sys_bool_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_HANDLE_INPUT.call(c, 0x002c_c590)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED_3.call(c, 0x002c_c59c)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        if !disabled {
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                c.lq(16, frame.at(c, 0x20));
            } else {
                let mut toggle = false;
                let mut value_ptr = 0u64;
                c.cycles += 3;
                if Addr(p_3(c.g[S0][0], 0x18)).lw(c) != 0 {
                    c.cycles += 3;
                    if Addr(p_3(c.g[S0][0], 0x1c)).lw(c) != 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5cc)?;
                        c.g[V1][0] = 0x2;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x20)).lw(c);
                            value_ptr = c.g[V1][0];
                            toggle = true;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5e0)?;
                            c.g[V1][0] = 0x3;
                            if c.g[V0][0] == c.g[V1][0] {
                                c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x20)).lw(c);
                                value_ptr = c.g[V1][0];
                                toggle = true;
                            }
                        }
                    }
                }
                if !toggle {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MENU_SYS_ITEM_GET_INPUT_CODE.call(c, 0x002c_c5f4)?;
                    c.g[V1][0] = 0x6;
                    let other = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = c.g[S1][0];
                    if !other {
                        c.cycles += 1;
                        c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x20)).lw(c);
                        value_ptr = c.g[V1][0];
                        toggle = true;
                    }
                }
                if toggle {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S0][0];
                    let value = Addr(value_ptr as u32).lw(c) ^ 1;
                    Addr(value_ptr as u32).sw(c, value);
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = Addr(p_3(c.g[A2][0], 0x12c)).lw(c);
                    c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x20)).lh(c);
                    c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002c_c62c)?;
                    c.g[V0][0] = c.g[S1][0];
                }
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x20));
            }
        } else {
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_SelectNext(MenuSysPage* self)` (0x2cab08): advance the
/// page cursor modulo the item count and select the next item that accepts
/// input; when the cursor wraps back to the start, do nothing.
pub fn menu_sys_page_select_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = sx(p(r32(c, S0), 0xc));
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
        'page: loop {
            c.cycles += 5;
            c.g[V1][0] = Addr(r32(c, S0)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.div(0, 2, 3);
            c.cycles += 3;
            c.g[V0][0] = c.hi[0];
            let wrapped = c.g[V0][0] == c.g[S1][0];
            Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
            if wrapped {
                break;
            }
            c.cycles += 11;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[S2][0] = sx(p(r32(c, S0), 0xc));
            c.g[V0][0] = sx(p(r32(c, S3), r32(c, V0)));
            c.g[A1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A1), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
            let accepts = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
            call_ptr(c, accepts, 0x002c_ab74)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_ab30);
                }
                continue 'page;
            }
            c.cycles += 18;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
            c.g[A1][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
            let sel = r32(c, A1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, sel, 0x002c_ab9c)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 0;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
            let hov = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, hov, 0x002c_abc4)?;
            break;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu page navigation (previous).

/// `void MenuSysPage_SelectPrev(MenuSysPage* self)` (0x2cabe0): move the page
/// cursor back modulo the item count and select the previous valid item.
pub fn menu_sys_page_select_prev(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = sx(p(r32(c, S0), 0xc));
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        'page: loop {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.cycles += 6;
            c.g[V1][0] = sx(p(r32(c, V1), r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            c.div(0, 3, 2);
            c.g[V0][0] = c.hi[0];
            let wrapped = c.g[V0][0] == c.g[S1][0];
            Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
            if wrapped {
                break;
            }
            c.cycles += 11;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[S2][0] = sx(p(r32(c, S0), 0xc));
            c.g[V0][0] = sx(p(r32(c, S3), r32(c, V0)));
            c.g[A1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A1), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
            let accepts = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
            call_ptr(c, accepts, 0x002c_ac50)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_ac08);
                }
                continue 'page;
            }
            c.cycles += 18;
            c.g[V0][0] = sx(r32(c, S1) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
            c.g[A1][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
            let sel = r32(c, A1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, sel, 0x002c_ac78)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 1;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
            let hov = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, hov, 0x002c_aca0)?;
            break;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSys_BuildQuad(sVtx* v, const float* r, const float* t)`
/// (0x2cb350): fill a menu quad's vertices from the rectangle (`r` = x, y,
/// w, h) and the transform (`t` = x, y, z, w), then hand it to the draw
/// module's submit routine (slot at `*(D_004A289C+0x10d8) + 0x23c`, `this`
/// adjusted by the slot's `lh` delta).
pub fn menu_sys_build_quad(c: &mut Cpu) {
    c.cycles += 82;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V1][0] = c.g[A1][0];
    c.std(SP, 0x10, c.g[RA][0]);
    c.g[T0][0] = c.g[A2][0];
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A2][0] = c.g[A0][0];
    c.lwc1(0, c.pa(V1, 0));
    c.g[A1][0] = 0x4;
    c.lwc1(1, c.pa(V1, 0x4));
    c.g[A3][0] = 0u64;
    c.swc1(3, c.pa(SP, 0xc));
    c.swc1(0, c.pa(SP, 0));
    c.stw_2(SP, 0x8, 0u64);
    c.swc1(1, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.stw_2(SP, 0x8, 0u64);
    c.sq(2, c.pa(A0, 0x20));
    c.swc1(3, c.pa(SP, 0xc));
    c.lwc1(0, c.pa(T0, 0));
    c.swc1(0, c.pa(A0, 0));
    c.lwc1(1, c.pa(T0, 0x4));
    c.swc1(1, c.pa(A0, 0x4));
    c.lwc1(1, c.pa(V1, 0x8));
    c.lwc1(0, c.pa(V1, 0));
    c.lwc1(2, c.pa(V1, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(2, c.pa(SP, 0x4));
    c.swc1(0, c.pa(SP, 0));
    c.lq(2, c.pa(SP, 0));
    c.stw_2(SP, 0x8, 0u64);
    c.sq(2, c.pa(A0, 0x50));
    c.swc1(3, c.pa(SP, 0xc));
    c.lwc1(1, c.pa(T0, 0x8));
    c.lwc1(0, c.pa(T0, 0));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x30));
    c.lwc1(1, c.pa(T0, 0x4));
    c.swc1(1, c.pa(A0, 0x34));
    c.lwc1(1, c.pa(V1, 0xc));
    c.lwc1(0, c.pa(V1, 0x4));
    c.lwc1(2, c.pa(V1, 0));
    c.add_s(0, 0, 1);
    c.swc1(2, c.pa(SP, 0));
    c.swc1(0, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.sq(2, c.pa(A0, 0x80));
    c.lwc1(0, c.pa(T0, 0));
    c.swc1(3, c.pa(SP, 0xc));
    c.swc1(0, c.pa(A0, 0x60));
    c.stw_2(SP, 0x8, 0u64);
    c.g[T1][0] = c.ldw_2(GP, -0x854);
    c.lwc1(1, c.pa(T0, 0xc));
    c.lwc1(0, c.pa(T0, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x64));
    c.lwc1(3, c.pa(V1, 0xc));
    c.lwc1(2, c.pa(V1, 0x8));
    c.lwc1(1, c.pa(V1, 0));
    c.lwc1(0, c.pa(V1, 0x4));
    c.add_s(1, 1, 2);
    c.add_s(0, 0, 3);
    c.swc1(1, c.pa(SP, 0));
    c.swc1(0, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.sq(2, c.pa(A0, 0xb0));
    c.lwc1(1, c.pa(T0, 0x8));
    c.lwc1(0, c.pa(T0, 0));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x90));
    c.lwc1(1, c.pa(T0, 0xc));
    c.lwc1(0, c.pa(T0, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x94));
    c.g[V0][0] = c.ldw_2(T1, 0x10d8);
    c.g[A0][0] = c.ldh_2(V0, 0x238);
    c.g[V1][0] = c.ldw_2(V0, 0x23c);
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x002cb48c);
    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.pc = t;
    call_indirect(c, t);
    if c.pc != 0x002cb48c {
        return;
    }
    c.g[RA][0] = c.ldd(SP, 0x10);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = ra;
    return;
}

/// A `gp_5`-relative global.
#[inline(always)]
pub(crate) fn gp_5(c: &Cpu, off: i32) -> u32 {
    crate::abi::r32(c, GP).wrapping_add(off as u32)
}

/// The menu-system singleton (`gp_5 - 0x854`).
#[inline(always)]
pub(crate) fn menu_sys(c: &mut Cpu) -> u32 {
    c.lw(gp_5(c, -0x854)) as u32
}

// ---------------------------------------------------------------------------
// 0x002cb180 MenuSysPage_BeginDraw
// ---------------------------------------------------------------------------

/// `void MenuSysPage_BeginDraw(void* page)` (0x2cb180).
///
/// Pushes the twenty-byte draw state of the current menu object (the page
/// pointer at `menu_sys + 0xe84`) onto that object's save stack, then runs the
/// four renderer passes the interface singleton's dispatch table holds:
/// viewport, scissor, background and font. The `v0` result is the caller's
/// (the last pass's return value is left in the register).
pub fn menu_sys_page_begin_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 94;
        let frame = Frame::open(c, 0x10);
        let ms = menu_sys(c);
        c.g[A0][0] = sx(ms);
        frame.save_ra(c, 0);

        // Push the 20-byte state at obj[0..0x14) onto obj[0x14..0x28).
        let obj = c.lw(p(ms, 0xe84)) as u32;
        c.ldl(3, p(obj, 0x7));
        c.ldr(3, obj);
        c.ldl(5, p(obj, 0xf));
        c.ldr(5, p(obj, 8));
        c.g[A2][0] = c.lw(p(obj, 0x10));
        c.sdl(3, p(obj, 0x1b));
        c.sdr(3, p(obj, 0x14));
        c.sdl(5, p(obj, 0x23));
        c.sdr(5, p(obj, 0x1c));
        c.sw(p(obj, 0x24), c.g[A2][0]);
        // ... and advance the save pointer by one slot.
        c.g[V1][0] = c.lw(p(ms, 0xe84));
        c.g[A1][0] = sx(ms);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
        c.sw(p(ms, 0xe84), c.g[V1][0]);

        // Renderer pass at 0xa8/0xac.
        let renderer = c.lw(p(ms, 0x10d8)) as u32;
        c.g[V0][0] = sx(renderer);
        c.g[A0][0] = c.lh(p(renderer, 0xa8));
        c.g[V1][0] = c.lw(p(renderer, 0xac));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b1dc)?;

        // Renderer pass at 0xf8/0xfc.
        c.g[A1][0] = sx(ms);
        c.g[V0][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0xf8));
        c.g[V1][0] = c.lw(p(renderer, 0xfc));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b1f4)?;

        // The draw-state bit fields: clear and set the pass flags of the
        // current menu object.
        c.g[T0][0] = sx(ms);
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V1][0] = 0xffff_ffff_fe7f_0000;
        c.g[T3][0] = 0x0100_0000;
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[T1][0] = 0xffff_ffff_ffcf_0000;
        c.g[T4][0] = 0x0030_0000;
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 8));
        c.g[T1][0] = c.g[T1][0] | 0xffff;
        c.g[T2][0] = 0xffff_ffff_fff0_0000;
        c.g[A3][0] = 0x0001_0000;
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[T2][0] = c.g[T2][0] | 0xfff;
        c.g[V0][0] = c.g[V0][0] | 0x220;
        c.g[A3][0] = c.g[A3][0] | 0x4000;
        c.sw(p(c.g[A0][0] as u32, 8), c.g[V0][0]);
        c.g[T5][0] = 0xffff_ffff_ffff_ff83;
        c.mtc1(0, 12); // f12 = 0.0
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[A2][0] = 0;
        c.g[AT][0] = 0x4420_0000; // 40.0
        c.mtc1(1, 14);
        c.mov_s(13, 12);
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
        c.mov_s(16, 12);
        c.g[AT][0] = 0x43f0_0000; // 480.0
        c.mtc1(1, 15);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(1, 17);
        c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
        c.sw(p(c.g[A0][0] as u32, 4), c.g[V0][0]);
        c.g[V1][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
        c.sw(p(c.g[V1][0] as u32, 4), c.g[V0][0]);
        c.g[A0][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
        c.sw(p(c.g[A0][0] as u32, 4), c.g[V0][0]);
        c.g[A3][0] = c.lw(p(ms, 0xe84));
        c.g[V0][0] = c.lw(p(c.g[A3][0] as u32, 4));
        c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw(p(c.g[A3][0] as u32, 4), c.g[V0][0]);

        // Renderer pass at 0xd0/0xd4.
        c.g[V1][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0xd0));
        c.g[V0][0] = c.lw(p(renderer, 0xd4));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(ms.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002c_b2cc)?;

        // Renderer pass at 0x110/0x114.
        c.g[A2][0] = sx(menu_sys(c));
        c.g[A1][0] = 0x0050_0000;
        c.g[A1][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        c.g[V0][0] = c.lw(p(ms, 0x10d8));
        c.g[A0][0] = c.lh(p(renderer, 0x110));
        c.g[V1][0] = c.lw(p(renderer, 0x114));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(c.g[A2][0] as u32 + c.g[A0][0] as u32);
        call_ptr(c, t, 0x002c_b2ec)?;

        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002d0d48 MenuSysPoint2DItem_Measure
// ---------------------------------------------------------------------------

/// `void* MenuSysPoint2DItem_Measure(void* item, void* page)` (0x2d0d48).
///
/// Measures a point item: when the item is the page's selected item the height
/// grows by the selection margin, then the item's text is scaled to the
/// available width and the text's box (16 bytes of floats) is written back to
/// the item. `v0` is the item.
pub fn menu_sys_point2d_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x50);
        frame.save(c, 18, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, 17, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        c.g[RA][0] = sx(0x002d_0d7c);
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        call_ptr(c, 0x002c_bf30, 0x002d_0d7c)?; // MenuSysPage_GetSelectedItem
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        let t = c.g[S0][0] != c.g[V0][0];
        if !t {
            c.cycles += 2;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x40));
            c.g[S2][0] = (0 < c.g[V0][0]) as u64;
        }

        c.cycles += 31;
        c.g[V0][0] = 0x50;
        c.g[S1][0] = 0x14;
        c.g[A1][0] = c.lw(p(c.g[S0][0] as u32, 0x14));
        c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0x124));
        if c.g[S2][0] != 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x58));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x14));
        c.cvt_s_w(1, 1);
        c.g[A0][0] = c.g[V0][0];
        c.lwc1(0, p(c.g[V0][0] as u32, 0x34));
        c.lwc1(12, p(c.g[A0][0] as u32, 0x38));
        c.lwc1(13, p(c.g[A0][0] as u32, 0x3c));
        c.g[RA][0] = sx(0x002d_0dcc);
        c.mul_s(21, 1, 0);
        call_ptr(c, 0x0039_1fb0, 0x002d_0dcc)?;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = 0x0048_0000;
        c.mov_s(20, 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x6748));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x124));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.lw(p(c.g[V0][0] as u32, 0x58));
        c.lwc1(12, p(c.g[A0][0] as u32, 0x38));
        c.g[RA][0] = sx(0x002d_0df8);
        c.lwc1(13, p(c.g[A0][0] as u32, 0x3c));
        call_ptr(c, 0x0039_1fb0, 0x002d_0df8)?;
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.mov_s(20, 0);
        }

        'height: {
            'half: {
                c.cycles += 4;
                c.mtc1(0, 0);
                c.swc1(0, c.g[SP][0] as u32);
                let t = c.g[S2][0] == 0;
                c.swc1(0, p(c.g[SP][0] as u32, 4));
                if t {
                    break 'half;
                }
                'clamp: {
                    c.cycles += 9;
                    c.add_s(1, 21, 21);
                    c.g[AT][0] = 0x40c0_0000;
                    c.mtc1(1, 0);
                    c.mtc1(17, 2);
                    c.cvt_s_w(2, 2);
                    c.add_s(1, 1, 0);
                    c.c_lt_s(1, 2);
                    let t = !c.fcc();
                    if t {
                        break 'clamp;
                    }
                    c.cycles += 4;
                    c.g[AT][0] = 0x41c0_0000;
                    c.mtc1(1, 0);
                    c.add_s(1, 2, 0);
                    break 'clamp;
                }
                c.cycles += 3;
                c.g[AT][0] = 0x41c0_0000;
                c.mtc1(1, 0);
                c.add_s(1, 1, 0);
                c.cycles += 2;
                c.swc1(1, p(c.g[SP][0] as u32, 0xc));
                break 'height;
            }
            // Not selected: the plain text box.
            c.cycles += 11;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[AT][0] = 0x4140_0000;
            c.mtc1(1, 2);
            c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x124));
            c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x58));
            c.lwc1(0, p(c.g[V1][0] as u32, 0x14));
            c.cvt_s_w(0, 0);
            c.lwc1(1, p(c.g[V1][0] as u32, 0x34));
            c.mul_s(0, 0, 1);
            c.add_s(0, 0, 2);
            c.swc1(0, p(c.g[SP][0] as u32, 0xc));
            break 'height;
        }

        c.cycles += 26;
        c.g[V0][0] = 0x64;
        c.g[AT][0] = 0x4280_0000;
        c.mtc1(1, 1);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.lq(16, p(c.g[SP][0] as u32, 0x50));
        c.g[V0][0] = c.g[S3][0];
        c.lq(17, p(c.g[SP][0] as u32, 0x40));
        c.lq(18, p(c.g[SP][0] as u32, 0x30));
        c.add_s(0, 0, 20);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.lwc1(21, p(c.g[SP][0] as u32, 0x68));
        c.lwc1(20, p(c.g[SP][0] as u32, 0x60));
        c.add_s(0, 0, 1);
        c.swc1(0, p(c.g[SP][0] as u32, 8));
        c.ldl(3, p(c.g[SP][0] as u32, 7));
        c.ldr(3, c.g[SP][0] as u32);
        c.ldl(4, p(c.g[SP][0] as u32, 0xf));
        c.ldr(4, p(c.g[SP][0] as u32, 8));
        c.sdl(3, p(c.g[S3][0] as u32, 7));
        c.sdr(3, c.g[S3][0] as u32);
        c.sdl(4, p(c.g[S3][0] as u32, 0xf));
        c.sdr(4, p(c.g[S3][0] as u32, 8));
        c.lq(19, p(c.g[SP][0] as u32, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `MenuSysGroupItem_Draw` (0x002cfd28): lay out and draw the rows of a
/// grouped menu list.  Starting from the rect at `a1`, each visible item
/// (`MenuSysItem_IsVisible`) gets a 0x10-byte rect at `self+0x14c+i*16` --
/// `x` = pos.x + (item == current ? 0 : 20), `y` = the running pos.y,
/// `w` = pos.w - the same offset -- and is drawn through its vtable slot
/// `+0x2c` (with the `+0x28` this-adjustment); `pos.y` then advances by the
/// rect's height (`+0x158`).  The selected item is either the group's
/// current item or every item while `self` is its page's selected item.
pub fn menu_sys_group_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[S4][0] = 0u64;
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18)); // count
        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[SP][0] as u32);
        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        let no_rows = (c.g[V0][0] as i64) <= 0;
        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        if !no_rows {
            c.cycles += 5;
            c.g[S6][0] = 0x14; // indent of a non-selected row
            c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14c)); // rects
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24)); // items
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x154));
            loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002c_fd98);
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                call_ptr(c, 0x002c_a378, 0x002c_fd98)?;
                let mut draw = false;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002c_fda8);
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    call_ptr(c, 0x002c_bf30, 0x002c_fda8)?;
                    if c.g[S1][0] == c.g[V0][0] {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        draw = true;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            draw = true;
                        }
                    }
                }
                if draw {
                    c.cycles += 28;
                    c.g[V1][0] = c.g[S6][0];
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = c.g[S5][0];
                    c.lwc1(3, c.g[SP][0] as u32);
                    c.g[S0][0] = sx((c.g[S4][0] as u32) << 4);
                    c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.mtc1(3, 1); // f1 = (float)off
                    c.cvt_s_w(1, 1);
                    c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x4));
                    c.sub_s(2, 2, 1);
                    c.add_s(3, 3, 1);
                    c.swc1(2, c.g[S3][0] as u32);
                    c.swc1(3, (c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002c_fe1c)?;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x158));
                    c.add_s(0, 0, 1);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                }
                c.cycles += 6;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_fd90);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysGroupItem_Measure` (0x002cfb58): measure a grouped menu list.  For
/// every visible item (`MenuSysItem_IsVisible`) the item's measure method
/// (vtable `+0x24`, with the `+0x20` this-adjustment) fills the 0x10-byte rect
/// at `sp+0x10`, which is copied to `self+0x14c+i*16`; the item's height is
/// added to the running total at `sp+0xc` when it is the selected row, and
/// non-selected rows get the 20-pixel indent added before the tallest height
/// is kept at `sp+8` and finally written to the 16-byte result at `a0`.
pub fn menu_sys_group_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xc0);
        c.mtc1(0, 0); // f0 = 0.0
        frame.save(c, S0, 0xa0);
        frame.save(c, S4, 0x60);
        c.g[S0][0] = c.g[A1][0]; // the block
        frame.save(c, S7, 0x30);
        c.g[S4][0] = 0u64;
        frame.save(c, S1, 0x90);
        c.g[S7][0] = c.g[A0][0]; // the result rect
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0xb0);
        c.g[A1][0] = c.lw(c.g[S0][0] as u32); // the page
        c.swc1(0, c.g[SP][0] as u32);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18)); // count
        let empty = (c.g[V1][0] as i64) <= 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
        if !empty {
            c.cycles += 8;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 20); // f20 = 20.0
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c)); // rects
            c.g[S3][0] = 0u64;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24)); // items
            c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x158));
            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x154));
            loop {
                'iter: {
                    'a: {
                        {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x002c_fbe0);
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            call_ptr(c, 0x002c_a378, 0x002c_fbe0)?;
                            let is_hidden = c.g[V0][0] == 0;
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            if !is_hidden {
                                'measured: {
                                    'selected: {
                                        c.cycles += 18;
                                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                        let target = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x002c_fc00);
                                        c.g[A1][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                        call_ptr(c, target, 0x002c_fc00)?;
                                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
                                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                                        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                                        c.sdr(2, c.g[S2][0] as u32);
                                        c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0xf));
                                        c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[RA][0] = sx(0x002c_fc28);
                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                        call_ptr(c, 0x002c_bf30, 0x002c_fc28)?;
                                        let whole_group = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        if whole_group {
                                            break 'selected;
                                        }
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        let not_current = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        if not_current {
                                            break 'measured;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        break 'selected;
                                    }
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.lwc1(1, c.g[V0][0] as u32);
                                    c.add_s(0, 0, 1);
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.g[V0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                    break 'measured;
                                }
                                {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    let current = c.g[A0][0] == c.g[V1][0];
                                    c.lwc1(1, c.g[V0][0] as u32);
                                    if !current {
                                        c.cycles += 1;
                                        c.add_s(1, 1, 20);
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.c_lt_s(0, 1);
                                if c.fcc() {
                                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    break 'a;
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                break 'iter;
                            }
                        }
                        // A hidden item contributes an empty rect.
                        c.cycles += 12;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x1f));
                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[S2][0] as u32);
                        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0xf));
                        c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                }
                c.cycles += 6;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_fbd8);
                }
            }
        }
        c.cycles += 21;
        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(5, c.g[SP][0] as u32);
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(5, (c.g[S7][0] as u32).wrapping_add(0x7));
        c.sdr(5, c.g[S7][0] as u32);
        c.sdl(6, (c.g[S7][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = c.g[S7][0];
        c.sdr(6, (c.g[S7][0] as u32).wrapping_add(0x8));
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

/// `void func_002CC0C0(void* stack)` (0x2cc0c0): the menu system's per-frame
/// update, driven as a resumable block dispatcher (re-entered at_5 the `pc` the
/// trampoline resumes): walk the eight menu entries of the active page, call
/// each one's update slot, and after the loop run the page's own update.
pub fn menu_sys_update(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002cc0c0 => {
                c.cycles += 8;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                sq(c, 16, at_5(c, SP, 0x20));
                sq(c, 17, at_5(c, SP, 0x10));
                c.g[S0][0] = c.g[A0][0];
                sd(c, r32(c, SP), c.g[RA][0]);
                c.g[V0][0] = lw(c, r32(c, S0));

                if c.g[V0][0] == 0u64 {
                    lq(c, 16, at_5(c, SP, 0x20));
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc0e0;
                continue;
            }
            0x002cc0e0 => {
                c.cycles += 4;
                c.g[A0][0] = lw(c, at_5(c, S0, 0x5c));
                c.g[V0][0] = lw(c, r32(c, A0));
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0u64;
                if t {
                    pc = 0x002cc0f8;
                    continue;
                }
                pc = 0x002cc0f0;
                continue;
            }
            0x002cc0f0 => {
                c.cycles += 2;
                c.g[V0][0] = lw(c, r32(c, V0));
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x002cc0f8;
                continue;
            }
            0x002cc0f8 => {
                c.cycles += 2;

                if c.g[V1][0] == 0u64 {
                    lq(c, 16, at_5(c, SP, 0x20));
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc100;
                continue;
            }
            0x002cc100 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x002cc108);
                c.g[A1][0] = 0x3d;
                if call_ptr(c, 0x00320bf0, 0x002cc108).is_err() {
                    return;
                }
                c.cvt_w_s(1, 0);
                c.mfc1(17, 1);
                c.g[A2][0] = sx(at_5(c, S0, 0x4c));
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A3][0] = sx(at_5(c, S0, 0x50));
                sw(c, at_5(c, S0, 0x4c), c.g[V0][0]);
                sw(c, at_5(c, S0, 0x50), c.g[V0][0]);
                let t = c.g[S1][0] != c.g[V0][0];
                sw(c, at_5(c, S0, 0x48), c.g[S1][0]);
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc12c;
                continue;
            }
            0x002cc12c => {
                c.cycles += 3;
                c.g[A0][0] = lw(c, at_5(c, GP, 0x300));

                if c.g[A0][0] == 0u64 {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc1c8;
                    continue;
                }
                pc = 0x002cc138;
                continue;
            }
            0x002cc138 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002cc140);
                c.g[A1][0] = 0u64;
                if call_ptr(c, 0x00266ba8, 0x002cc140).is_err() {
                    return;
                }

                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc1c8;
                    continue;
                }
                pc = 0x002cc148;
                continue;
            }
            0x002cc148 => {
                c.cycles += 4;
                c.g[V0][0] = lw(c, at_5(c, S0, 0x50));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;

                if c.g[V0][0] != 0u64 {
                    sw(c, at_5(c, S0, 0x50), c.g[S1][0]);
                    pc = 0x002cc158;
                    continue;
                }
                pc = 0x002cc158;
                continue;
            }
            0x002cc158 => {
                c.cycles += 5;
                c.g[V0][0] = lw(c, at_5(c, S0, 0x4c));
                c.g[A0][0] = sx(r32(c, V0).wrapping_sub(0xd));
                c.g[V1][0] = (c.g[A0][0] < 0x22u64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc16c;
                continue;
            }
            0x002cc16c => {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, A0) << 2);
                c.g[V0][0] = sx(at_5(c, V0, 0x6630));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = lw(c, r32(c, V1));
                let t = c.g[A0][0] as u32;

                pc = t;
                continue;
            }
            0x002cc184 => {
                c.cycles += 2;
                let t = true;
                sw(c, at_5(c, S0, 0x48), 0u64);
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc18c;
                continue;
            }
            0x002cc18c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x1;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc194;
                continue;
            }
            0x002cc194 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x2;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc19c;
                continue;
            }
            0x002cc19c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x3;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1a4;
                continue;
            }
            0x002cc1a4 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x9;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1ac;
                continue;
            }
            0x002cc1ac => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xa;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1b4;
                continue;
            }
            0x002cc1b4 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x6;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1bc;
                continue;
            }
            0x002cc1bc => {
                c.cycles += 1;
                c.g[V0][0] = 0x7;
                pc = 0x002cc1c0;
                continue;
            }
            0x002cc1c0 => {
                c.cycles += 1;
                sw(c, at_5(c, S0, 0x48), c.g[V0][0]);
                pc = 0x002cc1c4;
                continue;
            }
            0x002cc1c4 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, r32(c, S0));
                pc = 0x002cc1c8;
                continue;
            }
            0x002cc1c8 => {
                c.cycles += 13;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[A1][0] = lw(c, at_5(c, V0, 0x4));
                c.g[V1][0] = lw(c, at_5(c, A1, 0x12c));
                c.g[A0][0] = lh_2(c, at_5(c, V1, 0x10));
                c.g[V0][0] = lw(c, at_5(c, V1, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002cc1ec);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x002cc1ec).is_err() {
                    return;
                }
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x2;
                if t {
                    pc = 0x002cc20c;
                    continue;
                }
                pc = 0x002cc1fc;
                continue;
            }
            0x002cc1fc => {
                c.cycles += 2;

                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc22c;
                    continue;
                }
                pc = 0x002cc204;
                continue;
            }
            0x002cc204 => {
                c.cycles += 2;
                let t = true;
                lq(c, 16, at_5(c, SP, 0x20));
                if t {
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc20c;
                continue;
            }
            0x002cc20c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002cc214);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x002cc460, 0x002cc214).is_err() {
                    return;
                }
                let t = true;
                lq(c, 16, at_5(c, SP, 0x20));
                if t {
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc21c;
                continue;
            }
            0x002cc21c => {
                c.cycles += 1;
                pc = 0x002cc220;
                continue;
            }
            0x002cc220 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002cc228);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x002cc460, 0x002cc228).is_err() {
                    return;
                }
                c.g[V0][0] = lw(c, r32(c, S0));
                pc = 0x002cc22c;
                continue;
            }
            0x002cc22c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x002cc220) {
                        return;
                    }
                    pc = 0x002cc220;
                    continue;
                }
                pc = 0x002cc234;
                continue;
            }
            0x002cc234 => {
                c.cycles += 1;
                lq(c, 16, at_5(c, SP, 0x20));
                pc = 0x002cc238;
                continue;
            }
            0x002cc238 => {
                c.cycles += 4;
                lq(c, 17, at_5(c, SP, 0x10));
                c.g[RA][0] = ld(c, r32(c, SP));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at_5(c, SP, 0x30));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x002cc0c0, 0x002cc248);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// World_GetPeakNumber (0x2d1ba0).

/// `BELibrary* BELibrary_GetInstance()`.
pub(crate) const WORLD_BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

/// `void* BELibrary_GetInterface(BELibrary*, int)`.
pub(crate) const WORLD_BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

/// `int BENewRaceInt_GetTrackPeak(BENewRaceInt*)`.
pub(crate) const WORLD_BE_NEW_RACE_GET_TRACK_PEAK: u32 = 0x0014_4c98;

/// `int World_GetPeakNumber()` (0x2d1ba0): the current track's peak,
/// counted from 1:
/// `BENewRaceInt_GetTrackPeak(BELibrary_GetInterface(BELibrary_GetInstance(), 0)) + 1`.
/// Frame 0x10 (ra +0).
pub fn world_get_peak_number(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, WORLD_BE_LIBRARY_GET_INSTANCE, 0x002d_1bb0)?;
        set_reg(c, A1, 0);
        let library = reg(c, V0);
        set_reg(c, A0, library);
        call_ptr(c, WORLD_BE_LIBRARY_GET_INTERFACE, 0x002d_1bbc)?;
        let race = reg(c, V0);
        set_reg(c, A0, race);
        call_ptr(c, WORLD_BE_NEW_RACE_GET_TRACK_PEAK, 0x002d_1bc4)?;
        frame.restore_ra(c, 0);
        let peak = reg(c, V0);
        set_reg(c, V0, addu(peak, 1));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_TryRunKillScript (0x2d19e8)

/// The script manager pointer global (`D_004A3DD8`), at `gp + 0xce8`.
pub(crate) const WSCRIPT_MANAGER_GP: u32 = 0xce8;

/// `WScriptMan_HasKillScript` (0x30a598, cross-shard).
pub(crate) const WSCRIPT_HAS_KILL_SCRIPT: u32 = 0x0030_a598;

/// `WScriptMan_RunKillScript` (0x30a5c0, cross-shard).
pub(crate) const WSCRIPT_RUN_KILL_SCRIPT: u32 = 0x0030_a5c0;

/// `int WScriptMan_TryRunKillScript(int id)` (0x2d19e8): when the script
/// manager (`*(gp + 0xce8)`) has a kill script for `id`, run it and return 1,
/// else return 0. Frame 0x20 (s0 = id).
pub fn wscript_man_try_run_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(WSCRIPT_MANAGER_GP)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, WSCRIPT_HAS_KILL_SCRIPT, 0x002d_1a04)?;
        let has_script = c.g[V0][0] != 0;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(WSCRIPT_MANAGER_GP)).lw(c);
        if has_script {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, WSCRIPT_RUN_KILL_SCRIPT, 0x002d_1a1c)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002d1bf0 World_AddFocusObject

/// `void World_AddFocusObject(FocusObject *obj)` (0x2d1bf0): add `obj` to the
/// focus manager hanging off the world (`world->+0x84 -> +0xc -> +0xa4`).
pub fn world_add_focus_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xa4).lw(c);
        Callee {
            addr: 0x0010_32c0,
            f: ee::s00::FocusMan_AddObject,
        }
        .call(c, 0x002d_1c10)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_RunObjScript0Hook(void* inst)` (0x2d1a78): forwards the
/// instance to `cWScriptMan_doOnInitScript` with the global script manager.
pub fn wscriptman_run_obj_script0_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0030_a610, 0x002d_1a90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysColorSwatchItem_Draw` (0x002d0008): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn menu_sys_color_swatch_item_draw(c: &mut Cpu) {
    if c.pc != 0x002d0008 {
        return ee::s09::f_002d0008__generated(c);
    }
    guest(c, |c| {
        'b_2d0260: {
            'b_2d024c: {
                c.cycles += 139;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.g[A3][0] = 0u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[T0][0] = 0u64;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[S0][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                c.g[S1][0] = c.g[A0][0];
                c.g[RA][0] = sx(0x002d0030);
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                callee::MENU_SYS_ITEM_DRAW_ROW_6.call(c, 0x002d0030)?;
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = 0x3;
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.lwc1(3, c.g[S0][0] as u32);
                c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.add_s(3, 3, 0);
                c.g[AT][0] = 0x41800000;
                c.mtc1(1, 1);
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 0);
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 7);
                c.add_s(5, 5, 0);
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 2);
                c.sub_s(3, 3, 1);
                c.g[AT][0] = 0x41880000;
                c.mtc1(1, 8);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.add_s(5, 5, 2);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xc8));
                c.sub_s(3, 3, 7);
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xcc));
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 2);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
                c.add_s(8, 5, 8);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.add_s(7, 3, 7);
                c.mtc1(0, 4);
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 6);
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lwc1(0, c.g[V1][0] as u32);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.mul_s(0, 0, 2);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xd8));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xdc));
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(4, c.g[SP][0] as u32);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.lwc1(1, c.g[V1][0] as u32);
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4c));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.lwc1(1, c.g[V1][0] as u32);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.mul_s(1, 1, 2);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x7c));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa4));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa8));
                c.lwc1(1, c.g[V1][0] as u32);
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xac));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x98));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lwc1(0, c.g[V1][0] as u32);
                c.c_eq_s(0, 6);
                let t = c.fcc();
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                if t {
                    break 'b_2d024c;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0xffffffffffffff83;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    if t {
                        break 'b_2d0260;
                    } else {
                        break 'b_2d024c;
                    }
                }
            }
            c.cycles += 5;
            c.g[A1][0] = 0xffffffffffffff83;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | 0x4;
            break 'b_2d0260;
        }
        c.cycles += 19;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x4;
        c.g[A2][0] = c.g[SP][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[A3][0] = 0u64;
        c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x002d0298);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002d0298)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    });
}

/// void `MenuSysPoint2DItem_Draw(MenuSysPoint2DItem *item, ...)` (0x2d0ef8): draw the point item.
pub fn menu_sys_point2d_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d0fa0: {
            'b_2d0f9c: {
                c.cycles += 38;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1e0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[S3][0] = c.g[A0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[S4][0] = c.g[A1][0];
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                callee::MENU_SYS_PAGE_GET_SELECTED_ITEM.call(c, 0x002d0f40)?;
                c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                c.g[V0][0] = c.g[S3][0] ^ c.g[V0][0];
                c.g[S5][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.g[A1][0] = c.g[S4][0];
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x12c));
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002d0f68)?;
                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.cvt_s_w(1, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.mul_s(21, 1, 0);
                callee::MENU_SYS_PAGE_GET_SELECTED_ITEM.call(c, 0x002d0f88)?;
                let t = c.g[S3][0] != c.g[V0][0];
                c.g[V0][0] = 0x4d0000;
                if t {
                    break 'b_2d0f9c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4d0000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                    break 'b_2d0fa0;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
            break 'b_2d0fa0;
        }
        'b_2d1174: {
            'b_2d1168: {
                'b_2d1164: {
                    'b_2d1140: {
                        c.cycles += 50;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 20);
                        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        c.g[S6][0] = c.g[V0][0];
                        c.swc1(20, c.g[SP][0] as u32);
                        c.g[AT][0] = 0x40000000;
                        c.mtc1(1, 2);
                        c.lwc1(0, c.g[V1][0] as u32);
                        c.g[AT][0] = 0x40c00000;
                        c.mtc1(1, 3);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[AT][0] = 0x41800000;
                        c.mtc1(1, 4);
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(2, c.g[SP][0] as u32);
                        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
                        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
                        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
                        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
                        c.lwc1(13, (c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                        c.add_s(13, 13, 3);
                        c.lwc1(12, c.g[S4][0] as u32);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                        c.add_s(12, 12, 4);
                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                        c.sub_s(13, 13, 20);
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                        call_ptr(c, 0x00391cb0, 0x002d105c)?;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_2d1140;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S5][0] == 0u64;
                            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_2d1164;
                            } else {
                                c.cycles += 52;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                c.g[A0][0] = c.g[S2][0];
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.lwc1(12, c.g[V0][0] as u32);
                                callee::MENU_SYS_FORMAT_FLOAT.call(c, 0x002d1090)?;
                                c.g[A2][0] = 0x4a0000;
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x39f0));
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x00318630, 0x002d10a4)?;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
                                callee::MENU_SYS_FORMAT_FLOAT.call(c, 0x002d10c4)?;
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x003185c8, 0x002d10d4)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10e0)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10ec)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10f8)?;
                                c.lwc1(13, (c.g[S4][0] as u32).wrapping_add(0x4));
                                c.g[AT][0] = 0x41400000;
                                c.mtc1(1, 0);
                                c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                                c.add_s(13, 13, 0);
                                c.lwc1(1, c.g[S4][0] as u32);
                                c.g[AT][0] = 0x42000000;
                                c.mtc1(1, 12);
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                                c.sub_s(13, 13, 20);
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.add_s(12, 1, 12);
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.add_s(13, 13, 21);
                                call_ptr(c, 0x00391cb0, 0x002d1134)?;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d1140)?;
                                break 'b_2d1140;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                    if t {
                        break 'b_2d1168;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_2d1168;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x41c00000;
                            c.mtc1(1, 0);
                            c.sub_s(6, 1, 0);
                            break 'b_2d1174;
                        }
                    }
                }
                c.cycles += 1;
                c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                break 'b_2d1168;
            }
            c.cycles += 3;
            c.g[AT][0] = 0x41400000;
            c.mtc1(1, 0);
            c.sub_s(6, 1, 0);
            break 'b_2d1174;
        }
        c.cycles += 29;
        c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x3;
        c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0xc));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.lwc1(0, c.g[S4][0] as u32);
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.sub_s(2, 2, 6);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 3);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 4);
        c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = 0xffffffffffffffff;
        c.mul_s(2, 2, 3);
        c.g[AT][0] = 0x42c80000;
        c.mtc1(1, 5);
        c.sub_s(0, 0, 4);
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.add_s(1, 1, 2);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sub_s(0, 0, 5);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & 0x3e0;
        c.g[S2][0] = sx((c.g[V0][0] as u32) >> 5);
        'l_2d11e8: loop {
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            let t = c.g[A1][0] != c.g[A2][0];
            c.g[V1][0] = 0x4d0000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002d11e8);
                }
                continue 'l_2d11e8;
            } else {
                c.cycles += 27;
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x53c0));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x53c0));
                c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.mul_s(3, 3, 0);
                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.mul_s(4, 4, 0);
                c.mul_s(1, 1, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 5);
                c.mul_s(2, 2, 0);
                c.g[S4][0] = sx((c.g[S2][0] as u32) << 5);
                c.cvt_w_s(0, 3);
                c.mfc1(8, 0);
                c.g[V1][0] = c.g[S1][0];
                c.cvt_w_s(0, 1);
                c.mfc1(7, 0);
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc0));
                c.cvt_w_s(0, 2);
                c.mfc1(5, 0);
                c.cvt_w_s(0, 4);
                c.mfc1(4, 0);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                'l_2d1270: loop {
                    c.cycles += 8;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A1][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x002d1270);
                        }
                        continue 'l_2d1270;
                    } else {
                        'b_2d131c: {
                            c.cycles += 34;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            c.g[A3][0] = 0xffffffffffffffff;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[A1][0] = c.g[S6][0];
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[S5][0];
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x11c));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x118));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x110), 0u64);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x114), 0u64);
                            callee::MENU_SYS_BUILD_QUAD.call(c, 0x002d12c8)?;
                            c.lwc1(3, (c.g[S3][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = 0x1;
                            c.lwc1(7, (c.g[S3][0] as u32).wrapping_add(0x38));
                            c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x2c));
                            c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x34));
                            c.sub_s(4, 3, 7);
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                            c.sub_s(1, 1, 2);
                            c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x18));
                            c.div_s(23, 0, 4);
                            c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.mtc1(0, 8);
                            c.div_s(22, 5, 1);
                            c.c_le_s(8, 3);
                            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                            c.mul_s(0, 7, 23);
                            c.mul_s(2, 2, 22);
                            c.sub_s(21, 3, 0);
                            let t = c.fcc();
                            c.sub_s(24, 6, 2);
                            if t {
                                break 'b_2d131c;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0u64;
                                break 'b_2d131c;
                            }
                        }
                        'b_2d1330: {
                            c.cycles += 4;
                            c.c_lt_s(8, 7);
                            let t = c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_2d1330;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_2d1330;
                            }
                        }
                        'b_2d1444: {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            if t {
                                break 'b_2d1444;
                            } else {
                                c.cycles += 36;
                                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                c.g[V1][0] = 0x1f;
                                c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                if c.g[A0][0] == 0 {
                                    c.g[V0][0] = c.g[V1][0];
                                }
                                c.g[A1][0] = 0xfffffffffffffc1f;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.g[V0][0] & 0x3e0;
                                c.g[A2][0] = 0x4d0000;
                                c.g[AT][0] = 0x43000000;
                                c.mtc1(1, 4);
                                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5380));
                                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                                c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x5380));
                                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                                c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0xc));
                                c.mul_s(2, 2, 4);
                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                c.mul_s(1, 1, 4);
                                c.mul_s(3, 3, 4);
                                c.mul_s(0, 0, 4);
                                c.cvt_w_s(4, 2);
                                c.mfc1(7, 4);
                                c.cvt_w_s(2, 1);
                                c.mfc1(6, 2);
                                c.cvt_w_s(1, 0);
                                c.mfc1(4, 1);
                                c.cvt_w_s(0, 3);
                                c.mfc1(3, 0);
                                c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                                'l_2d13c8: loop {
                                    c.cycles += 7;
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002d13c8);
                                        }
                                        continue 'l_2d13c8;
                                    } else {
                                        c.cycles += 24;
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = 0x2;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 2);
                                        c.g[A2][0] = c.g[S1][0];
                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                                        c.g[A3][0] = 0u64;
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                        c.add_s(0, 0, 1);
                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x114));
                                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x002d1444)?;
                                        break 'b_2d1444;
                                    }
                                }
                            }
                        }
                        c.cycles += 35;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[A0][0] = ((c.g[S0][0] as i64) < 32i64) as u64;
                        c.g[V0][0] = 0x1f;
                        c.g[A3][0] = 0xfffffffffffffc1f;
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                        if c.g[A0][0] != 0 {
                            c.g[V0][0] = c.g[S0][0];
                        }
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                        c.g[A1][0] = 0x4d0000;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.g[V0][0] & 0x3e0;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x53d0));
                        c.g[AT][0] = 0x43000000;
                        c.mtc1(1, 4);
                        c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                        c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60));
                        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                        c.g[A3][0] = c.g[S1][0];
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x53d0));
                        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                        c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0xc));
                        c.mul_s(2, 2, 4);
                        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.mul_s(1, 1, 4);
                        c.mul_s(3, 3, 4);
                        c.mul_s(0, 0, 4);
                        c.cvt_w_s(4, 2);
                        c.mfc1(6, 4);
                        c.cvt_w_s(2, 1);
                        c.mfc1(5, 2);
                        c.cvt_w_s(1, 0);
                        c.mfc1(4, 1);
                        c.cvt_w_s(0, 3);
                        c.mfc1(3, 0);
                        c.sw((c.g[A3][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                        'l_2d14d0: loop {
                            c.cycles += 7;
                            c.sw((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                            c.sw((c.g[A3][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                            c.sw((c.g[A3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.sw((c.g[A3][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                                if event_due(c) {
                                    deliver_event_here(c, 0x002d14d0);
                                }
                                continue 'l_2d14d0;
                            } else {
                                'b_2d15b8: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                                    c.g[S0][0] = 0x1;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        break 'b_2d15b8;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 20);
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                        'l_2d1510: loop {
                                            c.cycles += 41;
                                            c.g[V1][0] = sx((c.g[S0][0] as u32) << 3);
                                            c.g[T1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                                            c.g[A1][0] = 0x2;
                                            c.g[V0][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.g[A3][0] = 0u64;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_sub(0x8));
                                            c.lwc1(2, (c.g[A2][0] as u32).wrapping_sub(0x4));
                                            c.g[V1][0] = c.g[V0][0];
                                            c.mul_s(1, 1, 22);
                                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                                            c.mul_s(2, 2, 23);
                                            c.lwc1(0, c.g[V1][0] as u32);
                                            c.mul_s(3, 3, 23);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                            c.add_s(1, 1, 24);
                                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                            c.add_s(2, 2, 21);
                                            c.g[A2][0] = c.g[S1][0];
                                            c.mul_s(0, 0, 22);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.add_s(3, 3, 21);
                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x114));
                                            c.add_s(0, 0, 24);
                                            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x114));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x118), 0u64);
                                            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.g[V1][0] =
                                                c.lw((c.g[T1][0] as u32).wrapping_add(0x10d8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
                                            c.g[T0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
                                            let t = c.g[T0][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[T1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x002d15a4)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                                            c.g[V0][0] =
                                                ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002d1510);
                                                }
                                                continue 'l_2d1510;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                break 'b_2d15b8;
                                            }
                                        }
                                    }
                                }
                                'b_2d16e4: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    if t {
                                        break 'b_2d16e4;
                                    } else {
                                        c.cycles += 36;
                                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3));
                                        c.g[A0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                        c.g[V1][0] = 0x1f;
                                        c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                                        if c.g[A0][0] == 0 {
                                            c.g[V0][0] = c.g[V1][0];
                                        }
                                        c.g[A1][0] = 0xfffffffffffffc1f;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = c.g[V0][0] & 0x3e0;
                                        c.g[A2][0] = 0x4d0000;
                                        c.g[AT][0] = 0x43000000;
                                        c.mtc1(1, 4);
                                        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x53e0));
                                        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                                        c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                        c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc0));
                                        c.g[V1][0] = c.g[S1][0];
                                        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x53e0));
                                        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                                        c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0xc));
                                        c.mul_s(2, 2, 4);
                                        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.mul_s(1, 1, 4);
                                        c.mul_s(3, 3, 4);
                                        c.mul_s(0, 0, 4);
                                        c.cvt_w_s(4, 2);
                                        c.mfc1(7, 4);
                                        c.cvt_w_s(2, 1);
                                        c.mfc1(6, 2);
                                        c.cvt_w_s(1, 0);
                                        c.mfc1(5, 1);
                                        c.cvt_w_s(0, 3);
                                        c.mfc1(4, 0);
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                                        'l_2d1650: loop {
                                            c.cycles += 7;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[A2][0],
                                            );
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x14),
                                                c.g[A1][0],
                                            );
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x18),
                                                c.g[A0][0],
                                            );
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                            c.g[V0][0] =
                                                ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x1c),
                                                    c.g[A3][0],
                                                );
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002d1650);
                                                }
                                                continue 'l_2d1650;
                                            } else {
                                                c.cycles += 30;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                                c.g[A1][0] = c.g[S5][0];
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                c.g[AT][0] = 0x40000000;
                                                c.mtc1(1, 4);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.g[AT][0] = 0x40a00000;
                                                c.mtc1(1, 2);
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                c.g[A2][0] = c.g[T1][0];
                                                c.lwc1(0, c.g[V0][0] as u32);
                                                c.mul_s(1, 1, 23);
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 3);
                                                c.mul_s(0, 0, 22);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x11c));
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x12c));
                                                c.add_s(1, 1, 21);
                                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x118));
                                                c.add_s(0, 0, 24);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x120), 0u64);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x124), 0u64);
                                                c.sub_s(1, 1, 4);
                                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x128));
                                                c.sub_s(0, 0, 4);
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x114));
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                callee::MENU_SYS_BUILD_QUAD.call(c, 0x002d16e4)?;
                                                break 'b_2d16e4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 22;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                c.g[A1][0] = 0xfffffffffffffc1f;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
                                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
                                c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
                                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
                                c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                                c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                                c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1e0));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    });
}

/// void `MenuSysColorChannelItem_Draw(MenuSysColorChannelItem *item, ...)` (0x2cf1c8): draw the colour channel item.
pub fn menu_sys_color_channel_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        'b_2cf270: {
            'b_2cf26c: {
                c.cycles += 38;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.g[S2][0] = c.g[A0][0];
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
                c.g[S3][0] = c.g[A1][0];
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[S0][0] = c.lw(c.g[S2][0] as u32);
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x48));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                callee::MENU_SYS_PAGE_GET_SELECTED_ITEM.call(c, 0x002cf210)?;
                c.g[V0][0] = c.g[S2][0] ^ c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = (c.g[V0][0] < 0x1u64) as u64;
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, t, 0x002cf228)?;
                c.lwc1(0, c.g[S3][0] as u32);
                c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.g[AT][0] = 0x41800000;
                c.mtc1(1, 3);
                c.add_s(0, 0, 1);
                c.g[AT][0] = 0x42a00000;
                c.mtc1(1, 2);
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                c.sub_s(0, 0, 3);
                c.sub_s(22, 0, 2);
                c.sub_s(23, 22, 3);
                callee::MENU_SYS_PAGE_GET_SELECTED_ITEM.call(c, 0x002cf258)?;
                if c.g[S2][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x4d0000;
                    break 'b_2cf26c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4d0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                    break 'b_2cf270;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
            break 'b_2cf270;
        }
        c.cycles += 128;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[A2][0] = 0x480000;
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7470));
        c.swc1(20, c.g[SP][0] as u32);
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 2);
        c.lwc1(0, c.g[V0][0] as u32);
        c.g[AT][0] = 0x40c00000;
        c.mtc1(1, 21);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 3);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x2f));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x28));
        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.add_s(13, 13, 21);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.lwc1(12, c.g[S3][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.sub_s(13, 13, 20);
        c.add_s(12, 12, 3);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, 0x00391cb0, 0x002cf338)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[S0][0] = 0x480000;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6710));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x00413af8, 0x002cf35c)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fa4));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, c.g[A3][0] as u32);
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(7, 1);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00417828, 0x002cf390)?;
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x3c));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00391fb0, 0x002cf3b8)?;
        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.sub_s(12, 23, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.add_s(13, 13, 21);
        c.sub_s(13, 13, 20);
        call_ptr(c, 0x00391cb0, 0x002cf3d4)?;
        c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = 0x3;
        c.add_s(0, 0, 1);
        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x37));
        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x3f));
        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.sub_s(0, 0, 21);
        c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(10, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x47));
        c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x4f));
        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[AT][0] = 0x42a00000;
        c.mtc1(1, 1);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x30), 0u64);
        c.g[A0][0] = c.g[V1][0];
        c.sub_s(0, 0, 2);
        c.swc1(20, (c.g[A0][0] as u32).wrapping_add(0x40));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x5c));
        'l_2cf470: loop {
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            let t = c.g[A2][0] != c.g[A3][0];
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002cf470);
                }
                continue 'l_2cf470;
            } else {
                'b_2cf60c: {
                    'b_2cf5f8: {
                        c.cycles += 85;
                        c.g[AT][0] = 0x43000000;
                        c.mtc1(1, 5);
                        c.g[A3][0] = 0x80;
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
                        c.mul_s(0, 0, 5);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.mul_s(1, 1, 5);
                        c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.mtc1(0, 3);
                        c.cvt_w_s(9, 0);
                        c.mfc1(6, 9);
                        c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x54));
                        c.cvt_w_s(0, 1);
                        c.mfc1(5, 0);
                        c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x5c));
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.add_s(7, 8, 7);
                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.mul_s(2, 2, 5);
                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.mul_s(0, 0, 5);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x128));
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x12c));
                        c.cvt_w_s(1, 2);
                        c.mfc1(4, 1);
                        c.cvt_w_s(1, 0);
                        c.mfc1(8, 1);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[A1][0]);
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[A0][0]);
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[A1][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[A2][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[A2][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), c.g[A3][0]);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x64));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x68));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xac), c.g[A3][0]);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x94));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x98));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[T0][0]);
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.add_s(6, 6, 0);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4c));
                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.mul_s(2, 2, 5);
                        c.mul_s(1, 1, 5);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[T0][0]);
                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.cvt_w_s(0, 2);
                        c.mfc1(4, 0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A3][0]);
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.cvt_w_s(0, 1);
                        c.mfc1(5, 0);
                        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[A0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[A1][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), c.g[A0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xdc), c.g[A3][0]);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xf4));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xf8));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.c_eq_s(0, 4);
                        let t = c.fcc();
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        if t {
                            break 'b_2cf5f8;
                        } else {
                            c.cycles += 6;
                            c.g[A1][0] = 0xffffffffffffff83;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                            c.g[V0][0] = c.g[V0][0] | 0x14;
                            break 'b_2cf60c;
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0xffffffffffffff83;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x4;
                    break 'b_2cf60c;
                }
                c.cycles += 95;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = 0xffffffffffffffff;
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = 0x4;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = 0u64;
                c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002cf644)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                c.g[A2][0] = c.g[S0][0];
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = 0x3;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(1, c.g[V1][0] as u32);
                c.g[A3][0] = 0u64;
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.mul_s(0, 0, 1);
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 7);
                c.g[AT][0] = 0x40e00000;
                c.mtc1(1, 8);
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 1);
                c.add_s(4, 4, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 9);
                c.sub_s(8, 5, 8);
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 0);
                c.sub_s(5, 5, 1);
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.sub_s(6, 4, 7);
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.mul_s(3, 3, 0);
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x124));
                c.mul_s(1, 1, 0);
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.mul_s(2, 2, 0);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                c.add_s(7, 4, 7);
                c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x12c));
                c.cvt_w_s(0, 3);
                c.mfc1(8, 0);
                c.cvt_w_s(0, 2);
                c.mfc1(4, 0);
                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.cvt_w_s(0, 1);
                c.mfc1(9, 0);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x124));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[T0][0]);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x124));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[T0][0]);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), c.g[A0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[A0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), c.g[T1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[T1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[T1][0]);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002cf75c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
                c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
                c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// int `MenuSysPoint2DItem_HandleInput(MenuSysPoint2DItem *item)` (0x2d0908): move the point with pad input or the axes.
pub fn menu_sys_point2d_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d0d28: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = 0u64;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            callee::MENU_SYS_ITEM_IS_DISABLED_4.call(c, 0x002d0930)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.g[S2][0];
            if t {
                break 'b_2d0d28;
            } else {
                'b_2d095c: {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = 0u64;
                    callee::MENU_SYS_ITEM_GET_INPUT_CODE_2.call(c, 0x002d0944)?;
                    c.g[V1][0] = 0x6;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_2d095c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        break 'b_2d095c;
                    }
                }
                'b_2d0d04: {
                    'b_2d0cf8: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_2d0cf8;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_INPUT_CODE_2.call(c, 0x002d096c)?;
                            c.g[V1][0] = 0x8;
                            if c.g[V0][0] == c.g[V1][0] {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                                break 'b_2d0d04;
                            } else {
                                'b_2d0994: {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::MENU_SYS_ITEM_GET_INPUT_CODE_2.call(c, 0x002d0980)?;
                                    c.g[V1][0] = 0x7;
                                    if c.g[V0][0] != c.g[V1][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                        break 'b_2d0994;
                                    } else {
                                        c.cycles += 2;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                                        break 'b_2d0d04;
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_2d0cf8;
                                } else {
                                    'b_2d09f8: {
                                        'b_2d09f4: {
                                            'b_2d09f0: {
                                                'b_2d09c4: {
                                                    c.cycles += 5;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    callee::MENU_SYS_ITEM_GET_INPUT_CODE_2
                                                        .call(c, 0x002d09a4)?;
                                                    c.g[V1][0] = 0x4;
                                                    let t = c.g[V0][0] != c.g[V1][0];

                                                    if t {
                                                        break 'b_2d09c4;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x24));
                                                        let t = (c.g[V0][0] as i64) <= 0;
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        if t {
                                                            break 'b_2d09f4;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[S1][0] = 0x1;
                                                            break 'b_2d09f0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S0][0];
                                                callee::MENU_SYS_ITEM_GET_INPUT_CODE_2
                                                    .call(c, 0x002d09cc)?;
                                                c.g[V1][0] = 0x5;
                                                let t = c.g[V0][0] != c.g[V1][0];
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                if t {
                                                    break 'b_2d09f8;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                    c.g[V0][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[V0][0] as i64))
                                                        as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        break 'b_2d09f4;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[S1][0] = 0x1;
                                                        break 'b_2d09f0;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x24),
                                                c.g[V0][0],
                                            );
                                            break 'b_2d09f4;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        break 'b_2d09f8;
                                    }
                                    'b_2d0bc0: {
                                        c.cycles += 22;
                                        c.g[V1][0] = 0x31;
                                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x2c));
                                        c.g[A1][0] = 0x32;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x34));
                                        c.g[V1][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        if c.g[V1][0] != 0 {
                                            c.g[A1][0] = c.g[V0][0];
                                        }
                                        c.sub_s(1, 1, 2);
                                        c.mtc1(5, 0);
                                        c.cvt_s_w(0, 0);
                                        c.div_s(21, 1, 0);
                                        c.g[A0][0] = c.g[S0][0];
                                        callee::MENU_SYS_ITEM_GET_UP_AXIS.call(c, 0x002d0a2c)?;
                                        c.mov_s(20, 0);
                                        c.g[A0][0] = c.g[S0][0];
                                        callee::MENU_SYS_ITEM_GET_DOWN_AXIS.call(c, 0x002d0a38)?;
                                        c.sub_s(20, 20, 0);
                                        c.mtc1(0, 0);
                                        c.c_eq_s(20, 0);
                                        let t = c.fcc();
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_2d0bc0;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    << ((c.g[A0][0] as u32) & 31));
                                            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                            let t = c.g[V1][0] != 0u64;
                                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                                            if t {
                                                break 'b_2d0bc0;
                                            } else {
                                                'b_2d0ac0: {
                                                    c.cycles += 21;
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S0][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                    c.sub_s(0, 0, 1);
                                                    c.lwc1(
                                                        2,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x3fa0),
                                                    );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.mul_s(0, 20, 0);
                                                    c.mul_s(0, 0, 2);
                                                    c.add_s(1, 1, 0);
                                                    c.swc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S0][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.c_lt_s(1, 0);
                                                    let t = !c.fcc();
                                                    c.g[S1][0] = 0x1;
                                                    if t {
                                                        break 'b_2d0ac0;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.swc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x4),
                                                        );
                                                        break 'b_2d0ac0;
                                                    }
                                                }
                                                'b_2d0ae8: {
                                                    c.cycles += 10;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                    c.lwc1(
                                                        1,
                                                        (c.g[S0][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.c_lt_s(0, 1);
                                                    if c.fcc() {
                                                        c.swc1(
                                                            1,
                                                            (c.g[V0][0] as u32).wrapping_add(0x4),
                                                        );
                                                        break 'b_2d0ae8;
                                                    } else {
                                                        break 'b_2d0ae8;
                                                    }
                                                }
                                                'b_2d0b64: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                    c.g[V0][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[V0][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x24));
                                                        break 'b_2d0b64;
                                                    } else {
                                                        'b_2d0b60: {
                                                            'b_2d0b2c: {
                                                                c.cycles += 3;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x3c));
                                                                let t = (c.g[A0][0] as i64) <= 0;
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 3);
                                                                if t {
                                                                    break 'b_2d0b2c;
                                                                } else {
                                                                    c.cycles += 8;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x28));
                                                                    c.g[V0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.lwc1(
                                                                        1,
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0xc),
                                                                    );
                                                                    c.lwc1(
                                                                        0,
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x4),
                                                                    );
                                                                    c.c_lt_s(1, 0);
                                                                    if c.fcc() {
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[V0][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        break 'b_2d0b60;
                                                                    } else {
                                                                        break 'b_2d0b2c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            if (c.g[A0][0] as i64) >= 0 {
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                break 'b_2d0b64;
                                                            } else {
                                                                c.cycles += 10;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 3);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                );
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.c_lt_s(0, 1);
                                                                if !c.fcc() {
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x24));
                                                                    break 'b_2d0b64;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    c.swc1(
                                                                        1,
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x4),
                                                                    );
                                                                    break 'b_2d0b60;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x24));
                                                        break 'b_2d0b64;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[A1][0] as i64) <= 0;

                                                if t {
                                                    break 'b_2d0bc0;
                                                } else {
                                                    'b_2d0b98: {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x3c));
                                                        let t = (c.g[A0][0] as i64) <= 0;
                                                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                                                        if t {
                                                            break 'b_2d0b98;
                                                        } else {
                                                            c.cycles += 8;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x28));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_sub(0x4),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.c_lt_s(0, 1);
                                                            if c.fcc() {
                                                                c.swc1(
                                                                    1,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                break 'b_2d0bc0;
                                                            } else {
                                                                break 'b_2d0b98;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = (c.g[A0][0] as i64) >= 0;
                                                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                                                    if t {
                                                        break 'b_2d0bc0;
                                                    } else {
                                                        c.cycles += 8;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x28));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        c.lwc1(
                                                            1,
                                                            (c.g[V1][0] as u32).wrapping_sub(0x4),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[V1][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.c_lt_s(1, 0);
                                                        if c.fcc() {
                                                            c.swc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            break 'b_2d0bc0;
                                                        } else {
                                                            break 'b_2d0bc0;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::MENU_SYS_ITEM_GET_RIGHT_AXIS.call(c, 0x002d0bc8)?;
                                    c.mov_s(20, 0);
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::MENU_SYS_ITEM_GET_LEFT_AXIS.call(c, 0x002d0bd4)?;
                                    c.sub_s(20, 20, 0);
                                    c.mtc1(0, 0);
                                    c.c_eq_s(20, 0);
                                    let t = c.fcc();
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_2d0d04;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
                                        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                                        if t {
                                            break 'b_2d0d04;
                                        } else {
                                            'b_2d0c5c: {
                                                c.cycles += 21;
                                                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x2c));
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x34));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                c.sub_s(0, 0, 1);
                                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3f9c));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.lwc1(1, c.g[V0][0] as u32);
                                                c.mul_s(0, 20, 0);
                                                c.mul_s(0, 0, 2);
                                                c.add_s(1, 1, 0);
                                                c.swc1(1, c.g[V0][0] as u32);
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x34));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.lwc1(0, c.g[V0][0] as u32);
                                                c.c_lt_s(1, 0);
                                                let t = !c.fcc();
                                                c.g[S1][0] = 0x1;
                                                if t {
                                                    break 'b_2d0c5c;
                                                } else {
                                                    c.cycles += 1;
                                                    c.swc1(1, c.g[V0][0] as u32);
                                                    break 'b_2d0c5c;
                                                }
                                            }
                                            'b_2d0c84: {
                                                c.cycles += 10;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x2c));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.lwc1(0, c.g[V0][0] as u32);
                                                c.c_lt_s(0, 1);
                                                if c.fcc() {
                                                    c.swc1(1, c.g[V0][0] as u32);
                                                    break 'b_2d0c84;
                                                } else {
                                                    break 'b_2d0c84;
                                                }
                                            }
                                            'b_2d0cc0: {
                                                c.cycles += 6;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                c.g[V0][0] = ((c.g[A0][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                                                if t {
                                                    break 'b_2d0cc0;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V0][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.lwc1(1, c.g[V0][0] as u32);
                                                    c.sub_s(0, 0, 21);
                                                    c.c_lt_s(0, 1);
                                                    if c.fcc() {
                                                        c.swc1(0, c.g[V0][0] as u32);
                                                        break 'b_2d0cc0;
                                                    } else {
                                                        break 'b_2d0cc0;
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                            let t = (c.g[V0][0] as i64) <= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                            if t {
                                                break 'b_2d0d04;
                                            } else {
                                                c.cycles += 9;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(0x8));
                                                c.lwc1(1, c.g[V0][0] as u32);
                                                c.add_s(0, 0, 21);
                                                c.c_lt_s(1, 0);
                                                if c.fcc() {
                                                    c.swc1(0, c.g[V0][0] as u32);
                                                    break 'b_2d0d04;
                                                } else {
                                                    c.cycles += 2;

                                                    break 'b_2d0d04;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MENU_SYS_ITEM_HANDLE_INPUT_2.call(c, 0x002d0d00)?;
                    c.g[S2][0] = c.g[V0][0];
                    break 'b_2d0d04;
                }
                'b_2d0d24: {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        break 'b_2d0d24;
                    } else {
                        c.cycles += 6;
                        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002d0d24)?;
                        break 'b_2d0d24;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
                break 'b_2d0d28;
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// void `MenuSysPage_Draw(MenuSysPage *page, ...)` (0x2cad58): draw every visible item, then the scroll arrows.
pub fn menu_sys_page_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4e0));
        c.g[V0][0] = 0x45;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x4a0));
        c.g[V1][0] = 0xffffffffffffffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x4c0));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x4b0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x490));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x480), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4d0));
        'l_2cad80: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[S3][0] = 0u64;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002cad80);
                }
                continue 'l_2cad80;
            } else {
                'b_2cae60: {
                    c.cycles += 7;
                    c.mtc1(0, 0);
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.swc1(0, c.g[SP][0] as u32);
                    if t {
                        break 'b_2cae60;
                    } else {
                        c.cycles += 4;
                        c.mov_s(20, 0);
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                        'l_2cadc8: loop {
                            'b_2cae4c: {
                                'b_2cae48: {
                                    'b_2cae40: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                        callee::MENU_SYS_ITEM_IS_VISIBLE_2.call(c, 0x002cadd0)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x470));
                                        if t {
                                            break 'b_2cae40;
                                        } else {
                                            c.cycles += 24;
                                            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A1][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                            call_ptr(c, t, 0x002cadf0)?;
                                            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x477));
                                            c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x470));
                                            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x47f));
                                            c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x478));
                                            c.sdl(2, (c.g[S0][0] as u32).wrapping_sub(0x1));
                                            c.sdr(2, (c.g[S0][0] as u32).wrapping_sub(0x8));
                                            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
                                            c.sdr(3, c.g[S0][0] as u32);
                                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                                            c.add_s(0, 0, 1);
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                            c.lwc1(0, c.g[S0][0] as u32);
                                            c.c_lt_s(2, 0);
                                            if c.fcc() {
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                break 'b_2cae48;
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                                break 'b_2cae4c;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.swc1(20, c.g[S0][0] as u32);
                                    break 'b_2cae48;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                break 'b_2cae4c;
                            }
                            c.cycles += 5;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002cadc8);
                                }
                                continue 'l_2cadc8;
                            } else {
                                break 'b_2cae60;
                            }
                        }
                    }
                }
                'b_2cafc8: {
                    'b_2cafa8: {
                        c.cycles += 19;
                        c.g[V1][0] = 0x1e0;
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[AT][0] = 0x44200000;
                        c.mtc1(1, 0);
                        c.mtc1(3, 1);
                        c.cvt_s_w(1, 1);
                        c.g[AT][0] = 0x42d40000;
                        c.mtc1(1, 3);
                        c.sub_s(0, 0, 2);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 4);
                        c.sub_s(1, 1, 3);
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
                        c.mul_s(0, 0, 4);
                        c.c_lt_s(1, 2);
                        let t = !c.fcc();
                        c.swc1(0, c.g[SP][0] as u32);
                        if t {
                            break 'b_2cafa8;
                        } else {
                            'b_2caec0: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    break 'b_2caec0;
                                } else {
                                    break 'b_2caec0;
                                }
                            }
                            'b_2caf04: {
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                c.g[A2][0] = c.g[V1][0];
                                c.mtc1(0, 1);
                                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[T0][0] = c.g[A0][0];
                                if t {
                                    break 'b_2caf04;
                                } else {
                                    c.cycles += 3;
                                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
                                    c.g[A1][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    'l_2caee8: loop {
                                        c.cycles += 7;
                                        c.lwc1(0, c.g[A1][0] as u32);
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] =
                                            ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.add_s(1, 1, 0);
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002caee8);
                                            }
                                            continue 'l_2caee8;
                                        } else {
                                            break 'b_2caf04;
                                        }
                                    }
                                }
                            }
                            'b_2caf98: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x1e0;
                                if t {
                                    break 'b_2caf98;
                                } else {
                                    c.cycles += 12;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
                                    c.g[AT][0] = 0x42d40000;
                                    c.mtc1(1, 2);
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                    c.mtc1(2, 0);
                                    c.cvt_s_w(0, 0);
                                    c.sub_s(0, 0, 2);
                                    c.c_lt_s(0, 1);
                                    let t = !c.fcc();
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_2caf98;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                        c.g[A3][0] = 0x1e0;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        'l_2caf50: loop {
                                            c.cycles += 8;
                                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 4);
                                            c.g[V1][0] =
                                                sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.lwc1(0, c.g[V1][0] as u32);
                                            c.g[A0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                            let t = c.g[A0][0] == 0u64;
                                            c.sub_s(1, 1, 0);
                                            if t {
                                                break 'b_2caf98;
                                            } else {
                                                c.cycles += 10;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                                                c.g[V1][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_sub(c.g[V1][0] as u32));
                                                c.mtc1(3, 0);
                                                c.cvt_s_w(0, 0);
                                                c.sub_s(0, 0, 2);
                                                c.c_lt_s(0, 1);
                                                if c.fcc() {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002caf50);
                                                    }
                                                    continue 'l_2caf50;
                                                } else {
                                                    break 'b_2caf98;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[AT][0] = 0x42540000;
                            c.mtc1(1, 0);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                            break 'b_2cafc8;
                        }
                    }
                    c.cycles += 8;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    c.sub_s(0, 0, 2);
                    c.mul_s(0, 0, 4);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    break 'b_2cafc8;
                }
                'b_2cb0d0: {
                    'b_2cb0cc: {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S2][0];
                        callee::MENU_SYS_PAGE_BEGIN_DRAW.call(c, 0x002cafd0)?;
                        c.g[S3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                        if t {
                            break 'b_2cb0cc;
                        } else {
                            c.cycles += 3;
                            c.g[S0][0] = sx((c.g[S3][0] as u32) << 4);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            'l_2caff0: loop {
                                'b_2cb0b8: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    callee::MENU_SYS_ITEM_IS_VISIBLE_2.call(c, 0x002caff8)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                        break 'b_2cb0b8;
                                    } else {
                                        'b_2cb050: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                            let t = c.g[S3][0] == c.g[V0][0];
                                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c));
                                            if t {
                                                break 'b_2cb050;
                                            } else {
                                                c.cycles += 17;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
                                                c.g[V0][0] = 0x1e0;
                                                c.g[A1][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.g[V1][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
                                                c.lwc1(3, c.g[A1][0] as u32);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[V1][0] as u32));
                                                c.g[AT][0] = 0x42540000;
                                                c.mtc1(1, 2);
                                                c.mtc1(2, 0);
                                                c.cvt_s_w(0, 0);
                                                c.add_s(1, 1, 3);
                                                c.sub_s(0, 0, 2);
                                                c.c_lt_s(0, 1);
                                                if c.fcc() {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    break 'b_2cb0d0;
                                                } else {
                                                    break 'b_2cb050;
                                                }
                                            }
                                        }
                                        c.cycles += 26;
                                        c.lwc1(0, c.g[SP][0] as u32);
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.swc1(0, c.g[A1][0] as u32);
                                        c.g[V1][0] =
                                            sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[A0][0] = c.g[V1][0];
                                        c.g[A3][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x64));
                                        c.cvt_s_w(1, 1);
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                                        c.add_s(1, 1, 0);
                                        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.lwc1(1, c.g[A3][0] as u32);
                                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.add_s(0, 0, 1);
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x002cb0b4)?;
                                        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                        break 'b_2cb0b8;
                                    }
                                }
                                c.cycles += 5;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002caff0);
                                    }
                                    continue 'l_2caff0;
                                } else {
                                    break 'b_2cb0cc;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    break 'b_2cb0d0;
                }
                'b_2cb11c: {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'b_2cb11c;
                    } else {
                        c.cycles += 17;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x124));
                        c.g[AT][0] = 0x42000000;
                        c.mtc1(1, 4);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x470));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
                        c.cvt_s_w(0, 0);
                        c.g[AT][0] = 0x41b00000;
                        c.mtc1(1, 3);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4000));
                        c.g[AT][0] = 0x41700000;
                        c.mtc1(1, 2);
                        c.add_s(0, 0, 4);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x470));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x478));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x47c));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x474));
                        callee::MENU_SYS_PAGE_DRAW_SCROLL_ARROW_DOWN.call(c, 0x002cb11c)?;
                        break 'b_2cb11c;
                    }
                }
                'b_2cb158: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        break 'b_2cb158;
                    } else {
                        c.cycles += 12;
                        c.g[AT][0] = 0x41b00000;
                        c.mtc1(1, 3);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3ffc));
                        c.g[AT][0] = 0x41700000;
                        c.mtc1(1, 2);
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ff8));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x470));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x470));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x474));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x478));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x47c));
                        callee::MENU_SYS_PAGE_DRAW_SCROLL_ARROW_UP.call(c, 0x002cb158)?;
                        break 'b_2cb158;
                    }
                }
                c.cycles += 10;
                c.g[A0][0] = c.g[S2][0];
                callee::MENU_SYS_PAGE_END_DRAW.call(c, 0x002cb160)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x4c0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x4b0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x4a0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x490));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x480));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x4d0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4e0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// void `MenuSysTextEditItem_Draw(MenuSysTextEditItem *item, ...)` (0x2ce910): draw the item row and the edit text.
pub fn menu_sys_text_edit_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ce998: {
            'b_2ce994: {
                c.cycles += 24;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.g[S2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x178));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                callee::MENU_SYS_PAGE_GET_SELECTED_ITEM.call(c, 0x002ce94c)?;
                c.g[V0][0] = c.g[S1][0] ^ c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[S3][0] = (c.g[V0][0] < 0x1u64) as u64;
                call_ptr(c, 0x004166f4, 0x002ce960)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416810, 0x002ce968)?;
                let t = c.g[S3][0] == 0u64;
                c.g[V1][0] = c.g[V0][0];
                if t {
                    break 'b_2ce994;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                        break 'b_2ce998;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x4a0000;
                        if t {
                            break 'b_2ce994;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x39d0));
                            call_ptr(c, 0x004162d0, 0x002ce994)?;
                            break 'b_2ce994;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            break 'b_2ce998;
        }
        'b_2ce9c0: {
            'b_2ce9b8: {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x12c));
                c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x44));
                let t = c.g[S3][0] == 0u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if t {
                    break 'b_2ce9b8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4d0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
                    break 'b_2ce9c0;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x4d0000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5390));
            break 'b_2ce9c0;
        }
        'b_2ce9ec: {
            'b_2ce9e8: {
                c.cycles += 7;
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                let t = c.g[V0][0] as u32;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, t, 0x002ce9cc)?;
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                callee::MENU_SYS_PAGE_GET_SELECTED_ITEM.call(c, 0x002ce9d4)?;
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x4d0000;
                    break 'b_2ce9e8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4d0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                    break 'b_2ce9ec;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
            break 'b_2ce9ec;
        }
        'b_2ceaa8: {
            c.cycles += 34;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 22);
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[AT][0] = 0x40000000;
            c.mtc1(1, 2);
            c.lwc1(0, c.g[V0][0] as u32);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
            c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x2f));
            c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
            c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
            c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            let t = c.g[A1][0] == 0u64;
            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
            if t {
                break 'b_2ceaa8;
            } else {
                c.cycles += 13;
                c.lwc1(13, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 0);
                c.add_s(13, 13, 0);
                c.lwc1(1, c.g[S2][0] as u32);
                c.g[AT][0] = 0x41800000;
                c.mtc1(1, 12);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.add_s(12, 1, 12);
                c.sub_s(13, 13, 22);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                call_ptr(c, 0x00391cb0, 0x002ceaa4)?;
                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                break 'b_2ceaa8;
            }
        }
        'b_2cecd0: {
            c.cycles += 27;
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
            c.g[AT][0] = 0x40c00000;
            c.mtc1(1, 23);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
            c.g[AT][0] = 0x41800000;
            c.mtc1(1, 21);
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            call_ptr(c, 0x00391fb0, 0x002cead8)?;
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(12, c.g[S2][0] as u32);
            c.sub_s(1, 1, 0);
            c.lwc1(13, (c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
            c.add_s(13, 13, 23);
            c.add_s(12, 12, 1);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
            c.sub_s(13, 13, 22);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
            c.sub_s(12, 12, 21);
            call_ptr(c, 0x00391cb0, 0x002ceb0c)?;
            let t = c.g[S3][0] == 0u64;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_2cecd0;
            } else {
                c.cycles += 41;
                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                c.g[A3][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                call_ptr(c, 0x00391fb0, 0x002ceb38)?;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                c.mov_s(20, 0);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                call_ptr(c, 0x00391fb0, 0x002ceb68)?;
                c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0xffffffffffffffff;
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.sub_s(0, 2, 0);
                c.lwc1(3, c.g[S2][0] as u32);
                c.sub_s(2, 2, 20);
                c.add_s(1, 1, 4);
                c.g[AT][0] = 0x40800000;
                c.mtc1(1, 5);
                c.add_s(0, 3, 0);
                c.g[V0][0] = 0x3;
                c.add_s(3, 3, 2);
                c.sub_s(1, 1, 23);
                c.sub_s(7, 0, 21);
                c.sub_s(12, 3, 21);
                c.sub_s(8, 1, 22);
                c.sub_s(6, 1, 5);
                'l_2cebb8: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[T2][0] = 0xffffffffffffff83;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002cebb8);
                        }
                        continue 'l_2cebb8;
                    } else {
                        c.cycles += 63;
                        c.mtc1(0, 4);
                        c.g[V0][0] = 0x7f;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 5);
                        c.g[AT][0] = 0x42fe0000;
                        c.mtc1(1, 1);
                        c.g[T3][0] = 0xffffffffffffffff;
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.g[A1][0] = 0x4;
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.g[A3][0] = 0u64;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
                        c.mul_s(2, 2, 1);
                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x104));
                        c.mul_s(3, 3, 1);
                        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.mul_s(0, 0, 1);
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x108));
                        c.cvt_w_s(1, 2);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x10c));
                        c.cvt_w_s(1, 3);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                        c.cvt_w_s(1, 0);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
                        c.lq(9, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x104));
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.lq(8, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x104));
                        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.sq(8, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.sq(9, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x104));
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                        c.g[V0][0] = c.g[V0][0] | 0x4;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                        c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002cecd0)?;
                        break 'b_2cecd0;
                    }
                }
            }
        }
        c.cycles += 20;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x178));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
        c.pc = t;
        return Ok(());
    });
}

/// void `MenuSysPage_DrawFrame(MenuSysPage *page, ...)` (0x2cb498): draw the page frame quads.
pub fn menu_sys_page_draw_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
        c.g[T4][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[V0][0] = 0x3;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[V1][0] = 0xffffffffffffffff;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x188));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x178));
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        'l_2cb4f8: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002cb4f8);
                }
                continue 'l_2cb4f8;
            } else {
                c.cycles += 23;
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 1);
                c.g[V1][0] = c.g[SP][0];
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(2, c.g[A2][0] as u32);
                c.mul_s(0, 0, 1);
                c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.mul_s(4, 4, 1);
                c.mul_s(2, 2, 1);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 5);
                c.mul_s(3, 3, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(6, 1);
                c.cvt_w_s(1, 2);
                c.mfc1(7, 1);
                c.cvt_w_s(1, 3);
                c.mfc1(5, 1);
                c.cvt_w_s(0, 4);
                c.mfc1(4, 0);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                'l_2cb570: loop {
                    c.cycles += 8;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A2][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x002cb570);
                        }
                        continue 'l_2cb570;
                    } else {
                        c.cycles += 187;
                        c.g[T1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[S1][0] = 0xffffffffffffffff;
                        c.g[S2][0] = 0xfffffffffffffc1f;
                        c.g[T3][0] = 0xffffffffffffff83;
                        c.g[T2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xe84));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[AT][0] = 0x40a00000;
                        c.mtc1(1, 20);
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A3][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = c.g[S5][0];
                        c.g[AT][0] = 0x3e800000;
                        c.mtc1(1, 21);
                        c.g[S0][0] = c.g[A3][0] & 0x3e0;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 2);
                        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 5);
                        c.g[A3][0] = c.g[A3][0] & c.g[S2][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
                        c.g[AT][0] = 0x3f400000;
                        c.mtc1(1, 26);
                        c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 27);
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = 0u64;
                        }
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                        c.g[V0][0] = c.g[V0][0] & 0x3e0;
                        c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
                        c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        c.g[T0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                        c.g[V0][0] = c.g[V0][0] | 0x14;
                        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x124));
                        c.g[A3][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xe84));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x60));
                        c.sh((c.g[A3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.lwc1(0, c.g[S3][0] as u32);
                        c.lwc1(24, (c.g[S3][0] as u32).wrapping_add(0x8));
                        c.add_s(29, 1, 20);
                        c.lwc1(25, (c.g[S3][0] as u32).wrapping_add(0xc));
                        c.add_s(28, 0, 20);
                        c.add_s(23, 0, 24);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.add_s(22, 1, 25);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.sub_s(25, 25, 2);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.sub_s(24, 24, 2);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.sub_s(23, 23, 20);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
                        c.sub_s(22, 22, 20);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb67c)?;
                        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb6b0)?;
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd4));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb6e0)?;
                        c.lwc1(0, c.g[S3][0] as u32);
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd4));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb714)?;
                        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb748)?;
                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd4));
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb778)?;
                        c.lwc1(0, c.g[S3][0] as u32);
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd4));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb7ac)?;
                        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd4));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb7dc)?;
                        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[A1][0] = c.g[S4][0];
                        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0xc4));
                        c.g[A2][0] = c.g[S5][0];
                        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xc8));
                        c.g[A0][0] = c.g[SP][0];
                        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xcc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd4));
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        callee::MENU_SYS_BUILD_QUAD.call(c, 0x002cb80c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[S1][0] = ((c.g[S1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                        if c.g[S1][0] == 0 {
                            c.g[S0][0] = 0u64;
                        }
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
                        c.g[S0][0] = sx((c.g[S0][0] as u32) << 5);
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                        c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x198));
                        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x190));
                        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x188));
                        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x180));
                        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x178));
                        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x170));
                        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x168));
                        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
                        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
                        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// int `MenuSysTextEditItem_HandleInput(MenuSysTextEditItem *item)` (0x2ce488): edit the text with pad input.
pub fn menu_sys_text_edit_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ce7fc: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            callee::MENU_SYS_ITEM_HANDLE_INPUT_2.call(c, 0x002ce4ac)?;
            c.g[S4][0] = c.g[V0][0];
            let t = c.g[S4][0] != 0u64;
            c.g[V0][0] = c.g[S4][0];
            if t {
                break 'b_2ce7fc;
            } else {
                'b_2ce7f8: {
                    'b_2ce530: {
                        'b_2ce520: {
                            'b_2ce508: {
                                c.cycles += 14;
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                                c.g[S3][0] = c.g[A0][0];
                                call_ptr(c, 0x00416810, 0x002ce4c4)?;
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V1][0] = 0xf;
                                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[V0][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
                                c.g[RA][0] = sx(0x002ce4e0);
                                if c.g[V0][0] == 0 {
                                    c.g[S0][0] = c.g[V1][0];
                                }
                                callee::MENU_SYS_ITEM_GET_INPUT_CODE_2.call(c, 0x002ce4e0)?;
                                c.g[V1][0] = c.g[V0][0];
                                c.g[V0][0] = 0x2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[V0][0] = 0x3;
                                    break 'b_2ce508;
                                } else {
                                    'b_2ce4fc: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                        if c.g[S0][0] == 0u64 {
                                            break 'b_2ce4fc;
                                        } else {
                                            break 'b_2ce4fc;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    break 'b_2ce520;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x4;
                            if t {
                                break 'b_2ce530;
                            } else {
                                'b_2ce51c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                    if c.g[S0][0] == 0u64 {
                                        break 'b_2ce51c;
                                    } else {
                                        break 'b_2ce51c;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                break 'b_2ce520;
                            }
                        }
                        c.cycles += 4;
                        c.div(0, 2, 16);
                        c.g[V1][0] = c.hi[0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                        break 'b_2ce7f8;
                    }
                    'b_2ce5a4: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_2ce5a4;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                            c.g[A3][0] = 0x5f;
                            c.g[A2][0] = 0x21;
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                            'l_2ce550: loop {
                                'b_2ce594: {
                                    'b_2ce590: {
                                        c.cycles += 8;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                                        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
                                        c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A1][0] = 0u64;
                                        if t {
                                            break 'b_2ce590;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
                                            c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A1][0] = 0x1;
                                                break 'b_2ce594;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[V1][0] == c.g[A3][0] {
                                                    c.g[A1][0] = 0x1;
                                                    break 'b_2ce594;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[A2][0];

                                                    if t {
                                                        break 'b_2ce594;
                                                    } else {
                                                        break 'b_2ce590;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A1][0] = 0x1;
                                    break 'b_2ce594;
                                }
                                c.cycles += 2;
                                if c.g[A1][0] == 0u64 {
                                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002ce550);
                                    }
                                    continue 'l_2ce550;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.g[S4][0];
                                    break 'b_2ce7fc;
                                }
                            }
                        }
                    }
                    'b_2ce61c: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x9;
                        if t {
                            break 'b_2ce61c;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                            c.g[A3][0] = 0x5f;
                            c.g[A2][0] = 0x21;
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                            'l_2ce5c8: loop {
                                'b_2ce60c: {
                                    'b_2ce608: {
                                        c.cycles += 8;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                                        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
                                        c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A1][0] = 0u64;
                                        if t {
                                            break 'b_2ce608;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
                                            c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A1][0] = 0x1;
                                                break 'b_2ce60c;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[V1][0] == c.g[A3][0] {
                                                    c.g[A1][0] = 0x1;
                                                    break 'b_2ce60c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[A2][0];

                                                    if t {
                                                        break 'b_2ce60c;
                                                    } else {
                                                        break 'b_2ce608;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A1][0] = 0x1;
                                    break 'b_2ce60c;
                                }
                                c.cycles += 2;
                                if c.g[A1][0] == 0u64 {
                                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002ce5c8);
                                    }
                                    continue 'l_2ce5c8;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.g[S4][0];
                                    break 'b_2ce7fc;
                                }
                            }
                        }
                    }
                    'b_2ce678: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0xa;
                        if t {
                            break 'b_2ce678;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[S4][0];
                            if t {
                                break 'b_2ce7fc;
                            } else {
                                'b_2ce668: {
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                        break 'b_2ce668;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        'l_2ce648: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] =
                                                ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                            c.sb((c.g[A0][0] as u32).wrapping_add(0x1), c.g[V0][0]);
                                            let t = c.g[V1][0] != 0u64;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002ce648);
                                                }
                                                continue 'l_2ce648;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                                break 'b_2ce668;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V1][0] = 0x5f;
                                c.g[V0][0] =
                                    sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                                break 'b_2ce7f8;
                            }
                        }
                    }
                    'b_2ce6d8: {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_2ce6d8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S2][0] == 0u64;
                            c.g[V0][0] = c.g[S4][0];
                            if t {
                                break 'b_2ce7fc;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                if t {
                                    break 'b_2ce7f8;
                                } else {
                                    'b_2ce6cc: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[V1][0];
                                        c.g[V0][0] =
                                            ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[T0][0] = c.g[A2][0];
                                        if t {
                                            break 'b_2ce6cc;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A1][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                            'l_2ce6b0: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = ((c.g[A0][0] as i64)
                                                    < (c.g[A2][0] as i64))
                                                    as u64;
                                                c.sb(c.g[A1][0] as u32, c.g[V0][0]);
                                                let t = c.g[V1][0] != 0u64;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002ce6b0);
                                                    }
                                                    continue 'l_2ce6b0;
                                                } else {
                                                    break 'b_2ce6cc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                    c.sb(c.g[V0][0] as u32, 0u64);
                                    break 'b_2ce7f8;
                                }
                            }
                        }
                    }
                    'b_2ce6e8: {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                            break 'b_2ce6e8;
                        } else {
                            c.cycles += 2;
                            c.g[S4][0] = 0x3;
                            break 'b_2ce7f8;
                        }
                    }
                    'b_2ce7f4: {
                        'b_2ce768: {
                            c.cycles += 10;
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            callee::MENU_SYS_GET_KEY_CODE.call(c, 0x002ce6f0)?;
                            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                            callee::MENU_SYS_GET_KEY_CHAR.call(c, 0x002ce700)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[V0][0] = 0x8;
                            let t = c.g[S0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[A2][0] as u32) << 24);
                            if t {
                                break 'b_2ce768;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S2][0] == 0u64;
                                c.g[V0][0] = c.g[S4][0];
                                if t {
                                    break 'b_2ce7fc;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_2ce7f8;
                                    } else {
                                        'b_2ce754: {
                                            c.cycles += 4;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[V0][0] =
                                                ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[T0][0] = c.g[A3][0];
                                            if t {
                                                break 'b_2ce754;
                                            } else {
                                                c.cycles += 1;
                                                c.g[A1][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[S3][0] as u32));
                                                'l_2ce738: loop {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = c
                                                        .lbu((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                    c.g[V1][0] = ((c.g[A2][0] as i64)
                                                        < (c.g[A3][0] as i64))
                                                        as u64;
                                                    c.sb(c.g[A1][0] as u32, c.g[V0][0]);
                                                    let t = c.g[V1][0] != 0u64;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x002ce738);
                                                        }
                                                        continue 'l_2ce738;
                                                    } else {
                                                        break 'b_2ce754;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[V1][0] =
                                            sx((c.g[S3][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                        c.sb(c.g[V1][0] as u32, 0u64);
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        break 'b_2ce7f4;
                                    }
                                }
                            }
                        }
                        'b_2ce7a4: {
                            'b_2ce7a0: {
                                c.cycles += 5;
                                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
                                c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = 0u64;
                                if t {
                                    break 'b_2ce7a0;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
                                    c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A0][0] = 0x1;
                                        break 'b_2ce7a4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x5f;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x21;
                                        if t {
                                            break 'b_2ce7a0;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];

                                            if t {
                                                break 'b_2ce7a4;
                                            } else {
                                                break 'b_2ce7a0;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = 0x1;
                            break 'b_2ce7a4;
                        }
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
                        if t {
                            break 'b_2ce7f8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[S4][0];
                            if t {
                                break 'b_2ce7fc;
                            } else {
                                'b_2ce7e8: {
                                    c.cycles += 4;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                        break 'b_2ce7e8;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        'l_2ce7c8: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] =
                                                ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                            c.sb((c.g[A0][0] as u32).wrapping_add(0x1), c.g[V0][0]);
                                            let t = c.g[V1][0] != 0u64;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x002ce7c8);
                                                }
                                                continue 'l_2ce7c8;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                                                break 'b_2ce7e8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V1][0] =
                                    sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.sb(c.g[V1][0] as u32, c.g[A2][0]);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                break 'b_2ce7f4;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                    break 'b_2ce7f8;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S4][0];
                break 'b_2ce7fc;
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// void `MenuSysColorChannelItem_Measure(MenuSysColorChannelItem *item, ...)` (0x2ceee8): measure the channel row.
pub fn menu_sys_color_channel_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        'b_2cf138: {
            'b_2cf110: {
                'b_2cf080: {
                    'b_2cf04c: {
                        c.cycles += 75;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.g[S0][0] = 0x480000;
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6700));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[S3][0] = c.g[A1][0];
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[S2][0] = 0x480000;
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7470));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        c.g[S6][0] = c.g[A0][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
                        c.g[S5][0] = c.g[S2][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.lwc1(12, c.g[V1][0] as u32);
                        call_ptr(c, 0x00413af8, 0x002cef4c)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x00417828, 0x002cef5c)?;
                        c.mtc1(0, 0);
                        c.g[A2][0] = 0u64;
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        c.g[A3][0] = 0u64;
                        c.swc1(0, c.g[SP][0] as u32);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[AT][0] = 0x41400000;
                        c.mtc1(1, 2);
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
                        c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                        c.cvt_s_w(0, 0);
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
                        c.mul_s(0, 0, 1);
                        c.add_s(0, 0, 2);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                        call_ptr(c, 0x00391fb0, 0x002cefb4)?;
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        c.g[A2][0] = 0u64;
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x7470));
                        c.g[A3][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                        c.mov_s(21, 0);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                        call_ptr(c, 0x00391fb0, 0x002cefdc)?;
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        c.mov_s(20, 0);
                        c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                        call_ptr(c, 0x00391fb0, 0x002cf004)?;
                        c.c_lt_s(0, 20);
                        let t = !c.fcc();
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_2cf04c;
                        } else {
                            c.cycles += 12;
                            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                            c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x7470));
                            c.g[A3][0] = 0u64;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                            call_ptr(c, 0x00391fb0, 0x002cf034)?;
                            c.c_lt_s(21, 0);
                            if c.fcc() {
                                c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                                break 'b_2cf080;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7470));
                                break 'b_2cf110;
                            }
                        }
                    }
                    c.cycles += 12;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0u64;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                    call_ptr(c, 0x00391fb0, 0x002cf06c)?;
                    c.c_lt_s(21, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7470));
                    if t {
                        break 'b_2cf110;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        break 'b_2cf080;
                    }
                }
                'b_2cf0f0: {
                    'b_2cf0e8: {
                        c.cycles += 23;
                        c.g[A2][0] = 0u64;
                        c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x7470));
                        c.g[A3][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                        c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7470));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                        call_ptr(c, 0x00391fb0, 0x002cf0a4)?;
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        c.mov_s(20, 0);
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                        call_ptr(c, 0x00391fb0, 0x002cf0cc)?;
                        c.c_lt_s(0, 20);
                        let t = !c.fcc();
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_2cf0e8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                            c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x7470));
                            break 'b_2cf0f0;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    break 'b_2cf0f0;
                }
                c.cycles += 8;
                c.g[A3][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                call_ptr(c, 0x00391fb0, 0x002cf108)?;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                break 'b_2cf138;
            }
            c.cycles += 10;
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            call_ptr(c, 0x00391fb0, 0x002cf134)?;
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
            break 'b_2cf138;
        }
        c.cycles += 36;
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
        call_ptr(c, 0x00391fb0, 0x002cf15c)?;
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[S6][0];
        c.g[AT][0] = 0x43100000;
        c.mtc1(1, 2);
        c.add_s(1, 1, 0);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.add_s(1, 1, 2);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[SP][0] as u32);
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[S6][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[S6][0] as u32);
        c.sdl(4, (c.g[S6][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[S6][0] as u32).wrapping_add(0x8));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        return Ok(());
    });
}

/// void `MenuSysItem_MeasureRow(...)` (0x2ca4c8): measure a row into the 16-byte layout.
pub fn menu_sys_item_measure_row(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ca6ec: {
            'b_2ca6e8: {
                'b_2ca6dc: {
                    'b_2ca654: {
                        'b_2ca64c: {
                            c.cycles += 34;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                            c.mtc1(0, 0);
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[S0][0] = c.g[A1][0];
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[S1][0] = c.g[T0][0];
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[S2][0] = c.g[A3][0];
                            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[S3][0] = c.g[A2][0];
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                            c.g[S4][0] = c.g[A0][0];
                            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            c.swc1(0, c.g[SP][0] as u32);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x124));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                            c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x34));
                            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x14));
                            c.cvt_s_w(0, 0);
                            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x38));
                            c.div_s(2, 2, 4);
                            c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x30));
                            c.mul_s(0, 0, 4);
                            c.div_s(1, 1, 3);
                            c.mul_s(0, 0, 2);
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                            let t = c.g[S3][0] == 0u64;
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                            if t {
                                break 'b_2ca64c;
                            } else {
                                'b_2ca61c: {
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == 0u64;

                                    if t {
                                        break 'b_2ca61c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;
                                        c.g[A2][0] = 0u64;
                                        if t {
                                            break 'b_2ca61c;
                                        } else {
                                            'b_2ca5ec: {
                                                'b_2ca5e4: {
                                                    c.cycles += 31;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[A1][0] as u32).wrapping_add(0x124)
                                                        );
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A3][0] = 0u64;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x58));
                                                    c.lwc1(
                                                        12,
                                                        (c.g[A0][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.lwc1(
                                                        13,
                                                        (c.g[A0][0] as u32).wrapping_add(0x3c),
                                                    );
                                                    call_ptr(c, 0x00391fb0, 0x002ca57c)?;
                                                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                                    c.g[A1][0] = c.g[S3][0];
                                                    c.g[A2][0] = 0u64;
                                                    c.g[A3][0] = 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x124)
                                                        );
                                                    c.mov_s(21, 0);
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x58));
                                                    c.lwc1(
                                                        12,
                                                        (c.g[A0][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.lwc1(
                                                        13,
                                                        (c.g[A0][0] as u32).wrapping_add(0x3c),
                                                    );
                                                    call_ptr(c, 0x00391fb0, 0x002ca5a4)?;
                                                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                                    c.mov_s(20, 0);
                                                    c.g[A1][0] = c.g[S1][0];
                                                    c.g[A2][0] = 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x124)
                                                        );
                                                    c.g[A3][0] = 0u64;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x58));
                                                    c.lwc1(
                                                        12,
                                                        (c.g[A0][0] as u32).wrapping_add(0x38),
                                                    );
                                                    c.lwc1(
                                                        13,
                                                        (c.g[A0][0] as u32).wrapping_add(0x3c),
                                                    );
                                                    call_ptr(c, 0x00391fb0, 0x002ca5cc)?;
                                                    c.c_lt_s(0, 20);
                                                    let t = !c.fcc();
                                                    c.g[A1][0] = c.g[S3][0];
                                                    if t {
                                                        break 'b_2ca5e4;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                                        break 'b_2ca5ec;
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                                c.g[A1][0] = c.g[S1][0];
                                                break 'b_2ca5ec;
                                            }
                                            c.cycles += 12;
                                            c.g[A2][0] = 0u64;
                                            c.g[A3][0] = 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                                            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                            call_ptr(c, 0x00391fb0, 0x002ca608)?;
                                            c.add_s(0, 0, 0);
                                            c.g[AT][0] = 0x42000000;
                                            c.mtc1(1, 1);
                                            c.add_s(0, 21, 0);
                                            break 'b_2ca6dc;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S3][0] == 0u64;
                                c.g[A1][0] = c.g[S3][0];
                                if t {
                                    break 'b_2ca64c;
                                } else {
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                                    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                    call_ptr(c, 0x00391fb0, 0x002ca644)?;
                                    c.mov_s(20, 0);
                                    break 'b_2ca654;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.mtc1(0, 0);
                        c.mov_s(20, 0);
                        break 'b_2ca654;
                    }
                    'b_2ca680: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.g[A1][0] = c.g[S2][0];
                        if t {
                            break 'b_2ca680;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                            call_ptr(c, 0x00391fb0, 0x002ca67c)?;
                            c.add_s(20, 20, 0);
                            break 'b_2ca680;
                        }
                    }
                    'b_2ca6ac: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == 0u64;
                        c.g[A1][0] = c.g[S1][0];
                        if t {
                            break 'b_2ca6ac;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                            call_ptr(c, 0x00391fb0, 0x002ca6a8)?;
                            c.add_s(20, 20, 0);
                            break 'b_2ca6ac;
                        }
                    }
                    'b_2ca6d4: {
                        'b_2ca6c4: {
                            c.cycles += 2;
                            let t = c.g[S3][0] == 0u64;
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_2ca6c4;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S2][0] != 0u64;
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_2ca6d4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] != 0u64;

                                    if t {
                                        break 'b_2ca6d4;
                                    } else {
                                        break 'b_2ca6c4;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_2ca6e8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S1][0] == 0u64;
                            c.g[V0][0] = c.g[S4][0];
                            if t {
                                break 'b_2ca6ec;
                            } else {
                                break 'b_2ca6d4;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[AT][0] = 0x41800000;
                    c.mtc1(1, 1);
                    break 'b_2ca6dc;
                }
                c.cycles += 3;
                c.add_s(0, 0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                break 'b_2ca6e8;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
            break 'b_2ca6ec;
        }
        c.cycles += 30;
        c.g[AT][0] = 0x42c80000;
        c.mtc1(1, 2);
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[AT][0] = 0x41400000;
        c.mtc1(1, 4);
        c.max_s(0, 0, 2);
        c.g[AT][0] = 0x42000000;
        c.mtc1(1, 3);
        c.add_s(1, 1, 4);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.add_s(0, 0, 3);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[SP][0] as u32);
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[S4][0] as u32);
        c.sdl(4, (c.g[S4][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[S4][0] as u32).wrapping_add(0x8));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// int `MenuSysEnumItem_HandleInput(MenuSysEnumItem *item)` (0x2cc8f0): run the item input, then scroll the enum value on next/prev codes.
pub fn menu_sys_enum_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ccaf4: {
            'b_2ccaf0: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                callee::MENU_SYS_ITEM_HANDLE_INPUT_3.call(c, 0x002cc918)?;
                c.g[S5][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_ITEM_IS_DISABLED_5.call(c, 0x002cc924)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[S5][0];
                if t {
                    break 'b_2ccaf0;
                } else {
                    c.cycles += 2;
                    if c.g[S5][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_2ccaf4;
                    } else {
                        'b_2ccaec: {
                            'b_2cc970: {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_GET_INPUT_CODE_3.call(c, 0x002cc93c)?;
                                c.g[V1][0] = 0x6;
                                if c.g[V0][0] != c.g[V1][0] {
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                    break 'b_2cc970;
                                } else {
                                    c.cycles += 10;
                                    c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A2][0] = 0u64;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x12c));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x002cc968)?;
                                    c.g[S5][0] = c.g[V0][0];
                                    break 'b_2ccaec;
                                }
                            }
                            c.cycles += 4;
                            c.g[S1][0] = 0u64;
                            c.g[S2][0] = 0x2;
                            c.g[S3][0] = 0x3;
                            c.g[S4][0] = 0x1;
                            'l_2cc980: loop {
                                'b_2ccacc: {
                                    'b_2ccac8: {
                                        'b_2cca00: {
                                            c.cycles += 2;
                                            if c.g[S1][0] != c.g[A2][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                                break 'b_2cca00;
                                            } else {
                                                'b_2cc9c0: {
                                                    'b_2cc9ac: {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                            .call(c, 0x002cc990)?;
                                                        if c.g[V0][0] == c.g[S2][0] {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_2cc9ac;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                                .call(c, 0x002cc9a0)?;
                                                            let t = c.g[V0][0] != 0u64;

                                                            if t {
                                                                break 'b_2cc9c0;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                break 'b_2cc9ac;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                                    break 'b_2ccacc;
                                                }
                                                'b_2cc9e4: {
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                        .call(c, 0x002cc9c8)?;
                                                    if c.g[V0][0] == c.g[S3][0] {
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        break 'b_2cc9e4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                            .call(c, 0x002cc9d8)?;
                                                        let t = c.g[V0][0] != c.g[S4][0];
                                                        c.g[V0][0] = c.g[S5][0];
                                                        if t {
                                                            break 'b_2ccaf0;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_2cc9e4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                                c.g[A3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_sub(0x4));
                                                break 'b_2ccac8;
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[V1][0] = sx((c.g[S1][0] as u32) << 3);
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                        if c.g[A1][0] != c.g[V0][0] {
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002cc980);
                                            }
                                            continue 'l_2cc980;
                                        } else {
                                            'b_2cca78: {
                                                'b_2cca40: {
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                        .call(c, 0x002cca24)?;
                                                    if c.g[V0][0] == c.g[S2][0] {
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        break 'b_2cca40;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                            .call(c, 0x002cca34)?;
                                                        let t = c.g[V0][0] != 0u64;

                                                        if t {
                                                            break 'b_2cca78;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_2cca40;
                                                        }
                                                    }
                                                }
                                                'b_2cca54: {
                                                    c.cycles += 5;
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A3][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                                    c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    if c.g[A0][0] == 0u64 {
                                                        break 'b_2cca54;
                                                    } else {
                                                        break 'b_2cca54;
                                                    }
                                                }
                                                c.cycles += 9;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                c.g[A2][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                                c.div(0, 3, 4);
                                                c.g[V0][0] = c.hi[0];
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A2][0] as u32));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                                                break 'b_2ccacc;
                                            }
                                            'b_2cca9c: {
                                                c.cycles += 4;
                                                c.g[A0][0] = c.g[S0][0];
                                                callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                    .call(c, 0x002cca80)?;
                                                if c.g[V0][0] == c.g[S3][0] {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                    break 'b_2cca9c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    callee::MENU_SYS_ITEM_GET_INPUT_CODE_3
                                                        .call(c, 0x002cca90)?;
                                                    let t = c.g[V0][0] != c.g[S4][0];
                                                    c.g[V0][0] = c.g[S5][0];
                                                    if t {
                                                        break 'b_2ccaf0;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        break 'b_2cca9c;
                                                    }
                                                }
                                            }
                                            'b_2ccab4: {
                                                c.cycles += 6;
                                                c.g[A0][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                                c.g[A3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                                c.g[A1][0] = c.g[S0][0];
                                                c.div(0, 4, 2);
                                                if c.g[V0][0] == 0u64 {
                                                    break 'b_2ccab4;
                                                } else {
                                                    break 'b_2ccab4;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                            c.g[V1][0] = c.hi[0];
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                            break 'b_2ccac8;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                                    break 'b_2ccacc;
                                }
                                c.cycles += 8;
                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002ccae4)?;
                                c.g[V0][0] = c.g[S5][0];
                                break 'b_2ccaf0;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S5][0];
                        break 'b_2ccaf0;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_2ccaf4;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// void `MenuSysItem_DrawRowText(MenuSysItem *item, ...)` (0x2ca768): draw the row text and its optional value strings.
pub fn menu_sys_item_draw_row_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ca858: {
            c.cycles += 45;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[T2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S4][0] = c.g[A2][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[A2][0] = c.g[T1][0];
            c.g[S3][0] = c.g[A3][0];
            c.g[S5][0] = c.g[T0][0];
            c.g[T3][0] = c.lw(c.g[S2][0] as u32);
            c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x12c));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x002ca7bc)?;
            c.g[A0][0] = c.lw(c.g[S2][0] as u32);
            c.g[AT][0] = 0x40000000;
            c.mtc1(1, 0);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[S0][0] as u32);
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0xf));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.swc1(0, c.g[SP][0] as u32);
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[SP][0] as u32);
            c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
            c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
            let t = c.g[S4][0] == 0u64;
            c.g[A1][0] = c.g[S4][0];
            if t {
                break 'b_2ca858;
            } else {
                c.cycles += 15;
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 0);
                c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.add_s(13, 13, 0);
                c.lwc1(1, c.g[S1][0] as u32);
                c.g[AT][0] = 0x41800000;
                c.mtc1(1, 12);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.add_s(12, 1, 12);
                c.sub_s(13, 13, 0);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                call_ptr(c, 0x00391cb0, 0x002ca858)?;
                break 'b_2ca858;
            }
        }
        'b_2ca8cc: {
            c.cycles += 2;
            let t = c.g[S3][0] == 0u64;
            c.g[A1][0] = c.g[S3][0];
            if t {
                break 'b_2ca8cc;
            } else {
                c.cycles += 27;
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                call_ptr(c, 0x00391fb0, 0x002ca880)?;
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[S3][0];
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 3);
                c.sub_s(12, 12, 0);
                c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 1);
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.mul_s(12, 12, 3);
                c.lwc1(2, c.g[S1][0] as u32);
                c.add_s(13, 13, 1);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.add_s(12, 2, 12);
                c.sub_s(13, 13, 0);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                call_ptr(c, 0x00391cb0, 0x002ca8cc)?;
                break 'b_2ca8cc;
            }
        }
        'b_2ca940: {
            c.cycles += 2;
            let t = c.g[S5][0] == 0u64;
            c.g[A1][0] = c.g[S5][0];
            if t {
                break 'b_2ca940;
            } else {
                c.cycles += 27;
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                call_ptr(c, 0x00391fb0, 0x002ca8f4)?;
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[S5][0];
                c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.sub_s(1, 1, 0);
                c.lwc1(12, c.g[S1][0] as u32);
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 2);
                c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                c.add_s(12, 12, 1);
                c.g[AT][0] = 0x41800000;
                c.mtc1(1, 3);
                c.add_s(13, 13, 2);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                c.sub_s(12, 12, 3);
                c.sub_s(13, 13, 0);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                call_ptr(c, 0x00391cb0, 0x002ca940)?;
                break 'b_2ca940;
            }
        }
        c.cycles += 18;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// int `MenuSysInt8Item_HandleInput(MenuSysInt8Item *item)` (0x2cd5a0): adjust the byte value with the left/right axis.
pub fn menu_sys_int8_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2cd7a0: {
            'b_2cd79c: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                callee::MENU_SYS_ITEM_HANDLE_INPUT_3.call(c, 0x002cd5c0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_ITEM_IS_DISABLED_5.call(c, 0x002cd5cc)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[S1][0];
                if t {
                    break 'b_2cd79c;
                } else {
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_2cd7a0;
                    } else {
                        'b_2cd630: {
                            'b_2cd618: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_GET_RIGHT_AXIS.call(c, 0x002cd5e4)?;
                                c.mov_s(20, 0);
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_GET_LEFT_AXIS.call(c, 0x002cd5f0)?;
                                c.sub_s(20, 20, 0);
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ff0));
                                c.c_le_s(20, 0);
                                let t = c.fcc();

                                if t {
                                    break 'b_2cd618;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fec));
                                    c.c_le_s(0, 20);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0u64;
                                    if t {
                                        break 'b_2cd630;
                                    } else {
                                        break 'b_2cd618;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.mtc1(0, 0);
                            c.c_lt_s(20, 0);
                            let t = !c.fcc();
                            c.g[V1][0] = 0x1;
                            if t {
                                break 'b_2cd630;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0xffffffffffffffff;
                                break 'b_2cd630;
                            }
                        }
                        'b_2cd65c: {
                            'b_2cd64c: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
                                    break 'b_2cd64c;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                    break 'b_2cd65c;
                                }
                            }
                            c.cycles += 4;
                            c.mtc1(3, 0);
                            c.cvt_s_w(0, 0);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
                            break 'b_2cd65c;
                        }
                        'b_2cd798: {
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V1][0] = 0x120;
                            if t {
                                break 'b_2cd798;
                            } else {
                                'b_2cd674: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fe8));
                                    if c.g[V1][0] == 0u64 {
                                        break 'b_2cd674;
                                    } else {
                                        break 'b_2cd674;
                                    }
                                }
                                'b_2cd700: {
                                    'b_2cd6c8: {
                                        c.cycles += 13;
                                        c.mult(0, 2, 2, 2);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 21);
                                        c.mul_s(20, 20, 0);
                                        c.g[A0][0] = c.g[S0][0];
                                        c.div(0, 2, 3);
                                        c.g[V0][0] = c.lo[0];
                                        c.mtc1(2, 0);
                                        c.cvt_s_w(0, 0);
                                        c.mul_s(20, 20, 0);
                                        callee::MENU_SYS_ITEM_IS_SLOW_MOD_HELD
                                            .call(c, 0x002cd6a0)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cd6c8;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.g[S0][0];
                                            callee::MENU_SYS_ITEM_IS_FINE_MOD_HELD
                                                .call(c, 0x002cd6b0)?;
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_2cd6c8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[AT][0] = 0x42480000;
                                                c.mtc1(1, 21);

                                                break 'b_2cd700;
                                            }
                                        }
                                    }
                                    'b_2cd6e8: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S0][0];
                                        callee::MENU_SYS_ITEM_IS_SLOW_MOD_HELD
                                            .call(c, 0x002cd6d0)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cd6e8;
                                        } else {
                                            c.cycles += 4;
                                            c.g[AT][0] = 0x41a00000;
                                            c.mtc1(1, 21);

                                            break 'b_2cd700;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::MENU_SYS_ITEM_IS_FINE_MOD_HELD.call(c, 0x002cd6f0)?;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_2cd700;
                                    } else {
                                        c.cycles += 2;
                                        c.g[AT][0] = 0x40a00000;
                                        c.mtc1(1, 21);
                                        break 'b_2cd700;
                                    }
                                }
                                'b_2cd744: {
                                    c.cycles += 14;
                                    c.div_s(20, 20, 21);
                                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
                                    c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                    c.add_s(20, 0, 20);
                                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x28));
                                    c.g[T0][0] = c.lb(c.g[T1][0] as u32);
                                    c.mtc1(8, 0);
                                    c.cvt_s_w(0, 0);
                                    c.add_s(0, 0, 20);
                                    c.cvt_w_s(1, 0);
                                    c.cvt_s_w(1, 1);
                                    c.c_lt_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2cd744;
                                    } else {
                                        c.cycles += 3;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 0);
                                        c.sub_s(1, 1, 0);
                                        break 'b_2cd744;
                                    }
                                }
                                c.cycles += 21;
                                c.cvt_w_s(0, 1);
                                c.mfc1(2, 0);
                                c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x1c));
                                c.g[A3][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x1d));
                                c.g[A1][0] = c.g[S0][0];
                                c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                if c.g[V1][0] != 0 {
                                    c.g[V0][0] = c.g[A2][0];
                                }
                                c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[A0][0] != 0 {
                                    c.g[V0][0] = c.g[A3][0];
                                }
                                c.g[V1][0] =
                                    sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                c.mtc1(3, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 20, 0);
                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
                                c.sb(c.g[T1][0] as u32, c.g[V0][0]);
                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002cd798)?;
                                break 'b_2cd798;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S1][0];
                        break 'b_2cd79c;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_2cd7a0;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// int `MenuSysIntItem_HandleInput(MenuSysIntItem *item)` (0x2cd9d8): adjust the word value with the left/right axis.
pub fn menu_sys_int_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2cdbd8: {
            'b_2cdbd4: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                callee::MENU_SYS_ITEM_HANDLE_INPUT_3.call(c, 0x002cd9f8)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_ITEM_IS_DISABLED_5.call(c, 0x002cda04)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[S1][0];
                if t {
                    break 'b_2cdbd4;
                } else {
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_2cdbd8;
                    } else {
                        'b_2cda68: {
                            'b_2cda50: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_GET_RIGHT_AXIS.call(c, 0x002cda1c)?;
                                c.mov_s(20, 0);
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_GET_LEFT_AXIS.call(c, 0x002cda28)?;
                                c.sub_s(20, 20, 0);
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fe4));
                                c.c_le_s(20, 0);
                                let t = c.fcc();

                                if t {
                                    break 'b_2cda50;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fe0));
                                    c.c_le_s(0, 20);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0u64;
                                    if t {
                                        break 'b_2cda68;
                                    } else {
                                        break 'b_2cda50;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.mtc1(0, 0);
                            c.c_lt_s(20, 0);
                            let t = !c.fcc();
                            c.g[V1][0] = 0x1;
                            if t {
                                break 'b_2cda68;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0xffffffffffffffff;
                                break 'b_2cda68;
                            }
                        }
                        'b_2cda94: {
                            'b_2cda84: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
                                    break 'b_2cda84;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                    break 'b_2cda94;
                                }
                            }
                            c.cycles += 4;
                            c.mtc1(3, 0);
                            c.cvt_s_w(0, 0);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
                            break 'b_2cda94;
                        }
                        'b_2cdbd0: {
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0u64;
                            c.g[V1][0] = 0x120;
                            if t {
                                break 'b_2cdbd0;
                            } else {
                                'b_2cdaac: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fdc));
                                    if c.g[V1][0] == 0u64 {
                                        break 'b_2cdaac;
                                    } else {
                                        break 'b_2cdaac;
                                    }
                                }
                                'b_2cdb38: {
                                    'b_2cdb00: {
                                        c.cycles += 13;
                                        c.mult(0, 2, 2, 2);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 21);
                                        c.mul_s(20, 20, 0);
                                        c.g[A0][0] = c.g[S0][0];
                                        c.div(0, 2, 3);
                                        c.g[V0][0] = c.lo[0];
                                        c.mtc1(2, 0);
                                        c.cvt_s_w(0, 0);
                                        c.mul_s(20, 20, 0);
                                        callee::MENU_SYS_ITEM_IS_SLOW_MOD_HELD
                                            .call(c, 0x002cdad8)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cdb00;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.g[S0][0];
                                            callee::MENU_SYS_ITEM_IS_FINE_MOD_HELD
                                                .call(c, 0x002cdae8)?;
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_2cdb00;
                                            } else {
                                                c.cycles += 4;
                                                c.g[AT][0] = 0x42480000;
                                                c.mtc1(1, 21);

                                                break 'b_2cdb38;
                                            }
                                        }
                                    }
                                    'b_2cdb20: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S0][0];
                                        callee::MENU_SYS_ITEM_IS_SLOW_MOD_HELD
                                            .call(c, 0x002cdb08)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cdb20;
                                        } else {
                                            c.cycles += 4;
                                            c.g[AT][0] = 0x41a00000;
                                            c.mtc1(1, 21);

                                            break 'b_2cdb38;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::MENU_SYS_ITEM_IS_FINE_MOD_HELD.call(c, 0x002cdb28)?;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_2cdb38;
                                    } else {
                                        c.cycles += 2;
                                        c.g[AT][0] = 0x40a00000;
                                        c.mtc1(1, 21);
                                        break 'b_2cdb38;
                                    }
                                }
                                'b_2cdb7c: {
                                    c.cycles += 14;
                                    c.div_s(20, 20, 21);
                                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
                                    c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                    c.add_s(20, 0, 20);
                                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x2c));
                                    c.g[T0][0] = c.lw(c.g[T1][0] as u32);
                                    c.mtc1(8, 0);
                                    c.cvt_s_w(0, 0);
                                    c.add_s(0, 0, 20);
                                    c.cvt_w_s(1, 0);
                                    c.cvt_s_w(1, 1);
                                    c.c_lt_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2cdb7c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 0);
                                        c.sub_s(1, 1, 0);
                                        break 'b_2cdb7c;
                                    }
                                }
                                c.cycles += 21;
                                c.cvt_w_s(0, 1);
                                c.mfc1(2, 0);
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                c.g[A1][0] = c.g[S0][0];
                                c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                if c.g[V1][0] != 0 {
                                    c.g[V0][0] = c.g[A2][0];
                                }
                                c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[A0][0] != 0 {
                                    c.g[V0][0] = c.g[A3][0];
                                }
                                c.g[V1][0] =
                                    sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                c.mtc1(3, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 20, 0);
                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
                                c.sw(c.g[T1][0] as u32, c.g[V0][0]);
                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002cdbd0)?;
                                break 'b_2cdbd0;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S1][0];
                        break 'b_2cdbd4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_2cdbd8;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// void `MenuSysPage_DrawScrollArrowDown(MenuSysPage *page, ...)` (0x2cb8c8): draw the down scroll arrow.
pub fn menu_sys_page_draw_scroll_arrow_down(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
        c.g[A2][0] = 0x4d0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5380));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x12c));
        c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, t, 0x002cb8f4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0xffffffffffffff83;
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[A2][0] = 0x5;
        c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        'l_2cb928: loop {
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            let t = c.g[A2][0] != c.g[A3][0];
            c.g[V0][0] = 0x4d0000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002cb928);
                }
                continue 'l_2cb928;
            } else {
                c.cycles += 21;
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x53a0));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = 0x80;
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(1, 1, 0);
                c.mul_s(3, 3, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 4);
                c.mul_s(2, 2, 0);
                c.g[V1][0] = c.g[SP][0];
                c.cvt_w_s(0, 1);
                c.mfc1(7, 0);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                c.cvt_w_s(0, 3);
                c.mfc1(5, 0);
                c.cvt_w_s(0, 2);
                c.mfc1(4, 0);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                'l_2cb998: loop {
                    c.cycles += 10;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x002cb998);
                        }
                        continue 'l_2cb998;
                    } else {
                        c.cycles += 65;
                        c.lwc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = 0x6;
                        c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A2][0] = c.g[SP][0];
                        c.g[AT][0] = 0x40800000;
                        c.mtc1(1, 1);
                        c.g[A3][0] = 0u64;
                        c.add_s(3, 6, 3);
                        c.lwc1(4, c.g[S0][0] as u32);
                        c.g[AT][0] = 0x40c00000;
                        c.mtc1(1, 9);
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                        c.sub_s(3, 3, 1);
                        c.g[AT][0] = 0x40400000;
                        c.mtc1(1, 2);
                        c.add_s(8, 4, 9);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 1);
                        c.add_s(7, 4, 0);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 5);
                        c.mul_s(0, 0, 1);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.sub_s(1, 3, 2);
                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.add_s(6, 6, 2);
                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x12c));
                        c.add_s(4, 4, 0);
                        c.sub_s(7, 7, 9);
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.add_s(2, 6, 2);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002cbab4)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// void `MenuSysPage_DrawScrollArrowUp(MenuSysPage *page, ...)` (0x2cbac8): draw the up scroll arrow.
pub fn menu_sys_page_draw_scroll_arrow_up(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
        c.g[A2][0] = 0x4d0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5380));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x12c));
        c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, t, 0x002cbaf4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0xffffffffffffff83;
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[A2][0] = 0x5;
        c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        'l_2cbb28: loop {
            c.cycles += 7;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            let t = c.g[A2][0] != c.g[A3][0];
            c.g[V0][0] = 0x4d0000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002cbb28);
                }
                continue 'l_2cbb28;
            } else {
                c.cycles += 21;
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x53a0));
                c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = 0x80;
                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(1, 1, 0);
                c.mul_s(3, 3, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 4);
                c.mul_s(2, 2, 0);
                c.g[V1][0] = c.g[SP][0];
                c.cvt_w_s(0, 1);
                c.mfc1(7, 0);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                c.cvt_w_s(0, 3);
                c.mfc1(5, 0);
                c.cvt_w_s(0, 2);
                c.mfc1(4, 0);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                'l_2cbb98: loop {
                    c.cycles += 10;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.sw(c.g[V1][0] as u32, 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x002cbb98);
                        }
                        continue 'l_2cbb98;
                    } else {
                        c.cycles += 65;
                        c.lwc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = 0x6;
                        c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A2][0] = c.g[SP][0];
                        c.lwc1(4, c.g[S0][0] as u32);
                        c.g[A3][0] = 0u64;
                        c.g[AT][0] = 0x40400000;
                        c.mtc1(1, 3);
                        c.add_s(5, 6, 5);
                        c.g[AT][0] = 0x40c00000;
                        c.mtc1(1, 9);
                        c.add_s(6, 6, 3);
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x8));
                        c.add_s(8, 4, 9);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 7);
                        c.g[AT][0] = 0x40800000;
                        c.mtc1(1, 2);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 0);
                        c.add_s(10, 6, 3);
                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sub_s(5, 5, 2);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.mul_s(0, 1, 0);
                        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.add_s(1, 4, 1);
                        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x12c));
                        c.sub_s(3, 5, 3);
                        c.add_s(4, 4, 0);
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sub_s(1, 1, 9);
                        c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                        c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x124));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
                        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002cbcb4)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// int `MenuSysFloatItem_HandleInput(MenuSysFloatItem *item)` (0x2cdd78): adjust the float value with the left/right axis.
pub fn menu_sys_float_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2cdf5c: {
            'b_2cdf58: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                callee::MENU_SYS_ITEM_HANDLE_INPUT_3.call(c, 0x002cdd98)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                callee::MENU_SYS_ITEM_IS_DISABLED_5.call(c, 0x002cdda4)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[S1][0];
                if t {
                    break 'b_2cdf58;
                } else {
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_2cdf5c;
                    } else {
                        'b_2cde08: {
                            'b_2cddf0: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_GET_RIGHT_AXIS.call(c, 0x002cddbc)?;
                                c.mov_s(20, 0);
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_GET_LEFT_AXIS.call(c, 0x002cddc8)?;
                                c.sub_s(20, 20, 0);
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fd8));
                                c.c_le_s(20, 0);
                                let t = c.fcc();

                                if t {
                                    break 'b_2cddf0;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fd4));
                                    c.c_le_s(0, 20);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0u64;
                                    if t {
                                        break 'b_2cde08;
                                    } else {
                                        break 'b_2cddf0;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.mtc1(0, 0);
                            c.c_lt_s(20, 0);
                            let t = !c.fcc();
                            c.g[V1][0] = 0x1;
                            if t {
                                break 'b_2cde08;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0xffffffffffffffff;
                                break 'b_2cde08;
                            }
                        }
                        'b_2cde28: {
                            'b_2cde24: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
                                    break 'b_2cde24;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                    break 'b_2cde28;
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
                            break 'b_2cde28;
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = c.g[S1][0];
                        if t {
                            break 'b_2cdf58;
                        } else {
                            'b_2cdea0: {
                                'b_2cde68: {
                                    c.cycles += 6;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 21);
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::MENU_SYS_ITEM_IS_SLOW_MOD_HELD.call(c, 0x002cde40)?;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_2cde68;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S0][0];
                                        callee::MENU_SYS_ITEM_IS_FINE_MOD_HELD
                                            .call(c, 0x002cde50)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cde68;
                                        } else {
                                            c.cycles += 4;
                                            c.g[AT][0] = 0x42480000;
                                            c.mtc1(1, 21);
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                                            break 'b_2cdea0;
                                        }
                                    }
                                }
                                'b_2cde88: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    callee::MENU_SYS_ITEM_IS_SLOW_MOD_HELD.call(c, 0x002cde70)?;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_2cde88;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0x41a00000;
                                        c.mtc1(1, 21);
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                                        break 'b_2cdea0;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                callee::MENU_SYS_ITEM_IS_FINE_MOD_HELD.call(c, 0x002cde90)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                                if t {
                                    break 'b_2cdea0;
                                } else {
                                    c.cycles += 2;
                                    c.g[AT][0] = 0x40a00000;
                                    c.mtc1(1, 21);
                                    break 'b_2cdea0;
                                }
                            }
                            'b_2cdf1c: {
                                c.cycles += 31;
                                c.g[AT][0] = 0x42480000;
                                c.mtc1(1, 3);
                                c.mult(0, 2, 2, 2);
                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3fd0));
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 4);
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fcc));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.lwc1(7, (c.g[S0][0] as u32).wrapping_add(0x1c));
                                c.lwc1(9, (c.g[S0][0] as u32).wrapping_add(0x24));
                                c.lwc1(2, c.g[V1][0] as u32);
                                c.mul_s(0, 0, 3);
                                c.lwc1(6, (c.g[S0][0] as u32).wrapping_add(0x20));
                                c.mul_s(2, 9, 2);
                                c.lwc1(8, (c.g[S0][0] as u32).wrapping_add(0x28));
                                c.sub_s(3, 6, 7);
                                c.div_s(0, 0, 21);
                                c.add_s(2, 2, 8);
                                c.mul_s(0, 0, 5);
                                c.max_s(0, 0, 4);
                                c.mul_s(1, 0, 1);
                                c.div_s(1, 1, 21);
                                c.mul_s(1, 20, 1);
                                c.mul_s(1, 1, 3);
                                c.add_s(2, 2, 1);
                                c.c_lt_s(2, 7);
                                if c.fcc() {
                                    c.mov_s(2, 7);
                                    break 'b_2cdf1c;
                                } else {
                                    break 'b_2cdf1c;
                                }
                            }
                            'b_2cdf2c: {
                                c.cycles += 4;
                                c.c_lt_s(6, 2);
                                if c.fcc() {
                                    c.mov_s(2, 6);
                                    break 'b_2cdf2c;
                                } else {
                                    break 'b_2cdf2c;
                                }
                            }
                            c.cycles += 11;
                            c.sub_s(0, 2, 8);
                            c.g[A1][0] = c.g[S0][0];
                            c.div_s(0, 0, 9);
                            c.swc1(0, c.g[V1][0] as u32);
                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x002cdf54)?;
                            c.g[V0][0] = c.g[S1][0];
                            break 'b_2cdf58;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_2cdf5c;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void MenuSysBoolItem_dtor(MenuSysBoolItem *item)` (0x2d3a68): destroy the
/// base item.
pub fn menu_sys_bool_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3a78)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysEnumItem_dtor(MenuSysEnumItem *item)` (0x2d3a98): destroy the
/// base item.
pub fn menu_sys_enum_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3aa8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysSubmenuItem_dtor(MenuSysSubmenuItem *item)` (0x2d3ae0):
/// destroy the base item.
pub fn menu_sys_submenu_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3af0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysBackItem_dtor(MenuSysBackItem *item)` (0x2d3b10): destroy the
/// base item.
pub fn menu_sys_back_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3b20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysActionItem_dtor(MenuSysActionItem *item)` (0x2d3b38):
/// destroy the base item.
pub fn menu_sys_action_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3b48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysCallbackItem_dtor(MenuSysCallbackItem *item)` (0x2d3b80):
/// destroy the base item.
pub fn menu_sys_callback_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3b90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysLabelItem_dtor(MenuSysLabelItem *item)` (0x2d3bb0): destroy
/// the base item.
pub fn menu_sys_label_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3bc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysSpacerItem_dtor(MenuSysSpacerItem *item)` (0x2d3be0):
/// destroy the base item.
pub fn menu_sys_spacer_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3bf0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysInt8Item_dtor(MenuSysInt8Item *item)` (0x2d3c00): destroy the
/// base item.
pub fn menu_sys_int8_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3c10)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysIntItem_dtor(MenuSysIntItem *item)` (0x2d3c30): destroy the
/// base item.
pub fn menu_sys_int_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3c40)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysFloatItem_dtor(MenuSysFloatItem *item)` (0x2d3c60): destroy
/// the base item.
pub fn menu_sys_float_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3c70)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysFloatRangeItem_dtor(MenuSysFloatRangeItem *item)` (0x2d3ce0):
/// destroy the base item.
pub fn menu_sys_float_range_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3cf0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysTextEditItem_dtor(MenuSysTextEditItem *item)` (0x2d3d20):
/// destroy the base item.
pub fn menu_sys_text_edit_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3d30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysColorChannelItem_dtor(MenuSysColorChannelItem *item)`
/// (0x2d3d70): destroy the base item.
pub fn menu_sys_color_channel_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3d80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_PageDtor(MenuSysGroupItem *item)` (0x2d3d90):
/// destroy the item's page.
pub fn menu_sys_group_item_page_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_PAGE_DTOR.call(c, 0x002d_3da0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysColorSwatchItem_dtor(MenuSysColorSwatchItem *item)`
/// (0x2d3df8): destroy the base item.
pub fn menu_sys_color_swatch_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3e08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysVectorLabelItem_dtor(MenuSysVectorLabelItem *item)`
/// (0x2d3ea8): destroy the base item.
pub fn menu_sys_vector_label_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3eb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPoint2DItem_dtor(MenuSysPoint2DItem *item)` (0x2d3f50):
/// destroy the base item.
pub fn menu_sys_point2_d_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3f60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_dtor(MenuSysGroupItem *item, int flag)` (0x2d3db0):
/// destroy the page at `item+0x18`, then the base item with the flag.
pub fn menu_sys_group_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = 0x2;
        callee::MENU_SYS_PAGE_DTOR.call(c, 0x002d_3dd4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::MENU_SYS_ITEM_DTOR.call(c, 0x002d_3de0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSys_ctor(MenuSys *sys)` (0x2cc018): zero the object, then reset
/// it.
pub fn menu_sys_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0);
        callee::MENU_SYS_RESET.call(c, 0x002c_c030)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysEnumItem_ctor_2(MenuSysEnumItem *item)` (0x2cc7f8): the base
/// item plus the enum vtable, all fields zero.
pub fn menu_sys_enum_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_c80c)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e30));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysCallbackItem_ctor(MenuSysCallbackItem *item)` (0x2cd128): the
/// base item plus the callback vtable, all fields zero.
pub fn menu_sys_callback_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d140)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6cf0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysEnumItem_ctor(MenuSysEnumItem *item)` (0x2cc7b0): the base
/// item plus the enum vtable, all fields zero.
pub fn menu_sys_enum_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_c7c8)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e30));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysActionItem_ctor_2(MenuSysActionItem *item)` (0x2ccf50): the
/// base item plus the action vtable, all fields zero.
pub fn menu_sys_action_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_cf64)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6d40));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysLabelItem_ctor(MenuSysLabelItem *item, int id)` (0x2cd2a0):
/// the base item with `id` at `+0x14`, plus the label vtable.
pub fn menu_sys_label_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffffffffffffffff;
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d2c0)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca0));
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysSpacerItem_ctor(MenuSysSpacerItem *item, int id)` (0x2cd350):
/// the base item with `id` at `+0x14`, plus the spacer vtable.
pub fn menu_sys_spacer_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffffffffffffffff;
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d370)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c50));
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysInt8Item_ctor_2(MenuSysInt8Item *item)` (0x2cd450): the base
/// item plus the int8 vtable, min/max zero.
pub fn menu_sys_int8_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d464)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c00));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x1d), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysIntItem_ctor_2(MenuSysIntItem *item)` (0x2cd8a8): the base
/// item plus the int vtable, all fields zero.
pub fn menu_sys_int_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d8bc)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6bb0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysCallbackItem_ctor_2(MenuSysCallbackItem *item, void *cb,
/// void *arg)` (0x2cd168): the base item with the callback at `+0x14` and
/// its argument at `+0x18`, plus the callback vtable.
pub fn menu_sys_callback_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d18c)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6cf0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysInt8Item_ctor(MenuSysInt8Item *item)` (0x2cd3f8): the base
/// item plus the int8 vtable, min/max zero.
pub fn menu_sys_int8_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d410)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c00));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x1d), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysIntItem_ctor(MenuSysIntItem *item)` (0x2cd850): the base item
/// plus the int vtable, all fields zero.
pub fn menu_sys_int_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d868)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6bb0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysVectorLabelItem_ctor(MenuSysVectorLabelItem *item, int id,
/// void *vec)` (0x2d03f0): the base item with `id` at `+0x14` and the vector
/// at `+0x18`, plus the vector-label vtable.
pub fn menu_sys_vector_label_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffffffffffffffff;
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002d_0418)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x68b8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysFloatItem_ctor(MenuSysFloatItem *item)` (0x2cdc88): the base
/// item plus the float vtable, step `1.0` at `+0x24`.
pub fn menu_sys_float_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_dca0)?;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V1][0] = 0x480000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6b60));
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x24));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_ctor(MenuSysGroupItem *group)` (0x2cf7e8): the base
/// item, the page at `group+0x18`, the vtable, and the item slots (69
/// cleared words) at `+0x144`.
pub fn menu_sys_group_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_f800)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x69b8));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        callee::MENU_SYS_PAGE_CTOR.call(c, 0x002c_f818)?;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a18));
        c.g[V1][0] = 0x45;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x144), c.g[V0][0]);
        'init_items: loop {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = c.g[S0][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002c_f830);
                }
                continue 'init_items;
            } else {
                c.cycles += 5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x148), c.g[S0][0]);
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void MenuSysGroupItem_ctor_2(MenuSysGroupItem *group, void *head)`
/// (0x2cf860): the base item with `head` at `+0x14`, the page at `+0x18`,
/// the vtable, and the first item appended unless `head` is null.
pub fn menu_sys_group_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_f87c)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x69b8));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        callee::MENU_SYS_PAGE_CTOR.call(c, 0x002c_f894)?;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a18));
        c.g[V1][0] = 0x45;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x144), c.g[V0][0]);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        let t = c.g[A1][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x148), c.g[S1][0]);
        // 'b_2cf8c0: append the head unless it is null.
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0xffffffffffffffff;
            callee::MENU_SYS_GROUP_ITEM_ADD_ITEM.call(c, 0x002c_f8c0)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysFloatRangeItem_ctor(MenuSysFloatRangeItem *item, float lo,
/// float hi)` (0x2ce170): the base item with `lo` at `+0x18`, `hi` at
/// `+0x20`, range zeroed, plus the float-range vtable.
pub fn menu_sys_float_range_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.g[S2][0] = c.g[A2][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.mov_s(21, 13);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_e1a4)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6b10));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(21, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysInt8Item_ctor_3(MenuSysInt8Item *item, int lo, int hi)`
/// (0x2cd4a0): the base item with the range bytes at `+0x1c/+0x1d`, plus the
/// int8 vtable.
pub fn menu_sys_int8_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx((c.g[A3][0] as u32).wrapping_shl(24));
        frame.save(c, S3, 0x20);
        c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_shl(24));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        c.g[S2][0] = sx(((c.g[S2][0] as u32) as i32 >> 24) as u32);
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_d4dc)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c00));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sb((c.g[S0][0] as u32).wrapping_add(0x1d), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0);
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

/// `void MenuSysPoint2DItem_ctor(MenuSysPoint2DItem *item, ...)`
/// (0x2d0808): the base item plus the point-2D vtable, the two 8-byte
/// points copied from the arguments, all fields zero.
pub fn menu_sys_point2_d_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 53;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S1][0] = c.g[T1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[T2][0];
        frame.save(c, S7, 0x10);
        c.g[S6][0] = c.g[T3][0];
        c.g[S7][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffffffffffffffff;
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002d_0858)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6808));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
        c.g[A0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6738));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[S7][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.ldl(3, (c.g[S5][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[S5][0] as u32);
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x33));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x2c));
        c.ldl(3, (c.g[S6][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[S6][0] as u32);
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x3b));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x34));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysBoolItem_ctor(MenuSysBoolItem *item, ...)` (0x2cc500): the
/// base item with the label id, on/off ids and value, plus the bool vtable.
pub fn menu_sys_bool_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[T1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_c534)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e80));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
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

/// `void MenuSysEnumItem_ctor_3(MenuSysEnumItem *item, ...)` (0x2cc838): the
/// base item with the ids, table and value, plus the enum vtable.
pub fn menu_sys_enum_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[T1][0];
        callee::MENU_SYS_ITEM_CTOR_5.call(c, 0x002c_c86c)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e30));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S4][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
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

/// `void MenuSysItem_OnFocus(MenuSysItem *item)` (0x2ca368): the base-class
/// focus hook, a no-op.
pub fn menu_sys_item_on_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysItem_OnBlur(MenuSysItem *item)` (0x2ca370): the base-class
/// blur hook, a no-op.
pub fn menu_sys_item_on_blur(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `int MenuSysPage_OnAction(MenuSysPage *page)` (0x2cad48): the base action
/// handler, which handles nothing (returns 0).
pub fn menu_sys_page_on_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysPage_v04(MenuSysPage *page)` (0x2cad50): vtable slot 4, a no-op.
pub fn menu_sys_page_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSys_SetActive(MenuSys *sys, int active)` (0x2cc0b8): store the
/// active flag at `sys+0x54`.
pub fn menu_sys_set_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[A1][0]);
        c.pc = t;
        Ok(())
    })
}

/// `int MenuSys_GetInputCode(MenuSys *sys)` (0x2cc248): load the latched
/// input code at `sys+0x48`.
pub fn menu_sys_get_input_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
        c.pc = t;
        Ok(())
    })
}

/// `int MenuSys_GetKeyCode(MenuSys *sys)` (0x2cc250): load the latched key
/// code at `sys+0x4c`.
pub fn menu_sys_get_key_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
        c.pc = t;
        Ok(())
    })
}

/// `int MenuSys_GetKeyChar(MenuSys *sys)` (0x2cc258): load the latched key
/// character at `sys+0x50`.
pub fn menu_sys_get_key_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `bool MenuSysLabelItem_IsSelectable(MenuSysLabelItem *item)` (0x2cd2e8):
/// labels are never selectable (returns 0).
pub fn menu_sys_label_item_is_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = t;
        Ok(())
    })
}

/// `bool MenuSysSpacerItem_IsSelectable(MenuSysSpacerItem *item)` (0x2cd398):
/// spacers are never selectable (returns 0).
pub fn menu_sys_spacer_item_is_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = t;
        Ok(())
    })
}

/// `bool MenuSysColorSwatchItem_IsSelectable(MenuSysColorSwatchItem *item)`
/// (0x2cff80): swatches are never selectable (returns 0).
pub fn menu_sys_color_swatch_item_is_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = t;
        Ok(())
    })
}

/// `bool MenuSysVectorLabelItem_IsSelectable(MenuSysVectorLabelItem *item)`
/// (0x2d0448): vector labels are never selectable (returns 0).
pub fn menu_sys_vector_label_item_is_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0;
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysItem_ResetValue(MenuSysItem *item)` (0x2d39f8): the base
/// reset hook, a no-op.
pub fn menu_sys_item_reset_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `float MenuSysItem_GetLeftAxis(MenuSysItem *item)` (0x2ca408): read the
/// left axis through the item's menu (`item+0` at `+0x124`).
pub fn menu_sys_item_get_left_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        callee::ITEM_GET_LEFT_AXIS.call(c, 0x002c_a41c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSysItem_GetRightAxis(MenuSysItem *item)` (0x2ca428): read the
/// right axis through the item's menu (`item+0` at `+0x124`).
pub fn menu_sys_item_get_right_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        callee::ITEM_GET_RIGHT_AXIS.call(c, 0x002c_a43c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSysItem_GetUpAxis(MenuSysItem *item)` (0x2ca448): read the up
/// axis through the item's menu (`item+0` at `+0x124`).
pub fn menu_sys_item_get_up_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        callee::ITEM_GET_UP_AXIS.call(c, 0x002c_a45c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSysItem_GetDownAxis(MenuSysItem *item)` (0x2ca468): read the
/// down axis through the item's menu (`item+0` at `+0x124`).
pub fn menu_sys_item_get_down_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        callee::ITEM_GET_DOWN_AXIS.call(c, 0x002c_a47c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSysItem_IsSlowModHeld(MenuSysItem *item)` (0x2ca488): the slow
/// modifier through the item's menu (`item+0` at `+0x124`).
pub fn menu_sys_item_is_slow_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        callee::ITEM_IS_SLOW_MOD_HELD.call(c, 0x002c_a49c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSysItem_IsFineModHeld(MenuSysItem *item)` (0x2ca4a8): the fine
/// modifier through the item's menu (`item+0` at `+0x124`).
pub fn menu_sys_item_is_fine_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        callee::ITEM_IS_FINE_MOD_HELD.call(c, 0x002c_a4bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSys_GetLeftAxis(MenuSys *sys)` (0x2cc260): evaluate the left
/// binding (`sys+0x5c`, axis id `0x3e`).
pub fn menu_sys_get_left_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x3e;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
        callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x002c_c274)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSys_GetRightAxis(MenuSys *sys)` (0x2cc280): evaluate the right
/// binding (`sys+0x5c`, axis id `0x3f`).
pub fn menu_sys_get_right_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x3f;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
        callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x002c_c294)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSys_GetUpAxis(MenuSys *sys)` (0x2cc2a0): evaluate the up
/// binding (`sys+0x5c`, axis id `0x40`).
pub fn menu_sys_get_up_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x40;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
        callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x002c_c2b4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSys_GetDownAxis(MenuSys *sys)` (0x2cc2c0): evaluate the down
/// binding (`sys+0x5c`, axis id `0x41`).
pub fn menu_sys_get_down_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x41;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
        callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x002c_c2d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSys_IsSlowModHeld(MenuSys *sys)` (0x2cc2e0): the slow modifier
/// reads nonzero (binding `sys+0x5c`, modifier id `0x42`).
pub fn menu_sys_is_slow_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x42;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
        callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x002c_c2f4)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let t = !c.fcc();
        c.g[V0][0] = 0x1;
        if t {
            // b_2cc30c: nonzero binding.
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSys_IsFineModHeld(MenuSys *sys)` (0x2cc318): the fine modifier
/// reads nonzero (binding `sys+0x5c`, modifier id `0x43`).
pub fn menu_sys_is_fine_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x43;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
        callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x002c_c32c)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let t = !c.fcc();
        c.g[V0][0] = 0x1;
        if t {
            // b_2cc344: nonzero binding.
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_v09(MenuSysGroupItem *item)` (0x2cfaf8): the
/// group's v09 draw hook: call the item vtable's `+0x18/+0x1c` entry with
/// the item plus its delta.
pub fn menu_sys_group_item_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = c.lw(c.g[A0][0] as u32);
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x12c));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x18).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = crate::abi::addu(c.g[A3][0], c.g[A0][0]);
        call_ptr(c, target, 0x002c_fb18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_OnBlur(MenuSysGroupItem *item)` (0x2cfef8): blur
/// the item, then deactivate its page (`item+0x18`).
pub fn menu_sys_group_item_on_blur(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::ITEM_ON_BLUR.call(c, 0x002c_ff0c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        callee::PAGE_ON_DEACTIVATE.call(c, 0x002c_ff14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysSpacerItem_Draw(MenuSysSpacerItem *item)` (0x2cd3f0):
/// spacers draw nothing.
pub fn menu_sys_spacer_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysGroupItem_AddItem(MenuSysGroupItem *group, ...)` (0x2cf908):
/// insert the item into the page at `group+0x18`.
pub fn menu_sys_group_item_add_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        callee::MENU_SYS_PAGE_INSERT_ITEM.call(c, 0x002c_f918)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_RemoveItem(MenuSysGroupItem *group, ...)`
/// (0x2cf928): remove the item from the page at `group+0x18`.
pub fn menu_sys_group_item_remove_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        callee::MENU_SYS_PAGE_REMOVE_ITEM.call(c, 0x002c_f938)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysSubmenuItem_Draw(MenuSysSubmenuItem *item, ...)` (0x2ccdc8):
/// draw the item's row with no prefix.
pub fn menu_sys_submenu_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        callee::MENU_SYS_ITEM_DRAW_ROW_7.call(c, 0x002c_cde0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysBackItem_Draw(MenuSysBackItem *item, ...)` (0x2ccee0): draw
/// the item's row with no prefix.
pub fn menu_sys_back_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        callee::MENU_SYS_ITEM_DRAW_ROW_7.call(c, 0x002c_cef8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysLabelItem_Draw(MenuSysLabelItem *item, ...)` (0x2cd328):
/// draw the item's row with no prefix.
pub fn menu_sys_label_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        callee::MENU_SYS_ITEM_DRAW_ROW_7.call(c, 0x002c_d340)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysPage_SortCompare(MenuSysItem **a, MenuSysItem **b)`
/// (0x2d4008): order two items with the comparator at `gp+0x23cc`.
pub fn menu_sys_page_sort_compare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x23cc));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(c.g[A0][0] as u32);
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = c.lw(c.g[A1][0] as u32);
        call_ptr(c, t, 0x002d_4020)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysCallbackItem_Draw(MenuSysCallbackItem *item, ...)`
/// (0x2cd278): draw the item's row with the callback vtable.
pub fn menu_sys_callback_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x66c8));
        c.g[A3][0] = 0;
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        callee::MENU_SYS_ITEM_DRAW_ROW_7.call(c, 0x002c_d294)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysTextEditItem_SetText(MenuSysTextEditItem *item, char *text)`
/// (0x2d3d48): copy at most 15 characters into the buffer at `item+0x18`.
pub fn menu_sys_text_edit_item_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = 0xf;
        c.sb((c.g[V0][0] as u32).wrapping_add(0x18), 0);
        call_ptr(c, 0x0041_6958, 0x002d_3d64)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_PageForwardV03(MenuSysGroupItem *group, ...)`
/// (0x2cf788): forward the page's v03 hook to the item at `group+0x130`.
pub fn menu_sys_group_item_page_forward_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_f7a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_PageForwardV04(MenuSysGroupItem *group, ...)`
/// (0x2cf7b8): forward the page's v04 hook to the item at `group+0x130`.
pub fn menu_sys_group_item_page_forward_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_f7d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_Reset(MenuSysGroupItem *group)` (0x2cf948):
/// remove every page item, then re-add the header unless it is null.
pub fn menu_sys_group_item_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        'reset: {
            frame.save(c, S0, 0x10);
            c.g[S0][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            callee::MENU_SYS_PAGE_REMOVE_ALL_ITEMS.call(c, 0x002c_f960)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            let t = c.g[A1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'reset;
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0xffffffffffffffff;
                callee::MENU_SYS_GROUP_ITEM_ADD_ITEM.call(c, 0x002c_f974)?;
                break 'reset;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSys_DebugPrintf_4080(char *fmt, ...)` (0x2d4080): a debug
/// print call site that marshals the varargs without printing.
pub fn menu_sys_debug_printf_4080(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
        c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
        c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysPage_SortItems(MenuSysPage *page, ...)` (0x2cbe20): sort
/// the page's items with `MenuSysPage_SortCompare`, then revalidate.
pub fn menu_sys_page_sort_items(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x23cc), c.g[A1][0]);
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0x2d0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4008));
        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x4;
        call_ptr(c, 0x0041_8ef8, 0x002c_be4c)?;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[GP][0] as u32).wrapping_add(0x23cc), 0);
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION_3.call(c, 0x002c_be58)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_SetHeaderItem(MenuSysGroupItem *group, MenuSysItem *head)`
/// (0x2cf988): replace the header item: drop a matching entry from the page,
/// then insert `head`. Converted from the generated body (resumable: resume
/// heads re-enter through the generated alias).
pub fn menu_sys_group_item_set_header_item(c: &mut Cpu) {
    if c.pc != 0x002cf988 {
        return ee::s09::f_002cf988__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002cf988;
        loop {
            match pc {
                0x002cf988 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.g[S1][0] = c.g[A1][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A1][0] = 0u64;
                    let t = true;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                    if t {
                        pc = 0x002cf9b4;
                        continue;
                    }
                    pc = 0x002cf9ac;
                    continue;
                }
                0x002cf9ac => {
                    c.cycles += 1;
                    pc = 0x002cf9b0;
                    continue;
                }
                0x002cf9b0 => {
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    pc = 0x002cf9b4;
                    continue;
                }
                0x002cf9b4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                        pc = 0x002cf9e0;
                        continue;
                    }
                    pc = 0x002cf9c4;
                    continue;
                }
                0x002cf9c4 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x002cf9b0) {
                            return Ok(());
                        }
                        pc = 0x002cf9b0;
                        continue;
                    }
                    pc = 0x002cf9d4;
                    continue;
                }
                0x002cf9d4 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x002cf9dc);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    callee::MENU_SYS_PAGE_REMOVE_ITEM.call(c, 0x002cf9dc)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                    pc = 0x002cf9e0;
                    continue;
                }
                0x002cf9e0 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002cf9f0);
                    c.g[A2][0] = 0u64;
                    callee::MENU_SYS_GROUP_ITEM_ADD_ITEM.call(c, 0x002cf9f0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002cf988, 0x002cfa04);
                    return Ok(());
                }
            }
        }
    });
}

/// `void MenuSysPage_RemoveItem(MenuSysPage *page, int index)` (0x2cbd50):
/// close the gap left by the removed item, fix up the count and selection.
pub fn menu_sys_page_remove_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[A1][0];
        // 'b_2cbda0: shift the tail down when the index is in range.
        if !t {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            'shift_down: loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_bd80);
                    }
                    continue 'shift_down;
                } else {
                    break;
                }
            }
        }
        // 'b_2cbdc0: drop the count when the index was in range.
        c.cycles += 6;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let t = c.g[V1][0] == 0;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        if !t {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        }
        // 'b_2cbde0: clamp the selection to the new count.
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        }
        // 'b_2cbdf4: revalidate when the selection moved.
        c.cycles += 2;
        if c.g[A1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_PAGE_VALIDATE_SELECTION_3.call(c, 0x002c_bdf0)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        }
        // 'b_2cbdfc: clear a negative selection.
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysItem_DrawRow(MenuSysItem *item, ...)` (0x2ca988): draw the
/// row in the selected or unselected colour, then its text.
pub fn menu_sys_item_draw_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S4][0] = c.g[T0][0];
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        callee::MENU_SYS_PAGE_GET_SELECTED_ITEM_2.call(c, 0x002c_a9c0)?;
        c.g[V0][0] = c.g[S0][0] ^ c.g[V0][0];
        c.g[V1][0] = (c.g[V0][0] < 0x1) as u64;
        // 'b_2ca9d8/'b_2ca9e0: the selected colour or the plain one.
        'row_color: {
            'row_plain: {
                let t = c.g[V1][0] == 0;
                c.g[V0][0] = 0x4d0000;
                if t {
                    break 'row_plain;
                } else {
                    c.cycles += 2;
                    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
                    break 'row_color;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x4d0000;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5390));
            break 'row_color;
        }
        // 'b_2ca9f0/'b_2ca9f8: the row colour table.
        c.cycles += 2;
        let t = c.g[V1][0] == 0;
        c.g[V0][0] = 0x4d0000;
        if t {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
        }
        c.cycles += 24;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.swc1(0, c.g[SP][0] as u32);
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[T2][0] = c.g[SP][0];
        c.lwc1(1, c.g[V0][0] as u32);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        callee::MENU_SYS_ITEM_DRAW_ROW_TEXT.call(c, 0x002c_aa38)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSysTextEditItem_Measure(MenuSysTextEditItem *item, ...)`
/// (0x2ce820): measure the selected or edited text into the row.
pub fn menu_sys_text_edit_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        callee::MENU_SYS_PAGE_GET_SELECTED_ITEM_2.call(c, 0x002c_e84c)?;
        c.g[S0][0] = c.g[S2][0] ^ c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[S0][0] = (c.g[S0][0] < 0x1) as u64;
        call_ptr(c, 0x0041_66f4, 0x002c_e860)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x002c_e868)?;
        // 'b_2ce894/'b_2ce898: measure the edited text when selected, the
        // stored one otherwise; clamp a wide result to the maximum.
        'measure_outer: {
            'measure_inner: {
                let t = c.g[S0][0] == 0;
                c.g[V1][0] = c.g[V0][0];
                if t {
                    break 'measure_inner;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        break 'measure_outer;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
                        let t = c.g[V0][0] == 0;
                        c.g[A1][0] = 0x4a0000;
                        if t {
                            break 'measure_inner;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x39d0));
                            call_ptr(c, 0x0041_62d0, 0x002c_e894)?;
                            break 'measure_inner;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[S2][0];
            break 'measure_outer;
        }
        c.cycles += 29;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        c.g[A3][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::MENU_SYS_ITEM_MEASURE_ROW_2.call(c, 0x002c_e8ac)?;
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x3f));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x80);
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.sdl(6, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(6, c.g[S3][0] as u32);
        c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0xf));
        c.sdr(3, (c.g[S3][0] as u32).wrapping_add(0x8));
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        Ok(())
    })
}

/// `int MenuSysGroupItem_HandleInput(MenuSysGroupItem *group)` (0x2cfa08):
/// validate the selection, run the selected item's handler, and move the
/// selection on next/prev codes.
pub fn menu_sys_group_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[S3][0];
        callee::MENU_SYS_PAGE_VALIDATE_SELECTION_3.call(c, 0x002c_fa30)?;
        c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_fa54)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::MENU_SYS_ITEM_IS_DISABLED_6.call(c, 0x002c_fa60)?;
        // 'b_2cfad8/'b_2cfadc: the handler's code, unless the item is
        // disabled; next (3) and prev (4) move the selection.
        'handle: {
            'code: {
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S0][0];
                if t {
                    break 'code;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                let t = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x4;
                if t {
                    // Next: advance past the selected item when possible.
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S3][0];
                    callee::MENU_SYS_PAGE_SELECT_NEXT_3.call(c, 0x002c_fa8c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if t {
                        break 'code;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S2][0];
                        callee::MENU_SYS_PAGE_SET_SELECTION_2.call(c, 0x002c_faa8)?;
                        c.g[V0][0] = 0x3;
                        break 'code;
                    }
                } else {
                    c.cycles += 2;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        // Prev: retreat when past the first item.
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S3][0];
                        callee::MENU_SYS_PAGE_SELECT_PREV_2.call(c, 0x002c_fab8)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0;
                        c.g[V0][0] = 0;
                        if t {
                            break 'code;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S2][0];
                            callee::MENU_SYS_PAGE_SET_SELECTION_2.call(c, 0x002c_fad4)?;
                            c.g[V0][0] = 0x4;
                            break 'code;
                        }
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x40);
                        break 'handle;
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
            break 'handle;
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysBoolItem_Measure(MenuSysBoolItem *item, ...)` (0x2cc648):
/// measure the row, keeping the wider of the bool strings.
pub fn menu_sys_bool_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        let t = c.g[T0][0] == 0;
        c.g[S2][0] = c.g[A0][0];
        // 'b_2cc720/'b_2cc738: measure the bool strings when set. The
        // both-strings path copies the wider row and returns; the keep
        // paths fall through to the final row measure below.
        let mut both = false;
        if t {
            // No bool strings: keep the row.
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            let t = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            if t {
                // No off string: keep the row.
            } else {
                // 'b_2cc6fc: measure both strings, keep the wider row.
                c.cycles += 32;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.g[S0][0];
                c.g[A3][0] = 0;
                callee::MENU_SYS_ITEM_MEASURE_ROW_2.call(c, 0x002c_c688)?;
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, c.g[SP][0] as u32);
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
                callee::MENU_SYS_ITEM_MEASURE_ROW_2.call(c, 0x002c_c6c0)?;
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
                c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x2f));
                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.c_lt_s(0, 1);
                let t = !c.fcc();
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                if t {
                    // On string wider or equal: keep the row.
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[SP][0];
                }
                c.cycles += 9;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.ldr(5, c.g[V0][0] as u32);
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[S2][0] as u32).wrapping_add(0x7));
                c.sdr(5, c.g[S2][0] as u32);
                c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0xf));
                c.sdr(6, (c.g[S2][0] as u32).wrapping_add(0x8));
                both = true;
            }
        }
        if !both {
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            callee::MENU_SYS_ITEM_MEASURE_ROW_2.call(c, 0x002c_c738)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysEnumItem_Measure(MenuSysEnumItem *item, ...)` (0x2ccb18):
/// measure every entry, keeping the widest row.
pub fn menu_sys_enum_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x70);
        c.g[T0][0] = 0x480000;
        frame.save(c, S0, 0x60);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x66b8));
        frame.save(c, S1, 0x50);
        c.g[A3][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = c.g[A0][0];
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[S0][0] = 0;
        callee::MENU_SYS_ITEM_MEASURE_ROW_2.call(c, 0x002c_cb58)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x1f));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[SP][0] as u32);
        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        let t = (c.g[V0][0] as i64) <= 0;
        // 'b_2ccbf4: walk the entries while any remain.
        if !t {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            // 'l_2ccb88: measure entry rows, keeping the widest.
            'measure_entries: loop {
                'measure_entry: {
                    c.cycles += 14;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A3][0] = 0;
                    callee::MENU_SYS_ITEM_MEASURE_ROW_2.call(c, 0x002c_cba8)?;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.c_lt_s(0, 1);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'measure_entry;
                    } else {
                        c.cycles += 9;
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(2, c.g[SP][0] as u32);
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'measure_entry;
                    }
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_cb88);
                    }
                    continue 'measure_entries;
                } else {
                    break;
                }
            }
        }
        c.cycles += 16;
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(6, c.g[SP][0] as u32);
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.sdr(6, c.g[S2][0] as u32);
        c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = c.g[S2][0];
        c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0x8));
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `void MenuSys_FormatFloat(char *out, float v)` (0x2cdfd8): format `v`
/// with a magnitude-appropriate format string.
pub fn menu_sys_format_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
        frame.save(c, S1, 0x20);
        c.mov_s(21, 12);
        frame.save(c, S0, 0x30);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0040_da10, 0x002c_dffc)?;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 12);
        c.mov_s(20, 0);
        call_ptr(c, 0x0040_da10, 0x002c_e00c)?;
        c.div_s(20, 20, 0);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc8));
        c.add_s(20, 20, 0);
        c.cvt_w_s(1, 20);
        c.cvt_s_w(1, 1);
        c.c_lt_s(20, 1);
        let t = !c.fcc();
        // 'b_2ce038: split the integer and fraction digits.
        if t {
            // Fraction rounds up: carry into the integer part.
        } else {
            c.cycles += 3;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.sub_s(1, 1, 0);
        }
        // 'b_2ce0a8: pick the format by magnitude.
        'pick_format: {
            'whole: {
                c.cycles += 5;
                c.cvt_w_s(0, 1);
                c.mfc1(3, 0);
                let t = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'whole;
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3998));
                    break 'pick_format;
                }
            }
            'small: {
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) > 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                if t {
                    break 'small;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39a0));
                    break 'pick_format;
                }
            }
            'medium: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'medium;
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39a8));
                    break 'pick_format;
                }
            }
            'large: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'large;
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39b0));
                    break 'pick_format;
                }
            }
            'huge: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 7i64) as u64;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'huge;
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39b8));
                    break 'pick_format;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39c0));
            break 'pick_format;
        }
        c.cycles += 22;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.mov_s(12, 21);
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x0041_3af8, 0x002c_e0b8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0031_94a8, 0x002c_e0c8)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_7fe8, 0x002c_e0d4)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0031_8260, 0x002c_e0e0)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `int MenuSysColorChannelItem_HandleInput(MenuSysColorChannelItem *item)`
/// (0x2ced78): adjust the channel with the left/right input.
pub fn menu_sys_color_channel_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        callee::MENU_SYS_ITEM_HANDLE_INPUT_4.call(c, 0x002c_ed94)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED_6.call(c, 0x002c_eda0)?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        // 'b_2ceed0/'b_2ceed4: the handler's code, unless disabled.
        'handle_item: {
            if t {
                // Disabled: converge below.
            } else {
                c.cycles += 2;
                if c.g[S1][0] != 0 {
                    // A code was produced: keep it.
                    frame.restore(c, S0, 0x20);
                    break 'handle_item;
                } else {
                    // No code: read the left/right axis.
                    'deflect: {
                        'classify: {
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_RIGHT_AXIS_2.call(c, 0x002c_edb8)?;
                            c.mov_s(20, 0);
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_LEFT_AXIS_2.call(c, 0x002c_edc4)?;
                            c.sub_s(20, 20, 0);
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fb0));
                            c.c_le_s(20, 0);
                            let t = c.fcc();
                            if t {
                                // Dead zone: take the sign path.
                                break 'classify;
                            } else {
                                c.cycles += 5;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fac));
                                c.c_le_s(0, 20);
                                let t = !c.fcc();
                                c.g[V1][0] = 0;
                                if t {
                                    // Moderate deflection: no step.
                                    break 'deflect;
                                } else {
                                    // Hard deflection: take the sign path.
                                    break 'classify;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(20, 0);
                        let t = !c.fcc();
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'deflect;
                        } else {
                            c.cycles += 1;
                            c.g[V1][0] = 0xffffffffffffffff;
                            break 'deflect;
                        }
                    }
                    // 'b_2cee24/'b_2cee20: step the repeat counter.
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
                        c.cycles += 1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    }
                    // 'b_2ceecc: apply the step when it fires.
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[A1][0] = 0;
                    if t {
                        // No step: keep the code.
                    } else {
                        // 'b_2cee44: zero-step repeats keep the code.
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        c.g[A0][0] = 0x120;
                        c.g[A3][0] = 0x1;
                        c.mult(0, 3, 3, 3);
                        if c.g[A0][0] == c.g[A1][0] {
                            // Zero repeats: keep the code.
                        }
                        // 'b_2cee94: scale the step and clamp the channel.
                        c.cycles += 20;
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fa8));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.div(0, 3, 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.mtc1(0, 3);
                        c.lwc1(2, c.g[A2][0] as u32);
                        c.g[V1][0] = c.lo[0];
                        c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[A1][0] != 0 {
                            c.g[A3][0] = c.g[V1][0];
                        }
                        c.mtc1(7, 0);
                        c.cvt_s_w(0, 0);
                        c.mul_s(1, 0, 1);
                        c.mul_s(1, 20, 1);
                        c.add_s(2, 2, 1);
                        c.c_lt_s(2, 3);
                        if c.fcc() {
                            c.mov_s(2, 3);
                        }
                        // 'b_2ceeac: clamp at zero from below.
                        c.cycles += 6;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.c_lt_s(0, 2);
                        if c.fcc() {
                            c.mov_s(2, 0);
                        }
                        c.cycles += 8;
                        c.swc1(2, c.g[A2][0] as u32);
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x002c_eecc)?;
                    }
                    // Convergence 1 (axis only).
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S1][0];
                }
                // Convergence 2 (disabled and axis).
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'handle_item;
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        frame.close(c);
        Ok(())
    })
}

/// `int MenuSysFloatRangeItem_HandleInput(MenuSysFloatRangeItem *item)`
/// (0x2ce1f0): adjust the range with the left/right input.
pub fn menu_sys_float_range_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        callee::MENU_SYS_ITEM_HANDLE_INPUT_4.call(c, 0x002c_e20c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MENU_SYS_ITEM_IS_DISABLED_6.call(c, 0x002c_e218)?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        // 'b_2ce34c/'b_2ce350: the handler's code, unless disabled.
        'handle_range: {
            if t {
                // Disabled: converge below.
            } else {
                c.cycles += 2;
                if c.g[S1][0] != 0 {
                    // A code was produced: keep it.
                    frame.restore(c, S0, 0x20);
                    break 'handle_range;
                } else {
                    // No code: read the left/right axis.
                    'deflect: {
                        'classify: {
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_RIGHT_AXIS_2.call(c, 0x002c_e230)?;
                            c.mov_s(20, 0);
                            c.g[A0][0] = c.g[S0][0];
                            callee::MENU_SYS_ITEM_GET_LEFT_AXIS_2.call(c, 0x002c_e23c)?;
                            c.sub_s(20, 20, 0);
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc4));
                            c.c_le_s(20, 0);
                            let t = c.fcc();
                            if t {
                                break 'classify;
                            } else {
                                c.cycles += 5;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc0));
                                c.c_le_s(0, 20);
                                let t = !c.fcc();
                                c.g[V1][0] = 0;
                                if t {
                                    break 'deflect;
                                } else {
                                    break 'classify;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(20, 0);
                        let t = !c.fcc();
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'deflect;
                        } else {
                            c.cycles += 1;
                            c.g[V1][0] = 0xffffffffffffffff;
                            break 'deflect;
                        }
                    }
                    // 'b_2ce298/'b_2ce29c: step the repeat counter.
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[V0][0] = c.g[S1][0];
                    if t {
                        break 'handle_range;
                    }
                    // 'b_2ce310: scale the step and move the value.
                    c.cycles += 27;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3fbc));
                    c.mult(0, 2, 2, 2);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 5);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fb8));
                    c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x20));
                    c.lwc1(4, (c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sub_s(3, 3, 4);
                    c.lwc1(2, c.g[V1][0] as u32);
                    c.mul_s(0, 0, 6);
                    c.max_s(0, 0, 5);
                    c.mul_s(1, 0, 1);
                    c.mul_s(1, 20, 1);
                    c.mul_s(1, 1, 3);
                    c.add_s(2, 2, 1);
                    c.swc1(2, c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.lwc1(0, c.g[V0][0] as u32);
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        c.swc1(1, c.g[V0][0] as u32);
                    }
                    // 'b_2ce32c: clamp the other end from below.
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x20));
                    c.lwc1(0, c.g[V0][0] as u32);
                    c.c_lt_s(1, 0);
                    if c.fcc() {
                        c.swc1(1, c.g[V0][0] as u32);
                    }
                    c.cycles += 8;
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002c_e348)?;
                    // Convergence 1 (axis only).
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S1][0];
                }
                // Convergence 2 (disabled and axis).
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'handle_range;
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::slot as slot_4;

    pub(crate) use crate::game::ai::ai::tests::wf_2 as wf;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::ai::tests::LOAD_B0;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_5;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_6;
    pub(crate) use crate::game::bx::appman::tests::ptr as ptr_2;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;

    pub(crate) use crate::game::camera::cameracontroller::tests::pool as pool_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_10;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_12;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_7;
    pub(crate) use crate::game::fe::festaterewards::tests::be_library;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_8;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_5;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_6;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_9;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::render::bezierman::tests::b;
    pub(crate) use crate::game::render::font::tests::words;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::clear as clear_3;
    pub(crate) use crate::game::render::particle::tests::common as common_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_5;
    pub(crate) use crate::game::replay::replaycache::tests::dtor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_5;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_11;

    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;

    pub(crate) use crate::game::world::worldcache::tests::check_all;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_7;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_4;

    use ssx3_ee as ee;

    use ssx3_ee::s09;

    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn world_get_ai_frame_count_matches() {
        check(
            s09::f_002d1c98__generated,
            world_get_ai_frame_count,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 1);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = ptr_2(&mut r, 0);
                let manager = ptr_2(&mut r, 1);
                let ai = ptr_2(&mut r, 2);
                c.bus.write32(GAME_GP.wrapping_sub(WORLD_GLOBAL), world);
                c.bus.write32(world + WORLD_AI_MANAGER, manager);
                c.bus.write32(manager + AI_MANAGER_CURRENT, ai);
                c.bus.write32(ai + AI_FRAME_COUNT, r.next_u32());
            },
        );
    }

    #[test]
    fn world_get_flag_manager_matches() {
        check(
            s09::f_002d1cb0__generated,
            world_get_flag_manager,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 2);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = ptr_2(&mut r, 0);
                let manager = ptr_2(&mut r, 1);
                c.bus.write32(GAME_GP.wrapping_sub(WORLD_GLOBAL), world);
                c.bus.write32(world + WORLD_AI_MANAGER, manager);
                c.bus.write32(manager + FLAG_MANAGER, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_item_ctor_matches() {
        check(
            ssx3_ee::s09::f_002ca258,
            menu_sys_item_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0xa258 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_item_is_visible_matches() {
        check(
            ssx3_ee::s09::f_002ca378,
            menu_sys_item_is_visible,
            50,
            &|c, seed| {
                let mut r = Rng(0xa378 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_item_set_visible_matches() {
        check(
            ssx3_ee::s09::f_002ca388,
            menu_sys_item_set_visible,
            100,
            &|c, seed| {
                let mut r = Rng(0xa388 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    c.g[A1][0] = 0;
                } else {
                    c.g[A1][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 1);
                }
                c.bus.write32(DATA + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_item_is_disabled_matches() {
        check(
            ssx3_ee::s09::f_002ca3b0,
            menu_sys_item_is_disabled,
            50,
            &|c, seed| {
                let mut r = Rng(0xa3b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_page_ctor_matches() {
        check(
            ssx3_ee::s09::f_002caa58,
            menu_sys_page_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0xaa58 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_page_remove_all_items_matches() {
        check(
            ssx3_ee::s09::f_002cbe10,
            menu_sys_page_remove_all_items,
            50,
            &|c, seed| {
                let mut r = Rng(0xbe10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_reset_matches() {
        check(ssx3_ee::s09::f_002cc070, menu_sys_reset, 50, &|c, seed| {
            let mut r = Rng(0xc070 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn world_get_rider_count_matches() {
        check(
            ssx3_ee::s09::f_002d1af0,
            world_get_rider_count,
            50,
            &|c, seed| {
                let mut r = Rng(0x1af0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                let sub = DATA + 0x300;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0xc, sub);
                c.bus.write32(sub + 0x78, r.next_u32());
            },
        );
    }

    #[test]
    fn world_get_rider_matches() {
        check(
            ssx3_ee::s09::f_002d1b08,
            world_get_rider,
            200,
            &|c, seed| {
                let mut r = Rng(0x1b08 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                let table = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0xc, table);
                let idx = seed % 4;
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | idx as u64;
                for i in 0..4u32 {
                    let e = DATA + 0x800 + i * 0x100;
                    c.bus.write32(table + i * 4, e);
                    let v = if i == idx && seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(e + 0x28, v);
                }
            },
        );
    }

    #[test]
    fn wscript_man_run_obj_script5_hook_matches() {
        check(
            ee::s09::f_002d1ac8__generated,
            wscript_man_run_obj_script5_hook,
            200,
            &|c, seed| {
                machine(c, seed, 0x2d1ac8, 2);
                let man = DATA + 0x2000;
                let id = DATA + 0x1000;
                c.bus.write32(GAME_GP + 0xce8, man);
                c.bus.write32(man + 0x28c, 0);
                c.bus.write32(id + 0x78, 0);
                c.bus.write32(id + 0x88, 0);
                c.g[A0][0] = sx(id);
            },
        );
    }

    #[test]
    fn menu_sys_item_get_input_code_matches() {
        check(
            ee::s09::f_002ca3e8__generated,
            menu_sys_item_get_input_code,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_a3e8, 0);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x124, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_dtor_matches() {
        check(
            ee::s09::f_002cc048__generated,
            menu_sys_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2cc048),
        );
    }

    #[test]
    fn menu_sys_page_get_selected_index_matches() {
        check(
            s09::f_002cbf08__generated,
            menu_sys_page_get_selected_index,
            150,
            &|c, seed| {
                machine(c, seed, 0x2cbf08, 1);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x40);
                clear(c, CHILD, 0x40);
                // Selection state: no items selected / one item at +0x4.
                c.bus.write32(OBJ + 4, seed as u32 % 4);
                // MenuSysPage_ValidateSelection walks the page's item list at
                // +0xc; a null list is the "nothing to validate" path.
                c.bus.write32(OBJ + 0xc, 0);
            },
        );
    }

    #[test]
    fn menu_sys_point2d_item_on_focus_matches() {
        check(
            s09::f_002d08e0__generated,
            menu_sys_point2d_item_on_focus,
            150,
            &|c, seed| {
                machine(c, seed, 0x2d08e0, 1);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x40);
                c.bus.write32(OBJ + 0x40, seed as u32 & 0xff);
            },
        );
    }

    // -----------------------------------------------------------------
    // Table and vtable dispatchers.

    #[test]
    fn mod_obj_call_slot7_matches() {
        covered(
            s09::f_002d1b30__generated,
            mod_obj_call_slot7,
            120,
            &|c, seed| {
                machine(c, seed, 0x2d1b30, 1);
                clear(c, OBJ, 0x20);
                clear(c, VT, 0x40);
                // `ModObj::v07`: the vtable lives at the object's first word.
                slot(c, VT, 0x38);
                c.bus.write32(OBJ, VT);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed as u32);
                c.g[A2][0] = sx(seed as u32);
                c.g[A3][0] = sx(seed as u32);
            },
        );
    }

    /// The behaviour-library singleton (a zeroed dummy, so
    /// `BELibrary_GetInstance` neither constructs one nor reads the disc) and
    /// a world whose race manager holds one rider.
    pub(crate) fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x6_8400);
        c.bus.write32(arr + 0x7c, 1);
    }

    #[test]
    fn menu_sys_item_dtor_matches() {
        dtor_case(ee::s09::f_002ca280__generated, menu_sys_item_dtor, 0x2ca280);
    }

    #[test]
    fn menu_sys_page_dtor_matches() {
        dtor_case(ee::s09::f_002caa80__generated, menu_sys_page_dtor, 0x2caa80);
    }

    /// A menu page at `DATA`: item array at `+0xc`, selected index at `+0x4`,
    /// each item's vtable at `+0x10` (slots 0x18/0x1c for ValidateSelection,
    /// 0x40/0x44 for the group-item notify).
    pub(crate) fn menu_page_setup(c: &mut Cpu, seed: u32, salt: u64, page: u32) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        let n = 1 + (seed % 3);
        let vt = vtable(c, &mut r, &[0x18, 0x40]);
        c.bus.write32(page + 4, seed % n);
        for i in 0..3u32 {
            let item = DATA + 0x300 + 0x40 * i;
            c.bus.write32(page + 4 * i + 0xc, item);
            c.bus.write32(item + 0x10, vt);
        }
        r
    }

    #[test]
    fn menu_sys_page_get_selected_item_matches() {
        check(
            ee::s09::f_002cbf30__generated,
            menu_sys_page_get_selected_item,
            SEEDS_2,
            &|c, seed| {
                menu_page_setup(c, seed, 0x2cbf30, DATA);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// `MenuSysGroupItem_Measure`: 0 to 3 items (visible bit random, the
    /// current item one of them or none), the page's selected item either an
    /// ordinary item or the group itself; the measure methods are fakes and
    /// leave the stack rect as random data, so the height comparison goes
    /// both ways.
    #[test]
    fn menu_sys_group_item_measure_matches() {
        check(
            ee::s09::f_002cfb58__generated,
            menu_sys_group_item_measure,
            120,
            &|c, seed| {
                let mut r = menu_page_setup(c, seed, 0x2cfb58, DATA);
                let block = DATA + 0x1000;
                let ivt = vtable(c, &mut r, &[0x20]);
                w(c, block, DATA);
                w(c, block + 0x18, seed % 4);
                for i in 0..3u32 {
                    let item = DATA + 0x2000 + 0x100 * i;
                    w(c, block + 0x24 + 4 * i, item);
                    w(c, item + 4, r.next_u32());
                    w(c, item + 0x10, ivt);
                }
                let cur = DATA + 0x2000 + 0x100 * ((seed / 4) % 3);
                w(
                    c,
                    block + 0x14,
                    if (seed / 12) % 4 == 0 {
                        r.next_u32()
                    } else {
                        cur
                    },
                );
                if seed % 5 == 0 {
                    // The page's selected item is the group itself.
                    let pvt = c.lw(DATA + 0x310) as u32;
                    w(c, block + 0x10, pvt);
                    for i in 0..3u32 {
                        w(c, DATA + 0xc + 4 * i, block);
                    }
                }
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(block);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_v08_matches() {
        check(
            ee::s09::f_002cf8d8__generated,
            menu_sys_group_item_v08,
            SEEDS_2,
            &|c, seed| {
                // The group item wraps its page at +0x18.
                menu_page_setup(c, seed, 0x2cf8d8, DATA + 0x18);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_group_item_v10_matches() {
        check(
            ee::s09::f_002cfb28__generated,
            menu_sys_group_item_v10,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cfb28, 0);
                let vt = vtable(c, &mut r, &[0x20]);
                let owner = DATA + 0x100;
                c.bus.write32(DATA, owner);
                c.bus.write32(owner + 0x12c, vt);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    // ------------------------------------------------------- script

    #[test]
    fn wscript_man_obj_trigger_hook_matches() {
        check(
            ee::s09::f_002d19b8__generated,
            wscript_man_obj_trigger_hook,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d19b8, 8);
                let manager = DATA + 0x100;
                let trigger = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x38, 0x40]);
                c.bus.write32(GAME_GP + 0xce8, manager);
                c.bus.write32(manager, r.next_u32());
                c.bus.write32(trigger, vt);
                // The second half of WScriptMan_OnObjectTrigger must take its
                // early return: the trigger object at a3+0x480 is non-null and
                // the world's race manager reports a mode other than 4.
                c.bus.write32(trigger.wrapping_sub(0x6c0) + 0x480, 1);
                globals(c);
                c.bus.write32(DATA + 0x6_8100 + 0x214, 0);
                c.g[A0][0] = (DATA + 0x3000) as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.g[A2][0] = trigger as u64;
                c.bus.write32(DATA + 0x4000 + 0x50, r.next_u32());
            },
        );
    }

    // ---------------------------------------------------------- menu/stats

    #[test]
    fn menu_sys_submenu_item_measure_matches() {
        check(
            ee::s09::f_002ccd90__generated,
            menu_sys_submenu_item_measure,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccd_90, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn menu_sys_back_item_measure_matches() {
        check(
            ee::s09::f_002ccea8__generated,
            menu_sys_back_item_measure,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cce_a8, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn menu_sys_label_item_measure_matches() {
        check(
            ee::s09::f_002cd2f0__generated,
            menu_sys_label_item_measure,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd2_f0, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x14, (seed % 8) as u32);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    // -----------------------------------------------------------------------
    // MenuSys*Item_Measure trampolines.

    /// A zeroed item and row: `*(item + 0x14)` is 0, so `MenuSysItem_MeasureRow`
    /// takes its cheapest path and calls nothing.
    pub(crate) fn menu_item_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x800) as u64;
    }

    #[test]
    fn menu_sys_callback_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cd240__generated,
            menu_sys_callback_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cd2_40);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cd7b8__generated,
            menu_sys_int8_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cd7_b8);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cdbf0__generated,
            menu_sys_int_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cdb_f0);
            },
        );
    }

    #[test]
    fn menu_sys_float_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cdfa0__generated,
            menu_sys_float_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2cdf_a0);
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002ce368__generated,
            menu_sys_float_range_item_measure,
            32,
            &|c, seed| {
                menu_item_setup(c, seed, 0x2ce3_68);
            },
        );
    }

    #[test]
    fn menu_sys_action_item_draw_matches() {
        check(
            ee::s09::f_002cd0e8__generated,
            menu_sys_action_item_draw,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x2cd0e8, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x14, (seed % 5) as u32);
                c.bus
                    .write32(self_at + 0x18, if seed % 2 == 0 { 0 } else { seed % 9 });
            },
        );
    }

    #[test]
    fn world_get_rider_position_matches() {
        covered(
            ee::s09::f_002d1b58__generated,
            world_get_rider_position,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_1b58, 1);
                let world = DATA + 0x4000;
                let mgr = DATA + 0x5000;
                let race = DATA + 0x6000;
                let rider = DATA + 0x7000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + 0xc, race);
                let i = (r.next_u32() % 4) as u32;
                c.bus.write32(race + 4 * i + 0x28, rider);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(rider + 0x6c0, vt);
                c.g[A0][0] = i as u64;
            },
        );
    }

    #[test]
    fn menu_sys_item_is_selectable_matches() {
        check(
            ee::s09::f_002ca320__generated,
            menu_sys_item_is_selectable,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca3_20, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(
                    this + 4,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 8,
                        _ => 9,
                    },
                );
                c.bus.write32(this + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_page_set_row_color_matches() {
        check(
            ee::s09::f_002cb880__generated,
            menu_sys_page_set_row_color,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cb8_80, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 8);
                c.g[A2][0] = sx(if seed % 2 == 0 { 0 } else { 1 + seed % 3 });
                let vt = DATA + 0x400;
                c.bus.write32(this + 0x12c, vt);
                slot_2(c, &mut r, vt, 0x40, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn menu_sys_page_on_deactivate_matches() {
        check(
            ee::s09::f_002cbfd0__generated,
            menu_sys_page_on_deactivate,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cbf_d0, 1);
                let this = DATA + 0x100;
                let index = seed % 4;
                let item = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 4, index);
                c.bus.write32(this + 0xc + 4 * index, item);
                c.bus.write32(this + 0x124, r.next_u32());
                c.bus.write32(item + 0x10, DATA + 0x800);
                slot_2(c, &mut r, DATA + 0x800, 0x38, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn menu_sys_back_item_ctor_matches() {
        check(
            ee::s09::f_002ccdf0__generated,
            menu_sys_back_item_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccd_f0, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(if seed % 2 == 0 { DATA + 0x200 } else { 0 });
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn menu_sys_action_item_ctor_matches() {
        check(
            ee::s09::f_002ccf08__generated,
            menu_sys_action_item_ctor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccf_08, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn wscript_man_try_run_obj_script3_matches() {
        check(
            ee::s09::f_002d1a30__generated,
            wscript_man_try_run_obj_script3,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2d1a_30, 2);
                c.g[A0][0] = sx(seed as u32 & 0xf);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x28c, DATA + 0x2000);
                c.bus.write32(DATA + 0x1000 + 0x290, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_page_validate_selection_matches() {
        check(
            ee::s09::f_002caab0__generated,
            menu_sys_page_validate_selection,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2caab0, 1);
                let this = DATA + 0x100;
                let item = DATA + 0x2000;
                let obj = DATA + 0x3000;
                clear_2(c, this, 0x200);
                clear_2(c, item, 0x100);
                clear_2(c, obj, 0x100);
                let idx = seed % 3;
                c.bus.write32(this + 4, idx);
                c.bus.write32(this + 0xc + 4 * idx, item);
                c.bus.write32(item + 0x10, obj);
                c.bus.write16(obj + 0x18, 0x20);
                c.bus.write32(obj + 0x1c, FAKE);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_RESULTS, 1);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn menu_sys_page_end_draw_matches() {
        check(
            ee::s09::f_002cb2f8__generated,
            menu_sys_page_end_draw,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2cb2f8, 2);
                let mgr = DATA + 0x100;
                let obj = DATA + 0x2000;
                clear_2(c, mgr, 0x2000);
                clear_2(c, obj, 0x200);
                c.bus.write32(mgr + 0x10d8, obj);
                c.bus.write16(obj + 0x100, 0x20);
                c.bus.write32(obj + 0x104, FAKE);
                c.bus.write16(obj + 0xb0, 0x20);
                c.bus.write32(obj + 0xb4, FAKE + 0x10);
                c.bus.write32(mgr + 0xe84, 0x40 + seed % 8);
                c.bus.write32(gp_at(-0x854), mgr);
            },
        );
    }

    #[test]
    fn menu_sys_action_item_measure_matches() {
        check(
            ee::s09::f_002cd090__generated,
            menu_sys_action_item_measure,
            SEEDS_4,
            &|c, seed| {
                let mut r = common(c, seed, 0x2cd090, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                let item = DATA + 0x2000;
                clear_2(c, this, 0x400);
                clear_2(c, item, 0x100);
                c.bus.write32(item + 0x14, r.next_u32() % 100);
                let icon = seed % 3 != 0;
                c.bus
                    .write32(item + 0x18, if icon { DATA + 0x3000 } else { 0 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(item);
            },
        );
    }

    // ------------------------------------------------------------------
    // Menus.

    #[test]
    fn menu_sys_bool_item_draw_matches() {
        check(
            ee::s09::f_002cc758__generated,
            menu_sys_bool_item_draw,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2cc758, 1);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.bus.write32(
                    this + 0x18,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(this + 0x1c, ((seed / 3) % 2) as u32);
                c.bus.write32(this + 0x20, DATA + 0x200);
                c.bus.write32(this + 0x14, r.next_u32());
                c.bus.write32(DATA + 0x200, ((seed / 6) % 2) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_submenu_item_ctor_matches() {
        check(
            ee::s09::f_002cccc8__generated,
            menu_sys_submenu_item_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0x2cccc8);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_color_channel_item_ctor_matches() {
        check(
            ee::s09::f_002ced20__generated,
            menu_sys_color_channel_item_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0x2ced20);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_ctor_matches() {
        check(
            ee::s09::f_002cff28__generated,
            menu_sys_color_swatch_item_ctor,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0x2cff28);
                let this = DATA + 0x100;
                clear_3(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn menu_sys_spacer_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002cd3a0__generated,
            menu_sys_spacer_item_measure,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cd3_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                words(
                    c,
                    DATA + 0x300,
                    &[
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                    ],
                );
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_reset_value_matches() {
        check(
            ssx3_ee::s09::f_002cd578__generated,
            menu_sys_int8_item_reset_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cd5_7800 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA;
                let dst = DATA + 0x400;
                c.g[A0][0] = item as u64;
                c.bus.write32(item + 0x18, dst);
                let byte = match seed % 4 {
                    0 => 0,
                    1 => 0x7f,
                    2 => 0x80,
                    _ => r.next_u32() as u8,
                };
                c.bus.write8(item + 0x1c, byte);
                c.bus.write8(dst, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_reset_value_matches() {
        check(
            ssx3_ee::s09::f_002cd9b8__generated,
            menu_sys_int_item_reset_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cd9_b800 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA;
                let dst = DATA + 0x400;
                c.g[A0][0] = item as u64;
                c.bus.write32(item + 0x18, dst);
                let v = match seed % 4 {
                    0 => 0,
                    1 => -1i32,
                    2 => r.next_u32() as i32,
                    _ => (r.next_u32() % 1000) as i32,
                };
                c.bus.write32(item + 0x1c, v as u32);
                c.bus.write32(dst, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_float_item_reset_value_matches() {
        check(
            ssx3_ee::s09::f_002cdf78__generated,
            menu_sys_float_item_reset_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cdf_7800 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA;
                let dst = DATA + 0x400;
                c.g[A0][0] = item as u64;
                c.bus.write32(item + 0x18, dst);
                let v = match seed % 5 {
                    0 => 0.0f32.to_bits(),
                    1 => (-1.0f32).to_bits(),
                    2 => 2.5f32.to_bits(),
                    3 => f32::MIN_POSITIVE.to_bits(),
                    _ => r.edge_f32(),
                };
                c.bus.write32(item + 0x1c, v);
                c.bus.write32(dst, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------- menu items

    #[test]
    fn menu_sys_action_item_ctor_3_matches() {
        check(
            ee::s09::f_002ccf98__generated,
            menu_sys_action_item_ctor_3,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_cf98, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (seed % 7) as u64;
                c.g[A3][0] = (seed % 11) as u64;
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_draw_matches() {
        check(
            ee::s09::f_002cd7f0__generated,
            menu_sys_int8_item_draw,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_d7f0, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                w(c, DATA + 0x18, DATA + 0x300);
                b(c, DATA + 0x300, (seed % 100) as u8);
                w(c, DATA + 0x14, (seed % 9) as u32);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_draw_matches() {
        check(
            ee::s09::f_002cdc28__generated,
            menu_sys_int_item_draw,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_dc28, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                w(c, DATA + 0x18, DATA + 0x300);
                w(c, DATA + 0x300, (seed.wrapping_mul(0x9e37)) as u32);
                w(c, DATA + 0x14, (seed % 9) as u32);
            },
        );
    }

    #[test]
    fn menu_sys_draw_matches() {
        check(
            ee::s09::f_002cc350__generated,
            menu_sys_draw,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2cc350, 0);
                globals_2(c);
                let obj = OBJ2;
                w_2(c, GAME_GP - 0x3ff4, 0x3f00_0000);
                w_2(c, OBJ_2 + 0x58, obj);
                w_2(c, OBJ_2, if seed % 2 == 0 { 1 } else { 2 });
                w_2(c, OBJ_2 + 4, OBJ3);
                w_2(c, obj + 0x30, 0x4000_0000);
                w_2(c, obj + 0x34, 0x4040_0000);
                w_2(c, OBJ3 + 0x12c, VT);
                slot_3(c, VT, 0x28, 0, fakes::FAKE);
                c.g[A0][0] = OBJ_2 as u64;
                c.f[12] = 0x3f80_0000;
                c.f[13] = 0x4000_0000;
            },
        );
    }

    // ------------------------------------------------------------- menus

    pub(crate) fn menu_input_test(gen: GuestFn, hand: GuestFn, salt: u64, callback: bool) {
        check(gen, hand, 8, &|c, seed| {
            let mut r = base_2(c, seed, salt, 3);
            c.g[A0][0] = OBJ as u64;
            let menu = DATA + 0x2000;
            let page = DATA + 0x2200;
            let item = DATA + 0x2400;
            let disabled = seed % 4 == 1 || seed % 4 == 3;
            w(c, OBJ + 4, if disabled { 8 } else { 0 });
            w(c, OBJ, menu);
            w(c, menu + 0x124, page);
            let code = match seed % 4 {
                0 | 1 => 6u32,
                2 => 1,
                _ => 8,
            };
            w(c, page + 0x48, code);
            w(c, page, 1);
            w(c, page + 4, item);
            w(c, page + 8, item);
            let vt = vtable(c, &mut r, &[0x38]);
            w(c, item + 0x12c, vt);
            if callback {
                w(c, OBJ + 0x18, FAKE);
            }
        });
    }

    #[test]
    fn menu_sys_back_item_handle_input_matches() {
        menu_input_test(
            ee::s09::f_002cce38__generated,
            menu_sys_back_item_handle_input,
            0x002c_ce38,
            false,
        );
    }

    #[test]
    fn menu_sys_callback_item_handle_input_matches() {
        menu_input_test(
            ee::s09::f_002cd1d0__generated,
            menu_sys_callback_item_handle_input,
            0x002c_d1d0,
            true,
        );
    }

    #[test]
    fn menu_sys_text_edit_item_ctor_matches() {
        check(
            ee::s09::f_002ce418__generated,
            menu_sys_text_edit_item_ctor,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x002c_e418, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (r.next_u32() & 0xff) as u64;
                c.g[A2][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b'n');
                c.bus.write8(DATA + 0x4001, 0);
            },
        );
    }

    /// Point every virtual slot_4 a test may reach at [`EMPTY`].
    pub(crate) fn empty_slots(c: &mut Cpu, vt: u32, offs: &[u32]) {
        for off in offs {
            slot_4(c, vt, *off, 0, EMPTY);
        }
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for
    /// paths that allocate beyond the size-class pools.
    pub(crate) fn heap_init_2(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    #[test]
    fn menu_sys_item_handle_input_matches() {
        check(
            ssx3_ee::s09::f_002ca2b0__generated,
            menu_sys_item_handle_input,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca2b0, 0);
                let item = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let input = DATA + 0x3000;
                c.g[A0][0] = item as u64;
                w32(c, item, menu);
                w32(c, menu + 0x124, input);
                let code = match seed % 9 {
                    0 => 0,
                    1 => 1,
                    2 => 7,
                    3 => 8,
                    n => r.next_u32() % 10 + n as u32,
                };
                w32(c, input + 0x48, code);
            },
        );
    }

    #[test]
    fn menu_sys_page_on_activate_matches() {
        check(
            ssx3_ee::s09::f_002cbf60__generated,
            menu_sys_page_on_activate,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cbf60, 0);
                let page = DATA + 0x1000;
                c.g[A0][0] = page as u64;
                c.g[A1][0] = (seed % 3) as u64;
                let count = (seed % 4) as u32;
                let selected = if seed % 3 == 0 {
                    count + 1
                } else {
                    (seed % 5) as u32
                };
                w32(c, page, count);
                w32(c, page + 4, selected);
                // Item array lives at page + 0xc, indexed by the selection.
                let vt = DATA + 0x5000;
                slot_4(c, vt, 0x30, 0, LOAD_B0);
                slot_4(c, vt, 0x18, 0, LOAD_B0);
                for i in 0..8u32 {
                    let item = DATA + 0x3000 + i * 0x100;
                    w32(c, page + 0xc + i * 4, item);
                    w32(c, item + 0x10, vt);
                    w32(c, item + 0xb0, 1); // non-zero: no SelectNext walk
                }
                w32(c, DATA + 0x3000 + 0xb0, r.next_u32() | 1);
            },
        );
    }

    #[test]
    fn menu_sys_submenu_item_handle_input_matches() {
        check(
            ssx3_ee::s09::f_002ccd20__generated,
            menu_sys_submenu_item_handle_input,
            SEEDS_7,
            &|c, seed| {
                let r = machine(c, seed, 0x2ccd20, 0);
                let item = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let input = DATA + 0x3000;
                let page = DATA + 0x4000;
                c.g[A0][0] = item as u64;
                w32(c, item, menu);
                w32(c, menu + 0x124, input);
                w32(
                    c,
                    input + 0x48,
                    match seed % 4 {
                        0 => 6,
                        1 => 0,
                        n => n as u32,
                    },
                );
                // disabled flag (`item+4 & 8`) and no base action
                w32(c, item + 4, if seed % 5 == 0 { 8 } else { 0 });
                w32(c, item + 0x18, page);
                // PushPage's item: `page+0x12c` vtable.
                let vt = DATA + 0x5000;
                slot_4(c, vt, 0x30, 0, EMPTY);
                w32(c, page + 0x12c, vt);
                let _ = r;
            },
        );
    }

    #[test]
    fn menu_sys_float_item_draw_matches() {
        check(
            ssx3_ee::s09::f_002ce100__generated,
            menu_sys_float_item_draw,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ce100, 0);
                pool_2(c);
                heap_init_2(c);
                be_library(c);
                let item = DATA + 0x1000;
                let value = DATA + 0x2000;
                let page = DATA + 0x3000;
                c.g[A0][0] = item as u64;
                c.g[A1][0] = (seed % 5) as u64;
                w32(c, item + 0x18, value);
                w32(c, value, r.f32(1000.0));
                w32(
                    c,
                    item + 0x24,
                    if seed % 4 == 0 {
                        0x3f80_0000
                    } else {
                        r.f32(10.0)
                    },
                );
                w32(c, item + 0x28, r.f32(100.0));
                w32(c, item, page);
                w32(c, item + 0x14, r.next_u32());
                // DrawRow: item is not the selected item (different page selection).
                w32(c, page + 4, 0);
                w32(c, page + 0xc, item);
                let vt = DATA + 0x5000;
                empty_slots(c, vt, &[0x18, 0x1c]);
                w32(c, item + 0x10, vt);
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_draw_matches() {
        check(
            ee::s09::f_002ce3a0__generated,
            menu_sys_float_range_item_draw,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ce3a0, 3);
                be_library_2(c);
                let item = DATA;
                let row = DATA + 0x800;
                c.g[A0][0] = item as u64;
                c.g[A1][0] = row as u64;
                w(c, item, 0);
                w(c, item + 0x18, DATA + 0x100);
                w(c, DATA + 0x100, r.edge_f32());
                w(c, item + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn menu_sys_group_item_on_focus_matches() {
        check(
            ee::s09::f_002cfe78__generated,
            menu_sys_group_item_on_focus,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2cfe_78, 4);
                let self_ = DATA;
                let page = DATA + 0x18;
                w(c, self_, 0);
                w(c, self_ + 0x18, 0);
                let mode = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + (seed % 7) as u64,
                };
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = mode;
                let _ = page;
            },
        );
    }

    #[test]
    fn menu_sys_int_item_ctor_3_matches() {
        check(
            ee::s09::f_002cd8f8__generated,
            menu_sys_int_item_ctor_3,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x2cd8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed as u64) << 1;
                c.g[A3][0] = (seed as u64) << 2;
                c.g[T0][0] = (seed as u64) << 3;
                c.g[T1][0] = (seed as u64) << 4;
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_measure_matches() {
        check(
            ee::s09::f_002cff88__generated,
            menu_sys_color_swatch_item_measure,
            50,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x2cff, 1);
                fill(c, &mut r, DATA, 0x40);
                w(c, DATA + 0x200 + 0x18, (seed % 4) as u32);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    // -----------------------------------------------------------------------
    // Menu system.

    #[test]
    fn menusyspage_handle_input_matches() {
        covered(
            ee::s09::f_002cacc0__generated,
            menusyspage_handle_input,
            SEEDS_9,
            &|c, seed| {
                base_5(c, seed, 0xcacc0, 1);
                let page = DATA + 0x100;
                let item = DATA + 0x200;
                c.g[A0][0] = sx(page);
                c.bus.write32(page + 4, (seed % 3) as u32);
                c.bus.write32(page + 0xc + 4 * (seed % 3), item);
                let vt = DATA + 0x300;
                slot_6(c, vt, 0x10, 0, FAKE);
                c.bus.write32(item + 0x10, vt);
                // The fake returns a forced code: 3 (next), 4 (prev) or other.
                let code = match seed % 3 {
                    0 => 3,
                    1 => 4,
                    _ => 0x11,
                };
                for n in 0..8u32 {
                    c.bus.write32(LOG_RESULTS + 4 * n, code);
                }
            },
        );
    }

    #[test]
    fn menusyspage_insert_item_matches() {
        check(
            ee::s09::f_002cbcc8__generated,
            menusyspage_insert_item,
            SEEDS_9,
            &|c, seed| {
                base_5(c, seed, 0xbcc8, 0);
                let item = DATA + 0x100;
                let page = DATA + 0x200;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(page);
                let count = 3u32;
                c.bus.write32(item, count);
                c.bus.write32(item + 4, (seed % 4) as u32);
                c.bus
                    .write32(item + 0x124, if seed % 2 == 0 { 0 } else { 1 });
                for i in 0..5u32 {
                    c.bus.write32(item + 0xc + 4 * i, 0x1000 + i);
                }
                c.g[A2][0] = match seed % 5 {
                    0 => sx(0xffff_ffff),
                    1 => sx(count),
                    _ => ((seed / 5) % 4) as u64,
                };
            },
        );
    }

    #[test]
    fn menusysactionitem_handle_input_matches() {
        check(
            ee::s09::f_002cd008__generated,
            menusysactionitem_handle_input,
            SEEDS_9,
            &|c, seed| {
                base_5(c, seed, 0xcd008, 0);
                let item = DATA + 0x100;
                let parent = DATA + 0x200;
                let input = DATA + 0x300;
                c.g[A0][0] = sx(item);
                c.bus.write32(item, parent);
                c.bus.write32(item + 4, if seed % 2 == 0 { 0 } else { 8 });
                c.bus.write32(item + 0x1c, 0x55);
                c.bus.write32(parent + 0x124, input);
                c.bus
                    .write32(input + 0x48, [0u32, 1, 2, 6, 7][(seed / 2 % 5) as usize]);
                c.bus.write32(parent + 0x12c, input);
                slot_6(c, input, 0x18, 0, FAKE);
            },
        );
    }

    driven!(enum_draw_generated, ee::s09::f_002ccc38__generated);

    driven!(enum_draw_hand, menu_sys_enum_item_draw);

    #[test]
    fn menu_sys_enum_item_draw_matches() {
        check_events(enum_draw_generated, enum_draw_hand, 90, &|c, seed| {
            let mut r = base_6(c, seed, 0x2c_cc38);
            globals_5(c);
            // The page the row belongs to, and the item's own row.
            w(c, DATA, DATA + 0x6000);
            w(c, DATA + 0x14, 7);
            // Enum table: {name, value} pairs.
            let count = 1 + (seed % 3);
            w(c, DATA + 0x18, count);
            w(c, DATA + 0x1c, DATA + 0x2000);
            for i in 0..3u32 {
                w(c, DATA + 0x2000 + 8 * i, DATA + 0x7000);
                w(c, DATA + 0x2000 + 8 * i + 4, 0x100 + i);
            }
            // The selected value: matching one entry on most seeds.
            w(c, DATA + 0x20, DATA + 0x2100);
            w(
                c,
                DATA + 0x2100,
                if seed % 4 == 0 {
                    0x999
                } else {
                    0x100 + seed % 3
                },
            );
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            schedule(c, &mut r, seed, ENUM_DRAW_ENTRY)
        });
    }

    #[test]
    fn menu_sys_float_item_ctor_2_matches() {
        check(
            ee::s09::f_002cdce8__generated,
            menu_sys_float_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2c_dce8);
                globals_5(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    // ----------------------------------------------------------- menu pages

    #[test]
    fn menu_sys_page_set_selection_matches() {
        check(
            ssx3_ee::s09::f_002cbe68__generated,
            menu_sys_page_set_selection,
            16,
            &|c, seed| {
                let _ = base_7(c, seed, 0x002c_be68);
                let page = DATA + 0x100;
                let items = DATA + 0x2000;
                clear_4(c, page, 0x100);
                let count = 1 + (seed % 4);
                w(c, page, count);
                w(c, page + 4, (seed % count) as u32);
                // The item array starts at +0xc but is indexed with `(index >> 2)`
                // by the original's own convention; fill both layouts with valid
                // item pointers.
                for i in 0..count {
                    let item = DATA + 0x3000 + 0x80 * i;
                    let vt = DATA + 0x4000 + 0x80 * i;
                    clear_4(c, item, 0x40);
                    clear_4(c, vt, 0x40);
                    w(c, item + 0x10, vt);
                    c.bus.write16(vt + 0x30, 0);
                    w(c, vt + 0x34, crate::difftest::fakes::FAKE + 0x10);
                    c.bus.write16(vt + 0x38, 0);
                    w(c, vt + 0x3c, crate::difftest::fakes::FAKE);
                    w(c, items + 4 * i, item);
                    w(c, page + 0xc + 4 * i, item);
                }
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                w(c, crate::difftest::fakes::LOG_RESULTS + 4, 1);
                c.g[A0][0] = sx(page);
                c.g[A1][0] = match seed % 5 {
                    0 => 0xffff_ffff_ffff_ffff,
                    1 => 0,
                    _ => (seed % 5) as u64,
                };
            },
        );
    }

    /// An object with a vtable whose `+off` slot is `{delta, fn}`.
    pub(crate) fn vt_slot(c: &mut Cpu, obj: u32, off: u32, delta: i16, f: u32) {
        w_3(c, obj, DATA + 0x34_0000);
        c.bus.write16(DATA + 0x34_0000 + off, delta as u16);
        w_3(c, DATA + 0x34_0000 + off + 4, f);
    }

    #[test]
    fn menu_sys_pop_page_matches() {
        check(
            ssx3_ee::s09::f_002cc460__generated,
            menu_sys_pop_page,
            30,
            &|c, seed| {
                let _r = base_8(c, seed, 0x2cc460, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                let count = (seed % 3) as u32;
                w_3(c, this, count);
                for i in 0..3u32 {
                    let page = DATA + 0x2000 + i * 0x100;
                    w_3(c, this + 4 + 4 * i, page);
                    vt_slot(c, page + 0x120, 0x38, 0, 0);
                    w_3(c, page + 0x12c, page + 0x120);
                }
            },
        );
    }

    // ----------------------------------------------------------------- menus

    #[test]
    fn menu_sys_push_page_matches() {
        check(
            ee::s09::f_002cc3b8__generated,
            menu_sys_push_page,
            SEEDS_10,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2cc3_b8, 6);
                let obj = SCRATCH_2;
                let item = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = item as u64;
                // The item list: 0, 1 or 2 pages, each with a vtable at_4 +0x12c.
                w(c, obj, (seed % 3) as u32);
                for i in 0..3u32 {
                    let e = obj + 0x100 + 4 * i;
                    w(c, e, obj + 0x200 + 0x40 * i);
                    let v = obj + 0x200 + 0x40 * i;
                    w(c, v + 0x100, 0);
                    w(c, v + 0x12c, obj + 0x800 + 0x40 * i);
                    slot_7(c, obj + 0x800 + 0x40 * i, 0x30, i);
                    slot_7(c, obj + 0x800 + 0x40 * i, 0x38, i + 1);
                }
                w(c, obj + 4, item);
                w(c, item + 0x12c, obj + 0x900);
                slot_7(c, obj + 0x900, 0x30, 4);
                slot_7(c, obj + 0x900, 0x38, 5);
            },
        );
    }

    #[test]
    fn menu_sys_vector_label_item_draw_matches() {
        check(
            ee::s09::f_002d0500__generated,
            menu_sys_vector_label_item_draw,
            SEEDS_10,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2d05_00, 6);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                // Three positions converted to doubles, then a formatted row.
                let pos = obj + 0x2000;
                wf(c, pos, (seed as f32) * 0.5);
                wf(c, pos + 4, 1.0);
                wf(c, pos + 8, 2.0);
                w(c, obj + 0x14, pos);
                w(c, obj + 0x18, 0x40);
                w(c, obj + 0x1000 + 0x14, pos);
                w(c, obj + 0x1000 + 0x18, 0);
            },
        );
    }

    // ---------------------------------------------------------- label measure

    #[test]
    fn menu_sys_vector_label_item_measure_matches() {
        check(
            ee::s09::f_002d0450__generated,
            menu_sys_vector_label_item_measure,
            SEEDS_10,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2d04_50, 6);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                let pos = obj + 0x2000;
                wf(c, pos, (seed as f32) * 0.25);
                wf(c, pos + 4, 1.5);
                wf(c, pos + 8, 3.0);
                w(c, obj + 0x14, pos);
                w(c, obj + 0x18, 0x48);
                w(c, obj + 0x1000 + 0x14, pos);
                w(c, obj + 0x1000 + 0x18, 0);
            },
        );
    }

    #[test]
    fn menu_sys_bool_item_handle_input_matches() {
        check(
            ee::s09::f_002cc578__generated,
            menu_sys_bool_item_handle_input,
            80,
            &|c, seed| {
                let _r = base_10(c, seed, 0x2cc5);
                c.g[A0][0] = DATA as u64;
                // Owner -> menu -> input code.
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x124, DATA + 0x1100);
                let code = [2u32, 3, 6, 5][(seed % 4) as usize];
                c.bus.write32(DATA + 0x1100 + 0x48, code);
                c.bus.write32(DATA + 0x1000 + 0x12c, DATA + 0x1300);
                c.bus.write16(DATA + 0x1300 + 0x20, 0);
                c.bus.write32(DATA + 0x1300 + 0x24, FAKE);
                // Item flags and the bool's address.
                c.bus.write32(DATA + 4, if seed % 8 == 0 { 8 } else { 0 });
                c.bus
                    .write32(DATA + 0x18, if seed % 2 == 0 { 1 } else { 0 });
                c.bus
                    .write32(DATA + 0x1c, if seed % 2 == 0 { 1 } else { 0 });
                c.bus.write32(DATA + 0x20, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200, seed & 1);
            },
        );
    }

    #[test]
    fn menu_sys_page_select_next_matches() {
        check_lohi(
            ee::s09::f_002cab08__generated,
            menu_sys_page_select_next,
            16,
            &|c, seed| {
                let mut r = base_11(c, seed, 0x2cab08, 3);
                let this = DATA;
                clear_5(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let count = 4u32;
                w(c, this, if seed % 2 == 0 { 0 } else { count - 1 });
                w(c, this + 4, count);
                let vt = vtable(c, &mut r, &[0x18, 0x30, 0x38]);
                for i in 0..count {
                    let item = DATA + 0x200 + 0x80 * i;
                    clear_5(c, item, 0x40);
                    w(c, item + 0x10, vt);
                }
                for i in 0..count {
                    w(c, this + 0xc + 4 * i, DATA + 0x200 + 0x80 * i);
                }
            },
        );
    }

    #[test]
    fn menu_sys_page_select_prev_matches() {
        check_lohi(
            ee::s09::f_002cabe0__generated,
            menu_sys_page_select_prev,
            16,
            &|c, seed| {
                let mut r = base_11(c, seed, 0x2cabe0, 3);
                let this = DATA;
                clear_5(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let count = 4u32;
                w(c, this, if seed % 2 == 0 { 0 } else { 1 });
                w(c, this + 4, count);
                let vt = vtable(c, &mut r, &[0x18, 0x30, 0x38]);
                for i in 0..count {
                    let item = DATA + 0x200 + 0x80 * i;
                    clear_5(c, item, 0x40);
                    w(c, item + 0x10, vt);
                }
                for i in 0..count {
                    w(c, this + 0xc + 4 * i, DATA + 0x200 + 0x80 * i);
                }
            },
        );
    }

    #[test]
    fn menu_sys_build_quad_matches() {
        same(
            ssx3_ee::s09::f_002cb350__generated,
            menu_sys_build_quad,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cb350 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = DATA + 0x100;
                c.g[A0][0] = v as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                for i in 0..8 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.f32(1000.0));
                }
                for i in 0..4 {
                    c.bus.write32(DATA + 0x240 + 4 * i, r.f32(1000.0));
                }
                // D_004A289C -> draw module; slot[0x23c] = a short generated leaf
                // (MenuSys_IsActive) called with this = draw + 0x100.
                let draw = DATA + 0x2000;
                c.bus.write32(GAME_GP - 0x854, draw);
                let slot = DATA + 0x4000;
                c.bus.write32(draw + 0x10d8, slot);
                c.bus.write32(slot + 0x238, 0x100);
                c.bus.write32(slot + 0x23c, 0x002c_c098);
                c.bus.write32(draw + 0x100, 0);
            },
        );
    }

    pub(crate) fn wf_2(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    // -- 0x002cb180 MenuSysPage_BeginDraw --------------------------------

    #[test]
    fn menu_sys_page_begin_draw_matches() {
        covered(
            ssx3_ee::s09::f_002cb180__generated,
            menu_sys_page_begin_draw,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2cb180, 4);
                let ms = DATA + 0x1000;
                let page = DATA + 0x2000;
                let renderer = DATA + 0x3000;
                w(c, GAME_GP.wrapping_sub(0x854), ms);
                w(c, ms + 0xe84, page);
                w(c, ms + 0x10d8, renderer);
                for (i, off) in [0xa8u32, 0xd0, 0xf8, 0x110].into_iter().enumerate() {
                    c.bus
                        .write16(renderer + off, (r.next_u32() & 0x7ff0) as u16);
                    w(c, renderer + off + 4, FAKE + 0x10 * i as u32);
                }
                for i in 0..0x40u32 {
                    c.bus
                        .write8(page + i, (seed as u8).wrapping_add(i as u8) | 1);
                }
                w(c, page + 0x4, r.next_u32());
                w(c, page + 8, r.next_u32());
            },
        );
    }

    // -- 0x002d0d48 MenuSysPoint2DItem_Measure --------------------------

    #[test]
    fn menu_sys_point2d_item_measure_matches() {
        check(
            ssx3_ee::s09::f_002d0d48__generated,
            menu_sys_point2d_item_measure,
            100,
            &|c, seed| {
                let mut r = Rng(0x2d0d48 ^ (seed as u64) << 4);
                c.g[GP][0] = GAME_GP as u64;
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let item = DATA + 0x1000;
                let page = DATA + 0x2000;
                let menu = DATA + 0x3000;
                let font = DATA + 0x4000;
                let style = DATA + 0x5000;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(page);
                w(c, item, menu);
                w(c, page, menu);
                w(c, item + 0x14, DATA + 0x6000);
                w(c, item + 0x40, if seed % 2 == 0 { 1 } else { 0 });
                w(c, menu + 0x124, font);
                w(c, font + 0x58, style);
                wf_2(c, style + 0x14, 20.0f32.to_bits());
                wf_2(c, style + 0x34, 1.0f32.to_bits());
                wf_2(c, style + 0x38, 3.0f32.to_bits());
                wf_2(c, style + 0x3c, 4.0f32.to_bits());
                wf_2(c, item + 0x54, r.f32(100.0));
                wf_2(c, item + 0x64, r.f32(100.0));
                wf_2(c, item + 0x74, r.f32(100.0));
                for i in 0..0x20u32 {
                    w(c, item + 0x80 + 4 * i, r.next_u32());
                }
                // an empty string for the text measurement
                c.bus.write8(DATA + 0x6000, 0);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_draw_matches() {
        const S: u64 = 0x002c_fd28;
        check_all(
            ee::s09::f_002cfd28__generated,
            menu_sys_group_item_draw,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, S, 32);
                let this = DATA + 0x2000;
                let menu = DATA + 0x4000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1000); // the block's starting rect
                let count = 1 + (seed % 3) as u32;
                w(c, this + 0x18, count);
                w(c, this, menu);
                let cur = DATA + 0x5000 + (seed % count) * 0x100;
                w(c, this + 0x14, cur);
                for i in 0..count {
                    let it = DATA + 0x5000 + i * 0x100;
                    // The item list is embedded at self+0x24.
                    w(c, this + 0x24 + i * 4, it);
                    // Visibility bit 0, then arbitrary flags.
                    w(c, it + 0x4, ((r.next_u32() & 1) << 1) | (r.next_u32() & 1));
                    let vt = DATA + 0x8000 + i * 0x100;
                    w(c, it + 0x10, vt);
                    c.bus.write16(vt + 0x28, (r.next_u32() as u16) & 0xfff0);
                    w(c, vt + 0x2c, 0x4000);
                    // The row height the vtable draw would leave.
                    w(c, this + 0x14c + i * 0x10 + 0xc, r.next_u32());
                }
                // The page's selected item: sometimes this group itself.
                let idx = seed % count;
                w(c, menu + 4, idx);
                w(
                    c,
                    menu + 0xc + idx * 4,
                    if seed % 2 == 0 { this } else { DATA + 0x9000 },
                );
                for k in 0..4u32 {
                    w(c, DATA + 0x1000 + k * 4, r.edge_f32());
                }
            },
        );
    }

    pub(crate) const MENU: u32 = DATA + 0x1_0000;

    pub(crate) const MENU_ENTRY: u32 = DATA + 0x1_1000;

    #[test]
    fn menu_sys_update_matches() {
        check_all_2(
            ee::s09::f_002cc0c0__generated,
            menu_sys_update,
            20,
            &|c, seed| {
                let mut r = base_12(c, seed, 0x2cc0_c0, 16);
                c.pc = 0x002c_c0c0; // the dispatcher is entered at_5 its entry block
                c.g[A0][0] = sx(MENU);
                c.g[A1][0] = sx(seed);
                c.bus.write32(MENU + 0xbc, (seed % 4) as u32);
                c.bus
                    .write32(MENU + 0xc0, if seed % 3 == 0 { 0 } else { MENU_ENTRY });
                c.bus.write8(GAME_GP - 0xc88, (seed % 2) as u8);
                for i in 0..8u32 {
                    c.bus.write32(
                        MENU + 0xd8 + 4 * i,
                        if seed % 2 == 0 {
                            0
                        } else {
                            MENU_ENTRY + 0x100 * i
                        },
                    );
                }
                c.bus.write32(MENU_ENTRY + 0x8, 0);
                c.bus.write32(MENU + 0x9c, DATA + 0x1_2000);
                let _ = &mut r;
            },
        );
    }

    /// `World_GetPeakNumber` through the real BE library (a dummy singleton
    /// at gp-0x1f04, interface 0 resolving to a fake) and the real
    /// `BENewRaceInt_GetTrackPeak`; the fake call may unwind.
    #[test]
    fn world_get_peak_number_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        check(
            ee::s09::f_002d1ba0__generated,
            world_get_peak_number,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_1ba0, 1);
                // A created BE library singleton, interface index 0.
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
                c.bus.write32(0x0053_08a8, 0);
                // The current track (BENewRaceInt) and its peak in the table.
                let track = seed % 8;
                c.bus.write32(0x0053_5c08, track);
                c.bus
                    .write32(0x0043_d950 + track * 0x64 + 0x54, r.next_u32() % 5);
                c.pc = 0x002d_1ba0;
            },
        );
    }

    pub(crate) use crate::game::lib::snd::tests::{WscriptFakeSlots, FAKE_CTRL, FAKE_MAGIC};

    #[test]
    fn wscript_man_try_run_kill_script_matches() {
        use crate::difftest::fakes::{machine, LOG_RESULTS};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        let _slots = WscriptFakeSlots::install();
        check(
            ssx3_ee::s09::f_002d19e8__generated,
            wscript_man_try_run_kill_script,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_19e8, 1);
                // This machine's call to the faked callees (see snd.rs).
                c.bus.write32(FAKE_CTRL, FAKE_MAGIC);
                // The script manager, with junk around the two fields the
                // callee touches.
                let manager = DATA + 0x100;
                fill(c, &mut r, manager + 0x280, 8);
                c.bus.write32(GAME_GP + WSCRIPT_MANAGER_GP, manager);
                c.bus.write32(manager + 0x28c, DATA + 0x200);
                // Half the seeds have no kill script for the id.
                c.bus.write32(LOG_RESULTS, (seed % 2) as u32);
                c.g[A0][0] = (r.next_u32() % 64) as u64;
            },
        );
    }

    /// A world whose systems block holds the focus manager, with a focus
    /// count of 0..4 and the added object's flags.
    #[test]
    fn world_add_focus_object_matches() {
        use crate::difftest::fakes::machine;
        const WORLD: u32 = DATA + 0x1000;
        const SYSTEMS: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        const OBJ: u32 = DATA + 0x4000;
        check(
            ee::s09::f_002d1bf0__generated,
            world_add_focus_object,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_1bf0, 0);
                c.pc = 0x002d_1bf0;
                let gp = GAME_GP;
                c.bus.write32(gp.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x84, SYSTEMS);
                c.bus.write32(SYSTEMS + 0xc, DATA + 0x2500);
                c.bus.write32(DATA + 0x2500 + 0xa4, FOCUS);
                c.bus
                    .write32(FOCUS + 0xd4, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(FOCUS + 0xd8, (seed % 5) as u32);
                c.bus.write32(OBJ + 8, r.next_u32());
                c.g[A0][0] = sx(OBJ);
            },
        );
    }

    #[test]
    fn wscriptman_run_obj_script0_hook_matches() {
        check(
            ee::s09::f_002d1a78__generated,
            wscriptman_run_obj_script0_hook,
            100,
            &|c, seed| {
                let mut r = Rng(0x002d_1a78 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002d_1a78;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x8000);
                c.g[A0][0] = sx(DATA + 0x9000 + (r.next_u32() & 0xf0));
                let _ = seed;
            },
        );
    }
    use crate::difftest::elf_image;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use std::ptr::addr_of_mut;

    /// Swatch fixture: the item on a real page (for the row draw), an RGB
    /// triple, a colour struct, and the `gp - 0x854` + vtable chain the tail
    /// walks. `equal` steers the `c_eq_s` arm via `struct+0`.
    pub(crate) fn swatch_draw_fixture(c: &mut Cpu, seed: u32, equal: bool) -> u32 {
        elf_image(c);
        let mut r = machine(c, seed, 0x2d00_08, 2);
        let item = DATA + 0x3000;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xD000_0000 ^ seed ^ w);
        }
        // Page for the row draw: the item owns it, selection steered.
        let page = DATA + 0x100;
        c.bus.write32(page, 2);
        let sel = seed % 3;
        c.bus.write32(page + 4, sel);
        for i in 0..2u32 {
            c.bus.write32(page + 0xc + 4 * i, DATA + 0x2000 + i * 0x40);
        }
        c.bus.write32(item, page);
        c.bus.write32(item + 4, sel);
        c.bus.write32(page + 0xc + 4 * (sel % 2), item);
        c.bus.write32(item + 0x18, 0x2222_2222 ^ seed);
        let rgb = DATA + 0x6000;
        c.bus.write32(rgb, r.edge_f32());
        c.bus.write32(rgb + 4, r.edge_f32());
        c.bus.write32(rgb + 8, r.edge_f32());
        let st = DATA + 0x6100;
        c.bus
            .write32(st, if equal { 0x3F80_0000 } else { r.edge_f32() });
        c.bus.write32(st + 4, r.edge_f32());
        c.bus.write32(st + 8, r.edge_f32());
        c.bus.write32(st + 0xc, r.edge_f32());
        c.bus.write32(item + 0x14, st);
        c.g[A0][0] = sx(item);
        c.g[A1][0] = sx(rgb);
        c.g[A2][0] = sx(0x2222_2222 ^ seed);
        c.g[A3][0] = sx(0x3333_3333 ^ seed);
        c.g[T0][0] = sx(0x4444_4444 ^ seed);
        // `gp - 0x854` chain for the tail call.
        let g = DATA + 0xb000;
        let h = DATA + 0xb100;
        c.bus.write32(GAME_GP.wrapping_sub(0x854), g);
        c.bus.write32(g + 0xe84, h);
        c.bus.write32(h + 4, 0x1000_1000 ^ seed);
        let u = DATA + 0xb200;
        c.bus.write32(g + 0x10d8, u);
        c.bus.write16(u + 0x238, 0x0020);
        let vt = DATA + 0x7100;
        c.bus.write16(vt + 0x20, 0x0010);
        c.bus.write32(vt + 0x24, FAKE + 0x10);
        c.bus.write32(u + 0x23c, vt);
        for n in 0..8u32 {
            c.bus
                .write32(crate::difftest::fakes::LOG_RESULTS + 4 * n, seed * 3 + 1);
        }
        c.pc = 0x002d_0008;
        item
    }

    #[test]
    fn menu_sys_color_swatch_item_draw_matches() {
        check(
            ee::s09::f_002d0008__generated,
            menu_sys_color_swatch_item_draw,
            60,
            &|c, seed| {
                swatch_draw_fixture(c, seed, seed % 2 == 0);
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_draw_resumes() {
        // Resume heads (call returns): both sides run the generated body
        // there by design (the entry-only guard); pin the deferral.
        for head in [0x002d_0030u32, 0x002d_0298] {
            for seed in [1u32, 2] {
                let mut a = crate::difftest::fresh();
                machine(&mut a, seed, 0x2d00_08, 0);
                swatch_draw_fixture(&mut a, seed, seed % 2 == 0);
                let mut b = crate::difftest::fresh();
                machine(&mut b, seed, 0x2d00_08, 0);
                swatch_draw_fixture(&mut b, seed, seed % 2 == 0);
                for c in [&mut a, &mut b] {
                    c.pc = head;
                }
                ee::s09::f_002d0008__generated(&mut a);
                menu_sys_color_swatch_item_draw(&mut b);
                assert_eq!(a.g[V0][0], b.g[V0][0], "head {head:#x} seed {seed}");
                assert_eq!(a.pc, b.pc, "head {head:#x} seed {seed}");
            }
        }
    }

    driven!(point2d_draw_generated, ee::s09::f_002d0ef8__generated);

    driven!(point2d_draw_hand, menu_sys_point2d_item_draw);

    /// `gp - 0x854` chain + colour-table reads need the game image and a
    /// planted table.
    pub(crate) fn item_draw_globals(c: &mut Cpu, seed: u32) {
        elf_image(c);
        let g = DATA + 0xb000;
        let h = DATA + 0xb100;
        c.bus.write32(GAME_GP.wrapping_sub(0x854), g);
        c.bus.write32(g + 0xe84, h);
        c.bus.write32(h + 4, 0x1000_1000 ^ seed);
        c.bus.write32(h + 8, 0x2000_2000 ^ seed);
        let u = DATA + 0xb200;
        c.bus.write32(g + 0x10d8, u);
        c.bus.write16(u + 0x238, 0x0020);
        let vt = DATA + 0x7100;
        c.bus.write16(vt + 0x20, 0x0010);
        c.bus.write32(vt + 0x24, FAKE + 0x10);
        c.bus.write32(u + 0x23c, vt);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, seed * 3 + 1);
        }
    }

    /// Point2D-draw fixture: item on a real page with the point fields, a
    /// row buffer, and the draw globals.
    pub(crate) fn point2d_draw_fixture(c: &mut Cpu, seed: u32, selected: bool) {
        let mut r = machine(c, seed, 0x2d0e_f8, 2);
        let _ = &mut r;
        item_draw_globals(c, seed);
        let item = DATA + 0x3000;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xC000_0000 ^ seed ^ w);
        }
        let page = DATA + 0x100;
        c.bus.write32(page, 2);
        c.bus.write32(page + 4, if selected { 0 } else { 1 });
        c.bus.write32(page + 0xc, item);
        c.bus.write32(page + 0x10, DATA + 0x2000);
        c.bus.write32(item, page);
        c.bus.write32(item + 4, 0);
        let vt = DATA + 0x7000;
        c.bus.write16(vt + 0x48, 0x0010);
        c.bus.write32(vt + 0x4c, FAKE + 0x10);
        c.bus.write32(page + 0x12c, vt);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, seed * 3 + 1);
        }
        c.bus.write32(item + 0x14, DATA + 0x4000);
        c.bus.write32(item + 0x18, 2 + seed % 2);
        c.bus.write32(item + 0x1c, 0xFFFF_FFFF);
        c.bus.write32(item + 0x20, 0);
        c.bus.write32(item + 0x24, seed % 3);
        let base = DATA + 0x2000;
        c.bus.write32(item + 0x28, base);
        for w in 0..0x20u32 {
            c.bus.write32(base + 4 * w, 0xB000_0000 ^ seed ^ w);
        }
        c.bus.write32(item + 0x2c, r.edge_f32());
        c.bus.write32(item + 0x30, r.edge_f32());
        c.bus.write32(item + 0x34, r.edge_f32());
        c.bus.write32(item + 0x38, r.edge_f32());
        c.bus.write32(item + 0x3c, seed % 2);
        c.bus.write32(item + 0x40, seed / 2 % 2);
        c.g[A0][0] = sx(item);
        let buf = DATA + 0x5000;
        for w in 0..4u32 {
            c.bus.write32(buf + 4 * w, r.edge_f32());
        }
        c.g[A1][0] = sx(buf);
    }

    #[test]
    fn menu_sys_point2d_item_draw_matches() {
        check(
            ee::s09::f_002d0ef8__generated,
            menu_sys_point2d_item_draw,
            40,
            &|c, seed| {
                point2d_draw_fixture(c, seed, seed % 2 == 0);
            },
        );
    }

    #[test]
    fn menu_sys_point2d_item_draw_events() {
        check_events(point2d_draw_generated, point2d_draw_hand, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2d0e_f8, 0);
            let _ = &mut r;
            point2d_draw_fixture(c, seed, seed % 2 == 0);
            // Skip the `+0x40` printf arm here (its nested printf-core
            // dispatcher leaves `pc` at its own resume head on unwind,
            // which `drive` cannot re-enter); matches covers that arm.
            c.bus.write32(DATA + 0x3000 + 0x40, 0);
            // No fake unwinds: re-entry goes through `drive`.
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002d_0ef8)
        });
    }

    driven!(colorchannel_draw_generated, ee::s09::f_002cf1c8__generated);

    driven!(colorchannel_draw_hand, menu_sys_color_channel_item_draw);

    /// Colour-channel draw fixture: item on a real page with the channel
    /// fields and a row buffer.
    pub(crate) fn color_channel_draw_fixture(c: &mut Cpu, seed: u32, selected: bool) {
        let mut r = machine(c, seed, 0x2cf1_c8, 2);
        let _ = &mut r;
        item_draw_globals(c, seed);
        let item = DATA + 0x3000;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xC000_0000 ^ seed ^ w);
        }
        let page = DATA + 0x100;
        c.bus.write32(page, 2);
        c.bus.write32(page + 4, if selected { 0 } else { 1 });
        c.bus.write32(page + 0xc, item);
        c.bus.write32(page + 0x10, DATA + 0x2000);
        let parent = DATA + 0x1100;
        c.bus.write32(item, parent);
        c.bus.write32(item + 4, 0);
        let vt = DATA + 0x7000;
        c.bus.write16(vt + 0x48, 0x0010);
        c.bus.write32(vt + 0x4c, FAKE + 0x10);
        c.bus.write32(parent + 0x12c, vt);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, seed * 3 + 1);
        }
        c.bus.write32(item + 0x14, DATA + 0x4000);
        c.bus.write32(item + 0x18, 1 + seed % 3);
        c.g[A0][0] = sx(item);
        let buf = DATA + 0x5000;
        for w in 0..4u32 {
            c.bus.write32(buf + 4 * w, r.edge_f32());
        }
        c.g[A1][0] = sx(buf);
    }

    #[test]
    fn menu_sys_color_channel_item_draw_matches() {
        check(
            ee::s09::f_002cf1c8__generated,
            menu_sys_color_channel_item_draw,
            40,
            &|c, seed| {
                color_channel_draw_fixture(c, seed, seed % 2 == 0);
            },
        );
    }

    #[test]
    fn menu_sys_color_channel_item_draw_events() {
        check_events(
            colorchannel_draw_generated,
            colorchannel_draw_hand,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cf1_c8, 0);
                let _ = &mut r;
                color_channel_draw_fixture(c, seed, seed % 2 == 0);
                // No fake unwinds: re-entry goes through `drive`.
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                schedule(c, &mut r, seed, 0x002c_f1c8)
            },
        );
    }

    driven!(page_draw_generated, ee::s09::f_002cad58__generated);

    driven!(page_draw_hand, menu_sys_page_draw);

    driven!(draw_frame_generated, ee::s09::f_002cb498__generated);

    driven!(draw_frame_hand, menu_sys_page_draw_frame);

    driven!(text_draw_generated, ee::s09::f_002ce910__generated);

    driven!(text_draw_hand, menu_sys_text_edit_item_draw);

    driven!(text_handle_generated, ee::s09::f_002ce488__generated);

    driven!(text_handle_hand, menu_sys_text_edit_item_handle_input);

    /// Page fixture: count `n`, a selected index, and `n` items.
    pub(crate) fn page_fixture(c: &mut Cpu, seed: u32, n: u32) -> u32 {
        let page = DATA + 0x100;
        c.bus.write32(page, n);
        c.bus.write32(page + 4, seed % (n + 1));
        for i in 0..n {
            let item = DATA + 0x2000 + i * 0x40;
            c.bus.write32(page + 0xc + 4 * i, item);
            for w in 0..8u32 {
                c.bus
                    .write32(item + 4 * w, 0xA100_0000 ^ seed ^ (i * 8 + w));
            }
        }
        page
    }

    /// Gated fake for `InputBinding_EvalFloat` (0x320bf0): ids 0x3e-0x41
    /// return the steered axis floats; slow (0x42) and fine (0x43) return
    /// `f1` (not held) or a value that cannot equal `f1` (held).
    pub(crate) const AXIS_GATE: u32 = DATA + 0x720;

    pub(crate) const AXIS_MAGIC: u32 = 0xA715_0001;

    pub(crate) static AXIS_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn four_way_axis_fake(c: &mut Cpu) {
        if c.bus.read32(AXIS_GATE) != AXIS_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &AXIS_PREV,
                ee::s10::f_00320bf0__generated,
            );
        }
        c.f[0] = match c.g[A1][0] as u32 {
            0x3e => c.bus.read32(DATA + 0x728),
            0x3f => c.bus.read32(DATA + 0x724),
            0x40 => c.bus.read32(DATA + 0x73c),
            0x41 => c.bus.read32(DATA + 0x740),
            0x42 => {
                if c.bus.read32(DATA + 0x72c) != 0 {
                    if c.f[1] == 0 {
                        1.0f32.to_bits()
                    } else {
                        0
                    }
                } else {
                    c.f[1]
                }
            }
            0x43 => {
                if c.bus.read32(DATA + 0x730) != 0 {
                    if c.f[1] == 0 {
                        1.0f32.to_bits()
                    } else {
                        0
                    }
                } else {
                    c.f[1]
                }
            }
            _ => 0x7FC0_0002,
        };
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn install_four_way_axis_fake() {
        crate::difftest::fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            four_way_axis_fake,
            &AXIS_PREV,
        );
    }

    /// Shared menu/input fixture: `item -> parent -> input`, the input code
    /// at `input + 0x48`, keycode/ch at `+0x4c/+0x50`, the axis fake with
    /// steered floats, and the `gp` thresholds. Returns the item.
    pub(crate) fn four_way_input_fixture(
        c: &mut Cpu,
        seed: u32,
        code: u32,
        keycode: u32,
        keychar: u32,
        right: u32,
        left: u32,
        up: u32,
        down: u32,
        slow_held: bool,
        fine_held: bool,
        disabled: bool,
    ) -> u32 {
        install_four_way_axis_fake();
        let item = DATA + 0x100;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xE000_0000 ^ seed ^ w);
        }
        let parent = DATA + 0x1000;
        c.bus.write32(item, parent);
        let input = DATA + 0x1100;
        c.bus.write32(parent + 0x124, input);
        c.bus.write32(input + 0x48, code);
        c.bus.write32(input + 0x4c, keycode);
        c.bus.write32(input + 0x50, keychar);
        c.bus.write32(item + 0x5c, DATA + 0x1200);
        c.g[A0][0] = sx(item);
        c.bus.write32(AXIS_GATE, AXIS_MAGIC);
        c.bus.write32(DATA + 0x724, right);
        c.bus.write32(DATA + 0x728, left);
        c.bus.write32(DATA + 0x73c, up);
        c.bus.write32(DATA + 0x740, down);
        c.bus.write32(DATA + 0x72c, if slow_held { 1 } else { 0 });
        c.bus.write32(DATA + 0x730, if fine_held { 1 } else { 0 });
        c.bus.write32(item + 4, if disabled { 8 } else { 0 });
        for (off, v) in [
            (0x3ff0u32, 0x3FC0_0000u32),
            (0x3fec, 0x4000_0000),
            (0x3fe8, 0x3F80_0000),
            (0x3fe4, 0x3FC0_0000),
            (0x3fe0, 0x4000_0000),
            (0x3fdc, 0x3F80_0000),
            (0x3fd8, 0x3FC0_0000),
            (0x3fd4, 0x4000_0000),
            (0x3fd0, 0x3F80_0000),
            (0x3fcc, 0x4000_0000),
            (0x3fa0, 0x3F80_0000),
            (0x3f9c, 0x4000_0000),
        ] {
            c.bus.write32(GAME_GP.wrapping_sub(off), v);
        }
        item
    }

    /// Redraw fixture: the item's parent vtable with a fake method.
    pub(crate) fn redraw_fixture(c: &mut Cpu, parent: u32, seed: u32) {
        let vt = DATA + 0x7000 + (seed % 4) * 0x80;
        c.bus.write16(vt + 0x20, 0x0010);
        c.bus.write32(vt + 0x24, FAKE + 0x10);
        c.bus.write32(parent + 0x12c, vt);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, seed * 3 + 1);
        }
    }

    /// Deflection pairs: none, positive, negative, hard either way, or
    /// moderate (no step).
    pub(crate) fn deflect_pair(seed: u32) -> (u32, u32) {
        match seed / 4 % 6 {
            0 => (0x0000_0000, 0x0000_0000),
            1 => (0x3F80_0000, 0x0000_0000),
            2 => (0x0000_0000, 0x3F80_0000),
            3 => (0x4120_0000, 0x0000_0000),
            4 => (0x0000_0000, 0xC120_0000),
            _ => (0x3FE0_0000, 0x0000_0000),
        }
    }

    /// `gp - 0x854` chain + colour-table reads need the game image and a
    /// planted table: shared by the draw tests.
    pub(crate) fn page_draw_globals(c: &mut Cpu, seed: u32) {
        elf_image(c);
        let g = DATA + 0xb000;
        let h = DATA + 0xb100;
        c.bus.write32(GAME_GP.wrapping_sub(0x854), g);
        c.bus.write32(g + 0xe84, h);
        c.bus.write32(h + 4, 0x1000_1000 ^ seed);
        let u = DATA + 0xb200;
        c.bus.write32(g + 0x10d8, u);
        c.bus.write16(u + 0x238, 0x0020);
        let vt = DATA + 0x7100;
        c.bus.write16(vt + 0x20, 0x0010);
        c.bus.write32(vt + 0x24, FAKE + 0x10);
        c.bus.write32(u + 0x23c, vt);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, seed * 3 + 1);
        }
    }

    #[test]
    fn menu_sys_point2d_item_handle_input_matches() {
        check(
            ee::s09::f_002d0908__generated,
            menu_sys_point2d_item_handle_input,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d09_08, 2);
                let _ = &mut r;
                let (ud_pos, ud_neg) = deflect_pair(seed);
                let (lr_pos, lr_neg) = deflect_pair(seed ^ 0x9e37);
                // Codes: action (6), arrows (4/5/7/8), or no input.
                let code = [6u32, 4, 5, 7, 8, 2][(seed % 6) as usize];
                let item = four_way_input_fixture(
                    c,
                    seed,
                    code,
                    0,
                    0,
                    lr_pos,
                    lr_neg,
                    ud_pos,
                    ud_neg,
                    seed / 6 % 2 == 0,
                    seed / 12 % 2 == 0,
                    seed / 24 % 4 == 0,
                );
                // Axis (0) or button-step (1) mode.
                c.bus.write32(item + 0x40, seed / 96 % 2);
                c.bus.write32(item + 0x18, 2 + seed % 2);
                c.bus.write32(item + 0x1c, 0xFFFF_FFFF);
                c.bus.write32(item + 0x20, 0);
                c.bus.write32(item + 0x24, seed / 192 % 3);
                let base = DATA + 0x2000;
                c.bus.write32(item + 0x28, base);
                for w in 0..0x20u32 {
                    c.bus.write32(base + 4 * w, 0xB000_0000 ^ seed ^ w);
                }
                c.bus.write32(item + 0x2c, r.edge_f32());
                c.bus.write32(item + 0x30, r.edge_f32());
                c.bus.write32(item + 0x34, r.edge_f32());
                c.bus.write32(item + 0x38, r.edge_f32());
                c.bus.write32(item + 0x3c, seed / 384 % 2);
                let parent = DATA + 0x1000;
                redraw_fixture(c, parent, seed);
                c.bus.write32(item, parent);
            },
        );
    }

    /// Page-draw fixture: `n` items with fake draw methods (`+0x10` vtable,
    /// delta `+0x20`, slot `+0x24`), visibility steered per item.
    pub(crate) fn page_draw_fixture(c: &mut Cpu, seed: u32, n: u32) -> u32 {
        let page = page_fixture(c, seed, n);
        for i in 0..n {
            let item = DATA + 0x2000 + i * 0x40;
            let vt = DATA + 0x7000 + i * 0x80;
            c.bus.write16(vt + 0x20, 0x0010);
            c.bus.write32(vt + 0x24, FAKE + 0x10);
            c.bus.write32(item + 0x10, vt);
            // Visible or not, independently per item.
            c.bus.write32(item + 4, (seed >> i) & 1);
            for w in 0..8u32 {
                c.bus.write32(LOG_RESULTS + 4 * w, seed * 3 + w + i);
            }
        }
        c.g[A0][0] = sx(page);
        page
    }

    #[test]
    fn menu_sys_page_draw_matches() {
        check(
            ee::s09::f_002cad58__generated,
            menu_sys_page_draw,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cad_58, 2);
                let _ = &mut r;
                page_draw_globals(c, seed);
                page_draw_fixture(c, seed, 1 + seed % 3);
            },
        );
    }

    #[test]
    fn menu_sys_page_draw_events() {
        check_events(page_draw_generated, page_draw_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2cad_58, 0);
            let _ = &mut r;
            page_draw_globals(c, seed);
            page_draw_fixture(c, seed, 1 + seed % 3);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002c_ad58)
        });
    }

    #[test]
    fn menu_sys_page_draw_frame_matches() {
        check(
            ee::s09::f_002cb498__generated,
            menu_sys_page_draw_frame,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cb4_98, 1);
                let _ = &mut r;
                let page = page_fixture(c, seed, 2);
                c.g[A0][0] = sx(page);
                let extra = DATA + 0x3000;
                for w in 0..4u32 {
                    c.bus.write32(extra + 4 * w, r.edge_f32());
                }
                c.g[A1][0] = sx(extra);
                let more = DATA + 0x3100;
                for w in 0..4u32 {
                    c.bus.write32(more + 4 * w, r.edge_f32());
                }
                c.g[A2][0] = sx(more);
            },
        );
    }

    #[test]
    fn menu_sys_page_draw_frame_events() {
        check_events(draw_frame_generated, draw_frame_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2cb4_98, 0);
            let _ = &mut r;
            let page = page_fixture(c, seed, 2);
            c.g[A0][0] = sx(page);
            let extra = DATA + 0x3000;
            for w in 0..4u32 {
                c.bus.write32(extra + 4 * w, r.edge_f32());
            }
            c.g[A1][0] = sx(extra);
            let more = DATA + 0x3100;
            for w in 0..4u32 {
                c.bus.write32(more + 4 * w, r.edge_f32());
            }
            c.g[A2][0] = sx(more);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002c_b498)
        });
    }

    /// Text-edit fixture: item on a real page with `len` text bytes and a
    /// steered cursor; the input chain steers keys and codes.
    pub(crate) fn text_edit_fixture(
        c: &mut Cpu,
        seed: u32,
        machine_calls: u32,
        code: u32,
        keycode: u32,
        keychar: u32,
        len: u32,
        cursor: u32,
    ) -> u32 {
        let mut r = machine(c, seed, 0x2ce4_88, machine_calls);
        let _ = &mut r;
        let item = DATA + 0x100;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xD000_0000 ^ seed ^ w);
        }
        let text = item + 0x18;
        for i in 0..len {
            c.bus.write8(text + i, b'k' + (i % 10) as u8);
        }
        c.bus.write8(text + len, 0);
        c.bus.write32(item + 0x28, cursor);
        let parent = DATA + 0x1000;
        c.bus.write32(item, parent);
        let input = DATA + 0x1100;
        c.bus.write32(parent + 0x124, input);
        c.bus.write32(input + 0x48, code);
        c.bus.write32(input + 0x4c, keycode);
        c.bus.write32(input + 0x50, keychar);
        c.g[A0][0] = sx(item);
        item
    }

    #[test]
    fn menu_sys_text_edit_item_handle_input_matches() {
        check(
            ee::s09::f_002ce488__generated,
            menu_sys_text_edit_item_handle_input,
            60,
            &|c, seed| {
                // Nav keys, edit keys, and typing (backspace vs insert).
                let (code, keycode, keychar) = match seed % 8 {
                    0 => (2, 0, 0),
                    1 => (4, 0, 0),
                    2 => (6, 0, 0),
                    3 => (8, 0, 0),
                    4 => (0x11, 8, 0),
                    5 => (0x11, 0x29, b'a' as u32),
                    6 => (0x11, 0x29, b'!' as u32),
                    _ => (0x11, 0x29, 0x7FC0_0002),
                };
                // Lengths around the 14/15/16 bounds; cursor around the end.
                let len = [0u32, 5, 13, 14, 15, 20][(seed / 8 % 6) as usize];
                let cursor = match seed / 48 % 4 {
                    0 => 0,
                    1 => len,
                    2 => len + 1,
                    _ => len.saturating_sub(1),
                };
                text_edit_fixture(c, seed, 2, code, keycode, keychar, len, cursor);
            },
        );
    }

    #[test]
    fn menu_sys_text_edit_item_handle_input_events() {
        check_events(text_handle_generated, text_handle_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2ce4_88, 0);
            let _ = &mut r;
            let (code, keycode, keychar) = match seed % 4 {
                0 => (2, 0, 0),
                1 => (4, 0, 0),
                2 => (0x11, 8, 0),
                _ => (0x11, 0x29, b'm' as u32),
            };
            let len = 5 + seed % 12;
            text_edit_fixture(c, seed, 0, code, keycode, keychar, len, len);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002c_e488)
        });
    }

    /// Text-draw fixture: like the handle fixture plus a real page, the
    /// selected bit, and the row buffer; `long` steers the >= 15 path.
    pub(crate) fn text_edit_draw_fixture(c: &mut Cpu, seed: u32, long: bool, selected: bool) {
        elf_image(c);
        let mut r = machine(c, seed, 0x2ce9_10, 2);
        let _ = &mut r;
        let item = DATA + 0x100;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xD000_0000 ^ seed ^ w);
        }
        let text = item + 0x18;
        let len = if long { 15 + seed % 6 } else { seed % 14 };
        for i in 0..len {
            c.bus.write8(text + i, b'k' + (i % 10) as u8);
        }
        c.bus.write8(text + len, 0);
        c.bus.write32(item + 0x28, len);
        if seed % 2 == 0 {
            c.bus.write32(item + 0x14, 0);
        } else {
            c.bus.write32(item + 0x14, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000, 0x5454_5454);
        }
        let page = DATA + 0x4000;
        c.bus.write32(page, 2);
        c.bus.write32(page + 4, if selected { 0 } else { 1 });
        c.bus.write32(page + 0xc, item);
        c.bus.write32(page + 0x10, DATA + 0x2000);
        c.bus.write32(item, page);
        c.bus.write32(item + 4, 0);
        c.g[A0][0] = sx(item);
        let buf = DATA + 0x5000;
        for w in 0..4u32 {
            c.bus.write32(buf + 4 * w, r.edge_f32());
        }
        c.g[A1][0] = sx(buf);
    }

    #[test]
    fn menu_sys_text_edit_item_draw_matches() {
        check(
            ee::s09::f_002ce910__generated,
            menu_sys_text_edit_item_draw,
            60,
            &|c, seed| {
                page_draw_globals(c, seed);
                text_edit_draw_fixture(c, seed, seed % 2 == 0, seed / 2 % 2 == 0);
            },
        );
    }

    #[test]
    fn menu_sys_text_edit_item_draw_events() {
        check_events(text_draw_generated, text_draw_hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2ce9_10, 0);
            let _ = &mut r;
            page_draw_globals(c, seed);
            text_edit_draw_fixture(c, seed, seed % 2 == 0, seed / 2 % 2 == 0);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002c_e910)
        });
    }

    /// Measure-row fixture: row buffer out, item with a page, a descriptor
    /// table (`[0]` = page), and the font chain; `desc`/`extra`/`text`
    /// steer the three skip arms.
    pub(crate) fn measure_row_fixture(c: &mut Cpu, seed: u32, desc: bool, extra: bool, text: bool) {
        elf_image(c);
        let mut r = machine(c, seed, 0x2ca4_c8, 1);
        let _ = &mut r;
        let row = DATA + 0x6000;
        for w in 0..8u32 {
            c.bus.write32(row + 4 * w, 0xBEEF_0000 ^ seed ^ w);
        }
        c.bus.write32(row + 8, DATA + 0x6100);
        c.bus.write32(DATA + 0x6100, 0x4E4E_4E4E);
        let page = DATA + 0x100;
        c.bus.write32(page, 2);
        c.bus.write32(page + 4, seed % 3);
        c.bus.write32(page + 0xc, DATA + 0x2000);
        c.bus.write32(page + 0x10, DATA + 0x2040);
        let font = DATA + 0x8000;
        c.bus.write32(font + 0x14, 0x3F80_0000);
        c.bus.write32(font + 0x30, 0x4000_0000);
        c.bus.write32(font + 0x34, 0x4040_0000);
        c.bus.write32(font + 0x38, r.edge_f32());
        c.bus.write32(font + 0x3c, r.edge_f32());
        c.bus.write32(page + 0x124, font);
        let item = DATA + 0x3000;
        for w in 0..0x10u32 {
            c.bus.write32(item + 4 * w, 0xF000_0000 ^ seed ^ w);
        }
        c.bus.write32(item, page);
        let table = DATA + 0x4000;
        let count = seed % 4;
        c.bus.write32(item + 0x18, count);
        c.bus.write32(item + 0x14, table);
        for i in 0..=count {
            c.bus.write32(table + 4 * i, r.edge_f32());
        }
        let buf = DATA + 0x7000;
        for w in 0..8u32 {
            c.bus.write32(buf + 4 * w, 0x5151_5100 ^ seed ^ w);
        }
        c.g[A0][0] = sx(row);
        c.g[A1][0] = sx(item);
        c.g[A2][0] = sx(if desc { table } else { 0 });
        c.g[A3][0] = sx(if extra { 1 } else { 0 });
        c.g[T0][0] = sx(if text { buf } else { 0 });
    }

    #[test]
    fn menu_sys_item_measure_row_matches() {
        check(
            ee::s09::f_002ca4c8__generated,
            menu_sys_item_measure_row,
            60,
            &|c, seed| {
                measure_row_fixture(c, seed, seed % 2 == 0, seed / 2 % 2 == 0, seed / 4 % 2 == 0);
            },
        );
    }

    /// Colour-channel measure fixture: row out, item with count + a float
    /// table, and the font chain; `elf_image` for the format strings.
    pub(crate) fn color_channel_measure_fixture(c: &mut Cpu, seed: u32, count: u32) {
        elf_image(c);
        let mut r = machine(c, seed, 0x2cee_e8, 1);
        let _ = &mut r;
        let row = DATA + 0x6000;
        for w in 0..8u32 {
            c.bus.write32(row + 4 * w, 0xBEEF_0000 ^ seed ^ w);
        }
        let page = DATA + 0x100;
        c.bus.write32(page, 2);
        c.bus.write32(page + 4, seed % 3);
        let font = DATA + 0x8000;
        c.bus.write32(font + 0x14, 0x3F80_0000);
        c.bus.write32(font + 0x30, 0x4000_0000);
        c.bus.write32(font + 0x34, 0x4040_0000);
        c.bus.write32(font + 0x38, r.edge_f32());
        c.bus.write32(font + 0x3c, r.edge_f32());
        c.bus.write32(page + 0x124, font);
        let item = DATA + 0x3000;
        for w in 0..0x10u32 {
            c.bus.write32(item + 4 * w, 0xF000_0000 ^ seed ^ w);
        }
        c.bus.write32(item, page);
        let table = DATA + 0x4000;
        c.bus.write32(item + 0x18, count);
        c.bus.write32(item + 0x14, table);
        for i in 0..=count {
            c.bus.write32(table + 4 * i, r.edge_f32());
        }
        c.g[A0][0] = sx(row);
        c.g[A1][0] = sx(item);
    }

    #[test]
    fn menu_sys_color_channel_item_measure_matches() {
        check(
            ee::s09::f_002ceee8__generated,
            menu_sys_color_channel_item_measure,
            60,
            &|c, seed| {
                color_channel_measure_fixture(c, seed, seed % 4);
            },
        );
    }

    /// World-list file fakes: the read returns our buffer; the free is a
    /// no-op (the buffer is test RAM, not heap); `AddWorldIfMissing` only
    /// records its token and returns a steered found flag (its list
    /// mutation is that function's own unit, and its file/path callees
    /// need filesystem state).
    pub(crate) const WORLD_GATE: u32 = DATA + 0x750;

    pub(crate) const WORLD_MAGIC: u32 = 0xA901_D00D;

    pub(crate) const WORLD_RET: u32 = DATA + 0x764;

    pub(crate) static WORLD_PREV_READ: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static WORLD_PREV_FREE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static WORLD_PREV_ADD: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn world_list_read_fake(c: &mut Cpu) {
        if c.bus.read32(WORLD_GATE) != WORLD_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &WORLD_PREV_READ,
                ee::s14::f_003e1a70__generated,
            );
        }
        let buf = DATA + 0x3000;
        let len = c.bus.read32(DATA + 0x760);
        c.bus.write32(c.g[A1][0] as u32, len);
        c.g[V0][0] = sx(buf);
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn world_list_free_fake(c: &mut Cpu) {
        if c.bus.read32(WORLD_GATE) != WORLD_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &WORLD_PREV_FREE,
                ee::s06::f_002523a8__generated,
            );
        }
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn world_list_add_fake(c: &mut Cpu) {
        if c.bus.read32(WORLD_GATE) != WORLD_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &WORLD_PREV_ADD,
                ee::s02::f_0017a830__generated,
            );
        }
        c.bus.write32(DATA + 0x768, c.g[A1][0] as u32);
        c.g[V0][0] = sx(c.bus.read32(WORLD_RET));
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn install_world_list_fakes() {
        crate::difftest::fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1A70),
            world_list_read_fake,
            &WORLD_PREV_READ,
        );
        crate::difftest::fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            world_list_free_fake,
            &WORLD_PREV_FREE,
        );
        crate::difftest::fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0017A830),
            world_list_add_fake,
            &WORLD_PREV_ADD,
        );
    }

    /// World fixture: the list struct plus a text buffer the gated
    /// file-read fake returns. Entries steer quotes/whitespace/endings.
    pub(crate) fn world_list_fixture(c: &mut Cpu, seed: u32, style: u32) -> u32 {
        // The allocator runtime: `AddWorldIfMissing` allocates through it.
        let mut r = crate::difftest::fakes::machine_rt(c, seed, 0x0017_a378, 0);
        let _ = &mut r;
        install_world_list_fakes();
        let list = DATA + 0x100;
        for w in 0..0x20u32 {
            c.bus.write32(list + 4 * w, 0);
        }
        c.bus.write32(list, seed % 3);
        let buf = DATA + 0x3000;
        let text: &[u8] = match style % 4 {
            0 => b"  \"alpine\"  'megaplex'  ",
            1 => b"\n\t\"antoine\"\n\"garibaldi\"\n",
            2 => b"\"snow:jam\" 'see\\nmore'  ",
            _ => b"  peak3  ",
        };
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(buf + i as u32, *b);
        }
        c.bus.write8(buf + text.len() as u32, 0);
        // Length excludes the NUL: the scan ends at `buf + len`, and a
        // trailing NUL inside the bound reprocesses forever.
        c.bus.write32(DATA + 0x760, text.len() as u32);
        // Steer the faked `AddWorldIfMissing` found flag (both arms).
        c.bus.write32(WORLD_RET, seed % 2);
        c.bus.write32(WORLD_GATE, WORLD_MAGIC);
        c.g[A0][0] = sx(list);
        c.g[A1][0] = sx(DATA + 0x2000);
        list
    }

    driven!(enum_handle_generated, ee::s09::f_002cc8f0__generated);

    driven!(enum_handle_hand, menu_sys_enum_item_handle_input);

    driven!(arrow_down_generated, ee::s09::f_002cb8c8__generated);

    driven!(arrow_down_hand, menu_sys_page_draw_scroll_arrow_down);

    driven!(arrow_up_generated, ee::s09::f_002cbac8__generated);

    driven!(arrow_up_hand, menu_sys_page_draw_scroll_arrow_up);

    pub(crate) const AXIS_RIGHT: u32 = DATA + 0x724;

    pub(crate) const AXIS_LEFT: u32 = DATA + 0x728;

    pub(crate) const SLOW_WORD: u32 = DATA + 0x72c;

    pub(crate) const FINE_WORD: u32 = DATA + 0x730;

    pub(crate) fn lr_axis_fake(c: &mut Cpu) {
        if c.bus.read32(AXIS_GATE) != AXIS_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &AXIS_PREV,
                ee::s10::f_00320bf0__generated,
            );
        }
        c.f[0] = match c.g[A1][0] as u32 {
            0x3f => c.bus.read32(AXIS_RIGHT),
            0x3e => c.bus.read32(AXIS_LEFT),
            0x42 => {
                let held = c.bus.read32(SLOW_WORD) != 0;
                if held {
                    if c.f[1] == 0 {
                        1.0f32.to_bits()
                    } else {
                        0
                    }
                } else {
                    c.f[1]
                }
            }
            0x43 => {
                let held = c.bus.read32(FINE_WORD) != 0;
                if held {
                    if c.f[1] == 0 {
                        1.0f32.to_bits()
                    } else {
                        0
                    }
                } else {
                    c.f[1]
                }
            }
            _ => 0x7FC0_0002,
        };
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn install_lr_axis_fake() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            lr_axis_fake,
            &AXIS_PREV,
        );
    }

    /// Shared input fixture: item with a menu chain (`item -> parent ->
    /// input`, the input code at `input + 0x48`), the axis fake, and the
    /// `gp`-relative thresholds. Returns the item.
    pub(crate) fn lr_input_fixture(
        c: &mut Cpu,
        seed: u32,
        code: u32,
        right: u32,
        left: u32,
        slow_held: bool,
        fine_held: bool,
        disabled: bool,
    ) -> u32 {
        install_lr_axis_fake();
        let item = DATA + 0x100;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xE000_0000 ^ seed ^ w);
        }
        let parent = DATA + 0x1000;
        c.bus.write32(item, parent);
        let input = DATA + 0x1100;
        c.bus.write32(parent + 0x124, input);
        c.bus.write32(input + 0x48, code);
        c.bus.write32(item + 0x5c, DATA + 0x1200);
        c.g[A0][0] = sx(item);
        c.bus.write32(AXIS_GATE, AXIS_MAGIC);
        c.bus.write32(AXIS_RIGHT, right);
        c.bus.write32(AXIS_LEFT, left);
        c.bus.write32(SLOW_WORD, if slow_held { 1 } else { 0 });
        c.bus.write32(FINE_WORD, if fine_held { 1 } else { 0 });
        c.bus.write32(item + 4, if disabled { 8 } else { 0 });
        for (off, v) in [
            (0x3ff0u32, 0x3FC0_0000u32),
            (0x3fec, 0x4000_0000),
            (0x3fe8, 0x3F80_0000),
            (0x3fe4, 0x3FC0_0000),
            (0x3fe0, 0x4000_0000),
            (0x3fdc, 0x3F80_0000),
            (0x3fd8, 0x3FC0_0000),
            (0x3fd4, 0x4000_0000),
            (0x3fd0, 0x3F80_0000),
            (0x3fcc, 0x4000_0000),
        ] {
            c.bus.write32(GAME_GP.wrapping_sub(off), v);
        }
        item
    }

    #[test]
    fn menu_sys_int8_item_handle_input_matches() {
        check(
            ee::s09::f_002cd5a0__generated,
            menu_sys_int8_item_handle_input,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd5_a0, 2);
                let _ = &mut r;
                let (right, left) = deflect_pair(seed);
                let (code, disabled) = match seed % 4 {
                    0 => (2, true),
                    1 => (8, false),
                    _ => (2, false),
                };
                let item = lr_input_fixture(
                    c,
                    seed,
                    code,
                    right,
                    left,
                    seed / 24 % 2 == 0,
                    seed / 48 % 2 == 0,
                    disabled,
                );
                // Byte value with a steered below/inside/above position.
                let value = DATA + 0x1500;
                c.bus.write32(item + 0x18, value);
                c.bus.write8(value, [3u8, 7, 12][(seed / 96 % 3) as usize]);
                c.bus.write8(item + 0x1c, 5);
                c.bus.write8(item + 0x1d, 10);
                c.bus.write32(item + 0x20, seed / 192 % 2);
                c.bus.write32(item + 0x24, (seed / 24 % 3) as u32);
                redraw_fixture(c, DATA + 0x1000, seed);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_handle_input_matches() {
        check(
            ee::s09::f_002cd9d8__generated,
            menu_sys_int_item_handle_input,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd9_d8, 2);
                let _ = &mut r;
                let (right, left) = deflect_pair(seed);
                let (code, disabled) = match seed % 4 {
                    0 => (2, true),
                    1 => (8, false),
                    _ => (2, false),
                };
                let item = lr_input_fixture(
                    c,
                    seed,
                    code,
                    right,
                    left,
                    seed / 24 % 2 == 0,
                    seed / 48 % 2 == 0,
                    disabled,
                );
                let value = DATA + 0x1500;
                c.bus.write32(item + 0x18, value);
                c.bus
                    .write32(value, [3u32, 7, 12][(seed / 96 % 3) as usize]);
                c.bus.write32(item + 0x1c, 5);
                c.bus.write32(item + 0x20, 10);
                c.bus.write32(item + 0x24, seed / 192 % 2);
                c.bus.write32(item + 0x28, (seed / 24 % 3) as u32);
                redraw_fixture(c, DATA + 0x1000, seed);
            },
        );
    }

    #[test]
    fn menu_sys_float_item_handle_input_matches() {
        check(
            ee::s09::f_002cdd78__generated,
            menu_sys_float_item_handle_input,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cdd_78, 2);
                let _ = &mut r;
                let (right, left) = deflect_pair(seed);
                let (code, disabled) = match seed % 4 {
                    0 => (2, true),
                    1 => (8, false),
                    _ => (2, false),
                };
                let item = lr_input_fixture(
                    c,
                    seed,
                    code,
                    right,
                    left,
                    seed / 24 % 2 == 0,
                    seed / 48 % 2 == 0,
                    disabled,
                );
                let value = DATA + 0x1500;
                c.bus.write32(item + 0x18, value);
                c.bus.write32(value, r.edge_f32());
                c.bus.write32(item + 0x1c, 0x3F80_0000);
                c.bus.write32(item + 0x20, 0x4120_0000);
                c.bus.write32(item + 0x24, r.edge_f32());
                c.bus.write32(item + 0x28, r.edge_f32());
                c.bus.write32(item + 0x2c, seed / 192 % 2);
                c.bus.write32(item + 0x30, (seed / 24 % 3) as u32);
                redraw_fixture(c, DATA + 0x1000, seed);
            },
        );
    }

    /// Draw-text fixture: item on a real page, three float params, the text
    /// slots steered (`s4`, `s3`, `s5` zero or not), T2's 16 bytes.
    pub(crate) fn draw_row_text_fixture(c: &mut Cpu, seed: u32, s4: u32, s3: u32, s5: u32) {
        elf_image(c);
        let mut r = machine(c, seed, 0x2ca7_68, 3);
        let _ = &mut r;
        let page = DATA + 0x100;
        c.bus.write32(page, 2);
        c.bus.write32(page + 4, seed % 3);
        for i in 0..2u32 {
            let sub = DATA + 0x2000 + i * 0x40;
            c.bus.write32(page + 0xc + 4 * i, sub);
            for w in 0..8u32 {
                c.bus.write32(sub + 4 * w, 0xA100_0000 ^ seed ^ (i * 8 + w));
            }
        }
        let item = DATA + 0x2000;
        c.bus.write32(item, page);
        c.bus.write32(item + 4, seed % 3);
        c.g[A0][0] = sx(item);
        let p1 = DATA + 0x3000;
        c.bus.write32(p1, r.edge_f32());
        c.bus.write32(p1 + 4, r.edge_f32());
        c.bus.write32(p1 + 8, r.edge_f32());
        c.g[A1][0] = sx(p1);
        c.g[A2][0] = sx(s4);
        c.g[A3][0] = sx(s5);
        c.g[T0][0] = sx(s3);
        let t2 = DATA + 0x4000;
        for w in 0..4u32 {
            c.bus.write32(t2 + 4 * w, 0x5151_5100 ^ seed ^ w);
        }
        c.g[T2][0] = sx(t2);
    }

    #[test]
    fn menu_sys_item_draw_row_text_matches() {
        check(
            ee::s09::f_002ca768__generated,
            menu_sys_item_draw_row_text,
            60,
            &|c, seed| {
                // Each optional text arm independently on or off.
                draw_row_text_fixture(
                    c,
                    seed,
                    if seed % 2 == 0 { 0 } else { DATA + 0x5000 },
                    if seed / 2 % 2 == 0 { 0 } else { DATA + 0x5100 },
                    if seed / 4 % 2 == 0 { 0 } else { DATA + 0x5200 },
                );
            },
        );
    }

    /// Enum fixture: `+0x18` count, `+0x1c` entries (`{name, value}` pairs),
    /// `+0x20` the current-value holder (holds 0). The scan reads
    /// `entries - 4` then `entries + 8k - 4` per iteration and compares
    /// against the holder:
    /// - `want == 0`: `entries - 4` points at a zero word → match at `S1=0`.
    /// - `want == 1`: a miss at `S1=0`, entry 0's value slot points so the
    ///   `S1=1` read hits the zero holder → match at `S1=1`.
    /// `code == 0` re-selects entry 0 first, so it only pairs with `want=0`
    /// and a zero entry-0 value.
    pub(crate) fn enum_scan_fixture(c: &mut Cpu, seed: u32, code: u32, want: u32) -> u32 {
        let mut r = machine(c, seed, 0x2cc8_f0, 2);
        let _ = &mut r;
        let item = DATA + 0x100;
        for w in 0..0x20u32 {
            c.bus.write32(item + 4 * w, 0xE000_0000 ^ seed ^ w);
        }
        let count = 2 + seed % 3;
        c.bus.write32(item + 0x18, count);
        let entries = DATA + 0x4000;
        c.bus.write32(item + 0x1c, entries);
        for i in 0..count {
            c.bus.write32(entries + 8 * i, DATA + 0x5000 + i * 8);
            c.bus.write32(entries + 8 * i + 4, 0x100 + i);
            c.bus.write32(DATA + 0x5000 + i * 8, 0x4E4E_4E00 + i);
        }
        let holder = DATA + 0x6500;
        c.bus.write32(item + 0x20, holder);
        c.bus.write32(holder, 0);
        if want == 0 {
            c.bus.write32(entries - 4, DATA + 0x6000);
            c.bus.write32(DATA + 0x6004, 0);
            if code == 0 {
                c.bus.write32(entries + 4, 0);
            }
        } else {
            c.bus.write32(entries - 4, DATA + 0x6000);
            c.bus.write32(DATA + 0x6004, 0xDEAD_0000 ^ seed);
            c.bus.write32(entries + 4, DATA + 0x60F4);
            c.bus.write32(DATA + 0x6100, 0);
        }
        let parent = DATA + 0x1000;
        c.bus.write32(item, parent);
        let input = DATA + 0x1100;
        c.bus.write32(parent + 0x124, input);
        c.bus.write32(input + 0x48, code);
        c.bus.write32(item + 0x5c, DATA + 0x1200);
        redraw_fixture(c, parent, seed);
        c.bus.write32(item + 4, 0);
        c.g[A0][0] = sx(item);
        item
    }

    #[test]
    fn menu_sys_enum_item_handle_input_matches() {
        check(
            ee::s09::f_002cc8f0__generated,
            menu_sys_enum_item_handle_input,
            60,
            &|c, seed| {
                let code = [0u32, 1, 2, 3, 4, 6, 0x11][(seed % 7) as usize];
                // `code == 0` re-selects entry 0: only with `want = 0`.
                let want = if code == 0 { 0 } else { seed / 7 % 2 };
                enum_scan_fixture(c, seed, code, want);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_handle_input_events() {
        check_events(enum_handle_generated, enum_handle_hand, 60, &|c, seed| {
            // No fake unwinds: re-entry goes through `drive`. Miss at
            // `S1=0`, match at `S1=1`: the back edge always runs.
            let mut r = machine(c, seed, 0x2cc8_f0, 0);
            let _ = &mut r;
            enum_scan_fixture(c, seed, [1u32, 2, 3, 4, 6, 0x11][(seed % 6) as usize], 1);
            // No fake unwinds: re-entry goes through `drive`.
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002c_c8f0)
        });
    }

    /// Scroll-arrow fixture: item with a row page, four floats, the
    /// `+0x12c` vtable (delta `+0x40`, slot `+0x44`) and the `gp - 0x854`
    /// chain the tails walk. `elf_image` for the colour table.
    pub(crate) fn scroll_arrow_fixture(c: &mut Cpu, seed: u32) -> u32 {
        elf_image(c);
        let mut r = machine(c, seed, 0x2cb8_c8, 2);
        let _ = &mut r;
        let item = DATA + 0x3000;
        for w in 0..0x10u32 {
            c.bus.write32(item + 4 * w, 0xD000_0000 ^ seed ^ w);
        }
        let page = DATA + 0x100;
        c.bus.write32(page, 2);
        c.bus.write32(page + 4, seed % 3);
        c.bus.write32(item, page);
        c.bus.write32(item + 4, seed % 3);
        c.bus.write32(page + 0xc, item);
        let vt = DATA + 0x7000;
        c.bus.write16(vt + 0x40, 0x0020);
        c.bus.write32(vt + 0x44, FAKE + 0x10);
        c.bus.write32(item + 0x12c, vt);
        for n in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, seed * 3 + 1);
        }
        let floats = DATA + 0x6000;
        c.bus.write32(floats, r.edge_f32());
        c.bus.write32(floats + 4, r.edge_f32());
        c.bus.write32(floats + 8, r.edge_f32());
        c.bus.write32(floats + 0xc, r.edge_f32());
        c.g[A0][0] = sx(item);
        c.g[A1][0] = sx(floats);
        let g = DATA + 0xb000;
        let h = DATA + 0xb100;
        c.bus.write32(GAME_GP.wrapping_sub(0x854), g);
        c.bus.write32(g + 0xe84, h);
        c.bus.write32(h + 4, 0x1000_1000 ^ seed);
        let u = DATA + 0xb200;
        c.bus.write32(g + 0x10d8, u);
        c.bus.write16(u + 0x238, 0x0020);
        let vt2 = DATA + 0x7100;
        c.bus.write16(vt2 + 0x20, 0x0010);
        c.bus.write32(vt2 + 0x24, FAKE + 0x10);
        c.bus.write32(u + 0x23c, vt2);
        item
    }

    #[test]
    fn menu_sys_page_draw_scroll_arrow_down_matches() {
        check(
            ee::s09::f_002cb8c8__generated,
            menu_sys_page_draw_scroll_arrow_down,
            60,
            &|c, seed| {
                scroll_arrow_fixture(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_page_draw_scroll_arrow_down_events() {
        check_events(arrow_down_generated, arrow_down_hand, 60, &|c, seed| {
            // No fake unwinds: re-entry goes through `drive`.
            let mut r = machine(c, seed, 0x2cb8_c8, 0);
            let _ = &mut r;
            scroll_arrow_fixture(c, seed);
            // No fake unwinds: re-entry goes through `drive`.
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002c_b8c8)
        });
    }

    #[test]
    fn menu_sys_page_draw_scroll_arrow_up_matches() {
        check(
            ee::s09::f_002cbac8__generated,
            menu_sys_page_draw_scroll_arrow_up,
            60,
            &|c, seed| {
                scroll_arrow_fixture(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_page_draw_scroll_arrow_up_events() {
        check_events(arrow_up_generated, arrow_up_hand, 60, &|c, seed| {
            // No fake unwinds: re-entry goes through `drive`.
            let mut r = machine(c, seed, 0x2cb8_c8, 0);
            let _ = &mut r;
            scroll_arrow_fixture(c, seed);
            // No fake unwinds: re-entry goes through `drive`.
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            schedule(c, &mut r, seed, 0x002c_bac8)
        });
    }

    /// Gated recorders for the base destructors both families call. Gate off:
    /// the installed bodies. Gate on: log `(a0, a1)` per call and return.
    pub(crate) const DTOR_GATE: u32 = DATA + 0x700;

    pub(crate) const DTOR_MAGIC: u32 = 0xD70A0001;

    pub(crate) const DTOR_ITEM_LOG: u32 = DATA + 0x710;

    pub(crate) const DTOR_PAGE_LOG: u32 = DATA + 0x780;

    pub(crate) static ITEM_DTOR_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static PAGE_DTOR_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn menu_sys_item_dtor_fake(c: &mut Cpu) {
        if c.bus.read32(DTOR_GATE) != DTOR_MAGIC {
            return fakes::fall_back(c, &ITEM_DTOR_PREV, ee::s09::f_002ca280__generated);
        }
        let n = c.bus.read32(DTOR_ITEM_LOG);
        c.bus.write32(DTOR_ITEM_LOG, n + 1);
        c.bus.write32(DTOR_ITEM_LOG + 8 + n * 8, c.g[A0][0] as u32);
        c.bus.write32(DTOR_ITEM_LOG + 12 + n * 8, c.g[A1][0] as u32);
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn menu_sys_page_dtor_fake(c: &mut Cpu) {
        if c.bus.read32(DTOR_GATE) != DTOR_MAGIC {
            return fakes::fall_back(c, &PAGE_DTOR_PREV, ee::s09::f_002caa80__generated);
        }
        let n = c.bus.read32(DTOR_PAGE_LOG);
        c.bus.write32(DTOR_PAGE_LOG, n + 1);
        c.bus.write32(DTOR_PAGE_LOG + 8 + n * 8, c.g[A0][0] as u32);
        c.bus.write32(DTOR_PAGE_LOG + 12 + n * 8, c.g[A1][0] as u32);
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn menu_sys_install_dtor_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            menu_sys_item_dtor_fake,
            &ITEM_DTOR_PREV,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA80),
            menu_sys_page_dtor_fake,
            &PAGE_DTOR_PREV,
        );
    }

    /// Dtor fixture: gated recorders on, `a0` the object, `a1` a distinctive
    /// flag; logs start empty.
    pub(crate) fn menu_sys_dtor_fixture(c: &mut Cpu, seed: u32, flag: u64) {
        menu_sys_install_dtor_fakes();
        c.bus.write32(DTOR_GATE, DTOR_MAGIC);
        c.bus.write32(DTOR_ITEM_LOG, 0);
        c.bus.write32(DTOR_PAGE_LOG, 0);
        c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
        c.g[A1][0] = flag;
    }

    #[test]
    fn menu_sys_bool_item_dtor_matches() {
        check(
            ee::s09::f_002d3a68__generated,
            menu_sys_bool_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3a_68, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_dtor_matches() {
        check(
            ee::s09::f_002d3a98__generated,
            menu_sys_enum_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3a_98, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_submenu_item_dtor_matches() {
        check(
            ee::s09::f_002d3ae0__generated,
            menu_sys_submenu_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3a_e0, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_back_item_dtor_matches() {
        check(
            ee::s09::f_002d3b10__generated,
            menu_sys_back_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3b_10, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_action_item_dtor_matches() {
        check(
            ee::s09::f_002d3b38__generated,
            menu_sys_action_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3b_38, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_callback_item_dtor_matches() {
        check(
            ee::s09::f_002d3b80__generated,
            menu_sys_callback_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3b_80, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_label_item_dtor_matches() {
        check(
            ee::s09::f_002d3bb0__generated,
            menu_sys_label_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3b_b0, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_spacer_item_dtor_matches() {
        check(
            ee::s09::f_002d3be0__generated,
            menu_sys_spacer_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3b_e0, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_dtor_matches() {
        check(
            ee::s09::f_002d3c00__generated,
            menu_sys_int8_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3c_00, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_int_item_dtor_matches() {
        check(
            ee::s09::f_002d3c30__generated,
            menu_sys_int_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3c_30, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_float_item_dtor_matches() {
        check(
            ee::s09::f_002d3c60__generated,
            menu_sys_float_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3c_60, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_dtor_matches() {
        check(
            ee::s09::f_002d3ce0__generated,
            menu_sys_float_range_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3c_e0, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_text_edit_item_dtor_matches() {
        check(
            ee::s09::f_002d3d20__generated,
            menu_sys_text_edit_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3d_20, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_color_channel_item_dtor_matches() {
        check(
            ee::s09::f_002d3d70__generated,
            menu_sys_color_channel_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3d_70, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_group_item_page_dtor_matches() {
        check(
            ee::s09::f_002d3d90__generated,
            menu_sys_group_item_page_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3d_90, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_dtor_matches() {
        check(
            ee::s09::f_002d3df8__generated,
            menu_sys_color_swatch_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3d_f8, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_vector_label_item_dtor_matches() {
        check(
            ee::s09::f_002d3ea8__generated,
            menu_sys_vector_label_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3e_a8, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_point2_d_item_dtor_matches() {
        check(
            ee::s09::f_002d3f50__generated,
            menu_sys_point2_d_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3f_50, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    #[test]
    fn menu_sys_group_item_dtor_matches() {
        check(
            ee::s09::f_002d3db0__generated,
            menu_sys_group_item_dtor,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d3d_b0, 1);
                menu_sys_dtor_fixture(c, seed, sx(seed as u32 * 3 + 1));
            },
        );
    }

    driven!(
        menu_sys_group_item_ctor_generated,
        ee::s09::f_002cf7e8__generated
    );

    driven!(menu_sys_group_item_ctor_hand, menu_sys_group_item_ctor);

    /// Pre-fill `words` words at `addr` with nonzero seed-distinctive data.
    pub(crate) fn menu_sys_fill_nonzero(c: &mut Cpu, addr: u32, words: u32, seed: u32) {
        for i in 0..words {
            c.bus.write32(
                addr + 4 * i,
                0xA000_0000 ^ seed.wrapping_mul(0x9E37_79B1).wrapping_add(i * 0x0100_0001),
            );
        }
    }

    /// Ctor fixture: `a0` the object (pre-filled nonzero), the argument
    /// registers seeded distinctive.
    pub(crate) fn menu_sys_ctor_fixture(c: &mut Cpu, r: &mut Rng, obj: u32, seed: u32) {
        menu_sys_fill_nonzero(c, obj, 0x40, seed);
        c.g[A0][0] = sx(obj);
        c.g[A1][0] = sx(0xB000_0000 ^ seed * 3);
        c.g[A2][0] = sx(0xC000_0000 ^ seed * 5);
        c.g[A3][0] = sx(0xD000_0000 ^ seed * 7);
        c.g[T0][0] = sx(0xE000_0000 ^ seed * 11);
        c.g[T1][0] = sx(0xF000_0000 ^ seed * 13);
        c.g[T2][0] = sx(0x0100_0000 ^ seed * 17);
        c.g[T3][0] = sx(0x0200_0000 ^ seed * 19);
        c.f[12] = r.edge_f32();
        c.f[13] = r.edge_f32();
        let _ = r.next_u32();
    }

    #[test]
    fn menu_sys_ctor_matches() {
        check(
            ee::s09::f_002cc018__generated,
            menu_sys_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc0_18, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_ctor_2_matches() {
        check(
            ee::s09::f_002cc7f8__generated,
            menu_sys_enum_item_ctor_2,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc7_f8, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_callback_item_ctor_matches() {
        check(
            ee::s09::f_002cd128__generated,
            menu_sys_callback_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd1_28, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_ctor_matches() {
        check(
            ee::s09::f_002cc7b0__generated,
            menu_sys_enum_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc7_b0, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_action_item_ctor_2_matches() {
        check(
            ee::s09::f_002ccf50__generated,
            menu_sys_action_item_ctor_2,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ccf_50, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_label_item_ctor_matches() {
        check(
            ee::s09::f_002cd2a0__generated,
            menu_sys_label_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd2_a0, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_spacer_item_ctor_matches() {
        check(
            ee::s09::f_002cd350__generated,
            menu_sys_spacer_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd3_50, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd450__generated,
            menu_sys_int8_item_ctor_2,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd4_50, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd8a8__generated,
            menu_sys_int_item_ctor_2,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd8_a8, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_callback_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd168__generated,
            menu_sys_callback_item_ctor_2,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd1_68, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_matches() {
        check(
            ee::s09::f_002cd3f8__generated,
            menu_sys_int8_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd3_f8, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_ctor_matches() {
        check(
            ee::s09::f_002cd850__generated,
            menu_sys_int_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd8_50, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_vector_label_item_ctor_matches() {
        check(
            ee::s09::f_002d03f0__generated,
            menu_sys_vector_label_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d03_f0, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_float_item_ctor_matches() {
        check(
            ee::s09::f_002cdc88__generated,
            menu_sys_float_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cdc_88, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_ctor_matches() {
        check(
            ee::s09::f_002cf7e8__generated,
            menu_sys_group_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cf7_e8, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x2000, seed);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_ctor_events() {
        check_events(
            menu_sys_group_item_ctor_generated,
            menu_sys_group_item_ctor_hand,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cf7_e8, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x2000, seed);
                schedule(c, &mut r, seed, 0x002c_f7e8)
            },
        );
    }

    #[test]
    fn menu_sys_group_item_ctor_2_matches() {
        check(
            ee::s09::f_002cf860__generated,
            menu_sys_group_item_ctor_2,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cf8_60, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x2000, seed);
                // Null head skips the append; any other head appends it.
                if seed % 2 == 0 {
                    c.g[A2][0] = 0;
                }
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_ctor_matches() {
        check(
            ee::s09::f_002ce170__generated,
            menu_sys_float_range_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ce1_70, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_3_matches() {
        check(
            ee::s09::f_002cd4a0__generated,
            menu_sys_int8_item_ctor_3,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cd4_a0, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    /// Composite fixture: object big enough for the member structs, plus
    /// text for the message-page label.
    pub(crate) fn menu_sys_composite_fixture(c: &mut Cpu, r: &mut Rng, obj: u32, seed: u32) {
        menu_sys_fill_nonzero(c, obj, 0x200, seed);
        c.g[A0][0] = sx(obj);
        c.g[A1][0] = sx(DATA + 0x3000 + (seed % 4) * 0x40);
        c.g[A2][0] = sx(0xC000_0000 ^ seed * 5);
        c.g[A3][0] = sx(0xD000_0000 ^ seed * 7);
        c.g[T0][0] = sx(0xE000_0000 ^ seed * 11);
        c.g[T1][0] = sx(0xF000_0000 ^ seed * 13);
        c.g[T2][0] = sx(obj + 0x800);
        c.g[T3][0] = sx(obj + 0x900);
        c.f[12] = r.edge_f32();
        c.f[13] = r.edge_f32();
        // Label text.
        let text = DATA + 0x3000 + (seed % 4) * 0x40;
        menu_sys_fill_nonzero(c, text, 0x10, seed ^ 0x5555);
        let _ = r.next_u32();
    }

    #[test]
    fn menu_sys_point2_d_item_ctor_matches() {
        check(
            ee::s09::f_002d0808__generated,
            menu_sys_point2_d_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d08_08, 4);
                menu_sys_composite_fixture(c, &mut r, DATA + 0x2000, seed);
                // Two 8-byte points the ctor copies.
                menu_sys_fill_nonzero(c, DATA + 0x2800, 8, seed ^ 0x1111);
                menu_sys_fill_nonzero(c, DATA + 0x2900, 8, seed ^ 0x2222);
                c.g[T2][0] = sx(DATA + 0x2800);
                c.g[T3][0] = sx(DATA + 0x2900);
            },
        );
    }

    #[test]
    fn menu_sys_bool_item_ctor_matches() {
        check(
            ee::s09::f_002cc500__generated,
            menu_sys_bool_item_ctor,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc5_00, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_ctor_3_matches() {
        check(
            ee::s09::f_002cc838__generated,
            menu_sys_enum_item_ctor_3,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc8_38, 1);
                menu_sys_ctor_fixture(c, &mut r, DATA + 0x100, seed);
            },
        );
    }

    /// Gated fake for `InputBinding_EvalFloat` (0x320bf0), driven by the
    /// modifier tests below. Gate off: the body the slot held. Gate on: the
    /// forced f32, so both compare arms run.
    pub(crate) const EVAL_GATE: u32 = DATA + 0x700;

    pub(crate) const EVAL_FORCE: u32 = DATA + 0x704;

    pub(crate) const EVAL_WANT_A1: u32 = DATA + 0x708;

    pub(crate) const EVAL_MAGIC: u32 = 0xE1A70001;

    pub(crate) static EVAL_PREV: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn menu_sys_eval_float_fake(c: &mut Cpu) {
        if c.bus.read32(EVAL_GATE) != EVAL_MAGIC {
            return fakes::fall_back(c, &EVAL_PREV, ee::s10::f_00320bf0__generated);
        }
        // A caller passing the wrong axis/modifier id gets a sentinel, so a
        // mistranscribed id fails the test instead of matching.
        if c.g[A1][0] as u32 == c.bus.read32(EVAL_WANT_A1) {
            c.f[0] = c.bus.read32(EVAL_FORCE);
        } else {
            c.f[0] = 0x7FC0_0001;
        }
        let t = c.g[RA][0] as u32;
        c.pc = t;
    }

    pub(crate) fn menu_sys_install_eval_fake() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            menu_sys_eval_float_fake,
            &EVAL_PREV,
        );
    }

    #[test]
    fn menu_sys_item_on_focus_matches() {
        check(
            ee::s09::f_002ca368__generated,
            menu_sys_item_on_focus,
            50,
            &|c, seed| {
                let mut r = Rng(0xa368 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_item_on_blur_matches() {
        check(
            ee::s09::f_002ca370__generated,
            menu_sys_item_on_blur,
            50,
            &|c, seed| {
                let mut r = Rng(0xa370 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_page_on_action_matches() {
        check(
            ee::s09::f_002cad48__generated,
            menu_sys_page_on_action,
            50,
            &|c, seed| {
                let mut r = Rng(0xad48 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_page_v04_matches() {
        check(
            ee::s09::f_002cad50__generated,
            menu_sys_page_v04,
            50,
            &|c, seed| {
                let mut r = Rng(0xad50 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_set_active_matches() {
        check(
            ee::s09::f_002cc0b8__generated,
            menu_sys_set_active,
            50,
            &|c, seed| {
                let mut r = Rng(0xc0b8 ^ seed as u64);
                scramble(c, &mut r);
                let sys = DATA + 0x100;
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(if seed % 2 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(sys + 0x54, 0xDEAD_BEEF ^ seed);
            },
        );
    }

    #[test]
    fn menu_sys_get_input_code_matches() {
        check(
            ee::s09::f_002cc248__generated,
            menu_sys_get_input_code,
            50,
            &|c, seed| {
                let mut r = Rng(0xc248 ^ seed as u64);
                scramble(c, &mut r);
                let sys = DATA + 0x100;
                c.g[A0][0] = sx(sys);
                c.bus.write32(sys + 0x48, 0xA11CE ^ seed);
            },
        );
    }

    #[test]
    fn menu_sys_get_key_code_matches() {
        check(
            ee::s09::f_002cc250__generated,
            menu_sys_get_key_code,
            50,
            &|c, seed| {
                let mut r = Rng(0xc250 ^ seed as u64);
                scramble(c, &mut r);
                let sys = DATA + 0x100;
                c.g[A0][0] = sx(sys);
                c.bus.write32(sys + 0x4c, 0xB0BAC ^ (seed * 7));
            },
        );
    }

    #[test]
    fn menu_sys_get_key_char_matches() {
        check(
            ee::s09::f_002cc258__generated,
            menu_sys_get_key_char,
            50,
            &|c, seed| {
                let mut r = Rng(0xc258 ^ seed as u64);
                scramble(c, &mut r);
                let sys = DATA + 0x100;
                c.g[A0][0] = sx(sys);
                c.bus.write32(sys + 0x50, 0xC4A2 ^ (seed * 13));
            },
        );
    }

    #[test]
    fn menu_sys_label_item_is_selectable_matches() {
        check(
            ee::s09::f_002cd2e8__generated,
            menu_sys_label_item_is_selectable,
            50,
            &|c, seed| {
                let mut r = Rng(0xd2e8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_spacer_item_is_selectable_matches() {
        check(
            ee::s09::f_002cd398__generated,
            menu_sys_spacer_item_is_selectable,
            50,
            &|c, seed| {
                let mut r = Rng(0xd398 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_is_selectable_matches() {
        check(
            ee::s09::f_002cff80__generated,
            menu_sys_color_swatch_item_is_selectable,
            50,
            &|c, seed| {
                let mut r = Rng(0xff80 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_vector_label_item_is_selectable_matches() {
        check(
            ee::s09::f_002d0448__generated,
            menu_sys_vector_label_item_is_selectable,
            50,
            &|c, seed| {
                let mut r = Rng(0x0448 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    #[test]
    fn menu_sys_item_reset_value_matches() {
        check(
            ee::s09::f_002d39f8__generated,
            menu_sys_item_reset_value,
            50,
            &|c, seed| {
                let mut r = Rng(0x39f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 8);
            },
        );
    }

    /// Item at `item`, whose `+0` points at `obj`, whose `+0x124` is the menu.
    pub(crate) fn menu_sys_item_menu_fixture(c: &mut Cpu, item: u32, seed: u32) -> u32 {
        let obj = DATA + 0x200 + (seed % 4) * 0x20;
        let menu = DATA + 0x300;
        c.bus.write32(item, obj | 0x11);
        c.bus.write32(obj + 0x124, menu | 0x22);
        c.bus.write32(menu + 0x5c, DATA + 0x400 + (seed % 3) * 0x10);
        c.bus.write32(DATA + 0x400, 0x0102_0304 ^ seed);
        c.bus.write32(DATA + 0x404, 0x0506_0708 ^ seed);
        c.bus.write32(DATA + 0x408, 0x090A_0B0C ^ seed);
        c.bus.write32(DATA + 0x40c, 0x0D0E_0F10 ^ seed);
        c.g[A0][0] = sx(item);
        menu
    }

    #[test]
    fn menu_sys_item_get_left_axis_matches() {
        check(
            ee::s09::f_002ca408__generated,
            menu_sys_item_get_left_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca4_08, 2);
                menu_sys_item_menu_fixture(c, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_get_right_axis_matches() {
        check(
            ee::s09::f_002ca428__generated,
            menu_sys_item_get_right_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca4_28, 2);
                menu_sys_item_menu_fixture(c, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_get_up_axis_matches() {
        check(
            ee::s09::f_002ca448__generated,
            menu_sys_item_get_up_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca4_48, 2);
                menu_sys_item_menu_fixture(c, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_get_down_axis_matches() {
        check(
            ee::s09::f_002ca468__generated,
            menu_sys_item_get_down_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca4_68, 2);
                menu_sys_item_menu_fixture(c, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_is_slow_mod_held_matches() {
        check(
            ee::s09::f_002ca488__generated,
            menu_sys_item_is_slow_mod_held,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca4_88, 2);
                menu_sys_item_menu_fixture(c, DATA + 0x100, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_is_fine_mod_held_matches() {
        check(
            ee::s09::f_002ca4a8__generated,
            menu_sys_item_is_fine_mod_held,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca4_a8, 2);
                menu_sys_item_menu_fixture(c, DATA + 0x100, seed);
            },
        );
    }

    /// Menu at `menu` whose `+0x5c` binding evaluates through the real
    /// `InputBinding_EvalFloat`.
    pub(crate) fn menu_sys_menu_binding_fixture(c: &mut Cpu, menu: u32, seed: u32) {
        let binding = DATA + 0x400 + (seed % 3) * 0x10;
        c.bus.write32(menu + 0x5c, binding);
        c.bus.write32(binding, 0x1112_1314 ^ seed);
        c.bus.write32(binding + 4, 0x1516_1718 ^ (seed * 3));
        c.bus.write32(binding + 8, 0x191A_1B1C ^ (seed * 5));
        c.bus.write32(binding + 0xc, 0x1D1E_1F20 ^ (seed * 7));
        c.g[A0][0] = sx(menu);
    }

    #[test]
    fn menu_sys_get_left_axis_matches() {
        check(
            ee::s09::f_002cc260__generated,
            menu_sys_get_left_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cc2_60, 2);
                menu_sys_menu_binding_fixture(c, DATA + 0x300, seed);
            },
        );
    }

    #[test]
    fn menu_sys_get_right_axis_matches() {
        check(
            ee::s09::f_002cc280__generated,
            menu_sys_get_right_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cc2_80, 2);
                menu_sys_menu_binding_fixture(c, DATA + 0x300, seed);
            },
        );
    }

    #[test]
    fn menu_sys_get_up_axis_matches() {
        check(
            ee::s09::f_002cc2a0__generated,
            menu_sys_get_up_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cc2_a0, 2);
                menu_sys_menu_binding_fixture(c, DATA + 0x300, seed);
            },
        );
    }

    #[test]
    fn menu_sys_get_down_axis_matches() {
        check(
            ee::s09::f_002cc2c0__generated,
            menu_sys_get_down_axis,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cc2_c0, 2);
                menu_sys_menu_binding_fixture(c, DATA + 0x300, seed);
            },
        );
    }

    #[test]
    fn menu_sys_is_slow_mod_held_matches() {
        check(
            ee::s09::f_002cc2e0__generated,
            menu_sys_is_slow_mod_held,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cc2_e0, 2);
                menu_sys_install_eval_fake();
                let menu = DATA + 0x300;
                c.bus.write32(menu + 0x5c, DATA + 0x400);
                c.g[A0][0] = sx(menu);
                c.bus.write32(EVAL_GATE, EVAL_MAGIC);
                c.bus.write32(EVAL_WANT_A1, 0x42);
                c.bus.write32(
                    EVAL_FORCE,
                    if seed % 2 == 0 {
                        0x0000_0000
                    } else {
                        0x3FC0_0000
                    },
                );
            },
        );
    }

    #[test]
    fn menu_sys_is_fine_mod_held_matches() {
        check(
            ee::s09::f_002cc318__generated,
            menu_sys_is_fine_mod_held,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cc3_18, 2);
                menu_sys_install_eval_fake();
                let menu = DATA + 0x300;
                c.bus.write32(menu + 0x5c, DATA + 0x400);
                c.g[A0][0] = sx(menu);
                c.bus.write32(EVAL_GATE, EVAL_MAGIC);
                c.bus.write32(EVAL_WANT_A1, 0x43);
                c.bus.write32(
                    EVAL_FORCE,
                    if seed % 2 == 0 {
                        0x0000_0000
                    } else {
                        0x3FC0_0000
                    },
                );
            },
        );
    }

    #[test]
    fn menu_sys_group_item_v09_matches() {
        check(
            ee::s09::f_002cfaf8__generated,
            menu_sys_group_item_v09,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cfa_f8, 2);
                let item = DATA + 0x100;
                let vt = DATA + 0x500 + (seed % 3) * 0x40;
                c.bus.write32(item, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x12c, vt);
                c.g[A0][0] = sx(item);
                c.bus.write16(vt + 0x18, ((seed * 0x11 + 0x20) & !3) as u16);
                c.bus.write32(vt + 0x1c, fakes::FAKE + 0x10);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_on_blur_matches() {
        check(
            ee::s09::f_002cfef8__generated,
            menu_sys_group_item_on_blur,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cfef8, 2);
                let item = DATA + 0x100;
                let page = item + 0x18;
                let index = seed % 3;
                c.g[A0][0] = sx(item);
                c.bus.write32(page + 4, index);
                let slot_item = DATA + 0x600 + (seed % 3) * 0x40;
                c.bus.write32(page + 0xc + 4 * index, slot_item);
                let vt = DATA + 0x800;
                c.bus.write32(slot_item + 0x10, vt);
                c.bus.write16(vt + 0x38, 0x0010);
                c.bus.write32(vt + 0x3c, fakes::FAKE + 0x10);
            },
        );
    }
}
