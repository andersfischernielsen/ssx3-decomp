//! `src/ui/uivector.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::fepopup::oc508_at;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::object::instancenode::ui_vector_lw;
pub(crate) use crate::game::object::railmodifier::reset_vtable_and_delete;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::ui::uitext::spch_ea;
pub(crate) use crate::game::ui::uitext::ui_text_field;
pub(crate) use crate::game::ui::uithing::ui_anim_update;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_3d28, ui_vector_update_anim, "UIVector_UpdateAnim"),
    (0x003a_3d70, ui_vector_v20, "UIVector_v20"),
    (0x003a_3e38, ui_vector_v21, "UIVector_v21"),
    (0x003a_47a8, ui_visitor_dtor, "UIVisitor_dtor"),
    (0x003a_47f8, ui_list_node_dtor, "UIListNode_dtor"),
    (0x003a_4888, ui_list_dtor, "UIList_dtor"),
    (0x003a_49e8, ui_object_v04, "UIObject_v04"),
    (0x003a_4a08, ui_object_v05, "UIObject_v05"),
    (0x003a_4a28, ui_object_v06, "UIObject_v06"),
    (0x003a_4a48, ui_object_set_focus, "UIObject_SetFocus"),
    (0x003a_4a68, ui_object_v08, "UIObject_v08"),
    (0x003a_4a88, ui_object_set_visible, "UIObject_SetVisible"),
    (0x003a_4af8, ui_object_set_color, "UIObject_SetColor"),
    (0x003a_4c90, ui_object_get_vec3_at60, "UIObject_GetVec3At60"),
    (0x003a_4cb0, ui_object_set_vec3_at60, "UIObject_SetVec3At60"),
    (0x003a_4cd0, ui_input_visitor_dtor, "UIInputVisitor_dtor"),
    (0x003a_4d00, ui_input_visitor_visit, "UIInputVisitor_Visit"),
    (0x003a_4d40, ui_focus_visitor_dtor, "UIFocusVisitor_dtor"),
    (0x003a_4d70, ui_focus_visitor_visit, "UIFocusVisitor_Visit"),
    (0x003a_5068, c_ui_group_dtor, "cUIGroup_dtor"),
    (0x003a_51a0, c_ui_group_accept, "cUIGroup_Accept"),
    (0x003a_51d0, ui_group_v03, "cUIGroup_v03"),
    (0x003a_52a0, ui_menu_dtor, "UIMenu_dtor"),
    (0x003a_54b8, ui_pair_dtor, "UIPair_dtor"),
    (0x003a_5658, ui_list_box_dtor, "UIListBox_dtor"),
    (0x003a_5708, ui_list_box_v24, "UIListBox_v24"),
    (0x003a_5b48, ui_thread_dtor, "UIThread_dtor"),
    (0x003a_5da8, ui_text_dtor, "UIText_dtor"),
    (0x003a_5e68, ui_text_v23, "UIText_v23"),
    (0x003a_5e88, ui_text_v24, "UIText_v24"),
    (0x003a_5ea8, ui_text_v25, "UIText_v25"),
    (0x003a_5ec8, ui_text_v26, "UIText_v26"),
    (0x003a_5ee8, ui_text_v27, "UIText_v27"),
    (0x003a_62a0, c_ui_text_scroll_dtor, "cUITextScroll_dtor"),
    (0x003a_6378, c_ui_text_scroll_v23, "cUITextScroll_v23"),
    (0x003a_6398, c_ui_text_scroll_v24, "cUITextScroll_v24"),
    (
        0x003a_36d0,
        ui_vector_set_anim_param,
        "UIVector_SetAnimParam",
    ),
    (0x003a_39f8, ui_vector_render, "UIVector_Render"),
    (
        0x001d_d950,
        ui_object_set_vec3_at_9c,
        "UIObject_SetVec3At9c",
    ),
    (0x003a_34c0, cui_vector_set_ui_data, "cUIVector_setUIData"),
    (0x003a_4e80, uiicon_dtor, "UIIcon_dtor"),
    (0x003a_3f08, ui_vector_return_self, "UIVector_ReturnSelf"),
    (
        0x003a_3f48,
        oc1079_ui_engine_static_init_destroy,
        "UIEngine_StaticInitDestroy",
    ),
    (0x003a_4900, ui_object_ctor_2, "UIObject_ctor_2"),
    (0x003a_4da8, ui_sound_base_dtor, "UISound_BaseDtor"),
    (0x003a_4df0, uiicon_ctor_2, "UIIcon_ctor_2"),
    (0x003a_4e38, uiicon_ctor, "UIIcon_ctor"),
    (0x003a_4fb8, c_ui_group_set_offset, "cUIGroup_SetOffset"),
    (0x003a_4fe0, c_ui_group_ctor_2, "cUIGroup_ctor_2"),
    (0x003a_5028, c_ui_group_ctor, "cUIGroup_ctor"),
    (0x003a_5100, c_ui_group_first_child, "cUIGroup_FirstChild"),
    (0x003a_5120, c_ui_group_last_child, "cUIGroup_LastChild"),
    (0x003a_5140, ui_group_get_child_at, "cUIGroup_GetChildAt"),
    (
        0x003a_5160,
        ui_group_push_front_child,
        "cUIGroup_PushFrontChild",
    ),
    (
        0x003a_5180,
        ui_group_push_back_child,
        "cUIGroup_PushBackChild",
    ),
    (0x003a_5230, ui_menu_ctor, "UIMenu_ctor"),
    (0x003a_5420, ui_pair_ctor_2, "UIPair_ctor_2"),
    (0x003a_5468, ui_pair_ctor, "UIPair_ctor"),
    (0x003a_5530, ui_pair_set_first, "UIPair_SetFirst"),
    (0x003a_5588, ui_pair_set_second, "UIPair_SetSecond"),
    (0x003a_55d8, ui_list_box_ctor_2, "UIListBox_ctor_2"),
    (0x003a_57b8, ui_state_ctor_3, "UIState_ctor_3"),
    (0x003a_5b98, ui_screen_ctor, "UIScreen_ctor"),
    (0x003a_5c08, ui_screen_ctor_2, "UIScreen_ctor_2"),
    (
        0x003a_5cf0,
        ui_screen_add_root_child,
        "UIScreen_AddRootChild",
    ),
    (0x003a_5d60, ui_text_ctor_2, "UIText_ctor_2"),
    (0x003a_5f60, cui_scroll_ctor_2, "cUIScroll_ctor_2"),
    (0x003a_5fa0, cui_scroll_ctor, "cUIScroll_ctor"),
    (
        0x003a_6038,
        ui_scroll_set_up_thread_name,
        "cUIScroll_SetUpThreadName",
    ),
    (
        0x003a_6088,
        ui_scroll_set_down_thread_name,
        "cUIScroll_SetDownThreadName",
    ),
    (0x003a_60e0, ui_progress_ctor_2, "cUIProgress_ctor_2"),
    (0x003a_6120, ui_progress_ctor, "cUIProgress_ctor"),
    (0x003a_6190, ui_slider_ctor_2, "cUISlider_ctor_2"),
    (0x003a_61d0, ui_slider_ctor, "cUISlider_ctor"),
    (0x003a_6250, ui_text_scroll_ctor_2, "cUITextScroll_ctor_2"),
    (
        0x003a_63e0,
        ui_text_scroll_list_first,
        "cUITextScroll_ListFirst",
    ),
    (
        0x003a_6400,
        cui_text_scroll_list_last,
        "cUITextScroll_ListLast",
    ),
    (
        0x003a_6420,
        cui_text_scroll_list_get_at,
        "cUITextScroll_ListGetAt",
    ),
    (
        0x003a_6440,
        cui_text_scroll_list_push_front,
        "cUITextScroll_ListPushFront",
    ),
    (
        0x003a_6460,
        cui_text_scroll_list_push_back,
        "cUITextScroll_ListPushBack",
    ),
    (0x003a_64f8, ui_vector_ctor_2, "UIVector_ctor_2"),
    (0x003a_6548, ui_object_flags_ctor, "UIObjectFlags_ctor"),
    (0x003a_65a0, ui_list_box_flags_ctor, "UIListBoxFlags_ctor"),
    (0x003a_65b8, ui_list_box_scroll_ctor, "UIListBoxScroll_ctor"),
    (0x003a_65d8, ui_state_flags_ctor, "UIStateFlags_ctor"),
    (0x003a_65e8, ui_text_flags_ctor, "UITextFlags_ctor"),
    (
        0x003a_6600,
        c_ui_text_scroll_flags_ctor,
        "cUITextScrollFlags_ctor",
    ),
    (0x003a_6618, ui_vector_flags_ctor, "UIVectorFlags_ctor"),
    (0x003a_6648, ui_engine_global_ctors, "UIEngine_GlobalCtors"),
    (0x003a_6668, ui_engine_global_dtors, "UIEngine_GlobalDtors"),
    (
        0x003a_0274,
        ui_object_anim_loop_tail,
        "UIObject_AnimLoopTail",
    ),
    (
        0x003a_3808,
        ui_vector_set_anim_params,
        "UIVector_SetAnimParams",
    ),
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
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_4;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR_3;

    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE;

    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;

    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR as UI_OBJECT_DTOR_2;

    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;

    pub const UI_PAIR_FREE_CHILDREN: Callee = Callee {
        addr: 0x0039_be70,
        f: ee::s13::f_0039be70,
    };

    pub const UI_LIST_BOX_CLEAR: Callee = Callee {
        addr: 0x0039_a670,
        f: ee::s13::f_0039a670,
    };

    pub const UI_TEXT_FREE_STRINGS: Callee = Callee {
        addr: 0x003a_0b38,
        f: ee::s13::f_003a0b38,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
}

/// `UIVisitor` destroying destructor (0x3a47a8): the visitor vtable
/// `D_00494CE8`.
pub fn ui_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_47cc, 0x0049_4ce8, 0)
    })
}

/// `UIInputVisitor` destroying destructor (0x3a4cd0).
pub fn ui_input_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_4cf4, 0x0049_4ce8, 0)
    })
}

/// `UIFocusVisitor` destroying destructor (0x3a4d40).
pub fn ui_focus_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_4d64, 0x0049_4ce8, 0)
    })
}

/// `cUIGroup::Accept(self, node)` (0x3a51a0): `node->v02(self)`.
pub fn c_ui_group_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x003a_51c4, None))
}

/// `int UIFocusVisitor_Visit(UIList* list, UIObject* obj)` (0x3a4d70): call
/// `obj->vtable[0x38/4]` with the visitor state `list+4`, then return 0.
pub fn ui_focus_visitor_visit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x003a_4d94)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `void* UIInputVisitor_Visit(Visitor* unused, UIObject* obj /*a1*/)`
/// (0x3a4d00): call the object's accept method (virtual slot 0x1e) with
/// `obj->+4`; return the object (or null when it was not accepted).
pub fn ui_input_visitor_visit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[V1][0] = Addr(r32(c, S0)).off(8).lw(c);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x78).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x7c).lw(c);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x003a_4d28);
        c.g[A0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x003a_4d28 {
            return Err(crate::call::Unwound);
        }
        if c.g[V0][0] == 0 {
            c.g[S0][0] = 0;
        }
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    })
}

/// `void UIVector_UpdateAnim(UIVector* v, ushort event)` (0x3a3d28): the same
/// for a vector widget.
pub fn ui_vector_update_anim(c: &mut Cpu) {
    guest(c, |c| ui_anim_update(c, 0x003a_3d50, 0x003a_3d5c))
}

/// The shared body of the three derived UI destructors: install `vtable` at
/// `+8`, run the class's own teardown (`clear`), then `UIObject_dtor`.
pub(crate) fn ui_derived_dtor(c: &mut Cpu, vtable: u32, clear: Callee, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(vtable);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).off(8).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        clear.call(c, ret1)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        UI_OBJECT_DTOR.call(c, ret2)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIPair_dtor(this, flags)` (0x3a54b8).
pub fn ui_pair_dtor(c: &mut Cpu) {
    ui_derived_dtor(
        c,
        0x0049_4868,
        UI_PAIR_FREE_CHILDREN,
        0x003a_54e0,
        0x003a_54ec,
    )
}

/// `void UIListBox_dtor(this, flags)` (0x3a5658).
pub fn ui_list_box_dtor(c: &mut Cpu) {
    ui_derived_dtor(c, 0x0049_4798, UI_LIST_BOX_CLEAR, 0x003a_5680, 0x003a_568c)
}

/// `void UIText_dtor(this, flags)` (0x3a5da8).
pub fn ui_text_dtor(c: &mut Cpu) {
    ui_derived_dtor(
        c,
        0x0049_4588,
        UI_TEXT_FREE_STRINGS,
        0x003a_5dd0,
        0x003a_5ddc,
    )
}

/// `void UIListNode_dtor(UIListNode* self, int flags)` (0x3a47f8): install
/// the list-node vtable, run the body and maybe delete the node.
pub fn ui_list_node_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_4cc0; // D_00494CC0
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_4820)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_4834)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIThread_dtor(UIThread* self, int flags)` (0x3a5b48): the thread
/// node shares the list-node body and vtable.
pub fn ui_thread_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_4cc0; // D_00494CC0
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_5b70)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x003a_5b84)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Field offsets.

/// The UI widget flag words: `UIObject` at `+0x14`, its subclasses at `+0x74`.
pub(crate) const UI_FLAGS_OBJECT: u32 = 0x14;

pub(crate) const UI_FLAGS_WIDGET: u32 = 0x74;

/// `sVec3` at `+0x60` of a UI object, `+0x1c` the colour, `+0x80` the point
/// array, `+0x74` the `{lo:17, count:15}` size field.
pub(crate) const UI_VEC3: u32 = 0x60;

pub(crate) const UI_COLOR: u32 = 0x1c;

pub(crate) const UI_POINTS: u32 = 0x80;

pub(crate) const UI_SIZE: u32 = 0x74;

/// The low 32 bits of a register (a pointer).
#[inline(always)]
pub(crate) fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

// ---------------------------------------------------------------------------
// UI widget flags.

/// A `bitN: 1` setter: `w = *(self + off); w = (w & !bit) | (enable != 0) << n;
/// *(self + off) = w`, leaving the word in `v0` and the mask/value in `v1`/`a1`
/// as the original does.
pub(crate) fn set_ui_flag(c: &mut Cpu, off: u32, bit: u32) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    c.g[V0][0] = c.lw(p(ui, off));
    c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
    c.g[V1][0] = sx(!(1u32 << bit));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << bit);
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A1][0];
    let ra = r(c, RA);
    c.sw(p(ui, off), c.g[V0][0]);
    c.pc = ra;
}

/// `UIObject_v04` (0x003a49e8): `bit1 = enable` of the flag word at `+0x14`.
pub fn ui_object_v04(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 1);
}

/// `UIObject_v05` (0x003a4a08): `bit2 = enable`.
pub fn ui_object_v05(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 2);
}

/// `UIObject_v06` (0x003a4a28): `bit3 = enable`.
pub fn ui_object_v06(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 3);
}

/// `UIObject_SetFocus` (0x003a4a48): `bit4 = enable` (the focused state).
pub fn ui_object_set_focus(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 4);
}

/// `UIObject_v08` (0x003a4a68): `bit5 = enable`.
pub fn ui_object_v08(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 5);
}

/// `UIObject_SetVisible` (0x003a4a88): `bit6 = enable` (the visible state).
pub fn ui_object_set_visible(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 6);
}

/// `UIListBox_v24` (0x003a5708): `bit5 = enable` of the widget flag word at
/// `+0x74`.
pub fn ui_list_box_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 5);
}

/// `UIText_v23` (0x003a5e68): `bit2 = enable` of the widget flag word.
pub fn ui_text_v23(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 2);
}

/// `UIText_v24` (0x003a5e88): `bit3 = enable`.
pub fn ui_text_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 3);
}

/// `UIText_v25` (0x003a5ea8): `bit4 = enable`.
pub fn ui_text_v25(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 4);
}

/// `UIText_v26` (0x003a5ec8): `bit5 = enable`.
pub fn ui_text_v26(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 5);
}

/// `UIText_v27` (0x003a5ee8): `bit6 = enable`.
pub fn ui_text_v27(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 6);
}

/// `cUITextScroll_v23` (0x003a6378): `bit3 = enable`.
pub fn c_ui_text_scroll_v23(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 3);
}

/// `cUITextScroll_v24` (0x003a6398): `bit4 = enable`.
pub fn c_ui_text_scroll_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 4);
}

// ---------------------------------------------------------------------------
// UI vec3 accessors and colour.

/// `UIObject_SetColor` (0x003a4af8): `*(self + 0x1c) = *in` (16 bytes).
pub fn ui_object_set_color(c: &mut Cpu) {
    c.cycles += 10;
    let ui = abi::arg(c, 0);
    let color = abi::arg(c, 1);
    c.ldl(2, p(color, 7));
    c.ldr(2, color);
    c.ldl(3, p(color, 0xf));
    c.ldr(3, p(color, 8));
    c.sdl(2, p(ui, UI_COLOR + 7));
    c.sdr(2, p(ui, UI_COLOR));
    c.sdl(3, p(ui, UI_COLOR + 0xf));
    c.sdr(3, p(ui, UI_COLOR + 8));
    abi::ret(c);
}

/// `UIObject_GetVec3At60` (0x003a4c90): `*out = *(sVec3*)(self + 0x60)`.
pub fn ui_object_get_vec3_at60(c: &mut Cpu) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    c.ldl(2, p(ui, UI_VEC3 + 7));
    c.ldr(2, p(ui, UI_VEC3));
    c.g[V1][0] = c.lw(p(ui, UI_VEC3 + 8));
    c.sdl(2, p(out, 7));
    c.sdr(2, out);
    c.sw(p(out, 8), c.g[V1][0]);
    abi::ret(c);
}

/// `UIObject_SetVec3At60` (0x003a4cb0): `*(sVec3*)(self + 0x60) = *in`.
pub fn ui_object_set_vec3_at60(c: &mut Cpu) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    let v = abi::arg(c, 1);
    c.ldl(2, p(v, 7));
    c.ldr(2, v);
    c.g[V1][0] = c.lw(p(v, 8));
    c.sdl(2, p(ui, UI_VEC3 + 7));
    c.sdr(2, p(ui, UI_VEC3));
    c.sw(p(ui, UI_VEC3 + 8), c.g[V1][0]);
    abi::ret(c);
}

