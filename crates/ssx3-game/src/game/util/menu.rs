//! `src/util/menu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::arg;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
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
    (
        0x002c_ad48,
        menu_sys_page_on_action_002cad48,
        "MenuSysPage_OnAction",
    ),
    (0x002c_ad50, menu_sys_page_v04_002cad50, "MenuSysPage_v04"),
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
        menu_sys_label_item_is_selectable_002cd2e8,
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
        menu_sys_spacer_item_is_selectable_002cd398,
        "MenuSysSpacerItem_IsSelectable",
    ),
    (
        0x002c_d3f0,
        menu_sys_spacer_item_draw_002cd3f0,
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
        menu_sys_color_swatch_item_is_selectable_002cff80,
        "MenuSysColorSwatchItem_IsSelectable",
    ),
    (
        0x002d_0008,
        menu_sys_color_swatch_item_draw,
        "MenuSysColorSwatchItem_Draw",
    ),
    (
        0x002d_02b0,
        menu_sys_color_item_ctor,
        "MenuSysColorItem_ctor",
    ),
    (
        0x002d_03d0,
        menu_sys_color_item_handle_input,
        "MenuSysColorItem_HandleInput",
    ),
    (
        0x002d_03f0,
        menu_sys_vector_label_item_ctor,
        "MenuSysVectorLabelItem_ctor",
    ),
    (
        0x002d_0448,
        menu_sys_vector_label_item_is_selectable_002d0448,
        "MenuSysVectorLabelItem_IsSelectable",
    ),
    (
        0x002d_05a8,
        menu_sys_vector_item_ctor,
        "MenuSysVectorItem_ctor",
    ),
    (
        0x002d_06e8,
        menu_sys_vector_item_handle_input,
        "MenuSysVectorItem_HandleInput",
    ),
    (
        0x002d_0808,
        menu_sys_point2d_item_ctor,
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
        0x002d_1740,
        menu_sys_message_page_ctor,
        "MenuSysMessagePage_ctor",
    ),
    (
        0x002d_17f0,
        menu_sys_message_page_ctor_2,
        "MenuSysMessagePage_ctor_2",
    ),
    (0x002d_1cc0, world_create_dead_node, "World_CreateDeadNode"),
    (
        0x002d_39f8,
        menu_sys_item_reset_value_002d39f8,
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
        0x002d_3e20,
        menu_sys_color_item_dtor,
        "MenuSysColorItem_dtor",
    ),
    (
        0x002d_3ea8,
        menu_sys_vector_label_item_dtor,
        "MenuSysVectorLabelItem_dtor",
    ),
    (
        0x002d_3ed0,
        menu_sys_vector_item_dtor,
        "MenuSysVectorItem_dtor",
    ),
    (
        0x002d_3f50,
        menu_sys_point2d_item_dtor,
        "MenuSysPoint2DItem_dtor",
    ),
    (
        0x002d_3f78,
        menu_sys_message_page_dtor,
        "MenuSysMessagePage_dtor",
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

    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
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

/// `MenuSys::MenuSys(MenuSys* this)` (0x2cc018): clear the first word of
/// the menu, then `MenuSys_Reset(this)`; returns `this`.
pub fn menu_sys_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Callee {
            addr: 0x002c_c070,
            f: ee::s09::f_002cc070,
        }
        .call(c, 0x002c_c030)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysItem_OnFocus` (0x002c_a368): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_item_on_focus(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSysItem_OnBlur` (0x002c_a370): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_item_on_blur(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSysPage_OnAction` (0x002c_ad48): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_page_on_action_002cad48(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysPage_v04` (0x002c_ad50): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_page_v04_002cad50(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSys_SetActive` (0x002c_c0b8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn menu_sys_set_active(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[A1][0]);
    ret(c);
}

/// `MenuSys_GetInputCode` (0x002c_c248): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn menu_sys_get_input_code(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `MenuSys_GetKeyCode` (0x002c_c250): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn menu_sys_get_key_code(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
    ret(c);
}

/// `MenuSys_GetKeyChar` (0x002c_c258): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn menu_sys_get_key_char(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `MenuSysLabelItem_IsSelectable` (0x002c_d2e8): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_label_item_is_selectable_002cd2e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysSpacerItem_IsSelectable` (0x002c_d398): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_spacer_item_is_selectable_002cd398(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysSpacerItem_Draw` (0x002c_d3f0): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_spacer_item_draw_002cd3f0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSysColorSwatchItem_IsSelectable` (0x002c_ff80): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_color_swatch_item_is_selectable_002cff80(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysVectorLabelItem_IsSelectable` (0x002d_0448): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_vector_label_item_is_selectable_002d0448(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysItem_ResetValue` (0x002d_39f8): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_item_reset_value_002d39f8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSys_DebugPrintf_4080` (0x002d_4080): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn menu_sys_debug_printf_4080(c: &mut Cpu) {
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
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `void MenuSysColorChannelItem_Draw(Item* self, const float* rect)`
/// (0x2cf1c8; `cColorMenuItem::render` in the On Tour symbols): one colour
/// channel row of the colour editor. Draws the page row, the channel name
/// and its value (`%s`-style text through sprintf), a 4-vertex gradient bar
/// (80 wide, 10 high) from the colour with the channel at 0 to the colour
/// with the channel at 1, and a 3-vertex marker triangle at the channel's
/// current value. The selected row uses the highlight colour (0x4d53a0),
/// the others the plain colour (0x4d53b0).
pub fn menu_sys_color_channel_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x1a0);
        let sp0 = sp(c);
        let s = |o: u32| sp0.wrapping_add(o);
        frame.save(c, 18, 0x150);
        frame.save(c, 19, 0x140);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x198);
        c.g[S3][0] = c.g[A1][0];
        frame.save_fpr(c, 22, 0x190);
        frame.save(c, 17, 0x160);
        frame.save_ra(c, 0x130);
        frame.save_fpr(c, 21, 0x188);
        frame.save_fpr(c, 20, 0x180);
        frame.save(c, 16, 0x170);
        // Page vtable call: vt[9](menu + delta, rect, self == selected item).
        c.g[S0][0] = c.lw(r32(c, S2));
        c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(0x12c));
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = c.lh(r32(c, S1).wrapping_add(0x48));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        call_ptr(c, 0x002c_bf30, 0x002c_f210)?; // MenuSysPage_GetSelectedItem
        c.g[V0][0] = c.g[S2][0] ^ c.g[V0][0];
        c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = (c.g[V0][0] < 1) as u64;
        let method = r32(c, V1);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, method, 0x002c_f228)?;
        // Bar geometry: f22 = rect.x + rect.w - 16 - 80 (bar x), f23 = f22 - 16 (text right edge).
        c.lwc1(0, r32(c, S3));
        c.lwc1(1, r32(c, S3).wrapping_add(0x8));
        c.g[AT][0] = 0x4180_0000;
        c.mtc1(1, 3);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x42a0_0000;
        c.mtc1(1, 2);
        c.g[A0][0] = c.lw(r32(c, S2));
        c.sub_s(0, 0, 3);
        c.sub_s(22, 0, 2);
        c.sub_s(23, 22, 3);
        call_ptr(c, 0x002c_bf30, 0x002c_f258)?; // MenuSysPage_GetSelectedItem
                                                // Highlight colour for the selected row, plain colour otherwise.
        if c.g[S2][0] != c.g[V0][0] {
            c.g[V0][0] = 0x4d_0000;
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x53b0));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4d_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x53a0));
        }
        c.cycles += 128;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[A2][0] = 0x480000;
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7470));
        c.swc1(20, sp0);
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 2);
        c.lwc1(0, c.g[V0][0] as u32);
        c.g[AT][0] = 0x40c00000;
        c.mtc1(1, 21);
        c.swc1(0, s(0x4));
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 3);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.swc1(0, s(0x8));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.swc1(1, s(0xc));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.ldl(2, s(0x7));
        c.ldr(2, sp0);
        c.ldl(4, s(0xf));
        c.ldr(4, s(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.swc1(2, s(0x14));
        c.swc1(2, s(0x10));
        c.ldl(8, s(0x17));
        c.ldr(8, s(0x10));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x2f));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x28));
        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.add_s(13, 13, 21);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.lwc1(12, c.g[S3][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        c.sub_s(13, 13, 20);
        c.add_s(12, 12, 3);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, 0x00391cb0, 0x002cf338)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[S0][0] = 0x480000;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[S1][0] = sx(s(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6710));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x00413af8, 0x002cf35c)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fa4));
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
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
        c.g[S0][0] = sx(s(0x60));
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
        c.sdl(3, s(0x37));
        c.sdr(3, s(0x30));
        c.sdl(4, s(0x3f));
        c.sdr(4, s(0x38));
        c.sub_s(0, 0, 21);
        c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(10, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(10, s(0x47));
        c.sdr(10, s(0x40));
        c.sdl(4, s(0x4f));
        c.sdr(4, s(0x48));
        c.g[V1][0] = sx(sp0.wrapping_add(c.g[V0][0] as u32));
        c.g[AT][0] = 0x42a00000;
        c.mtc1(1, 1);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x30), 0u64);
        c.g[A0][0] = c.g[V1][0];
        c.sub_s(0, 0, 2);
        c.swc1(20, (c.g[A0][0] as u32).wrapping_add(0x40));
        c.swc1(22, s(0x50));
        c.swc1(1, s(0x58));
        c.swc1(0, s(0x54));
        c.swc1(2, s(0x5c));

        // Vertex setup is done; this loop only counts a2 from 3 down to -1.
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            let more = c.g[A2][0] != c.g[A3][0];
            c.lwc1(0, s(0x34));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_f470);
            }
        }
        c.cycles += 85;
        c.g[AT][0] = 0x43000000;
        c.mtc1(1, 5);
        c.g[A3][0] = 0x80;
        c.lwc1(1, s(0x38));
        c.mul_s(0, 0, 5);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 4);
        c.mul_s(1, 1, 5);
        c.lwc1(6, s(0x50));
        c.mtc1(0, 3);
        c.cvt_w_s(9, 0);
        c.mfc1(6, 9);
        c.lwc1(8, s(0x54));
        c.cvt_w_s(0, 1);
        c.mfc1(5, 0);
        c.lwc1(7, s(0x5c));
        c.lwc1(2, s(0x3c));
        c.lwc1(0, s(0x44));
        c.add_s(7, 8, 7);
        c.swc1(6, s(0x120));
        c.mul_s(2, 2, 5);
        c.swc1(8, s(0x124));
        c.mul_s(0, 0, 5);
        c.swc1(3, s(0x128));
        c.swc1(4, s(0x12c));
        c.cvt_w_s(1, 2);
        c.mfc1(4, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(8, 1);
        c.sw(s(0xa4), c.g[A1][0]);
        c.lq(3, s(0x120));
        c.swc1(7, s(0x124));
        c.sq(3, s(0x80));
        c.lq(2, s(0x120));
        c.sw(s(0xa8), c.g[A0][0]);
        c.sq(2, s(0xb0));
        c.sw(s(0x74), c.g[A1][0]);
        c.sw(s(0x78), c.g[A0][0]);
        c.sw(s(0xa0), c.g[A2][0]);
        c.sw(s(0x70), c.g[A2][0]);
        c.sw(s(0x7c), c.g[A3][0]);
        c.swc1(3, s(0x64));
        c.swc1(3, s(0x60));
        c.swc1(4, s(0x68));
        c.sw(s(0xac), c.g[A3][0]);
        c.swc1(3, s(0x94));
        c.swc1(3, s(0x90));
        c.swc1(4, s(0x98));
        c.sw(s(0xd0), c.g[T0][0]);
        c.lwc1(0, s(0x58));
        c.lwc1(2, s(0x48));
        c.add_s(6, 6, 0);
        c.lwc1(1, s(0x4c));
        c.swc1(8, s(0x124));
        c.mul_s(2, 2, 5);
        c.mul_s(1, 1, 5);
        c.sw(s(0x100), c.g[T0][0]);
        c.swc1(6, s(0x120));
        c.cvt_w_s(0, 2);
        c.mfc1(4, 0);
        c.sw(s(0x10c), c.g[A3][0]);
        c.lq(3, s(0x120));
        c.cvt_w_s(0, 1);
        c.mfc1(5, 0);
        c.swc1(7, s(0x124));
        c.sq(3, s(0xe0));
        c.lq(2, s(0x120));
        c.sw(s(0x104), c.g[A0][0]);
        c.sw(s(0x108), c.g[A1][0]);
        c.sw(s(0xd4), c.g[A0][0]);
        c.sw(s(0xd8), c.g[A1][0]);
        c.sw(s(0xdc), c.g[A3][0]);
        c.swc1(3, s(0xc4));
        c.swc1(3, s(0xc0));
        c.swc1(4, s(0xc8));
        c.swc1(3, s(0xf4));
        c.swc1(3, s(0xf0));
        c.swc1(4, s(0xf8));
        c.sq(2, s(0x110));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.lwc1(0, c.g[V0][0] as u32);
        c.c_eq_s(0, 4);

        // Selection state: the item's value at +0x14 equals f4 (1.0): flags 4, else 0x14.
        let fcc = c.fcc();
        c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        let flag = if fcc {
            c.cycles += 5;
            0x4
        } else {
            c.cycles += 6;
            0x14
        };
        c.g[A1][0] = 0xffff_ffff_ffff_ff83;
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0xe84));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
        c.g[V0][0] &= c.g[A1][0];
        c.g[V0][0] |= flag;
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
        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x002cf644)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = c.g[S0][0];
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x3;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.lwc1(0, s(0x58));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.lwc1(4, s(0x50));
        c.lwc1(1, c.g[V1][0] as u32);
        c.g[A3][0] = 0u64;
        c.lwc1(5, s(0x54));
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
        c.lwc1(3, s(0x4));
        c.sub_s(6, 4, 7);
        c.lwc1(1, s(0x8));
        c.lwc1(2, s(0xc));
        c.mul_s(3, 3, 0);
        c.swc1(8, s(0x124));
        c.mul_s(1, 1, 0);
        c.swc1(6, s(0x120));
        c.mul_s(2, 2, 0);
        c.sw(s(0x128), 0u64);
        c.add_s(7, 4, 7);
        c.swc1(9, s(0x12c));
        c.cvt_w_s(0, 3);
        c.mfc1(8, 0);
        c.cvt_w_s(0, 2);
        c.mfc1(4, 0);
        c.lq(3, s(0x120));
        c.cvt_w_s(0, 1);
        c.mfc1(9, 0);
        c.swc1(5, s(0x124));
        c.swc1(4, s(0x120));
        c.sw(s(0x128), 0u64);
        c.sw(s(0xd0), c.g[T0][0]);
        c.lq(2, s(0x120));
        c.swc1(7, s(0x120));
        c.swc1(8, s(0x124));
        c.sw(s(0x128), 0u64);
        c.sq(2, s(0xb0));
        c.sw(s(0x70), c.g[T0][0]);
        c.sw(s(0xa0), c.g[T0][0]);
        c.lq(2, s(0x120));
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sq(3, s(0x80));
        c.sw(s(0xd8), c.g[A0][0]);
        c.sw(s(0x78), c.g[A0][0]);
        c.sw(s(0xa8), c.g[A0][0]);
        c.sw(s(0xd4), c.g[T1][0]);
        c.sw(s(0x74), c.g[T1][0]);
        c.sw(s(0xa4), c.g[T1][0]);
        c.sq(2, s(0xe0));
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x002cf75c)?;

        frame.restore(c, 16, 0x170);
        frame.restore(c, 17, 0x160);
        frame.restore(c, 18, 0x150);
        frame.restore(c, 19, 0x140);
        frame.restore_ra(c, 0x130);
        frame.restore_fpr(c, 23, 0x198);
        frame.restore_fpr(c, 22, 0x190);
        frame.restore_fpr(c, 21, 0x188);
        frame.restore_fpr(c, 20, 0x180);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysItem_MeasureRow(out: *mut [f32;4], item, a2, a3, t0)` (0x2ca4c8): measure
/// one menu row made of up to three strings (`a2` left, `a3` middle, `t0` right) with the
/// text helper `0x391fb0`; writes the 16-byte rect to `*a0` and returns `a0` in `v0`.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
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
                                                    c.g[RA][0] = sx(0x002ca57c);
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
                                                    c.g[RA][0] = sx(0x002ca5a4);
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
                                                    c.g[RA][0] = sx(0x002ca5cc);
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
                                                        let t = true;
                                                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                                        if t {
                                                            break 'b_2ca5ec;
                                                        } else {
                                                            break 'b_2ca5e4;
                                                        }
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
                                            c.g[RA][0] = sx(0x002ca608);
                                            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                            call_ptr(c, 0x00391fb0, 0x002ca608)?;
                                            c.add_s(0, 0, 0);
                                            c.g[AT][0] = 0x42000000;
                                            c.mtc1(1, 1);
                                            let t = true;
                                            c.add_s(0, 21, 0);
                                            if t {
                                                break 'b_2ca6dc;
                                            } else {
                                                break 'b_2ca61c;
                                            }
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
                                    c.g[RA][0] = sx(0x002ca644);
                                    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                                    call_ptr(c, 0x00391fb0, 0x002ca644)?;
                                    let t = true;
                                    c.mov_s(20, 0);
                                    if t {
                                        break 'b_2ca654;
                                    } else {
                                        break 'b_2ca64c;
                                    }
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
                            c.g[RA][0] = sx(0x002ca67c);
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
                            c.g[RA][0] = sx(0x002ca6a8);
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
    })
}

/// `MenuSysItem_DrawRowText` (0x2ca768): draw the up to three strings of a menu row
/// with the text helpers `0x391cb0` / `0x391fb0`.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
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
            c.g[RA][0] = sx(0x002ca7bc);
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
                c.g[RA][0] = sx(0x002ca858);
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
                c.g[RA][0] = sx(0x002ca880);
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
                c.g[RA][0] = sx(0x002ca8cc);
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
                c.g[RA][0] = sx(0x002ca8f4);
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
                c.g[RA][0] = sx(0x002ca940);
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
    })
}

/// `MenuSysPage_Draw` (0x2cad58): draw a menu page: the visible items through their
/// vtable draw slots, then the scroll arrows.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
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
                                        c.g[RA][0] = sx(0x002cadd0);
                                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                        Callee {
                                            addr: 0x002ca378,
                                            f: ssx3_ee::s09::f_002ca378,
                                        }
                                        .call(c, 0x002cadd0)?;
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
                                            c.g[RA][0] = sx(0x002cadf0);
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
                                                let t = true;
                                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                                if t {
                                                    break 'b_2cae4c;
                                                } else {
                                                    break 'b_2cae40;
                                                }
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
                            let t = true;
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                            if t {
                                break 'b_2cafc8;
                            } else {
                                break 'b_2cafa8;
                            }
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
                        c.g[RA][0] = sx(0x002cafd0);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x002cb180,
                            f: ssx3_ee::s09::f_002cb180,
                        }
                        .call(c, 0x002cafd0)?;
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
                                    c.g[RA][0] = sx(0x002caff8);
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    Callee {
                                        addr: 0x002ca378,
                                        f: ssx3_ee::s09::f_002ca378,
                                    }
                                    .call(c, 0x002caff8)?;
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
                                        c.g[RA][0] = sx(0x002cb0b4);
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
                        c.g[RA][0] = sx(0x002cb11c);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x474));
                        Callee {
                            addr: 0x002cb8c8,
                            f: ssx3_ee::s09::f_002cb8c8,
                        }
                        .call(c, 0x002cb11c)?;
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
                        c.g[RA][0] = sx(0x002cb158);
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x47c));
                        Callee {
                            addr: 0x002cbac8,
                            f: ssx3_ee::s09::f_002cbac8,
                        }
                        .call(c, 0x002cb158)?;
                        break 'b_2cb158;
                    }
                }
                c.cycles += 10;
                c.g[RA][0] = sx(0x002cb160);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x002cb2f8,
                    f: ssx3_ee::s09::f_002cb2f8,
                }
                .call(c, 0x002cb160)?;
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
    })
}

/// `MenuSysPage_DrawFrame` (0x2cb498): build the quads of a menu page frame
/// (`MenuSys_BuildQuad`).
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
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
                        c.g[RA][0] = sx(0x002cb67c);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb67c)?;
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
                        c.g[RA][0] = sx(0x002cb6b0);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb6b0)?;
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
                        c.g[RA][0] = sx(0x002cb6e0);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb6e0)?;
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
                        c.g[RA][0] = sx(0x002cb714);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb714)?;
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
                        c.g[RA][0] = sx(0x002cb748);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), 0u64);
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb748)?;
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
                        c.g[RA][0] = sx(0x002cb778);
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb778)?;
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
                        c.g[RA][0] = sx(0x002cb7ac);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb7ac)?;
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
                        c.g[RA][0] = sx(0x002cb7dc);
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xdc));
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb7dc)?;
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
                        c.g[RA][0] = sx(0x002cb80c);
                        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0xd8));
                        Callee {
                            addr: 0x002cb350,
                            f: ssx3_ee::s09::f_002cb350,
                        }
                        .call(c, 0x002cb80c)?;
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
    })
}

/// `void MenuSysPage_DrawScrollArrowDown(MenuSysPage* self, float* rect)`
/// (0x2cb8c8, reference name `cMenu_renderUpArrow`): asks the page's vtable
/// (slot 8) to draw the background, marks the current draw layer, fills six
/// vertices with the arrow colour (global at 0x4d53a0, scaled to 0..128) and
/// positions them as a small triangle at the bottom of `rect` (x, y, w, h),
/// then submits them through the renderer's vtable (+0x238). The generated
/// body is the authority for registers and cycles.
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
        c.g[RA][0] = sx(0x002cb8f4);
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
                        c.g[RA][0] = sx(0x002cbab4);
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

/// `void MenuSysPage_DrawScrollArrowUp(MenuSysPage* self, float* rect)`
/// (0x2cbac8, reference name `cMenu_renderDownArrow`): the mirror of
/// `MenuSysPage_DrawScrollArrowDown`, with the triangle at the top of `rect`.
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
        c.g[RA][0] = sx(0x002cbaf4);
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
                        c.g[RA][0] = sx(0x002cbcb4);
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

/// `int MenuSysEnumItem_HandleInput(MenuSysEnumItem* self)` (0x2cc8f0,
/// reference name `cEnumMenuItem_input`): the base item input first (returns
/// its result when non-zero or the item is disabled); input code 6 selects
/// through the menu's vtable; otherwise it finds the current value among the
/// entries (count at +0x18, `{label, value}` at +0x1c, value pointer at
/// +0x20) and moves to the previous (codes 2/0) or next (codes 3/1) entry
/// with wrap-around, notifying the menu (vtable slot 4) after a change.
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
                c.g[RA][0] = sx(0x002cc918);
                c.g[S0][0] = c.g[A0][0];
                call_ptr(c, 0x002ca2b0, 0x002cc918)?;
                c.g[S5][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002cc924);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002ca3b0, 0x002cc924)?;
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
                                c.g[RA][0] = sx(0x002cc93c);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca3e8, 0x002cc93c)?;
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
                                    c.g[RA][0] = sx(0x002cc968);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x002cc968)?;
                                    let t = true;
                                    c.g[S5][0] = c.g[V0][0];
                                    if t {
                                        break 'b_2ccaec;
                                    } else {
                                        break 'b_2cc970;
                                    }
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
                                                        c.g[RA][0] = sx(0x002cc990);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x002ca3e8, 0x002cc990)?;
                                                        if c.g[V0][0] == c.g[S2][0] {
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_2cc9ac;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x002cc9a0);
                                                            c.g[A0][0] = c.g[S0][0];
                                                            call_ptr(c, 0x002ca3e8, 0x002cc9a0)?;
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
                                                    let t = true;
                                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                                    if t {
                                                        break 'b_2ccacc;
                                                    } else {
                                                        break 'b_2cc9c0;
                                                    }
                                                }
                                                'b_2cc9e4: {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x002cc9c8);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x002ca3e8, 0x002cc9c8)?;
                                                    if c.g[V0][0] == c.g[S3][0] {
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        break 'b_2cc9e4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x002cc9d8);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x002ca3e8, 0x002cc9d8)?;
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
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_sub(0x4));
                                                if t {
                                                    break 'b_2ccac8;
                                                } else {
                                                    break 'b_2cca00;
                                                }
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
                                                    c.g[RA][0] = sx(0x002cca24);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x002ca3e8, 0x002cca24)?;
                                                    if c.g[V0][0] == c.g[S2][0] {
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        break 'b_2cca40;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x002cca34);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x002ca3e8, 0x002cca34)?;
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
                                                let t = true;
                                                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                                                if t {
                                                    break 'b_2ccacc;
                                                } else {
                                                    break 'b_2cca78;
                                                }
                                            }
                                            'b_2cca9c: {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x002cca80);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x002ca3e8, 0x002cca80)?;
                                                if c.g[V0][0] == c.g[S3][0] {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                    break 'b_2cca9c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x002cca90);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x002ca3e8, 0x002cca90)?;
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
                                c.g[RA][0] = sx(0x002ccae4);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002ccae4)?;
                                let t = true;
                                c.g[V0][0] = c.g[S5][0];
                                if t {
                                    break 'b_2ccaf0;
                                } else {
                                    break 'b_2ccaec;
                                }
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

/// `int MenuSysInt8Item_HandleInput(MenuSysInt8Item* self)` (0x2cd5a0): the
/// signed-byte slider. Direction = sign of (right axis - left axis) outside a
/// +-0.1 dead zone; a held direction counts up (+0x20) and accelerates with
/// `n * n / 0x120`, the slow/fine modifiers divide the step by 20/5 (50 both);
/// the float remainder at +0x28 carries the fraction, the byte (pointer at
/// +0x18) is clamped to the limits at +0x1c/+0x1d and the menu is notified.
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
                c.g[RA][0] = sx(0x002cd5c0);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                call_ptr(c, 0x002ca2b0, 0x002cd5c0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002cd5cc);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002ca3b0, 0x002cd5cc)?;
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
                                c.g[RA][0] = sx(0x002cd5e4);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca428, 0x002cd5e4)?;
                                c.mov_s(20, 0);
                                c.g[RA][0] = sx(0x002cd5f0);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca408, 0x002cd5f0)?;
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
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                    if t {
                                        break 'b_2cd65c;
                                    } else {
                                        break 'b_2cd64c;
                                    }
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
                                        c.g[RA][0] = sx(0x002cd6a0);
                                        c.mul_s(20, 20, 0);
                                        call_ptr(c, 0x002ca488, 0x002cd6a0)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cd6c8;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x002cd6b0);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x002ca4a8, 0x002cd6b0)?;
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_2cd6c8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[AT][0] = 0x42480000;
                                                c.mtc1(1, 21);
                                                let t = true;

                                                if t {
                                                    break 'b_2cd700;
                                                } else {
                                                    break 'b_2cd6c8;
                                                }
                                            }
                                        }
                                    }
                                    'b_2cd6e8: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x002cd6d0);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x002ca488, 0x002cd6d0)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cd6e8;
                                        } else {
                                            c.cycles += 4;
                                            c.g[AT][0] = 0x41a00000;
                                            c.mtc1(1, 21);
                                            let t = true;

                                            if t {
                                                break 'b_2cd700;
                                            } else {
                                                break 'b_2cd6e8;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x002cd6f0);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x002ca4a8, 0x002cd6f0)?;
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
                                c.g[RA][0] = sx(0x002cd798);
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

/// `int MenuSysIntItem_HandleInput(MenuSysIntItem* self)` (0x2cd9d8): the
/// same slider for a 32-bit value (limits at +0x1c/+0x20 as words, repeat
/// counter +0x24, direction +0x28, float remainder +0x2c).
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
                c.g[RA][0] = sx(0x002cd9f8);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                call_ptr(c, 0x002ca2b0, 0x002cd9f8)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002cda04);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002ca3b0, 0x002cda04)?;
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
                                c.g[RA][0] = sx(0x002cda1c);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca428, 0x002cda1c)?;
                                c.mov_s(20, 0);
                                c.g[RA][0] = sx(0x002cda28);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca408, 0x002cda28)?;
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
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                    if t {
                                        break 'b_2cda94;
                                    } else {
                                        break 'b_2cda84;
                                    }
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
                                        c.g[RA][0] = sx(0x002cdad8);
                                        c.mul_s(20, 20, 0);
                                        call_ptr(c, 0x002ca488, 0x002cdad8)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cdb00;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x002cdae8);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x002ca4a8, 0x002cdae8)?;
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_2cdb00;
                                            } else {
                                                c.cycles += 4;
                                                c.g[AT][0] = 0x42480000;
                                                c.mtc1(1, 21);
                                                let t = true;

                                                if t {
                                                    break 'b_2cdb38;
                                                } else {
                                                    break 'b_2cdb00;
                                                }
                                            }
                                        }
                                    }
                                    'b_2cdb20: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x002cdb08);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x002ca488, 0x002cdb08)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cdb20;
                                        } else {
                                            c.cycles += 4;
                                            c.g[AT][0] = 0x41a00000;
                                            c.mtc1(1, 21);
                                            let t = true;

                                            if t {
                                                break 'b_2cdb38;
                                            } else {
                                                break 'b_2cdb20;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x002cdb28);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x002ca4a8, 0x002cdb28)?;
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
                                c.g[RA][0] = sx(0x002cdbd0);
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

/// `int MenuSysFloatItem_HandleInput(FloatItem* self)` (0x2cdd78): the
/// float-editing menu row. Runs the base item input (a non-zero result or a
/// disabled item returns at once), then reads the stick (`right - left`,
/// dead zone +-0.1) into a direction -1/0/+1. Holding the same direction
/// counts up `+0x2c` (a new direction resets it, direction kept at `+0x30`);
/// the step grows with the square of the hold count and is divided by 1/5/20/50
/// while the fine/slow modifiers are held. The value `*(+0x18)` (scaled by
/// `+0x24`, offset `+0x28`) is moved by `d * step * (max - min)`, clamped to
/// `[+0x1c, +0x20]` and stored back; then the menu's change callback (vtable
/// `+0x12c`, slot at `+0x20`/`+0x24`) runs with the item.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
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
                c.g[RA][0] = sx(0x002cdd98);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                Callee {
                    addr: 0x002ca2b0,
                    f: ee::s09::MenuSysItem_HandleInput,
                }
                .call(c, 0x002cdd98)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002cdda4);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x002ca3b0,
                    f: ee::s09::MenuSysItem_IsDisabled,
                }
                .call(c, 0x002cdda4)?;
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
                                c.g[RA][0] = sx(0x002cddbc);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002ca428,
                                    f: ee::s09::MenuSysItem_GetRightAxis,
                                }
                                .call(c, 0x002cddbc)?;
                                c.mov_s(20, 0);
                                c.g[RA][0] = sx(0x002cddc8);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002ca408,
                                    f: ee::s09::MenuSysItem_GetLeftAxis,
                                }
                                .call(c, 0x002cddc8)?;
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
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                                    if t {
                                        break 'b_2cde28;
                                    } else {
                                        break 'b_2cde24;
                                    }
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
                                    c.g[RA][0] = sx(0x002cde40);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x002ca488,
                                        f: ee::s09::MenuSysItem_IsSlowModHeld,
                                    }
                                    .call(c, 0x002cde40)?;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_2cde68;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x002cde50);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x002ca4a8,
                                            f: ee::s09::MenuSysItem_IsFineModHeld,
                                        }
                                        .call(c, 0x002cde50)?;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_2cde68;
                                        } else {
                                            c.cycles += 4;
                                            c.g[AT][0] = 0x42480000;
                                            c.mtc1(1, 21);
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                                            if t {
                                                break 'b_2cdea0;
                                            } else {
                                                break 'b_2cde68;
                                            }
                                        }
                                    }
                                }
                                'b_2cde88: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x002cde70);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x002ca488,
                                        f: ee::s09::MenuSysItem_IsSlowModHeld,
                                    }
                                    .call(c, 0x002cde70)?;
                                    let t = c.g[V0][0] == 0u64;

                                    if t {
                                        break 'b_2cde88;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0x41a00000;
                                        c.mtc1(1, 21);
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                                        if t {
                                            break 'b_2cdea0;
                                        } else {
                                            break 'b_2cde88;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x002cde90);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002ca4a8,
                                    f: ee::s09::MenuSysItem_IsFineModHeld,
                                }
                                .call(c, 0x002cde90)?;
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
                            c.g[RA][0] = sx(0x002cdf54);
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

/// `int MenuSysTextEditItem_HandleInput(TextEditItem* self)` (0x2ce488): the
/// name-entry row. After the base input (non-zero result returns), edits the
/// 15 character buffer at `+0x18` with the cursor at `+0x28` from the menu
/// input code: 2/3 move the cursor (wrapping at 16), 4/5 step the character
/// under the cursor down/up until it is a letter, `_` or `!`, 9 inserts a `_`
/// at the cursor (shifting the tail right), 10 deletes at the cursor (shifting
/// left), 6 returns 3 (done), and any other code takes the typed key: code 8
/// is backspace, an accepted character is inserted at the cursor.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
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
            c.g[RA][0] = sx(0x002ce4ac);
            c.g[S1][0] = c.g[A0][0];
            Callee {
                addr: 0x002ca2b0,
                f: ee::s09::MenuSysItem_HandleInput,
            }
            .call(c, 0x002ce4ac)?;
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
                                c.g[RA][0] = sx(0x002ce4c4);
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
                                Callee {
                                    addr: 0x002ca3e8,
                                    f: ee::s09::MenuSysItem_GetInputCode,
                                }
                                .call(c, 0x002ce4e0)?;
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
                                    let t = true;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_2ce520;
                                    } else {
                                        break 'b_2ce508;
                                    }
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
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                        if t {
                            break 'b_2ce7f8;
                        } else {
                            break 'b_2ce530;
                        }
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
                                    let t = true;
                                    c.g[V0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_2ce7fc;
                                    } else {
                                        break 'b_2ce5a4;
                                    }
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
                                    let t = true;
                                    c.g[V0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_2ce7fc;
                                    } else {
                                        break 'b_2ce61c;
                                    }
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
                                let t = true;
                                c.sb(c.g[V0][0] as u32, c.g[V1][0]);
                                if t {
                                    break 'b_2ce7f8;
                                } else {
                                    break 'b_2ce678;
                                }
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
                                    let t = true;
                                    c.sb(c.g[V0][0] as u32, 0u64);
                                    if t {
                                        break 'b_2ce7f8;
                                    } else {
                                        break 'b_2ce6d8;
                                    }
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
                            let t = true;
                            c.g[S4][0] = 0x3;
                            if t {
                                break 'b_2ce7f8;
                            } else {
                                break 'b_2ce6e8;
                            }
                        }
                    }
                    'b_2ce7f4: {
                        'b_2ce768: {
                            c.cycles += 10;
                            c.g[RA][0] = sx(0x002ce6f0);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            Callee {
                                addr: 0x002cc250,
                                f: ee::s09::MenuSys_GetKeyCode,
                            }
                            .call(c, 0x002ce6f0)?;
                            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x002ce700);
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                            Callee {
                                addr: 0x002cc258,
                                f: ee::s09::MenuSys_GetKeyChar,
                            }
                            .call(c, 0x002ce700)?;
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
                                        let t = true;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_2ce7f4;
                                        } else {
                                            break 'b_2ce768;
                                        }
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