/// `UIVector_v20` (0x003a3d70): `getBounds(out)` — copy the default 12-byte
/// extent to `out`, then grow it to the x/y extent of the `count = size>>17`
/// points at `+0x80` (1000 / -1000 sentinels). No data: return at once.
pub fn ui_vector_v20(c: &mut Cpu) {
    c.cycles += 3;
    let vec = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    c.g[V0][0] = c.lw(p(vec, UI_POINTS));
    let no_points = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0050_0000;
    if no_points {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // `D_004FF0D8`, the 12-byte default extent.
    c.cycles += 17;
    let default_extent = 0x0050_0000u32.wrapping_sub(0xf28);
    c.g[AT][0] = 0x447a_0000;
    c.mtc1(1, 3); // f3 = 1000.0
    c.g[T0][0] = sx(default_extent);
    c.ldl(3, p(default_extent, 7));
    c.ldr(3, default_extent);
    c.g[A2][0] = c.lw(p(default_extent, 8));
    c.sdl(3, p(out, 7));
    c.sdr(3, out);
    c.sw(p(out, 8), c.g[A2][0]);
    c.g[AT][0] = sx(0xc47a_0000);
    c.mtc1(1, 0); // f0 = -1000.0
    c.g[V1][0] = 0;
    c.g[V0][0] = c.lw(p(vec, UI_SIZE));
    c.mov_s(4, 3); // f4 = 1000.0
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
    c.mov_s(1, 0); // f1 = -1000.0
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = c.lw(p(vec, UI_POINTS));
        c.g[A2][0] = c.g[V0][0];
        loop {
            c.cycles += 5;
            c.lwc1(2, r(c, A0));
            c.c_lt_s(2, 3);
            if c.fcc() {
                c.mov_s(3, 2); // minX = x
            }
            c.cycles += 4;
            c.c_lt_s(0, 2);
            if c.fcc() {
                c.mov_s(0, 2); // maxX = x
            }
            c.cycles += 5;
            c.lwc1(2, p(r(c, A0), 4));
            c.c_lt_s(2, 4);
            if c.fcc() {
                c.mov_s(4, 2); // minY = y
            }
            c.cycles += 4;
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.mov_s(1, 2); // maxY = y
            }
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_3dc8);
            }
        }
    }
    c.cycles += 4;
    c.sub_s(0, 0, 3); // maxX - minX
    c.sub_s(1, 1, 4); // maxY - minY
    c.swc1(0, out);
    c.swc1(1, p(out, 4));
    c.cycles += 2;
    abi::ret(c);
}

/// `UIVector_v21` (0x003a3e38): when the vector holds exactly four points,
/// fan them out from point 0 and add the x/y offset pair to points 1/3 and
/// 2; then `*(self + 0x60) = *off`. The `{lo:17, count:15}` field at `+0x74`
/// must read 4.
pub fn ui_vector_v21(c: &mut Cpu) {
    c.cycles += 7;
    let vec = abi::arg(c, 0);
    let off = abi::arg(c, 1);
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = sx(0xfffe_0000);
    c.g[V1][0] = c.lw(p(vec, UI_SIZE));
    c.g[A0][0] = 0x8_0000;
    c.g[V1][0] &= c.g[V0][0];
    let not_four = c.g[V1][0] != c.g[A0][0];
    c.g[T0][0] = 0x0200_0000;
    if !not_four {
        c.cycles += 3;
        c.g[A3][0] = 0xc;
        c.g[T1][0] = 0x0100_0000;
        loop {
            c.cycles += 14;
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.g[V1][0] = c.g[T0][0];
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(r(c, V0)));
            c.g[V1][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
            c.ldl(10, p(r(c, V0), 7));
            c.ldr(10, r(c, V0));
            c.g[T3][0] = c.lw(p(r(c, V0), 8));
            c.sdl(10, p(r(c, A0), 7));
            c.sdr(10, r(c, A0));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
            let more = c.g[V1][0] != 0;
            c.sw(p(r(c, A0), 8), c.g[T3][0]);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003a_3e60);
                }
                continue;
            }
            c.cycles += 20;
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, off);
            c.lwc1(0, p(r(c, V0), 0xc));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V0), 0xc));
            c.g[V1][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, p(off, 4));
            c.lwc1(0, p(r(c, V1), 0x28));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V1), 0x28));
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, off);
            c.lwc1(0, p(r(c, V0), 0x18));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V0), 0x18));
            c.g[V1][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, p(off, 4));
            c.lwc1(0, p(r(c, V1), 0x1c));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V1), 0x1c));
            break;
        }
    }
    c.cycles += 8;
    c.ldl(2, p(off, 7));
    c.ldr(2, off);
    c.g[V1][0] = c.lw(p(off, 8));
    c.sdl(2, p(vec, UI_VEC3 + 7));
    c.sdr(2, p(vec, UI_VEC3));
    c.sw(p(vec, UI_VEC3 + 8), c.g[V1][0]);
    abi::ret(c);
}

/// `void* cUIGroup_v03(cUIGroup* self, void* fn)` (0x3a51d0): let the functor
/// claim the group; when it declines, offer it to the group's list.
pub fn ui_group_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p_2(c.g[V1][0], 0x10)).lh(c);
        c.g[V0][0] = lw(c, p_2(c.g[V1][0], 0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x003a_5200)?;
        let claimed = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if claimed {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0039_7a68, 0x003a_5214)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// UI construction / teardown.

/// `void UIList_dtor(self, flags)` (0x3a4888): deleting destructor of a UI
/// list — delete the entries, reset the two embedded list nodes and free the
/// object when `flags & 1`.
pub fn ui_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = 0x49_0000;
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x4c98));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        sw_r(c, S2, 0x18, c.g[V0][0]);
        callee::UI_LIST_DELETE_ALL.call(c, 0x003a_48b4)?;
        c.g[S0][0] = 0x49_0000;
        c.g[A0][0] = sx(low(c, S2).wrapping_add(0xc));
        c.g[S0][0] = sx(low(c, S0).wrapping_add(0x4cc0));
        sw_r(c, S2, 0x14, c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY_2.call(c, 0x003a_48c8)?;
        sw_r(c, S2, 0x8, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY_2.call(c, 0x003a_48d4)?;
        c.g[S1][0] = c.g[S1][0] & 0x1;
        let keep = c.g[S1][0] == 0;
        c.lq(16, sp_2(c).wrapping_add(0x30));
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x003a_48e8)?;
            c.lq(16, sp_2(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI groups / menus.

/// `void cUIGroup_dtor(cUIGroup* self, int flags)` (0x3a5068): republish the
/// vtables, empty the child list, destroy the two embedded list nodes and run
/// the base object destructor.
pub fn c_ui_group_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V1][0] = 0x0049_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x49e8));
        frame.save(c, S3, 0x10);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x4c98));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(p(r32(c, S1), 0x74));
        c.g[S3][0] = c.g[A1][0];
        Addr(p(r32(c, S1), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S1), 0x8c)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x003a_50ac)?;
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(p(r32(c, S1), 0x80));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x4cc0));
        Addr(p(r32(c, S1), 0x88)).sw(c, c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_50c0)?;
        Addr(p(r32(c, S1), 0x7c)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_50cc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        callee::UI_OBJECT_DTOR_2.call(c, 0x003a_50d8)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIMenu_dtor(UIMenu* self, int flags)` (0x3a52a0): the same teardown
/// as [`c_ui_group_dtor`] with the menu vtables.
pub fn ui_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V1][0] = 0x0049_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x49e8));
        frame.save(c, S3, 0x10);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x4c98));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(p(r32(c, S1), 0x74));
        c.g[S3][0] = c.g[A1][0];
        Addr(p(r32(c, S1), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S1), 0x8c)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x003a_52e4)?;
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(p(r32(c, S1), 0x80));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x4cc0));
        Addr(p(r32(c, S1), 0x88)).sw(c, c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_52f8)?;
        Addr(p(r32(c, S1), 0x7c)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY_3.call(c, 0x003a_5304)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        callee::UI_OBJECT_DTOR_2.call(c, 0x003a_5310)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUITextScroll_dtor(cUITextScroll* self, int flags)` (0x3a62a0):
/// reset the vtable slots, drop the scroll lines and both list nodes, then
/// run the UI object destructor.
pub fn c_ui_text_scroll_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(p(r32(c, V0), 0x4278));
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(p(self_, 0x8c));
        frame.save_ra(c, 0);
        c.g[S3][0] = flags;
        c.sw(at(c, S1, 8), c.g[V0][0]);
        call_ptr(c, callee::CUI_TEXT_SCROLL_CLEAR_LINES, 0x003a_62d4)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = sx(p(r32(c, V0), 0x4c98));
        c.sw(at(c, S1, 0xa4), c.g[V0][0]);
        call_ptr(c, callee::UI_LIST_DELETE_ALL_2, 0x003a_62e8)?;
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(p(r32(c, S1), 0x98));
        c.g[S0][0] = sx(p(r32(c, S0), 0x4cc0));
        c.sw(at(c, S1, 0xa0), c.g[S0][0]);
        call_ptr(c, callee::UI_LIST_NODE_DTOR_BODY_4, 0x003a_62fc)?;
        c.sw(at(c, S1, 0x94), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::UI_LIST_NODE_DTOR_BODY_4, 0x003a_6308)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::UI_OBJECT_DTOR_3, 0x003a_6314)?;
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
// UIVector_Render (0x3a39f8)

/// `cUIVector` fields.
pub(crate) const UI_VECTOR_POSITION_X: u32 = 0x44;

pub(crate) const UI_VECTOR_SCALE_X: u32 = 0x50;

/// `count << 17 | mode << 2 | indexed` (bits 17.. vertex count, bit 0: draw indexed triangles).
pub(crate) const UI_VECTOR_COUNTS: u32 = 0x74;

pub(crate) const UI_VECTOR_TRIANGLES: u32 = 0x78;

pub(crate) const UI_VECTOR_COLOURS: u32 = 0x7c;

pub(crate) const UI_VECTOR_POSITIONS: u32 = 0x80;

pub(crate) const UI_VECTOR_RENDER_POSITIONS: u32 = 0x84;

pub(crate) const UI_VECTOR_VERTICES: u32 = 0x8c;

/// `GrMan` (the graphics manager) at `gp - 0x854`; its current draw state
/// at `+0xe84`, its vtable at `+0x10d8`.
pub(crate) const UI_VECTOR_GRMAN_GP: u32 = 0x854;

/// `__fixunssfsi` (float to unsigned int).
pub(crate) const UI_VECTOR_FIXUNSSFSI: u32 = 0x0041_39f8;

/// `void cUIVector::render2D(cUIVector* this, float x, float y)` (0x3a39f8):
/// scale the vertex positions into the render copy, set the draw layer, then
/// either draw indexed triangles through GrMan (vtable `+0x240`) or build
/// screen vertices (position + offset, colour * 0.5 as integers) and draw
/// them as a strip (vtable `+0x248`).
pub fn ui_vector_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x40);
        frame.save_fpr(c, 23, 0x98);
        c.g[S3][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x90);
        c.mov_s(23, 13);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        let this = c.g[S3][0];
        c.g[V0][0] = ui_vector_lw(c, this, UI_VECTOR_POSITIONS);
        c.mov_s(22, 12);
        if c.g[V0][0] != 0 {
            if ui_vector_render_body(c, this, frame)? {
                // Indexed draw: s0 already reloaded in the call's delay slot.
                ui_vector_render_epilogue(c, frame);
                return Ok(());
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
        ui_vector_render_epilogue(c, frame);
        Ok(())
    })
}

pub(crate) fn ui_vector_render_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 23, 0x98);
    frame.restore_fpr(c, 22, 0x90);
    frame.restore_fpr(c, 21, 0x88);
    frame.restore_fpr(c, 20, 0x80);
    frame.close(c);
}

/// The body after the `mpVertexPosition != 0` test; `Ok(true)` when it took
/// the indexed draw (which reloads s0 itself).
pub(crate) fn ui_vector_render_body(
    c: &mut Cpu,
    this: u64,
    frame: Frame,
) -> Result<bool, crate::call::Unwound> {
    // Render positions = positions scaled by mScale.
    c.cycles += 4;
    c.g[V0][0] = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
    c.g[A2][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 28;
            let off = c.g[A1][0] as u32;
            let dst = (ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS) as u32).wrapping_add(off);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            let src = (ui_vector_lw(c, this, UI_VECTOR_POSITIONS) as u32).wrapping_add(off);
            c.g[A0][0] = sx(dst);
            c.g[V0][0] = sx(src);
            let xy = Addr(src).read_u64_unaligned(c);
            c.g[V1][0] = xy;
            let z = Addr(src).off(8).lw(c);
            c.g[A3][0] = z;
            Addr(dst).write_u64_unaligned(c, xy);
            Addr(dst).off(8).sw(c, z);
            for (k, scale) in [(0u32, UI_VECTOR_SCALE_X), (4, UI_VECTOR_SCALE_X + 4)] {
                let p =
                    (ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS) as u32).wrapping_add(off);
                if k == 0 {
                    c.g[V1][0] = sx(p);
                } else {
                    c.g[A0][0] = sx(p);
                }
                let s = Addr((this as u32).wrapping_add(scale)).read_fpu(c);
                c.f[1] = s.0;
                let v = Addr(p).off(k).read_fpu(c) * s;
                c.f[0] = v.0;
                Addr(p).off(k).write_fpu(c, v);
            }
            let n = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
            let more = c.g[A2][0] < n;
            c.g[V0][0] = more as u64;
            c.g[A1][0] = sx(off.wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_3a50);
            }
        }
    }

    // Draw state: layer (flags +0x15 & 0x1f) + 9 into bits 5..9, word +0x10 = -1.
    c.cycles += 19;
    let gp = c.g[GP][0] as u32;
    let grman = Addr(gp.wrapping_sub(UI_VECTOR_GRMAN_GP)).lw(c);
    c.g[A2][0] = grman;
    c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
    c.g[A3][0] = u64::MAX;
    let layer = Addr((this as u32).wrapping_add(0x15)).lbu(c) & 0x1f;
    let ds = ui_vector_lw(c, grman, 0xe84);
    c.g[A0][0] = ds;
    let bits = sx(((layer as u32).wrapping_add(9)) << 5) & 0x3e0;
    c.g[V0][0] = bits;
    let w = (ui_vector_lw(c, ds, 8) & 0xffff_ffff_ffff_fc1f) | bits;
    c.g[V1][0] = w;
    Addr((ds as u32).wrapping_add(8)).sw(c, w);
    let ds2 = ui_vector_lw(c, grman, 0xe84);
    c.g[V0][0] = ds2;
    Addr((ds2 as u32).wrapping_add(0x10)).sh(c, u64::MAX);
    let counts = ui_vector_lw(c, this, UI_VECTOR_COUNTS);
    c.g[A2][0] = counts;
    c.g[V0][0] = counts & 1;
    c.g[A1][0] = sx((counts as u32) >> 2);
    if counts & 1 != 0 {
        // Indexed triangles: GrMan->vt[0x240](mode, count, tris, render positions, 0, colours, pos).
        c.cycles += 22;
        let x = Addr((this as u32).wrapping_add(UI_VECTOR_POSITION_X)).read_fpu(c);
        let y = Addr((this as u32).wrapping_add(UI_VECTOR_POSITION_X + 4)).read_fpu(c);
        c.g[A1][0] &= 0x7fff;
        let sp = frame.at(c, 0);
        Addr(sp).off(8).sw(c, 0);
        let px = x + FpuF32(c.f[22]);
        let py = y + FpuF32(c.f[23]);
        c.f[0] = px.0;
        c.f[1] = py.0;
        let t4 = Addr(gp.wrapping_sub(UI_VECTOR_GRMAN_GP)).lw(c);
        c.g[T4][0] = t4;
        c.g[T2][0] = ui_vector_lw(c, this, UI_VECTOR_COLOURS);
        c.g[A2][0] = sx((counts as u32) >> 17);
        Addr(sp).write_fpu(c, px);
        c.g[T1][0] = 0;
        Addr(sp).off(4).write_fpu(c, py);
        c.g[T3][0] = c.g[SP][0];
        let vt = ui_vector_lw(c, t4, 0x10d8);
        c.g[V0][0] = vt;
        c.g[A3][0] = ui_vector_lw(c, this, UI_VECTOR_TRIANGLES);
        let delta = Addr((vt as u32).wrapping_add(0x240)).lh(c);
        c.g[T0][0] = ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS);
        let func = ui_vector_lw(c, vt, 0x244);
        c.g[V1][0] = func;
        c.g[A0][0] = sx((t4 as u32).wrapping_add(delta as u32));
        call_ptr(c, func as u32, 0x003a_3b5c)?;
        frame.restore(c, S0, 0x70);
        return Ok(true);
    }

    c.cycles += 3;
    let verts = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
    let count = sx((counts as u32) >> 17);
    c.g[V0][0] = count;
    if verts == 0 {
        return Ok(false);
    }
    c.cycles += 2;
    c.g[S4][0] = 0;
    if count != 0 {
        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000;
        c.f[21] = 0x3f80_0000; // 1.0f
        c.g[AT][0] = 0x3f00_0000;
        c.f[20] = 0x3f00_0000; // 0.5f
        c.g[V0][0] = 0xc;
        loop {
            // Vertex i (0x30 bytes): pos (x, y, z, 1) + offset at +0x20, q = 1, s = t = 0,
            // colour (A, R, G, B) * 0.5 as integers at +0x1c, +0x10, +0x14, +0x18.
            c.cycles += 80;
            let i = c.g[S4][0];
            let rp = ui_vector_lw(c, this, UI_VECTOR_RENDER_POSITIONS);
            c.g[V1][0] = rp;
            c.mult(0, V0, S4, V0);
            let v = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
            c.g[A0][0] = v;
            c.g[S1][0] = 0x30;
            c.g[S2][0] = sx((i as u32) << 4);
            c.mult(0, S1, S4, S1);
            c.g[S4][0] = sx((i as u32).wrapping_add(1));
            let s1 = c.g[S1][0] as u32;
            let p = (c.g[V0][0] as u32).wrapping_add(rp as u32);
            c.g[V0][0] = sx(p);
            let tmp = frame.at(c, 0x10);
            c.lwc1(1, p);
            c.g[A0][0] = sx(s1.wrapping_add(v as u32));
            c.swc1(1, tmp);
            c.lwc1(0, p.wrapping_add(4));
            c.swc1(0, tmp + 4);
            c.lwc1(1, p.wrapping_add(8));
            c.swc1(21, tmp + 0xc);
            c.swc1(1, tmp + 8);
            c.lq(V0, tmp);
            c.sq(V0, (c.g[A0][0] as u32).wrapping_add(0x20));
            // pos.x += mPosition.x + x; pos.y += mPosition.y + y.
            for k in [0u32, 4] {
                let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
                let at = vtx.wrapping_add(0x20);
                let reg = if k == 0 { V1 } else { A0 };
                c.g[reg][0] = sx(at);
                let base = Addr((this as u32).wrapping_add(UI_VECTOR_POSITION_X + k)).read_fpu(c);
                let off = base + FpuF32(c.f[if k == 0 { 22 } else { 23 }]);
                c.f[1] = off.0;
                let r = Addr(at).off(k).read_fpu(c) + off;
                c.f[0] = r.0;
                Addr(at).off(k).write_fpu(c, r);
            }
            let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
            c.g[V0][0] = sx(vtx);
            c.swc1(21, vtx.wrapping_add(8));
            let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
            c.g[V1][0] = sx(vtx);
            Addr(vtx).sw(c, 0);
            let vtx = (ui_vector_lw(c, this, UI_VECTOR_VERTICES) as u32).wrapping_add(s1);
            c.g[V0][0] = sx(vtx);
            Addr(vtx).off(4).sw(c, 0);
            // Colours: (channel offset in the colour, field in the vertex, return address).
            let s2 = c.g[S2][0] as u32;
            for (k, field, ret) in [
                (0u32, 0x1cu32, 0x003a_3c58u32),
                (4, 0x10, 0x003a_3c78),
                (8, 0x14, 0x003a_3c98),
                (0xc, 0x18, 0x003a_3cb8),
            ] {
                let colours = ui_vector_lw(c, this, UI_VECTOR_COLOURS);
                let vtx = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
                let col = s2.wrapping_add(colours as u32);
                let dst = s1.wrapping_add(vtx as u32);
                if k == 0xc {
                    c.g[V0][0] = colours;
                    c.g[V1][0] = vtx;
                    c.g[S2][0] = sx(col);
                    c.g[S1][0] = sx(dst);
                } else {
                    c.g[if k == 0 { V1 } else { V0 }][0] = sx(col);
                    c.g[S0][0] = sx(dst);
                }
                c.lwc1(12, col.wrapping_add(k));
                c.mul_s(12, 12, 20);
                call_ptr(c, UI_VECTOR_FIXUNSSFSI, ret)?;
                let reg = if k == 0xc { S1 } else { S0 };
                Addr((c.g[reg][0] as u32).wrapping_add(field)).sw(c, c.g[V0][0]);
            }
            let n = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
            let more = c.g[S4][0] < n;
            c.g[V0][0] = 0xc;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_3b90);
            }
        }
    }
    // Strip: GrMan->vt[0x248](count, vertices, 0).
    c.cycles += 10;
    let t0 = Addr(gp.wrapping_sub(UI_VECTOR_GRMAN_GP)).lw(c);
    c.g[T0][0] = t0;
    c.g[A3][0] = 0;
    let counts = ui_vector_lw(c, this, UI_VECTOR_COUNTS);
    let vt = ui_vector_lw(c, t0, 0x10d8);
    c.g[V1][0] = vt;
    c.g[A2][0] = ui_vector_lw(c, this, UI_VECTOR_VERTICES);
    c.g[A1][0] = sx((counts as u32) >> 17);
    let delta = Addr((vt as u32).wrapping_add(0x248)).lh(c);
    let func = ui_vector_lw(c, vt, 0x24c);
    c.g[V0][0] = func;
    c.g[A0][0] = sx((t0 as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, 0x003a_3cf8)?;
    Ok(false)
}

// ---------------------------------------------------------------------------
// UIVector_SetAnimParam (0x3a36d0)

/// `cUIVector` per-vertex channels: (base field, element shift or `None`
/// for the 12-byte position stride, offset in the element, cycles of the
/// case block before the common store).
pub(crate) fn ui_vector_anim_channel(sub: u64) -> (u32, Option<u32>, u32, u64) {
    match sub {
        0 => (UI_VECTOR_POSITIONS, None, 0, 4),    // vertex x
        1 => (UI_VECTOR_POSITIONS, None, 4, 4),    // vertex y
        2 => (UI_VECTOR_POSITIONS, None, 8, 4),    // vertex z
        3 => (0x88, Some(3), 0, 3),                // uv u
        4 => (0x88, Some(3), 4, 3),                // uv v
        5 => (UI_VECTOR_COLOURS, Some(4), 0, 2),   // colour r
        6 => (UI_VECTOR_COLOURS, Some(4), 4, 2),   // colour g
        7 => (UI_VECTOR_COLOURS, Some(4), 8, 2),   // colour b
        _ => (UI_VECTOR_COLOURS, Some(4), 0xc, 2), // colour a
    }
}

/// `void cUIVector::setChannelData(cUIVector* this, u16 channel, float v)`
/// (0x3a36d0): channels below 0x15 go to `cUIThing::setChannelData`; the
/// rest address vertex `(ch - 0x15) / 9`, field `(ch - 0x15) % 9` (position
/// xyz, uv, colour rgba). Nothing without vertices or past the count.
pub fn ui_vector_set_anim_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[A2][0] = this;
        frame.save_ra(c, 0);
        let verts = ui_vector_lw(c, this, UI_VECTOR_POSITIONS);
        c.g[V0][0] = verts;
        let key = c.g[A1][0] & 0xffff;
        c.g[A1][0] = key;
        if verts != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx((key as u32).wrapping_sub(0x15));
            if key < 0x15 {
                c.cycles += 2;
                c.g[A0][0] = this;
                Callee {
                    addr: 0x003a_0048,
                    f: ee::s13::UIObject_SetAnimParam,
                }
                .call(c, 0x003a_37f8)?;
            } else {
                c.cycles += 4 + 9;
                c.g[V1][0] = 9;
                let ch = c.g[V0][0] & 0xffff;
                c.g[A1][0] = ch;
                c.divu(0, A1, V1);
                let count = sx((ui_vector_lw(c, this, UI_VECTOR_COUNTS) as u32) >> 17);
                c.g[V1][0] = c.lo[0];
                c.g[A0][0] = c.hi[0];
                let idx = c.lo[0] & 0xffff;
                c.g[A1][0] = idx;
                c.g[V0][0] = 9;
                if idx < count {
                    c.cycles += 3 + 3;
                    let sub = c.hi[0] & 0xffff;
                    c.g[A0][0] = sub;
                    c.g[V1][0] = (sub < 9) as u64;
                    c.g[V0][0] = 0x49_0000;
                    if sub < 9 {
                        c.cycles += 6;
                        let entry = 0x0049_4110 + 4 * sub as u32;
                        c.g[V0][0] = 0x0049_4110;
                        c.g[V1][0] = sx(entry);
                        c.g[A0][0] = Addr(entry).lw(c);
                        let (field, shift, elem_off, pre) = ui_vector_anim_channel(sub);
                        c.cycles += pre;
                        let base = ui_vector_lw(c, this, field);
                        c.g[V1][0] = base;
                        match shift {
                            None => {
                                c.g[V0][0] = 0xc;
                                c.mult(0, V0, A1, V0);
                            }
                            Some(sh) => c.g[V0][0] = sx((idx as u32) << sh),
                        }
                        c.cycles += 3;
                        let at = (c.g[V0][0] as u32).wrapping_add(base as u32);
                        c.g[V0][0] = sx(at);
                        c.swc1(12, at.wrapping_add(elem_off));
                    }
                }
            }
        }
        c.cycles += 3;
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

/// `void cUIVector_setUIData(cUIVector* self, UIVectorData* data)`
/// (0x3a34c0): loads the base object (`cUIThing_setUIData`, 0x39fe00), then
/// sets the vector's flag bits from `data` (+0x22, +0x21) in the word at
/// +0x74, and allocates the element arrays for `data->count` (+0x20, a
/// signed byte): one of 16 bytes per element (self+0x80), two of 12 bytes
/// (+0x84 and self+0x7c, which holds its pointer on the second) and an
/// optional 48-byte block when the flag at +0x74 says so. The element copy
/// loop moves 16-byte records from the static table at 0x4ff0d8 into the
/// element array at +0x80.
pub fn cui_vector_set_ui_data(c: &mut Cpu) {
    guest(c, cui_vector_set_ui_data_body);
}

pub(crate) fn cui_vector_set_ui_data_body(c: &mut Cpu) -> Flow {
    c.cycles += 32;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x60);
    frame.save(c, S3, 0x30);
    frame.save(c, S4, 0x20);
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    c.g[RA][0] = sx(0x003a_34ec);
    c.g[S2][0] = c.g[A1][0];
    Callee {
        addr: 0x0039_fe00,
        f: ee::s13::f_0039fe00,
    }
    .call(c, 0x003a_34ec)?;
    // 'b_3a3548: the flag bits at +0x74.
    c.g[A0][0] = c.lbu(spch_ea(c.g[S2][0], 0x22));
    c.g[A2][0] = 0xffff_ffff_ffff_fffe;
    c.g[V0][0] = c.lw(spch_ea(c.g[S1][0], 0x74));
    c.g[V1][0] = 0xffff_ffff_ffff_fffd;
    c.g[A0][0] &= 0x1;
    c.g[A1][0] = 0xffff_ffff_fffe_0000;
    c.g[V0][0] &= c.g[A2][0];
    c.g[A1][0] |= 0x3;
    c.g[V0][0] |= c.g[A0][0];
    c.g[A2][0] = 0x10000;
    c.g[V1][0] &= c.g[V0][0];
    c.g[A2][0] |= 0xfffc;
    c.sw(spch_ea(c.g[S1][0], 0x74), c.g[V1][0]);
    c.g[V1][0] &= c.g[A1][0];
    c.g[V0][0] = c.lb(spch_ea(c.g[S2][0], 0x21));
    c.g[V0][0] &= 0x7fff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] |= c.g[V0][0];
    c.g[A2][0] &= c.g[V1][0];
    let t = c.g[A2][0] == 0;
    c.sw(spch_ea(c.g[S1][0], 0x74), c.g[V1][0]);
    if !t {
        c.cycles += 2;
        c.g[V0][0] = sx(spch_ea(c.g[S2][0], 0x24));
        c.sw(spch_ea(c.g[S1][0], 0x78), c.g[V0][0]);
    }
    'b_3a3660: {
        c.cycles += 11;
        c.g[A0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
        c.g[V1][0] = 0x10000;
        c.g[V0][0] = c.lw(spch_ea(c.g[S1][0], 0x74));
        c.g[V1][0] |= 0xffff;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 17);
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= c.g[A0][0];
        c.sw(spch_ea(c.g[S1][0], 0x74), c.g[V0][0]);
        c.g[S0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
        let t = c.g[S0][0] == 0;
        c.g[A2][0] = 0x100;
        if t {
            break 'b_3a3660;
        }
        'b_3a35a0: {
            c.cycles += 10;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 4);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40c8));
            c.g[A3][0] = 0;
            c.g[S3][0] = sx(spch_ea(c.g[S1][0], 0x7c));
            call_ptr(c, 0x0031_7e30, 0x003a_358c)?;
            c.g[S4][0] = sx(spch_ea(c.g[S1][0], 0x80));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            let t = c.g[S0][0] == 0;
            c.g[S5][0] = sx(spch_ea(c.g[S1][0], 0x84));
            if t {
                break 'b_3a35a0;
            }
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        'b_3a35d8: {
            c.cycles += 12;
            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = 0x49_0000;
            c.g[A2][0] = 0x100;
            c.g[S0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40d8));
            c.mult(0, 0, 16, 4);
            c.g[A0][0] = c.lo[0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x003a_35c8)?;
            let t = c.g[S0][0] == 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_3a35d8;
            }
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        'b_3a3610: {
            c.cycles += 12;
            c.sw(c.g[S4][0] as u32, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = 0x49_0000;
            c.g[A2][0] = 0x100;
            c.g[S0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40e8));
            c.mult(0, 0, 16, 4);
            c.g[A0][0] = c.lo[0];
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x003a_3600)?;
            let t = c.g[S0][0] == 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_3a3610;
            }
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 4;
        c.sw(c.g[S5][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if t {
            break 'b_3a3660;
        }
        c.cycles += 2;
        c.g[A1][0] = 0x50_0000;
        c.g[A0][0] = 0;
        loop {
            c.cycles += 14;
            c.g[V0][0] = c.lw(spch_ea(c.g[S1][0], 0x80));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
            c.ldl(6, spch_ea(c.g[T1][0], 0x7));
            c.ldr(6, spch_ea(c.g[T1][0], 0));
            c.g[A3][0] = c.lw(spch_ea(c.g[T1][0], 0x8));
            c.sdl(6, spch_ea(c.g[V0][0], 0x7));
            c.sdr(6, spch_ea(c.g[V0][0], 0));
            c.sw(spch_ea(c.g[V0][0], 0x8), c.g[A3][0]);
            c.g[V0][0] = c.lb(spch_ea(c.g[S2][0], 0x20));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003a_3628);
                }
                continue;
            }
            break 'b_3a3660;
        }
    }
    'b_3a36b0: {
        c.cycles += 4;
        c.g[S0][0] = c.lw(spch_ea(c.g[S1][0], 0x74));
        c.g[V0][0] = c.g[S0][0] & 0x1;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x60);
            break 'b_3a36b0;
        }
        'b_3a36a8: {
            c.cycles += 12;
            c.g[S0][0] = sx((c.g[S0][0] as u32) >> 17);
            c.g[A0][0] = 0x30;
            c.mult(0, 0, 16, 4);
            c.g[A1][0] = 0x49_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40f8));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lo[0];
            c.g[S1][0] = sx(spch_ea(c.g[S1][0], 0x8c));
            call_ptr(c, 0x0031_7e30, 0x003a_3698)?;
            let t = c.g[S0][0] == 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_3a36a8;
            }
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 2;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        frame.restore(c, S0, 0x60);
    }
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

pub(crate) const UI_OBJECT_DTOR: Callee = Callee {
    addr: 0x0039_fc48,
    f: ee::s13::f_0039fc48,
};

// ---------------------------------------------------------------------------
// UIIcon_dtor