/// `void MenuSysTextEditItem_Draw(TextEditItem* self, Row* row)` (0x2ce910):
/// draws the name-entry row: builds the text (with a trailing `_` caret when
/// the item is the page's selected one and the cursor is at the end), draws
/// the row background through the menu's slot `+0x40/+0x44`, the label
/// (`+0x14`) and the text with a drop shadow (four offset copies and the
/// colour quad through the draw context at `gp-0x854`), then restores the
/// font colour.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
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
                c.g[RA][0] = sx(0x002ce94c);
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                Callee {
                    addr: 0x002cbf30,
                    f: ee::s09::MenuSysPage_GetSelectedItem,
                }
                .call(c, 0x002ce94c)?;
                c.g[V0][0] = c.g[S1][0] ^ c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002ce960);
                c.g[S3][0] = (c.g[V0][0] < 0x1u64) as u64;
                call_ptr(c, 0x004166f4, 0x002ce960)?;
                c.g[RA][0] = sx(0x002ce968);
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
                            c.g[RA][0] = sx(0x002ce994);
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
                    let t = true;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
                    if t {
                        break 'b_2ce9c0;
                    } else {
                        break 'b_2ce9b8;
                    }
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
                c.g[RA][0] = sx(0x002ce9cc);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, t, 0x002ce9cc)?;
                c.g[RA][0] = sx(0x002ce9d4);
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                Callee {
                    addr: 0x002cbf30,
                    f: ee::s09::MenuSysPage_GetSelectedItem,
                }
                .call(c, 0x002ce9d4)?;
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x4d0000;
                    break 'b_2ce9e8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4d0000;
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                    if t {
                        break 'b_2ce9ec;
                    } else {
                        break 'b_2ce9e8;
                    }
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
                c.g[RA][0] = sx(0x002ceaa4);
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
            c.g[RA][0] = sx(0x002cead8);
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
            c.g[RA][0] = sx(0x002ceb0c);
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
                c.g[RA][0] = sx(0x002ceb38);
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
                c.g[RA][0] = sx(0x002ceb68);
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
                        c.g[RA][0] = sx(0x002cecd0);
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

/// `Vec4* MenuSysColorChannelItem_Measure(Vec4* out, Item* item)` (0x2ceee8):
/// formats the item's float value as text (`sprintf` of the value widened to
/// double), measures the label and the channel names with `Font_MeasureText`
/// (keeping the wider of each pair) and writes the row's size vector (the
/// widths plus a fixed padding) to `out`; returns `out` in `v0`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
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
                        c.g[RA][0] = sx(0x002cef4c);
                        c.lwc1(12, c.g[V1][0] as u32);
                        call_ptr(c, 0x00413af8, 0x002cef4c)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002cef5c);
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
                        c.g[RA][0] = sx(0x002cefb4);
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
                        c.g[RA][0] = sx(0x002cefdc);
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
                        c.g[RA][0] = sx(0x002cf004);
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
                            c.g[RA][0] = sx(0x002cf034);
                            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                            call_ptr(c, 0x00391fb0, 0x002cf034)?;
                            c.c_lt_s(21, 0);
                            if c.fcc() {
                                c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                                break 'b_2cf080;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7470));
                                if t {
                                    break 'b_2cf110;
                                } else {
                                    break 'b_2cf04c;
                                }
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
                    c.g[RA][0] = sx(0x002cf06c);
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
                        c.g[RA][0] = sx(0x002cf0a4);
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
                        c.g[RA][0] = sx(0x002cf0cc);
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
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x7470));
                            if t {
                                break 'b_2cf0f0;
                            } else {
                                break 'b_2cf0e8;
                            }
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
                c.g[RA][0] = sx(0x002cf108);
                c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                call_ptr(c, 0x00391fb0, 0x002cf108)?;
                let t = true;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_2cf138;
                } else {
                    break 'b_2cf110;
                }
            }
            c.cycles += 10;
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.g[RA][0] = sx(0x002cf134);
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
        c.g[RA][0] = sx(0x002cf15c);
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

// ---------------------------------------------------------------------------
// `MenuSysItem_GetLeftAxis` (0x002ca408) and `MenuSysItem_GetRightAxis`
// (0x002ca428): the item's menu system's axis value, from `item->sys +0x124`.

pub fn menu_sys_item_get_left_axis(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_item_axis_body(
            c,
            Callee {
                addr: 0x002c_c260,
                f: ee::s09::f_002cc260,
            },
            0x002c_a41c,
        )
    })
}

pub fn menu_sys_item_get_right_axis(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_item_axis_body(
            c,
            Callee {
                addr: 0x002c_c280,
                f: ee::s09::f_002cc280,
            },
            0x002c_a43c,
        )
    })
}

pub(crate) fn menu_sys_item_axis_body(c: &mut Cpu, axis: Callee, ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x124)).lw(c);
    axis.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The item wrappers of the menu axis queries: `item->menu` is the word at
/// `*item + 0x124`, and the query `callee(menu)` answers for the item. Shared
/// body of `MenuSysItem_GetUpAxis` and its three siblings (0x2ca448..0x2ca4a8):
/// each differs only in the callee and its return address.
pub(crate) fn menu_sys_item_forward(c: &mut Cpu, target: Callee, ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x124).lw(c);
    target.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `float MenuSysItem_GetUpAxis(MenuSysItem* item)` (0x2ca448): the item's
/// menu's up axis, from `MenuSys_GetUpAxis`.
pub fn menu_sys_item_get_up_axis(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee {
            addr: 0x002c_c2a0,
            f: ee::s09::MenuSys_GetUpAxis,
        };
        menu_sys_item_forward(c, target, 0x002c_a45c)
    })
}

/// `float MenuSysItem_GetDownAxis(MenuSysItem* item)` (0x2ca468): the item's
/// menu's down axis, from `MenuSys_GetDownAxis`.
pub fn menu_sys_item_get_down_axis(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee {
            addr: 0x002c_c2c0,
            f: ee::s09::MenuSys_GetDownAxis,
        };
        menu_sys_item_forward(c, target, 0x002c_a47c)
    })
}

/// `bool MenuSysItem_IsSlowModHeld(MenuSysItem* item)` (0x2ca488): whether the
/// item's menu has the slow modifier held, from `MenuSys_IsSlowModHeld`.
pub fn menu_sys_item_is_slow_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee {
            addr: 0x002c_c2e0,
            f: ee::s09::MenuSys_IsSlowModHeld,
        };
        menu_sys_item_forward(c, target, 0x002c_a49c)
    })
}

/// `bool MenuSysItem_IsFineModHeld(MenuSysItem* item)` (0x2ca4a8): whether the
/// item's menu has the fine modifier held, from `MenuSys_IsFineModHeld`.
pub fn menu_sys_item_is_fine_mod_held(c: &mut Cpu) {
    guest(c, |c| {
        let target = Callee {
            addr: 0x002c_c318,
            f: ee::s09::MenuSys_IsFineModHeld,
        };
        menu_sys_item_forward(c, target, 0x002c_a4bc)
    })
}

/// `float MenuSys_GetLeftAxis(MenuSys* menu)` (0x2cc260): the input binding
/// at `menu+0x5c` evaluated for the left axis (code 0x3e) through the function
/// table (`InputBinding_EvalFloat`, 0x320bf0).
pub fn menu_sys_get_left_axis(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x3e;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x5c).lw(c);
        call_ptr(c, 0x0032_0bf0, 0x002c_c274)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_SortItems(MenuSysPage* page, int key)` (0x2cbe20): stores
/// the sort key in the global at `gp+0x23cc`, qsorts the page's items (count
/// at `+0`, 4-byte entries from `+0xc`) with the comparator at 0x2d4008, clears
/// the global again and re-validates the selection.
pub fn menu_sys_page_sort_items(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        let key_global = (c.g[GP][0] as u32).wrapping_add(0x23cc);
        c.sw(key_global, c.g[A1][0]);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = 0x002d_4008;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = 0x4;
        call_ptr(c, 0x0041_8ef8, 0x002c_be4c)?;
        c.g[A0][0] = c.g[S0][0];
        c.sw(key_global, 0u64);
        let validate = Callee {
            addr: 0x002c_aab0,
            f: ee::s09::MenuSysPage_ValidateSelection,
        };
        validate.call(c, 0x002c_be58)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_RemoveItem(MenuSysPage* page, int index)` (0x2cbd50): shifts
/// the items after `index` down one slot (count at `+0`, selection at `+4`,
/// entries from `+0xc`), drops the count, clamps the selection to the new count
/// and re-validates it when it is now `index`.
pub(crate) fn remove_item_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    'b_remove_shift: {
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[A1][0];
        if t {
            break 'b_remove_shift;
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        loop {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if !more {
                break 'b_remove_shift;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_bd80);
            }
        }
    }
    'b_remove_count: {
        c.cycles += 6;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let t = c.g[V1][0] == 0u64;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        if t {
            break 'b_remove_count;
        }
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    }
    'b_remove_clamp: {
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            break 'b_remove_clamp;
        }
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
    }
    'b_remove_validate: {
        c.cycles += 2;
        if c.g[A1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            break 'b_remove_validate;
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        let validate = Callee {
            addr: 0x002c_aab0,
            f: ee::s09::MenuSysPage_ValidateSelection,
        };
        validate.call(c, 0x002c_bdf0)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
    }
    {
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        }
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// See [`remove_item_body`].
pub fn menu_sys_page_remove_item(c: &mut Cpu) {
    guest(c, |c| remove_item_body(c))
}

/// `void MenuSysItem_DrawRow(MenuSysItem* item, ...)` (0x2ca988): picks the
/// row's text colour words from the page's selection (the item's own row is
/// the selected one: `0x4d5380`/`0x4d53a0`, others `0x4d5390`/`0x4d53b0`), loads
/// three float words from the table entry and draws the row text through
/// `MenuSysItem_DrawRowText`.
pub(crate) fn draw_row_body(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    frame.save(c, S4, 0x20);
    frame.save_ra(c, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A2][0];
    c.g[S2][0] = c.g[A1][0];
    c.g[S3][0] = c.g[A3][0];
    c.g[S4][0] = c.g[T0][0];
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    let selected = Callee {
        addr: 0x002c_bf30,
        f: ee::s09::MenuSysPage_GetSelectedItem,
    };
    selected.call(c, 0x002c_a9c0)?;
    c.g[V0][0] = c.g[S0][0] ^ c.g[V0][0];
    c.g[V1][0] = (c.g[V0][0] < 0x1u64) as u64;
    let t = c.g[V1][0] == 0u64;
    c.g[V0][0] = 0x004d_0000;
    'b_row_selected: {
        'b_row_first: {
            if t {
                break 'b_row_first;
            }
            c.cycles += 2;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
            break 'b_row_selected;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x004d_0000;
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5390));
    }
    'b_row_colour: {
        'b_row_colour_inner: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x004d_0000;
            if t {
                break 'b_row_colour_inner;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
            break 'b_row_colour;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x004d_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
    }
    c.cycles += 24;
    c.g[AT][0] = 0x3f80_0000;
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
    let text = Callee {
        addr: 0x002c_a768,
        f: ee::s09::MenuSysItem_DrawRowText,
    };
    text.call(c, 0x002c_aa38)?;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void MenuSysItem_DrawRow(MenuSysItem* item, ...)` (0x2ca988): see
/// [`draw_row_body`].
pub fn menu_sys_item_draw_row(c: &mut Cpu) {
    guest(c, |c| draw_row_body(c))
}

/// `InputBinding_EvalFloat(binding, id)` (0x320bf0): the float for input `id`
/// comes back in f0.
pub(crate) const INPUT_BINDING_EVAL_FLOAT: u32 = 0x0032_0bf0;

/// `float MenuSys_Get*Axis(MenuSys* m)` helper: `InputBinding_EvalFloat(
/// m->binding (+0x5c), id)`, the callee's f0 returned unchanged. `ret_addr` is
/// the return address after the call.
pub(crate) fn menu_sys_eval_axis(c: &mut Cpu, id: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = id as u64;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x5c)).lw(c);
        call_ptr(c, INPUT_BINDING_EVAL_FLOAT, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float MenuSys_GetRightAxis(MenuSys* m)` (0x2cc280): the right-stick axis
/// (input id 0x3f).
pub fn menu_sys_get_right_axis(c: &mut Cpu) {
    menu_sys_eval_axis(c, 0x3f, 0x002c_c294);
}

/// `float MenuSys_GetUpAxis(MenuSys* m)` (0x2cc2a0): input id 0x40.
pub fn menu_sys_get_up_axis(c: &mut Cpu) {
    menu_sys_eval_axis(c, 0x40, 0x002c_c2b4);
}

/// `float MenuSys_GetDownAxis(MenuSys* m)` (0x2cc2c0): input id 0x41.
pub fn menu_sys_get_down_axis(c: &mut Cpu) {
    menu_sys_eval_axis(c, 0x41, 0x002c_c2d4);
}

/// `bool` helper of the modifier keys: `InputBinding_EvalFloat(m->binding,
/// id)` compared with 0.0f (`c.eq.s`), returned as 1 when not equal (a NaN
/// counts as held) and 0 when equal.
pub(crate) fn menu_sys_modifier_held(c: &mut Cpu, id: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = id as u64;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x5c)).lw(c);
        call_ptr(c, INPUT_BINDING_EVAL_FLOAT, ret_addr)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let not_equal = !c.fcc();
        c.g[V0][0] = 0x1;
        if !not_equal {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool MenuSys_IsSlowModHeld(MenuSys* m)` (0x2cc2e0): the slow-modifier
/// input (id 0x42) is non-zero.
pub fn menu_sys_is_slow_mod_held(c: &mut Cpu) {
    menu_sys_modifier_held(c, 0x42, 0x002c_c2f4);
}

/// `bool MenuSys_IsFineModHeld(MenuSys* m)` (0x2cc318): the fine-modifier
/// input (id 0x43) is non-zero.
pub fn menu_sys_is_fine_mod_held(c: &mut Cpu) {
    menu_sys_modifier_held(c, 0x43, 0x002c_c32c);
}

/// `cBoolMenuItem_cBoolMenuItem(self, text, flag, caption, on, off)` (0x2cc500):
/// the base `MenuSysItem_ctor(self, text)`, then the item's words: a3 at +0x14,
/// t0 at +0x18, t1 at +0x1c, a2 at +0x20, and the vtable `0x486e80` at +0x10.
/// Returns `self` in v0.
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
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c534)?;
        c.g[V1][0] = 0x48_0000;
        let item = c.g[S0][0] as u32;
        let flag = c.g[S1][0];
        Addr(item.wrapping_add(0x14)).sw(c, flag);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e80));
        let caption = c.g[S2][0];
        Addr(item.wrapping_add(0x18)).sw(c, caption);
        let on_text = c.g[S3][0];
        Addr(item.wrapping_add(0x1c)).sw(c, on_text);
        c.g[V0][0] = c.g[S0][0];
        let off_text = c.g[S4][0];
        Addr(item.wrapping_add(0x20)).sw(c, off_text);
        let vtable = c.g[V1][0];
        Addr(item.wrapping_add(0x10)).sw(c, vtable);
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

/// The two-row branch of `MenuSysBoolItem_Measure`: lays out the row into the
/// scratch at `sp+0x20` (s0), copies its two eight-byte halves out, lays the
/// row out again with the item's `+0x1c` as t0, then keeps the row whose first
/// float (`sp+0x18`) is not below `sp+0x8`, and copies that row to the output
/// at s2.
pub(crate) fn menu_sys_measure_two_rows(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 32;
    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x14)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = 0u64;
    Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::MenuSysItem_MeasureRow,
    }
    .call(c, 0x002c_c688)?;
    c.ldl(2, frame.at(c, 0x27));
    c.ldr(2, frame.at(c, 0x20));
    c.ldl(3, frame.at(c, 0x2f));
    c.ldr(3, frame.at(c, 0x28));
    c.sdl(2, frame.at(c, 0x7));
    c.sdr(2, frame.at(c, 0x0));
    c.sdl(3, frame.at(c, 0xf));
    c.sdr(3, frame.at(c, 0x8));
    c.g[A3][0] = 0u64;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
    c.g[T0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1c)).lw(c);
    Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::MenuSysItem_MeasureRow,
    }
    .call(c, 0x002c_c6c0)?;
    c.ldl(6, frame.at(c, 0x27));
    c.ldr(6, frame.at(c, 0x20));
    c.ldl(2, frame.at(c, 0x2f));
    c.ldr(2, frame.at(c, 0x28));
    c.sdl(6, frame.at(c, 0x17));
    c.sdr(6, frame.at(c, 0x10));
    c.sdl(2, frame.at(c, 0x1f));
    c.sdr(2, frame.at(c, 0x18));
    c.lwc1(1, frame.at(c, 0x8));
    c.lwc1(0, frame.at(c, 0x18));
    c.c_lt_s(0, 1);
    let not_less = !c.fcc();
    c.g[V0][0] = sx(frame.at(c, 0x10));
    if !not_less {
        c.cycles += 1;
        c.g[V0][0] = c.g[SP][0];
    }
    c.cycles += 9;
    let src = c.g[V0][0] as u32;
    c.ldl(5, src.wrapping_add(0x7));
    c.ldr(5, src);
    c.ldl(6, src.wrapping_add(0xf));
    c.ldr(6, src.wrapping_add(0x8));
    let dst = c.g[S2][0] as u32;
    c.sdl(5, dst.wrapping_add(0x7));
    c.sdr(5, dst);
    c.sdl(6, dst.wrapping_add(0xf));
    c.sdr(6, dst.wrapping_add(0x8));
    Ok(())
}

/// `void cBoolMenuItem_preRender(...)` (0x2cc648): measures the bool item's
/// row into the output `a0` (kept in s2). With a non-zero `+0x18` and a
/// non-zero `+0x1c` it lays out two rows and keeps one (see
/// `menu_sys_measure_two_rows`); otherwise a single row. `s0` is set on the
/// two-row test but not saved (the generated frame does not save it), and the
/// output is returned in v0.
pub fn menu_sys_bool_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x30);
        c.g[T0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[S2][0] = c.g[A0][0];
        let two_rows = if c.g[T0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[S0][0] = sx(frame.at(c, 0x20));
            c.g[V0][0] != 0
        } else {
            false
        };
        if two_rows {
            menu_sys_measure_two_rows(c, frame)?;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[A3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[A2][0] = 0u64;
            c.g[T0][0] = 0u64;
            Callee {
                addr: 0x002c_a4c8,
                f: ee::s09::MenuSysItem_MeasureRow,
            }
            .call(c, 0x002c_c738)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cEnumMenuItem_cEnumMenuItem(self)` (0x2cc7b0): `MenuSysItem_ctor(self,
/// -1)`, then the value word at +0x14 and the words at +0x18 and +0x20 zeroed,
/// and the vtable `0x486e30` at +0x10. Returns `self` in v0.
pub fn menu_sys_enum_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffff_ffff_ffff_ffffu64;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c7c8)?;
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6e30));
        let item = c.g[S0][0] as u32;
        Addr(item.wrapping_add(0x14)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(item.wrapping_add(0x10)).sw(c, vtable);
        Addr(item.wrapping_add(0x20)).sw(c, 0);
        Addr(item.wrapping_add(0x18)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Vtable of `MenuSysEnumItem` (`D_00486E30`).
pub(crate) const ENUM_ITEM_VTABLE: u32 = 0x0048_6e30;

/// Vtable of `MenuSysActionItem` (`D_00486D40`).
pub(crate) const ACTION_ITEM_VTABLE: u32 = 0x0048_6d40;

/// Vtable of `MenuSysCallbackItem` (`D_00486CF0`).
pub(crate) const CALLBACK_ITEM_VTABLE: u32 = 0x0048_6cf0;

/// `cEnumMenuItem_cEnumMenuItem1` (0x2cc7f8), `MenuSysEnumItem_ctor_2(self, text)`:
/// the base item constructor, then the enum vtable at +0x10 and zeroed
/// fields at +0x14, +0x18 and +0x20.
pub fn menu_sys_enum_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c80c)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(ENUM_ITEM_VTABLE);
        item.off(0x14).sw(c, 0);
        item.off(0x10).sw(c, c.g[V1][0]);
        item.off(0x20).sw(c, 0);
        item.off(0x18).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cEnumMenuItem_cEnumMenuItem2` (0x2cc838), `MenuSysEnumItem_ctor_3(self, text,
/// a2, a3, a4, a5)`: the base constructor, then the enum vtable and the five
/// caller-given fields (+0x14 = a3, +0x18 = a4, +0x1c = a5, +0x20 = a2).
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
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_c86c)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(ENUM_ITEM_VTABLE);
        item.off(0x14).sw(c, c.g[S1][0]);
        item.off(0x20).sw(c, c.g[S2][0]);
        item.off(0x18).sw(c, c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x1c).sw(c, c.g[S4][0]);
        item.off(0x10).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy the 16 bytes at sp+0x10 to sp+0 through the registers `a` and `b`,
/// with the unaligned `ldl`/`ldr`/`sdl`/`sdr` pairs the generated code uses.
pub(crate) fn copy_block_through(c: &mut Cpu, a: usize, b: usize) {
    let s = sp(c);
    c.ldl(a, s.wrapping_add(0x17));
    c.ldr(a, s.wrapping_add(0x10));
    c.ldl(b, s.wrapping_add(0x1f));
    c.ldr(b, s.wrapping_add(0x18));
    c.sdl(a, s.wrapping_add(0x7));
    c.sdr(a, s);
    c.sdl(b, s.wrapping_add(0xf));
    c.sdr(b, s.wrapping_add(0x8));
}

/// `cEnumMenuItem_preRender` (0x2ccb18), `MenuSysEnumItem_Measure(out, item)`:
/// measures the item's rows with `MenuSysItem_MeasureRow` (row count at +0x18,
/// row table at +0x1c, 8 bytes per row, row text argument at +0x14), keeps
/// the 16-byte block of the measure in a stack buffer and copies it to `out`.
/// Returns `out`. The back edge of the row loop checks for due events.
pub fn menu_sys_enum_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x70);
        // The style pointer the row measure takes as its fifth argument.
        c.g[T0][0] = sx(0x0048_66b8);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[A3][0] = 0;
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let buf = frame.at(c, 0x10);
        c.g[A0][0] = sx(buf);
        c.g[S3][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[S0][0] = 0;
        Callee {
            addr: 0x002c_a4c8,
            f: ee::s09::MenuSysItem_MeasureRow,
        }
        .call(c, 0x002c_cb58)?;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x18).lw(c);
        copy_block_through(c, 3, 4);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
            loop {
                c.cycles += 14;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A3][0] = 0;
                Callee {
                    addr: 0x002c_a4c8,
                    f: ee::s09::MenuSysItem_MeasureRow,
                }
                .call(c, 0x002c_cba8)?;
                let f1 = frame.at(c, 0x18);
                c.lwc1(1, f1);
                let f0 = frame.at(c, 0x8);
                c.lwc1(0, f0);
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x18).lw(c);
                } else {
                    c.cycles += 9;
                    copy_block_through(c, 2, 3);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x18).lw(c);
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_cb88);
                }
            }
        }
        c.cycles += 16;
        let s = sp(c);
        c.ldl(6, s.wrapping_add(0x7));
        c.ldr(6, s);
        c.ldl(3, s.wrapping_add(0xf));
        c.ldr(3, s.wrapping_add(0x8));
        let out = c.g[S2][0] as u32;
        c.sdl(6, out.wrapping_add(0x7));
        c.sdr(6, out);
        c.sdl(3, out.wrapping_add(0xf));
        c.g[V0][0] = c.g[S2][0];
        c.sdr(3, out.wrapping_add(0x8));
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysSubmenuItem_Draw(item, a1)`, 0x2ccdc8: draws the item's row with
/// `MenuSysItem_DrawRow(item, a1, 0, item.+0x14, 0)` and returns.
pub fn menu_sys_submenu_item_draw(c: &mut Cpu) {
    draw_row_and_return(c, 0x002c_cde0);
}

/// `MenuSysBackItem_Draw(item, a1)`, 0x2ccee0: the same row draw as the
/// submenu item (`MenuSysItem_DrawRow` with the item's +0x14 field).
pub fn menu_sys_back_item_draw(c: &mut Cpu) {
    draw_row_and_return(c, 0x002c_cef8);
}

/// The shared body of the two row draws: one call to `MenuSysItem_DrawRow`
/// returning to `ret_to`, with a2 = 0, t0 = 0 and a3 = the item's +0x14 word.
pub(crate) fn draw_row_and_return(c: &mut Cpu, ret_to: u32) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A3][0] = Addr(arg(c, 0)).off(0x14).lw(c);
        Callee {
            addr: 0x002c_a988,
            f: ee::s09::MenuSysItem_DrawRow,
        }
        .call(c, ret_to)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cActionMenuItem_cActionMenuItem2` (0x2ccf50), `MenuSysActionItem_ctor_2(self)`:
/// the base constructor, then the action vtable at +0x10 and zeroed fields.
pub fn menu_sys_action_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_cf64)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(ACTION_ITEM_VTABLE);
        item.off(0x14).sw(c, 0);
        item.off(0x10).sw(c, c.g[V1][0]);
        item.off(0x1c).sw(c, 0);
        item.off(0x20).sw(c, 0);
        item.off(0x18).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysCallbackItem_ctor(self)`, 0x2cd128: the base constructor with a
/// data argument of -1, then the callback vtable at +0x10 and zeroed fields.
pub fn menu_sys_callback_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d140)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(CALLBACK_ITEM_VTABLE);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x14).sw(c, 0);
        item.off(0x10).sw(c, c.g[V1][0]);
        item.off(0x18).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `MenuSysCallbackItem_ctor_2(self, a1, a2, a3)`, 0x2cd168: the base
/// constructor, then the callback vtable and the caller's a2 (+0x14) and a3
/// (+0x18) fields.
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
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d18c)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(CALLBACK_ITEM_VTABLE);
        item.off(0x14).sw(c, c.g[S1][0]);
        item.off(0x18).sw(c, c.g[S2][0]);
        item.off(0x10).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MenuSysCallbackItem_Draw(MenuSysItem* item, rect)` (0x2cd278): draws
/// the item row through `MenuSysItem_DrawRow` with the callback flags 0x4866c8
/// in t0, the item's +0x14 word as a2 and a3 = 0. Arguments a0/a1 pass through.
pub fn menu_sys_callback_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = 0x0048_66c8;
        c.g[A3][0] = 0;
        c.g[A2][0] = Addr(abi::arg(c, 0)).off(0x14).lw(c);
        Callee {
            addr: 0x002c_a988,
            f: ee::s09::MenuSysItem_DrawRow,
        }
        .call(c, 0x002c_d294)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysLabelItem_Draw(MenuSysItem* item, rect)` (0x2cd328): draws the
/// row with a2 = 0, t0 = 0 and a3 = the item's +0x14 word.
pub fn menu_sys_label_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        c.g[A3][0] = Addr(abi::arg(c, 0)).off(0x14).lw(c);
        Callee {
            addr: 0x002c_a988,
            f: ee::s09::MenuSysItem_DrawRow,
        }
        .call(c, 0x002c_d340)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `MenuSysLabelItem_ctor` and `MenuSysSpacerItem_ctor`:
/// `MenuSysItem_ctor(item, -1)`, then the item's data word a1 at +0x14, and
/// the class vtable at +0x10. Returns to `ret_to` from the base constructor.
pub(crate) fn named_item_ctor(c: &mut Cpu, vtable: u32, ret_to: u32) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    Callee {
        addr: 0x002c_a258,
        f: ee::s09::MenuSysItem_ctor,
    }
    .call(c, ret_to)?;
    let item = Addr(c.g[S0][0] as u32);
    let data = c.g[S1][0];
    item.off(0x14).sw(c, data);
    c.g[V1][0] = sx(vtable);
    c.g[V0][0] = c.g[S0][0];
    let v1 = c.g[V1][0];
    item.off(0x10).sw(c, v1);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysLabelItem* MenuSysLabelItem_ctor(MenuSysLabelItem* item, int data)`