/// `void UIIcon_dtor(UIIcon* self)` (0x3a4e80): set the icon's vtable word
/// (+0x8, 0x494aa8) and run `UIObject_dtor(self)`.
pub fn uiicon_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4aa8));
        let vtable = c.g[V0][0];
        let obj = c.g[V1][0] as u32;
        c.sw(obj.wrapping_add(0x8), vtable);
        UI_OBJECT_DTOR.call(c, 0x003a_4e9c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Where a static-init store takes its word from.
#[derive(Clone, Copy)]
pub(crate) enum Oc1079Src {
    /// The float literal at `gp - offset` (a `.lit4` pool entry).
    Lit(u32),
    /// A literal bit pattern (0.0 or a constant the compiler built in a register).
    Bits(u32),
}

/// The sColor/t2Vector words stored into the UI colour table
/// (0x5083d0..0x508750), in the generated body's store order.
pub(crate) const OC1079_STORES: &[(u32, Oc1079Src)] = &[
    (0x005083e0, Oc1079Src::Lit(0x24d8)),
    (0x005083f0, Oc1079Src::Lit(0x24d4)),
    (0x00508400, Oc1079Src::Lit(0x24d0)),
    (0x0050840c, Oc1079Src::Lit(0x24cc)),
    (0x00508410, Oc1079Src::Lit(0x24c8)),
    (0x00508420, Oc1079Src::Lit(0x24c4)),
    (0x00508430, Oc1079Src::Lit(0x24c0)),
    (0x0050843c, Oc1079Src::Lit(0x24bc)),
    (0x00508440, Oc1079Src::Lit(0x24b8)),
    (0x00508450, Oc1079Src::Lit(0x24b4)),
    (0x005083d0, Oc1079Src::Bits(0x3f800000)),
    (0x005083dc, Oc1079Src::Bits(0x3f000000)),
    (0x005083ec, Oc1079Src::Bits(0x0)),
    (0x005083fc, Oc1079Src::Bits(0x0)),
    (0x00508404, Oc1079Src::Lit(0x24b0)),
    (0x00508408, Oc1079Src::Lit(0x24ac)),
    (0x00508414, Oc1079Src::Lit(0x24a8)),
    (0x00508418, Oc1079Src::Lit(0x24a4)),
    (0x0050841c, Oc1079Src::Bits(0x0)),
    (0x0050842c, Oc1079Src::Bits(0x0)),
    (0x00508434, Oc1079Src::Lit(0x24a0)),
    (0x00508438, Oc1079Src::Lit(0x249c)),
    (0x00508444, Oc1079Src::Lit(0x2498)),
    (0x0050844c, Oc1079Src::Bits(0x3f800000)),
    (0x005083d4, Oc1079Src::Bits(0x3f800000)),
    (0x005083d8, Oc1079Src::Bits(0x3f800000)),
    (0x005083e4, Oc1079Src::Bits(0x3f800000)),
    (0x005083e8, Oc1079Src::Bits(0x3f800000)),
    (0x005083f4, Oc1079Src::Bits(0x3f800000)),
    (0x005083f8, Oc1079Src::Bits(0x3f800000)),
    (0x00508424, Oc1079Src::Bits(0x3f800000)),
    (0x00508428, Oc1079Src::Bits(0x3f800000)),
    (0x00508448, Oc1079Src::Bits(0x3f800000)),
    (0x00508454, Oc1079Src::Lit(0x2494)),
    (0x00508460, Oc1079Src::Lit(0x2490)),
    (0x0050846c, Oc1079Src::Lit(0x248c)),
    (0x0050849c, Oc1079Src::Lit(0x2488)),
    (0x00508458, Oc1079Src::Lit(0x2484)),
    (0x0050845c, Oc1079Src::Bits(0x0)),
    (0x00508464, Oc1079Src::Lit(0x2480)),
    (0x00508468, Oc1079Src::Lit(0x247c)),
    (0x00508470, Oc1079Src::Bits(0x3f800000)),
    (0x0050847c, Oc1079Src::Bits(0x3f800000)),
    (0x00508480, Oc1079Src::Bits(0x3f800000)),
    (0x0050848c, Oc1079Src::Bits(0x0)),
    (0x00508490, Oc1079Src::Bits(0x3f800000)),
    (0x00508498, Oc1079Src::Lit(0x2478)),
    (0x005084a0, Oc1079Src::Bits(0x3f800000)),
    (0x005084ac, Oc1079Src::Bits(0x3f800000)),
    (0x005084b0, Oc1079Src::Bits(0x3f800000)),
    (0x005084bc, Oc1079Src::Bits(0x3f800000)),
    (0x005084c0, Oc1079Src::Bits(0x3f800000)),
    (0x005084cc, Oc1079Src::Bits(0x0)),
    (0x005084d0, Oc1079Src::Bits(0x3f800000)),
    (0x00508474, Oc1079Src::Bits(0x3f800000)),
    (0x00508478, Oc1079Src::Bits(0x3f800000)),
    (0x00508484, Oc1079Src::Bits(0x0)),
    (0x00508488, Oc1079Src::Bits(0x0)),
    (0x00508494, Oc1079Src::Lit(0x2488)),
    (0x005084a4, Oc1079Src::Bits(0x3f800000)),
    (0x005084a8, Oc1079Src::Bits(0x3f800000)),
    (0x005084b4, Oc1079Src::Bits(0x3f800000)),
    (0x005084b8, Oc1079Src::Bits(0x3f800000)),
    (0x005084c4, Oc1079Src::Bits(0x0)),
    (0x005084c8, Oc1079Src::Bits(0x0)),
    (0x005084d4, Oc1079Src::Bits(0x0)),
    (0x005084d8, Oc1079Src::Bits(0x0)),
    (0x005084dc, Oc1079Src::Bits(0x0)),
    (0x0050851c, Oc1079Src::Bits(0x3f000000)),
    (0x0050852c, Oc1079Src::Lit(0x2474)),
    (0x00508540, Oc1079Src::Lit(0x2470)),
    (0x005084e0, Oc1079Src::Bits(0x3f800000)),
    (0x005084ec, Oc1079Src::Bits(0x3f800000)),
    (0x005084f0, Oc1079Src::Bits(0x3f800000)),
    (0x005084fc, Oc1079Src::Bits(0x3f000000)),
    (0x00508500, Oc1079Src::Bits(0x3f800000)),
    (0x0050850c, Oc1079Src::Bits(0x3f000000)),
    (0x00508510, Oc1079Src::Bits(0x3f800000)),
    (0x00508520, Oc1079Src::Bits(0x3f800000)),
    (0x00508524, Oc1079Src::Lit(0x246c)),
    (0x00508528, Oc1079Src::Lit(0x2468)),
    (0x00508530, Oc1079Src::Bits(0x3f800000)),
    (0x00508538, Oc1079Src::Lit(0x2464)),
    (0x0050853c, Oc1079Src::Bits(0x0)),
    (0x00508544, Oc1079Src::Lit(0x2460)),
    (0x00508548, Oc1079Src::Lit(0x245c)),
    (0x0050854c, Oc1079Src::Bits(0x0)),
    (0x00508550, Oc1079Src::Bits(0x3f800000)),
    (0x0050855c, Oc1079Src::Bits(0x3f800000)),
    (0x00508560, Oc1079Src::Bits(0x3f800000)),
    (0x005084e4, Oc1079Src::Bits(0x3f800000)),
    (0x005084e8, Oc1079Src::Bits(0x3f800000)),
    (0x005084f4, Oc1079Src::Bits(0x3f000000)),
    (0x005084f8, Oc1079Src::Bits(0x3f800000)),
    (0x00508504, Oc1079Src::Bits(0x3f000000)),
    (0x00508508, Oc1079Src::Bits(0x3f800000)),
    (0x00508514, Oc1079Src::Bits(0x3f000000)),
    (0x00508518, Oc1079Src::Bits(0x3f800000)),
    (0x00508534, Oc1079Src::Bits(0x3f800000)),
    (0x00508554, Oc1079Src::Bits(0x3f800000)),
    (0x00508558, Oc1079Src::Bits(0x3f800000)),
    (0x00508564, Oc1079Src::Bits(0x3f800000)),
    (0x0050858c, Oc1079Src::Lit(0x2458)),
    (0x005085ac, Oc1079Src::Lit(0x2454)),
    (0x00508568, Oc1079Src::Lit(0x2450)),
    (0x0050856c, Oc1079Src::Bits(0x0)),
    (0x00508570, Oc1079Src::Bits(0x3f800000)),
    (0x00508574, Oc1079Src::Lit(0x244c)),
    (0x00508578, Oc1079Src::Lit(0x2448)),
    (0x0050857c, Oc1079Src::Bits(0x0)),
    (0x00508580, Oc1079Src::Bits(0x3f800000)),
    (0x00508584, Oc1079Src::Lit(0x2444)),
    (0x00508588, Oc1079Src::Lit(0x2440)),
    (0x00508590, Oc1079Src::Bits(0x3f800000)),
    (0x00508598, Oc1079Src::Lit(0x243c)),
    (0x0050859c, Oc1079Src::Bits(0x3f800000)),
    (0x005085a0, Oc1079Src::Bits(0x3f800000)),
    (0x005085a4, Oc1079Src::Lit(0x2438)),
    (0x005085b0, Oc1079Src::Bits(0x3f800000)),
    (0x005085bc, Oc1079Src::Bits(0x3f800000)),
    (0x005085c0, Oc1079Src::Bits(0x3f800000)),
    (0x005085cc, Oc1079Src::Bits(0x3f800000)),
    (0x005085d0, Oc1079Src::Bits(0x3f800000)),
    (0x005085d4, Oc1079Src::Lit(0x2434)),
    (0x005085d8, Oc1079Src::Lit(0x2430)),
    (0x005085dc, Oc1079Src::Bits(0x0)),
    (0x005085e0, Oc1079Src::Bits(0x3f800000)),
    (0x00508594, Oc1079Src::Lit(0x242c)),
    (0x005085a8, Oc1079Src::Bits(0x3f800000)),
    (0x005085b4, Oc1079Src::Bits(0x3f800000)),
    (0x005085b8, Oc1079Src::Bits(0x3f800000)),
    (0x005085c4, Oc1079Src::Bits(0x3f800000)),
    (0x005085c8, Oc1079Src::Bits(0x3f800000)),
    (0x005085e4, Oc1079Src::Bits(0x3f800000)),
    (0x005085e8, Oc1079Src::Bits(0x3f800000)),
    (0x005085ec, Oc1079Src::Bits(0x3f800000)),
    (0x00508620, Oc1079Src::Lit(0x2428)),
    (0x0050863c, Oc1079Src::Lit(0x2424)),
    (0x0050864c, Oc1079Src::Lit(0x2420)),
    (0x0050866c, Oc1079Src::Lit(0x241c)),
    (0x005085f0, Oc1079Src::Bits(0x3f800000)),
    (0x005085f4, Oc1079Src::Lit(0x2418)),
    (0x005085fc, Oc1079Src::Bits(0x0)),
    (0x00508600, Oc1079Src::Bits(0x3f800000)),
    (0x0050860c, Oc1079Src::Bits(0x3f800000)),
    (0x00508610, Oc1079Src::Bits(0x3f800000)),
    (0x0050861c, Oc1079Src::Bits(0x0)),
    (0x0050862c, Oc1079Src::Bits(0x0)),
    (0x00508630, Oc1079Src::Bits(0x3f800000)),
    (0x00508634, Oc1079Src::Lit(0x2414)),
    (0x00508638, Oc1079Src::Lit(0x2410)),
    (0x00508640, Oc1079Src::Bits(0x3f800000)),
    (0x00508644, Oc1079Src::Lit(0x240c)),
    (0x00508650, Oc1079Src::Bits(0x3f800000)),
    (0x00508658, Oc1079Src::Lit(0x2408)),
    (0x0050865c, Oc1079Src::Bits(0x0)),
    (0x00508660, Oc1079Src::Bits(0x3f800000)),
    (0x00508664, Oc1079Src::Lit(0x2404)),
    (0x00508670, Oc1079Src::Bits(0x3f800000)),
    (0x005085f8, Oc1079Src::Bits(0x3f800000)),
    (0x00508604, Oc1079Src::Bits(0x3f800000)),
    (0x00508608, Oc1079Src::Bits(0x3f800000)),
    (0x00508614, Oc1079Src::Bits(0x3f800000)),
    (0x00508618, Oc1079Src::Bits(0x0)),
    (0x00508624, Oc1079Src::Bits(0x0)),
    (0x00508628, Oc1079Src::Bits(0x0)),
    (0x00508648, Oc1079Src::Bits(0x3f800000)),
    (0x00508654, Oc1079Src::Bits(0x3f800000)),
    (0x00508668, Oc1079Src::Bits(0x3f800000)),
    (0x00508674, Oc1079Src::Bits(0x3f800000)),
    (0x0050868c, Oc1079Src::Lit(0x242c)),
    (0x005086cc, Oc1079Src::Lit(0x2400)),
    (0x005086dc, Oc1079Src::Lit(0x23fc)),
    (0x0050867c, Oc1079Src::Bits(0x3f800000)),
    (0x00508680, Oc1079Src::Bits(0x3f800000)),
    (0x00508690, Oc1079Src::Bits(0x3f800000)),
    (0x0050869c, Oc1079Src::Bits(0x0)),
    (0x005086a0, Oc1079Src::Bits(0x3f800000)),
    (0x005086ac, Oc1079Src::Bits(0x0)),
    (0x005086b0, Oc1079Src::Bits(0x3f800000)),
    (0x005086bc, Oc1079Src::Bits(0x0)),
    (0x005086c0, Oc1079Src::Bits(0x3f800000)),
    (0x005086c4, Oc1079Src::Lit(0x23f8)),
    (0x005086d0, Oc1079Src::Bits(0x3f800000)),
    (0x005086d8, Oc1079Src::Lit(0x23f4)),
    (0x005086e0, Oc1079Src::Bits(0x3f800000)),
    (0x005086e8, Oc1079Src::Lit(0x23f0)),
    (0x005086ec, Oc1079Src::Bits(0x0)),
    (0x005086f0, Oc1079Src::Bits(0x3f800000)),
    (0x005086f4, Oc1079Src::Lit(0x23ec)),
    (0x005086f8, Oc1079Src::Lit(0x23e8)),
    (0x00508678, Oc1079Src::Bits(0x3f800000)),
    (0x00508688, Oc1079Src::Bits(0x3f800000)),
    (0x00508684, Oc1079Src::Lit(0x242c)),
    (0x00508694, Oc1079Src::Bits(0x3f800000)),
    (0x00508698, Oc1079Src::Bits(0x0)),
    (0x005086a4, Oc1079Src::Bits(0x3f800000)),
    (0x005086a8, Oc1079Src::Bits(0x0)),
    (0x005086b4, Oc1079Src::Bits(0x3f800000)),
    (0x005086b8, Oc1079Src::Bits(0x0)),
    (0x005086c8, Oc1079Src::Bits(0x3f800000)),
    (0x005086d4, Oc1079Src::Bits(0x3f800000)),
    (0x005086e4, Oc1079Src::Bits(0x3f800000)),
    (0x005086fc, Oc1079Src::Bits(0x0)),
    (0x0050871c, Oc1079Src::Lit(0x23e4)),
    (0x0050872c, Oc1079Src::Lit(0x23e0)),
    (0x00508700, Oc1079Src::Bits(0x3f800000)),
    (0x00508704, Oc1079Src::Lit(0x23dc)),
    (0x00508708, Oc1079Src::Lit(0x23d8)),
    (0x0050870c, Oc1079Src::Bits(0x0)),
    (0x00508710, Oc1079Src::Bits(0x3f800000)),
    (0x00508714, Oc1079Src::Lit(0x23d4)),
    (0x00508718, Oc1079Src::Lit(0x23d0)),
    (0x00508720, Oc1079Src::Bits(0x3f800000)),
    (0x00508724, Oc1079Src::Lit(0x23cc)),
    (0x00508728, Oc1079Src::Lit(0x23c8)),
    (0x00508730, Oc1079Src::Bits(0x3f800000)),
    (0x0050873c, Oc1079Src::Bits(0x0)),
    (0x00508740, Oc1079Src::Bits(0x3f800000)),
    (0x00508744, Oc1079Src::Bits(0x3f800000)),
    (0x0050874c, Oc1079Src::Bits(0x0)),
    (0x00508738, Oc1079Src::Bits(0x3f800000)),
    (0x00508734, Oc1079Src::Bits(0x0)),
    (0x00508748, Oc1079Src::Bits(0x0)),
];

/// `void UIEngine_StaticInitDestroy(int initialize, int priority)`
/// (0x3a3f48): the UI engine's static constructor/destructor. With priority
/// 0xffff and `initialize != 0` it fills the UI colour table
/// (`D_005083D0..D_00508740`, sColor RGBA floats; the pool literals sit at
/// `gp - 0x24d8..`), runs `UISound_BaseCtor(&gUISoundNull)` (`gp + 0x2968`)
/// and sets the `t2Vector` at `gp + 0x2970` to (2.0, 2.0). With
/// `initialize == 0` it only stores the base sound vtable (0x46dd60) into
/// `gUISoundNull`. Any other priority does nothing. Scratch registers and
/// `f0..f19` end as the compiled body leaves them.
pub fn oc1079_ui_engine_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0xffff;
        for (reg, off) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.save(c, reg, off);
        }
        let priority_ok = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if priority_ok {
            c.cycles += 2;
            let initialize = c.g[A0][0] != 0;
            c.g[T5][0] = 0x0051_0000;
            let gp = c.g[GP][0] as u32;
            if initialize {
                c.cycles += 493;
                for &(addr, s) in OC1079_STORES {
                    let word = match s {
                        Oc1079Src::Lit(off) => c.bus.read32(gp.wrapping_sub(off)),
                        Oc1079Src::Bits(b) => b,
                    };
                    c.bus.write32(addr, word);
                }
                c.g[T5][0] = 0x005086e0;
                c.g[T6][0] = 0x005086d0;
                c.g[T7][0] = 0x00508700;
                c.g[V0][0] = 0x00510000;
                c.g[S1][0] = 0x00508710;
                c.g[S2][0] = 0x00510000;
                c.g[S3][0] = 0x00508740;
                c.g[S5][0] = 0x005084e0;
                c.g[S4][0] = 0x00508530;
                c.g[T3][0] = 0x00508550;
                c.g[A1][0] = 0x00508570;
                c.g[A3][0] = 0x005085d0;
                c.g[T1][0] = 0x005085a0;
                c.g[V1][0] = 0x00510000;
                c.g[T0][0] = 0x00508630;
                c.g[A2][0] = 0x00508580;
                c.g[T4][0] = 0x00508650;
                c.g[T9][0] = 0x00510000;
                c.g[T2][0] = 0x00508660;
                c.g[S6][0] = 0x00510000;
                c.g[FP][0] = 0x00510000;
                c.g[T8][0] = 0x00510000;
                c.g[S7][0] = 0x00510000;
                c.g[S0][0] = 0x00508730;
                c.g[A0][0] = sx(gp.wrapping_add(0x2968));
                c.f[0] = 0x3f800000;
                c.lwc1(1, gp.wrapping_sub(0x23e0));
                c.lwc1(2, gp.wrapping_sub(0x23e4));
                c.lwc1(3, gp.wrapping_sub(0x23dc));
                c.lwc1(4, gp.wrapping_sub(0x23d8));
                c.lwc1(5, gp.wrapping_sub(0x23d4));
                c.lwc1(6, gp.wrapping_sub(0x23d0));
                c.lwc1(7, gp.wrapping_sub(0x23cc));
                c.lwc1(8, gp.wrapping_sub(0x23c8));
                c.lwc1(9, gp.wrapping_sub(0x2408));
                c.lwc1(10, gp.wrapping_sub(0x2404));
                c.lwc1(11, gp.wrapping_sub(0x2430));
                c.lwc1(12, gp.wrapping_sub(0x242c));
                c.lwc1(13, gp.wrapping_sub(0x24a8));
                c.lwc1(14, gp.wrapping_sub(0x24a4));
                c.lwc1(15, gp.wrapping_sub(0x24a0));
                c.lwc1(16, gp.wrapping_sub(0x249c));
                c.lwc1(17, gp.wrapping_sub(0x24b0));
                c.lwc1(18, gp.wrapping_sub(0x24ac));
                c.lwc1(19, gp.wrapping_sub(0x2494));
                Callee {
                    addr: 0x0039_e288,
                    f: ee::s13::f_0039e288,
                }
                .call(c, 0x003a_4724)?;
                c.g[AT][0] = 0x4000_0000;
                c.f[0] = 0x4000_0000;
                c.swc1(0, gp.wrapping_add(0x2974));
                c.swc1(0, gp.wrapping_add(0x2970));
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x0047_0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22a0));
                let v = c.g[V0][0];
                c.sw(gp.wrapping_add(0x2968), v);
            }
        }
        c.cycles += 12;
        for (reg, off) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIVector* UIVector_ReturnSelf(UIVector* self, ...)` (0x3a3f08): spills
/// the argument registers and the f12..f18 arguments to the caller's frame
/// and returns `self` in v0.
pub fn ui_vector_return_self(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    for (reg, off) in [
        (A1, 0x48),
        (A2, 0x50),
        (A3, 0x58),
        (T0, 0x60),
        (T1, 0x68),
        (T2, 0x70),
        (T3, 0x78),
    ] {
        let slot = frame.at(c, off);
        let value = c.g[reg][0];
        c.sd(slot, value);
    }
    for (fpr, off) in [(12, 0x38), (14, 0x3c), (16, 0x40), (18, 0x44)] {
        let slot = frame.at(c, off);
        c.swc1(fpr, slot);
    }
    frame.close(c);
}

/// `void UIObject_ctor_2(UIObject* self)` (0x3a4900): stores the object's
/// vtable word and two self pointers, then runs the flags constructor.
/// Returns `self` in v0.
pub fn ui_object_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x4bd8));
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        ui_text_field(this, 4).sw(c, this);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x14));
        ui_text_field(this, 0).sw(c, this);
        let vtable = c.g[V0][0];
        ui_text_field(this, 8).sw(c, vtable);
        let flags_ctor = Callee {
            addr: 0x003a_6548,
            f: ee::s13::f_003a6548,
        };
        flags_ctor.call(c, 0x003a_492c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject_ctor` (0x39fb30): the base constructor of every UI object.
pub(crate) const UIOBJ_BASE_CTOR: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::f_0039fb30,
};

/// `cList_cList` (0x3977e8): the group's child list, at object +0x74.
pub(crate) const UILIST_BASE_CTOR: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::f_003977e8,
};

/// `UIList_First` (0x397840) on the group's child list.
pub(crate) const UILIST_FIRST_CALL: Callee = Callee {
    addr: 0x0039_7840,
    f: ee::s13::f_00397840,
};

/// `UIList_Last` (0x397858) on the group's child list.
pub(crate) const UILIST_LAST_CALL: Callee = Callee {
    addr: 0x0039_7858,
    f: ee::s13::f_00397858,
};

/// `UISound_BaseDtor(self, flags)` (0x3a4da8): publish the sound base
/// vtable (`0x46dd60`) in word 0 and free the object when bit 0 of `flags`
/// is set.
pub fn ui_sound_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        reset_vtable_and_delete(c, 0x003a_4dcc, 0x0046_dd60, 0)
    });
}

/// `UIIcon* UIIcon_ctor_2(UIIcon* self)` (0x3a4df0): `UIObject_ctor(self, 0,
/// 0)`, then the icon vtable (`0x494aa8`) at +8 and the `0x21ccc8` setup of
/// the sub-object at +0x74. Returns `self`.
pub fn uiicon_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_4e0c)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4aa8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x0021_ccc8, 0x003a_4e20)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIIcon* UIIcon_ctor(UIIcon* self)` (0x3a4e38): as [`uiicon_ctor_2`]
/// (the caller's a1/a2 reach `UIObject_ctor` unchanged), then the
/// `-1`/`0` pair at +0x78/+0x7c. Returns `self`.
pub fn uiicon_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_4e4c)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4aa8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x0021_ccc8, 0x003a_4e60)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = c.g[S0][0];
        let minus_one = c.g[V1][0];
        Addr(this.wrapping_add(0x78)).sw(c, minus_one);
        Addr(this.wrapping_add(0x7c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIGroup_SetOffset(cUIGroup* self, float x, float y)` (0x3a4fb8):