/// (0x2cd2a0): a named item with the label vtable 0x486ca0.
pub fn menu_sys_label_item_ctor(c: &mut Cpu) {
    guest(c, |c| named_item_ctor(c, 0x0048_6ca0, 0x002c_d2c0));
}

/// `MenuSysSpacerItem* MenuSysSpacerItem_ctor(MenuSysSpacerItem* item, int data)`
/// (0x2cd350): the same shape as the label item, with the spacer vtable 0x486c50.
pub fn menu_sys_spacer_item_ctor(c: &mut Cpu) {
    guest(c, |c| named_item_ctor(c, 0x0048_6c50, 0x002c_d370));
}

/// The Int8 item fields after the base constructor: vtable 0x486c00 at +0x10,
/// the int word at +0x14 and +0x18 cleared, the two bytes at +0x1c/+0x1d
/// cleared, and the words at +0x20, +0x24 and +0x28 cleared.
pub(crate) fn int8_item_fields(c: &mut Cpu) {
    let item = Addr(c.g[S0][0] as u32);
    c.g[V1][0] = sx(0x0048_6c00);
    c.g[V0][0] = c.g[S0][0];
    item.off(0x14).sw(c, 0);
    let v1 = c.g[V1][0];
    item.off(0x10).sw(c, v1);
    item.off(0x18).sw(c, 0);
    item.off(0x1c).sb(c, 0);
    item.off(0x1d).sb(c, 0);
    item.off(0x24).sw(c, 0);
    item.off(0x20).sw(c, 0);
    item.off(0x28).sw(c, 0);
}

/// `MenuSysInt8Item* MenuSysInt8Item_ctor(MenuSysInt8Item* item, ...)`
/// (0x2cd3f8): `MenuSysItem_ctor(item, -1)` and the Int8 item fields.
pub fn menu_sys_int8_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d410)?;
        int8_item_fields(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysInt8Item_ctor_2` (0x2cd450): the same as `MenuSysInt8Item_ctor`,
/// but the base constructor keeps the caller's a1.
pub fn menu_sys_int8_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d464)?;
        int8_item_fields(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysInt8Item_ctor_3(item, a1, a2, a3, t0, t1)` (0x2cd4a0): the base
/// constructor, then the vtable 0x486c00, the int word a2 at +0x18, the data
/// word t1 at +0x14, the bytes a3 and t0 (sign-extended from their low byte)
/// at +0x1c and +0x1d, and the words at +0x20, +0x24 and +0x28 cleared.
pub fn menu_sys_int8_item_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx((c.g[A3][0] as u32) << 24);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = sx((c.g[T0][0] as u32) << 24);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        c.g[S2][0] = sx(((c.g[S2][0] as u32) as i32 >> 24) as u32);
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d4dc)?;
        let item = Addr(c.g[S0][0] as u32);
        let (s1, s2, s3, s4) = (c.g[S1][0], c.g[S2][0], c.g[S3][0], c.g[S4][0]);
        c.g[V1][0] = sx(0x0048_6c00);
        item.off(0x14).sw(c, s3);
        item.off(0x18).sw(c, s4);
        item.off(0x1c).sb(c, s1);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x1d).sb(c, s2);
        let v1 = c.g[V1][0];
        item.off(0x10).sw(c, v1);
        item.off(0x24).sw(c, 0);
        item.off(0x20).sw(c, 0);
        item.off(0x28).sw(c, 0);
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

/// `MenuSysIntItem* MenuSysIntItem_ctor(MenuSysIntItem* item, ...)`
/// (0x2cd850): `MenuSysItem_ctor(item, -1)`, then the vtable 0x486bb0 and the
/// cleared int words at +0x14, +0x18, +0x1c, +0x20, +0x24 and +0x28, +0x2c.
pub fn menu_sys_int_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002c_d868)?;
        let item = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = sx(0x0048_6bb0);
        c.g[V0][0] = c.g[S0][0];
        item.off(0x14).sw(c, 0);
        let v1 = c.g[V1][0];
        item.off(0x10).sw(c, v1);
        item.off(0x18).sw(c, 0);
        item.off(0x1c).sw(c, 0);
        item.off(0x20).sw(c, 0);
        item.off(0x28).sw(c, 0);
        item.off(0x24).sw(c, 0);
        item.off(0x2c).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysIntItem* MenuSysIntItem_ctor_2(item, a1)` (0x002cd8a8): the base menu
/// item, then the int item's class vtable and its zeroed value fields.
pub fn menu_sys_int_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_a258, 0x002c_d8bc)?; // MenuSysItem_ctor
        c.g[V1][0] = sx(0x0048_6bb0);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x14), 0);
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.sw(this.wrapping_add(0x18), 0);
        c.sw(this.wrapping_add(0x1c), 0);
        c.sw(this.wrapping_add(0x20), 0);
        c.sw(this.wrapping_add(0x28), 0);
        c.sw(this.wrapping_add(0x24), 0);
        c.sw(this.wrapping_add(0x2c), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysFloatItem* MenuSysFloatItem_ctor(item, a1)` (0x002cdc88): the base
/// menu item (with the -1 data word), then the float item's vtable, unit scale
/// 1.0 and zeroed fields.
pub fn menu_sys_float_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_a258, 0x002c_dca0)?; // MenuSysItem_ctor
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V1][0] = sx(0x0048_6b60);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.swc1(0, this.wrapping_add(0x24));
        c.sw(this.wrapping_add(0x14), 0);
        c.sw(this.wrapping_add(0x18), 0);
        c.sw(this.wrapping_add(0x1c), 0);
        c.sw(this.wrapping_add(0x20), 0);
        c.sw(this.wrapping_add(0x30), 0);
        c.sw(this.wrapping_add(0x2c), 0);
        c.sw(this.wrapping_add(0x28), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysFloatRangeItem* MenuSysFloatRangeItem_ctor(item, a1, a2, a3, f12, f13)`
/// (0x002ce170): the base menu item, then the range item's vtable, its two
/// int fields and the float range (f12 = low, f13 = high).
pub fn menu_sys_float_range_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_fpr(c, 21, 0x48);
        c.g[S2][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x40);
        c.mov_s(21, 13);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        call_ptr(c, 0x002c_a258, 0x002c_e1a4)?; // MenuSysItem_ctor
        c.g[V1][0] = sx(0x0048_6b10);
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x14), c.g[S1][0]);
        c.sw(this.wrapping_add(0x18), c.g[S2][0]);
        c.swc1(20, this.wrapping_add(0x1c));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(21, this.wrapping_add(0x20));
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.sw(this.wrapping_add(0x28), 0);
        c.sw(this.wrapping_add(0x24), 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_PageForwardV03(item)` (0x002cf788): forward one page
/// through the group's vtable: the slot at +0x4c of the vtable at +0x10 of the
/// group (`item+0x130`), called with `this` moved by the slot's delta at +0x48.
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

/// `void MenuSys_FormatFloat(char* buf, float v, ...)` (0x002cdfd8): formats a float item value with the fixed-point rounding table (0x40da10 helpers) and writes it through the string formatter.
#[allow(
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    unused_labels
)]
pub fn menu_sys_format_float(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ce038: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.mov_s(21, 12);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x002cdffc);
            c.g[S1][0] = c.g[A0][0];
            call_ptr(c, 0x0040da10, 0x002cdffc)?;
            c.g[AT][0] = 0x41200000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x002ce00c);
            c.mov_s(20, 0);
            call_ptr(c, 0x0040da10, 0x002ce00c)?;
            c.div_s(20, 20, 0);
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc8));
            c.add_s(20, 20, 0);
            c.cvt_w_s(1, 20);
            c.cvt_s_w(1, 1);
            c.c_lt_s(20, 1);
            let t = !c.fcc();

            if t {
                break 'b_2ce038;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sub_s(1, 1, 0);
                break 'b_2ce038;
            }
        }
        'b_2ce0a8: {
            'b_2ce054: {
                c.cycles += 5;
                c.cvt_w_s(0, 1);
                c.mfc1(3, 0);
                let t = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'b_2ce054;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3998));
                    if t {
                        break 'b_2ce0a8;
                    } else {
                        break 'b_2ce054;
                    }
                }
            }
            'b_2ce068: {
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) > 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                if t {
                    break 'b_2ce068;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4a0000;
                    let t = true;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39a0));
                    if t {
                        break 'b_2ce0a8;
                    } else {
                        break 'b_2ce068;
                    }
                }
            }
            'b_2ce078: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'b_2ce078;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39a8));
                    if t {
                        break 'b_2ce0a8;
                    } else {
                        break 'b_2ce078;
                    }
                }
            }
            'b_2ce08c: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'b_2ce08c;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39b0));
                    if t {
                        break 'b_2ce0a8;
                    } else {
                        break 'b_2ce08c;
                    }
                }
            }
            'b_2ce0a0: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 7i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    break 'b_2ce0a0;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39b8));
                    if t {
                        break 'b_2ce0a8;
                    } else {
                        break 'b_2ce0a0;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x39c0));
            break 'b_2ce0a8;
        }
        c.cycles += 22;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.mov_s(12, 21);
        c.g[RA][0] = sx(0x002ce0b8);
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x00413af8, 0x002ce0b8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002ce0c8);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003194a8, 0x002ce0c8)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x002ce0d4);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00317fe8, 0x002ce0d4)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x002ce0e0);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x002ce0e0)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int MenuSysFloatRangeItem_HandleInput(item)` (0x002ce1f0): base input, the disabled check, then the range slider step, clamp and value callback.
#[allow(
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    unused_labels
)]
pub fn menu_sys_float_range_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ce350: {
            'b_2ce34c: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x002ce20c);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                call_ptr(c, 0x002ca2b0, 0x002ce20c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002ce218);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002ca3b0, 0x002ce218)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[S1][0];
                if t {
                    break 'b_2ce34c;
                } else {
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_2ce350;
                    } else {
                        'b_2ce27c: {
                            'b_2ce264: {
                                c.cycles += 10;
                                c.g[RA][0] = sx(0x002ce230);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca428, 0x002ce230)?;
                                c.mov_s(20, 0);
                                c.g[RA][0] = sx(0x002ce23c);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca408, 0x002ce23c)?;
                                c.sub_s(20, 20, 0);
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc4));
                                c.c_le_s(20, 0);
                                let t = c.fcc();

                                if t {
                                    break 'b_2ce264;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fc0));
                                    c.c_le_s(0, 20);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0u64;
                                    if t {
                                        break 'b_2ce27c;
                                    } else {
                                        break 'b_2ce264;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.mtc1(0, 0);
                            c.c_lt_s(20, 0);
                            let t = !c.fcc();
                            c.g[V1][0] = 0x1;
                            if t {
                                break 'b_2ce27c;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0xffffffffffffffff;
                                break 'b_2ce27c;
                            }
                        }
                        'b_2ce29c: {
                            'b_2ce298: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
                                    break 'b_2ce298;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                    if t {
                                        break 'b_2ce29c;
                                    } else {
                                        break 'b_2ce298;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
                            break 'b_2ce29c;
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = c.g[S1][0];
                        if t {
                            break 'b_2ce34c;
                        } else {
                            'b_2ce310: {
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
                                    break 'b_2ce310;
                                } else {
                                    break 'b_2ce310;
                                }
                            }
                            'b_2ce32c: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x20));
                                c.lwc1(0, c.g[V0][0] as u32);
                                c.c_lt_s(1, 0);
                                if c.fcc() {
                                    c.swc1(1, c.g[V0][0] as u32);
                                    break 'b_2ce32c;
                                } else {
                                    break 'b_2ce32c;
                                }
                            }
                            c.cycles += 8;
                            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002ce348);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x002ce348)?;
                            c.g[V0][0] = c.g[S1][0];
                            break 'b_2ce34c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_2ce350;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void MenuSysTextEditItem_Measure(out, row)` (0x002ce820): measure the row, then the text-edit padding and the row width helper.
#[allow(
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    unused_labels
)]
pub fn menu_sys_text_edit_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ce898: {
            'b_2ce894: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.g[S3][0] = c.g[A0][0];
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                c.g[RA][0] = sx(0x002ce84c);
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                call_ptr(c, 0x002cbf30, 0x002ce84c)?;
                c.g[S0][0] = c.g[S2][0] ^ c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x002ce860);
                c.g[S0][0] = (c.g[S0][0] < 0x1u64) as u64;
                call_ptr(c, 0x004166f4, 0x002ce860)?;
                c.g[RA][0] = sx(0x002ce868);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00416810, 0x002ce868)?;
                let t = c.g[S0][0] == 0u64;
                c.g[V1][0] = c.g[V0][0];
                if t {
                    break 'b_2ce894;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        break 'b_2ce898;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x4a0000;
                        if t {
                            break 'b_2ce894;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x002ce894);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x39d0));
                            call_ptr(c, 0x004162d0, 0x002ce894)?;
                            break 'b_2ce894;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[S2][0];
            break 'b_2ce898;
        }
        c.cycles += 29;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x002ce8ac);
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, 0x002ca4c8, 0x002ce8ac)?;
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x3f));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x2f));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.sdl(6, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(6, c.g[S3][0] as u32);
        c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0xf));
        c.sdr(3, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `int MenuSysColorChannelItem_HandleInput(item)` (0x002ced78): base input, the disabled check, then the channel step, clamp and value callback.
#[allow(
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    unused_labels
)]
pub fn menu_sys_color_channel_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ceed4: {
            'b_2ceed0: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x002ced94);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                call_ptr(c, 0x002ca2b0, 0x002ced94)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002ceda0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002ca3b0, 0x002ceda0)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.g[S1][0];
                if t {
                    break 'b_2ceed0;
                } else {
                    c.cycles += 2;
                    if c.g[S1][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_2ceed4;
                    } else {
                        'b_2cee04: {
                            'b_2cedec: {
                                c.cycles += 10;
                                c.g[RA][0] = sx(0x002cedb8);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca428, 0x002cedb8)?;
                                c.mov_s(20, 0);
                                c.g[RA][0] = sx(0x002cedc4);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002ca408, 0x002cedc4)?;
                                c.sub_s(20, 20, 0);
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fb0));
                                c.c_le_s(20, 0);
                                let t = c.fcc();

                                if t {
                                    break 'b_2cedec;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fac));
                                    c.c_le_s(0, 20);
                                    let t = !c.fcc();
                                    c.g[V1][0] = 0u64;
                                    if t {
                                        break 'b_2cee04;
                                    } else {
                                        break 'b_2cedec;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.mtc1(0, 0);
                            c.c_lt_s(20, 0);
                            let t = !c.fcc();
                            c.g[V1][0] = 0x1;
                            if t {
                                break 'b_2cee04;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0xffffffffffffffff;
                                break 'b_2cee04;
                            }
                        }
                        'b_2cee24: {
                            'b_2cee20: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
                                    break 'b_2cee20;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                                    if t {
                                        break 'b_2cee24;
                                    } else {
                                        break 'b_2cee20;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                            break 'b_2cee24;
                        }
                        'b_2ceecc: {
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_2ceecc;
                            } else {
                                'b_2cee44: {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                    c.g[A0][0] = 0x120;
                                    c.g[A3][0] = 0x1;
                                    c.mult(0, 3, 3, 3);
                                    if c.g[A0][0] == c.g[A1][0] {
                                        break 'b_2cee44;
                                    } else {
                                        break 'b_2cee44;
                                    }
                                }
                                'b_2cee94: {
                                    c.cycles += 20;
                                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fa8));
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    c.div(0, 3, 4);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[A2][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                                        break 'b_2cee94;
                                    } else {
                                        break 'b_2cee94;
                                    }
                                }
                                'b_2ceeac: {
                                    c.cycles += 6;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);
                                    c.c_lt_s(0, 2);
                                    if c.fcc() {
                                        c.mov_s(2, 0);
                                        break 'b_2ceeac;
                                    } else {
                                        break 'b_2ceeac;
                                    }
                                }
                                c.cycles += 8;
                                c.swc1(2, c.g[A2][0] as u32);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x002ceecc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x002ceecc)?;
                                break 'b_2ceecc;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S1][0];
                        break 'b_2ceed0;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_2ceed4;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// Offsets in a `MenuSysGroupItem`: the header item (+0x14) and the page it
/// holds (+0x18: item count at +0, selected index at +4, item pointers from +0xc).
pub(crate) const MSGI_HEADER: u32 = 0x14;

pub(crate) const MSGI_PAGE: u32 = 0x18;

pub(crate) const MSGI_PAGE_SELECTED: u32 = MSGI_PAGE + 4;

pub(crate) const MSGI_PAGE_ITEMS: u32 = MSGI_PAGE + 0xc;

/// `void MenuSysGroupItem_PageForwardV04(MenuSysGroupItem* this, int a)` (0x2cf7b8):
/// a virtual call on the object at +0x130, with the object's `+0x50` thunk
/// delta applied to `this`; returns what the callee returns.
pub fn menu_sys_group_item_page_forward_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x130)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x50)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x002c_f7d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem* MenuSysGroupItem_ctor(MenuSysGroupItem* this)` (0x2cf7e8):
/// construct the base item with id -1, the page at +0x18 (vtable 0x4869b8 at +0x10),
/// clear the header, the page's id slot at +0x144 (0x486a18), and store `this` at +0x148.
pub fn menu_sys_group_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::f_002ca258,
        }
        .call(c, 0x002c_f800)?;
        c.g[V1][0] = 0x480000;
        Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x69b8));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE));
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, vtable);
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::f_002caa58,
        }
        .call(c, 0x002c_f818)?;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a18));
        c.g[V1][0] = 0x45;
        let id_slot = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x144)).sw(c, id_slot);
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = c.g[S0][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002c_f830);
                }
                continue;
            }
            c.cycles += 5;
            let this = c.g[S0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x148)).sw(c, this);
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `MenuSysGroupItem* MenuSysGroupItem_ctor_2(MenuSysGroupItem* this, void* name,
/// int id)` (0x2cf860): the same construction with a header item (`id` at +0x14);
/// a non-zero id is added to the page.
pub fn menu_sys_group_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::f_002ca258,
        }
        .call(c, 0x002c_f87c)?;
        c.g[V1][0] = 0x480000;
        let id = c.g[S0][0];
        Addr((c.g[S1][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, id);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x69b8));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE));
        let vtable = c.g[V1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x10)).sw(c, vtable);
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::f_002caa58,
        }
        .call(c, 0x002c_f894)?;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a18));
        c.g[V1][0] = 0x45;
        let id_slot = c.g[V0][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x144)).sw(c, id_slot);
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_HEADER)).lw(c);
        let t = c.g[A1][0] == 0;
        let this = c.g[S1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x148)).sw(c, this);
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            Callee {
                addr: 0x002c_f908,
                f: ee::s09::f_002cf908,
            }
            .call(c, 0x002c_f8c0)?;
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

/// `void MenuSysGroupItem_AddItem(MenuSysGroupItem* this, MenuSysItem* item, int index)`
/// (0x2cf908): insert the item into the group's page.
pub fn menu_sys_group_item_add_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MSGI_PAGE));
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::f_002cbcc8,
        }
        .call(c, 0x002c_f918)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_RemoveItem(MenuSysGroupItem* this, int item)` (0x2cf928):
/// remove the item from the group's page.
pub fn menu_sys_group_item_remove_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MSGI_PAGE));
        Callee {
            addr: 0x002c_bd50,
            f: ee::s09::f_002cbd50,
        }
        .call(c, 0x002c_f938)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_Reset(MenuSysGroupItem* this)` (0x2cf948): empty the
/// page, then add the header item back if there is one.
pub fn menu_sys_group_item_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE));
        Callee {
            addr: 0x002c_be10,
            f: ee::s09::f_002cbe10,
        }
        .call(c, 0x002c_f960)?;
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).lw(c);
        let t = c.g[A1][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !t {
            c.cycles += 2;
            c.g[A2][0] = u64::MAX;
            Callee {
                addr: 0x002c_f908,
                f: ee::s09::f_002cf908,
            }
            .call(c, 0x002c_f974)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_SetHeaderItem(MenuSysGroupItem* this, MenuSysItem* item)`
/// (0x2cf988): make `item` the header. A previous header is taken out of the page
/// first, and the new one is added to it. A resumable dispatcher: the event check
/// in the search loop returns with `pc` at the loop head, and the function is
/// re-entered there.
pub fn menu_sys_group_item_set_header_item(c: &mut Cpu) {
    guest(c, set_header_item_steps);
}

pub(crate) fn set_header_item_steps(c: &mut Cpu) -> Flow {
    let frame = Frame { size: 0x30 };
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002c_f988 => {
                c.cycles += 9;
                let _ = Frame::open(c, 0x30);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A0][0];
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[A1][0] = 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                pc = 0x002c_f9b4;
            }
            0x002c_f9ac => {
                c.cycles += 1;
                pc = 0x002c_f9b0;
            }
            0x002c_f9b0 => {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                pc = 0x002c_f9b4;
            }
            0x002c_f9b4 => {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    let item = c.g[S1][0];
                    Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, item);
                    pc = 0x002c_f9e0;
                } else {
                    pc = 0x002c_f9c4;
                }
            }
            0x002c_f9c4 => {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).lw(c);
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                if t {
                    if back_edge_unwinds(c, 0x002c_f9b0) {
                        return Ok(());
                    }
                    pc = 0x002c_f9b0;
                } else {
                    pc = 0x002c_f9d4;
                }
            }
            0x002c_f9d4 => {
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE));
                Callee {
                    addr: 0x002c_bd50,
                    f: ee::s09::f_002cbd50,
                }
                .call(c, 0x002c_f9dc)?;
                let item = c.g[S1][0];
                Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, item);
                pc = 0x002c_f9e0;
            }
            0x002c_f9e0 => {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0;
                Callee {
                    addr: 0x002c_f908,
                    f: ee::s09::f_002cf908,
                }
                .call(c, 0x002c_f9f0)?;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x002c_f988, 0x002c_fa04);
                return Ok(());
            }
        }
    }
}

/// Leave `HandleInput` after its selection step: `cycles += 1`, s0 restored,
/// then the epilogue.
pub(crate) fn handle_input_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x40);
    handle_input_epilogue(c, frame);
    Ok(())
}

/// The epilogue of `HandleInput`: restore s1..s3 and ra, pop the frame.
pub(crate) fn handle_input_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// Key 3 (`MenuSysPage_SelectNext`): when the selection did not move back,
/// set the selection to the previous index and return 3.
pub(crate) fn handle_input_select_next(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S3][0];
    Callee {
        addr: 0x002c_ab08,
        f: ee::s09::f_002cab08,
    }
    .call(c, 0x002c_fa8c)?;
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE_SELECTED)).lw(c);
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let moved_back = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !moved_back {
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x002c_be68,
            f: ee::s09::f_002cbe68,
        }
        .call(c, 0x002c_faa8)?;
        c.g[V0][0] = 3;
    }
    handle_input_exit(c, frame)
}

/// Key 4 (`MenuSysPage_SelectPrev`): when the selection did not move back,
/// set the selection to the previous index and return 4.
pub(crate) fn handle_input_select_prev(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S3][0];
    Callee {
        addr: 0x002c_abe0,
        f: ee::s09::f_002cabe0,
    }
    .call(c, 0x002c_fab8)?;
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE_SELECTED)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
    let moved_back = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !moved_back {
        c.cycles += 4;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x002c_be68,
            f: ee::s09::f_002cbe68,
        }
        .call(c, 0x002c_fad4)?;
        c.g[V0][0] = 4;
    }
    handle_input_exit(c, frame)
}

/// `int MenuSysGroupItem_HandleInput(MenuSysGroupItem* this)` (0x2cfa08): validate
/// the selection, ask the selected item's virtual method for a key, and handle
/// the keys 3 (next) and 4 (previous) itself; other keys are returned.
pub fn menu_sys_group_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE));
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: 0x002c_aab0,
            f: ee::s09::f_002caab0,
        }
        .call(c, 0x002c_fa30)?;
        c.g[S2][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE_SELECTED)).lw(c);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(MSGI_PAGE_ITEMS)).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x002c_fa54)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x002c_a3b0,
            f: ee::s09::f_002ca3b0,
        }
        .call(c, 0x002c_fa60)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if disabled {
            return handle_input_exit(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = 3;
        let is_next = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 4;
        if is_next {
            return handle_input_select_next(c, frame);
        }
        c.cycles += 2;
        let is_prev = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0];
        if is_prev {
            return handle_input_select_prev(c, frame);
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x40);
        handle_input_epilogue(c, frame);
        Ok(())
    })
}

/// `MenuSysGroupItem_v09` (0x002cfaf8): a forwarding call on the menu object.
/// The object is `*a0`; its vtable is at `+0x12c`. The call goes to the
/// vtable's `+0x1c` entry with `this = obj + delta`, where `delta` is the
/// halfword at `vtable + 0x18`. `a1` and `a2` pass through unchanged.
pub fn menu_sys_group_item_v09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = Addr(r32(c, A0)).lw(c);
        c.g[V1][0] = Addr(r32(c, A3)).off(0x12c).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x18).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_fb18)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPoint2DItem_Draw(MenuSysPoint2DItem* item, float* rect)`
/// (0x2d0ef8), a menu row that shows a 2D point curve.
///
/// Draws the owning menu's row background through its vtable (slot at
/// `menu + 0x12c`), prints the row label in the focus/idle colour (the selected
/// item of the page is the focused one), and, while the item shows its cursor
/// and is focused, the current point as "x<sep>y" below it. Then it draws a
/// 100-unit-wide graph on the right of the rect: a filled background quad
/// (`D_004D53C0`), the zero axis when the Y range straddles 0 (`D_004D5380`),
/// the polyline through the item's points (`D_004D53D0`) and the cursor marker
/// (`D_004D53E0`), each on its own depth layer of the draw module's top object
/// (`gp - 0x854 -> +0xe84`). The top object's depth field is restored at the
/// end; `v0` is the value stored back there.
///
/// Fields: item+0x00 menu, +0x14 label, +0x18 point count, +0x24 current index,
/// +0x28 points (x, y pairs), +0x2c/+0x30 X range, +0x34/+0x38 Y range,
/// +0x40 show-cursor flag.
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
                c.g[RA][0] = sx(0x002d0f40);
                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                call_ptr(c, 0x002cbf30, 0x002d0f40)?;
                c.g[A3][0] = c.lw(c.g[S3][0] as u32);
                c.g[V0][0] = c.g[S3][0] ^ c.g[V0][0];
                c.g[S5][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.g[A1][0] = c.g[S4][0];
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x12c));
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x002d0f68);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002d0f68)?;
                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.cvt_s_w(1, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[RA][0] = sx(0x002d0f88);
                c.mul_s(21, 1, 0);
                call_ptr(c, 0x002cbf30, 0x002d0f88)?;
                let t = c.g[S3][0] != c.g[V0][0];
                c.g[V0][0] = 0x4d0000;
                if t {
                    break 'b_2d0f9c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4d0000;
                    let t = true;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
                    if t {
                        break 'b_2d0fa0;
                    } else {
                        break 'b_2d0f9c;
                    }
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
                        c.g[RA][0] = sx(0x002d105c);
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
                                c.g[RA][0] = sx(0x002d1090);
                                c.lwc1(12, c.g[V0][0] as u32);
                                call_ptr(c, 0x002cdfd8, 0x002d1090)?;
                                c.g[A2][0] = 0x4a0000;
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x39f0));
                                c.g[RA][0] = sx(0x002d10a4);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x00318630, 0x002d10a4)?;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x24));
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x28));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[RA][0] = sx(0x002d10c4);
                                c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
                                call_ptr(c, 0x002cdfd8, 0x002d10c4)?;
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x002d10d4);
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x003185c8, 0x002d10d4)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x002d10e0);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10e0)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x002d10ec);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x00318260, 0x002d10ec)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x002d10f8);
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
                                c.g[RA][0] = sx(0x002d1134);
                                c.add_s(13, 13, 21);
                                call_ptr(c, 0x00391cb0, 0x002d1134)?;
                                c.g[A0][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x002d1140);
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
                            let t = true;
                            c.sub_s(6, 1, 0);
                            if t {
                                break 'b_2d1174;
                            } else {
                                break 'b_2d1164;
                            }
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
                            c.g[RA][0] = sx(0x002d12c8);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x114), 0u64);
                            call_ptr(c, 0x002cb350, 0x002d12c8)?;
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
                                        c.g[RA][0] = sx(0x002d1444);
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
                                            c.g[RA][0] = sx(0x002d15a4);
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
                                                c.g[RA][0] = sx(0x002d16e4);
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                call_ptr(c, 0x002cb350, 0x002d16e4)?;
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
    })
}

/// `MenuSysColorSwatchItem_Draw` (0x002d0008): hand port of the whole resumable body (every entry
/// of the generated `match pc`, so a resume after an unwound callee or event
/// continues natively). Every register, cycle and call is kept.
pub fn menu_sys_color_swatch_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002d0008 => {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                    c.g[A3][0] = 0u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[T0][0] = 0u64;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[S0][0] = c.g[A1][0];
                    pc = 0x002d0020;
                    continue;
                }
                0x002d0020 => {
                    c.cycles += 133;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[RA][0] = sx(0x002d0030);
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    Callee {
                        addr: 0x002ca988,
                        f: ee::s09::MenuSysItem_DrawRow,
                    }
                    .call(c, 0x002d0030)?;
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
                        pc = 0x002d024c;
                        continue;
                    }
                    pc = 0x002d0234;
                    continue;
                }
                0x002d0234 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0xffffffffffffff83;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    if t {
                        pc = 0x002d0260;
                        continue;
                    }
                    pc = 0x002d024c;
                    continue;
                }
                0x002d024c => {
                    c.cycles += 5;
                    c.g[A1][0] = 0xffffffffffffff83;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x4;
                    pc = 0x002d0260;
                    continue;
                }
                0x002d0260 => {
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
                }
                _ => {
                    c.jump_out(pc, 0x002d0008, 0x002d02ac);
                    return Ok(());
                }
            }
        }
    });
}

/// `MenuSysVectorItem_HandleInput` (0x002d06e8): hand port of the whole resumable body (every entry
/// of the generated `match pc`, so a resume after an unwound callee or event
/// continues natively). Every register, cycle and call is kept.
pub fn menu_sys_vector_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        let mut charged = false;
        loop {
            match pc {
                0x002d06e8 => {
                    // The main body charges the first two blocks together (9) before
                    // the call; only the `__entries` twin splits them 6 + 3.
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[RA][0] = sx(0x002d06fc);
                    c.g[S0][0] = c.g[A0][0];
                    Callee {
                        addr: 0x002cfa08,
                        f: ee::s09::MenuSysGroupItem_HandleInput,
                    }
                    .call(c, 0x002d06fc)?;
                    c.g[T1][0] = c.g[V0][0];
                    charged = true;
                    pc = 0x002d0700;
                    continue;
                }
                0x002d0700 => {
                    if !charged {
                        c.cycles += 3;
                    }
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x670));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b8));
                    if t {
                        pc = 0x002d07f4;
                        continue;
                    }
                    pc = 0x002d070c;
                    continue;
                }
                0x002d070c => {
                    c.cycles += 37;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x5dc));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ec));
                    c.g[A2][0] = c.g[V0][0];
                    c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x610));
                    c.lwc1(1, c.g[A0][0] as u32);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x620));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[T0][0] = c.g[V1][0];
                    c.mul_s(0, 0, 1);
                    c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0x648));
                    c.lwc1(2, c.g[V0][0] as u32);
                    c.g[A3][0] = c.g[A1][0];
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x5e0));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.mul_s(3, 3, 2);
                    c.add_s(7, 0, 1);
                    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x614));
                    c.lwc1(4, c.g[V0][0] as u32);
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x644));
                    c.add_s(6, 3, 2);
                    c.mul_s(0, 7, 7);
                    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.mul_s(1, 1, 4);
                    c.mul_s(2, 6, 6);
                    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.mtc1(0, 3);
                    c.add_s(4, 1, 5);
                    c.add_s(0, 0, 2);
                    c.mul_s(1, 4, 4);
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.add_s(0, 0, 1);
                    c.sqrt_s(1, 0);
                    c.c_eq_s(1, 3);

                    if c.fcc() {
                        c.swc1(7, c.g[SP][0] as u32);
                        pc = 0x002d07c8;
                        continue;
                    }
                    pc = 0x002d07a0;
                    continue;
                }
                0x002d07a0 => {
                    c.cycles += 10;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.div_s(0, 0, 1);
                    c.mul_s(1, 4, 0);
                    c.mul_s(2, 7, 0);
                    c.mul_s(0, 6, 0);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(2, c.g[SP][0] as u32);
                    let t = true;
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x002d07d0;
                        continue;
                    }
                    pc = 0x002d07c8;
                    continue;
                }
                0x002d07c8 => {
                    c.cycles += 2;
                    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                    pc = 0x002d07d0;
                    continue;
                }
                0x002d07d0 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.lwc1(0, c.g[SP][0] as u32);
                    c.swc1(0, c.g[V0][0] as u32);
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x18));
                    c.swc1(0, c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(0, c.g[V0][0] as u32);
                    pc = 0x002d07f4;
                    continue;
                }
                0x002d07f4 => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = c.g[T1][0];
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d06e8, 0x002d0808);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// 0x002d0908 MenuSysPoint2DItem_HandleInput

/// `MenuSysItem_IsDisabled` (0x2ca3b0).
pub(crate) const OC1567_ITEM_IS_DISABLED: Callee = Callee {
    addr: 0x002c_a3b0,
    f: ee::s09::MenuSysItem_IsDisabled,
};

/// `MenuSysItem_GetInputCode` (0x2ca3e8): the menu's current input code.
pub(crate) const OC1567_ITEM_INPUT_CODE: Callee = Callee {
    addr: 0x002c_a3e8,
    f: ee::s09::MenuSysItem_GetInputCode,
};

/// `MenuSysItem_GetLeftAxis` (0x2ca408).
pub(crate) const OC1567_ITEM_LEFT_AXIS: Callee = Callee {
    addr: 0x002c_a408,
    f: ee::s09::MenuSysItem_GetLeftAxis,
};

/// `MenuSysItem_GetRightAxis` (0x2ca428).
pub(crate) const OC1567_ITEM_RIGHT_AXIS: Callee = Callee {
    addr: 0x002c_a428,
    f: ee::s09::MenuSysItem_GetRightAxis,
};

/// `MenuSysItem_GetUpAxis` (0x2ca448).
pub(crate) const OC1567_ITEM_UP_AXIS: Callee = Callee {
    addr: 0x002c_a448,
    f: ee::s09::MenuSysItem_GetUpAxis,
};

/// `MenuSysItem_GetDownAxis` (0x2ca468).
pub(crate) const OC1567_ITEM_DOWN_AXIS: Callee = Callee {
    addr: 0x002c_a468,
    f: ee::s09::MenuSysItem_GetDownAxis,
};

/// `MenuSysItem_HandleInput` (0x2ca2b0): the base item's handler.
pub(crate) const OC1567_ITEM_HANDLE_INPUT: Callee = Callee {
    addr: 0x002c_a2b0,
    f: ee::s09::MenuSysItem_HandleInput,
};

// Fields of the 2D point item (`cGraphMenuItem`), relative to `self`.
/// `void* menu` (its vtable pointer sits at +0x12c of the menu).
#[cfg(test)]
pub(crate) const OC1567_P2D_MENU: u32 = 0x00;

/// `int count`: the number of points.
#[cfg(test)]
pub(crate) const OC1567_P2D_COUNT: u32 = 0x18;

/// `u32 lockX`, `u32 lockY`: one bit per point.
#[cfg(test)]
pub(crate) const OC1567_P2D_LOCK_X: u32 = 0x1c;

#[cfg(test)]
pub(crate) const OC1567_P2D_LOCK_Y: u32 = 0x20;

/// `int cur`: the selected point.
#[cfg(test)]
pub(crate) const OC1567_P2D_CUR: u32 = 0x24;

/// `Vec2* pts` (8 bytes per point: x, y).
#[cfg(test)]
pub(crate) const OC1567_P2D_PTS: u32 = 0x28;

/// Bounds: `minX, minY, maxX, maxY` floats at +0x2c..+0x38.
#[cfg(test)]
pub(crate) const OC1567_P2D_BOUNDS: u32 = 0x2c;

/// `int order`: >0 keeps y ascending along the points, <0 descending.
#[cfg(test)]
pub(crate) const OC1567_P2D_ORDER: u32 = 0x3c;

/// `int editing`: 1 while a point is being dragged.
#[cfg(test)]
pub(crate) const OC1567_P2D_EDITING: u32 = 0x40;

/// `int MenuSysPoint2DItem_HandleInput(Point2DItem* self)` (0x2d0908, the
/// reference's `cGraphMenuItem::input`): the input handler of the curve
/// editor item. A disabled item returns 0. Input code 6 toggles `editing`;
/// 7 and 8 end it. While editing, left (4) and right (5) move the selection,
/// and the up/down and left/right axes drag the selected point: `y += dy *
/// (maxY - minY) / 120`, `x += dx * (maxX - minX) / 120` (unless that
/// point's bit is set in `lockY` / `lockX`), clamped to the bounds, kept in
/// order against its neighbours (y by `order`, x at least `step` apart,
/// `step = (maxX - minX) / max(2 * count, 50)`). When not editing the base
/// item's handler decides the result. A change notifies the menu through its
/// vtable (`menu->vt[4](menu, self)`).
pub fn menu_sys_point2d_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        OC1567_ITEM_IS_DISABLED.call(c, 0x002d_0930)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S2][0];
        if !disabled {
            // S1: whether a point changed; S2: the result.
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = 0;
            OC1567_ITEM_INPUT_CODE.call(c, 0x002d_0944)?;
            c.g[V1][0] = 6;
            let toggle = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
            if toggle {
                c.cycles += 3;
                c.g[V0][0] ^= 1;
                let editing = c.g[V0][0];
                Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, editing);
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
            }
            'done: {
                'base: {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break 'base;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    OC1567_ITEM_INPUT_CODE.call(c, 0x002d_096c)?;
                    c.g[V1][0] = 8;
                    if c.g[V0][0] == c.g[V1][0] {
                        // Code 8: stop editing.
                        Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, 0);
                        break 'done;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    OC1567_ITEM_INPUT_CODE.call(c, 0x002d_0980)?;
                    c.g[V1][0] = 7;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    } else {
                        // Code 7: stop editing.
                        c.cycles += 2;
                        Addr((c.g[S0][0] as u32).wrapping_add(0x40)).sw(c, 0);
                        break 'done;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break 'base;
                    }
                    oc1567_point2d_edit(c)?;
                    break 'done;
                }
                // Not editing: the base item decides.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                OC1567_ITEM_HANDLE_INPUT.call(c, 0x002d_0d00)?;
                c.g[S2][0] = c.g[V0][0];
            }
            // A changed point tells the menu: `menu->vt[4](menu, self)`.
            c.cycles += 2;
            let changed = c.g[S1][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if changed {
                c.cycles += 6;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x12c)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002d_0d24)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The address of a field of the item (`S0 + off`).
pub(crate) fn oc1567_p2d(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// The editing part of [`menu_sys_point2d_item_handle_input`] (`editing` is
/// set and the code is not 7/8): selection moves, then the y drag, then the
/// x drag.
pub(crate) fn oc1567_point2d_edit(c: &mut Cpu) -> Flow {
    'after_move: {
        'moved_next: {
            'store_cur: {
                'not_left: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    OC1567_ITEM_INPUT_CODE.call(c, 0x002d_09a4)?;
                    c.g[V1][0] = 4;
                    if c.g[V0][0] != c.g[V1][0] {
                        break 'not_left;
                    }
                    // Left: select the previous point.
                    c.cycles += 3;
                    let a = oc1567_p2d(c, 0x24);
                    c.g[V0][0] = Addr(a).lw(c);
                    let at_first = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if at_first {
                        break 'moved_next;
                    }
                    c.cycles += 2;
                    c.g[S1][0] = 1;
                    break 'store_cur;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                OC1567_ITEM_INPUT_CODE.call(c, 0x002d_09cc)?;
                c.g[V1][0] = 5;
                let not_right = c.g[V0][0] != c.g[V1][0];
                let a = oc1567_p2d(c, 0x18);
                c.g[V0][0] = Addr(a).lw(c);
                if not_right {
                    break 'after_move;
                }
                // Right: select the next point.
                c.cycles += 5;
                let a = oc1567_p2d(c, 0x24);
                c.g[V1][0] = Addr(a).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let at_last = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if at_last {
                    break 'moved_next;
                }
                c.cycles += 1;
                c.g[S1][0] = 1;
            }
            c.cycles += 1;
            let new_cur = c.g[V0][0];
            let a = oc1567_p2d(c, 0x24);
            Addr(a).sw(c, new_cur);
        }
        c.cycles += 1;
        let a = oc1567_p2d(c, 0x18);
        c.g[V0][0] = Addr(a).lw(c);
    }

    // The y drag. f21 = step = (maxX - minX) / max(2 * count, 50).
    c.cycles += 22;
    c.g[V1][0] = 0x31;
    c.lwc1(2, oc1567_p2d(c, 0x2c));
    c.g[A1][0] = 0x32;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.lwc1(1, oc1567_p2d(c, 0x34));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A1][0] = c.g[V0][0];
    }
    c.sub_s(1, 1, 2);
    c.mtc1(A1, 0);
    c.cvt_s_w(0, 0);
    c.div_s(21, 1, 0);
    c.g[A0][0] = c.g[S0][0];
    OC1567_ITEM_UP_AXIS.call(c, 0x002d_0a2c)?;
    c.mov_s(20, 0);
    c.g[A0][0] = c.g[S0][0];
    OC1567_ITEM_DOWN_AXIS.call(c, 0x002d_0a38)?;
    c.sub_s(20, 20, 0);
    c.mtc1(0, 0);
    c.c_eq_s(20, 0);
    let dy_zero = c.fcc();
    c.g[V0][0] = 1;
    'x_drag: {
        'y_done: {
            if dy_zero {
                break 'y_done;
            }
            c.cycles += 6;
            let a = oc1567_p2d(c, 0x24);
            c.g[A0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x20);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
            c.g[V1][0] &= c.g[V0][0];
            let locked = c.g[V1][0] != 0;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
            if locked {
                break 'y_done;
            }
            // y += dy * (maxY - minY) * (1 / 120), clamped to the bounds.
            c.cycles += 21;
            c.lwc1(1, oc1567_p2d(c, 0x30));
            c.lwc1(0, oc1567_p2d(c, 0x38));
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.sub_s(0, 0, 1);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3fa0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            c.mul_s(0, 20, 0);
            c.mul_s(0, 0, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            let a = oc1567_p2d(c, 0x24);
            c.g[V0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.lwc1(1, oc1567_p2d(c, 0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            c.c_lt_s(1, 0);
            let within_max = !c.fcc();
            c.g[S1][0] = 1;
            if !within_max {
                c.cycles += 1;
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            }
            c.cycles += 10;
            let a = oc1567_p2d(c, 0x24);
            c.g[V0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.lwc1(1, oc1567_p2d(c, 0x30));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
            }
            // Keep y in order against the next point (`order`).
            c.cycles += 6;
            let a = oc1567_p2d(c, 0x18);
            c.g[V0][0] = Addr(a).lw(c);
            let a = oc1567_p2d(c, 0x24);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            'order_next_done: {
                if c.g[V0][0] == 0 {
                    let a = oc1567_p2d(c, 0x24);
                    c.g[A1][0] = Addr(a).lw(c);
                    break 'order_next_done;
                }
                'next_fixed: {
                    'not_ascending: {
                        c.cycles += 3;
                        let a = oc1567_p2d(c, 0x3c);
                        c.g[A0][0] = Addr(a).lw(c);
                        let order_le0 = (c.g[A0][0] as i64) <= 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                        if order_le0 {
                            break 'not_ascending;
                        }
                        c.cycles += 8;
                        let a = oc1567_p2d(c, 0x28);
                        c.g[V0][0] = Addr(a).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
                            break 'next_fixed;
                        }
                    }
                    c.cycles += 2;
                    if (c.g[A0][0] as i64) >= 0 {
                        let a = oc1567_p2d(c, 0x24);
                        c.g[A1][0] = Addr(a).lw(c);
                        break 'order_next_done;
                    }
                    c.cycles += 10;
                    let a = oc1567_p2d(c, 0x24);
                    c.g[V0][0] = Addr(a).lw(c);
                    let a = oc1567_p2d(c, 0x28);
                    c.g[V1][0] = Addr(a).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(4));
                    c.c_lt_s(0, 1);
                    if !c.fcc() {
                        let a = oc1567_p2d(c, 0x24);
                        c.g[A1][0] = Addr(a).lw(c);
                        break 'order_next_done;
                    }
                    c.cycles += 1;
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(4));
                }
                c.cycles += 1;
                let a = oc1567_p2d(c, 0x24);
                c.g[A1][0] = Addr(a).lw(c);
            }
            // ... and against the previous point.
            c.cycles += 2;
            if (c.g[A1][0] as i64) <= 0 {
                break 'y_done;
            }
            'not_ascending_prev: {
                c.cycles += 3;
                let a = oc1567_p2d(c, 0x3c);
                c.g[A0][0] = Addr(a).lw(c);
                let order_le0 = (c.g[A0][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                if order_le0 {
                    break 'not_ascending_prev;
                }
                c.cycles += 8;
                let a = oc1567_p2d(c, 0x28);
                c.g[V0][0] = Addr(a).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_sub(4));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(4));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.swc1(1, (c.g[V1][0] as u32).wrapping_add(4));
                    break 'y_done;
                }
            }
            c.cycles += 2;
            let order_ge0 = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
            if order_ge0 {
                break 'y_done;
            }
            c.cycles += 8;
            let a = oc1567_p2d(c, 0x28);
            c.g[V0][0] = Addr(a).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_sub(4));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(4));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(4));
            }
        }
        // The x drag: x += dx * (maxX - minX) * (1 / 120), clamped, and at
        // least `step` away from its neighbours.
        c.cycles += 11;
        c.g[A0][0] = c.g[S0][0];
        OC1567_ITEM_RIGHT_AXIS.call(c, 0x002d_0bc8)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        OC1567_ITEM_LEFT_AXIS.call(c, 0x002d_0bd4)?;
        c.sub_s(20, 20, 0);
        c.mtc1(0, 0);
        c.c_eq_s(20, 0);
        let dx_zero = c.fcc();
        c.g[V0][0] = 1;
        if dx_zero {
            break 'x_drag;
        }
        c.cycles += 6;
        let a = oc1567_p2d(c, 0x24);
        c.g[A0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x1c);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A0][0] as u32) & 31));
        c.g[V1][0] &= c.g[V0][0];
        let locked = c.g[V1][0] != 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
        if locked {
            break 'x_drag;
        }
        c.cycles += 21;
        c.lwc1(1, oc1567_p2d(c, 0x2c));
        c.lwc1(0, oc1567_p2d(c, 0x34));
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.sub_s(0, 0, 1);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3f9c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(1, c.g[V0][0] as u32);
        c.mul_s(0, 20, 0);
        c.mul_s(0, 0, 2);
        c.add_s(1, 1, 0);
        c.swc1(1, c.g[V0][0] as u32);
        let a = oc1567_p2d(c, 0x24);
        c.g[V0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.lwc1(1, oc1567_p2d(c, 0x34));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, c.g[V0][0] as u32);
        c.c_lt_s(1, 0);
        let within_max = !c.fcc();
        c.g[S1][0] = 1;
        if !within_max {
            c.cycles += 1;
            c.swc1(1, c.g[V0][0] as u32);
        }
        c.cycles += 10;
        let a = oc1567_p2d(c, 0x24);
        c.g[V0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.lwc1(1, oc1567_p2d(c, 0x2c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, c.g[V0][0] as u32);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.swc1(1, c.g[V0][0] as u32);
        }
        // Against the next point: x <= next.x - step.
        c.cycles += 6;
        let a = oc1567_p2d(c, 0x18);
        c.g[V0][0] = Addr(a).lw(c);
        let a = oc1567_p2d(c, 0x24);
        c.g[A0][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let at_last = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
        if !at_last {
            c.cycles += 9;
            let a = oc1567_p2d(c, 0x28);
            c.g[V1][0] = Addr(a).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(8));
            c.lwc1(1, c.g[V0][0] as u32);
            c.sub_s(0, 0, 21);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.swc1(0, c.g[V0][0] as u32);
            }
        }
        // Against the previous point: x >= prev.x + step.
        c.cycles += 3;
        let a = oc1567_p2d(c, 0x24);
        c.g[V0][0] = Addr(a).lw(c);
        let at_first = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        if at_first {
            break 'x_drag;
        }
        c.cycles += 9;
        let a = oc1567_p2d(c, 0x28);
        c.g[V1][0] = Addr(a).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(8));
        c.lwc1(1, c.g[V0][0] as u32);
        c.add_s(0, 0, 21);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(0, c.g[V0][0] as u32);
        } else {
            c.cycles += 2;
        }
    }
    Ok(())
}

/// `void MenuSysGroupItem_OnBlur(group)` (0x2cfef8): blurs the group's base
/// item (`MenuSysItem_OnBlur(group)`), then deactivates the page embedded at
/// +0x18 (`MenuSysPage_OnDeactivate(group + 0x18)`).
pub fn menu_sys_group_item_on_blur(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a370,
            f: ee::s09::MenuSysItem_OnBlur,
        }
        .call(c, 0x002c_ff0c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        Callee {
            addr: 0x002c_bfd0,
            f: ee::s09::MenuSysPage_OnDeactivate,
        }
        .call(c, 0x002c_ff14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cARGBMenuItem_cARGBMenuItem(item, name, colour, adjust)` (0x2d02b0):
/// the colour-item constructor. Runs the group base on the item
/// (`MenuSysGroupItem_ctor_2(item, name, item + 0x640)`), builds the four
/// channel items at +0x5b0, +0x5d4, +0x5f8 and +0x61c (channels 0 to 3) and
/// the colour swatch at +0x640, stores the vtable (0x486908) at +0x10 and the
/// adjust pointer at +0x5ac, then adds the four sub-items to the group.
/// Returns `item`.
pub fn menu_sys_color_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 72;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        frame.save(c, S7, 0x10);
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x640));
        frame.save(c, S1, 0x70);
        c.g[S7][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[A2][0] = c.g[S5][0];
        frame.save(c, S6, 0x20);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b0));
        frame.save_ra(c, 0);
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5d4));
        Callee {
            addr: 0x002c_f860,
            f: ee::s09::MenuSysGroupItem_ctor_2,
        }
        .call(c, 0x002d_02f8)?;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5f8));
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5ac), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6908));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A2][0] = 0u64;
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x61c));
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0320)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x1;
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0330)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x2;
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0340)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x3;
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0350)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S1][0];
        Callee {
            addr: 0x002c_ff28,
            f: ee::s09::MenuSysColorSwatchItem_ctor,
        }
        .call(c, 0x002d_0360)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0370)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0380)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0390)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_03a0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x70);
        frame.restore(c, S0, 0x80);
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

/// `MenuSysColorItem_HandleInput(item)` (0x2d03d0): forwards to the group
/// handler at 0x2cfa08 (`MenuSysGroupItem_HandleInput(item)`), returns what it
/// returns.
pub fn menu_sys_color_item_handle_input(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x002c_fa08,
            f: ee::s09::MenuSysGroupItem_HandleInput,
        }),
        0x002d_03e0,
    )
}

/// `void* cVectorTitleMenuItem_cVectorTitleMenuItem(item, caption, vector)`
/// (0x2d03f0): the vector-title item constructor. Runs the item base with
/// id -1, then stores the vector pointer at +0x14, the caption pointer at
/// +0x18 and the vtable (0x4868b8) at +0x10. Returns `item`.
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
        c.g[A1][0] = u64::MAX;
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002d_0418)?;
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

/// `void* cVectorMenuItem_cVectorMenuItem(item, name, values, caption, minValue,
/// maxValue)` (0x2d05a8): the vector item constructor. Runs the group base on
/// the item (`MenuSysGroupItem_ctor_2(item, name, item + 0x654)`), stores the
/// vtable (0x486858) at +0x10 and the values pointer at +0x5ac, builds three
/// float editors for the components (`values[0..2]`) at +0x5b8, +0x5ec and
/// +0x620, the vector label (caption) at +0x654, adds the four sub-items to
/// the group and clears the active flag at +0x670. The bounds come in f12
/// (minValue) and f13 (maxValue) and are kept in f20 and f21.
pub fn menu_sys_vector_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S5, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x654));
        frame.save_fpr(c, 21, 0x88);
        c.g[S6][0] = c.g[A3][0];
        frame.save_fpr(c, 20, 0x80);
        c.mov_s(21, 13);
        frame.save(c, S2, 0x50);
        c.mov_s(20, 12);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        c.g[A2][0] = c.g[S5][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b8));
        Callee {
            addr: 0x002c_f860,
            f: ee::s09::MenuSysGroupItem_ctor_2,
        }
        .call(c, 0x002d_05f8)?;
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ec));
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5ac), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6858));
        c.g[A3][0] = sx(0x004a_39d8);
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x620));
        Callee {
            addr: 0x002c_dce8,
            f: ee::s09::MenuSysFloatItem_ctor_2,
        }
        .call(c, 0x002d_0630)?;
        c.g[A3][0] = sx(0x004a_39e0);
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A1][0] = 0x1;
        Callee {
            addr: 0x002c_dce8,
            f: ee::s09::MenuSysFloatItem_ctor_2,
        }
        .call(c, 0x002d_0650)?;
        c.g[A3][0] = sx(0x004a_39e8);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x2;
        Callee {
            addr: 0x002c_dce8,
            f: ee::s09::MenuSysFloatItem_ctor_2,
        }
        .call(c, 0x002d_0670)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = c.g[S1][0];
        Callee {
            addr: 0x002d_03f0,
            f: ee::s09::MenuSysVectorLabelItem_ctor,
        }
        .call(c, 0x002d_0680)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0690)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_06a0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_06b0)?;
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x670), 0u64);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void* cGraphMenuItem_cGraphMenuItem(item, numPoints, xLockBits, yLockBits,
/// slope, points, minPoint, maxPoint)` (0x2d0808): the graph item constructor.
/// Runs the item base with id -1, stores the vtable (0x486808) and the graph
/// fields (point count, lock bits, slope, points pointer), copies the two
/// 8-byte bound vectors (minPoint, maxPoint) into +0x2c and +0x34 with the
/// unaligned `ldl/ldr/sdl/sdr` pairs, and clears +0x24 and +0x40. Returns `item`.
pub fn menu_sys_point2d_item_ctor(c: &mut Cpu) {
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
        c.g[A1][0] = u64::MAX;
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002d_0858)?;
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
        c.ldl(V1, (c.g[S5][0] as u32).wrapping_add(0x7));
        c.ldr(V1, c.g[S5][0] as u32);
        c.sdl(V1, (c.g[S0][0] as u32).wrapping_add(0x33));
        c.sdr(V1, (c.g[S0][0] as u32).wrapping_add(0x2c));
        c.ldl(V1, (c.g[S6][0] as u32).wrapping_add(0x7));
        c.ldr(V1, c.g[S6][0] as u32);
        c.sdl(V1, (c.g[S0][0] as u32).wrapping_add(0x3b));
        c.sdr(V1, (c.g[S0][0] as u32).wrapping_add(0x34));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
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

/// `void* cErrorMenu_cErrorMenu(page, name, ...)` (0x2d1740): the error-menu
/// constructor. Builds the page (`MenuSysPage_ctor(page)`), the title label at
/// +0x130 (`MenuSysLabelItem_ctor`, vtable 0x4867b0 stored at +0x12c), a spacer
/// of 10, the back item at +0x164, and inserts the three items into the page.
/// Returns `page`.
pub fn menu_sys_message_page_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        frame.save_ra(c, 0);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::MenuSysPage_ctor,
        }
        .call(c, 0x002d_1768)?;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x67b0));
        c.g[A1][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        Callee {
            addr: 0x002c_d2a0,
            f: ee::s09::MenuSysLabelItem_ctor,
        }
        .call(c, 0x002d_1784)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0xa;
        Callee {
            addr: 0x002c_d350,
            f: ee::s09::MenuSysSpacerItem_ctor,
        }
        .call(c, 0x002d_1790)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x002c_cdf0,
            f: ee::s09::MenuSysBackItem_ctor,
        }
        .call(c, 0x002d_179c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_17ac)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_17bc)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_17cc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cErrorMenu_cErrorMenu1(page, name, flag)` (0x2d17f0): the second
/// error-menu constructor, the same page as [`menu_sys_message_page_ctor`]
/// with the title label taking `name` (a1) and the back item taking `flag`
/// (a2) as its second argument. Returns `page`.
pub fn menu_sys_message_page_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::MenuSysPage_ctor,
        }
        .call(c, 0x002d_1820)?;
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[V0][0] = 0x480000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x67b0));
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        Callee {
            addr: 0x002c_d2a0,
            f: ee::s09::MenuSysLabelItem_ctor,
        }
        .call(c, 0x002d_1840)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1e;
        Callee {
            addr: 0x002c_d350,
            f: ee::s09::MenuSysSpacerItem_ctor,
        }
        .call(c, 0x002d_184c)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: 0x002c_cdf0,
            f: ee::s09::MenuSysBackItem_ctor,
        }
        .call(c, 0x002d_1858)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_1868)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_1878)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_1888)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* World_CreateDeadNode(void* a0, void* a1)` (0x002d1cc0): forwards its
/// arguments unchanged to `WorldObjFactory_DeadNode` (0x002fc2c0), called
/// through the function table, in a 0x10 frame holding `ra`.
pub fn world_create_dead_node(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(0x002f_c2c0),
        0x002d_1cd0,
    );
}

/// The forwarder every `MenuSys*Item` destructor thunk shares: its arguments
/// (the object and the deleting flag) go unchanged to `MenuSysItem_dtor`
/// (0x002ca280) in a 0x10 frame holding `ra`. `ret` is the address after the
/// call in the thunk (`back`).
pub(crate) fn menu_sys_item_dtor_forward(c: &mut Cpu, back: u32) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(crate::call::Callee {
            addr: 0x002c_a280,
            f: ee::s09::f_002ca280,
        }),
        back,
    );
}

/// Destructor thunk of the bool menu item (0x002d3a68): `MenuSysItem_dtor`.
pub fn menu_sys_bool_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3a78);
}

/// Destructor thunk of the enum menu item (0x002d3a98): `MenuSysItem_dtor`.
pub fn menu_sys_enum_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3aa8);
}

/// Destructor thunk of the submenu item (0x002d3ae0): `MenuSysItem_dtor`.
pub fn menu_sys_submenu_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3af0);
}

/// Destructor thunk of the back item (0x002d3b10): `MenuSysItem_dtor`.
pub fn menu_sys_back_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3b20);
}

/// Destructor thunk of the action item (0x002d3b38): `MenuSysItem_dtor`.
pub fn menu_sys_action_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3b48);
}

/// Destructor thunk of the callback item (0x002d3b80): `MenuSysItem_dtor`.
pub fn menu_sys_callback_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3b90);
}

/// `MenuSysItem_dtor` (0x2ca280): the base item destructor that the eight
/// item destructors below forward to.
pub(crate) const OC1572_MENU_SYS_ITEM_DTOR: Callee = Callee {
    addr: 0x002c_a280,
    f: ee::s09::f_002ca280,
};

/// `MenuSysLabelItem_dtor` (0x2d3bb0): a forwarding wrapper. `a0` and `a1`
/// pass through unchanged to `MenuSysItem_dtor` (7 cycles, a 0x10 frame
/// holding `ra`).
pub fn menu_sys_label_item_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC1572_MENU_SYS_ITEM_DTOR),
        0x002d_3bc0,
    )
}

/// `MenuSysSpacerItem_dtor` (0x2d3be0): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_spacer_item_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC1572_MENU_SYS_ITEM_DTOR),
        0x002d_3bf0,
    )
}

/// `MenuSysInt8Item_dtor` (0x2d3c00): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_int8_item_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC1572_MENU_SYS_ITEM_DTOR),
        0x002d_3c10,
    )
}

/// `MenuSysIntItem_dtor` (0x2d3c30): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_int_item_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC1572_MENU_SYS_ITEM_DTOR),
        0x002d_3c40,
    )
}

/// `MenuSysFloatItem_dtor` (0x2d3c60): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_float_item_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC1572_MENU_SYS_ITEM_DTOR),
        0x002d_3c70,
    )
}

/// `MenuSysFloatRangeItem_dtor` (0x2d3ce0): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_float_range_item_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC1572_MENU_SYS_ITEM_DTOR),
        0x002d_3cf0,
    )
}

/// `MenuSysTextEditItem_dtor` (0x2d3d20): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_text_edit_item_dtor(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC1572_MENU_SYS_ITEM_DTOR),
        0x002d_3d30,
    )
}