/// the two float arguments (f12, f13) go through the stack and are copied
/// as one unaligned 8-byte word to the group's offset at +0x80.
pub fn c_ui_group_set_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_fpr(c, 12, 0);
        frame.save_fpr(c, 13, 4);
        let slot = sp(c);
        c.ldl(2, slot.wrapping_add(7));
        c.ldr(2, slot);
        let group = c.g[A0][0] as u32;
        c.sdl(2, group.wrapping_add(0x87));
        c.sdr(2, group.wrapping_add(0x80));
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup* cUIGroup_ctor_2(cUIGroup* self)` (0x3a4fe0): `UIObject_ctor`
/// with a1 = a2 = 0, then the group vtable (`0x4949e8`) at +8 and
/// `cList_cList` on the child list at +0x74. Returns `self`.
pub fn c_ui_group_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_4ffc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        UILIST_BASE_CTOR.call(c, 0x003a_5010)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup* cUIGroup_ctor(cUIGroup* self)` (0x3a5028): as
/// [`c_ui_group_ctor_2`] with the caller's a1/a2 passed on to
/// `UIObject_ctor`. Returns `self`.
pub fn c_ui_group_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UIOBJ_BASE_CTOR.call(c, 0x003a_503c)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        UILIST_BASE_CTOR.call(c, 0x003a_5050)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject* cUIGroup_FirstChild(cUIGroup* self)` (0x3a5100):
/// `UIList_First(&self->children)`, the child list at +0x74.
pub fn c_ui_group_first_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        UILIST_FIRST_CALL.call(c, 0x003a_5110)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject* cUIGroup_LastChild(cUIGroup* self)` (0x3a5120):
/// `UIList_Last(&self->children)`, the child list at +0x74.
pub fn c_ui_group_last_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        UILIST_LAST_CALL.call(c, 0x003a_5130)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup_getByIndex(group)` (0x3a5140): `UIList_GetAt(&group->children,
/// index)`. The group's child list sits at +0x74; the index is in a1.
pub fn ui_group_get_child_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        Callee {
            addr: 0x0039_7870,
            f: ee::s13::UIList_GetAt,
        }
        .call(c, 0x003a_5150)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup_addToFront(group, item)` (0x3a5160): `UIList_PushFront` on the
/// group's child list (+0x74). The item is in a1.
pub fn ui_group_push_front_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        Callee {
            addr: 0x0039_78f0,
            f: ee::s13::UIList_PushFront,
        }
        .call(c, 0x003a_5170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIGroup_addToEnd(group, item)` (0x3a5180): `UIList_PushBack` on the
/// group's child list (+0x74). The item is in a1.
pub fn ui_group_push_back_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        Callee {
            addr: 0x0039_7910,
            f: ee::s13::UIList_PushBack,
        }
        .call(c, 0x003a_5190)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIMenu::UIMenu(this)` (0x3a5230): the object base constructor, the menu's
/// child list at +0x74 (vtable word +0x8 set to the list's), then the menu
/// vtable at +0x8 again, `UIMenuFlags` constructor on the menu, and the
/// menu's own fields (+0x95..+0xa0) cleared.
pub fn ui_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0039_fb30,
            f: ee::s13::UIObject_ctor,
        }
        .call(c, 0x003a_5244)?;
        let menu = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(menu.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        let vt = c.g[V1][0];
        Addr(menu.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x0039_77e8,
            f: ee::s13::UIList_ctor,
        }
        .call(c, 0x003a_5258)?;
        let menu = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(menu.wrapping_add(0x90));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4928));
        let vt = c.g[V1][0];
        Addr(menu.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x003a_5338,
            f: ee::s13::UIMenuFlags_ctor,
        }
        .call(c, 0x003a_526c)?;
        let menu = c.g[S0][0] as u32;
        c.g[V1][0] = 0xff;
        c.g[V0][0] = c.g[S0][0];
        let flag = c.g[V1][0];
        Addr(menu.wrapping_add(0xa4)).sb(c, flag);
        Addr(menu.wrapping_add(0x95)).sb(c, 0);
        Addr(menu.wrapping_add(0x96)).sb(c, 0);
        Addr(menu.wrapping_add(0x97)).sb(c, 0);
        Addr(menu.wrapping_add(0x98)).sb(c, 0);
        Addr(menu.wrapping_add(0x9c)).sw(c, 0);
        Addr(menu.wrapping_add(0xa0)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIPair::UIPair(this)` without flags (0x3a5420, the first constructor
/// variant): the object base constructor, then the pair flags constructor
/// with a1 = a2 = 0 and the pair vtable at +0x8. Returns `this` in v0.
pub fn ui_pair_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0039_fb30,
            f: ee::s13::UIObject_ctor,
        }
        .call(c, 0x003a_543c)?;
        let pair = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(pair.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4868));
        let vt = c.g[V1][0];
        Addr(pair.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x003a_5508,
            f: ee::s13::UIPairFlags_ctor,
        }
        .call(c, 0x003a_5450)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIPair::UIPair(this)` (0x3a5468): the object base constructor, the pair
/// vtable at +0x8 and the pair flags constructor, then the first and second
/// widget slots (+0x78, +0x7c) cleared and bit 1 set in the flags word +0x14.
pub fn ui_pair_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0039_fb30,
            f: ee::s13::UIObject_ctor,
        }
        .call(c, 0x003a_547c)?;
        let pair = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(pair.wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4868));
        let vt = c.g[V1][0];
        Addr(pair.wrapping_add(0x8)).sw(c, vt);
        Callee {
            addr: 0x003a_5508,
            f: ee::s13::UIPairFlags_ctor,
        }
        .call(c, 0x003a_5490)?;
        let pair = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(pair.wrapping_add(0x14)).lw(c);
        c.g[V0][0] = c.g[S0][0];
        Addr(pair.wrapping_add(0x78)).sw(c, 0);
        c.g[V1][0] = c.g[V1][0] | 0x2;
        Addr(pair.wrapping_add(0x7c)).sw(c, 0);
        let flags = c.g[V1][0];
        Addr(pair.wrapping_add(0x14)).sw(c, flags);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `UIPair_SetFirst` (0x3a5530, `field` 0x78) and
/// `UIPair_SetSecond` (0x3a5588, `field` 0x7c): if the old widget in the
/// slot is set, call its vtable slot 3 with (old + delta, 3) returning to
/// `call_ret`; then store the new widget (a1) in the slot. The two differ only
/// in the slot offset and the return address of that call.
pub(crate) fn ui_pair_set_entry(c: &mut Cpu, field: u32, call_ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(field).lw(c);
    c.g[S1][0] = c.g[A1][0];
    if c.g[A2][0] != 0 {
        c.cycles += 6;
        let old = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(old).off(0x8).lw(c);
        c.g[A1][0] = 0x3;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(old.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, call_ret)?;
    }
    c.cycles += 6;
    let value = c.g[S1][0];
    Addr(c.g[S0][0] as u32).off(field).sw(c, value);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `UIPair::setEntry`: store the first widget (+0x78) (0x3a5530).
pub fn ui_pair_set_first(c: &mut Cpu) {
    guest(c, |c| ui_pair_set_entry(c, 0x78, 0x003a_5568))
}

/// `UIPair::setWidget`: store the second widget (+0x7c) (0x3a5588).
pub fn ui_pair_set_second(c: &mut Cpu) {
    guest(c, |c| ui_pair_set_entry(c, 0x7c, 0x003a_55c0))
}

/// Callees of the UI constructors, as the generated bodies call them.
pub(crate) const CT_OBJECT: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::UIObject_ctor,
};

pub(crate) const CT_LIST: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::UIList_ctor,
};

pub(crate) const CT_LIST_PUSH_BACK: Callee = Callee {
    addr: 0x0039_7910,
    f: ee::s13::UIList_PushBack,
};

pub(crate) const CT_LISTBOX_FLAGS: Callee = Callee {
    addr: 0x003a_65a0,
    f: ee::s13::UIListBoxFlags_ctor,
};

pub(crate) const CT_LISTBOX_SCROLL: Callee = Callee {
    addr: 0x003a_65b8,
    f: ee::s13::UIListBoxScroll_ctor,
};

pub(crate) const CT_STATE_FLAGS: Callee = Callee {
    addr: 0x003a_65d8,
    f: ee::s13::UIStateFlags_ctor,
};

pub(crate) const CT_TEXT_FLAGS: Callee = Callee {
    addr: 0x003a_65e8,
    f: ee::s13::UITextFlags_ctor,
};

/// `UIScreenFlags_ctor`, called through the function table (`jalr`).
pub(crate) const CT_SCREEN_FLAGS: u32 = 0x001d_d580;

/// Vtables the constructors store at +8 (or +0x48 for the list links), as
/// `0x490000 + the immediate` in the generated bodies.
#[allow(dead_code)]
pub(crate) const CT_VT_LISTBOX: u32 = 0x0049_4798;

#[allow(dead_code)]
pub(crate) const CT_VT_STATE: u32 = 0x0049_46c8;

#[allow(dead_code)]
pub(crate) const CT_VT_SCREEN: u32 = 0x0049_4670;

#[allow(dead_code)]
pub(crate) const CT_VT_TEXT: u32 = 0x0049_4588;

#[allow(dead_code)]
pub(crate) const CT_VT_SCROLL: u32 = 0x0049_44c8;

/// The vtable a screen's child lists point at (`+0x48`, `+0xb4` list).
#[allow(dead_code)]
pub(crate) const CT_VT_SCREEN_LIST: u32 = 0x0049_49e8;

/// `cUIListBox_cUIListBox1(self)` (0x3a55d8, `UIListBox_ctor_2`): the list box
/// constructor. Calls `UIObject_ctor(self, 0, 0)`, sets the vtable at +8,
/// builds the list box flags at +0x74, then initialises the 32 scroll records
/// of 0x14 bytes from +0x98 (the loop counts `s1` from 0x1f down to -1, one
/// `UIListBoxScroll_ctor` per record). The loop's back edge carries an event
/// check. Returns with `s2` (the object) still in place.
pub fn ui_list_box_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[A2][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x98));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x1f;
        CT_OBJECT.call(c, 0x003a_5608)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4798));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        CT_LISTBOX_FLAGS.call(c, 0x003a_5620)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            CT_LISTBOX_SCROLL.call(c, 0x003a_562c)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_5620);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cUIState_cUIState2(self)` (0x3a57b8, `UIState_ctor_3`): the UI state
/// constructor. Both list links (+0 and +4) point at the state itself, the
/// vtable (0x4946c8) is stored at +8, the state flags are built at +0x1c and
/// the child list at +0x24. Returns `self`.
pub fn ui_state_ctor_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x46c8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.sw(c.g[S0][0] as u32, c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        CT_STATE_FLAGS.call(c, 0x003a_57e4)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
        CT_LIST.call(c, 0x003a_57ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScreen_cUIScreen1(self)` (0x3a5b98, `UIScreen_ctor`): the screen
/// constructor, the variant with the screen flags built through the function
/// table. Both list links point at the screen, the vtable (0x494670) is at
/// +8, the screen flags at +0x14, the root child list at +0x18, the root
/// object at +0x40 (`UIObject_ctor(root, 0, 0)`), and the child list at +0xb4
/// with the 0x4949e8 vtable at +0x48. Returns `self`.
pub fn ui_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sw(c.g[S0][0] as u32, c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, CT_SCREEN_FLAGS, 0x003a_5bc4)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        CT_LIST.call(c, 0x003a_5bcc)?;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        CT_OBJECT.call(c, 0x003a_5bdc)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        CT_LIST.call(c, 0x003a_5bf0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScreen_cUIScreen(self, state)` (0x3a5c08, `UIScreen_ctor_2`): the
/// screen constructor that takes the owning UI state (`a1`, stored at +0xd0).
/// Same links and vtable as `UIScreen_ctor`, but the child list (+0x18) and
/// the root object (+0x40) are followed by zeroed fields: +0xc and +0x10
/// (before the flags call), +0x34, +0x38 and +0x3c (after the child list).
/// Returns `self`.
pub fn ui_screen_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sw(c.g[S0][0] as u32, c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, CT_SCREEN_FLAGS, 0x003a_5c44)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        CT_LIST.call(c, 0x003a_5c4c)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34), 0);
        c.g[A1][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0);
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        CT_OBJECT.call(c, 0x003a_5c68)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        CT_LIST.call(c, 0x003a_5c7c)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xd0), c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScreen_rootGroupPushBack(screen, obj)` (0x3a5cf0): appends `obj` to the
/// screen's child list at +0xb4 (`UIList_PushBack(&list, obj)`, the object in
/// `a1`, which the caller sets). Returns what `UIList_PushBack` returns.
pub fn ui_screen_add_root_child(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
        CT_LIST_PUSH_BACK.call(c, 0x003a_5d00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cUIText_cUIText1(self)` (0x3a5d60, `UIText_ctor_2`): the text object
/// constructor. Calls `UIObject_ctor(self, 0, 0)`, sets the vtable (0x494588)
/// at +8 and builds the text flags at +0x74. Returns `self`.
pub fn ui_text_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CT_OBJECT.call(c, 0x003a_5d7c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4588));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        CT_TEXT_FLAGS.call(c, 0x003a_5d90)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScrollBar_cUIScrollBar1(self)` (0x3a5f60, `cUIScroll_ctor_2`): the scroll
/// bar constructor that only sets the scroll vtable (0x4944c8) at +8 after
/// `UIObject_ctor(self, 0, 0)`. Returns `self`.
pub fn cui_scroll_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CT_OBJECT.call(c, 0x003a_5f7c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x44c8));
        frame.restore_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cUIScrollBar_cUIScrollBar(self)` (0x3a5fa0, `cUIScroll_ctor`): the scroll bar
/// constructor with its own fields. After `UIObject_ctor(self, a1, a2)` (the
/// caller's `a1` and `a2` pass through unchanged, as the generated body leaves
/// them), it sets the scroll vtable (0x4944c8) at +8 and clears the knob and
/// arrow pointers, the value and page words, the animation names and the hide
/// flags. Returns `self` in v0.
pub fn cui_scroll_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CT_OBJECT.call(c, 0x003a_5fb4)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x44c8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x84), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x88), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x8c), 0);
        c.sb((c.g[S0][0] as u32).wrapping_add(0xa5), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cUIThing_cUIThing` (0x39fb30): the base constructor the widget
/// constructors below run first.
pub(crate) const UI_THING_CTOR_OC1088: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::f_0039fb30,
};

/// `UIList_ctor` (0x3977e8): the scroll's name list.
pub(crate) const UI_LIST_CTOR_OC1088: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::f_003977e8,
};

/// `cUITextScrollFlags_ctor` (0x3a6600): one flag byte at its argument.
pub(crate) const TEXT_FLAGS_CTOR_OC1088: Callee = Callee {
    addr: 0x003a_6600,
    f: ee::s13::f_003a6600,
};

/// `UIList_First` (0x397840): the first node of a list whose head is at +4.
pub(crate) const UI_LIST_FIRST_OC1088: Callee = Callee {
    addr: 0x0039_7840,
    f: ee::s13::f_00397840,
};

/// Shared body of the two thread-name setters (`cUIScrollBar_setUpAnimationName`
/// and `setDownAnimationName` in the reference): when `strlen(name) < 25` the
/// name is copied with `strcpy` into `self+name_off`, otherwise the field's
/// first byte is cleared. The setters differ only in the field offset and in
/// their call return addresses.
pub(crate) fn set_scroll_thread_name(
    c: &mut Cpu,
    name_off: u32,
    strlen_ret: u32,
    strcpy_ret: u32,
) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0041_6810, strlen_ret)?;
    c.g[V0][0] = (c.g[V0][0] < 0x19) as u64;
    let short_name = c.g[V0][0] != 0;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(name_off));
    c.cycles += 2;
    if short_name {
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0041_66f4, strcpy_ret)?;
    } else {
        let field = (c.g[S0][0] as u32).wrapping_add(name_off);
        Addr(field).sb(c, 0);
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cUIScroll_SetUpThreadName(cUIScroll* self, const char* name)`
/// (0x3a6038): copy the name into `self+0x8c`, or clear it when it is 25
/// bytes or longer.
pub fn ui_scroll_set_up_thread_name(c: &mut Cpu) {
    guest(c, |c| {
        set_scroll_thread_name(c, 0x8c, 0x003a_6058, 0x003a_6074)
    });
}

/// `void cUIScroll_SetDownThreadName(cUIScroll* self, const char* name)`
/// (0x3a6088): the same as the up-name setter, for the field at `self+0xa5`.
pub fn ui_scroll_set_down_thread_name(c: &mut Cpu) {
    guest(c, |c| {
        set_scroll_thread_name(c, 0xa5, 0x003a_60a8, 0x003a_60c4)
    });
}

/// `cUIProgress* cUIProgress_ctor_2(cUIProgress* self)` (0x3a60e0): the UI
/// thing constructor with zero flags, then the progress vtable at `+8`.
pub fn ui_progress_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_60fc)?;
        c.g[V1][0] = sx(0x0049_4408);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIProgress* cUIProgress_ctor(cUIProgress* self)` (0x3a6120): the UI
/// thing constructor, the progress vtable at `+8`, and the four knob and
/// background fields (`+0x74..+0x84`) cleared.
pub fn ui_progress_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_6134)?;
        c.g[V1][0] = sx(0x0049_4408);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x74)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        Addr((c.g[S0][0] as u32).wrapping_add(0x78)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x80)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x84)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUISlider* cUISlider_ctor_2(cUISlider* self)` (0x3a6190): the UI thing
/// constructor with zero flags, then the slider vtable at `+8`.
pub fn ui_slider_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_61ac)?;
        c.g[V1][0] = sx(0x0049_4348);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUISlider* cUISlider_ctor(cUISlider* self)` (0x3a61d0): the UI thing
/// constructor, the slider vtable at `+8`, and the slider's value and range
/// fields (`+0x74..+0x8c`) cleared.
pub fn ui_slider_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_61e4)?;
        c.g[V1][0] = sx(0x0049_4348);
        c.g[V0][0] = c.g[S0][0];
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x74)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        Addr((c.g[S0][0] as u32).wrapping_add(0x78)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x80)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x88)).sw(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x8c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUITextScroll* cUITextScroll_ctor_2(cUITextScroll* self)` (0x3a6250):
/// the UI thing constructor, the text-scroll vtable at `+8`, the flag byte
/// at `+0x74` (`cUITextScrollFlags_ctor`), and the name list at `+0x8c`.
pub fn ui_text_scroll_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        UI_THING_CTOR_OC1088.call(c, 0x003a_626c)?;
        c.g[V1][0] = sx(0x0049_4278);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        TEXT_FLAGS_CTOR_OC1088.call(c, 0x003a_6280)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
        UI_LIST_CTOR_OC1088.call(c, 0x003a_6288)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cUITextScroll_ListFirst(cUITextScroll* self)` (0x3a63e0): the
/// first node of the name list at `self+0x8c`.
pub fn ui_text_scroll_list_first(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8c));
        UI_LIST_FIRST_OC1088.call(c, 0x003a_63f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `cList` member of a `cUITextScroll`, at +0x8c.
pub(crate) const TEXT_SCROLL_LIST_OFFSET: u32 = 0x8c;

/// `cUITextScroll_ListLast/GetAt/PushFront/PushBack(textScroll, ...)`: the
/// generated forwarder. A 0x10 frame holds `ra`, `a0` becomes
/// `textScroll + 0x8c` (the list), the list callee runs with the caller's other
/// arguments, and its result is returned.
pub(crate) fn text_scroll_list_forward(c: &mut Cpu, list_fn: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(TEXT_SCROLL_LIST_OFFSET));
        list_fn.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUITextScroll_ListLast(textScroll)` (0x3a6400): `UIList_Last(&list)`.
pub fn cui_text_scroll_list_last(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_7858,
            f: ssx3_ee::s13::f_00397858,
        },
        0x003a_6410,
    )
}

/// `cUITextScroll_ListGetAt(textScroll, index)` (0x3a6420): `UIList_GetAt(&list, index)`.
pub fn cui_text_scroll_list_get_at(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_7870,
            f: ssx3_ee::s13::f_00397870,
        },
        0x003a_6430,
    )
}

/// `cUITextScroll_ListPushFront(textScroll, node)` (0x3a6440): `UIList_PushFront(&list, node)`.
pub fn cui_text_scroll_list_push_front(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_78f0,
            f: ssx3_ee::s13::f_003978f0,
        },
        0x003a_6450,
    )
}

/// `cUITextScroll_ListPushBack(textScroll, node)` (0x3a6460): `UIList_PushBack(&list, node)`.
pub fn cui_text_scroll_list_push_back(c: &mut Cpu) {
    text_scroll_list_forward(
        c,
        Callee {
            addr: 0x0039_7910,
            f: ssx3_ee::s13::f_00397910,
        },
        0x003a_6470,
    )
}