/// `MenuSysTextEditItem_SetText(MenuSysTextEditItem* item, const char* text)`
/// (0x2d3d48): clear the first byte of the item's text buffer at `+0x18`, then
/// `strncat(buffer, text, 15)`. Returns what `strncat` returned.
pub fn menu_sys_text_edit_item_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let buffer = (c.g[V0][0] as u32).wrapping_add(0x18);
        c.g[A0][0] = sx(buffer);
        c.g[A2][0] = 0xf;
        Addr(buffer).sb(c, 0);
        call_ptr(c, 0x0041_6958, 0x002d_3d64)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysItem_dtor` (0x2ca280): the base item destructor, called on each
/// sub-object and on `this`.
pub(crate) const MENU_ITEM_DTOR: Callee = Callee {
    addr: 0x002c_a280,
    f: ee::s09::f_002ca280,
};

/// `MenuSysPage_dtor` (0x2caa80): the page member of the group items.
pub(crate) const MENU_PAGE_DTOR: Callee = Callee {
    addr: 0x002c_aa80,
    f: ee::s09::f_002caa80,
};

/// `MenuSysColorChannelItem_dtor` (0x2d3d70): forwards to `MenuSysItem_dtor`
/// with the same `this` and flags, returning to 0x2d3d80.
pub fn menu_sys_color_channel_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3d80);
}

/// `MenuSysGroupItem_PageDtor` (0x2d3d90): forwards to `MenuSysPage_dtor`
/// with the same `this` and flags, returning to 0x2d3da0.
pub fn menu_sys_group_item_page_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_PAGE_DTOR), 0x002d_3da0);
}

/// `MenuSysColorSwatchItem_dtor` (0x2d3df8): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_color_swatch_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3e08);
}

/// `MenuSysVectorLabelItem_dtor` (0x2d3ea8): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_vector_label_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3eb8);
}

/// `MenuSysPoint2DItem_dtor` (0x2d3f50): forwards to `MenuSysItem_dtor`.
pub fn menu_sys_point2d_item_dtor(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(MENU_ITEM_DTOR), 0x002d_3f60);
}

/// The composite item destructors (`MenuSysGroupItem`, `MenuSysColorItem`,
/// `MenuSysVectorItem`): a 0x30 frame holding `this` (`s0`) and the flags
/// (`s1`), then the member destructors, each called with flags 2 (no delete)
/// on `this + off`, and finally `base_dtor(this, flags)` on the object itself.
/// `subs` lists `(offset, destructor, return address)` in call order; the
/// last call returns to `last_ret`.
pub(crate) fn menu_sys_composite_dtor(
    c: &mut Cpu,
    cycles: u64,
    subs: &[(u32, Callee, u32)],
    last_ret: u32,
) -> Flow {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    for &(off, dtor, ret) in subs {
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
        c.g[A1][0] = 0x2;
        dtor.call(c, ret)?;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    MENU_ITEM_DTOR.call(c, last_ret)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MenuSysGroupItem_dtor` (0x2d3db0): the page member at `+0x18`, then the
/// item itself.
pub fn menu_sys_group_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_composite_dtor(c, 17, &[(0x18, MENU_PAGE_DTOR, 0x002d_3dd4)], 0x002d_3de0)
    });
}

/// `MenuSysColorItem_dtor` (0x2d3e20): five item members at `+0x5b0..+0x640`,
/// the page member at `+0x18`, then the item itself.
pub fn menu_sys_color_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_composite_dtor(
            c,
            32,
            &[
                (0x640, MENU_ITEM_DTOR, 0x002d_3e44),
                (0x61c, MENU_ITEM_DTOR, 0x002d_3e50),
                (0x5f8, MENU_ITEM_DTOR, 0x002d_3e5c),
                (0x5d4, MENU_ITEM_DTOR, 0x002d_3e68),
                (0x5b0, MENU_ITEM_DTOR, 0x002d_3e74),
                (0x18, MENU_PAGE_DTOR, 0x002d_3e80),
            ],
            0x002d_3e8c,
        )
    });
}

/// `MenuSysVectorItem_dtor` (0x2d3ed0): four item members at `+0x5b8..+0x654`,
/// the page member at `+0x18`, then the item itself.
pub fn menu_sys_vector_item_dtor(c: &mut Cpu) {
    guest(c, |c| {
        menu_sys_composite_dtor(
            c,
            29,
            &[
                (0x654, MENU_ITEM_DTOR, 0x002d_3ef4),
                (0x620, MENU_ITEM_DTOR, 0x002d_3f00),
                (0x5ec, MENU_ITEM_DTOR, 0x002d_3f0c),
                (0x5b8, MENU_ITEM_DTOR, 0x002d_3f18),
                (0x18, MENU_PAGE_DTOR, 0x002d_3f24),
            ],
            0x002d_3f30,
        )
    });
}