/// `UIVector* UIVector_ctor_2(UIVector* self)` (0x3a64f8): `UIObject_ctor(self, 0, 0)`,
/// the vtable pointer at +8 (0x4941b8), `UIVectorFlags_ctor(&self->flags)` at +0x74,
/// returns `self`.
pub fn ui_vector_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0039_fb30,
            f: ssx3_ee::s13::f_0039fb30,
        }
        .call(c, 0x003a_6514)?;
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x41b8));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        Callee {
            addr: 0x003a_6618,
            f: ssx3_ee::s13::f_003a6618,
        }
        .call(c, 0x003a_6528)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UIObjectFlags_ctor(UIObject* o)` (0x3a6548): sets the flags word at +0 of an
/// object: bits 0, 1, 3 and 6 on; 2, 4, 5, 7 and bits 8..12 off; bits 13 and 16
/// on, bits 13..18 cleared first; the rest kept. Leaves the same scratch registers
/// the generated code does.
pub fn ui_object_flags_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let obj = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = obj.lw(c);
    c.g[V0][0] = 0xffff_ffff_ffff_ff7f;
    c.g[A1][0] = 0xffff_ffff_ffff_e0ff;
    c.g[A3][0] = 0xffff_ffff_ffff_ffdf;
    c.g[V1][0] |= 0x1;
    c.g[T0][0] = 0xffff_ffff_ffff_ffef;
    c.g[V1][0] &= c.g[V0][0];
    c.g[T1][0] = 0xffff_ffff_ffff_fffb;
    c.g[V1][0] &= c.g[A1][0];
    c.g[A2][0] = 0xffff_ffff_fff8_0000;
    c.g[V1][0] &= c.g[A3][0];
    c.g[A2][0] |= 0x1fff;
    c.g[V1][0] &= c.g[T0][0];
    c.g[A1][0] = 0x10000;
    c.g[V1][0] &= c.g[T1][0];
    c.g[A1][0] |= 0x2000;
    c.g[V1][0] |= 0x4a;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] &= c.g[A2][0];
    c.g[V1][0] |= c.g[A1][0];
    let flags = c.g[V1][0];
    obj.sw(c, flags);
    ret(c);
}

/// `UIListBoxFlags_ctor(attr)` (0x3a65a0): the byte at +0 becomes
/// `(byte | 4) & 0xc4`; returns `attr`.
pub fn ui_list_box_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let obj = Addr(c.g[A0][0] as u32);
    c.g[V1][0] = obj.lbu(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = (c.g[V1][0] | 0x4) & 0xc4;
    let flags = c.g[V1][0];
    obj.sb(c, flags);
    ret(c);
}

/// `UIListBoxScroll_ctor(entry)` (0x3a65b8): zeroes the words at +0, +4, +8,
/// +0xc and the byte at +0x10; returns `entry`.
pub fn ui_list_box_scroll_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.g[A0][0];
    let obj = Addr(c.g[A0][0] as u32);
    obj.sw(c, 0);
    obj.off(0x4).sw(c, 0);
    obj.off(0x8).sw(c, 0);
    obj.off(0xc).sw(c, 0);
    obj.off(0x10).sb(c, 0);
    ret(c);
}

/// Direct call targets of this module.
pub(crate) const UIENGINE_STATIC_INIT_DESTROY_CALLEE: Callee = Callee {
    addr: 0x003a_3f48,
    f: ee::s13::UIEngine_StaticInitDestroy,
};

/// Initial flag halfword of a `tAttrState` (stored at +0 by its ctor).
pub(crate) const UI_STATE_FLAGS_INIT: u64 = 0x218;

/// `tAttrText` flags: bit 2 set, then bits 0-2 and 7 kept.
pub(crate) const UI_TEXT_FLAGS_SET: u64 = 0x4;

pub(crate) const UI_TEXT_FLAGS_KEEP: u64 = 0x87;

/// `tAttrTextScroll` flags: bits 1, 2, 6 and 7 kept.
pub(crate) const UI_TEXT_SCROLL_FLAGS_KEEP: u64 = 0xc6;

/// `tAttrVector` flags word: bits 0 and 1 cleared, bits 17 and up cleared.
pub(crate) const UI_VECTOR_FLAGS_CLEAR_LOW: u64 = !1;

pub(crate) const UI_VECTOR_FLAGS_CLEAR_HIGH: u64 = !2;

pub(crate) const UI_VECTOR_FLAGS_LIMIT: u64 = 0x1_ffff;

/// `tAttrState::tAttrState(this)` (0x3a65d8): stores the halfword 0x218 at
/// +0 and returns `this`.
pub fn ui_state_flags_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = c.g[A0][0];
    c.g[V0][0] = this;
    c.g[V1][0] = UI_STATE_FLAGS_INIT;
    let init = c.g[V1][0];
    Addr(this as u32).sh(c, init);
    ret(c);
}

/// `tAttrText::tAttrText(attr)` (0x3a65e8): the flag byte at +0 becomes
/// `(flags | 4) & 0x87`; returns `attr`.
pub fn ui_text_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let attr = c.g[A0][0];
    let flags = Addr(attr as u32).lbu(c);
    c.g[V1][0] = (flags | UI_TEXT_FLAGS_SET) & UI_TEXT_FLAGS_KEEP;
    c.g[V0][0] = attr;
    let v = c.g[V1][0];
    Addr(attr as u32).sb(c, v);
    ret(c);
}

/// `tAttrTextScroll::tAttrTextScroll(attr)` (0x3a6600): the flag byte at +0
/// becomes `flags & 0xc6`; returns `attr`.
pub fn c_ui_text_scroll_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let attr = c.g[A0][0];
    let flags = Addr(attr as u32).lbu(c);
    c.g[V1][0] = flags & UI_TEXT_SCROLL_FLAGS_KEEP;
    c.g[V0][0] = attr;
    let v = c.g[V1][0];
    Addr(attr as u32).sb(c, v);
    ret(c);
}

/// `tAttrVector::tAttrVector(this)` (0x3a6618): the flag word at +0 keeps
/// only bits 2-16 (`word & 0x1fffc`), stored as a word; returns `this`.
pub fn ui_vector_flags_ctor(c: &mut Cpu) {
    c.cycles += 11;
    let this = c.g[A0][0];
    let word = Addr(this as u32).lw(c);
    c.g[A2][0] = UI_VECTOR_FLAGS_CLEAR_LOW;
    c.g[A3][0] = UI_VECTOR_FLAGS_CLEAR_HIGH;
    c.g[A1][0] = UI_VECTOR_FLAGS_LIMIT;
    let kept = word & c.g[A2][0] & c.g[A3][0] & c.g[A1][0];
    c.g[V1][0] = kept;
    c.g[V0][0] = this;
    Addr(this as u32).sw(c, kept);
    ret(c);
}

/// `UIEngine_GlobalCtors` (0x3a6648): `UIEngine_StaticInitDestroy(1, 0xffff)`.
pub fn ui_engine_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        c.g[A1][0] = 0xffff;
        UIENGINE_STATIC_INIT_DESTROY_CALLEE.call(c, 0x003a_665c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIEngine_GlobalDtors` (0x3a6668): `UIEngine_StaticInitDestroy(0, 0xffff)`.
pub fn ui_engine_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0xffff;
        UIENGINE_STATIC_INIT_DESTROY_CALLEE.call(c, 0x003a_667c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIObject_AnimLoopTail` (0x003a0274): the shared loop tail of the `setChannelDataKCP`
/// style loops: `t0 += 1`, `a2 += 4`, again from 0x3a0168 while `t0 < count`, else return.
pub fn ui_object_anim_loop_tail(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003a0274 => {
                    c.cycles += 4;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[T0][0] < c.g[A1][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                    if t {
                        c.pc = 0x003a0168;
                        call_indirect(c, 0x003a0168);
                        return Ok(());
                    }
                    pc = 0x003a0284;
                    continue;
                }
                0x003a0284 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;

                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003a0274, 0x003a028c);
                    return Ok(());
                }
            }
        }
    });
}

/// `UIVector_SetAnimParams(this, count, shorts)` (0x003a3808): like the `UIObject` one with
/// a divide by 9 of the index and a bound from `this+0x74`; jump table at 0x494140.
pub fn ui_vector_set_anim_params(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003a3808 => {
                    c.cycles += 4;
                    c.g[T0][0] = c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x80));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        c.pc = 0x003a39f0;
                        call_indirect(c, 0x003a39f0);
                        return Ok(());
                    }
                    pc = 0x003a3818;
                    continue;
                }
                0x003a3818 => {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[T2][0] = 0u64;
                    if t {
                        c.pc = 0x003a39f0;
                        call_indirect(c, 0x003a39f0);
                        return Ok(());
                    }
                    pc = 0x003a3820;
                    continue;
                }
                0x003a3820 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x2));
                    c.g[A0][0] = c.lh(c.g[A2][0] as u32);
                    c.mtc1(3, 1);
                    c.cvt_s_w(1, 1);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 21i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = (c.g[A0][0] < 0x11u64) as u64;
                    if t {
                        c.pc = 0x003a3918;
                        call_indirect(c, 0x003a3918);
                        return Ok(());
                    }
                    pc = 0x003a383c;
                    continue;
                }
                0x003a383c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x9;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x15));
                    c.div(0, 4, 2);

                    if c.g[V0][0] == 0u64 {
                        pc = 0x003a3850;
                        continue;
                    }
                    pc = 0x003a3850;
                    continue;
                }
                0x003a3850 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 17);
                    c.g[A0][0] = c.lo[0];
                    c.g[T1][0] = c.hi[0];
                    c.g[A3][0] = c.g[A0][0] & 0xffff;
                    c.g[V1][0] = (c.g[A3][0] < c.g[V1][0]) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V1][0] = (c.g[T1][0] < 0x9u64) as u64;
                    if t {
                        c.pc = 0x003a39e0;
                        ee::s13::f_003a39e0(c);
                        return Ok(());
                    }
                    pc = 0x003a3870;
                    continue;
                }
                0x003a3870 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        pc = 0x003a3878;
                        continue;
                    }
                    pc = 0x003a3878;
                    continue;
                }
                0x003a3878 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        c.pc = 0x003a39e0;
                        ee::s13::f_003a39e0(c);
                        return Ok(());
                    }
                    pc = 0x003a3880;
                    continue;
                }
                0x003a3880 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[T1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4140));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                _ => {
                    c.jump_out(pc, 0x003a3808, 0x003a3898);
                    return Ok(());
                }
            }
        }
    });
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::virtual_setup;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::bx::bxstring::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_6;
    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;

    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::main::gamemode::tests::w as w_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::object::railmodifier::tests::destructor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_3;

    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::ui::uithing::tests::ui_anim_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn ui_visitor_dtor_matches() {
        check(
            ee::s13::f_003a47a8__generated,
            ui_visitor_dtor,
            SEEDS,
            &|c, seed| destructor_setup(c, seed, 0x3a47_a8),
        );
    }

    #[test]
    fn ui_input_visitor_dtor_matches() {
        check(
            ee::s13::f_003a4cd0__generated,
            ui_input_visitor_dtor,
            SEEDS,
            &|c, seed| destructor_setup(c, seed, 0x3a4c_d0),
        );
    }

    #[test]
    fn ui_focus_visitor_dtor_matches() {
        check(
            ee::s13::f_003a4d40__generated,
            ui_focus_visitor_dtor,
            SEEDS,
            &|c, seed| destructor_setup(c, seed, 0x3a4d_40),
        );
    }

    #[test]
    fn c_ui_group_accept_matches() {
        check(
            ee::s13::f_003a51a0__generated,
            c_ui_group_accept,
            SEEDS,
            &|c, seed| {
                virtual_setup(c, seed, 0x3a51_a0, 0x10);
            },
        );
    }

    #[test]
    fn ui_focus_visitor_visit_matches() {
        check(
            ee::s13::f_003a4d70__generated,
            ui_focus_visitor_visit,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a4d_70, 0);
                let list = DATA + 0x400;
                let obj = DATA + 0x600;
                fill(c, &mut r, list, 0x20);
                fill(c, &mut r, obj, 0x20);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x38);
            },
        );
    }

    #[test]
    fn ui_input_visitor_visit_matches() {
        check(
            ee::s13::f_003a4d00__generated,
            ui_input_visitor_visit,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a4d00, 0);
                let obj = DATA + 0x200;
                let vt = DATA + 0x600;
                // A real, harmless method: the script engine singleton (0 when the
                // global is clear), so both the accept and reject results occur.
                c.bus.write16(vt + 0x78, 0);
                c.bus.write32(vt + 0x7c, 0x0027_d2e8);
                c.g[A0][0] = (DATA + 0x300) as u64;
                c.g[A1][0] = obj as u64;
                c.bus.write32(DATA + 0x304, (seed % 4) as u32);
                c.bus.write32(obj + 8, vt);
                c.bus.write32(
                    GAME_GP + 0x3ec,
                    if seed % 2 == 0 { 0 } else { DATA + 0x500 },
                );
            },
        );
    }

    #[test]
    fn ui_vector_update_anim_matches() {
        check(
            ee::s13::f_003a3d28__generated,
            ui_vector_update_anim,
            40,
            &|c, seed| {
                ui_anim_setup(c, seed, 0x3a_3d28);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3a54b8 UIPair_dtor / 0x3a5658 UIListBox_dtor / 0x3a5da8 UIText_dtor.

    pub(crate) fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = ((seed as u64) & 0xfe) << 1; // flags bit 0 clear: no delete
                                                  // A zeroed UI object with empty child lists.
        for off in [0x78u32, 0x7c, 0xc, 0xac, 0xb4] {
            c.bus.write32(this + off, 0);
        }
        c.bus.write32(this + 0x9c, 0);
        c.bus.write8(this + 0x318, 0);
        let owner = DATA + 0x800;
        c.bus.write32(this + 0x10, owner);
        c.bus.write32(owner + 0x28, 0);
        let _ = r.next_u32();
    }

    #[test]
    fn ui_pair_dtor_matches() {
        check(
            ee::s13::f_003a54b8__generated,
            ui_pair_dtor,
            8,
            &|c, seed| dtor_setup(c, seed, 0x3a54_b8),
        );
    }

    #[test]
    fn ui_list_box_dtor_matches() {
        check(
            ee::s13::f_003a5658__generated,
            ui_list_box_dtor,
            8,
            &|c, seed| dtor_setup(c, seed, 0x3a56_58),
        );
    }

    #[test]
    fn ui_text_dtor_matches() {
        check(
            ee::s13::f_003a5da8__generated,
            ui_text_dtor,
            8,
            &|c, seed| dtor_setup(c, seed, 0x3a5d_a8),
        );
    }

    #[test]
    fn ui_list_node_dtor_matches() {
        check(
            ee::s13::f_003a47f8__generated,
            ui_list_node_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a47_f8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ui_thread_dtor_matches() {
        check(
            ee::s13::f_003a5b48__generated,
            ui_thread_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a5b_48, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// A fixture with `gp` set to the game's value and scrambled registers.
    pub(crate) fn start(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x2545_f491_4f6c_dd1d ^ (seed as u64) << 4 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    // -------------------------------------------------------------- UI flags

    /// One shared test body for the `bitN = enable` setters.
    pub(crate) fn check_flag_setter(hand: GuestFn, generated: GuestFn, off: u32) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = start(c, seed, off as u64);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 3 == 0 {
                0
            } else {
                r.next_u32() as u64 | (r.next_u32() as u64) << 32
            };
            c.bus.write32(DATA + off, r.next_u32());
        });
    }

    #[test]
    fn ui_object_v04_matches() {
        check_flag_setter(
            ui_object_v04,
            ssx3_ee::s13::f_003a49e8__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_v05_matches() {
        check_flag_setter(
            ui_object_v05,
            ssx3_ee::s13::f_003a4a08__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_v06_matches() {
        check_flag_setter(
            ui_object_v06,
            ssx3_ee::s13::f_003a4a28__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_set_focus_matches() {
        check_flag_setter(
            ui_object_set_focus,
            ssx3_ee::s13::f_003a4a48__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_v08_matches() {
        check_flag_setter(
            ui_object_v08,
            ssx3_ee::s13::f_003a4a68__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_object_set_visible_matches() {
        check_flag_setter(
            ui_object_set_visible,
            ssx3_ee::s13::f_003a4a88__generated,
            UI_FLAGS_OBJECT,
        );
    }

    #[test]
    fn ui_list_box_v24_matches() {
        check_flag_setter(
            ui_list_box_v24,
            ssx3_ee::s13::f_003a5708__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v23_matches() {
        check_flag_setter(
            ui_text_v23,
            ssx3_ee::s13::f_003a5e68__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v24_matches() {
        check_flag_setter(
            ui_text_v24,
            ssx3_ee::s13::f_003a5e88__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v25_matches() {
        check_flag_setter(
            ui_text_v25,
            ssx3_ee::s13::f_003a5ea8__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v26_matches() {
        check_flag_setter(
            ui_text_v26,
            ssx3_ee::s13::f_003a5ec8__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn ui_text_v27_matches() {
        check_flag_setter(
            ui_text_v27,
            ssx3_ee::s13::f_003a5ee8__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn c_ui_text_scroll_v23_matches() {
        check_flag_setter(
            c_ui_text_scroll_v23,
            ssx3_ee::s13::f_003a6378__generated,
            UI_FLAGS_WIDGET,
        );
    }

    #[test]
    fn c_ui_text_scroll_v24_matches() {
        check_flag_setter(
            c_ui_text_scroll_v24,
            ssx3_ee::s13::f_003a6398__generated,
            UI_FLAGS_WIDGET,
        );
    }

    // ------------------------------------------------------- UI vec3 copies

    #[test]
    fn ui_object_set_color_matches() {
        check(
            ssx3_ee::s13::f_003a4af8__generated,
            ui_object_set_color,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x1c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
                // The unaligned pairs also read-modify-write the neighbouring
                // words of the 16-byte destination at DATA + 0x41c.
                for i in 0..6 {
                    c.bus.write32(DATA + 0x418 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_object_get_vec3_at60_matches() {
        check(
            ssx3_ee::s13::f_003a4c90__generated,
            ui_object_get_vec3_at60,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x60);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + 0x400 + 0x60 + 4 * i, r.next_u32());
                }
                // The 12-byte destination at DATA + 0x100, including the
                // neighbouring words its unaligned pairs read-modify-write.
                for i in 0..6 {
                    c.bus.write32(DATA + 0x0f8 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_object_set_vec3_at60_matches() {
        check(
            ssx3_ee::s13::f_003a4cb0__generated,
            ui_object_set_vec3_at60,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x60);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
                for i in 0..6 {
                    c.bus.write32(DATA + 0x458 + 4 * i, r.next_u32());
                }
            },
        );
    }

    // --------------------------------------------------------- UI vectors

    #[test]
    fn ui_vector_v20_matches() {
        check(
            ssx3_ee::s13::f_003a3d70__generated,
            ui_vector_v20,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x3d70);
                let vec = DATA + 0x400;
                c.g[A0][0] = vec as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let n = match seed % 4 {
                    0 => 0, // no data pointer at all
                    1 => 0, // count 0: only the default extent
                    _ => 1 + seed % 3,
                };
                c.bus
                    .write32(vec + 0x80, if seed % 4 == 0 { 0 } else { DATA + 0x800 });
                c.bus
                    .write32(vec + 0x74, (n << 17) | (r.next_u32() & 0x1_ffff));
                for i in 0..n {
                    c.bus.write32(DATA + 0x800 + 0xc * i, r.edge_f32());
                    c.bus.write32(DATA + 0x804 + 0xc * i, r.edge_f32());
                }
                // The default extent constant.
                for i in 0..3 {
                    c.bus.write32(0x004f_f0d8 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_vector_v21_matches() {
        check(
            ssx3_ee::s13::f_003a3e38__generated,
            ui_vector_v21,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x3e38);
                let vec = DATA + 0x400;
                c.g[A0][0] = vec as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(
                    vec + 0x74,
                    if seed % 2 == 0 {
                        0x8_0000
                    } else {
                        r.next_u32() & 0xfffe_0000
                    },
                );
                c.bus.write32(vec + 0x80, DATA + 0x800);
                for i in 0..4 * 3 {
                    c.bus.write32(DATA + 0x800 + 4 * i, r.edge_f32());
                }
                for i in 0..3 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.edge_f32());
                }
                // The 12-byte copy into the vec3 at +0x60.
                for i in 0..4 {
                    c.bus.write32(DATA + 0x45c + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_group_v03_matches() {
        check(
            ee::s13::f_003a51d0__generated,
            ui_group_v03,
            SEEDS_4,
            &|c, seed| {
                let mut r = base(c, seed, 0x3a51_d0);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                let vt = DATA + 0x580;
                vtable_2(c, vt, &[(0x10, FAKE + 0x40)]);
                let holder = DATA + 0x300;
                clear(c, holder, 8);
                w(c, holder, vt as u32);
                let list = s + 0x74;
                let sent = DATA + 0x500;
                w(c, sent, sent as u32);
                w(c, sent + 4, sent as u32);
                w(c, list + 4, sent as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(holder);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ui_list_dtor_matches() {
        check(
            ee::s13::f_003a4888__generated,
            ui_list_dtor,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x3a4888, 4);
                be_library_2(c);
                let list = DATA;
                c.g[A0][0] = list as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let s = DATA + 0x2000;
                if seed % 3 == 0 {
                    w_2(c, list + 4, s);
                    w_2(c, s + 4, s);
                    w_2(c, s, s);
                } else {
                    // head -> a -> terminator (self-linked at +4)
                    let a = DATA + 0x2100;
                    w_2(c, list + 4, a);
                    w_2(c, a + 4, s);
                    w_2(c, a, 0);
                    w_2(c, s + 4, s);
                    w_2(c, s, 0);
                    let vt = DATA + 0x3000;
                    w_2(c, a + 8, vt);
                    slot(c, vt, 8, 0x20, FAKE);
                }
            },
        );
    }

    #[test]
    fn c_ui_group_dtor_matches() {
        check(
            ee::s13::f_003a5068__generated,
            c_ui_group_dtor,
            150,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3a_5068);
                globals(c);
                pool(c);
                // Empty child list and zeroed embedded list nodes.
                w_3(c, DATA + 0x74, 0);
                w_3(c, DATA + 0x7c, 0);
                w_3(c, DATA + 0x80, 0);
                w_3(c, DATA + 0x88, 0);
                w_3(c, DATA + 0x8c, 0);
                c.g[A0][0] = sx(DATA);
                // Even flags: the base_2 destructor must not free test data.
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn ui_menu_dtor_matches() {
        check(
            ee::s13::f_003a52a0__generated,
            ui_menu_dtor,
            150,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x3a_52a0);
                globals(c);
                pool(c);
                w_3(c, DATA + 0x74, 0);
                w_3(c, DATA + 0x7c, 0);
                w_3(c, DATA + 0x80, 0);
                w_3(c, DATA + 0x88, 0);
                w_3(c, DATA + 0x8c, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_dtor_matches() {
        check(
            ee::s13::f_003a62a0__generated,
            c_ui_text_scroll_dtor,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x3a62, 0);
                env(c);
                let this = DATA + 0x400;
                let sentinel = DATA + 0x1000;
                w32(c, sentinel + 4, sentinel);
                w32(c, this + 0x8c, sentinel);
                w32(c, this + 0x8c + 4, sentinel);
                w32(c, this + 0x98, sentinel);
                w32(c, this + 0x98 + 4, sentinel);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) use crate::game::object::instancenode::tests::oc383_floats;
    pub(crate) use crate::game::seg::seg_2fe810::tests::oc383_machine;

    #[test]
    fn ui_vector_render_matches() {
        use crate::difftest::fakes::{machine, vtable};
        check(
            ee::s13::f_003a39f8__generated,
            ui_vector_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39f8, 1);
                c.pc = 0x003a_39f8;
                let this = DATA;
                fill(c, &mut r, this, 0x28);
                oc383_floats(c, &mut r, this + 0x44, 2, 300.0);
                oc383_floats(c, &mut r, this + 0x50, 2, 3.0);
                let count = seed % 5;
                let indexed = (seed / 5) % 2;
                let counts =
                    count << 17 | (r.next_u32() & 0x7fff) << 2 | (r.next_u32() & 2) | indexed;
                c.bus.write32(this + UI_VECTOR_COUNTS, counts);
                let positions = if seed % 11 == 3 {
                    0
                } else {
                    DATA + 0x1000 + (seed % 3) * 4
                };
                let render = DATA + 0x2000 + ((seed / 3) % 3) * 4;
                let vertices = if seed % 13 == 5 { 0 } else { DATA + 0x3000 };
                c.bus.write32(this + UI_VECTOR_POSITIONS, positions);
                c.bus.write32(this + UI_VECTOR_RENDER_POSITIONS, render);
                c.bus.write32(this + UI_VECTOR_VERTICES, vertices);
                c.bus.write32(this + UI_VECTOR_COLOURS, DATA + 0x4000);
                c.bus.write32(this + UI_VECTOR_TRIANGLES, r.next_u32());
                oc383_floats(c, &mut r, DATA + 0x1000, 24, 200.0);
                fill(c, &mut r, DATA + 0x2000, 24);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                for i in 0..24 {
                    let v = (r.next_u32() % 2560) as f32 / 10.0;
                    c.bus.write32(DATA + 0x4000 + 4 * i, v.to_bits());
                }
                // GrMan with its draw state and vtable.
                let grman = DATA + 0x5000;
                c.bus.write32(GAME_GP - UI_VECTOR_GRMAN_GP, grman);
                c.bus.write32(grman + 0xe84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 8);
                let vt = vtable(c, &mut r, &[0x240, 0x248]);
                c.bus.write32(grman + 0x10d8, vt);
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(100.0);
            },
        );
    }

    #[test]
    fn ui_vector_set_anim_param_matches() {
        check(
            ee::s13::f_003a36d0__generated,
            ui_vector_set_anim_param,
            300,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0x36d0, 0x003a_36d0);
                let this = DATA;
                fill(c, &mut r, this, 0x28);
                let count = 1 + seed % 4;
                c.bus.write32(
                    this + UI_VECTOR_COUNTS,
                    count << 17 | (r.next_u32() & 0x1ffff),
                );
                let verts = if seed % 17 == 4 { 0 } else { DATA + 0x1000 };
                c.bus.write32(this + UI_VECTOR_POSITIONS, verts);
                c.bus.write32(this + 0x88, DATA + 0x2000);
                c.bus.write32(this + UI_VECTOR_COLOURS, DATA + 0x3000);
                let key = match seed % 6 {
                    0 => seed % 0x15,
                    // Vertex index up to `count` (one past the end too).
                    _ => 0x15 + (seed / 6) % (9 * (count + 1)),
                };
                c.g[A1][0] =
                    (r.next_u32() as u64) << 32 | (r.next_u32() & 0xffff_0000) as u64 | key as u64;
                c.f[12] = r.f32(100.0);
            },
        );
    }

    pub(crate) use crate::game::fe::fepopup::tests::oc508_setup;

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

    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    pub(crate) use crate::game::fe::ovstatereplay::tests::OC715_GATE;
    pub(crate) use crate::game::fe::ovstatereplay::tests::OC715_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `cUIVector_setUIData` on a zeroed-object base: the base load
    /// (`UIObject_Load`) is the real function, the element arrays come from
    /// the test heap, and `data->count` (0..=3) picks the loop and the
    /// allocation sizes; the flag word at +0x74 is kept small so the 48-byte
    /// block is taken for some seeds and skipped for others.
    #[test]
    fn cui_vector_set_ui_data_matches() {
        const SELF: u32 = DATA + 0x9000;
        const OBJ: u32 = DATA + 0x9200;
        check(
            ee::s13::f_003a34c0__generated,
            cui_vector_set_ui_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a34_c0 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x003a_34c0;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OBJ);
                fill(c, &mut r, SELF, 0x40);
                fill(c, &mut r, OBJ, 0x20);
                c.bus.write8(OBJ + 0x20, (seed % 4) as u8);
                c.bus.write32(SELF + 0x74, r.next_u32() & 0x0003_ffff);
            },
        );
    }

    pub(crate) static OC715_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc715_fake_object_dtor(c: &mut Cpu) {
        if c.bus.read32(OC715_GATE) != OC715_MAGIC {
            return fall_back(c, &OC715_PREV_DTOR, ee::s13::f_0039fc48__generated);
        }
        fake_call(c, 0x0039_fc48);
    }

    #[test]
    fn uiicon_dtor_matches() {
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039FC48),
            oc715_fake_object_dtor,
            &OC715_PREV_DTOR,
        );
        check(
            ee::s13::f_003a4e80__generated,
            uiicon_dtor,
            100,
            &|c, seed| {
                let mut r = Rng(0x003a_4e80 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC715_GATE, OC715_MAGIC);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
            },
        );
    }

    use crate::difftest::RA as DT_RA;

    use crate::difftest::fakes;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine_rt;

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    use crate::game::fe::feasyncfile::tests::base_13 as base_m2;

    pub(crate) use crate::game::ui::uitext::tests::gates_on;

    pub(crate) use crate::game::ui::uitext::tests::install;

    /// `UIEngine_StaticInitDestroy` against the generated body: the init
    /// path (colour table, `UISound_BaseCtor` real), the destroy path
    /// (vtable store) and the wrong-priority exit.
    #[test]
    fn oc1079_ui_engine_static_init_destroy_matches() {
        check(
            ee::s13::f_003a3f48__generated,
            oc1079_ui_engine_static_init_destroy,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1079, 0);
                let lit = GAME_GP.wrapping_sub(0x2500);
                for i in 0..0x80u32 {
                    c.bus.write32(lit + 4 * i, r.next_u32());
                }
                c.g[A0][0] = if seed % 3 == 1 {
                    0
                } else {
                    1 + (seed as u64 % 5)
                };
                c.g[A1][0] = if seed % 5 == 4 { 0xfffe } else { 0xffff };
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    #[test]
    fn ui_vector_return_self_matches() {
        check(
            ee::s13::f_003a3f08__generated,
            ui_vector_return_self,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a3f_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn ui_object_ctor_2_matches() {
        install();
        check(
            ee::s13::f_003a4900__generated,
            ui_object_ctor_2,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x3a49_00, 2);
                gates_on(c);
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    /// Object for the constructor tests: random words at `DATA + 0x400`
    /// (the group or icon), random a1/a2 for the pass-through callers.
    pub(crate) fn oc1085_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        fill(c, &mut r, DATA + 0x400, 0x40);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(r.next_u32());
    }

    /// Group whose child list (object +0x74) has a head and a tail node at
    /// `DATA + 0x800`; the node's link word is the zero/non-zero switch for
    /// `UIList_First` and `UIList_Last`.
    pub(crate) fn oc1085_list_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        fill(c, &mut r, DATA + 0x400, 0x40);
        c.g[A0][0] = sx(DATA + 0x400);
        r
    }

    #[test]
    fn ui_sound_base_dtor_matches() {
        check(
            ee::s13::f_003a4da8__generated,
            ui_sound_base_dtor,
            200,
            &|c, seed| destructor_setup(c, seed, 0x003a_4da8),
        );
    }

    #[test]
    fn uiicon_ctor_2_matches() {
        check(
            ee::s13::f_003a4df0__generated,
            uiicon_ctor_2,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_4df0),
        );
    }

    #[test]
    fn uiicon_ctor_matches() {
        check(
            ee::s13::f_003a4e38__generated,
            uiicon_ctor,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_4e38),
        );
    }

    #[test]
    fn c_ui_group_set_offset_matches() {
        check(
            ee::s13::f_003a4fb8__generated,
            c_ui_group_set_offset,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003a_4fb8, 0);
                fill(c, &mut r, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(DATA + 0x400);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
            },
        );
    }

    #[test]
    fn c_ui_group_ctor_2_matches() {
        check(
            ee::s13::f_003a4fe0__generated,
            c_ui_group_ctor_2,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_4fe0),
        );
    }

    #[test]
    fn c_ui_group_ctor_matches() {
        check(
            ee::s13::f_003a5028__generated,
            c_ui_group_ctor,
            200,
            &|c, seed| oc1085_ctor_fixture(c, seed, 0x003a_5028),
        );
    }

    #[test]
    fn c_ui_group_first_child_matches() {
        check(
            ee::s13::f_003a5100__generated,
            c_ui_group_first_child,
            200,
            &|c, seed| {
                let mut r = oc1085_list_fixture(c, seed, 0x003a_5100);
                let node = DATA + 0x800;
                let head_link = if seed % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(node + 0x4, head_link);
                c.bus.write32(DATA + 0x400 + 0x74 + 0x4, node);
            },
        );
    }

    #[test]
    fn c_ui_group_last_child_matches() {
        check(
            ee::s13::f_003a5120__generated,
            c_ui_group_last_child,
            200,
            &|c, seed| {
                let mut r = oc1085_list_fixture(c, seed, 0x003a_5120);
                let node = DATA + 0x800;
                let tail_link = if seed % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(node, tail_link);
                c.bus.write32(DATA + 0x400 + 0x74 + 0xc, node);
            },
        );
    }

    #[test]
    fn ui_group_get_child_at_matches() {
        check(
            ssx3_ee::s13::f_003a5140__generated,
            ui_group_get_child_at,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_5140, 0);
                let group = DATA + 0x1000;
                let list = group + 0x74;
                c.g[A0][0] = sx(group);
                c.g[A1][0] = (seed % 5) as u64;
                let s = DATA + 0x2000;
                if seed % 4 == 0 {
                    c.bus.write32(list + 4, s);
                    c.bus.write32(s + 4, s);
                    c.bus.write32(s, s);
                } else {
                    // head -> a -> b -> sentinel
                    let a = DATA + 0x2100;
                    let b = DATA + 0x2200;
                    c.bus.write32(list + 4, a);
                    c.bus.write32(a + 4, b);
                    c.bus.write32(a, 0);
                    c.bus.write32(b + 4, s);
                    c.bus.write32(b, 0);
                    c.bus.write32(s + 4, s);
                    c.bus.write32(s, 0);
                }
            },
        );
    }

    #[test]
    fn ui_group_push_front_child_matches() {
        check(
            ssx3_ee::s13::f_003a5160__generated,
            ui_group_push_front_child,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_5160, 0);
                let mut r = Rng(0x003a_5160 ^ seed as u64);
                let group = DATA + 0x1000;
                let list = group + 0x74;
                let item = DATA + 0x1100;
                let node = DATA + 0x1200;
                c.g[A0][0] = sx(group);
                c.g[A1][0] = sx(item);
                // An empty list (head = itself) or one node in front.
                c.bus
                    .write32(list + 4, if seed % 2 == 0 { node } else { list });
                c.bus.write32(node, r.next_u32());
                c.bus.write32(item, r.next_u32());
                c.bus.write32(item + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_group_push_back_child_matches() {
        check(
            ssx3_ee::s13::f_003a5180__generated,
            ui_group_push_back_child,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_5180, 0);
                let mut r = Rng(0x003a_5180 ^ seed as u64);
                let group = DATA + 0x1000;
                let list = group + 0x74;
                let item = DATA + 0x1100;
                let node = DATA + 0x1200;
                c.g[A0][0] = sx(group);
                c.g[A1][0] = sx(item);
                // An empty list (tail = itself) or one node at the back.
                c.bus
                    .write32(list + 0xc, if seed % 2 == 0 { node } else { list });
                c.bus.write32(node + 4, r.next_u32());
                c.bus.write32(item, r.next_u32());
                c.bus.write32(item + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_menu_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a5230__generated,
            ui_menu_ctor,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x003a_5230, 0);
                let menu = DATA + 0x1000;
                c.g[A0][0] = sx(menu);
                c.pc = 0x003a_5230;
            },
        );
    }

    #[test]
    fn ui_pair_ctor_2_matches() {
        check(
            ssx3_ee::s13::f_003a5420__generated,
            ui_pair_ctor_2,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x003a_5420, 0);
                let pair = DATA + 0x1000;
                c.bus.write32(pair + 0x14, r.next_u32());
                c.g[A0][0] = sx(pair);
                c.pc = 0x003a_5420;
            },
        );
    }

    #[test]
    fn ui_pair_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a5468__generated,
            ui_pair_ctor,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x003a_5468, 0);
                let pair = DATA + 0x1000;
                // Flags word with and without bit 1 already set.
                c.bus.write32(pair + 0x14, r.next_u32());
                c.bus.write32(pair + 0x78, r.next_u32());
                c.bus.write32(pair + 0x7c, r.next_u32());
                c.g[A0][0] = sx(pair);
                c.pc = 0x003a_5468;
            },
        );
    }

    /// The pair and its old widget (whose vtable slot 3 is a fake), with the
    /// new widget in a1. A third of the seeds leave the slot empty, and
    /// `machine(.., 2)` makes some fake calls unwind.
    pub(crate) fn pair_set_setup(c: &mut Cpu, seed: u32, field: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 2);
        let pair = DATA + 0x1000;
        let old = DATA + 0x1400;
        let vt = DATA + 0x1800;
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + 0x8, delta as u16);
        c.bus.write32(vt + 0xc, FAKE + 0x10 * (seed % 2));
        c.bus.write32(old + 0x8, vt);
        c.bus
            .write32(pair + field, if seed % 4 == 0 { 0 } else { old });
        c.g[A0][0] = sx(pair);
        c.g[A1][0] = sx(DATA + 0x1200);
    }

    #[test]
    fn ui_pair_set_first_matches() {
        covered(
            ssx3_ee::s13::f_003a5530__generated,
            ui_pair_set_first,
            200,
            &|c, seed| pair_set_setup(c, seed, 0x78, 0x003a_5530),
        );
    }

    #[test]
    fn ui_pair_set_second_matches() {
        covered(
            ssx3_ee::s13::f_003a5588__generated,
            ui_pair_set_second,
            200,
            &|c, seed| pair_set_setup(c, seed, 0x7c, 0x003a_5588),
        );
    }

    /// Common constructor fixture: scrambled registers, a random object at
    /// `DATA` (its first words), and `a0` pointing at it.
    pub(crate) fn ui_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        fill(c, &mut r, DATA, 0x80);
        c.g[A0][0] = sx(DATA);
        r
    }

    driven!(gen_ui_list_box_ctor_2, ee::s13::f_003a55d8__generated);

    driven!(hand_ui_list_box_ctor_2, ui_list_box_ctor_2);

    #[test]
    fn ui_list_box_ctor_2_matches() {
        check(
            ee::s13::f_003a55d8__generated,
            ui_list_box_ctor_2,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_55d8);
            },
        );
        // The scroll loop's back edge carries an event check: make the
        // first one due so both bodies deliver it at the same place.
        check_events(
            gen_ui_list_box_ctor_2,
            hand_ui_list_box_ctor_2,
            96,
            &|c, seed| {
                let mut r = ui_ctor_fixture(c, seed, 0x003a_55d9);
                c.g[RA][0] = crate::difftest::RA as u64;
                let mode = schedule(c, &mut r, seed, 0x003a_55d8);
                c.next_event = c.cycles + 4;
                mode
            },
        );
    }

    #[test]
    fn ui_state_ctor_3_matches() {
        check(
            ee::s13::f_003a57b8__generated,
            ui_state_ctor_3,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_57b8);
            },
        );
    }

    #[test]
    fn ui_screen_ctor_matches() {
        check(
            ee::s13::f_003a5b98__generated,
            ui_screen_ctor,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_5b98);
            },
        );
    }

    #[test]
    fn ui_screen_ctor_2_matches() {
        check(
            ee::s13::f_003a5c08__generated,
            ui_screen_ctor_2,
            200,
            &|c, seed| {
                let mut r = ui_ctor_fixture(c, seed, 0x003a_5c08);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ui_screen_add_root_child_matches() {
        check(
            ee::s13::f_003a5cf0__generated,
            ui_screen_add_root_child,
            200,
            &|c, seed| {
                let mut r = ui_ctor_fixture(c, seed, 0x003a_5cf0);
                // The child list at +0xb4 must hold a valid tail node: the
                // generated push reads and writes through it.
                c.bus.write32(DATA + 0xb4 + 0xc, DATA + 0x100);
                c.bus.write32(DATA + 0x104, r.next_u32());
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ui_text_ctor_2_matches() {
        check(
            ee::s13::f_003a5d60__generated,
            ui_text_ctor_2,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_5d60);
            },
        );
    }

    #[test]
    fn cui_scroll_ctor_2_matches() {
        check(
            ee::s13::f_003a5f60__generated,
            cui_scroll_ctor_2,
            200,
            &|c, seed| {
                ui_ctor_fixture(c, seed, 0x003a_5f60);
            },
        );
    }

    #[test]
    fn cui_scroll_ctor_matches() {
        check(
            ee::s13::f_003a5fa0__generated,
            cui_scroll_ctor,
            200,
            &|c, seed| {
                // The generated body passes a1/a2 through to UIObject_ctor
                // unchanged, so they stay scrambled here.
                ui_ctor_fixture(c, seed, 0x003a_5fa0);
            },
        );
    }

    /// A scroll at `DATA+0x1000` with junk around its name field (`+0x8c` and
    /// `+0xa5`), and a NUL-terminated thread name at `DATA+0x2000` of
    /// `seed % 41` bytes: names of 25 bytes or more take the clearing path.
    pub(crate) fn scroll_thread_name_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let scroll = DATA + 0x1000;
        let name = DATA + 0x2000;
        fill(c, &mut r, scroll + 0x80, 0x20);
        let len = seed % 41;
        for k in 0..len {
            c.bus.write8(name + k, b'a' + (k % 26) as u8);
        }
        c.bus.write8(name + len, 0);
        c.g[A0][0] = sx(scroll);
        c.g[A1][0] = sx(name);
    }

    /// A widget at `DATA+0x1000` filled with junk, for the constructors that
    /// only write their own fields and call the base constructor.
    pub(crate) fn widget_ctor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let obj = DATA + 0x1000;
        fill(c, &mut r, obj, 0x40);
        c.g[A0][0] = sx(obj);
    }

    #[test]
    fn ui_scroll_set_up_thread_name_matches() {
        check(
            ee::s13::f_003a6038__generated,
            ui_scroll_set_up_thread_name,
            200,
            &|c, seed| scroll_thread_name_setup(c, seed, 0x3a_6038),
        );
    }

    #[test]
    fn ui_scroll_set_down_thread_name_matches() {
        check(
            ee::s13::f_003a6088__generated,
            ui_scroll_set_down_thread_name,
            200,
            &|c, seed| scroll_thread_name_setup(c, seed, 0x3a_6088),
        );
    }

    #[test]
    fn ui_progress_ctor_2_matches() {
        check(
            ee::s13::f_003a60e0__generated,
            ui_progress_ctor_2,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_60e0),
        );
    }

    #[test]
    fn ui_progress_ctor_matches() {
        check(
            ee::s13::f_003a6120__generated,
            ui_progress_ctor,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_6120),
        );
    }

    #[test]
    fn ui_slider_ctor_2_matches() {
        check(
            ee::s13::f_003a6190__generated,
            ui_slider_ctor_2,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_6190),
        );
    }

    #[test]
    fn ui_slider_ctor_matches() {
        check(
            ee::s13::f_003a61d0__generated,
            ui_slider_ctor,
            200,
            &|c, seed| widget_ctor_setup(c, seed, 0x3a_61d0),
        );
    }

    #[test]
    fn ui_text_scroll_ctor_2_matches() {
        check(
            ee::s13::f_003a6250__generated,
            ui_text_scroll_ctor_2,
            200,
            &|c, seed| {
                let mut r = base_m2(c, seed, 0x3a_6250, 4);
                let obj = DATA + 0x8000;
                fill(c, &mut r, obj, 0x40);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn ui_text_scroll_list_first_matches() {
        check(
            ee::s13::f_003a63e0__generated,
            ui_text_scroll_list_first,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_63e0, 0);
                let obj = DATA + 0x1000;
                let node = DATA + 0x1200;
                fill(c, &mut r, obj, 0x40);
                // The list head at `obj+0x8c` points at `node`; `node+4` is
                // the node itself (empty) on even seeds, junk otherwise.
                c.bus.write32(obj + 0x8c + 4, node);
                let next = if seed % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(node + 4, next);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    /// The textScroll object of the list tests; its list is at +0x8c.
    pub(crate) const LIST_TEST_TEXT_SCROLL: u32 = DATA + 0x1000;

    /// The list sentinel: self-linked, so a list of only this node is empty.
    pub(crate) const LIST_TEST_SENTINEL: u32 = DATA + 0x2080;

    /// The first of up to three list nodes (0x20 bytes apart).
    pub(crate) const LIST_TEST_NODES: u32 = DATA + 0x2100;

    /// A node that is not in the list yet (the push tests add it).
    pub(crate) const LIST_TEST_NEW_NODE: u32 = DATA + 0x2200;

    /// The object the flag and scroll constructors write.
    pub(crate) const LIST_TEST_OBJECT: u32 = DATA + 0x4000;

    /// A text scroll whose list holds `seed % 4` nodes between the sentinel and
    /// the head/last pointers. Every few seeds the last node is self-prev (the
    /// `UIList_Last` zero branch).
    pub(crate) fn list_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1089_0000 ^ seed as u64);
        scramble(c, &mut r);
        let list = LIST_TEST_TEXT_SCROLL + TEXT_SCROLL_LIST_OFFSET;
        let k = seed % 4;
        c.bus.write32(LIST_TEST_SENTINEL, LIST_TEST_SENTINEL);
        c.bus.write32(LIST_TEST_SENTINEL + 4, LIST_TEST_SENTINEL);
        for i in 0..k {
            let node = LIST_TEST_NODES + 0x20 * i;
            let next = if i + 1 < k {
                LIST_TEST_NODES + 0x20 * (i + 1)
            } else {
                LIST_TEST_SENTINEL
            };
            c.bus.write32(node, LIST_TEST_SENTINEL);
            c.bus.write32(node + 4, next);
        }
        let (first, last) = if k > 0 {
            (LIST_TEST_NODES, LIST_TEST_NODES + 0x20 * (k - 1))
        } else {
            (LIST_TEST_SENTINEL, LIST_TEST_SENTINEL)
        };
        c.bus.write32(list + 4, first);
        c.bus.write32(list + 0xc, last);
        if k > 0 && seed % 6 == 5 {
            c.bus.write32(last, last);
        }
    }

    /// [`list_fixture`] plus the forwarder's arguments: `a0` = textScroll, `a1` = `a1`.
    pub(crate) fn list_setup(c: &mut Cpu, seed: u32, a1: u64) {
        list_fixture(c, seed);
        c.g[A0][0] = sx(LIST_TEST_TEXT_SCROLL);
        c.g[A1][0] = a1;
    }

    #[test]
    fn cui_text_scroll_list_last_matches() {
        check(
            ssx3_ee::s13::f_003a6400__generated,
            cui_text_scroll_list_last,
            300,
            &|c, seed| list_setup(c, seed, sx(seed)),
        );
    }

    #[test]
    fn cui_text_scroll_list_get_at_matches() {
        check(
            ssx3_ee::s13::f_003a6420__generated,
            cui_text_scroll_list_get_at,
            300,
            &|c, seed| {
                // 0 and a negative index take the first-node path; others walk.
                let index = match seed % 7 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => seed % 5,
                };
                list_setup(c, seed, sx(index));
            },
        );
    }

    #[test]
    fn cui_text_scroll_list_push_front_matches() {
        check(
            ssx3_ee::s13::f_003a6440__generated,
            cui_text_scroll_list_push_front,
            300,
            &|c, seed| list_setup(c, seed, sx(LIST_TEST_NEW_NODE)),
        );
    }

    #[test]
    fn cui_text_scroll_list_push_back_matches() {
        check(
            ssx3_ee::s13::f_003a6460__generated,
            cui_text_scroll_list_push_back,
            300,
            &|c, seed| list_setup(c, seed, sx(LIST_TEST_NEW_NODE)),
        );
    }

    #[test]
    fn ui_vector_ctor_2_matches() {
        check(
            ssx3_ee::s13::f_003a64f8__generated,
            ui_vector_ctor_2,
            100,
            &|c, seed| {
                let mut r = Rng(0x1089_1000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, LIST_TEST_OBJECT, 32);
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }

    #[test]
    fn ui_object_flags_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a6548__generated,
            ui_object_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1089_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(LIST_TEST_OBJECT, r.next_u32());
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }

    #[test]
    fn ui_list_box_flags_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a65a0__generated,
            ui_list_box_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1089_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(LIST_TEST_OBJECT, r.next_u32());
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }

    #[test]
    fn ui_list_box_scroll_ctor_matches() {
        check(
            ssx3_ee::s13::f_003a65b8__generated,
            ui_list_box_scroll_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1089_4000 ^ seed as u64);
                scramble(c, &mut r);
                fill(c, &mut r, LIST_TEST_OBJECT, 8);
                c.g[A0][0] = sx(LIST_TEST_OBJECT);
            },
        );
    }

    pub(crate) const SEEDS_7: u32 = 200;

    /// Gates of the callee fakes: each answers only while its own word holds
    /// `OC1090_MAGIC` (otherwise the body it replaced runs). Placed above the
    /// pointer area so random fixture pointers never hit them.
    pub(crate) const OC1090_STATIC_CTRL: u32 = DATA + 0x0020_0000;

    pub(crate) const OC1090_SNDSTRM_CTRL: u32 = DATA + 0x0020_0010;

    pub(crate) const OC1090_MAGIC: u32 = 0x1090_c0de;

    pub(crate) static OC1090_PREV_STATIC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1090_PREV_SNDSTRM: AtomicUsize = AtomicUsize::new(0);

    /// `UIEngine_StaticInitDestroy`: logs the call and returns.
    pub(crate) fn oc1090_fake_static_init(c: &mut Cpu) {
        if c.bus.read32(OC1090_STATIC_CTRL) != OC1090_MAGIC {
            return fakes::fall_back(c, &OC1090_PREV_STATIC, ee::s13::f_003a3f48__generated);
        }
        fakes::fake_call(c, 0x003a_3f48);
    }

    /// `SNDSTRM_QueueRequest`: logs the call and returns.
    pub(crate) fn oc1090_fake_sndstrm_queue(c: &mut Cpu) {
        if c.bus.read32(OC1090_SNDSTRM_CTRL) != OC1090_MAGIC {
            return fakes::fall_back(c, &OC1090_PREV_SNDSTRM, ee::s13::f_003b76e8__generated);
        }
        fakes::fake_call(c, 0x003b_76e8);
    }

    pub(crate) fn oc1090_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A3F48),
            oc1090_fake_static_init,
            &OC1090_PREV_STATIC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B76E8),
            oc1090_fake_sndstrm_queue,
            &OC1090_PREV_SNDSTRM,
        );
    }

    #[test]
    fn ui_state_flags_ctor_matches() {
        check(
            ee::s13::f_003a65d8__generated,
            ui_state_flags_ctor,
            SEEDS_7,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0001, 0);
                let this = fakes::ptr(&mut r);
                c.g[A0][0] = this as u64;
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_text_flags_ctor_matches() {
        check(
            ee::s13::f_003a65e8__generated,
            ui_text_flags_ctor,
            SEEDS_7,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0002, 0);
                let attr = fakes::ptr(&mut r);
                c.g[A0][0] = attr as u64;
                c.bus.write8(attr, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_flags_ctor_matches() {
        check(
            ee::s13::f_003a6600__generated,
            c_ui_text_scroll_flags_ctor,
            SEEDS_7,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0003, 0);
                let attr = fakes::ptr(&mut r);
                c.g[A0][0] = attr as u64;
                c.bus.write8(attr, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn ui_vector_flags_ctor_matches() {
        check(
            ee::s13::f_003a6618__generated,
            ui_vector_flags_ctor,
            SEEDS_7,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0004, 0);
                let this = fakes::ptr(&mut r);
                c.g[A0][0] = this as u64;
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_engine_global_ctors_matches() {
        oc1090_install_fakes();
        check(
            ee::s13::f_003a6648__generated,
            ui_engine_global_ctors,
            SEEDS_7,
            &|c, seed| {
                fakes::machine(c, seed, 0x1090_0005, 1);
                c.bus.write32(OC1090_STATIC_CTRL, OC1090_MAGIC);
            },
        );
    }

    #[test]
    fn ui_engine_global_dtors_matches() {
        oc1090_install_fakes();
        check(
            ee::s13::f_003a6668__generated,
            ui_engine_global_dtors,
            SEEDS_7,
            &|c, seed| {
                fakes::machine(c, seed, 0x1090_0006, 1);
                c.bus.write32(OC1090_STATIC_CTRL, OC1090_MAGIC);
            },
        );
    }

    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;

    /// The distinct cycle totals the generated body takes over the seeds.
    pub(crate) fn oc1477_paths(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    // ------------------------------------------------------------------
    // The (index, value) short-pair loops: 0x3a0158, 0x3a0274, 0x3a3808.

    pub(crate) const OC1477_SHORTS: u32 = DATA + 0x1000;

    pub(crate) const OC1477_OBJ: u32 = DATA + 0x2000;

    /// A machine with the game image (the jump tables) and the pair array.
    pub(crate) fn oc1477_pairs_machine(c: &mut Cpu, seed: u32, salt: u64) -> crate::difftest::Rng {
        let mut r = fakes::machine(c, seed, salt, 6);
        elf_image(c);
        fill(c, &mut r, OC1477_SHORTS, 16);
        fill(c, &mut r, OC1477_OBJ, 64);
        r
    }

    /// `UIObject_AnimLoopTail`: a counter below and at the count, and the return
    /// entry 0x3a0284.
    pub(crate) fn oc1477_tail_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1477_pairs_machine(c, seed, 0x3a0274);
        for i in 0..4u32 {
            let idx = 0x11 + r.next_u32() % 0x20;
            c.bus.write16(OC1477_SHORTS + 4 * i, idx as u16);
        }
        c.g[A0][0] = OC1477_OBJ as u64;
        c.g[A1][0] = (seed % 5) as u64;
        c.g[A2][0] = OC1477_SHORTS as u64;
        c.g[T0][0] = (seed / 5 % 4) as u64;
        c.pc = if seed % 7 == 6 {
            0x003a_0284
        } else {
            0x003a_0274
        };
    }

    #[test]
    fn ui_object_anim_loop_tail_matches() {
        check(
            ee::s13::f_003a0274__generated,
            ui_object_anim_loop_tail,
            140,
            &oc1477_tail_setup,
        );
        assert!(
            oc1477_paths(ee::s13::f_003a0274__generated, 140, &oc1477_tail_setup) >= 3,
            "fixture reaches too few paths"
        );
    }

    /// `UIVector_SetAnimParams` fixture: the object's channel word (zero or
    /// not), the bound in `this+0x74`, indices below 21 and `21 + 9 q + r`,
    /// every resume entry.
    pub(crate) fn oc1477_vector_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1477_pairs_machine(c, seed, 0x3a3808);
        c.bus
            .write32(OC1477_OBJ + 0x80, if seed % 7 == 0 { 0 } else { 0x1234 });
        c.bus.write32(
            OC1477_OBJ + 0x74,
            (r.next_u32() % 5) << 17 | 0x1_ffff & r.next_u32(),
        );
        for i in 0..4u32 {
            let idx = if r.next_u32() % 6 == 0 {
                r.next_u32() % 21
            } else {
                21 + 9 * (r.next_u32() % 5) + r.next_u32() % 9
            };
            c.bus.write16(OC1477_SHORTS + 4 * i, idx as u16);
            c.bus
                .write16(OC1477_SHORTS + 4 * i + 2, r.next_u32() as u16);
        }
        c.g[A0][0] = OC1477_OBJ as u64;
        c.g[A1][0] = (seed / 8 % 5) as u64;
        c.g[A2][0] = OC1477_SHORTS as u64;
        c.g[T0][0] = OC1477_OBJ as u64;
        c.g[T2][0] = (seed / 40 % 3) as u64;
        let idx21 = 21 + 9 * (r.next_u32() % 5) as u64 + (r.next_u32() % 9) as u64;
        c.pc = match seed % 8 {
            0 => 0x003a_3808,
            1 => 0x003a_3818,
            2 => 0x003a_3820,
            3 => {
                c.g[A0][0] = idx21;
                0x003a_383c
            }
            4 => {
                c.g[V0][0] = 9;
                c.lo[0] = r.next_u32() as u64 % 6;
                c.hi[0] = r.next_u32() as u64 % 9;
                0x003a_3850
            }
            5 => {
                c.g[V1][0] = (seed / 8 % 2) as u64;
                c.g[T1][0] = (r.next_u32() % 9) as u64;
                0x003a_3870
            }
            6 => {
                c.g[V1][0] = (seed / 8 % 2) as u64;
                c.g[T1][0] = (r.next_u32() % 9) as u64;
                0x003a_3878
            }
            _ => {
                c.g[T1][0] = (r.next_u32() % 9) as u64;
                0x003a_3880
            }
        };
    }

    #[test]
    fn ui_vector_set_anim_params_matches() {
        check_lohi(
            ee::s13::f_003a3808__generated,
            ui_vector_set_anim_params,
            320,
            &oc1477_vector_setup,
        );
        assert!(
            oc1477_paths(ee::s13::f_003a3808__generated, 320, &oc1477_vector_setup) >= 8,
            "fixture reaches too few paths"
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
}