/// `MenuSysMessagePage` destructor body (0x2d3f78): destroys the three item
/// members at +0x164, +0x14c and +0x130 (`MenuSysItem_dtor(item, 2)`), then
/// the base page through `MenuSysPage_dtor(this, flags)`.
pub fn menu_sys_message_page_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret_to) in [
            (0x164u32, 0x002d_3f9c),
            (0x14c, 0x002d_3fa8),
            (0x130, 0x002d_3fb4),
        ] {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
            c.g[A1][0] = 0x2;
            Callee {
                addr: 0x002c_a280,
                f: ee::s09::MenuSysItem_dtor,
            }
            .call(c, ret_to)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        Callee {
            addr: 0x002c_aa80,
            f: ee::s09::MenuSysPage_dtor,
        }
        .call(c, 0x002d_3fc0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysPage_SortCompare(a, b)` (0x2d4008): calls the comparator stored in
/// the global at `gp + 0x23cc` on the two items `*a` and `*b`.
pub fn menu_sys_page_sort_compare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x23cc)).lw(c);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
        call_ptr(c, target, 0x002d_4020)?;
        frame.restore_ra(c, 0);
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

    use crate::difftest;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::check_reaching;
    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as TEST_SP;
    pub(crate) use crate::game::seg::seg_1dcd10::tests::OC1574_MAGIC;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::OC1567_CTRL;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::OC1567_MAGIC;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::OC1567_RET;
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn menu_sys_ctor_matches() {
        check(
            ee::s09::f_002cc018__generated,
            menu_sys_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cc0_18 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    pub(crate) fn leaf_val(r: &mut Rng) -> u64 {
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
    pub(crate) fn leaf_fill(
        c: &mut Cpu,
        r: &mut Rng,
        start: u32,
        size: u32,
        chase: bool,
        ptr_lo: u32,
    ) {
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

    /// `MenuSysItem_OnFocus` (0x002c_a368) against the generated body.
    #[test]
    fn menu_sys_item_on_focus_matches() {
        check(
            ee::s09::f_002ca368__generated,
            menu_sys_item_on_focus,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca368, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysItem_OnBlur` (0x002c_a370) against the generated body.
    #[test]
    fn menu_sys_item_on_blur_matches() {
        check(
            ee::s09::f_002ca370__generated,
            menu_sys_item_on_blur,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca370, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysPage_OnAction` (0x002c_ad48) against the generated body.
    #[test]
    fn menu_sys_page_on_action_002cad48_matches() {
        check(
            ee::s09::f_002cad48__generated,
            menu_sys_page_on_action_002cad48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cad48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysPage_v04` (0x002c_ad50) against the generated body.
    #[test]
    fn menu_sys_page_v04_002cad50_matches() {
        check(
            ee::s09::f_002cad50__generated,
            menu_sys_page_v04_002cad50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cad50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_SetActive` (0x002c_c0b8) against the generated body.
    #[test]
    fn menu_sys_set_active_matches() {
        check(
            ee::s09::f_002cc0b8__generated,
            menu_sys_set_active,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc0b8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_GetInputCode` (0x002c_c248) against the generated body.
    #[test]
    fn menu_sys_get_input_code_matches() {
        check(
            ee::s09::f_002cc248__generated,
            menu_sys_get_input_code,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc248, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_GetKeyCode` (0x002c_c250) against the generated body.
    #[test]
    fn menu_sys_get_key_code_matches() {
        check(
            ee::s09::f_002cc250__generated,
            menu_sys_get_key_code,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc250, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_GetKeyChar` (0x002c_c258) against the generated body.
    #[test]
    fn menu_sys_get_key_char_matches() {
        check(
            ee::s09::f_002cc258__generated,
            menu_sys_get_key_char,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc258, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysLabelItem_IsSelectable` (0x002c_d2e8) against the generated body.
    #[test]
    fn menu_sys_label_item_is_selectable_002cd2e8_matches() {
        check(
            ee::s09::f_002cd2e8__generated,
            menu_sys_label_item_is_selectable_002cd2e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cd2e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysSpacerItem_IsSelectable` (0x002c_d398) against the generated body.
    #[test]
    fn menu_sys_spacer_item_is_selectable_002cd398_matches() {
        check(
            ee::s09::f_002cd398__generated,
            menu_sys_spacer_item_is_selectable_002cd398,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cd398, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysSpacerItem_Draw` (0x002c_d3f0) against the generated body.
    #[test]
    fn menu_sys_spacer_item_draw_002cd3f0_matches() {
        check(
            ee::s09::f_002cd3f0__generated,
            menu_sys_spacer_item_draw_002cd3f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cd3f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysColorSwatchItem_IsSelectable` (0x002c_ff80) against the generated body.
    #[test]
    fn menu_sys_color_swatch_item_is_selectable_002cff80_matches() {
        check(
            ee::s09::f_002cff80__generated,
            menu_sys_color_swatch_item_is_selectable_002cff80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cff80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysVectorLabelItem_IsSelectable` (0x002d_0448) against the generated body.
    #[test]
    fn menu_sys_vector_label_item_is_selectable_002d0448_matches() {
        check(
            ee::s09::f_002d0448__generated,
            menu_sys_vector_label_item_is_selectable_002d0448,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d0448, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysItem_ResetValue` (0x002d_39f8) against the generated body.
    #[test]
    fn menu_sys_item_reset_value_002d39f8_matches() {
        check(
            ee::s09::f_002d39f8__generated,
            menu_sys_item_reset_value_002d39f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d39f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_DebugPrintf_4080` (0x002d_4080) against the generated body.
    #[test]
    fn menu_sys_debug_printf_4080_matches() {
        check(
            ee::s09::f_002d4080__generated,
            menu_sys_debug_printf_4080,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d4080, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// Gate of this module's callee fakes: the magic word at `OC1525_CTRL`.
    pub(crate) const OC1525_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const OC1525_MAGIC: u32 = 0x1525_c0de;

    /// Gated fakes of the callees (hooked once per process; with the gate off
    /// they run the body they replaced).
    macro_rules! oc1525_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1525_CTRL) != OC1525_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr)
                }
            )*
            fn oc1525_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1525_fakes! {
        oc1525_fake_2cbf30, OC1525_PREV_2CBF30, 0x002c_bf30, ee::s09::OVERRIDE_002CBF30, ee::s09::f_002cbf30__generated;
        oc1525_fake_391cb0, OC1525_PREV_391CB0, 0x0039_1cb0, ee::s12::OVERRIDE_00391CB0, ee::s12::f_00391cb0__generated;
        oc1525_fake_391fb0, OC1525_PREV_391FB0, 0x0039_1fb0, ee::s12::OVERRIDE_00391FB0, ee::s12::f_00391fb0__generated;
        oc1525_fake_413af8, OC1525_PREV_413AF8, 0x0041_3af8, ee::s15::OVERRIDE_00413AF8, ee::s15::f_00413af8__generated;
        oc1525_fake_417828, OC1525_PREV_417828, 0x0041_7828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
    }

    /// A float in `[0, 1]` as raw bits.
    pub(crate) fn oc1525_unit(r: &mut crate::difftest::Rng) -> u32 {
        ((r.next_u32() % 1000) as f32 / 999.0).to_bits()
    }

    /// `MenuSysColorChannelItem_Draw` against the generated body: the page
    /// vtable call, both colour choices (selected row or not), the highlight
    /// flag of the item (value 1.0 or not), the four channels and every
    /// callee (fakes) unwinding at each call.
    #[test]
    fn menu_sys_color_channel_item_draw_matches() {
        covered(
            ee::s09::f_002cf1c8__generated,
            menu_sys_color_channel_item_draw,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cf1c8, 11);
                oc1525_install_fakes();
                c.bus.write32(OC1525_CTRL, OC1525_MAGIC);
                let this = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let vt = DATA + 0x3000;
                let rect = DATA + 0x4000;
                let vals = DATA + 0x5000;
                let font = DATA + 0x6000;
                let hold = DATA + 0x7000;
                let gfx = DATA + 0x8000;
                let target = DATA + 0x9000;
                let sys = DATA + 0xa000;
                let idx = seed % 4;
                c.bus.write32(this, menu);
                c.bus.write32(this + 0x14, vals);
                c.bus.write32(this + 0x18, idx);
                c.bus.write32(menu + 0x12c, vt);
                c.bus.write32(menu + 0x124, hold);
                c.bus.write32(hold + 0x58, font);
                c.bus.write16(vt + 0x48, 0x10);
                c.bus.write32(vt + 0x4c, fakes::FAKE + 0x10);
                for i in 0..4 {
                    let v = if i == 0 && (seed / 4) % 2 == 0 {
                        1.0f32.to_bits()
                    } else {
                        oc1525_unit(&mut r)
                    };
                    c.bus.write32(vals + 4 * i, v);
                    let v = ((r.next_u32() % 4000) as f32 / 10.0).to_bits();
                    c.bus.write32(rect + 4 * i, v);
                }
                for i in 0..3 {
                    let v = oc1525_unit(&mut r);
                    c.bus.write32(0x004d_53a0 + 4 * i, v);
                    let v = oc1525_unit(&mut r);
                    c.bus.write32(0x004d_53b0 + 4 * i, v);
                }
                let v = oc1525_unit(&mut r);
                c.bus.write32(font + 0x38, v);
                let v = oc1525_unit(&mut r);
                c.bus.write32(font + 0x3c, v);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), gfx);
                c.bus.write32(gfx + 0xe84, target);
                c.bus.write32(gfx + 0x10d8, sys);
                c.bus.write16(sys + 0x238, 0x20);
                c.bus.write32(sys + 0x23c, fakes::FAKE + 0x20);
                // Calls 0 and 2 are MenuSysPage_GetSelectedItem: the item
                // itself or a pointer from the fake's hash.
                if seed % 2 == 0 {
                    c.bus.write32(fakes::LOG_RESULTS, this);
                }
                if (seed / 2) % 2 == 0 {
                    c.bus.write32(fakes::LOG_RESULTS + 8, this);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = rect as u64;
            },
        );
    }

    /// Gate of this module's text-helper fakes: the magic word at `OC1532_CTRL`.
    /// The helpers `0x391cb0` / `0x391fb0` are real functions, so without the
    /// fakes their calls would run the generated body and never be logged.
    pub(crate) const OC1532_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const OC1532_MAGIC: u32 = 0x1532_c0de;

    pub(crate) static OC1532_PREV_391CB0: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1532_PREV_391FB0: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1532_fake_391cb0(c: &mut Cpu) {
        if c.bus.read32(OC1532_CTRL) != OC1532_MAGIC {
            return fakes::fall_back(c, &OC1532_PREV_391CB0, ee::s12::f_00391cb0__generated);
        }
        fakes::fake_call(c, 0x0039_1cb0)
    }

    pub(crate) fn oc1532_fake_391fb0(c: &mut Cpu) {
        if c.bus.read32(OC1532_CTRL) != OC1532_MAGIC {
            return fakes::fall_back(c, &OC1532_PREV_391FB0, ee::s12::f_00391fb0__generated);
        }
        fakes::fake_call(c, 0x0039_1fb0)
    }

    pub(crate) fn oc1532_install_fakes(c: &mut Cpu) {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391CB0),
            oc1532_fake_391cb0,
            &OC1532_PREV_391CB0,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391FB0),
            oc1532_fake_391fb0,
            &OC1532_PREV_391FB0,
        );
        c.bus.write32(OC1532_CTRL, OC1532_MAGIC);
    }

    /// Base of the fixture structures (clear of the fakes' random pointers
    /// and of their call log).
    pub(crate) const OC1532_BASE: u32 = DATA + 0x20_0000;

    pub(crate) fn oc1532_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Menu -> vtable -> font chain every row function walks:
    /// `item -> menu; menu+0x124 -> vt; vt+0x58 -> font`.
    pub(crate) fn oc1532_font_chain(c: &mut Cpu, r: &mut Rng, item: u32) -> (u32, u32) {
        let menu = OC1532_BASE + 0x300;
        let vt = OC1532_BASE + 0x400;
        let font = OC1532_BASE + 0x500;
        oc1532_w(c, item, menu);
        oc1532_w(c, menu + 0x124, vt);
        oc1532_w(c, vt + 0x58, font);
        oc1532_w(c, font + 0x14, 1 + r.next_u32() % 5);
        oc1532_w(c, font + 0x30, 0x3f80_0000);
        oc1532_w(c, font + 0x34, 0x4000_0000);
        oc1532_w(c, font + 0x38, r.f32(40.0));
        oc1532_w(c, font + 0x3c, r.f32(40.0));
        (menu, font)
    }

    /// Row measure: the three optional strings (`seed % 8` picks which exist)
    /// each go through the text helper; the fake helper's random float
    /// results decide the max and the layout.
    #[test]
    fn menu_sys_item_measure_row_matches() {
        check_reaching(
            ee::s09::f_002ca4c8__generated,
            menu_sys_item_measure_row,
            240,
            &[
                0x002c_a57c,
                0x002c_a5a4,
                0x002c_a5cc,
                0x002c_a608,
                0x002c_a644,
                0x002c_a67c,
                0x002c_a6a8,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca4c8, 8);
                oc1532_install_fakes(c);
                let item = OC1532_BASE + 0x200;
                oc1532_font_chain(c, &mut r, item);
                let s = OC1532_BASE + 0x600;
                let pick = |bit: u32| if (seed >> bit) & 1 != 0 { s } else { 0 };
                c.g[A0][0] = sx(OC1532_BASE + 0x100);
                c.g[A1][0] = sx(item);
                c.g[A2][0] = sx(pick(0));
                c.g[A3][0] = sx(pick(1));
                c.g[T0][0] = sx(pick(2));
            },
        );
    }

    /// Row text draw: sets the font colour/scale, then draws the left,
    /// centre and right strings (`seed % 8`) through the helpers.
    #[test]
    fn menu_sys_item_draw_row_text_matches() {
        check_reaching(
            ee::s09::f_002ca768__generated,
            menu_sys_item_draw_row_text,
            240,
            &[
                0x002c_a7bc,
                0x002c_a858,
                0x002c_a880,
                0x002c_a8cc,
                0x002c_a8f4,
                0x002c_a940,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ca768, 8);
                oc1532_install_fakes(c);
                let item = OC1532_BASE + 0x200;
                let (menu, _font) = oc1532_font_chain(c, &mut r, item);
                let vt = vtable(c, &mut r, &[0x40]);
                oc1532_w(c, menu + 0x12c, vt);
                let rect = OC1532_BASE + 0x600;
                let color = OC1532_BASE + 0x680;
                for i in 0..4 {
                    oc1532_w(c, rect + 4 * i, r.f32(300.0));
                    oc1532_w(c, color + 4 * i, r.next_u32());
                }
                let s = OC1532_BASE + 0x700;
                let pick = |bit: u32| if (seed >> bit) & 1 != 0 { s } else { 0 };
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(rect);
                c.g[A2][0] = sx(pick(0));
                c.g[A3][0] = sx(pick(1));
                c.g[T0][0] = sx(pick(2));
                c.g[T2][0] = sx(color);
            },
        );
    }

    /// Page draw: lay out the visible items of a page (`count` items at
    /// `+0xc`, scroll index at `+8`), draw them through their vtable and add
    /// the scroll arrows.
    #[test]
    fn menu_sys_page_draw_matches() {
        check_reaching(
            ee::s09::f_002cad58__generated,
            menu_sys_page_draw,
            300,
            &[0x002c_adf0, 0x002c_b0b4],
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cad58, 6);
                let page = OC1532_BASE + 0x200;
                let menu = OC1532_BASE + 0x300;
                let count = seed % 5;
                oc1532_w(c, page, count);
                let first = if count > 0 { (seed / 5) % count } else { 0 };
                oc1532_w(c, page + 4, first);
                oc1532_w(
                    c,
                    page + 8,
                    if count > 0 {
                        (seed / 25) % (count + 1)
                    } else {
                        0
                    },
                );
                oc1532_w(c, page + 0x124, menu);
                oc1532_w(c, menu + 0x64, (seed * 37) % 500);
                oc1532_w(c, menu + 0x60, (seed * 11) % 50);
                let ivt = vtable(c, &mut r, &[0x20, 0x28]);
                for i in 0..4u32 {
                    let item = OC1532_BASE + 0x1000 + 0x100 * i;
                    oc1532_w(c, page + 0xc + 4 * i, item);
                    // Item +4 bit 0 is its visible flag.
                    oc1532_w(c, item + 4, ((seed >> (i + 3)) ^ i) & 1);
                    oc1532_w(c, item + 0x10, ivt);
                }
                // The measure slot is a fake, so the item rect it "returns"
                // is whatever the stack holds: fill it with floats.
                let sp0 = c.g[SP][0] as u32;
                for i in 0..4u32 {
                    oc1532_w(c, sp0.wrapping_sub(0x70) + 4 * i, r.f32(200.0));
                }
                c.g[A0][0] = sx(page);
            },
        );
    }

    /// Frame draw: nine `MenuSys_BuildQuad` pieces (corners, edges, centre)
    /// of a menu box from a rect and a colour.
    #[test]
    fn menu_sys_page_draw_frame_matches() {
        check(
            ee::s09::f_002cb498__generated,
            menu_sys_page_draw_frame,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cb498, 0);
                let page = OC1532_BASE + 0x200;
                let menu = OC1532_BASE + 0x300;
                let grman = OC1532_BASE + 0x400;
                let state = OC1532_BASE + 0x500;
                let slot = OC1532_BASE + 0x6000;
                let rect = OC1532_BASE + 0x700;
                let color = OC1532_BASE + 0x740;
                oc1532_w(c, page + 0x124, menu);
                oc1532_w(c, menu + 0x60, r.next_u32() % 64);
                oc1532_w(c, GAME_GP.wrapping_sub(0x854), grman);
                oc1532_w(c, grman + 0xe84, state);
                oc1532_w(c, state + 0x4, r.next_u32());
                oc1532_w(c, state + 0x8, r.next_u32());
                // BuildQuad's submit slot: *(grman+0x10d8)+0x23c, a fake target.
                oc1532_w(c, grman + 0x10d8, slot);
                oc1532_w(c, slot + 0x238, 0x100);
                oc1532_w(c, slot + 0x23c, 0x0012_3450);
                for i in 0..4 {
                    oc1532_w(c, rect + 4 * i, r.f32(300.0));
                    oc1532_w(
                        c,
                        color + 4 * i,
                        r.f32(if seed % 3 == 0 { 3.0 } else { 0.9 }),
                    );
                }
                c.g[A0][0] = sx(page);
                c.g[A1][0] = sx(rect);
                c.g[A2][0] = sx(color);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// The item-callee fakes' gate word, the per-seed salt behind it, and the
    /// fixture addresses (all inside the test data area).
    pub(crate) const OC1533_CTRL: u32 = DATA + 0xe_0000;

    pub(crate) const OC1533_MAGIC: u32 = 0x1533_a5c3;

    pub(crate) const OC1533_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC1533_RECT: u32 = DATA + 0x200;

    pub(crate) const OC1533_MENU: u32 = DATA + 0x2000;

    pub(crate) const OC1533_VTABLE: u32 = DATA + 0x2800;

    pub(crate) const OC1533_WORLD: u32 = DATA + 0x3000;

    pub(crate) const OC1533_ENTRIES: u32 = DATA + 0x6000;

    pub(crate) const OC1533_VALUE: u32 = DATA + 0x7000;

    pub(crate) const OC1533_TARGET: u32 = DATA + 0x8000;

    pub(crate) const OC1533_HEAD: u32 = DATA + 0x9000;

    /// What a fake item callee answers (see `oc1533_fake_body`).
    pub(crate) const KIND_HANDLE_INPUT: u32 = 0;

    pub(crate) const KIND_IS_DISABLED: u32 = 1;

    pub(crate) const KIND_INPUT_CODE: u32 = 2;

    pub(crate) const KIND_AXIS: u32 = 3;

    pub(crate) const KIND_MOD_HELD: u32 = 4;

    pub(crate) static OC1533_PREV_HANDLE_INPUT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1533_PREV_IS_DISABLED: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1533_PREV_INPUT_CODE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1533_PREV_LEFT_AXIS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1533_PREV_RIGHT_AXIS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1533_PREV_SLOW_MOD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1533_PREV_FINE_MOD: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake of one `MenuSysItem_*` callee: logs the call like every
    /// fake, then answers from a hash of the per-seed salt and the call
    /// number (`v0` for the codes and flags, `f0` for the axes). With the
    /// gate off it runs the body the slot held before.
    pub(crate) fn oc1533_fake_body(
        c: &mut Cpu,
        prev: &AtomicUsize,
        addr: u32,
        generated: fn(&mut Cpu),
        kind: u32,
    ) {
        if c.bus.read32(OC1533_CTRL) != OC1533_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = r32(c, RA);
        let n = c.bus.read32(fakes::LOG_COUNT);
        let salt = c.bus.read32(OC1533_CTRL + 4);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let h = fakes::mix(salt as u64 ^ (n as u64) << 32 ^ addr as u64);
        let pick = (h >> 8) as usize;
        match kind {
            KIND_HANDLE_INPUT => {
                let v = if h % 4 == 0 { 1 + (pick % 7) as u32 } else { 0 };
                c.g[V0][0] = sx(v);
            }
            KIND_IS_DISABLED => {
                c.g[V0][0] = if h % 6 == 0 { 8 } else { 0 };
            }
            KIND_INPUT_CODE => {
                const CODES: [u32; 10] = [0, 1, 2, 3, 6, 2, 3, 1, 0, 2];
                c.g[V0][0] = sx(CODES[pick % 10]);
            }
            KIND_AXIS => {
                const AXES: [f32; 10] = [0.0, 0.05, -0.05, 0.1, -0.1, 0.3, -0.3, 1.0, -1.0, 0.12];
                c.f[0] = AXES[pick % 10].to_bits();
            }
            _ => {
                c.g[V0][0] = sx((pick % 2) as u32);
            }
        }
    }

    pub(crate) fn oc1533_fake_handle_input(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_HANDLE_INPUT,
            0x002c_a2b0,
            ee::s09::f_002ca2b0__generated,
            KIND_HANDLE_INPUT,
        )
    }

    pub(crate) fn oc1533_fake_is_disabled(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_IS_DISABLED,
            0x002c_a3b0,
            ee::s09::f_002ca3b0__generated,
            KIND_IS_DISABLED,
        )
    }

    pub(crate) fn oc1533_fake_input_code(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_INPUT_CODE,
            0x002c_a3e8,
            ee::s09::f_002ca3e8__generated,
            KIND_INPUT_CODE,
        )
    }

    pub(crate) fn oc1533_fake_left_axis(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_LEFT_AXIS,
            0x002c_a408,
            ee::s09::f_002ca408__generated,
            KIND_AXIS,
        )
    }

    pub(crate) fn oc1533_fake_right_axis(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_RIGHT_AXIS,
            0x002c_a428,
            ee::s09::f_002ca428__generated,
            KIND_AXIS,
        )
    }

    pub(crate) fn oc1533_fake_slow_mod(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_SLOW_MOD,
            0x002c_a488,
            ee::s09::f_002ca488__generated,
            KIND_MOD_HELD,
        )
    }

    pub(crate) fn oc1533_fake_fine_mod(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_FINE_MOD,
            0x002c_a4a8,
            ee::s09::f_002ca4a8__generated,
            KIND_MOD_HELD,
        )
    }

    pub(crate) fn oc1533_hook() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA2B0),
            oc1533_fake_handle_input,
            &OC1533_PREV_HANDLE_INPUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA3B0),
            oc1533_fake_is_disabled,
            &OC1533_PREV_IS_DISABLED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA3E8),
            oc1533_fake_input_code,
            &OC1533_PREV_INPUT_CODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA408),
            oc1533_fake_left_axis,
            &OC1533_PREV_LEFT_AXIS,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA428),
            oc1533_fake_right_axis,
            &OC1533_PREV_RIGHT_AXIS,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA488),
            oc1533_fake_slow_mod,
            &OC1533_PREV_SLOW_MOD,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA4A8),
            oc1533_fake_fine_mod,
            &OC1533_PREV_FINE_MOD,
        );
    }

    /// The menu object behind `self + 0`: its vtable (at `menu + 0x12c`)
    /// holds `{delta, fn}` pairs at 0x18, 0x20, 0x40 whose functions are
    /// fake addresses (the miss hook logs those calls).
    pub(crate) fn oc1533_menu(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC1533_SELF, OC1533_MENU);
        c.bus.write32(OC1533_MENU + 0x12c, OC1533_VTABLE);
        for (i, off) in [0x18u32, 0x20, 0x40].into_iter().enumerate() {
            let delta = (r.next_u32() % 0x100) & !3;
            c.bus.write16(OC1533_VTABLE + off, delta as u16);
            c.bus
                .write32(OC1533_VTABLE + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// Fixture of the scroll arrows: `self` with the menu vtable, the world
    /// global at `gp - 0x854` (target object at `+0xe84`, head object at
    /// `+0x10d8`), the arrow colour at `0x4d53a0` and a rectangle.
    pub(crate) fn oc1533_arrow_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 3);
        oc1533_menu(c, &mut r);
        c.g[A0][0] = sx(OC1533_SELF);
        c.g[A1][0] = sx(OC1533_RECT);
        c.bus.write32(GAME_GP - 0x854, OC1533_WORLD);
        c.bus.write32(OC1533_WORLD + 0xe84, OC1533_TARGET);
        c.bus.write32(OC1533_WORLD + 0x10d8, OC1533_HEAD);
        fill(c, &mut r, OC1533_TARGET, 8);
        c.bus
            .write16(OC1533_HEAD + 0x238, (r.next_u32() % 0x100 & !3) as u16);
        c.bus.write32(OC1533_HEAD + 0x23c, fakes::FAKE + 0x30);
        for i in 0..3 {
            let v = if seed % 5 == 0 {
                r.edge_f32()
            } else {
                r.f32(1.5)
            };
            c.bus.write32(0x004d_53a0 + 4 * i, v);
        }
        for i in 0..4 {
            let v = if seed % 7 == 0 {
                r.edge_f32()
            } else {
                r.f32(300.0)
            };
            c.bus.write32(OC1533_RECT + 4 * i, v);
        }
    }

    /// `MenuSysPage_DrawScrollArrowDown`: both vtable calls and the 6-vertex
    /// loop, with random rectangles and colours.
    #[test]
    fn menu_sys_page_draw_scroll_arrow_down_matches() {
        check_reaching(
            ee::s09::f_002cb8c8__generated,
            menu_sys_page_draw_scroll_arrow_down,
            300,
            &[0x002c_b8f4, 0x002c_bab4],
            &|c, seed| oc1533_arrow_setup(c, seed, 0x2c_b8c8),
        );
    }

    /// `MenuSysPage_DrawScrollArrowUp`: the same shape with the triangle
    /// pointing the other way.
    #[test]
    fn menu_sys_page_draw_scroll_arrow_up_matches() {
        check_reaching(
            ee::s09::f_002cbac8__generated,
            menu_sys_page_draw_scroll_arrow_up,
            300,
            &[0x002c_baf4, 0x002c_bcb4],
            &|c, seed| oc1533_arrow_setup(c, seed, 0x2c_bac8),
        );
    }

    /// Common part of the item fixtures: the gated fakes on, the menu object,
    /// the gp-relative float constants (0.1 steps), `self` in `a0`.
    pub(crate) fn oc1533_item_setup(c: &mut Cpu, seed: u32, salt: u64, consts: u32) -> Rng {
        oc1533_hook();
        let mut r = machine(c, seed, salt, 9);
        c.bus.write32(OC1533_CTRL, OC1533_MAGIC);
        c.bus.write32(OC1533_CTRL + 4, r.next_u32());
        oc1533_menu(c, &mut r);
        c.g[A0][0] = sx(OC1533_SELF);
        if consts != 0 {
            c.bus.write32(GAME_GP - consts, (-0.1f32).to_bits());
            c.bus.write32(GAME_GP - consts + 4, 0.1f32.to_bits());
            c.bus.write32(GAME_GP - consts + 8, 0.1f32.to_bits());
        }
        r
    }

    /// `MenuSysEnumItem_HandleInput` (an enum chooser): count at `+0x18`,
    /// `{label, value}` entries at `+0x1c`, the edited value behind the
    /// pointer at `+0x20`. Up/down codes step to the previous/next entry (or
    /// wrap when the value matches none), code 6 selects through the menu.
    #[test]
    fn menu_sys_enum_item_handle_input_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc1533_item_setup(c, seed, 0x2c_c8f0, 0);
            let count = r.next_u32() % 5;
            c.bus.write32(OC1533_SELF + 0x18, count);
            c.bus.write32(OC1533_SELF + 0x1c, OC1533_ENTRIES);
            c.bus.write32(OC1533_SELF + 0x20, OC1533_VALUE);
            let mut values = [0u32; 5];
            for (i, v) in values.iter_mut().enumerate() {
                *v = r.next_u32() % 4;
                c.bus.write32(OC1533_ENTRIES + 8 * i as u32, r.next_u32());
                c.bus.write32(OC1533_ENTRIES + 8 * i as u32 + 4, *v);
            }
            let current = if r.next_u32() % 4 == 0 || count == 0 {
                99
            } else {
                values[(r.next_u32() % count) as usize]
            };
            c.bus.write32(OC1533_VALUE, current);
        };
        check_lohi(
            ee::s09::f_002cc8f0__generated,
            menu_sys_enum_item_handle_input,
            400,
            &setup,
        );
        check_reaching(
            ee::s09::f_002cc8f0__generated,
            menu_sys_enum_item_handle_input,
            400,
            &[
                0x002c_c918,
                0x002c_c924,
                0x002c_c93c,
                0x002c_c968,
                0x002c_c990,
                0x002c_c9a0,
                0x002c_c9c8,
                0x002c_c9d8,
                0x002c_ca24,
                0x002c_ca34,
                0x002c_ca80,
                0x002c_ca90,
                0x002c_cae4,
            ],
            &setup,
        );
    }

    /// Fixture of the two number sliders. `base` is the offset of the first
    /// field after the pointer/limits (`0x20` for the 8-bit slider, `0x24` for
    /// the word one): repeat counter, last direction, float remainder.
    pub(crate) fn oc1533_slider_setup(c: &mut Cpu, seed: u32, salt: u64, wide: bool) {
        let consts = if wide { 0x3fe4 } else { 0x3ff0 };
        let mut r = oc1533_item_setup(c, seed, salt, consts);
        c.bus.write32(OC1533_SELF + 0x18, OC1533_VALUE);
        let lo = -((r.next_u32() % 11) as i32);
        let hi = (r.next_u32() % 11) as i32;
        let cur = (r.next_u32() % 25) as i32 - 12;
        let count = [0u32, 3, 10, 30, 90, 200, 1000][(r.next_u32() % 7) as usize];
        let dir = [-1i32, 0, 1][(r.next_u32() % 3) as usize] as u32;
        let accum = r.f32(2.0);
        if wide {
            c.bus.write32(OC1533_SELF + 0x1c, lo as u32);
            c.bus.write32(OC1533_SELF + 0x20, hi as u32);
            c.bus.write32(OC1533_VALUE, cur as u32);
            c.bus.write32(OC1533_SELF + 0x24, count);
            c.bus.write32(OC1533_SELF + 0x28, dir);
            c.bus.write32(OC1533_SELF + 0x2c, accum);
        } else {
            c.bus.write8(OC1533_SELF + 0x1c, lo as u8);
            c.bus.write8(OC1533_SELF + 0x1d, hi as u8);
            c.bus.write8(OC1533_VALUE, cur as u8);
            c.bus.write32(OC1533_SELF + 0x20, count);
            c.bus.write32(OC1533_SELF + 0x24, dir);
            c.bus.write32(OC1533_SELF + 0x28, accum);
        }
    }

    pub(crate) const OC1533_SLIDER_RETS_8: [u32; 9] = [
        0x002c_d5c0,
        0x002c_d5cc,
        0x002c_d5e4,
        0x002c_d5f0,
        0x002c_d6a0,
        0x002c_d6b0,
        0x002c_d6d0,
        0x002c_d6f0,
        0x002c_d798,
    ];

    pub(crate) const OC1533_SLIDER_RETS_32: [u32; 9] = [
        0x002c_d9f8,
        0x002c_da04,
        0x002c_da1c,
        0x002c_da28,
        0x002c_dad8,
        0x002c_dae8,
        0x002c_db08,
        0x002c_db28,
        0x002c_dbd0,
    ];

    /// `MenuSysInt8Item_HandleInput` (a signed-byte slider): the stick axis
    /// difference gives a direction (dead zone 0.1), a held direction speeds
    /// up with the square of the repeat count, slow/fine modifiers divide the
    /// step; the byte is clamped to its limits and the menu is notified.
    #[test]
    fn menu_sys_int8_item_handle_input_matches() {
        let setup = |c: &mut Cpu, seed: u32| oc1533_slider_setup(c, seed, 0x2c_d5a0, false);
        check_lohi(
            ee::s09::f_002cd5a0__generated,
            menu_sys_int8_item_handle_input,
            400,
            &setup,
        );
        check_reaching(
            ee::s09::f_002cd5a0__generated,
            menu_sys_int8_item_handle_input,
            400,
            &OC1533_SLIDER_RETS_8,
            &setup,
        );
    }

    /// `MenuSysIntItem_HandleInput`: the same slider on a 32-bit value (the
    /// fields sit 4 bytes further on).
    #[test]
    fn menu_sys_int_item_handle_input_matches() {
        let setup = |c: &mut Cpu, seed: u32| oc1533_slider_setup(c, seed, 0x2c_d9d8, true);
        check_lohi(
            ee::s09::f_002cd9d8__generated,
            menu_sys_int_item_handle_input,
            400,
            &setup,
        );
        check_reaching(
            ee::s09::f_002cd9d8__generated,
            menu_sys_int_item_handle_input,
            400,
            &OC1533_SLIDER_RETS_32,
            &setup,
        );
    }

    /// Gate word of this module's gated fakes (in the machine's own RAM).
    pub(crate) const OC1534_CTRL: u32 = DATA + 0x8_0000;

    pub(crate) const OC1534_MAGIC: u32 = 0x1534_a115;

    /// Forced result of the `MenuSysPage_GetSelectedItem` fake.
    pub(crate) const OC1534_SELECTED: u32 = OC1534_CTRL + 4;

    /// Float the `InputBinding_EvalFloat` fake returns for binding `a1`:
    /// `OC1534_AXES + 4 * a1`.
    pub(crate) const OC1534_AXES: u32 = OC1534_CTRL + 0x10;

    pub(crate) static OC1534_PREV_AXIS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1534_PREV_SELECTED: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1534_PREV_DRAW_TEXT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1534_PREV_MEASURE: AtomicUsize = AtomicUsize::new(0);

    /// `InputBinding_EvalFloat(binding, id)`: the float at `OC1534_AXES + 4 * id`
    /// in `f0` (what the stick and modifier reads of the menu input return).
    pub(crate) fn oc1534_fake_axis(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_AXIS, ee::s10::f_00320bf0__generated);
        }
        let id = c.g[A1][0] as u32;
        c.f[0] = c.bus.read32(OC1534_AXES + 4 * (id & 0xff));
        c.cycles += 9;
        ret(c);
    }

    /// `MenuSysPage_GetSelectedItem(menu)`: the item stored at `OC1534_SELECTED`.
    pub(crate) fn oc1534_fake_selected(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_SELECTED, ee::s09::f_002cbf30__generated);
        }
        let v = c.bus.read32(OC1534_SELECTED);
        c.g[V0][0] = sx(v);
        c.cycles += 7;
        ret(c);
    }

    /// `Font_DrawText`: a logged fake call (arguments compared).
    pub(crate) fn oc1534_fake_draw_text(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_DRAW_TEXT, ee::s12::f_00391cb0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0039_1cb0, ra);
    }

    /// `Font_MeasureText`: a logged fake call returning a pseudo-random float.
    pub(crate) fn oc1534_fake_measure(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_MEASURE, ee::s12::f_00391fb0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0039_1fb0, ra);
    }

    pub(crate) fn oc1534_install() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            oc1534_fake_axis,
            &OC1534_PREV_AXIS,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBF30),
            oc1534_fake_selected,
            &OC1534_PREV_SELECTED,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391CB0),
            oc1534_fake_draw_text,
            &OC1534_PREV_DRAW_TEXT,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391FB0),
            oc1534_fake_measure,
            &OC1534_PREV_MEASURE,
        );
    }

    /// Install the fakes and open their gate on this machine.
    pub(crate) fn oc1534_gate(c: &mut Cpu) {
        oc1534_install();
        c.bus.write32(OC1534_CTRL, OC1534_MAGIC);
    }

    pub(crate) fn w_4(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    pub(crate) fn gp_word(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// Distinct cycle totals of the generated body over the seeds: the
    /// fixture must reach at least `min` paths.
    pub(crate) fn oc1534_paths(
        gen: GuestFn,
        seeds: u32,
        min: usize,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, gen);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= min,
            "only {} distinct paths (wanted {min}): {seen:?}",
            seen.len()
        );
    }

    // ------------------------------------------- MenuSysFloatItem_HandleInput

    pub(crate) const FLOAT_ITEM: u32 = DATA + 0x1000;

    pub(crate) const FLOAT_MENU: u32 = DATA + 0x2000;

    pub(crate) const FLOAT_INPUT: u32 = DATA + 0x3000;

    pub(crate) const FLOAT_VALUE: u32 = DATA + 0x4000;

    pub(crate) const FLOAT_VT: u32 = DATA + 0x5000;

    /// A float item whose input code, stick, modifiers, hold counter and
    /// value range come from the seed.
    pub(crate) fn float_input_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_dd78, 1);
        oc1534_gate(c);
        w_4(c, gp_word(-0x3fd8), (-0.1f32).to_bits());
        w_4(c, gp_word(-0x3fd4), 0.1f32.to_bits());
        w_4(c, gp_word(-0x3fd0), 6.944_444_6e-5f32.to_bits());
        w_4(c, gp_word(-0x3fcc), 5.0e-5f32.to_bits());
        c.g[A0][0] = FLOAT_ITEM as u64;
        w_4(c, FLOAT_ITEM, FLOAT_MENU);
        w_4(c, FLOAT_MENU + 0x124, FLOAT_INPUT);
        w_4(c, FLOAT_MENU + 0x12c, FLOAT_VT);
        // Callback slot: a fake method with a (negative or positive) delta.
        c.bus
            .write16(FLOAT_VT + 0x20, (r.next_u32() & 0xfffc) as u16);
        w_4(c, FLOAT_VT + 0x24, crate::difftest::fakes::FAKE);
        let code = [0u32, 1, 7, 8, 2, 3, 4, 5, 6, 9][(seed % 10) as usize];
        w_4(c, FLOAT_INPUT + 0x48, code);
        w_4(c, FLOAT_INPUT + 0x5c, DATA + 0x3400);
        w_4(c, FLOAT_ITEM + 4, if seed % 7 == 0 { 8 } else { 0 });
        w_4(c, FLOAT_ITEM + 0x18, FLOAT_VALUE);
        w_4(c, FLOAT_VALUE, r.f32(100.0));
        w_4(c, FLOAT_ITEM + 0x1c, (-50.0f32).to_bits());
        w_4(c, FLOAT_ITEM + 0x20, 50.0f32.to_bits());
        w_4(
            c,
            FLOAT_ITEM + 0x24,
            (if seed % 5 == 0 {
                1.0f32
            } else {
                0.5 + (seed % 4) as f32
            })
            .to_bits(),
        );
        w_4(c, FLOAT_ITEM + 0x28, r.f32(5.0));
        w_4(c, FLOAT_ITEM + 0x2c, (seed * 37) % 300);
        w_4(
            c,
            FLOAT_ITEM + 0x30,
            if seed % 2 == 0 { 1 } else { 0xffff_ffff },
        );
        let (left, right) = match (seed / 5) % 6 {
            0 => (0.0f32, 0.0f32),
            1 => (0.0, 1.0),
            2 => (1.0, 0.0),
            3 => (0.03, 0.0),
            4 => (f32::from_bits(r.f32(1.0)), f32::from_bits(r.f32(1.0))),
            _ => (0.0, -0.8),
        };
        w_4(c, OC1534_AXES + 4 * 0x3e, left.to_bits());
        w_4(c, OC1534_AXES + 4 * 0x3f, right.to_bits());
        let slow = if (seed >> 2) & 1 == 1 { 1.0f32 } else { 0.0 };
        let fine = if (seed >> 3) & 1 == 1 { 1.0f32 } else { 0.0 };
        w_4(c, OC1534_AXES + 4 * 0x42, slow.to_bits());
        w_4(c, OC1534_AXES + 4 * 0x43, fine.to_bits());
    }

    /// The float row: base input result, disabled flag, dead zone, direction
    /// change versus repeat, the four slow/fine divisors, both clamps and the
    /// change callback (which also unwinds on some seeds).
    #[test]
    fn menu_sys_float_item_handle_input_matches() {
        check(
            ee::s09::f_002cdd78__generated,
            menu_sys_float_item_handle_input,
            160,
            &float_input_setup,
        );
        oc1534_paths(ee::s09::f_002cdd78__generated, 160, 8, &float_input_setup);
    }

    // ---------------------------------------- MenuSysTextEditItem_HandleInput

    pub(crate) const TEXT_ITEM: u32 = DATA + 0x1000;

    pub(crate) const TEXT_MENU: u32 = DATA + 0x2000;

    pub(crate) const TEXT_INPUT: u32 = DATA + 0x3000;

    pub(crate) fn text_char(r: &mut Rng) -> u8 {
        b"abcxyzABCXYZ_!09 ~"[(r.next_u32() % 18) as usize]
    }

    /// A 15 character name buffer with the cursor, and the input code / typed
    /// key from the seed.
    pub(crate) fn text_input_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_e488, 0);
        c.g[A0][0] = TEXT_ITEM as u64;
        w_4(c, TEXT_ITEM, TEXT_MENU);
        w_4(c, TEXT_MENU + 0x124, TEXT_INPUT);
        let code = [2u32, 3, 4, 5, 6, 9, 10, 11, 12, 13, 0, 1, 7, 8][(seed % 14) as usize];
        w_4(c, TEXT_INPUT + 0x48, code);
        let key = if (seed / 14) % 4 == 0 {
            8
        } else {
            (r.next_u32() % 3) + 0x20
        };
        w_4(c, TEXT_INPUT + 0x4c, key);
        w_4(c, TEXT_INPUT + 0x50, text_char(&mut r) as u32);
        // Buffer: junk, then `len` characters and the terminator.
        for i in 0..16 {
            c.bus
                .write8(TEXT_ITEM + 0x18 + i, (r.next_u32() & 0x7f) as u8 | 0x20);
        }
        let len = (seed / 3 * 7 + seed) % 16;
        for i in 0..len {
            let ch = text_char(&mut r);
            c.bus.write8(TEXT_ITEM + 0x18 + i, ch);
        }
        c.bus.write8(TEXT_ITEM + 0x18 + len, 0);
        w_4(c, TEXT_ITEM + 4, 0);
        w_4(c, TEXT_ITEM + 0x28, (seed * 5 + seed / 7) % 16);
    }

    /// The name-entry row: base input result, cursor moves (wrap at 16), the
    /// letter step loops, insert, delete, done, backspace and typed keys.
    #[test]
    fn menu_sys_text_edit_item_handle_input_matches() {
        check_lohi(
            ee::s09::f_002ce488__generated,
            menu_sys_text_edit_item_handle_input,
            420,
            &text_input_setup,
        );
        oc1534_paths(ee::s09::f_002ce488__generated, 420, 12, &text_input_setup);
    }

    // ---------------------------------------------- MenuSysTextEditItem_Draw

    pub(crate) const DRAW_ITEM: u32 = DATA + 0x1000;

    pub(crate) const DRAW_ROW: u32 = DATA + 0x1800;

    pub(crate) const DRAW_MENU: u32 = DATA + 0x2000;

    pub(crate) const DRAW_INPUT: u32 = DATA + 0x3000;

    pub(crate) const DRAW_FONT: u32 = DATA + 0x3800;

    pub(crate) const DRAW_MENU_VT: u32 = DATA + 0x5000;

    pub(crate) const DRAW_GFX: u32 = DATA + 0x2_0000;

    /// The row's text buffer, selection, cursor, label, font, colour tables
    /// and the draw context (`gp-0x854`) from the seed.
    pub(crate) fn text_draw_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_e910, 14);
        oc1534_gate(c);
        c.g[A0][0] = DRAW_ITEM as u64;
        c.g[A1][0] = DRAW_ROW as u64;
        w_4(c, DRAW_ITEM, DRAW_MENU);
        w_4(c, DRAW_MENU + 0x124, DRAW_INPUT);
        w_4(c, DRAW_MENU + 0x12c, DRAW_MENU_VT);
        c.bus
            .write16(DRAW_MENU_VT + 0x40, (r.next_u32() & 0xfffc) as u16);
        w_4(c, DRAW_MENU_VT + 0x44, crate::difftest::fakes::FAKE);
        w_4(c, DRAW_INPUT + 0x58, DRAW_FONT);
        w_4(c, DRAW_FONT + 0x14, seed % 9);
        w_4(c, DRAW_FONT + 0x34, r.f32(2.0));
        w_4(c, DRAW_FONT + 0x38, r.f32(2.0));
        w_4(c, DRAW_FONT + 0x3c, r.f32(2.0));
        for i in 0..4 {
            w_4(c, DRAW_ROW + 4 * i, r.f32(50.0));
        }
        // Text: `len` characters, cursor at the end unless the seed says so.
        let len = (seed / 2) % 16;
        for i in 0..len {
            let ch = text_char(&mut r);
            c.bus.write8(DRAW_ITEM + 0x18 + i, ch);
        }
        c.bus.write8(DRAW_ITEM + 0x18 + len, 0);
        let cursor = if seed % 4 == 1 { (len + 1) % 16 } else { len };
        w_4(c, DRAW_ITEM + 0x28, cursor);
        w_4(
            c,
            DRAW_ITEM + 0x14,
            if seed % 3 == 2 { 0 } else { DATA + 0x4000 },
        );
        let selected = if seed % 3 == 0 {
            DRAW_ITEM
        } else {
            DRAW_ITEM + 0x40
        };
        w_4(c, OC1534_SELECTED, selected);
        // Colour tables the item picks from (`0x4d5380..0x4d53bf`).
        for i in 0..16 {
            w_4(c, 0x004d_5380 + 4 * i, r.f32(1.0));
        }
        // Draw context: `gp-0x854` -> object with `+0xe84` (command word
        // block) and `+0x10d8` (an object whose slot `+0x238/+0x23c` runs).
        w_4(c, gp_word(-0x854), DRAW_GFX);
        w_4(c, DRAW_GFX + 0xe84, DRAW_GFX + 0x1000);
        w_4(c, DRAW_GFX + 0x10d8, DRAW_GFX + 0x2000);
        c.bus
            .write16(DRAW_GFX + 0x2000 + 0x238, (r.next_u32() & 0xfffc) as u16);
        w_4(c, DRAW_GFX + 0x2000 + 0x23c, crate::difftest::fakes::FAKE);
    }

    /// The name-entry row's drawing: selected or not, caret appended or not,
    /// label or none, with the shadow passes and the draw-context call.
    #[test]
    fn menu_sys_text_edit_item_draw_matches() {
        check(
            ee::s09::f_002ce910__generated,
            menu_sys_text_edit_item_draw,
            96,
            &text_draw_setup,
        );
        oc1534_paths(ee::s09::f_002ce910__generated, 96, 5, &text_draw_setup);
    }

    // ------------------------------------- MenuSysColorChannelItem_Measure

    /// A colour-channel item with a float value, a font and the output
    /// vector; `Font_MeasureText` is a logged fake returning random floats.
    pub(crate) fn color_measure_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_eee8, 8);
        oc1534_gate(c);
        let item = DATA + 0x1000;
        let menu = DATA + 0x2000;
        let input = DATA + 0x3000;
        let font = DATA + 0x3800;
        let values = DATA + 0x4000;
        c.g[A0][0] = (DATA + 0x800) as u64;
        c.g[A1][0] = item as u64;
        w_4(c, item, menu);
        w_4(c, menu + 0x124, input);
        w_4(c, input + 0x58, font);
        w_4(c, font + 0x14, seed % 11);
        w_4(c, font + 0x34, r.f32(2.0));
        w_4(c, font + 0x38, r.f32(2.0));
        w_4(c, font + 0x3c, r.f32(2.0));
        w_4(c, item + 0x14, values);
        w_4(c, item + 0x18, seed % 4);
        for i in 0..4 {
            w_4(c, values + 4 * i, r.f32(255.0));
        }
    }

    /// Both orders of the two channel-name widths decide which label width is
    /// kept; the fake's random floats take every branch over the seeds.
    #[test]
    fn menu_sys_color_channel_item_measure_matches() {
        check(
            ee::s09::f_002ceee8__generated,
            menu_sys_color_channel_item_measure,
            120,
            &color_measure_setup,
        );
        oc1534_paths(ee::s09::f_002ceee8__generated, 120, 3, &color_measure_setup);
    }

    /// A menu item: `*item` is the menu system, whose `+0x124` is the menu
    /// whose `+0x5c` is the input binding (zeroed).
    pub(crate) fn menu_item_machine(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        let (item, sys, menu, bind) = (DATA + 0x800, DATA + 0x880, DATA + 0x900, DATA + 0x980);
        c.bus.write32(item, sys);
        c.bus.write32(sys + 0x124, menu);
        c.bus.write32(menu + 0x5c, bind);
        c.g[A0][0] = sx(item);
    }

    #[test]
    fn menu_sys_item_get_left_axis_matches() {
        check(
            ee::s09::f_002ca408__generated,
            menu_sys_item_get_left_axis,
            100,
            &|c, seed| menu_item_machine(c, seed, 0x002c_a408),
        );
    }

    #[test]
    fn menu_sys_item_get_right_axis_matches() {
        check(
            ee::s09::f_002ca428__generated,
            menu_sys_item_get_right_axis,
            100,
            &|c, seed| menu_item_machine(c, seed, 0x002c_a428),
        );
    }

    pub(crate) const SEEDS_11: u32 = 200;

    /// Gate words (unique in each test machine) and the magic that opens them
    /// for the gated fakes below.
    pub(crate) const GATE_VALIDATE: u32 = DATA + 0x0020_0000;

    pub(crate) const GATE_SELECTED: u32 = DATA + 0x0020_0004;

    pub(crate) const GATE_TEXT: u32 = DATA + 0x0020_0008;

    pub(crate) const MAGIC: u32 = 0x1554_c0de;

    /// The input-binding VM table and its stop opcode (as the camera tests use).
    pub(crate) const VM_TABLE: u32 = 0x0048_e330;

    pub(crate) const VM_STOP_OP: u32 = 0x20;

    pub(crate) const VM_STOP: u32 = 0x0032_5ef8;

    pub(crate) fn oc1554_fake_validate(c: &mut Cpu) {
        if c.bus.read32(GATE_VALIDATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1554_fake_validate),
                ee::s09::f_002caab0__generated,
            );
        }
        fakes::fake_call(c, 0x002c_aab0);
    }

    pub(crate) fn oc1554_fake_selected(c: &mut Cpu) {
        if c.bus.read32(GATE_SELECTED) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1554_fake_selected),
                ee::s09::f_002cbf30__generated,
            );
        }
        fakes::fake_call(c, 0x002c_bf30);
    }

    pub(crate) fn oc1554_fake_text(c: &mut Cpu) {
        if c.bus.read32(GATE_TEXT) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1554_fake_text),
                ee::s09::f_002ca768__generated,
            );
        }
        fakes::fake_call(c, 0x002c_a768);
    }

    /// Hook the gated fakes into their callees' slots (once per process).
    pub(crate) fn oc1554_hooks() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAAB0),
            oc1554_fake_validate,
            fakes::prev_cell(oc1554_fake_validate),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBF30),
            oc1554_fake_selected,
            fakes::prev_cell(oc1554_fake_selected),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA768),
            oc1554_fake_text,
            fakes::prev_cell(oc1554_fake_text),
        );
    }

    /// A menu's binding and the input map behind it, so that
    /// `InputBinding_EvalFloat` (0x320bf0) runs for real and returns a value.
    pub(crate) fn oc1554_input_chain(c: &mut Cpu, menu: u32, seed: u32) {
        let binding = DATA + 0xa00;
        let map = DATA + 0xc00;
        c.bus.write32(menu + 0x5c, binding);
        c.bus.write32(binding, 0); // the map's binding index
        c.bus.write32(binding + 4, map);
        c.bus.write32(map + 0x4, map + 0x80);
        c.bus.write32(map + 0x14, map + 0x100);
        for i in 0..4u32 {
            c.bus.write32(map + 0x80 + 4 * i, i);
            c.bus.write32(map + 0x100 + 4 * i, VM_STOP_OP);
        }
        c.bus.write32(VM_TABLE + 4 * VM_STOP_OP, VM_STOP);
        // The stopped VM leaves `*(parser + 8)` in f0; the parser is the frame
        // of `InputMap_EvalFloat`, four frames below the test's stack. Both
        // compare outcomes of the menu queries' bools.
        let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
        c.bus.write32(difftest::SP - 0x1f8, value.to_bits());
    }

    /// An item whose first word points at a holder with the menu at `+0x124`.
    pub(crate) fn oc1554_item_chain(c: &mut Cpu, seed: u32) {
        let item = DATA;
        let holder = DATA + 0x100;
        let menu = DATA + 0x800;
        c.bus.write32(item, holder);
        c.bus.write32(holder + 0x124, menu);
        oc1554_input_chain(c, menu, seed);
    }

    #[test]
    fn menu_sys_item_get_up_axis_matches() {
        check(
            ee::s09::f_002ca448__generated,
            menu_sys_item_get_up_axis,
            SEEDS_11,
            &|c, seed| {
                let mut r = Rng(0x002c_a448 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_get_down_axis_matches() {
        check(
            ee::s09::f_002ca468__generated,
            menu_sys_item_get_down_axis,
            SEEDS_11,
            &|c, seed| {
                let mut r = Rng(0x002c_a468 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_is_slow_mod_held_matches() {
        check(
            ee::s09::f_002ca488__generated,
            menu_sys_item_is_slow_mod_held,
            SEEDS_11,
            &|c, seed| {
                let mut r = Rng(0x002c_a488 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_item_is_fine_mod_held_matches() {
        check(
            ee::s09::f_002ca4a8__generated,
            menu_sys_item_is_fine_mod_held,
            SEEDS_11,
            &|c, seed| {
                let mut r = Rng(0x002c_a4a8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                oc1554_item_chain(c, seed);
            },
        );
    }

    #[test]
    fn menu_sys_get_left_axis_matches() {
        check(
            ee::s09::f_002cc260__generated,
            menu_sys_get_left_axis,
            SEEDS_11,
            &|c, seed| {
                let mut r = Rng(0x002c_c260 ^ seed as u64);
                scramble(c, &mut r);
                let menu = DATA + 0x800;
                c.g[A0][0] = menu as u64;
                oc1554_input_chain(c, menu, seed);
            },
        );
    }

    #[test]
    fn menu_sys_page_remove_item_matches() {
        oc1554_hooks();
        check(
            ee::s09::f_002cbd50__generated,
            menu_sys_page_remove_item,
            SEEDS_11,
            &|c, seed| {
                fakes::machine(c, seed, 0x002c_bd50, 0);
                let page = DATA + 0x1000;
                let count = seed % 5;
                c.g[A0][0] = page as u64;
                c.g[A1][0] = ((seed / 5) % 5) as u64;
                c.bus.write32(page, count);
                let selected = if seed % 11 == 0 {
                    u32::MAX // negative: the clamp branch
                } else {
                    (seed / 25) % 5
                };
                c.bus.write32(page + 0x4, selected);
                for i in 0..5u32 {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x2000 + 0x10 * i);
                }
                c.bus.write32(GATE_VALIDATE, MAGIC);
            },
        );
    }

    #[test]
    fn menu_sys_page_sort_items_matches() {
        oc1554_hooks();
        check(
            ee::s09::f_002cbe20__generated,
            menu_sys_page_sort_items,
            SEEDS_11,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x002c_be20, 0);
                let page = DATA + 0x1000;
                c.g[A0][0] = page as u64;
                c.g[A1][0] = r.next_u32() as u64;
                // Zero or one item: the qsort never calls the comparator.
                c.bus.write32(page, seed % 2);
                c.bus.write32(GATE_VALIDATE, MAGIC);
            },
        );
    }

    #[test]
    fn menu_sys_item_draw_row_matches() {
        oc1554_hooks();
        check(
            ee::s09::f_002ca988__generated,
            menu_sys_item_draw_row,
            SEEDS_11,
            &|c, seed| {
                fakes::machine(c, seed, 0x002c_a988, 0);
                let page = DATA + 0x1000;
                c.bus.write32(page, DATA + 0x2000);
                c.g[A0][0] = page as u64;
                // The selected item: the page itself (the row is selected) or
                // another pointer (a forced 0 gives a random one).
                let selected = if seed % 2 == 0 { page } else { 0 };
                c.bus.write32(fakes::LOG_RESULTS, selected);
                c.bus.write32(GATE_SELECTED, MAGIC);
                c.bus.write32(GATE_TEXT, MAGIC);
            },
        );
    }

    /// Gate word and magic of the `InputBinding_EvalFloat` fake, and its
    /// forced float table (16 words, indexed by the input id & 15).
    pub(crate) const OC1555_AX_CTRL: u32 = DATA + 0xd_5550;

    pub(crate) const OC1555_AX_MAGIC: u32 = 0x1555_a115;

    pub(crate) const OC1555_AX_VALUES: u32 = DATA + 0xd_5560;

    /// Gate word, call counter and row table of the `MenuSysItem_MeasureRow`
    /// fake (two rows of four floats each).
    pub(crate) const OC1555_MS_CTRL: u32 = DATA + 0xd_55c0;

    pub(crate) const OC1555_MS_MAGIC: u32 = 0x1555_b115;

    pub(crate) const OC1555_MS_CALLS: u32 = DATA + 0xd_55c4;

    pub(crate) const OC1555_MS_VALUES: u32 = DATA + 0xd_55d0;

    /// The item and output buffers the measure fixtures use.
    pub(crate) const OC1555_ITEM: u32 = DATA + 0x1000;

    pub(crate) const OC1555_OUT: u32 = DATA + 0x2000;

    /// The object the item fixtures pass as `self`.
    pub(crate) const OC1555_SELF: u32 = DATA + 0x3000;

    pub(crate) static OC1555_PREV_AXIS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1555_PREV_MEASURE: AtomicUsize = AtomicUsize::new(0);

    /// `InputBinding_EvalFloat(binding, id)`: the forced float at
    /// `OC1555_AX_VALUES + 4 * (id & 15)` in f0.
    pub(crate) fn oc1555_fake_axis(c: &mut Cpu) {
        if c.bus.read32(OC1555_AX_CTRL) != OC1555_AX_MAGIC {
            return fall_back(c, &OC1555_PREV_AXIS, ee::s10::f_00320bf0__generated);
        }
        let id = c.g[A1][0] as u32;
        c.f[0] = c.bus.read32(OC1555_AX_VALUES + 4 * (id & 0xf));
        c.cycles += 9;
        ret(c);
    }

    /// `MenuSysItem_MeasureRow(out, item, a2, a3, t0)`: writes one row of
    /// four floats into `out` (a0), taken from the table by call parity.
    pub(crate) fn oc1555_fake_measure_row(c: &mut Cpu) {
        if c.bus.read32(OC1555_MS_CTRL) != OC1555_MS_MAGIC {
            return fall_back(c, &OC1555_PREV_MEASURE, ee::s09::f_002ca4c8__generated);
        }
        let n = c.bus.read32(OC1555_MS_CALLS);
        c.bus.write32(OC1555_MS_CALLS, n + 1);
        let out = c.g[A0][0] as u32;
        for i in 0..4 {
            let w = c.bus.read32(OC1555_MS_VALUES + 4 * ((n & 1) * 4 + i));
            c.bus.write32(out + 4 * i, w);
        }
        c.cycles += 20;
        ret(c);
    }

    pub(crate) fn oc1555_install() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            oc1555_fake_axis,
            &OC1555_PREV_AXIS,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA4C8),
            oc1555_fake_measure_row,
            &OC1555_PREV_MEASURE,
        );
    }

    /// Axis and modifier fixture: `self` at `DATA` with a binding word at +0x5c,
    /// and the forced inputs. Every fourth seed gives 0.0 (the modifier is not
    /// held), the others edge floats (zeros, NaN, infinities, normals).
    pub(crate) fn oc1555_axis_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1555_a0a0 ^ seed as u64);
        scramble(c, &mut r);
        oc1555_install();
        c.bus.write32(OC1555_AX_CTRL, OC1555_AX_MAGIC);
        for i in 0..16 {
            let v = if seed % 4 == 0 { 0 } else { r.edge_f32() };
            c.bus.write32(OC1555_AX_VALUES + 4 * i, v);
        }
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + 0x5c, r.next_u32());
    }

    /// Item constructor fixture: `self` at `OC1555_SELF`, random argument words.
    pub(crate) fn oc1555_ctor_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1555_c500 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(OC1555_SELF);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(r.next_u32());
        c.g[T0][0] = sx(r.next_u32());
        c.g[T1][0] = sx(r.next_u32());
    }

    /// Measure fixture: the item's `+0x14` (a2), `+0x18` (zero takes the
    /// single-row path) and `+0x1c` (zero takes the single row too, a non-zero
    /// one lays out two rows). Seeds cycle through the paths, and the row table
    /// gives both compare outcomes.
    pub(crate) fn oc1555_measure_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1555_c648 ^ seed as u64);
        scramble(c, &mut r);
        oc1555_install();
        c.bus.write32(OC1555_MS_CTRL, OC1555_MS_MAGIC);
        c.bus.write32(OC1555_MS_CALLS, 0);
        for i in 0..8 {
            c.bus.write32(OC1555_MS_VALUES + 4 * i, r.edge_f32());
        }
        let text = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
        let width = if seed % 4 == 1 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(OC1555_ITEM + 0x14, r.next_u32());
        c.bus.write32(OC1555_ITEM + 0x18, text);
        c.bus.write32(OC1555_ITEM + 0x1c, width);
        c.g[A0][0] = sx(OC1555_OUT);
        c.g[A1][0] = sx(OC1555_ITEM);
    }

    #[test]
    fn menu_sys_get_right_axis_matches() {
        check(
            ee::s09::f_002cc280__generated,
            menu_sys_get_right_axis,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_get_up_axis_matches() {
        check(
            ee::s09::f_002cc2a0__generated,
            menu_sys_get_up_axis,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_get_down_axis_matches() {
        check(
            ee::s09::f_002cc2c0__generated,
            menu_sys_get_down_axis,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_is_slow_mod_held_matches() {
        check(
            ee::s09::f_002cc2e0__generated,
            menu_sys_is_slow_mod_held,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_is_fine_mod_held_matches() {
        check(
            ee::s09::f_002cc318__generated,
            menu_sys_is_fine_mod_held,
            300,
            &|c, seed| oc1555_axis_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_bool_item_ctor_matches() {
        check(
            ee::s09::f_002cc500__generated,
            menu_sys_bool_item_ctor,
            200,
            &|c, seed| oc1555_ctor_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_bool_item_measure_matches() {
        check(
            ee::s09::f_002cc648__generated,
            menu_sys_bool_item_measure,
            300,
            &|c, seed| oc1555_measure_setup(c, seed),
        );
    }

    #[test]
    fn menu_sys_enum_item_ctor_matches() {
        check(
            ee::s09::f_002cc7b0__generated,
            menu_sys_enum_item_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1555_c7b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC1555_SELF);
            },
        );
    }

    /// Gate word of this module's callee fakes (a unique cell of the machine's
    /// RAM): the fakes answer only when a fixture sets it.
    pub(crate) const OC1556_GATE: u32 = DATA + 0x20_1556;

    pub(crate) const OC1556_MAGIC: u32 = 0x1556_f001;

    /// Four words the row-measure fake copies to its output buffer.
    pub(crate) const OC1556_MEASURE: u32 = DATA + 0x20_1600;

    /// A 64-bit value with random halves (sign and upper bits must match too).
    pub(crate) fn wide(r: &mut Rng) -> u64 {
        ((r.next_u32() as u64) << 32) | r.next_u32() as u64
    }

    /// `MenuSysItem_DrawRow` with this module's gate on: logs the call.
    pub(crate) fn oc1556_draw_row_fake(c: &mut Cpu) {
        if c.bus.read32(OC1556_GATE) != OC1556_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1556_draw_row_fake),
                ee::s09::f_002ca988__generated,
            );
        }
        fakes::fake_call(c, 0x002c_a988);
    }

    /// `MenuSysItem_MeasureRow` with this module's gate on: copies the four
    /// mailbox words to the output buffer (a0) and logs the call.
    pub(crate) fn oc1556_measure_row_fake(c: &mut Cpu) {
        if c.bus.read32(OC1556_GATE) != OC1556_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1556_measure_row_fake),
                ee::s09::f_002ca4c8__generated,
            );
        }
        let out = c.g[A0][0] as u32;
        for k in 0..4u32 {
            let w = c.bus.read32(OC1556_MEASURE + 4 * k);
            c.bus.write32(out + 4 * k, w);
        }
        fakes::fake_call(c, 0x002c_a4c8);
    }

    /// Hook this module's fakes into the callee slots (once per process).
    pub(crate) fn oc1556_install() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA988),
            oc1556_draw_row_fake,
            fakes::prev_cell(oc1556_draw_row_fake),
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA4C8),
            oc1556_measure_row_fake,
            fakes::prev_cell(oc1556_measure_row_fake),
        );
    }

    /// `fakes::machine` with this module's gate switched on.
    pub(crate) fn oc1556_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1556_GATE, OC1556_MAGIC);
        r
    }

    /// Arguments of a row draw: an item with a random +0x14 word.
    pub(crate) fn oc1556_draw_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = oc1556_machine(c, seed, salt, 2);
        c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
        c.g[A1][0] = wide(&mut r);
        c.bus.write32(DATA + 0x14, r.next_u32());
    }

    #[test]
    fn menu_sys_enum_item_ctor_2_matches() {
        check(
            ee::s09::f_002cc7f8__generated,
            menu_sys_enum_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cc7f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_ctor_3_matches() {
        check(
            ee::s09::f_002cc838__generated,
            menu_sys_enum_item_ctor_3,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cc838 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
                c.g[A1][0] = wide(&mut r);
                c.g[A2][0] = wide(&mut r);
                c.g[A3][0] = wide(&mut r);
                c.g[T0][0] = wide(&mut r);
                c.g[T1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_enum_item_measure_matches() {
        oc1556_install();
        check(
            ee::s09::f_002ccb18__generated,
            menu_sys_enum_item_measure,
            200,
            &|c, seed| {
                let mut r = oc1556_machine(c, seed, 0x2ccb18, 3);
                let (out, item, rows) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                // The four words the row measure leaves in its buffer.
                for k in 0..4u32 {
                    c.bus.write32(OC1556_MEASURE + 4 * k, r.f32(10.0));
                }
                // The frame word at sp+8 of the function: not written by the
                // row measure, so the float compare reads what is here.
                c.bus.write32(TEST_SP - 0x68, r.f32(10.0));
                // Zero, one, two or three rows.
                c.bus.write32(item + 0x14, r.next_u32());
                c.bus.write32(item + 0x18, seed % 4);
                c.bus.write32(item + 0x1c, rows);
                for i in 0..4u32 {
                    c.bus.write32(rows + 8 * i, r.next_u32());
                }
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | out as u64;
                c.g[A1][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | item as u64;
            },
        );
    }

    #[test]
    fn menu_sys_submenu_item_draw_matches() {
        oc1556_install();
        check(
            ee::s09::f_002ccdc8__generated,
            menu_sys_submenu_item_draw,
            120,
            &|c, seed| oc1556_draw_setup(c, seed, 0x2ccdc8),
        );
    }

    #[test]
    fn menu_sys_back_item_draw_matches() {
        oc1556_install();
        check(
            ee::s09::f_002ccee0__generated,
            menu_sys_back_item_draw,
            120,
            &|c, seed| oc1556_draw_setup(c, seed, 0x2ccee0),
        );
    }

    #[test]
    fn menu_sys_action_item_ctor_2_matches() {
        check(
            ee::s09::f_002ccf50__generated,
            menu_sys_action_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2ccf50 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_callback_item_ctor_matches() {
        check(
            ee::s09::f_002cd128__generated,
            menu_sys_callback_item_ctor,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cd128 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_callback_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd168__generated,
            menu_sys_callback_item_ctor_2,
            120,
            &|c, seed| {
                let mut r = Rng(0x2cd168 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (wide(&mut r) & 0xffff_ffff_0000_0000) | DATA as u64;
                c.g[A2][0] = wide(&mut r);
                c.g[A3][0] = wide(&mut r);
            },
        );
    }

    /// Gate word of this module's `MenuSysItem_DrawRow` fake (a unique cell of
    /// the machine's RAM): the fake answers only when a fixture sets it.
    pub(crate) const OC1557_GATE: u32 = DATA + 0x35_0000;

    pub(crate) const OC1557_MAGIC: u32 = 0x1557_f001;

    /// Gated fake for the draw-row callee: logs the call with the gate on,
    /// otherwise runs what the slot held before (or the generated body).
    pub(crate) fn oc1557_fake_draw_row(c: &mut Cpu) {
        if c.bus.read32(OC1557_GATE) != OC1557_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1557_fake_draw_row),
                ee::s09::f_002ca988__generated,
            );
        }
        fakes::fake_call(c, 0x002c_a988);
    }

    /// Hook the draw-row fake into its callee's slot (once per process).
    pub(crate) fn oc1557_install() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA988),
            oc1557_fake_draw_row,
            fakes::prev_cell(oc1557_fake_draw_row),
        );
    }

    /// `fakes::machine` with this module's draw-row gate switched on.
    pub(crate) fn oc1557_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1557_GATE, OC1557_MAGIC);
        r
    }

    #[test]
    fn menu_sys_callback_item_draw_matches() {
        oc1557_install();
        fakes::covered(
            ee::s09::f_002cd278__generated,
            menu_sys_callback_item_draw,
            96,
            &|c, seed| {
                let mut r = oc1557_machine(c, seed, 0x002c_d278, 1);
                let item = DATA + 0x1000;
                fill(c, &mut r, item, 0x10);
                c.g[A0][0] = sx(item);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_label_item_draw_matches() {
        oc1557_install();
        fakes::covered(
            ee::s09::f_002cd328__generated,
            menu_sys_label_item_draw,
            96,
            &|c, seed| {
                let mut r = oc1557_machine(c, seed, 0x002c_d328, 1);
                let item = DATA + 0x1000;
                fill(c, &mut r, item, 0x10);
                c.g[A0][0] = sx(item);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_label_item_ctor_matches() {
        check(
            ee::s09::f_002cd2a0__generated,
            menu_sys_label_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd2_a000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_spacer_item_ctor_matches() {
        check(
            ee::s09::f_002cd350__generated,
            menu_sys_spacer_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd3_5000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_matches() {
        check(
            ee::s09::f_002cd3f8__generated,
            menu_sys_int8_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd3_f800 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd450__generated,
            menu_sys_int8_item_ctor_2,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd4_5000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int8_item_ctor_3_matches() {
        check(
            ee::s09::f_002cd4a0__generated,
            menu_sys_int8_item_ctor_3,
            80,
            &|c, seed| {
                let mut r = Rng(0x2cd4_a000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x10);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
                c.g[A2][0] = wide(&mut r);
                c.g[A3][0] = wide(&mut r);
                c.g[T0][0] = wide(&mut r);
                c.g[T1][0] = wide(&mut r);
            },
        );
    }

    #[test]
    fn menu_sys_int_item_ctor_matches() {
        check(
            ee::s09::f_002cd850__generated,
            menu_sys_int_item_ctor,
            60,
            &|c, seed| {
                let mut r = Rng(0x2cd8_5000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, DATA + 0x1000, 0x12);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = wide(&mut r);
            },
        );
    }

    /// A menu item for the input handlers: its first word is the object, whose
    /// +0x12c word is a class table with the value callback (`delta` at +0x20,
    /// the function at +0x24, which the machine's miss hook answers), and the
    /// float and counter fields the handlers read.
    pub(crate) fn oc1558_input_item(c: &mut Cpu, r: &mut Rng, item: u32) {
        let obj = DATA + 0x3000;
        let table = DATA + 0x3100;
        let float_block = DATA + 0x3200;
        c.bus.write32(item, obj);
        c.bus.write32(obj + 0x12c, table);
        c.bus
            .write16(table + 0x20, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(table + 0x24, fakes::FAKE);
        c.bus.write32(item + 0x14, float_block);
        c.bus.write32(item + 0x18, float_block);
        for i in 0..4 {
            c.bus.write32(float_block + 4 * i, r.f32(10.0));
        }
        c.bus.write32(item + 0x1c, r.f32(10.0));
        c.bus.write32(item + 0x20, r.f32(10.0));
        c.bus.write32(item + 0x24, r.next_u32() % 4);
        c.bus.write32(item + 0x28, r.next_u32() % 4);
    }

    #[test]
    fn menu_sys_int_item_ctor_2_matches() {
        check(
            ee::s09::f_002cd8a8__generated,
            menu_sys_int_item_ctor_2,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x002c_d8a8, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn menu_sys_float_item_ctor_matches() {
        check(
            ee::s09::f_002cdc88__generated,
            menu_sys_float_item_ctor,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x002c_dc88, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_ctor_matches() {
        check(
            ee::s09::f_002ce170__generated,
            menu_sys_float_range_item_ctor,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x002c_e170, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_group_item_page_forward_v03_matches() {
        check(
            ee::s09::f_002cf788__generated,
            menu_sys_group_item_page_forward_v03,
            24,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x002c_f788, 1);
                let (item, group) = (DATA + 0x100, DATA + 0x200);
                // the group's vtable: delta at +0x48, the forwarding slot at +0x4c
                let table = fakes::vtable(c, &mut r, &[0x48]);
                c.bus.write32(group + 0x10, table);
                c.bus.write32(item + 0x130, group);
                c.g[A0][0] = sx(item);
            },
        );
    }

    #[test]
    fn menu_sys_format_float_matches() {
        check(
            ee::s09::f_002cdfd8__generated,
            menu_sys_format_float,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_dfd8, 3);
                elf_image(c);
                c.g[A0][0] = sx(DATA + 0x100);
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_float_range_item_handle_input_matches() {
        check(
            ee::s09::f_002ce1f0__generated,
            menu_sys_float_range_item_handle_input,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_e1f0, 3);
                elf_image(c);
                let item = DATA + 0x100;
                oc1558_input_item(c, &mut r, item);
                c.g[A0][0] = sx(item);
                c.f[20] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_color_channel_item_handle_input_matches() {
        check(
            ee::s09::f_002ced78__generated,
            menu_sys_color_channel_item_handle_input,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_ed78, 3);
                elf_image(c);
                let item = DATA + 0x100;
                oc1558_input_item(c, &mut r, item);
                c.g[A0][0] = sx(item);
                c.f[20] = r.edge_f32();
            },
        );
    }

    #[test]
    fn menu_sys_text_edit_item_measure_matches() {
        check(
            ee::s09::f_002ce820__generated,
            menu_sys_text_edit_item_measure,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x002c_e820, 3);
                elf_image(c);
                let (out, item, page) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                c.bus.write32(item, page);
                c.bus.write32(item + 0x14, DATA + 0x400);
                c.bus.write32(item + 0x18, r.next_u32());
                c.bus.write32(item + 0x28, r.next_u32() % 20);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(item);
            },
        );
    }

    /// Scrambled registers and the fake hooks (no unwinds) for a fixture.
    pub(crate) fn msgi_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        fakes::machine(c, seed, salt, 0)
    }

    /// A vtable at `vt` whose slot at +0x10 is `{delta, f}` (the group item's
    /// method table: `+0x10` is the thunk delta, `+0x14` the method).
    pub(crate) fn msgi_vtable(c: &mut Cpu, vt: u32, delta: u16, f: u32) {
        c.bus.write16(vt + 0x10, delta);
        c.bus.write32(vt + 0x14, f);
    }

    #[test]
    fn menu_sys_group_item_page_forward_v04_matches() {
        check(
            ee::s09::f_002cf7b8__generated,
            menu_sys_group_item_page_forward_v04,
            40,
            &|c, seed| {
                let mut r = msgi_setup(c, seed, 0x7b8);
                let this = DATA + 0x1000;
                let page = DATA + 0x2000;
                let item = DATA + 0x3000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x130, page);
                c.bus.write32(page + 0x10, item);
                c.bus.write16(item + 0x50, (r.next_u32() & 0xff) as u16);
                // A real leaf (MenuSysItem_IsVisible) on even seeds, a fake otherwise.
                let target = if seed % 2 == 0 { 0x002c_a378 } else { FAKE };
                c.bus.write32(item + 0x54, target);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_ctor_matches() {
        check(
            ee::s09::f_002cf7e8__generated,
            menu_sys_group_item_ctor,
            40,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x7e8);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_ctor_2_matches() {
        check(
            ee::s09::f_002cf860__generated,
            menu_sys_group_item_ctor_2,
            40,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x860);
                c.g[A0][0] = sx(DATA + 0x1000);
                // A zero id skips the add; otherwise the id is added to the page.
                c.g[A2][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(DATA + 0x5000 + 0x40 * (seed % 4))
                };
            },
        );
    }

    #[test]
    fn menu_sys_group_item_add_item_matches() {
        check(
            ee::s09::f_002cf908__generated,
            menu_sys_group_item_add_item,
            60,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x908);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = seed % 4;
                c.bus.write32(page, count);
                c.bus.write32(page + 4, seed % 3);
                c.bus.write32(page + 0x124, seed % 2);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x7000);
                c.g[A2][0] = match seed % 4 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => count as u64,
                    _ => 1,
                };
            },
        );
    }

    #[test]
    fn menu_sys_group_item_remove_item_matches() {
        check(
            ee::s09::f_002cf928__generated,
            menu_sys_group_item_remove_item,
            60,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x928);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = 1 + seed % 3;
                c.bus.write32(page, count);
                c.bus.write32(page + 4, seed % count);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                c.g[A0][0] = sx(this);
                // The item to remove: one of the page's items, or one that is not there.
                c.g[A1][0] = sx(DATA + 0x6000 + 0x40 * (seed % (count + 1)));
            },
        );
    }

    #[test]
    fn menu_sys_group_item_reset_matches() {
        check(
            ee::s09::f_002cf948__generated,
            menu_sys_group_item_reset,
            60,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x948);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = seed % 3;
                c.bus.write32(page, count);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                let header = if seed % 2 == 0 { 0 } else { DATA + 0x6000 };
                c.bus.write32(this + MSGI_HEADER, header);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_set_header_item_matches() {
        check(
            ee::s09::f_002cf988__generated,
            menu_sys_group_item_set_header_item,
            80,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x988);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = seed % 4;
                c.bus.write32(page, count);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                // The old header: one of the items (found, removed), or not in the page.
                let header = if seed % 5 == 4 {
                    DATA + 0x7000
                } else {
                    DATA + 0x6000 + 0x40 * (seed % 3)
                };
                c.bus.write32(this + MSGI_HEADER, header);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x8000);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_handle_input_matches() {
        check(
            ee::s09::f_002cfa08__generated,
            menu_sys_group_item_handle_input,
            120,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0xa08);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = 1 + seed % 4;
                c.bus.write32(page, count);
                c.bus.write32(page + 4, seed % count);
                // Every item shares one method table; its method is a fake that
                // returns the key (the first fake results of the call log).
                let vt = DATA + 0x20_0000;
                for i in 0..count {
                    let item = DATA + 0x6000 + 0x100 * i;
                    c.bus.write32(item + 0x10, vt);
                    c.bus.write32(page + 0xc + 4 * i, item);
                }
                msgi_vtable(c, vt, 0, FAKE);
                let key = [3u32, 4, 5, 1][(seed % 4) as usize];
                for n in 0..8 {
                    c.bus.write32(LOG_RESULTS + 4 * n, key);
                }
                c.bus.write32(this + 4, if seed % 4 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_v09_matches() {
        check(
            ee::s09::f_002cfaf8__generated,
            menu_sys_group_item_v09,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_faf8, 1);
                let obj = DATA + 0x8000;
                c.bus.write32(DATA, obj);
                c.g[A0][0] = sx(DATA);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(obj + 0x12c, vt);
            },
        );
    }

    /// Gate word of the fakes (own RAM word and magic: the fakes sit in shared
    /// override slots, so they act only on machines that set this).
    pub(crate) const OC1561_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1561_MAGIC: u32 = 0x1561_2d0e;

    pub(crate) const OC1561_ITEM: u32 = DATA + 0x21_0000;

    pub(crate) const OC1561_MENU: u32 = DATA + 0x22_0000;

    pub(crate) const OC1561_VT: u32 = DATA + 0x23_0000;

    pub(crate) const OC1561_FONT: u32 = DATA + 0x24_0000;

    pub(crate) const OC1561_STYLE: u32 = DATA + 0x25_0000;

    pub(crate) const OC1561_DRAW: u32 = DATA + 0x26_0000;

    pub(crate) const OC1561_TOP: u32 = DATA + 0x27_0000;

    pub(crate) const OC1561_REND: u32 = DATA + 0x28_0000;

    pub(crate) const OC1561_REND2: u32 = DATA + 0x29_0000;

    pub(crate) const OC1561_PTS: u32 = DATA + 0x2a_0000;

    pub(crate) const OC1561_RECT: u32 = DATA + 0x2b_0000;

    pub(crate) const OC1561_LABEL: u32 = DATA + 0x2c_0000;

    /// Gated fakes for the callees of the draw: with the gate word set they log
    /// the call (`fakes::fake_call`); otherwise they run the body they replaced.
    macro_rules! oc1561_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1561_CTRL) != OC1561_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1561_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1561_fakes!(
        (
            oc1561_fake_2cbf30,
            OC1561_PREV_2CBF30,
            ee::s09::OVERRIDE_002CBF30,
            ee::s09::f_002cbf30__generated,
            0x002c_bf30
        ),
        (
            oc1561_fake_2cdfd8,
            OC1561_PREV_2CDFD8,
            ee::s09::OVERRIDE_002CDFD8,
            ee::s09::f_002cdfd8__generated,
            0x002c_dfd8
        ),
        (
            oc1561_fake_2cb350,
            OC1561_PREV_2CB350,
            ee::s09::OVERRIDE_002CB350,
            ee::s09::f_002cb350__generated,
            0x002c_b350
        ),
        (
            oc1561_fake_391cb0,
            OC1561_PREV_391CB0,
            ee::s12::OVERRIDE_00391CB0,
            ee::s12::f_00391cb0__generated,
            0x0039_1cb0
        ),
        (
            oc1561_fake_318630,
            OC1561_PREV_318630,
            ee::s10::OVERRIDE_00318630,
            ee::s10::f_00318630__generated,
            0x0031_8630
        ),
        (
            oc1561_fake_3185c8,
            OC1561_PREV_3185C8,
            ee::s10::OVERRIDE_003185C8,
            ee::s10::f_003185c8__generated,
            0x0031_85c8
        ),
        (
            oc1561_fake_318260,
            OC1561_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
    );

    pub(crate) fn oc1561_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// One menu row with a point curve. Seed bits: 0 = show cursor, 1 = the item
    /// is the page's selected one, 2-3 = Y range kind (all positive, straddling
    /// zero, all negative, starting at zero); the point count is 1..=4.
    pub(crate) fn oc1561_setup(c: &mut Cpu, seed: u32) {
        oc1561_install_fakes();
        let mut r: Rng = machine(c, seed, 0x2d0ef8, 24);
        c.g[GP][0] = GAME_GP as u64;
        oc1561_w(c, OC1561_CTRL, OC1561_MAGIC);
        let focused = seed & 2 != 0;
        if focused {
            // Calls 0 and 2 are MenuSysPage_GetSelectedItem (1 is the menu's
            // draw-background vcall).
            oc1561_w(c, LOG_RESULTS, OC1561_ITEM);
            oc1561_w(c, LOG_RESULTS + 8, OC1561_ITEM);
        }
        c.g[A0][0] = sx(OC1561_ITEM);
        c.g[A1][0] = sx(OC1561_RECT);
        for i in 0..4 {
            oc1561_w(c, OC1561_RECT + 4 * i, r.f32(200.0));
        }
        let count = 1 + (seed >> 4) % 4;
        let cur = (seed >> 6) % count;
        let (min_y, max_y) = match (seed >> 2) % 4 {
            0 => (1.0f32, 9.0f32),
            1 => (-4.0, 6.0),
            2 => (-9.0, -1.0),
            _ => (0.0, 5.0),
        };
        oc1561_w(c, OC1561_ITEM, OC1561_MENU);
        oc1561_w(c, OC1561_ITEM + 0x14, OC1561_LABEL);
        oc1561_w(c, OC1561_ITEM + 0x18, count);
        oc1561_w(c, OC1561_ITEM + 0x24, cur);
        oc1561_w(c, OC1561_ITEM + 0x28, OC1561_PTS);
        oc1561_w(c, OC1561_ITEM + 0x2c, 0.0f32.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x30, 10.0f32.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x34, min_y.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x38, max_y.to_bits());
        oc1561_w(c, OC1561_ITEM + 0x40, seed & 1);
        for i in 0..8 {
            oc1561_w(c, OC1561_PTS + 4 * i, r.f32(10.0));
        }
        // The menu: font at +0x124, vtable at +0x12c (slot 9 at +0x48 = delta,
        // +0x4c = function) pointing at a fake method.
        oc1561_w(c, OC1561_MENU + 0x124, OC1561_FONT);
        oc1561_w(c, OC1561_MENU + 0x12c, OC1561_VT);
        c.bus.write16(OC1561_VT + 0x48, 0x40);
        oc1561_w(c, OC1561_VT + 0x4c, FAKE + 0x10);
        oc1561_w(c, OC1561_FONT + 0x58, OC1561_STYLE);
        oc1561_w(c, OC1561_STYLE + 0x14, 14);
        oc1561_w(c, OC1561_STYLE + 0x34, 1.25f32.to_bits());
        // Draw module (gp - 0x854) -> top object; renderer (gp + 0x2a90).
        oc1561_w(c, GAME_GP - 0x854, OC1561_DRAW);
        oc1561_w(c, OC1561_DRAW + 0xe84, OC1561_TOP);
        oc1561_w(c, OC1561_TOP + 8, r.next_u32());
        oc1561_w(c, GAME_GP + 0x2a90, OC1561_REND);
        oc1561_w(c, OC1561_REND + 0x10d8, OC1561_REND2);
        c.bus.write16(OC1561_REND2 + 0x248, 0x20);
        oc1561_w(c, OC1561_REND2 + 0x24c, FAKE + 0x20);
        // Colour globals D_004D5380/53a0/53b0/53c0/53d0/53e0 (4 floats each).
        for base in [
            0x4d_5380u32,
            0x4d_53a0,
            0x4d_53b0,
            0x4d_53c0,
            0x4d_53d0,
            0x4d_53e0,
        ] {
            for i in 0..4 {
                oc1561_w(c, base + 4 * i, r.f32(1.2));
            }
        }
    }

    /// The draw against the generated body: the focused and idle label colours,
    /// the cursor text, the graph (axis only when the Y range straddles 0,
    /// polyline only with 2+ points, cursor marker) and callee unwinds.
    #[test]
    fn menu_sys_point2d_item_draw_matches() {
        fakes::check_reaching(
            ee::s09::f_002d0ef8__generated,
            menu_sys_point2d_item_draw,
            256,
            &[
                0x002d_0f40, // GetSelectedItem
                0x002d_0f68, // background vcall
                0x002d_105c, // label
                0x002d_1090, // FormatFloat x
                0x002d_10d4, // string concat
                0x002d_1134, // cursor text
                0x002d_12c8, // graph background quad
                0x002d_1444, // axis line
                0x002d_15a4, // polyline segment
                0x002d_16e4, // cursor marker quad
            ],
            &oc1561_setup,
        );
    }

    /// Gate word of this module's callee fakes (a unique cell of the machine's
    /// RAM): the fakes answer only when a fixture sets it.
    pub(crate) const OC1564_GATE: u32 = DATA + 0x20_0000;

    pub(crate) const OC1564_MAGIC: u32 = 0x1564_f001;

    /// A gated fake for the real callee at `$addr`: with the gate off it runs
    /// whatever the slot held before (`fakes::fall_back`), with the gate on it
    /// logs the call and returns (or unwinds) like `fakes::fake_call`.
    macro_rules! oc1564_fake {
        ($name:ident, $gen:path, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1564_GATE) != OC1564_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc1564_fake!(
        oc1564_fake_draw_row,
        ee::s09::f_002ca988__generated,
        0x002c_a988
    );

    oc1564_fake!(
        oc1564_fake_group_input,
        ee::s09::f_002cfa08__generated,
        0x002c_fa08
    );

    oc1564_fake!(
        oc1564_fake_mem_copy,
        ee::s14::f_003e6574__generated,
        0x003e_6574
    );

    oc1564_fake!(
        oc1564_fake_14dc80,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );

    oc1564_fake!(
        oc1564_fake_14dd58,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );

    oc1564_fake!(
        oc1564_fake_144ce0,
        ee::s01::f_00144ce0__generated,
        0x0014_4ce0
    );

    oc1564_fake!(
        oc1564_fake_22cea8,
        ee::s05::f_0022cea8__generated,
        0x0022_cea8
    );

    oc1564_fake!(
        oc1564_fake_22d088,
        ee::s05::f_0022d088__generated,
        0x0022_d088
    );

    oc1564_fake!(
        oc1564_fake_231250,
        ee::s06::f_00231250__generated,
        0x0023_1250
    );

    oc1564_fake!(
        oc1564_fake_28b180,
        ee::s07::f_0028b180__generated,
        0x0028_b180
    );

    oc1564_fake!(
        oc1564_fake_2a49e8,
        ee::s08::f_002a49e8__generated,
        0x002a_49e8
    );

    oc1564_fake!(
        oc1564_fake_40fc90,
        ee::s15::f_0040fc90__generated,
        0x0040_fc90
    );

    oc1564_fake!(
        oc1564_fake_31adb0,
        ee::s10::f_0031adb0__generated,
        0x0031_adb0
    );

    oc1564_fake!(
        oc1564_fake_317d70,
        ee::s10::f_00317d70__generated,
        0x0031_7d70
    );

    oc1564_fake!(
        oc1564_fake_317458,
        ee::s10::f_00317458__generated,
        0x0031_7458
    );

    oc1564_fake!(
        oc1564_fake_316d60,
        ee::s10::f_00316d60__generated,
        0x0031_6d60
    );

    /// Hook every fake of this module into its callee's slot (once per process).
    pub(crate) fn oc1564_install() {
        macro_rules! hook {
            ($slot:path, $fake:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
            };
        }
        hook!(ee::s09::OVERRIDE_002CA988, oc1564_fake_draw_row);
        hook!(ee::s09::OVERRIDE_002CFA08, oc1564_fake_group_input);
        hook!(ee::s14::OVERRIDE_003E6574, oc1564_fake_mem_copy);
        hook!(ee::s01::OVERRIDE_0014DC80, oc1564_fake_14dc80);
        hook!(ee::s01::OVERRIDE_0014DD58, oc1564_fake_14dd58);
        hook!(ee::s01::OVERRIDE_00144CE0, oc1564_fake_144ce0);
        hook!(ee::s05::OVERRIDE_0022CEA8, oc1564_fake_22cea8);
        hook!(ee::s05::OVERRIDE_0022D088, oc1564_fake_22d088);
        hook!(ee::s06::OVERRIDE_00231250, oc1564_fake_231250);
        hook!(ee::s07::OVERRIDE_0028B180, oc1564_fake_28b180);
        hook!(ee::s08::OVERRIDE_002A49E8, oc1564_fake_2a49e8);
        hook!(ee::s15::OVERRIDE_0040FC90, oc1564_fake_40fc90);
        hook!(ee::s10::OVERRIDE_0031ADB0, oc1564_fake_31adb0);
        hook!(ee::s10::OVERRIDE_00317D70, oc1564_fake_317d70);
        hook!(ee::s10::OVERRIDE_00317458, oc1564_fake_317458);
        hook!(ee::s10::OVERRIDE_00316D60, oc1564_fake_316d60);
    }

    /// `fakes::machine` with this module's fake gate switched on, and (when
    /// `image`) the game image loaded for tables the function reads.
    pub(crate) fn oc1564_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        image: bool,
    ) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        if image {
            elf_image(c);
        }
        c.bus.write32(OC1564_GATE, OC1564_MAGIC);
        r
    }

    /// `void MenuSysColorSwatchItem_Draw(item, rect)`: draws the item row, then
    /// a colour swatch quad (four vertices from the item's colour) through the
    /// render list's vtable call. Entries: the start, and the resume points at
    /// the vertex setup, the two alpha arms and the vtable call.
    #[test]
    fn menu_sys_color_swatch_item_draw_matches() {
        oc1564_install();
        const ENTRIES: [u32; 8] = [
            0x002d_0008,
            0x002d_0008,
            0x002d_0008,
            0x002d_0008,
            0x002d_0020,
            0x002d_0234,
            0x002d_024c,
            0x002d_0260,
        ];
        fakes::covered(
            ee::s09::f_002d0008__generated,
            menu_sys_color_swatch_item_draw,
            96,
            &|c, seed| {
                let mut r = oc1564_machine(c, seed, 0x002d_0008, 2, false);
                let (rect, item, color) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                let (pool, ent, queue) = (DATA + 0x2000, DATA + 0x5000, DATA + 0x5400);
                for i in 0..4 {
                    c.bus.write32(color + 4 * i, r.f32(1.0));
                    c.bus.write32(rect + 4 * i, r.f32(100.0));
                }
                if seed % 2 == 0 {
                    // alpha (word 0) exactly 1.0: the opaque arm
                    c.bus.write32(color, 0x3f80_0000);
                }
                c.bus.write32(item + 0x14, color);
                c.bus.write32(item + 0x18, r.next_u32());
                c.bus.write32(GAME_GP.wrapping_sub(0x854), pool);
                c.bus.write32(pool + 0xe84, ent);
                c.bus.write32(ent + 4, r.next_u32());
                c.bus.write32(pool + 0x10d8, queue);
                c.bus.write16(queue + 0x238, (r.next_u32() % 8) as u16);
                c.bus.write32(queue + 0x23c, fakes::FAKE);
                c.g[A0][0] = item as u64;
                c.g[A1][0] = rect as u64;
                c.g[S0][0] = rect as u64;
                c.g[S1][0] = item as u64;
                let entry = ENTRIES[(seed % 8) as usize];
                match entry {
                    0x002d_0234 | 0x002d_024c => c.g[V1][0] = pool as u64,
                    0x002d_0260 => {
                        c.g[A0][0] = ent as u64;
                        c.g[V0][0] = r.next_u32() as u64;
                    }
                    _ => {}
                }
                if entry != 0x002d_0008 {
                    c.bus.write64(TEST_SP + 0xe0, TEST_RA as u64);
                }
                c.pc = entry;
            },
        );
    }

    /// `u32 MenuSysVectorItem_HandleInput(item)`: runs the group handler, then,
    /// when the vector item is active (+0x670), normalises the three edited
    /// components and writes them back through their value pointers. Entries:
    /// the start and every resume point of the body.
    #[test]
    fn menu_sys_vector_item_handle_input_matches() {
        oc1564_install();
        const ENTRIES: [u32; 10] = [
            0x002d_06e8,
            0x002d_06e8,
            0x002d_06e8,
            0x002d_06e8,
            0x002d_0700,
            0x002d_070c,
            0x002d_07a0,
            0x002d_07c8,
            0x002d_07d0,
            0x002d_07f4,
        ];
        fakes::covered(
            ee::s09::f_002d06e8__generated,
            menu_sys_vector_item_handle_input,
            100,
            &|c, seed| {
                let mut r = oc1564_machine(c, seed, 0x002d_06e8, 1, false);
                let obj = DATA + 0x1000;
                let zero = seed % 5 == 1;
                c.bus
                    .write32(obj + 0x670, if seed % 4 == 0 { 0 } else { 1 });
                for off in [0x5dc, 0x5e0, 0x610, 0x614, 0x644, 0x648] {
                    c.bus.write32(obj + off, if zero { 0 } else { r.f32(4.0) });
                }
                // three value records: +0x18 points at the float they edit
                for (i, rec) in [0x5b8u32, 0x5ec, 0x620].into_iter().enumerate() {
                    let p = DATA + 0x3000 + 0x10 * i as u32;
                    c.bus.write32(obj + rec + 0x18, p);
                    c.bus.write32(p, if zero { 0 } else { r.f32(4.0) });
                }
                c.g[A0][0] = obj as u64;
                c.g[S0][0] = obj as u64;
                c.g[A2][0] = (obj + 0x5b8) as u64;
                c.g[T0][0] = (obj + 0x5ec) as u64;
                c.g[A3][0] = (obj + 0x620) as u64;
                c.g[V0][0] = (obj + 0x5b8) as u64;
                for i in 0..3 {
                    c.bus.write32(TEST_SP + 4 * i, r.f32(4.0));
                }
                let entry = ENTRIES[(seed % 10) as usize];
                if entry != 0x002d_06e8 {
                    c.bus.write64(TEST_SP + 0x20, TEST_RA as u64);
                }
                c.pc = entry;
            },
        );
    }

    // -- MenuSysPoint2DItem_HandleInput -------------------------------------

    pub(crate) const OC1567_RET_DISABLED: u32 = 3;

    pub(crate) const OC1567_RET_CODE: u32 = 4;

    pub(crate) const OC1567_RET_UP: u32 = 5;

    pub(crate) const OC1567_RET_DOWN: u32 = 6;

    pub(crate) const OC1567_RET_LEFT: u32 = 7;

    pub(crate) const OC1567_RET_RIGHT: u32 = 8;

    pub(crate) const OC1567_RET_BASE: u32 = 9;

    /// [`oc1567_gated`], and for the axis getters the forced word also
    /// becomes the float result `f0`.
    pub(crate) fn oc1567_gated_axis(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        float: bool,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC1567_CTRL) == OC1567_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                let word = c.bus.read32(OC1567_RET + 4 * ret_idx);
                c.g[V0][0] = sx(word);
                if float {
                    c.f[0] = word;
                }
            }
            return;
        }
        fall_back(c, prev, fallback)
    }

    macro_rules! oc1567_item_fake {
        ($fake:ident, $prev:ident, $addr:expr, $idx:expr, $float:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                oc1567_gated_axis(c, $addr, $idx, $float, &$prev, $generated);
            }
        };
    }

    oc1567_item_fake!(
        oc1567_fake_disabled,
        OC1567_PREV_DISABLED,
        0x002c_a3b0,
        OC1567_RET_DISABLED,
        false,
        ee::s09::f_002ca3b0__generated
    );

    oc1567_item_fake!(
        oc1567_fake_code,
        OC1567_PREV_CODE,
        0x002c_a3e8,
        OC1567_RET_CODE,
        false,
        ee::s09::f_002ca3e8__generated
    );

    oc1567_item_fake!(
        oc1567_fake_left,
        OC1567_PREV_LEFT,
        0x002c_a408,
        OC1567_RET_LEFT,
        true,
        ee::s09::f_002ca408__generated
    );

    oc1567_item_fake!(
        oc1567_fake_right,
        OC1567_PREV_RIGHT,
        0x002c_a428,
        OC1567_RET_RIGHT,
        true,
        ee::s09::f_002ca428__generated
    );

    oc1567_item_fake!(
        oc1567_fake_up,
        OC1567_PREV_UP,
        0x002c_a448,
        OC1567_RET_UP,
        true,
        ee::s09::f_002ca448__generated
    );

    oc1567_item_fake!(
        oc1567_fake_down,
        OC1567_PREV_DOWN,
        0x002c_a468,
        OC1567_RET_DOWN,
        true,
        ee::s09::f_002ca468__generated
    );

    oc1567_item_fake!(
        oc1567_fake_base,
        OC1567_PREV_BASE,
        0x002c_a2b0,
        OC1567_RET_BASE,
        false,
        ee::s09::f_002ca2b0__generated
    );

    pub(crate) fn oc1567_install_item_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA3B0),
            oc1567_fake_disabled,
            &OC1567_PREV_DISABLED,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA3E8),
            oc1567_fake_code,
            &OC1567_PREV_CODE,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA408),
            oc1567_fake_left,
            &OC1567_PREV_LEFT,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA428),
            oc1567_fake_right,
            &OC1567_PREV_RIGHT,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA448),
            oc1567_fake_up,
            &OC1567_PREV_UP,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA468),
            oc1567_fake_down,
            &OC1567_PREV_DOWN,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002CA2B0),
            oc1567_fake_base,
            &OC1567_PREV_BASE,
        );
    }

    /// `MenuSysPoint2DItem_HandleInput`: the input code (6 toggles, 7/8 stop
    /// editing, 4/5 move the selection, anything else drags), the editing
    /// flag, the item disabled or not, the up/down and left/right axes equal
    /// and different, locks set and clear, points inside and outside the
    /// bounds, `order` -1/0/1, counts below and above 25 (the step's
    /// `max(2 * count, 50)`), the selection at the first, last and middle
    /// points, and the menu notification (a faked vtable call). The seven
    /// callees are gated fakes with forced results.
    #[test]
    fn menu_sys_point2d_item_handle_input_matches() {
        use crate::difftest::fakes::FAKE;
        const ITEM: u32 = DATA + 0x1000;
        const MENU: u32 = DATA + 0x2000;
        const MENU_VT: u32 = DATA + 0x2400;
        const PTS: u32 = DATA + 0x3000;
        oc1567_install_item_fakes();
        covered(
            ee::s09::f_002d0908__generated,
            menu_sys_point2d_item_handle_input,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_0908, 8);
                c.pc = 0x002d_0908;
                c.bus.write32(OC1567_CTRL, OC1567_MAGIC);
                let code = [6u32, 8, 7, 4, 5, 9, 6, 5, 4][(seed % 9) as usize];
                let editing = (seed / 9) % 2;
                c.bus.write32(
                    OC1567_RET + 4 * OC1567_RET_DISABLED,
                    if seed % 13 == 0 { 1 } else { 0 },
                );
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_CODE, code);
                c.bus
                    .write32(OC1567_RET + 4 * OC1567_RET_BASE, r.next_u32());
                // Axes: equal pairs (no drag) and differing ones.
                let axis = |r: &mut Rng| -> u32 {
                    let v = [0.0f32, 1.0, -1.0, 0.5, 2.5, -3.0][(r.next_u32() % 6) as usize];
                    v.to_bits()
                };
                let up = axis(&mut r);
                let right = axis(&mut r);
                let down = if (seed / 18) % 3 == 0 {
                    up
                } else {
                    axis(&mut r)
                };
                let left = if (seed / 54) % 3 == 0 {
                    right
                } else {
                    axis(&mut r)
                };
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_UP, up);
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_DOWN, down);
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_RIGHT, right);
                c.bus.write32(OC1567_RET + 4 * OC1567_RET_LEFT, left);
                // `.lit4`: 1 / 120 at gp - 0x3fa0 and gp - 0x3f9c.
                let gp = GAME_GP;
                c.bus.write32(gp.wrapping_sub(0x3fa0), 0x3c08_8889);
                c.bus.write32(gp.wrapping_sub(0x3f9c), 0x3c08_8889);
                // The item.
                let count = [1u32, 2, 3, 5, 30, 4][((seed / 7) % 6) as usize];
                let cur = match (seed / 3) % 4 {
                    0 => 0,
                    1 => count - 1,
                    _ => (seed / 5) % count,
                };
                let order = [0i32, 1, -1][((seed / 11) % 3) as usize];
                c.bus.write32(ITEM + OC1567_P2D_MENU, MENU);
                c.bus.write32(MENU + 0x12c, MENU_VT);
                c.bus.write32(MENU_VT + 0x20, 0x10 + seed % 3);
                c.bus.write32(MENU_VT + 0x24, FAKE + 0x80);
                c.bus.write32(ITEM + OC1567_P2D_COUNT, count);
                let lock = |r: &mut Rng, cur: u32, on: bool| -> u32 {
                    let mut m = r.next_u32();
                    if on {
                        m |= 1 << cur;
                    } else {
                        m &= !(1 << cur);
                    }
                    m
                };
                let lock_x = lock(&mut r, cur, seed % 17 == 0);
                let lock_y = lock(&mut r, cur, seed % 19 == 0);
                c.bus.write32(ITEM + OC1567_P2D_LOCK_X, lock_x);
                c.bus.write32(ITEM + OC1567_P2D_LOCK_Y, lock_y);
                c.bus.write32(ITEM + OC1567_P2D_CUR, cur);
                c.bus.write32(ITEM + OC1567_P2D_PTS, PTS);
                // Bounds (x -10..10, y 0..20) and points around them.
                let (min_x, max_x, min_y, max_y) = (-10.0f32, 10.0f32, 0.0f32, 20.0f32);
                for (i, v) in [min_x, min_y, max_x, max_y].into_iter().enumerate() {
                    c.bus
                        .write32(ITEM + OC1567_P2D_BOUNDS + 4 * i as u32, v.to_bits());
                }
                c.bus.write32(ITEM + OC1567_P2D_ORDER, order as u32);
                c.bus.write32(ITEM + OC1567_P2D_EDITING, editing);
                for i in 0..32u32 {
                    // Mostly inside the bounds, sometimes beyond one.
                    let x = (r.next_u32() % 2600) as f32 / 100.0 - 13.0;
                    let y = (r.next_u32() % 2600) as f32 / 100.0 - 3.0;
                    c.bus.write32(PTS + 8 * i, x.to_bits());
                    c.bus.write32(PTS + 8 * i + 4, y.to_bits());
                }
                c.g[A0][0] = sx(ITEM);
            },
        );
    }

    /// The machine for the constructor tests: the pool allocator and the
    /// dlmalloc arena (the sub-items allocate), with the game image loaded for
    /// the globals the real callees read. Scrambled registers come first.
    pub(crate) fn oc1570_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine_rt(c, seed, salt, 0);
        elf_image(c);
        r
    }

    /// `void MenuSysGroupItem_OnBlur(group)`: the generated body against the port,
    /// the group blurred with its page at +0x18 (real callees, zeroed object).
    #[test]
    fn menu_sys_group_item_on_blur_matches() {
        check(
            ee::s09::f_002cfef8__generated,
            menu_sys_group_item_on_blur,
            40,
            &|c, seed| {
                let _ = oc1570_machine(c, seed, 0x002c_fef8);
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    /// `cARGBMenuItem` constructor: the four channels, the swatch and the five
    /// sub-items, with a random colour and adjust pointer.
    #[test]
    fn menu_sys_color_item_ctor_matches() {
        check(
            ee::s09::f_002d02b0__generated,
            menu_sys_color_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_02b0);
                let (item, name, colour, adjust) = (
                    DATA + 0x1000,
                    DATA + 0x1_0000,
                    DATA + 0x1_0100,
                    DATA + 0x1_0200,
                );
                for i in 0..4 {
                    c.bus.write32(colour + 4 * i, r.f32(100.0));
                }
                c.g[A0][0] = item as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = colour as u64;
                c.g[A3][0] = adjust as u64;
            },
        );
    }

    /// `MenuSysColorItem_HandleInput(item)`: forwards to the group handler.
    #[test]
    fn menu_sys_color_item_handle_input_matches() {
        check(
            ee::s09::f_002d03d0__generated,
            menu_sys_color_item_handle_input,
            40,
            &|c, seed| {
                let _ = oc1570_machine(c, seed, 0x002d_03d0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    /// `cVectorTitleMenuItem` constructor: the item base, vector and vtable.
    #[test]
    fn menu_sys_vector_label_item_ctor_matches() {
        check(
            ee::s09::f_002d03f0__generated,
            menu_sys_vector_label_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_03f0);
                let (item, caption, vector) = (DATA + 0x1000, DATA + 0x1_0000, DATA + 0x1_0100);
                fill(c, &mut r, vector, 4);
                c.g[A0][0] = item as u64;
                c.g[A1][0] = caption as u64;
                c.g[A2][0] = vector as u64;
            },
        );
    }

    /// `cVectorMenuItem` constructor: the group base, three float editors, the
    /// vector label and the four sub-items; min and max come in f12 and f13.
    #[test]
    fn menu_sys_vector_item_ctor_matches() {
        check(
            ee::s09::f_002d05a8__generated,
            menu_sys_vector_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_05a8);
                let (item, name, values, caption) = (
                    DATA + 0x1000,
                    DATA + 0x1_0000,
                    DATA + 0x1_0100,
                    DATA + 0x1_0200,
                );
                for i in 0..3 {
                    c.bus.write32(values + 4 * i, r.f32(100.0));
                }
                c.g[A0][0] = item as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = values as u64;
                c.g[A3][0] = caption as u64;
                c.f[12] = r.f32(50.0);
                c.f[13] = r.f32(50.0);
            },
        );
    }

    /// `cGraphMenuItem` constructor: the point count, lock bits, slope, the
    /// points pointer and the two 8-byte bound vectors copied unaligned.
    #[test]
    fn menu_sys_point2d_item_ctor_matches() {
        check(
            ee::s09::f_002d0808__generated,
            menu_sys_point2d_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_0808);
                let (item, points, min, max) = (
                    DATA + 0x1000,
                    DATA + 0x1_0100,
                    DATA + 0x1_0200,
                    DATA + 0x1_0300,
                );
                fill(c, &mut r, min, 2);
                fill(c, &mut r, max, 2);
                c.g[A0][0] = item as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = points as u64;
                c.g[T2][0] = min as u64;
                c.g[T3][0] = max as u64;
            },
        );
    }

    /// `cErrorMenu_cErrorMenu(page, ...)`: the page, the title label, the
    /// spacer, the back item and three inserts.
    #[test]
    fn menu_sys_message_page_ctor_matches() {
        check(
            ee::s09::f_002d1740__generated,
            menu_sys_message_page_ctor,
            40,
            &|c, seed| {
                let _ = oc1570_machine(c, seed, 0x002d_1740);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x1_0000) as u64;
            },
        );
    }

    /// `cErrorMenu_cErrorMenu1(page, name, flag)`: the second error menu.
    #[test]
    fn menu_sys_message_page_ctor_2_matches() {
        check(
            ee::s09::f_002d17f0__generated,
            menu_sys_message_page_ctor_2,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_17f0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x1_0000) as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
            },
        );
    }

    /// Gate words (one per fake) in the machine's RAM: a fake answers only
    /// when its own word holds its own magic, else it falls back.
    pub(crate) const OC1571_DTOR_GATE: u32 = DATA + 0x5_7110;

    pub(crate) const OC1571_DEAD_NODE_GATE: u32 = DATA + 0x5_7114;

    pub(crate) const OC1571_AVALANCHE_GATE: u32 = DATA + 0x5_7118;

    pub(crate) const OC1571_DTOR_MAGIC: u32 = 0x1571_0001;

    pub(crate) const OC1571_DEAD_NODE_MAGIC: u32 = 0x1571_0002;

    pub(crate) const OC1571_AVALANCHE_MAGIC: u32 = 0x1571_0003;

    pub(crate) static OC1571_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1571_PREV_DEAD_NODE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1571_PREV_AVALANCHE: AtomicUsize = AtomicUsize::new(0);

    /// The gated fake of `MenuSysItem_dtor`: logs the call and returns to `ra`.
    pub(crate) fn oc1571_fake_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1571_DTOR_GATE) != OC1571_DTOR_MAGIC {
            return fakes::fall_back(c, &OC1571_PREV_DTOR, ee::s09::f_002ca280__generated);
        }
        fakes::fake_call(c, 0x002c_a280);
    }

    /// The gated fake of `WorldObjFactory_DeadNode`.
    pub(crate) fn oc1571_fake_dead_node(c: &mut Cpu) {
        if c.bus.read32(OC1571_DEAD_NODE_GATE) != OC1571_DEAD_NODE_MAGIC {
            return fakes::fall_back(c, &OC1571_PREV_DEAD_NODE, ee::s10::f_002fc2c0__generated);
        }
        fakes::fake_call(c, 0x002f_c2c0);
    }

    /// The gated fake of `Avalanche_GetObjectMatrix`.
    pub(crate) fn oc1571_fake_avalanche(c: &mut Cpu) {
        if c.bus.read32(OC1571_AVALANCHE_GATE) != OC1571_AVALANCHE_MAGIC {
            return fakes::fall_back(c, &OC1571_PREV_AVALANCHE, ee::s09::f_002d9c00__generated);
        }
        fakes::fake_call(c, 0x002d_9c00);
    }

    pub(crate) fn oc1571_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc1571_fake_dtor,
            &OC1571_PREV_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FC2C0),
            oc1571_fake_dead_node,
            &OC1571_PREV_DEAD_NODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9C00),
            oc1571_fake_avalanche,
            &OC1571_PREV_AVALANCHE,
        );
    }

    /// A destructor thunk against the generated one: the arguments are
    /// scrambled and pass through, the fake `MenuSysItem_dtor` returns to
    /// `ra`, and `machine` (calls = 1) makes some seeds unwind at the call.
    pub(crate) fn oc1571_dtor_check(generated: fn(&mut Cpu), port: fn(&mut Cpu), salt: u64) {
        oc1571_install_fakes();
        check(generated, port, 200, &|c, seed| {
            machine(c, seed, salt, 1);
            c.bus.write32(OC1571_DTOR_GATE, OC1571_DTOR_MAGIC);
        });
    }

    #[test]
    fn world_create_dead_node_matches() {
        oc1571_install_fakes();
        check(
            ee::s09::f_002d1cc0__generated,
            world_create_dead_node,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_1cc0, 1);
                c.bus.write32(OC1571_DEAD_NODE_GATE, OC1571_DEAD_NODE_MAGIC);
            },
        );
    }

    #[test]
    fn menu_sys_bool_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3a68__generated,
            menu_sys_bool_item_dtor,
            0x002d_3a68,
        );
    }

    #[test]
    fn menu_sys_enum_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3a98__generated,
            menu_sys_enum_item_dtor,
            0x002d_3a98,
        );
    }

    #[test]
    fn menu_sys_submenu_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3ae0__generated,
            menu_sys_submenu_item_dtor,
            0x002d_3ae0,
        );
    }

    #[test]
    fn menu_sys_back_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3b10__generated,
            menu_sys_back_item_dtor,
            0x002d_3b10,
        );
    }

    #[test]
    fn menu_sys_action_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3b38__generated,
            menu_sys_action_item_dtor,
            0x002d_3b38,
        );
    }

    #[test]
    fn menu_sys_callback_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3b80__generated,
            menu_sys_callback_item_dtor,
            0x002d_3b80,
        );
    }

    #[test]
    fn menu_sys_label_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3bb0__generated,
            menu_sys_label_item_dtor,
            0x002d_3bb0,
        );
    }

    #[test]
    fn menu_sys_spacer_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3be0__generated,
            menu_sys_spacer_item_dtor,
            0x002d_3be0,
        );
    }

    #[test]
    fn menu_sys_int8_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3c00__generated,
            menu_sys_int8_item_dtor,
            0x002d_3c00,
        );
    }

    #[test]
    fn menu_sys_int_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3c30__generated,
            menu_sys_int_item_dtor,
            0x002d_3c30,
        );
    }

    #[test]
    fn menu_sys_float_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3c60__generated,
            menu_sys_float_item_dtor,
            0x002d_3c60,
        );
    }

    #[test]
    fn menu_sys_float_range_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3ce0__generated,
            menu_sys_float_range_item_dtor,
            0x002d_3ce0,
        );
    }

    #[test]
    fn menu_sys_text_edit_item_dtor_matches() {
        dtor_case(
            ee::s09::f_002d3d20__generated,
            menu_sys_text_edit_item_dtor,
            0x002d_3d20,
        );
    }

    /// `SetText` with a random buffer (the first byte is cleared by the
    /// function) and a source string holding a NUL at a seed-chosen place, so
    /// `strncat` both stops early and copies the full 15 bytes.
    #[test]
    fn menu_sys_text_edit_item_set_text_matches() {
        check(
            ee::s09::f_002d3d48__generated,
            menu_sys_text_edit_item_set_text,
            200,
            &|c, seed| {
                let mut r = Rng(0x002d_3d48 ^ seed as u64);
                scramble(c, &mut r);
                let item = DATA + 0x100;
                let text = DATA + 0x400;
                c.g[A0][0] = sx(item);
                c.g[A1][0] = sx(text);
                for i in 0..4u32 {
                    c.bus.write32(item + 0x18 + 4 * i, r.next_u32());
                }
                let nul = seed % 20;
                for i in 0..24u32 {
                    let b = if i == nul {
                        0
                    } else {
                        (r.next_u32() as u8) | 1
                    };
                    c.bus.write8(text + i, b);
                }
            },
        );
    }

    /// The allocator's mutex (`MEM_MUTEX`), set up so that the `operator
    /// delete` the last member may call takes its short path.
    pub(crate) const OC1573_MEM_MUTEX: u32 = 0x0053_8b00;

    /// The deleting-destructor fixture: a random object pointer, flags 0..3
    /// (bit 0 makes the last member free its object) and the allocator words.
    pub(crate) fn oc1573_dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            c.bus.write32(OC1573_MEM_MUTEX + 4, 0x42);
            c.bus.write32(LOG_RESULTS, 0x42);
            c.bus.write32(OC1573_MEM_MUTEX + 8, r.next_u32() % 4);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

    #[test]
    fn menu_sys_color_channel_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3d70__generated,
            menu_sys_color_channel_item_dtor,
            0x2d_3d70,
        );
    }

    #[test]
    fn menu_sys_group_item_page_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3d90__generated,
            menu_sys_group_item_page_dtor,
            0x2d_3d90,
        );
    }

    #[test]
    fn menu_sys_group_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3db0__generated,
            menu_sys_group_item_dtor,
            0x2d_3db0,
        );
    }

    #[test]
    fn menu_sys_color_swatch_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3df8__generated,
            menu_sys_color_swatch_item_dtor,
            0x2d_3df8,
        );
    }

    #[test]
    fn menu_sys_color_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3e20__generated,
            menu_sys_color_item_dtor,
            0x2d_3e20,
        );
    }

    #[test]
    fn menu_sys_vector_label_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3ea8__generated,
            menu_sys_vector_label_item_dtor,
            0x2d_3ea8,
        );
    }

    #[test]
    fn menu_sys_vector_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3ed0__generated,
            menu_sys_vector_item_dtor,
            0x2d_3ed0,
        );
    }

    #[test]
    fn menu_sys_point2d_item_dtor_matches() {
        oc1573_dtor_case(
            ee::s09::f_002d3f50__generated,
            menu_sys_point2d_item_dtor,
            0x2d_3f50,
        );
    }

    pub(crate) const OC1574_ITEM_CTRL: u32 = DATA + 0x20_1000;

    pub(crate) const OC1574_PAGE_CTRL: u32 = DATA + 0x20_1004;

    pub(crate) static OC1574_PREV_ITEM: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1574_PREV_PAGE: AtomicUsize = AtomicUsize::new(0);

    /// `MenuSysItem_dtor`: a logged call while its gate is on.
    pub(crate) fn oc1574_fake_item_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1574_ITEM_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_ITEM, ee::s09::f_002ca280__generated);
        }
        fakes::fake_call(c, 0x002c_a280);
    }

    /// `MenuSysPage_dtor`: a logged call while its gate is on.
    pub(crate) fn oc1574_fake_page_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1574_PAGE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_PAGE, ee::s09::f_002caa80__generated);
        }
        fakes::fake_call(c, 0x002c_aa80);
    }

    pub(crate) fn oc1574_hook_menu() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc1574_fake_item_dtor,
            &OC1574_PREV_ITEM,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA80),
            oc1574_fake_page_dtor,
            &OC1574_PREV_PAGE,
        );
    }

    #[test]
    fn menu_sys_message_page_dtor_matches() {
        oc1574_hook_menu();
        check(
            ee::s09::f_002d3f78__generated,
            menu_sys_message_page_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x2d3f, 0);
                c.bus.write32(OC1574_ITEM_CTRL, OC1574_MAGIC);
                c.bus.write32(OC1574_PAGE_CTRL, OC1574_MAGIC);
                c.g[A0][0] = (DATA + 0x2_0000 + 0x40 * (seed % 3)) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x002d_3f78;
            },
        );
    }

    #[test]
    fn menu_sys_page_sort_compare_matches() {
        check(
            ee::s09::f_002d4008__generated,
            menu_sys_page_sort_compare,
            200,
            &|c, seed| {
                machine(c, seed, 0x2d40, 0);
                c.bus.write32(GAME_GP + 0x23cc, fakes::FAKE + 0x20);
                let (a, b) = (DATA + 0x100, DATA + 0x200);
                c.bus.write32(a, DATA + 0x300);
                c.bus.write32(b, DATA + 0x340);
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.pc = 0x002d_4008;
            },
        );
    }
}
