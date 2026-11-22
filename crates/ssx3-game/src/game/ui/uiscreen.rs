//! `src/ui/uiscreen.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::cameracontroller::delete_owner;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot as vslot_2;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festatetrophyroom::sh_r;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::object::instancenode::ui_vector_lw;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::ui::uiengine::UI_LIST_FIRST;
pub(crate) use crate::game::ui::uiengine::UI_LIST_NODE_IS_SENTINEL;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_c558, ui_screen_dtor, "UIScreen_dtor"),
    (0x0039_c688, ui_screen_set_data, "UIScreen_SetData"),
    (0x0039_c728, uiscreen_find_keyframe, "UIScreen_FindKeyframe"),
    (
        0x0039_c7c0,
        ui_screen_get_primary_thread,
        "UIScreen_GetPrimaryThread",
    ),
    (
        0x0039_c978,
        ui_screen_run_thread_frame,
        "UIScreen_RunThreadFrame",
    ),
    (
        0x0039_cbc8,
        uiscreen_cmd_stop_thread,
        "UIScreen_CmdStopThread",
    ),
    (
        0x0039_cbe0,
        ui_screen_cmd_goto_label,
        "UIScreen_CmdGotoLabel",
    ),
    (
        0x0039_cc38,
        ui_screen_cmd_start_thread,
        "UIScreen_CmdStartThread",
    ),
    (0x0039_cca8, ui_screen_cmd_play_anim, "UIScreen_CmdPlayAnim"),
    (
        0x0039_cd30,
        ui_screen_cmd_set_object_param,
        "UIScreen_CmdSetObjectParam",
    ),
    (
        0x0039_cda0,
        uiscreen_cmd_send_event,
        "UIScreen_CmdSendEvent",
    ),
    (
        0x0039_ce20,
        uiscreen_cmd_end_transition_in,
        "UIScreen_CmdEndTransitionIn",
    ),
    (
        0x0039_ce48,
        ui_screen_cmd_begin_transition_out,
        "UIScreen_CmdBeginTransitionOut",
    ),
    (
        0x0039_ce98,
        uiscreen_cmd_end_transition_out,
        "UIScreen_CmdEndTransitionOut",
    ),
    (0x0039_d778, ui_screen_update, "UIScreen_Update"),
    (0x0039_d860, ui_screen_find_object, "UIScreen_FindObject"),
    (0x0039_d8c0, c_ui_scroll_dtor, "cUIScroll_dtor"),
    (0x0039_d968, cui_scroll_render, "cUIScroll_Render"),
    (
        0x0039_de68,
        ui_scroll_set_parts_visible,
        "cUIScroll_SetPartsVisible",
    ),
    (0x0039_dfb0, cui_slider_load, "cUISlider_Load"),
    (0x0039_e2a0, ui_state_ctor, "UIState_ctor"),
    (0x0039_e318, ui_state_ctor_2, "UIState_ctor_2"),
    (0x0039_e390, ui_state_dtor, "UIState_dtor"),
    (0x0039_e688, ui_state_v13, "UIState_v13"),
    (0x0039_e6b8, ui_state_render, "UIState_Render"),
    (0x0039_e758, ui_state_add_screen, "UIState_AddScreen"),
    (0x0039_fcc8, ui_object_set_anim, "UIObject_SetAnim"),
    (0x0039_fd38, ui_object_clear_anim, "UIObject_ClearAnim"),
    (0x003a_4868, ui_list_node_dtor_body, "UIListNode_dtorBody"),
    (0x003a_5338, ui_menu_flags_ctor, "UIMenuFlags_ctor"),
    (0x003a_5508, ui_pair_flags_ctor, "UIPairFlags_ctor"),
    (
        0x0039_d078,
        ui_screen_create_object,
        "UIScreen_CreateObject",
    ),
    (0x0039_e510, ui_state_update, "UIState_Update"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_OBJECT_LOAD;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL as UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::ui::uistate::ui_callee::NODE_UNLINK as UI_LIST_NODE_UNLINK_2;

    // UI (shard 13).
    pub const C_UI_GROUP_SET_FOCUS: Callee = Callee {
        addr: 0x0039_8638,
        f: ee::s13::f_00398638,
    };

    pub const UI_SCREEN_GET_PRIMARY_THREAD: Callee = Callee {
        addr: 0x0039_c7c0,
        f: ee::s13::f_0039c7c0,
    };

    pub const UI_SCREEN_RUN_THREAD_FRAME: Callee = Callee {
        addr: 0x0039_c978,
        f: ee::s13::f_0039c978,
    };

    pub const C_UI_GROUP_UPDATE_ANIM: Callee = Callee {
        addr: 0x0039_8910,
        f: ee::s13::f_00398910,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;

    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;

    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
}

// ---------------------------------------------------------------------------
// UI screen commands

/// `void UIScreen_CmdStopThread(void *ctx)` (0x39cbc8): clear the "thread
/// running" bit (bit 16) of the flag word at `a3 + 0x10`.
pub fn uiscreen_cmd_stop_thread(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x10).lw(c);
    c.g[V0][0] = 0xffff_ffff_feff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    Addr(c.g[A3][0] as u32).off(0x10).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void UIScreen_CmdEndTransitionIn(void *ctx)` (0x39ce20): set the transition
/// state bits (0x300) of the screen's flag word when the screen exists.
pub fn uiscreen_cmd_end_transition_in(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0xd0).lw(c);
    let present = c.g[A0][0] != 0;
    c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
    if present {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x300;
        Addr(c.g[A0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void UIScreen_CmdEndTransitionOut(void *ctx)` (0x39ce98): clear the thread
/// bit of `a3 + 0x10` and set state bits 0x700 on the screen's flag word.
pub fn uiscreen_cmd_end_transition_out(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x10).lw(c);
    c.g[V0][0] = 0xffff_ffff_feff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    Addr(c.g[A3][0] as u32).off(0x10).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0xd0).lw(c);
    let present = c.g[A0][0] != 0;
    c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
    if present {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x700;
        Addr(c.g[A0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `UIMenuFlags* UIMenuFlags_ctor(UIMenuFlags* f)` (0x3a5338): clears the low
/// nibble of the flag byte and returns the object.
pub fn ui_menu_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = c.lbu(selfp);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] &= 0xf0;
    c.sb(selfp, c.g[V1][0]);
    abi::ret(c);
}

/// `void UIState_AddScreen(this, cListNode *node)` (0x0039e758): push the
/// node onto the screen list at `+0x24`.
pub fn ui_state_add_screen(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24)),
        callee::UI_LIST_PUSH_BACK,
        0x0039_e768,
    )
}

/// `void UIListNode_dtorBody(cListNode *node)` (0x003a4868): unlink the node.
pub fn ui_list_node_dtor_body(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::UI_LIST_NODE_UNLINK, 0x003a_4878)
}

/// `UIPairFlags* UIPairFlags_ctor(UIPairFlags* f)` (0x3a5508): mask the two
/// pair-flag bytes (`+0` and `+1`) down to their top two bits and return the
/// object.
pub fn ui_pair_flags_ctor(c: &mut Cpu) {
    c.cycles += 8;
    let flags = r32(c, A0);
    c.g[A1][0] = Addr(flags.wrapping_add(1)).lbu(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = Addr(flags).lbu(c);
    c.g[A1][0] = c.g[A1][0] & 0xc0;
    c.g[V1][0] = c.g[V1][0] & 0xc0;
    Addr(flags.wrapping_add(1)).sb(c, c.g[A1][0]);
    Addr(flags).sb(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UIState_v13.

/// `void UIState_v13(UIState* s, UIState* target)` (0x39e688): `target`'s
/// seventh virtual method called with 1 (`s`'s `+8` sub-object supplies the
/// vtable).
pub fn ui_state_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x8)).lw(c);
        let slot = VSlot::read(c, c.g[V0][0], 0x38);
        c.g[V1][0] = slot.delta;
        c.g[A2][0] = slot.func;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, 0x0039_e6ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUISlider_Load(cUISlider* self)` (0x39dfb0): same as the progress
/// bar, with the slider's range fields at `+0x78`/`+0x80`.
pub fn cui_slider_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_LOAD.call(c, 0x0039_dfc4)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = 0x64;
        let s0 = c.g[S0][0];
        let v0 = c.g[V0][0];
        let v1 = c.g[V1][0];
        Addr(s0 as u32).off(0x80).sw(c, v0);
        Addr(s0 as u32).off(0x78).sw(c, v1);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIObject_ClearAnim(UIObject* o)` (0x39fd38): detach (`this, 3`) and
/// drop the animation object at `+0xc`.
pub fn ui_object_clear_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0xc));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(8));
            c.g[V1][0] = c.lw(vt.wrapping_add(0xc));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0039_fd68)?;
            c.sw(r32(c, S0).wrapping_add(0xc), 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIScreen_CmdBeginTransitionOut(screen)` (0x39ce48): clear the
/// screen's focus and set the "transition out" mode (6) on its owner
/// (`+0xd0`), if any.
pub fn ui_screen_cmd_begin_transition_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        C_UI_GROUP_SET_FOCUS.call(c, 0x0039_ce64)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xd0)).lw(c);
        let owner = c.g[A0][0] as u32;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        if owner != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(owner).off(0x1c).lw(c);
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x600;
            Addr(owner).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIScreen_RunThreadFrame(UIScreen* s, UIThread* t)` (0x39c978):
/// one screen-script frame of a thread (called by `UIScreen_Update` while
/// the thread's frame `+0xc` differs from its last run `+0xe`): step the
/// frame by `+0x12` towards the end frame `+0x10`, then run the commands of
/// the keyframe at the old frame.
///
/// Screen scripts are authored at 60 fps: above 60 Hz each thread runs one
/// frame per `rate / 60` ticks of its own (`tick::frame60_due`; its first
/// frame on the tick it starts, as at 60), so transitions and screen
/// animations keep their wall-clock speed. The screen update itself (the
/// widgets' layout, `cUIGroup_UpdateAnim` at the held frame) still runs
/// every tick. At 60 every call runs the original.
pub fn ui_screen_run_thread_frame(c: &mut Cpu) {
    let thread = r32(c, A1);
    let frame = c.bus.read16(thread.wrapping_add(0xc)) as u32;
    if !crate::tick::frame60_due(c, thread, frame) {
        abi::ret(c);
        return;
    }
    ee::s13::f_0039c978__generated(c);
    let frame = c.bus.read16(thread.wrapping_add(0xc)) as u32;
    crate::tick::frame60_ran(c, thread, frame);
}

/// Resumed in the thread walk (the `UIScreen_RunThreadFrame` back edge).
pub(crate) const UI_SCREEN_UPDATE_WALK: u32 = 0x0039_d7b0;

/// `void UIScreen_Update(UIScreen* s)` (0x39d778): run the screen's threads
/// (`UIScreen_RunThreadFrame`), then its widgets' animations
/// (`cUIGroup_UpdateAnim`) at the frame of its timeline thread.
pub fn ui_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0039_d778 => {
                    c.cycles += 11;
                    // The widgets' animations find the timeline that
                    // clocks them through the screen being updated.
                    set_updating(c, Some(r32(c, A0)));
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, S2, 0x10);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S0, 0x30);
                    frame.save(c, S1, 0x20);
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0039_d798);
                    c.g[A0][0] = sx(p(r32(c, S2), 0x18));
                    callee::UI_LIST_FIRST_2.call(c, 0x0039_d798)?;
                    c.g[S0][0] = c.g[V0][0];
                    let empty = c.g[S0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    pc = if empty { 0x0039_d814 } else { 0x0039_d7a4 };
                    continue;
                }
                0x0039_d7a4 => {
                    c.cycles += 2;
                    pc = 0x0039_d804;
                    continue;
                }
                UI_SCREEN_UPDATE_WALK => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lhu(p(r32(c, S0), 0xe));
                    c.g[V0][0] = c.lhu(p(r32(c, S0), 0xc));
                    let finished = c.g[V1][0] == c.g[V0][0];
                    c.g[S1][0] = c.lw(p(r32(c, S0), 0x4));
                    if finished {
                        pc = 0x0039_d800;
                    } else {
                        pc = 0x0039_d7c0;
                    }
                    continue;
                }
                0x0039_d7c0 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    callee::UI_SCREEN_RUN_THREAD_FRAME.call(c, 0x0039_d7cc)?;
                    c.g[V0][0] = c.lb(p(r32(c, S0), 0x13)) & 0x1;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[S1][0];
                        pc = 0x0039_d804;
                    } else {
                        pc = 0x0039_d7dc;
                    }
                    continue;
                }
                0x0039_d7dc => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_UNLINK_2.call(c, 0x0039_d7e4)?;
                    let gone = c.g[S0][0] == 0;
                    c.g[A1][0] = 0x3;
                    pc = if gone { 0x0039_d800 } else { 0x0039_d7ec };
                    continue;
                }
                0x0039_d7ec => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(p(r32(c, S0), 0x8));
                    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x8));
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0xc));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0039_d800)?;
                    pc = 0x0039_d800;
                    continue;
                }
                0x0039_d800 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S1][0];
                    pc = 0x0039_d804;
                    continue;
                }
                0x0039_d804 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL_2.call(c, 0x0039_d80c)?;
                    let more = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if more {
                        if back_edge_unwinds(c, UI_SCREEN_UPDATE_WALK) {
                            return Err(Unwound);
                        }
                        pc = UI_SCREEN_UPDATE_WALK;
                    } else {
                        pc = 0x0039_d814;
                    }
                    continue;
                }
                0x0039_d814 => {
                    c.cycles += 4;
                    c.g[S0][0] = 0xffff;
                    callee::UI_SCREEN_GET_PRIMARY_THREAD.call(c, 0x0039_d81c)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xc));
                        pc = 0x0039_d838;
                    } else {
                        pc = 0x0039_d824;
                    }
                    continue;
                }
                0x0039_d824 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx(p(r32(c, S2), 0x18));
                    callee::UI_LIST_FIRST_2.call(c, 0x0039_d82c)?;
                    let empty = c.g[V0][0] == 0;
                    c.g[A0][0] = sx(p(r32(c, S2), 0x40));
                    pc = if empty { 0x0039_d83c } else { 0x0039_d834 };
                    continue;
                }
                0x0039_d834 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xc));
                    pc = 0x0039_d838;
                    continue;
                }
                0x0039_d838 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx(p(r32(c, S2), 0x40));
                    pc = 0x0039_d83c;
                    continue;
                }
                0x0039_d83c => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S0][0];
                    // (Again: a resumed walk may come back in a later frame.)
                    set_updating(c, Some(r32(c, S2)));
                    callee::C_UI_GROUP_UPDATE_ANIM.call(c, 0x0039_d844)?;
                    set_updating(c, None);
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, S0, 0x30);
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_d778, 0x0039_d85c);
                    return Ok(());
                }
            }
        }
    })
}

thread_local! {
    /// The screen whose `UIScreen_Update` is running, with the VSync count
    /// (`tick::vsync_count`) it was set at.
    static UPDATING: std::cell::Cell<Option<(u32, u32)>> = const { std::cell::Cell::new(None) };
}

/// Set (or clear) the screen whose `UIScreen_Update` is running.
fn set_updating(c: &mut Cpu, screen: Option<u32>) {
    let vsync = crate::tick::vsync_count(c);
    UPDATING.set(screen.map(|s| (s, vsync)));
}

/// The screen whose `UIScreen_Update` is running.
///
/// One slot, not a stack: an update that an event unwinds is not re-entered
/// here (the trampoline resumes it in the generated body), so nothing clears
/// its screen. In the unwind form (the browser build) the VBlank that
/// finishes a frame unwinds the thread, so there the screen counts only
/// until the VSync count moves. Host threads block at a VBlank and come back
/// to the update instead; only a call-depth unwind leaves the slot set, until
/// the next update replaces it.
pub(crate) fn updating_screen(c: &mut Cpu) -> Option<u32> {
    let vsync = crate::tick::vsync_count(c);
    slot_screen(UPDATING.get(), false, vsync)
}

/// [`updating_screen`]'s rule for a slot, in the unwind form or with host
/// threads, at the VSync count `vsync`.
fn slot_screen(slot: Option<(u32, u32)>, unwind_form: bool, vsync: u32) -> Option<u32> {
    let (screen, at) = slot?;
    (!unwind_form || at == vsync).then_some(screen)
}

/// Run `body` as if inside `UIScreen_Update(screen)` (tests).
#[cfg(test)]
pub(crate) fn with_updating_screen(c: &mut Cpu, screen: u32, body: impl FnOnce(&mut Cpu)) {
    set_updating(c, Some(screen));
    body(c);
    set_updating(c, None);
}

/// The thread whose frame `UIScreen_Update` passes to the widgets'
/// animations: `UIScreen_GetPrimaryThread` (the first thread of the list at
/// `+0x18` with bit 25 of `+0x10` set), else the list's first thread; 0 when
/// there is none. Reads only.
pub(crate) fn timeline_thread(c: &mut Cpu, screen: u32) -> u32 {
    // `UIList_First` (0x397840): 0 when the first node links to itself.
    let list = screen.wrapping_add(0x18);
    let first = c.bus.read32(list.wrapping_add(4));
    if first == 0 || c.bus.read32(first.wrapping_add(4)) == first {
        return 0;
    }
    // `UIListNode_IsSentinel` (0x397718): a node that links to itself.
    let mut node = first;
    for _ in 0..256 {
        if node == 0 || c.bus.read32(node.wrapping_add(4)) == node || c.bus.read32(node) == node {
            break;
        }
        if (c.bus.read32(node.wrapping_add(0x10)) >> 25) & 1 != 0 {
            return node;
        }
        node = c.bus.read32(node.wrapping_add(4));
    }
    first
}

/// `UIKeyframe* UIScreen_FindKeyframe(UIScreen* s, unsigned short frame)`
/// (0x39c728): walk the variable-length keyframe list at `s+0x34` until an
/// entry's `+8` matches `frame`; null when the list ends. 6 (+2, then
/// 3 (+4) per miss, 2 on a hit) + 2 cycles.
pub fn uiscreen_find_keyframe(c: &mut Cpu) {
    c.cycles += 6;
    let list = c.lw(p(r32(c, A0), 0x34)) as u32;
    c.g[A1][0] = c.g[A1][0] & 0xffff;
    c.g[A2][0] = 0;
    let count = c.lw(list);
    c.g[V1][0] = count;
    let mut entry = p(list, 4);
    c.g[A0][0] = sx(entry);
    if count != 0 {
        c.cycles += 2;
        c.g[A3][0] = count;
        loop {
            c.cycles += 3;
            let frame = Addr(p(entry, 8)).lhu(c);
            c.g[V0][0] = frame;
            if frame != c.g[A1][0] {
                let stride = c.lw(p(entry, 4));
                c.g[V1][0] = stride;
                c.cycles += 4;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = (c.g[A2][0] < c.g[A3][0]) as u64;
                entry = p(entry, stride as u32);
                c.g[A0][0] = sx(entry);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_c748);
                    }
                    continue;
                }
                break;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(entry);
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI widgets.

/// `void UIScreen_CmdGotoLabel(UIScreen* s, void* key, void* out)` (0x39cbe0):
/// walk the screen's label list (count at `+0`, entries `{label, stride,
/// value}` at `+4`) and store the matching entry's `u16` at `out+0xc`.
pub fn ui_screen_cmd_goto_label(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T1][0] = c.g[A0][0];
    c.g[T0][0] = 0;
    c.g[V1][0] = Addr(p_2(c, T1, 0x34)).lw(c);
    c.g[V0][0] = Addr(p_2(c, V1, 0)).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
    if !empty {
        loop {
            c.cycles += 4;
            c.g[V1][0] = Addr(p_2(c, A1, 0)).lw(c);
            c.g[V0][0] = Addr(p_2(c, A2, 4)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = Addr(p_2(c, T1, 0x34)).lw(c);
                c.cycles += 6;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V1][0] = Addr(p_2(c, A1, 4)).lw(c);
                c.g[V0][0] = Addr(p_2(c, A0, 0)).lw(c);
                c.g[V0][0] = (c.g[T0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cbf8);
                }
                continue;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_2(c, A1, 8)).lhu(c);
            c.sh(p_2(c, A3, 0xc), c.g[V0][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI screens and groups.

/// `void* UIScreen_FindObject(UIScreen* self, uint hash)` (0x39d860): walk the
/// screen's object array and return the first live object whose hash matches.
pub fn ui_screen_find_object(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = lw(c, p_3(c.g[A0][0], 0x3c));
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = lw(c, p_3(c.g[A0][0], 0x38));
        c.g[V0][0] = lw(c, c.g[V0][0] as u32);
        c.g[A3][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[V1][0];
            loop {
                c.cycles += 3;
                c.g[V1][0] = lw(c, c.g[A2][0] as u32);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                let more;
                if c.g[V1][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
                    more = c.g[V0][0] != 0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p_3(c.g[V1][0], 0x38));
                    if c.g[V0][0] != c.g[A1][0] {
                        c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
                        more = c.g[V0][0] != 0;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[V1][0];
                        abi::ret(c);
                        return;
                    }
                }
                c.cycles += 2;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_d888);
                    }
                    continue;
                }
                break;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI screens.

/// `UIThread* UIScreen_GetPrimaryThread(UIScreen* screen)` (0x39c7c0): walk
/// the screen's UI list and return the first non-sentinel node that is not
/// hidden (bit 25 of its flags clear), or null.
pub fn ui_screen_get_primary_thread(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0039_c7c0 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_7840, 0x0039_c7d4)?;
                    c.g[S0][0] = c.g[V0][0];
                    let none = c.g[S0][0] == 0;
                    c.g[V0][0] = 0;
                    if none {
                        pc = 0x0039_c810;
                        continue;
                    }
                    pc = 0x0039_c7e0;
                }
                0x0039_c7e0 => {
                    c.cycles += 2;
                    pc = 0x0039_c800;
                }
                0x0039_c7e8 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(at(c, S0, 0x10)).lw(c);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 25) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let hidden = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0];
                    if hidden {
                        pc = 0x0039_c810;
                        continue;
                    }
                    pc = 0x0039_c7fc;
                }
                0x0039_c7fc => {
                    c.cycles += 1;
                    c.g[S0][0] = Addr(at(c, S0, 0x4)).lw(c);
                    pc = 0x0039_c800;
                }
                0x0039_c800 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_7718, 0x0039_c808)?;
                    let sentinel = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if sentinel {
                        if back_edge_unwinds(c, 0x0039_c7e8) {
                            return Ok(());
                        }
                        pc = 0x0039_c7e8;
                        continue;
                    }
                    pc = 0x0039_c810;
                }
                0x0039_c810 => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = Addr(r32_2(c, SP)).ld(c);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_c7c0, 0x0039_c820);
                    return Ok(());
                }
            }
        }
    })
}

/// `void UIScreen_CmdSetObjectParam(UIScreen* screen, int a1, void* cmd)`
/// (0x39cd30): find the named object and apply the parameter command to it.
pub fn ui_screen_cmd_set_object_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let cmd = c.g[A2][0];
        c.g[S1][0] = cmd;
        c.g[A1][0] = Addr(at(c, S1, 0x4)).lw(c);
        call_ptr(c, 0x0039_d860, 0x0039_cd4c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_fd38, 0x0039_cd60)?;
            c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
            c.g[A2][0] = sx(at(c, S1, 0xc));
            c.g[A1][0] = Addr(at(c, S1, 0x8)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x68).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x6c).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, target, 0x0039_cd7c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIScreen_CmdStartThread(UIScreen* self, void* p2)` (0x39cc38): walk
/// the screen's frame-table entries and play the frame whose label matches
/// `*(p2 + 4)`.
pub fn ui_screen_cmd_start_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(at_2(c, T0, 0x34)).lw(c);
        c.g[V1][0] = Addr(at_2(c, V0, 0)).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[A1][0] = sx(at_2(c, V0, 4));
        if !empty {
            loop {
                c.cycles += 4;
                c.g[V1][0] = Addr(at_2(c, A1, 0)).lw(c);
                c.g[V0][0] = Addr(at_2(c, A2, 4)).lw(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = Addr(at_2(c, T0, 0x34)).lw(c);
                    break;
                }
                c.cycles += 6;
                c.g[A1][0] = Addr(at_2(c, A1, 8)).lhu(c);
                c.g[A0][0] = c.g[T0][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0039_cc78)?;
                c.g[RA][0] = Addr(at_2(c, SP, 0)).ld(c);
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                c.pc = ra;
                return Ok(());
            }
            loop {
                c.cycles += 6;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V1][0] = Addr(at_2(c, A1, 4)).lw(c);
                c.g[V0][0] = Addr(at_2(c, A0, 0)).lw(c);
                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(at_2(c, A1, 0).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cc58);
                }
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(at_2(c, A1, 0)).lw(c);
                    c.g[V0][0] = Addr(at_2(c, A2, 4)).lw(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = Addr(at_2(c, T0, 0x34)).lw(c);
                        break;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = Addr(at_2(c, A1, 8)).lhu(c);
                    c.g[A0][0] = c.g[T0][0];
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x0039_c870, 0x0039_cc78)?;
                    c.g[RA][0] = Addr(at_2(c, SP, 0)).ld(c);
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
            }
        }
        c.cycles += 1;
        c.g[RA][0] = Addr(at_2(c, SP, 0)).ld(c);
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `void UIObject_SetAnim(UIObject* self, void* anim, u8 mode, u16 flags)`
/// (0x39fcc8): clear the old animation, then set the new one's play mode.
pub fn ui_object_set_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0] & 0xff;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(c, S0, 0xc)).lw(c);
        let same = c.g[V0][0] == c.g[S1][0];
        c.g[S3][0] = c.g[A3][0] & 0xffff;
        if !same {
            c.cycles += 2;
            call_ptr(c, 0x0039_fd38, 0x0039_fd00)?;
        }
        c.cycles += 13;
        Addr(at_2(c, S0, 0x10)).sb(c, c.g[S2][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[S1][0]);
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0] & 0xff;
        call_ptr(c, 0x0039_7468, 0x0039_fd18)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIState_ctor` (0x39E2A0): the base constructor of every UI state object
/// (list links, vtable, engine, id byte and the widget list at +0x24).
pub fn ui_state_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0xf;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xffff_ffff_ffff_ffff;
        c.sb(r32(c, S0).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = 0x218;
        sw_at(c, S0, 0x4, c.g[S0][0]);
        c.g[V0][0] = low(c, V0).wrapping_add(0x46c8);
        sw_at(c, S0, 0, c.g[S0][0]);
        c.g[A0][0] = low(c, S0).wrapping_add(0x24);
        sw_at(c, S0, 0xc, 0);
        sw_at(c, S0, 0x20, 0);
        sw_at(c, S0, 0x10, c.g[A1][0]);
        c.sb(r32(c, S0).wrapping_add(0x15), c.g[A2][0]);
        c.sh(r32(c, S0).wrapping_add(0x1c), c.g[V1][0]);
        sw_at(c, S0, 0x8, c.g[V0][0]);
        callee::UI_LIST_CTOR.call(c, 0x0039_e2f4)?;
        c.sb(r32(c, S0).wrapping_add(0x44), c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw_at(c, S0, 0x40, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Constructors.

/// `func_0039E318` (0x39e318): the second half of a UI state's constructor.
/// Installs vtable `0x4946c8` at `+8`, the id byte `0x0f` at `+0x15`, clears
/// the flag words at `+0xc`/`+0x40` and constructs the widget list at `+0x24`
/// (`UIList_ctor`). Signature: `void ctor(this, owner, engine)`.
pub fn ui_state_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0xf;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX;
        sb_r(c, S0, 0x14, c.g[S1][0]);
        c.g[V1][0] = 0x218;
        sw_r(c, S0, 0x4, c.g[S0][0]);
        c.g[V0][0] = sx(low_2(c, V0).wrapping_add(0x46c8));
        sw_r(c, S0, 0x0, c.g[S0][0]);
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0x24));
        sw_r(c, S0, 0xc, 0);
        sw_r(c, S0, 0x10, c.g[A1][0]);
        sb_r(c, S0, 0x15, c.g[A3][0]);
        sh_r(c, S0, 0x1c, c.g[V1][0]);
        sw_r(c, S0, 0x20, c.g[A2][0]);
        c.g[RA][0] = sx(0x0039_e36c);
        sw_r(c, S0, 0x8, c.g[V0][0]);
        callee::UI_LIST_CTOR_2.call(c, 0x0039_e36c)?;
        sb_r(c, S0, 0x44, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw_r(c, S0, 0x40, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI screens.

/// `void UIScreen_CmdSendEvent(self, msg, cmd)` (0x39cda0): dispatch a
/// command through the screen's controller (`self+0xd0`), falling back to the
/// controller's child when the first handler declines.
pub fn uiscreen_cmd_send_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = c.g[S0][0];
        let ctrl = lw_at(c, p(r32(c, S0), 0xd0));
        c.g[A3][0] = ctrl;
        c.g[A2][0] = lw_at(c, p(r32(c, S1), 4));
        c.g[V1][0] = lw_at(c, p(ctrl as u32, 8));
        let func = vslot(c, c.g[V1][0], 0x80, V0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0039_cdd8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, p(r32(c, S0), 0xd0));
            c.g[V1][0] = lw_at(c, p(c.g[V0][0] as u32, 0x10));
            c.g[A3][0] = lw_at(c, c.g[V1][0] as u32);
            let none = c.g[A3][0] == 0;
            c.g[A1][0] = c.g[S0][0];
            if none {
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            } else {
                c.cycles += 6;
                c.g[V1][0] = lw_at(c, p(c.g[A3][0] as u32, 4));
                c.g[A2][0] = lw_at(c, p(r32(c, S1), 4));
                let func = vslot(c, c.g[V1][0], 0x18, V0);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0039_ce0c)?;
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI commands.

/// `void UIScreen_CmdPlayAnim(UIScreen* self, int, UIMsg* msg)` (0x39cca8):
/// play the animation named by `msg[+4]` on the object named by `msg[+8]`.
pub fn ui_screen_cmd_play_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0039_ccd4);
        c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x0039_ccd4)?;
        c.g[S2][0] = c.g[V0][0];
        if c.g[S2][0] == 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r32(c, S1), 0xd0)).lw(c);
            c.g[A1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lw(c);
            c.g[RA][0] = sx(0x0039_ccf4);
            c.g[A0][0] = sx(p(r32(c, A0), 0x50));
            callee::UI_ANIM_SET_CREATE_ANIM.call(c, 0x0039_ccf4)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S2][0];
            if !missing {
                c.cycles += 5;
                c.g[A2][0] = Addr(p(r32(c, S0), 0xc)).lbu(c);
                c.g[A3][0] = Addr(p(r32(c, S3), 8)).lhu(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0039_cd10);
                c.g[T0][0] = 0;
                callee::UI_OBJECT_SET_ANIM.call(c, 0x0039_cd10)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
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

/// `void cUIScroll_dtor(cUIScroll* self, int flags)` (0x39d8c0): release the
/// two child objects and run the base destructor.
pub fn c_ui_scroll_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_44c8);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x74)).lw(c);
        let first_null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !first_null {
            c.cycles += 6;
            c.g[A1][0] = 0x3;
            delete_owner(c, 8, 0x0039_d904)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x78)).lw(c);
        let second_null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !second_null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_d924)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_d930);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_d930)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUISlider_dtor(cUISlider* self, int flags)` (0x39df28): same shape as
/// [`c_ui_scroll_dtor`] with the slider's child slots at +0x88 and +0x8c.
pub fn c_ui_slider_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0049_4348);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x88)).lw(c);
        let first_null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !first_null {
            c.cycles += 6;
            delete_owner(c, 8, 0x0039_df6c)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0x8c)).lw(c);
        let second_null = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !second_null {
            c.cycles += 5;
            delete_owner(c, 8, 0x0039_df8c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039_df98);
        c.g[A1][0] = c.g[S1][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039_df98)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI states.

/// `void UIScreen_SetData(self, data)` (0x39c688): replace the screen's data
/// block with an unpacked copy (or clear the unpacked flag).
pub fn ui_screen_set_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, table::EAC_GET_UNPACKED_SIZE, 0x0039_c6a8)?;
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = 0x0049_0000;
        if !none {
            c.cycles += 14;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x0049_3e00;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, table::OPERATOR_NEW_TAG, 0x0039_c6c4)?;
            c.g[A0][0] = c.g[S0][0];
            sw(c, S1, 0xc, c.g[V0][0]);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, table::EAC_UNPACK, 0x0039_c6d8)?;
            c.g[V1][0] = lw_2(c, S1, 0x14);
            c.g[V1][0] |= 2;
            sw(c, S1, 0x14, c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[V0][0] = lw_2(c, S1, 0x14);
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            c.g[V0][0] &= c.g[V1][0];
            sw(c, S1, 0x14, c.g[V0][0]);
        }
        c.cycles += 12;
        sw(c, S1, 0x10, c.g[S0][0]);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        sw(c, S1, 0x34, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        sw(c, S1, 0x38, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void UIState_Render(self)` (0x39e6b8): prepare the list and render each
/// visible node.
pub fn ui_state_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(this.wrapping_add(0x24));
        frame.save_fpr(c, 20, 0x30);
        call_ptr(c, table::UI_LIST_FIRST, 0x0039_e6d8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 7;
            c.g[V0][0] = lw_2(c, S1, 0x8);
            c.mtc1(0, 20);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x70).lh(c);
            c.g[V1][0] = lw_2(c, V0, 0x74);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_e6fc)?;
            c.g[V0][0] = lw_2(c, S0, 0x54);
            'nodes: loop {
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.g[V0][0] &= 1;
                let hidden = c.g[V0][0] == 0;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                if !hidden {
                    c.cycles += 7;
                    c.g[V0][0] = lw_2(c, S0, 0x48);
                    c.mov_s(12, 20);
                    c.mov_s(13, 20);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
                    c.g[V1][0] = lw_2(c, V0, 0x8c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0039_e72c)?;
                }
                c.cycles += 5;
                c.g[S0][0] = lw_2(c, S0, 0x4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_LIST_NODE_IS_SENTINEL, 0x0039_e738)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw_2(c, S0, 0x54);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_e700);
                    }
                    continue 'nodes;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'nodes;
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void UIState_dtor(UIState* self, int flags)` (0x39e390): run the list and
/// node destructors, then free the object when `flags & 1`.
pub fn ui_state_dtor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'end: {
            c.cycles += 33;
            c.g[V0][0] = sx(0x0049_0000);
            frame.save(c, S3, 0x10);
            c.g[V0][0] = sx(0x0049_46c8);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S1, 0x30);
            frame.save(c, S0, 0x40);
            c.g[S1][0] = sx(at_4(c.g[S3][0], 0x24));
            frame.save(c, S2, 0x20);
            c.g[A0][0] = c.g[S1][0];
            frame.save_ra(c, 0);
            c.g[S2][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x0039_e3c8);
            Addr(at_4(c.g[S3][0], 8)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0039_7948, 0x0039_e3c8)?;
            c.g[V0][0] = sx(0x0049_0000);
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = sx(0x0049_4c98);
            c.g[RA][0] = sx(0x0039_e3dc);
            Addr(at_4(c.g[S3][0], 0x3c)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0039_7948, 0x0039_e3dc)?;
            c.g[S0][0] = sx(0x0049_0000);
            c.g[A0][0] = sx(at_4(c.g[S3][0], 0x30));
            c.g[S0][0] = sx(0x0049_4cc0);
            c.g[RA][0] = sx(0x0039_e3f0);
            Addr(at_4(c.g[S3][0], 0x38)).sw(c, c.g[S0][0]);
            call_ptr(c, 0x003a_4868, 0x0039_e3f0)?;
            Addr(at_4(c.g[S3][0], 0x2c)).sw(c, c.g[S0][0]);
            c.g[RA][0] = sx(0x0039_e3fc);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_4868, 0x0039_e3fc)?;
            Addr(at_4(c.g[S3][0], 8)).sw(c, c.g[S0][0]);
            c.g[RA][0] = sx(0x0039_e408);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x003a_4868, 0x0039_e408)?;
            c.g[S2][0] &= 1;
            let t = c.g[S2][0] == 0;
            frame.restore(c, S0, 0x40);
            if t {
                break 'end;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_e41c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_7e50, 0x0039_e41c)?;
            frame.restore(c, S0, 0x40);
            break 'end;
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

/// `void cUIScroll_Render(self)` (0x39d968): render the scroll thumb's
/// decoration (f12/f13 carry the caller's screen-space offsets).
pub fn cui_scroll_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.swc1(21, p(c.g[SP][0] as u32, 0x28));
        c.swc1(20, p(c.g[SP][0] as u32, 0x20));
        c.g[A1][0] = Addr(p(this as u32, 0x74)).lw(c);
        let no_thumb = c.g[A1][0] == 0;
        if !no_thumb {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(this as u32, 0x78)).lw(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 5;
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 21, 0x28);
                frame.restore_fpr(c, 20, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x14)).lw(c);
            c.lwc1(0, p(this as u32, 0x44));
            c.lwc1(1, p(this as u32, 0x48));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.add_s(21, 12, 0);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let skip = c.g[V0][0] == 0;
            c.add_s(20, 13, 1);
            if !skip {
                c.cycles += 7;
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
                c.mov_s(12, 21);
                c.mov_s(13, 20);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x88)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x8c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_d9d4)?;
            }
            c.cycles += 6;
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x78)).lw(c);
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0x14)).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let skip2 = c.g[V0][0] == 0;
            c.mov_s(12, 21);
            if !skip2 {
                c.cycles += 6;
                c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 8)).lw(c);
                c.mov_s(13, 20);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x88)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x8c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_da04)?;
            }
            c.cycles += 1;
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

// ---------------------------------------------------------------------------
// `cUIScroll_SetPartsVisible` (0x0039de68).
//
// `void cUIScroll::SetPartsVisible(self, bool visible)`: forward the
// visibility flag to the scroll bar, track and thumb objects.

pub fn ui_scroll_set_parts_visible(c: &mut Cpu) {
    guest(c, |c| {
        'b_39de9c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
            let t = c.g[A2][0] == 0u64;
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_39de9c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0039de9c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0039de9c)?;
                break 'b_39de9c;
            }
        }
        'b_39debc: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_39debc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0039debc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0039debc)?;
                break 'b_39debc;
            }
        }
        'b_39dedc: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_39dedc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0039dedc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0039dedc)?;
                break 'b_39dedc;
            }
        }
        c.cycles += 19;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039def8);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0039def8)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039df14);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0039df14)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void UIScreen_dtor(UIScreen* self, uint flags)` (0x39c558): free the
/// screen's name/child blocks, tear down its lists and node bodies, then
/// release the object itself if `flags & 1`.
pub fn ui_screen_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let f = Frame::open(c, 0x80);
        c.g[V0][0] = 0x0049_0000;
        f.save(c, S5, 0x20);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x4670));
        f.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A0][0];
        f.save(c, S0, 0x70);
        f.save(c, S1, 0x60);
        f.save(c, S2, 0x50);
        f.save(c, S3, 0x40);
        f.save(c, S4, 0x30);
        f.save_ra(c, 0);
        c.sw((c.g[S5][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x3c)).lw(c);
        let t = c.g[A0][0] == 0;
        c.g[S6][0] = c.g[A1][0];
        if !t {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0039c5a0)?;
        }
        // The two name/child blocks, freed when the flag bit is clear.
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[S1][0] = 0x0049_0000;
        let empty = c.g[A0][0] == 0;
        if empty {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x14)).lw(c) & 2;
            let skip = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0049_0000;
            if !skip {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x0039c5c4)?;
                c.cycles += 1;
            }
        }
        c.g[V0][0] = 0x0049_0000;
        // Teardown of the embedded lists and nodes.
        c.cycles += 35;
        let obj = c.g[S5][0] as u32;
        c.g[S1][0] = sx(0x0049_0000u32.wrapping_add(0x4c98));
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_add(0x49e8));
        c.g[S2][0] = sx(obj.wrapping_add(0xb4));
        c.sw(obj.wrapping_add(0xcc), c.g[S1][0]);
        c.sw(obj.wrapping_add(0x48), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x0039c5e4)?;
        c.g[S4][0] = sx(obj.wrapping_add(0x40));
        c.g[S0][0] = 0x0049_0000;
        c.g[A0][0] = sx(obj.wrapping_add(0xc0));
        c.g[S0][0] = sx(0x0049_0000u32.wrapping_add(0x4cc0));
        c.g[S3][0] = sx(obj.wrapping_add(0x18));
        c.sw(obj.wrapping_add(0xc8), c.g[S0][0]);
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c600)?;
        c.sw(obj.wrapping_add(0xbc), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c60c)?;
        c.g[A1][0] = 2;
        c.g[A0][0] = c.g[S4][0];
        callee::UI_OBJECT_DTOR.call(c, 0x0039c618)?;
        c.sw(obj.wrapping_add(0x30), c.g[S1][0]);
        c.g[A0][0] = c.g[S3][0];
        callee::UI_LIST_DELETE_ALL.call(c, 0x0039c624)?;
        c.sw(obj.wrapping_add(0x2c), c.g[S0][0]);
        c.g[A0][0] = sx(obj.wrapping_add(0x24));
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c630)?;
        c.sw(obj.wrapping_add(0x20), c.g[S0][0]);
        c.g[A0][0] = c.g[S3][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c63c)?;
        c.sw(obj.wrapping_add(8), c.g[S0][0]);
        c.g[A0][0] = c.g[S5][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x0039c648)?;
        c.g[V0][0] = c.g[S6][0] & 1;
        let t = c.g[V0][0] == 0;
        f.restore(c, S0, 0x70);
        if !t {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0039c65c)?;
        }
        c.cycles += 9;
        f.restore(c, S1, 0x60);
        f.restore(c, S2, 0x50);
        f.restore(c, S3, 0x40);
        f.restore(c, S4, 0x30);
        f.restore(c, S5, 0x20);
        f.restore(c, S6, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UIState_Update (0x39e510)

/// `UIState`: vtable (+8), owner (+0x10, its pad at +0xc), current pad
/// index (+0x14, -1 when idle), pad mask (+0x15), screen list (+0x24).
pub(crate) const UI_STATE_PAD_INDEX: u32 = 0x14;

pub(crate) const UI_STATE_PAD_MASK: u32 = 0x15;

pub(crate) const UI_STATE_SCREENS: u32 = 0x24;

/// `obj->vt[slot](obj + delta)` with `vt = *(obj + 8)`; the vtable pointer
/// goes to `vt_reg`, the function to `fn_reg` (as the original leaves them).
pub(crate) fn ui_state_vcall(
    c: &mut Cpu,
    obj: u64,
    slot: u32,
    vt_reg: usize,
    fn_reg: usize,
    ret: u32,
) -> crate::call::Flow {
    let vt = ui_vector_lw(c, obj, 8);
    c.g[vt_reg][0] = vt;
    let delta = Addr((vt as u32).wrapping_add(slot)).lh(c);
    let func = ui_vector_lw(c, vt, slot + 4);
    c.g[fn_reg][0] = func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, ret)
}

/// `void UIState_Update(UIState* this)` (0x39e510): for pads 0 and 1 (the
/// mask at +0x15 enabling the second test per pad): select the pad, and if
/// it has input (pad vtable `+0x168`) and pressed "accept" (`+0x148`), or
/// "alt" (`+0x140`) with the mask bit, offer the input to the state
/// (vtable `+0x40`), else to each active screen (`+0x20`) until one takes
/// it; then let the state react (`+0x48`) and stop. The pad index ends -1.
pub fn ui_state_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        let owner = ui_vector_lw(c, this, 0x10);
        c.g[V0][0] = owner;
        c.g[S0][0] = Addr((this as u32).wrapping_add(UI_STATE_PAD_MASK)).lb(c);
        c.g[S1][0] = ui_vector_lw(c, owner, 0xc);
        Addr((this as u32).wrapping_add(UI_STATE_PAD_INDEX)).sb(c, 0);
        loop {
            c.cycles += 13;
            let this = c.g[S2][0];
            let pad = c.g[S1][0];
            let vt = ui_vector_lw(c, pad, 8);
            c.g[V0][0] = vt;
            c.g[A1][0] = Addr((this as u32).wrapping_add(UI_STATE_PAD_INDEX)).lbu(c);
            let delta = Addr((vt as u32).wrapping_add(0x160)).lh(c);
            let func = ui_vector_lw(c, vt, 0x164);
            c.g[A2][0] = func;
            c.g[A0][0] = sx((pad as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x0039_e550)?;
            ui_state_vcall(c, c.g[S1][0], 0x168, V1, V0, 0x0039_e564)?;
            let mut handle = false;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                ui_state_vcall(c, c.g[S1][0], 0x148, V1, V0, 0x0039_e580)?;
                if c.g[V0][0] != 0 {
                    handle = true;
                } else {
                    c.cycles += 7;
                    ui_state_vcall(c, c.g[S1][0], 0x140, V1, V0, 0x0039_e59c)?;
                    let alt = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0] & 1;
                    if !alt {
                        c.cycles += 1;
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.cycles += 1;
                            handle = true;
                        }
                    }
                }
            }
            if handle {
                c.g[V1][0] = ui_vector_lw(c, c.g[S2][0], 8);
                ui_state_handle_input(c)?;
                c.cycles += 7;
                c.g[A1][0] = c.g[S1][0];
                ui_state_vcall(c, c.g[S2][0], 0x48, V0, V1, 0x0039_e63c)?;
                break;
            }
            let this = c.g[S2][0] as u32;
            c.g[V1][0] = Addr(this + UI_STATE_PAD_INDEX).lbu(c);
            c.cycles += 7;
            c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx(((c.g[V1][0] as u32) << 24) as i32 as u32 >> 0);
            c.g[V0][0] = sx((((c.g[V1][0] as u32) << 24) as i32 >> 24) as u32);
            let more = (c.g[V0][0] as i64) < 2;
            c.g[V0][0] = more as u64;
            Addr(this + UI_STATE_PAD_INDEX).sb(c, c.g[V1][0]);
            if !more {
                c.cycles += 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_e538);
            }
        }
        c.g[V0][0] = u64::MAX;
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        Addr((c.g[S2][0] as u32).wrapping_add(UI_STATE_PAD_INDEX)).sb(c, u64::MAX);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Offer the pad (s1) to the state (vtable in v1, slot `+0x40`), then to
/// the active screens until one takes it; leaves v0 = the state's vtable.
pub(crate) fn ui_state_handle_input(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 7;
    c.g[A1][0] = c.g[S1][0];
    let this = c.g[S2][0];
    let vt = c.g[V1][0];
    let delta = Addr((vt as u32).wrapping_add(0x40)).lh(c);
    let func = ui_vector_lw(c, vt, 0x44);
    c.g[V0][0] = func;
    c.g[A0][0] = sx((this as u32).wrapping_add(delta as u32));
    call_ptr(c, func as u32, 0x0039_e5c4)?;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(UI_STATE_SCREENS));
        UI_LIST_FIRST.call(c, 0x0039_e5d4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
            loop {
                c.cycles += 3;
                c.g[V0][0] &= 1;
                c.g[A1][0] = c.g[S1][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    ui_state_vcall(c, c.g[S0][0], 0x20, V1, V0, 0x0039_e608)?;
                    if c.g[V0][0] != 0 {
                        break;
                    }
                }
                c.cycles += 5;
                let node = c.g[S0][0];
                c.g[S0][0] = ui_vector_lw(c, node, 4);
                c.g[A0][0] = c.g[S0][0];
                UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_e61c)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_e5e8);
                }
            }
        }
    }
    c.g[V0][0] = ui_vector_lw(c, c.g[S2][0], 8);
    Ok(())
}

// ------------------------------------------------------------ UIScreen_CreateObject

/// The record-type jump table (`rodata`, indexed by `type - 0x10`).
pub(crate) const UCS_TABLE: u32 = 0x0049_3f70;

/// `operator new(size, name, flags, 0)` (called through the function table).
pub(crate) const UCS_ALLOC: u32 = 0x0031_7d70;

/// `cUIIcon`'s image-handle constructor (called through the function table).
pub(crate) const UCS_ICON_HANDLE_CTOR: u32 = 0x0021_ccc8;

pub(crate) const UCS_OBJECT_CTOR: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::UIObject_ctor,
};

pub(crate) const UCS_LIST_CTOR: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::UIList_ctor,
};

pub(crate) const UCS_FIND: Callee = Callee {
    addr: 0x0039_d860,
    f: ee::s13::UIScreen_FindObject,
};

pub(crate) const UCS_UNLINK: Callee = Callee {
    addr: 0x0039_7740,
    f: ee::s13::UIListNode_Unlink,
};

pub(crate) const UCS_PUSH_BACK: Callee = Callee {
    addr: 0x0039_7910,
    f: ee::s13::UIList_PushBack,
};

pub(crate) const UCS_LIST_BOX_CTOR: Callee = Callee {
    addr: 0x0039_9768,
    f: ee::s13::UIListBox_ctor,
};

pub(crate) const UCS_MENU_FLAGS_CTOR: Callee = Callee {
    addr: 0x003a_5338,
    f: ee::s13::UIMenuFlags_ctor,
};

pub(crate) const UCS_PAIR_FLAGS_CTOR: Callee = Callee {
    addr: 0x003a_5508,
    f: ee::s13::UIPairFlags_ctor,
};

pub(crate) const UCS_TEXT_CTOR: Callee = Callee {
    addr: 0x003a_0348,
    f: ee::s13::UIText_ctor,
};

pub(crate) const UCS_TEXT_SCROLL_CTOR: Callee = Callee {
    addr: 0x003a_1148,
    f: ee::s13::cUITextScroll_ctor,
};

pub(crate) const UCS_VECTOR_CTOR: Callee = Callee {
    addr: 0x003a_3280,
    f: ee::s13::UIVector_ctor,
};

/// Object vtables the constructor stores (`cUIIcon`, `cUIGroup`, `cUIMenu`,
/// `cUIPair`, `cUIProgress`, type 0x21, type 0x16).
pub(crate) const UCS_VT_ICON: u32 = 0x0049_4aa8;

pub(crate) const UCS_VT_GROUP: u32 = 0x0049_49e8;

pub(crate) const UCS_VT_MENU: u32 = 0x0049_4928;

pub(crate) const UCS_VT_PAIR: u32 = 0x0049_4868;

pub(crate) const UCS_VT_PROGRESS: u32 = 0x0049_4408;

pub(crate) const UCS_VT_21: u32 = 0x0049_44c8;

pub(crate) const UCS_VT_16: u32 = 0x0049_4348;

/// Object field offsets.
pub(crate) const UCS_OBJ_VTABLE: u32 = 0x8;

pub(crate) const UCS_OBJ_FLAGS: u32 = 0x14;

/// A child's parent pointer.
pub(crate) const UCS_CHILD_PARENT: u32 = 0x40;

/// Vtable slot of `initFromRec(rec)` (`{delta: i16, fn}`) and of the deleting
/// destructor.
pub(crate) const UCS_VT_INIT: u32 = 0x50;

pub(crate) const UCS_VT_DELETE: u32 = 0x8;

#[inline]
pub(crate) fn ucs_reg(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `operator new(size, name, 0x20000000, 0)` returning to `ret`.
pub(crate) fn ucs_alloc(c: &mut Cpu, size: u32, name: u32, ret: u32) -> Flow {
    c.g[A1][0] = sx(name);
    c.g[A3][0] = 0;
    c.g[A0][0] = size as u64;
    c.g[A2][0] = 0x2000_0000;
    call_ptr(c, UCS_ALLOC, ret)
}

/// `ctor(obj_reg, screen, 0)`: `a0 = obj`, `a1 = s5`, `a2 = 0`.
pub(crate) fn ucs_ctor(c: &mut Cpu, ctor: Callee, obj_reg: usize, ret: u32) -> Flow {
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[obj_reg][0];
    ctor.call(c, ret)
}

/// `ctor(new block in v0, screen, 0)`.
pub(crate) fn ucs_ctor_v0(c: &mut Cpu, ctor: Callee, ret: u32) -> Flow {
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = 0;
    ctor.call(c, ret)
}

/// Store vtable `vt` (left in `v1`) into the object in `obj_reg` and point
/// `a0` at its member at `member` (the next constructor's `this`).
pub(crate) fn ucs_set_vtable(c: &mut Cpu, obj_reg: usize, vt: u32, member: u32) {
    let obj = ucs_reg(c, obj_reg);
    c.g[V1][0] = sx(vt);
    c.g[A0][0] = sx(obj.wrapping_add(member));
    Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(vt));
}

/// The virtual call through vtable `vt`'s slot `slot` on `this`
/// (`a0 = this + delta`, `v1` = the method), returning to `ret`.
pub(crate) fn ucs_virtual(c: &mut Cpu, vt: u32, slot: u32, this: u32, ret: u32) -> Flow {
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// 0x39d6c8: `s1->initFromRec(rec)` through the vtable in `v0`, return `s1`.
pub(crate) fn ucs_init_s1(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = c.g[S4][0];
    let vt = c.g[V0][0] as u32;
    let this = ucs_reg(c, S1);
    ucs_virtual(c, vt, UCS_VT_INIT, this, 0x0039_d6dc)?;
    c.cycles += 2;
    c.g[V0][0] = c.g[S1][0];
    Ok(())
}

/// 0x39d6c4: load `s1`'s vtable, then [`ucs_init_s1`].
pub(crate) fn ucs_init_s1_load(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    let obj = ucs_reg(c, S1);
    c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
    ucs_init_s1(c)
}

/// 0x39d718: `s0->initFromRec(a1)` through the vtable in `v0`, return `s0`.
pub(crate) fn ucs_init_s0(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    let vt = c.g[V0][0] as u32;
    let this = ucs_reg(c, S0);
    ucs_virtual(c, vt, UCS_VT_INIT, this, 0x0039_d728)?;
    c.cycles += 2;
    c.g[V0][0] = c.g[S0][0];
    Ok(())
}

/// 0x39d710: `a1 = rec`, load `s0`'s vtable, then [`ucs_init_s0`].
pub(crate) fn ucs_init_s0_load(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[A1][0] = c.g[S4][0];
    let obj = ucs_reg(c, S0);
    c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
    ucs_init_s0(c)
}

/// Where a child loop reads its count from the record.
#[derive(Clone, Copy)]
pub(crate) enum UcsCount {
    /// `lw rec+off`.
    Word(u32),
    /// `lbu rec+off`.
    Byte(u32),
}

pub(crate) fn ucs_count(c: &mut Cpu, count: UcsCount) -> u64 {
    let rec = ucs_reg(c, S4);
    match count {
        UcsCount::Word(off) => Addr(rec.wrapping_add(off)).lw(c),
        UcsCount::Byte(off) => Addr(rec.wrapping_add(off)).lbu(c),
    }
}

/// The child loop (head `head`): for each name hash at `s3` (s2 counting up
/// to the record's count), find the child on the screen, unlink it, parent
/// it to `s1` and append it to `s1`'s list at `list`. Left at the exit test.
pub(crate) fn ucs_adopt_children(
    c: &mut Cpu,
    count: UcsCount,
    list: u32,
    head: u32,
    rets: [u32; 3],
) -> Flow {
    loop {
        c.cycles += 6;
        let hash_at = ucs_reg(c, S3);
        c.g[A1][0] = Addr(hash_at).lw(c);
        c.g[A0][0] = c.g[S5][0];
        UCS_FIND.call(c, rets[0])?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            UCS_UNLINK.call(c, rets[1])?;
            let (child, owner) = (ucs_reg(c, S0), c.g[S1][0]);
            Addr(child.wrapping_add(UCS_CHILD_PARENT)).sw(c, owner);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(ucs_reg(c, S1).wrapping_add(list));
            UCS_PUSH_BACK.call(c, rets[2])?;
        }
        c.g[V0][0] = ucs_count(c, count);
        c.cycles += 4;
        c.g[S2][0] = sx(ucs_reg(c, S2).wrapping_add(1));
        let more = c.g[S2][0] < c.g[V0][0];
        c.g[V0][0] = more as u64;
        c.g[S3][0] = sx(ucs_reg(c, S3).wrapping_add(4));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Two named parts (types 0x19, 0x21, 0x16): the hashes at `rec+hashes[i]`
/// name screen objects; each one found is unlinked, parented to the object
/// in `obj` and stored at `obj+slots[i]`. Returns the object in `v0`.
pub(crate) fn ucs_attach_two(
    c: &mut Cpu,
    obj: usize,
    part: usize,
    hashes: [u32; 2],
    slots: [u32; 2],
    rets: [u32; 4],
) -> Flow {
    let rec = ucs_reg(c, S4);
    c.g[A1][0] = Addr(rec.wrapping_add(hashes[0])).lw(c);
    c.g[A0][0] = c.g[S5][0];
    UCS_FIND.call(c, rets[0])?;
    c.g[part][0] = c.g[V0][0];
    if c.g[part][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[part][0];
        UCS_UNLINK.call(c, rets[1])?;
        ucs_parent_part(c, obj, part, slots[0]);
    }
    let rec = ucs_reg(c, S4);
    c.g[A1][0] = Addr(rec.wrapping_add(hashes[1])).lw(c);
    c.cycles += 5;
    c.g[A0][0] = c.g[S5][0];
    UCS_FIND.call(c, rets[2])?;
    c.g[part][0] = c.g[V0][0];
    c.g[V0][0] = c.g[obj][0];
    if c.g[part][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[part][0];
    UCS_UNLINK.call(c, rets[3])?;
    ucs_parent_part(c, obj, part, slots[1]);
    c.cycles += 2;
    c.g[V0][0] = c.g[obj][0];
    Ok(())
}

/// `part->parent = obj; obj->slot = part`.
pub(crate) fn ucs_parent_part(c: &mut Cpu, obj: usize, part: usize, slot: u32) {
    let (o, p) = (c.g[obj][0], c.g[part][0]);
    Addr((p as u32).wrapping_add(UCS_CHILD_PARENT)).sw(c, o);
    Addr((o as u32).wrapping_add(slot)).sw(c, p);
}

/// `cUIPair`: delete the part already at `s0+slot` (virtual deleting
/// destructor with `a1 = 3`), then store the new part `s1` there.
pub(crate) fn ucs_replace_part(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    let obj = ucs_reg(c, S0);
    c.g[A2][0] = Addr(obj.wrapping_add(slot)).lw(c);
    c.g[A1][0] = 3;
    if c.g[A2][0] != 0 {
        c.cycles += 5;
        let old = ucs_reg(c, A2);
        c.g[V0][0] = Addr(old.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
        let vt = c.g[V0][0] as u32;
        ucs_virtual(c, vt, UCS_VT_DELETE, old, ret)?;
    }
    c.cycles += 2;
    let (obj, part) = (ucs_reg(c, S0), c.g[S1][0]);
    Addr(obj.wrapping_add(slot)).sw(c, part);
    Ok(())
}

/// The object for record type `ty` (0x10..=0x21); `v0` = the object or 0.
pub(crate) fn ucs_build(c: &mut Cpu, ty: u32) -> Flow {
    match ty {
        // cUIGroup: children named by the word count at +0x20.
        0x10 => {
            c.cycles += 21;
            c.g[S3][0] = sx(ucs_reg(c, S4).wrapping_add(0x24));
            ucs_alloc(c, 0x90, 0x0049_3f00, 0x0039_d140)?;
            c.g[S2][0] = 0;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d158)?;
            ucs_set_vtable(c, S1, UCS_VT_GROUP, 0x74);
            UCS_LIST_CTOR.call(c, 0x0039_d16c)?;
            let rec = ucs_reg(c, S4);
            c.g[V1][0] = Addr(rec.wrapping_add(0x20)).lw(c);
            if c.g[V1][0] != 0 {
                ucs_adopt_children(
                    c,
                    UcsCount::Word(0x20),
                    0x74,
                    0x0039_d178,
                    [0x0039_d184, 0x0039_d198, 0x0039_d1a8],
                )?;
                c.cycles += 2;
            }
            let obj = ucs_reg(c, S1);
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            ucs_init_s1(c)
        }
        // cUIIcon.
        0x11 => {
            c.cycles += 22;
            ucs_alloc(c, 0x88, 0x004a_4788, 0x0039_d0e4)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S0, 0x0039_d0f8)?;
            ucs_set_vtable(c, S0, UCS_VT_ICON, 0x74);
            call_ptr(c, UCS_ICON_HANDLE_CTOR, 0x0039_d10c)?;
            let obj = ucs_reg(c, S0);
            c.g[V1][0] = u64::MAX;
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            Addr(obj.wrapping_add(0x78)).sw(c, u64::MAX);
            ucs_init_s0(c)
        }
        // cUIListBox.
        0x12 => {
            c.cycles += 12;
            ucs_alloc(c, 0x31c, 0x0049_3f10, 0x0039_d1dc)?;
            ucs_ctor_v0(c, UCS_LIST_BOX_CTOR, 0x0039_d1ec)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_init_s0_load(c)
        }
        // cUIMenu: a group with menu flags; byte count at +0x20.
        0x13 => {
            c.cycles += 33;
            c.g[S3][0] = sx(ucs_reg(c, S4).wrapping_add(0x24));
            ucs_alloc(c, 0xa8, 0x004a_4790, 0x0039_d210)?;
            c.g[S2][0] = 0;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d228)?;
            ucs_set_vtable(c, S1, UCS_VT_GROUP, 0x74);
            UCS_LIST_CTOR.call(c, 0x0039_d23c)?;
            ucs_set_vtable(c, S1, UCS_VT_MENU, 0x90);
            UCS_MENU_FLAGS_CTOR.call(c, 0x0039_d250)?;
            let (rec, obj) = (ucs_reg(c, S4), ucs_reg(c, S1));
            c.g[V0][0] = Addr(rec.wrapping_add(0x20)).lbu(c);
            c.g[V1][0] = 0xff;
            Addr(obj.wrapping_add(0xa4)).sb(c, 0xff);
            for off in [0x95, 0x96, 0x97, 0x98] {
                Addr(obj.wrapping_add(off)).sb(c, 0);
            }
            Addr(obj.wrapping_add(0x9c)).sw(c, 0);
            Addr(obj.wrapping_add(0xa0)).sw(c, 0);
            if c.g[V0][0] == 0 {
                return ucs_init_s1_load(c);
            }
            ucs_adopt_children(
                c,
                UcsCount::Byte(0x20),
                0x74,
                0x0039_d278,
                [0x0039_d284, 0x0039_d298, 0x0039_d2a8],
            )?;
            c.cycles += 2;
            let obj = ucs_reg(c, S1);
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            ucs_init_s1(c)
        }
        // cUIPair: two parts named at +0x20/+0x24, replacing older ones.
        0x15 => {
            c.cycles += 34;
            c.g[S2][0] = sx(ucs_reg(c, S4).wrapping_add(0x20));
            ucs_alloc(c, 0x80, 0x004a_4798, 0x0039_d2e0)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S0, 0x0039_d2f4)?;
            ucs_set_vtable(c, S0, UCS_VT_PAIR, 0x74);
            UCS_PAIR_FLAGS_CTOR.call(c, 0x0039_d308)?;
            let obj = ucs_reg(c, S0);
            c.g[V1][0] = Addr(obj.wrapping_add(UCS_OBJ_FLAGS)).lw(c);
            c.g[A1][0] = c.g[S4][0];
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            c.g[V1][0] |= 2;
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            let flags = c.g[V1][0];
            Addr(obj.wrapping_add(UCS_OBJ_FLAGS)).sw(c, flags);
            let vt = c.g[V0][0] as u32;
            ucs_virtual(c, vt, UCS_VT_INIT, obj, 0x0039_d334)?;
            let rec = ucs_reg(c, S4);
            c.g[A1][0] = Addr(rec.wrapping_add(0x20)).lw(c);
            c.g[A0][0] = c.g[S5][0];
            UCS_FIND.call(c, 0x0039_d340)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] != 0 {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                UCS_UNLINK.call(c, 0x0039_d354)?;
                let (part, owner) = (ucs_reg(c, S1), c.g[S0][0]);
                Addr(part.wrapping_add(UCS_CHILD_PARENT)).sw(c, owner);
                ucs_replace_part(c, 0x78, 0x0039_d378)?;
            }
            let names = ucs_reg(c, S2);
            c.g[A1][0] = Addr(names.wrapping_add(4)).lw(c);
            c.cycles += 5;
            c.g[A0][0] = c.g[S5][0];
            UCS_FIND.call(c, 0x0039_d388)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = !1u64;
            if c.g[S1][0] != 0 {
                // The second part starts hidden (flag bit 0 clear).
                c.cycles += 9;
                let part = ucs_reg(c, S1);
                c.g[V0][0] = Addr(part.wrapping_add(UCS_OBJ_FLAGS)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] &= c.g[V1][0];
                let flags = c.g[V0][0];
                Addr(part.wrapping_add(UCS_OBJ_FLAGS)).sw(c, flags);
                UCS_UNLINK.call(c, 0x0039_d3a8)?;
                let (part, owner) = (ucs_reg(c, S1), c.g[S0][0]);
                Addr(part.wrapping_add(UCS_CHILD_PARENT)).sw(c, owner);
                ucs_replace_part(c, 0x7c, 0x0039_d3cc)?;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S0][0];
            Ok(())
        }
        // Type 0x16: parts named at +0x24/+0x28 into +0x88/+0x8c.
        0x16 => {
            c.cycles += 31;
            ucs_alloc(c, 0x90, 0x0049_3f40, 0x0039_d56c)?;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d580)?;
            let obj = ucs_reg(c, S1);
            Addr(obj.wrapping_add(0x74)).sw(c, 0);
            c.g[V1][0] = sx(UCS_VT_16);
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = Addr(UCS_VT_16 + UCS_VT_INIT).lh(c);
            c.g[V0][0] = Addr(UCS_VT_16 + UCS_VT_INIT + 4).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            for off in [0x80, 0x88, 0x8c] {
                Addr(obj.wrapping_add(off)).sw(c, 0);
            }
            let method = c.g[V0][0] as u32;
            Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(UCS_VT_16));
            call_ptr(c, method, 0x0039_d5b8)?;
            ucs_attach_two(
                c,
                S1,
                S0,
                [0x24, 0x28],
                [0x88, 0x8c],
                [0x0039_d5c4, 0x0039_d5d8, 0x0039_d5ec, 0x0039_d600],
            )
        }
        // cUIText.
        0x17 => {
            c.cycles += 12;
            ucs_alloc(c, 0xbc, 0x004a_47a0, 0x0039_d624)?;
            ucs_ctor_v0(c, UCS_TEXT_CTOR, 0x0039_d634)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_init_s0_load(c)
        }
        // cUIVector.
        0x18 => {
            c.cycles += 11;
            ucs_alloc(c, 0x90, 0x0049_3f60, 0x0039_d6fc)?;
            ucs_ctor_v0(c, UCS_VECTOR_CTOR, 0x0039_d70c)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_init_s0_load(c)
        }
        // cUIProgress: parts named at +0x20/+0x24 into +0x74/+0x78.
        0x19 => {
            c.cycles += 30;
            ucs_alloc(c, 0x88, 0x0049_3f20, 0x0039_d3ec)?;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d400)?;
            let obj = ucs_reg(c, S1);
            Addr(obj.wrapping_add(0x74)).sw(c, 0);
            c.g[V1][0] = sx(UCS_VT_PROGRESS);
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = Addr(UCS_VT_PROGRESS + UCS_VT_INIT).lh(c);
            c.g[V0][0] = Addr(UCS_VT_PROGRESS + UCS_VT_INIT + 4).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            Addr(obj.wrapping_add(0x80)).sw(c, 0);
            Addr(obj.wrapping_add(0x84)).sw(c, 0);
            let method = c.g[V0][0] as u32;
            Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(UCS_VT_PROGRESS));
            call_ptr(c, method, 0x0039_d434)?;
            ucs_attach_two(
                c,
                S1,
                S0,
                [0x20, 0x24],
                [0x74, 0x78],
                [0x0039_d440, 0x0039_d454, 0x0039_d468, 0x0039_d47c],
            )
        }
        // cUITextScroll: children named by the byte count at +0x22.
        0x20 => {
            c.cycles += 16;
            c.g[S3][0] = sx(ucs_reg(c, S4).wrapping_add(0x2c));
            ucs_alloc(c, 0xd0, 0x0049_3f50, 0x0039_d658)?;
            c.g[S2][0] = 0;
            ucs_ctor_v0(c, UCS_TEXT_SCROLL_CTOR, 0x0039_d66c)?;
            c.g[S1][0] = c.g[V0][0];
            let rec = ucs_reg(c, S4);
            c.g[V0][0] = Addr(rec.wrapping_add(0x22)).lbu(c);
            if c.g[V0][0] == 0 {
                let obj = ucs_reg(c, S1);
                c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
                return ucs_init_s1(c);
            }
            c.cycles += 1;
            ucs_adopt_children(
                c,
                UcsCount::Byte(0x22),
                0x8c,
                0x0039_d680,
                [0x0039_d68c, 0x0039_d6a0, 0x0039_d6b0],
            )?;
            ucs_init_s1_load(c)
        }
        // Type 0x21: parts named at +0x20/+0x24 into +0x74/+0x78.
        0x21 => {
            c.cycles += 36;
            ucs_alloc(c, 0xcc, 0x0049_3f30, 0x0039_d4a0)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S0, 0x0039_d4b4)?;
            let obj = ucs_reg(c, S0);
            Addr(obj.wrapping_add(0x74)).sw(c, 0);
            c.g[V1][0] = sx(UCS_VT_21);
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = Addr(UCS_VT_21 + UCS_VT_INIT).lh(c);
            c.g[V0][0] = Addr(UCS_VT_21 + UCS_VT_INIT + 4).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            for off in [0x80, 0x84, 0x88, 0xc0, 0xc4, 0xc8] {
                Addr(obj.wrapping_add(off)).sw(c, 0);
            }
            Addr(obj.wrapping_add(0x8c)).sb(c, 0);
            Addr(obj.wrapping_add(0xa5)).sb(c, 0);
            let method = c.g[V0][0] as u32;
            Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(UCS_VT_21));
            call_ptr(c, method, 0x0039_d500)?;
            ucs_attach_two(
                c,
                S0,
                S1,
                [0x20, 0x24],
                [0x74, 0x78],
                [0x0039_d50c, 0x0039_d520, 0x0039_d534, 0x0039_d548],
            )
        }
        // 0x14, 0x1a..=0x1f: no object.
        _ => {
            c.cycles += 1;
            c.g[V0][0] = 0;
            Ok(())
        }
    }
}

/// `cUIThing* cUIScreen::createObjectByStruct(cUIScreen* self, Rec* rec)`
/// (`UIScreen_CreateObject`, 0x39d078): builds the UI object a screen file
/// record describes (type halfword at +0: icon, group, list box, menu,
/// pair, progress bar, text, text scroll, vector, ...), hands it the record
/// (`initFromRec`), and takes the named child objects off the screen's
/// object list into the new object. Returns the object, or 0 for unknown
/// types.
pub fn ui_screen_create_object(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S4, 0x20);
    frame.save(c, S5, 0x10);
    c.g[S4][0] = c.g[A1][0];
    frame.save(c, S0, 0x60);
    c.g[S5][0] = c.g[A0][0];
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    frame.save_ra(c, 0);
    let ty = Addr(ucs_reg(c, S4)).lhu(c);
    c.g[V0][0] = ty;
    c.g[A0][0] = sx((ty as u32).wrapping_sub(0x10));
    let known = c.g[A0][0] < 0x12;
    c.g[V1][0] = known as u64;
    c.g[V0][0] = 0x49_0000;
    if known {
        c.cycles += 6;
        let index = c.g[A0][0] as u32;
        c.g[V1][0] = sx(UCS_TABLE + (index << 2));
        c.g[V0][0] = sx(UCS_TABLE);
        // The jump table entry (left in a0 as the compiled code does).
        c.g[A0][0] = Addr(UCS_TABLE + (index << 2)).lw(c);
        if ucs_build(c, 0x10 + index).is_err() {
            return;
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0;
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
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::camera::cameracontroller::tests::fake_child;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::slot_8 as slot_5;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;

    pub(crate) use crate::game::fe::feasyncfile::tests::h;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_5;

    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::slot_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_6;
    pub(crate) use crate::game::object::effectlink::tests::rng_word;
    pub(crate) use crate::game::object::movenode::tests::OBJ4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;

    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::ui::uiengine::tests::alloc;
    use ssx3_ee as ee;

    #[test]
    fn uiscreen_cmd_stop_thread_matches() {
        check(
            ssx3_ee::s13::f_0039cbc8,
            uiscreen_cmd_stop_thread,
            200,
            &|c, seed| {
                let mut r = Rng(0x39cb_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A3][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn uiscreen_cmd_end_transition_in_matches() {
        check(
            ssx3_ee::s13::f_0039ce20,
            uiscreen_cmd_end_transition_in,
            200,
            &|c, seed| {
                let mut r = Rng(0x39ce_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0xd0, 0);
                } else {
                    let screen = DATA + 0x400;
                    c.bus.write32(DATA + 0xd0, screen);
                    c.bus.write32(screen + 0x1c, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn uiscreen_cmd_end_transition_out_matches() {
        check(
            ssx3_ee::s13::f_0039ce98,
            uiscreen_cmd_end_transition_out,
            200,
            &|c, seed| {
                let mut r = Rng(0x39ce_9800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A3][0] = (DATA + 0x800) as u64;
                c.bus.write32(DATA + 0x810, r.next_u32());
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0xd0, 0);
                } else {
                    let screen = DATA + 0x400;
                    c.bus.write32(DATA + 0xd0, screen);
                    c.bus.write32(screen + 0x1c, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ui_menu_flags_ctor_matches() {
        check(
            ee::s13::f_003a5338__generated,
            ui_menu_flags_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x003a_5338, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write8(DATA, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn ui_state_add_screen_matches() {
        check(
            ee::s13::f_0039e758__generated,
            ui_state_add_screen,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_e758, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                match seed % 3 {
                    0 => c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, 0),
                    1 => c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, DATA + 0x900),
                    _ => {
                        c.bus.write32(DATA + 0x1000 + 0x24 + 0xc, DATA + 0x900);
                        c.bus.write32(DATA + 0x900 + 4, DATA + 0xa00);
                    }
                }
                c.bus.write32(DATA + 0x800 + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_list_node_dtor_body_matches() {
        check(
            ee::s13::f_003a4868__generated,
            ui_list_node_dtor_body,
            200,
            &|c, seed| {
                machine(c, seed, 0x3a_4868, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                match seed % 3 {
                    // Head of the list, no neighbours.
                    0 => {}
                    // Linked between two other nodes.
                    1 => {
                        c.bus.write32(DATA + 0x1000, DATA + 0x800);
                        c.bus.write32(DATA + 0x1000 + 4, DATA + 0x900);
                    }
                    // Only a successor.
                    _ => {
                        c.bus.write32(DATA + 0x1000, 0);
                        c.bus.write32(DATA + 0x1000 + 4, DATA + 0x1000);
                    }
                }
            },
        );
    }

    #[test]
    fn ui_pair_flags_ctor_matches() {
        crate::difftest::check(
            ee::s13::f_003a5508__generated,
            ui_pair_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a55_0801 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_state_v13_matches() {
        check(
            ee::s13::f_0039e688__generated,
            ui_state_v13,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39e6_88, 2);
                be_library(c);
                let obj = DATA + 0x800;
                let sub = DATA + 0x900;
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 8, sub);
                let vt = fakes::vtable(c, &mut r, &[0x38]);
                c.bus.write32(sub, vt);
            },
        );
    }

    #[test]
    fn cui_slider_load_matches() {
        check(
            ee::s13::f_0039dfb0__generated,
            cui_slider_load,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39df_b0, 0);
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
    fn ui_object_clear_anim_matches() {
        covered(
            ee::s13::f_0039fd38__generated,
            ui_object_clear_anim,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_fd38, 1);
                let obj = DATA + 0x1000;
                let anim = DATA + 0x2000;
                if seed % 2 != 0 {
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(anim + 0x10, vt);
                    c.bus.write32(obj + 0xc, anim);
                }
                c.g[A0][0] = sx(obj);
                let _ = r.next_u32();
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x39ce48 UIScreen_CmdBeginTransitionOut.

    #[test]
    fn ui_screen_cmd_begin_transition_out_matches() {
        check(
            ee::s13::f_0039ce48__generated,
            ui_screen_cmd_begin_transition_out,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39ce_48, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                for i in 0..8u32 {
                    c.bus.write32(this + 0x40 + i * 4, r.next_u32());
                }
                if seed % 2 == 0 {
                    c.bus.write32(this + 0xd0, 0);
                } else {
                    let owner = DATA + 0x300;
                    c.bus.write32(this + 0xd0, owner);
                    c.bus.write32(owner + 0x1c, r.next_u32());
                }
            },
        );
    }

    // --------------------------------------------------------------- UI screen

    /// A thread at frame `frame` (speed `speed`, end `end`) of a screen whose
    /// keyframe list has an entry at frames 0..4 with `n` no-op commands.
    fn thread_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        frame: u16,
        speed: i8,
        end: u16,
        n: u16,
    ) -> (u32, u32) {
        let screen = DATA;
        let thread = DATA + 0x80;
        let list = DATA + 0x100;
        c.bus.write32(list, 5);
        let mut at = list + 4;
        for i in 0..5u32 {
            let size = 0xc + 4 * n as u32;
            c.bus.write32(at + 4, size);
            c.bus.write16(at + 8, i as u16);
            c.bus.write16(at + 0xa, n);
            for k in 0..n as u32 {
                // UIScreen_ThreadNopA/B/C.
                c.bus.write8(
                    at + 0xc + 4 * k,
                    [0x22u8, 0x31, 0x40][(r.next_u32() % 3) as usize],
                );
                c.bus.write16(at + 0xc + 4 * k + 2, 4);
            }
            at += size;
        }
        c.bus.write32(screen + 0x34, list);
        c.bus.write16(thread + 0xc, frame);
        c.bus.write16(thread + 0xe, frame.wrapping_sub(1));
        c.bus.write16(thread + 0x10, end);
        c.bus.write8(thread + 0x12, speed as u8);
        c.g[A0][0] = sx(screen);
        c.g[A1][0] = sx(thread);
        (screen, thread)
    }

    #[test]
    fn ui_screen_run_thread_frame_matches() {
        check(
            ee::s13::f_0039c978__generated,
            ui_screen_run_thread_frame,
            120,
            &|c, seed| {
                let mut r = Rng(0x39c9_7800 ^ seed as u64);
                scramble(c, &mut r);
                let frame = (seed % 7) as u16;
                let speed = [1i8, 2, -1, 0][(seed / 7 % 4) as usize];
                let end = [0xffffu16, 3, 0][(seed / 28 % 3) as usize];
                thread_fixture(c, &mut r, frame, speed, end, (seed % 3) as u16);
                // (Entered by a call: the generated body dispatches on pc.)
                c.pc = 0x0039_c978;
            },
        );
        // The fixture runs the body (the frame steps when speed != 0).
        let mut c = crate::difftest::fresh();
        thread_fixture(&mut c, &mut Rng(1), 2, 1, 0xffff, 1);
        c.pc = 0x0039_c978;
        ui_screen_run_thread_frame(&mut c);
        assert_eq!(
            (
                c.bus.read16(DATA + 0x80 + 0xc),
                c.bus.read16(DATA + 0x80 + 0xe)
            ),
            (3, 2)
        );
    }

    /// At 240 Hz a thread steps one frame per four ticks (60 per second),
    /// at 60 one per tick.
    #[test]
    fn ui_screen_run_thread_frame_rate() {
        for (hz, ticks) in [(60u32, 60u32), (240, 240)] {
            let mut c = crate::difftest::fresh();
            let mut r = Rng(0x39c978);
            crate::tick::set_for_test(&mut c, hz);
            let app = c.bus.read32(crate::tick::APPMAN_PTR);
            let (_, thread) = thread_fixture(&mut c, &mut r, 0, 1, 0xffff, 1);
            for t in 1..=ticks {
                c.bus.write32(app + crate::tick::APPMAN_VSYNC, t);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(thread);
                c.g[RA][0] = sx(0x0010_0000);
                // (Entered by a call, like UIScreen_Update does.)
                c.pc = 0x0039_c978;
                ui_screen_run_thread_frame(&mut c);
            }
            assert_eq!(c.bus.read16(thread + 0xc), 60, "{hz} Hz");
        }
    }

    #[test]
    fn ui_screen_update_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(
            ee::s13::f_0039d778__generated,
            ui_screen_update,
            240,
            &|c, seed| {
                let mut r = Rng(0x39d7_7800 ^ seed as u64);
                scramble(c, &mut r);
                let mode: Events = schedule(c, &mut r, seed, 0x0039_d778);
                // Each choice below is drawn on its own, so every path meets
                // every event mode.
                let frame = (r.next_u32() % 7) as u16;
                let speed = [1i8, 2, -1, 0][(r.next_u32() % 4) as usize];
                let end = [0xffffu16, 3, 0][(r.next_u32() % 3) as usize];
                let n = (r.next_u32() % 3) as u16;
                let (screen, thread) = thread_fixture(c, &mut r, frame, speed, end, n);
                let sentinel = DATA + 0x900;
                c.bus.write32(sentinel + 4, sentinel);
                // A second thread on half the seeds: the walk reaches it over
                // the back edge, where an event may unwind and resume.
                let thread2 = DATA + 0x180;
                let two = r.next_u32() % 2 == 0;
                if two {
                    let f2 = (r.next_u32() % 7) as u16;
                    let ran = if r.next_u32() % 3 == 0 {
                        f2
                    } else {
                        (r.next_u32() % 7) as u16
                    };
                    c.bus.write16(thread2 + 0xc, f2);
                    c.bus.write16(thread2 + 0xe, ran);
                    c.bus
                        .write16(thread2 + 0x10, [0xffffu16, 3][(r.next_u32() % 2) as usize]);
                    c.bus
                        .write8(thread2 + 0x12, [1i8, -1][(r.next_u32() % 2) as usize] as u8);
                    c.bus.write8(thread2 + 0x13, (r.next_u32() % 2) as u8);
                    c.bus.write32(thread2, 0);
                    c.bus.write32(thread2 + 4, sentinel);
                }
                // The first thread: finished (`+0xe` is its frame), held
                // (bit 0 of `+0x13`), or run and removed through the unlink
                // path.
                let kind = r.next_u32() % 3;
                if kind == 0 {
                    c.bus.write16(thread + 0xe, frame);
                }
                c.bus.write8(thread + 0x13, (kind == 1) as u8);
                c.bus.write32(thread, 0);
                c.bus
                    .write32(thread + 4, if two { thread2 } else { sentinel });
                // A null next link on some seeds: the walk steps onto node 0
                // (a runnable thread linking back to the last one), runs and
                // unlinks it, and skips the cleanup call of a null node.
                if r.next_u32() % 4 == 0 {
                    let last = if two { thread2 } else { thread };
                    c.bus.write32(last + 4, 0);
                    let f0 = (r.next_u32() % 5) as u16;
                    c.bus.write32(0, last);
                    c.bus.write32(4, sentinel);
                    c.bus.write16(0xc, f0);
                    c.bus.write16(0xe, f0.wrapping_add(1));
                    c.bus.write16(0x10, 0xffff);
                    c.bus.write8(0x12, 1);
                    c.bus.write8(0x13, 0);
                }
                // A primary thread (bit 25 of the word at `+0x10`: bit 1 of
                // `+0x13`) on some seeds, either thread.
                let primary = match r.next_u32() % 3 {
                    0 => thread,
                    1 if two => thread2,
                    _ => 0,
                };
                if primary != 0 {
                    let flags = c.bus.read8(primary + 0x13);
                    c.bus.write8(primary + 0x13, flags | 2);
                }
                // The removal path calls a cleanup through a fabricated
                // vtable: a safe read of the thread's next link.
                let cleanup = DATA + 0xa00;
                c.bus.write32(thread + 8, cleanup);
                c.bus.write32(thread2 + 8, cleanup);
                c.bus.write16(cleanup + 8, 0);
                c.bus.write32(cleanup + 0xc, 0x0039_7718);
                // An empty node list and a null animation for the group.
                c.bus.write32(screen + 0x40 + 0x74 + 4, 0);
                c.bus.write32(screen + 0x40 + 0xc, 0);
                // The thread list: no first node, empty, or the threads.
                let first = match r.next_u32() % 8 {
                    0 => 0,
                    1 => sentinel,
                    _ => thread,
                };
                c.bus.write32(screen + 0x18 + 4, first);
                c.g[A0][0] = sx(screen);
                mode
            },
        );
    }

    /// The tests below drive `ui_screen_update` through the real
    /// screen/thread/widget path: the callees with tick-rate handling must
    /// reach their ports (as in the game), the rest run their generated
    /// bodies. Installs those three slots and puts the previous ones back.
    /// The slots are process-wide and tests run in parallel: one test holds
    /// them at a time, or one test's drop would put the generated bodies
    /// back under another.
    struct Slots {
        run_thread_frame: fn(&mut Cpu),
        anim_advance: fn(&mut Cpu),
        anim_apply: fn(&mut Cpu),
        // Released after `drop` has put the slots back.
        _held: std::sync::MutexGuard<'static, ()>,
    }

    static SLOTS_HELD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    impl Slots {
        fn install() -> Slots {
            use crate::game::ui::uianimation::{ui_anim_advance, ui_anim_apply};
            // (A test that failed while holding them has already put them back.)
            let held = SLOTS_HELD.lock().unwrap_or_else(|e| e.into_inner());
            unsafe {
                let saved = Slots {
                    run_thread_frame: ee::s13::OVERRIDE_0039C978,
                    anim_advance: ee::s13::OVERRIDE_003974B0,
                    anim_apply: ee::s13::OVERRIDE_003975E0,
                    _held: held,
                };
                ee::s13::OVERRIDE_0039C978 = ui_screen_run_thread_frame;
                ee::s13::OVERRIDE_003974B0 = ui_anim_advance;
                ee::s13::OVERRIDE_003975E0 = ui_anim_apply;
                saved
            }
        }
    }

    impl Drop for Slots {
        fn drop(&mut self) {
            unsafe {
                ee::s13::OVERRIDE_0039C978 = self.run_thread_frame;
                ee::s13::OVERRIDE_003974B0 = self.anim_advance;
                ee::s13::OVERRIDE_003975E0 = self.anim_apply;
            }
        }
    }

    /// A screen with one timeline thread (empty keyframes), a group holding
    /// `widgets` (`(mode, start, count)` each with its own animation and
    /// target; mode 9 wraps) driven through the real update path. Layout
    /// lives at `base` (distinct per scenario, so host clocks never alias).
    /// Returns (screen, thread).
    fn full_path_fixture(
        c: &mut Cpu,
        base: u32,
        hz: u32,
        step: i8,
        widgets: &[(u32, u16, u16)],
    ) -> (u32, u32) {
        // The widgets' own updates are fake callees, as in the engine
        // tests: point the miss hook at them.
        c.hooks.miss = crate::difftest::fakes::fake_call;
        c.hooks.jump_out = crate::difftest::fakes::fake_jump_out;
        c.hooks.syscall = crate::difftest::fakes::fake_syscall;
        // Never unwind from a fake: the widgets' updates always return.
        c.bus
            .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
        crate::difftest::elf_image(c);
        crate::tick::set_for_test(c, hz);
        let (screen, thread, list) = (base, base + 0x80, base + 0x100);
        // Eight empty keyframes (frames 0..7): the thread steps regardless.
        c.bus.write32(list, 8);
        let mut at = list + 4;
        for i in 0..8u32 {
            c.bus.write32(at + 4, 0xc);
            c.bus.write16(at + 8, i as u16);
            c.bus.write16(at + 0xa, 0);
            at += 0xc;
        }
        c.bus.write32(screen + 0x34, list);
        c.bus.write16(thread + 0xc, 0);
        c.bus.write16(thread + 0xe, 0xffff);
        c.bus.write16(thread + 0x10, 0xffff);
        c.bus.write8(thread + 0x12, step as u8);
        // Held: the walk advances past the thread without unlinking it, so
        // it clocks the widgets on every tick.
        c.bus.write8(thread + 0x13, 1);
        c.bus.write32(thread, 0);
        let sentinel = base + 0x900;
        c.bus.write32(sentinel + 4, sentinel);
        c.bus.write32(thread + 4, sentinel);
        // The removal path calls a cleanup through a fabricated vtable:
        // a safe read of the thread's next link.
        let cleanup = base + 0xa00;
        c.bus.write32(thread + 8, cleanup);
        c.bus.write16(cleanup + 8, 0);
        c.bus.write32(cleanup + 0xc, 0x0039_7718);
        c.bus.write32(screen + 0x18 + 4, thread);
        // The group walks one node per widget below.
        let group = screen + 0x40;
        let mut node = base + 0xb00;
        let mut prev = 0u32;
        for (w, (mode, start, count)) in widgets.iter().enumerate() {
            let wu = w as u32;
            let (widget, vt) = (base + 0x1000 + wu * 0x100, base + 0x2000 + wu * 0x100);
            let (anim, data, chans, chan) = (
                base + 0x3000 + wu * 0x200,
                base + 0x3400 + wu * 0x40,
                base + 0x3480 + wu * 0x40,
                base + 0x3500 + wu * 0x40,
            );
            c.bus.write32(widget + 8, vt);
            c.bus.write32(widget + 0xc, anim);
            c.bus.write8(widget + 0x10, *mode as u8);
            c.bus.write16(vt + 0x60, 0);
            c.bus.write32(vt + 0x64, 0x003a_0048);
            c.bus.write32(anim, data);
            c.bus.write32(anim + 4, chans);
            c.bus.write16(anim + 0x8, 0xffff);
            c.bus
                .write16(anim + 0xa, if *start == 0xffff { 0 } else { 0xffff });
            c.bus.write16(anim + 0xc, *start);
            c.bus.write8(anim + 0xe, 0);
            c.bus.write16(data + 2, *count);
            c.bus.write32(data + 4, 1);
            c.bus.write32(chans, chan);
            c.bus.write16(chan + 2, 0);
            // Two segments, (0,0)-(8,80)-(10,100): every integer frame
            // evaluates exactly in binary floating point, so the integer
            // parameter below converts without rounding artifacts. Keys
            // are (frame delta, absolute value).
            c.bus.write16(chan + 6, 3);
            c.bus.write16(chan + 0xa, 0);
            c.bus.write16(chan + 0xc, 8);
            c.bus.write16(chan + 0xe, 80);
            c.bus.write16(chan + 0x10, 2);
            c.bus.write16(chan + 0x12, 100);
            c.bus.write32(node, widget);
            c.bus.write32(node + 4, sentinel);
            c.bus.write32(node + 8, widget);
            // The widget walk calls the widget's own update with the node:
            // route it to the real per-object animation runner (0x3a0000)
            // with the node-relative offset to the widget.
            c.bus.write16(widget + 0x98, (widget - node) as u16);
            c.bus.write32(widget + 0x9c, 0x003a_0000);
            if prev != 0 {
                c.bus.write32(prev + 4, node);
            } else {
                c.bus.write32(group + 0x74 + 4, node);
            }
            prev = node;
            node += 0xa8;
        }
        c.bus.write32(group + 0xc, 0);
        (screen, thread)
    }

    /// One real `UIScreen_Update` tick, driven through event unwinds; the
    /// update returns to its caller.
    fn update_tick(c: &mut Cpu, screen: u32) {
        use crate::difftest::events::drive;
        c.g[A0][0] = screen as u64;
        c.g[RA][0] = sx(crate::difftest::RA);
        c.pc = 0x0039_d778;
        drive(c, ui_screen_update);
        assert_eq!(c.pc, crate::difftest::RA);
    }

    /// A per-object `frame60_due`/`ran` clock, mirroring `tick.rs`: the
    /// oracle for the rates below, not the wiring under test.
    struct OracleClock {
        frame: u32,
        credit: u32,
        live: bool,
    }

    impl OracleClock {
        fn due(&mut self, rate: u32, frame: u32) -> bool {
            if !self.live || self.frame != frame {
                self.frame = frame;
                self.credit = 0;
                self.live = true;
                return true;
            }
            self.credit += 60;
            if self.credit >= rate {
                self.credit -= rate;
                true
            } else {
                false
            }
        }

        fn ran(&mut self, frame: u32) {
            if self.live {
                self.frame = frame;
            }
        }

        fn phase(&self, frame: u32) -> u32 {
            if self.live && self.frame == frame {
                self.credit
            } else {
                0
            }
        }
    }

    /// Through the real screen/thread/widget path, widget values follow the
    /// authored line at 60 frames per second at every rate: the original
    /// frame values at 30/60, blended between frames above.
    #[test]
    fn ui_anim_full_path_follows_the_rate() {
        let _slots = Slots::install();
        // Forward: a timeline widget (mode 0) and an own-clock widget
        // (mode 1) at 30/60/120/144/240.
        for (hi, hz) in [30u32, 60, 120, 144, 240].iter().enumerate() {
            let base = DATA + 0x1_0000 * hi as u32;
            let mut c = crate::difftest::fresh();
            let (screen, thread) =
                full_path_fixture(&mut c, base, *hz, 1, &[(0, 0, 20), (1, 0xffff, 20)]);
            let (w0, w1) = (base + 0x1000, base + 0x1100);
            // Oracle thread state (+0xc / +0xe) and its host clock.
            let (mut f, mut last) = (0u32, 0xffffu32);
            let mut tclock = OracleClock {
                frame: 0,
                credit: 0,
                live: false,
            };
            let anim_a = base + 0x3200;
            for t in 0..12u32 {
                // The own-clock widget applies the frame its advance has
                // left in +0xa, then steps; the blend reads that same frame
                // beforehand, so sample both up front.
                let a_pre = c.bus.read16(anim_a + 0xa) as u32;
                let aphase = crate::tick::frame60_phase(&mut c, anim_a, a_pre);
                update_tick(&mut c, screen);
                // The thread: mirror of the walk plus Anders's gate.
                if f != last && tclock.due(*hz, f) {
                    last = f;
                    f += 1;
                    tclock.ran(f);
                }
                assert_eq!(c.bus.read16(thread + 0xc), f as u16, "{hz} Hz tick {t}");
                // The timeline widget: its frame, or blended towards the
                // next one above 60. The blend gate mirrors the port: the
                // thread is running, and its clock is mid-frame.
                let running = c.bus.read16(thread + 0xe) as u32 != f;
                let phase = crate::tick::frame60_phase(&mut c, thread, f);
                let eval = |x: u32| 10.0 * (x.min(10)) as f32;
                let want = if running && phase != 0 && f + 1 < 20 {
                    eval(f) + (eval(f + 1) - eval(f)) * phase as f32 / *hz as f32
                } else {
                    eval(f)
                };
                let got = f32::from_bits(c.bus.read32(w0 + 0x44));
                assert!(
                    (got - want).abs() < 1e-3,
                    "{hz} Hz tick {t}: {got} != {want}"
                );
                // The own-clock widget: the same through its own advance.
                // The blend runs before the advance steps, off the
                // pre-tick frame above.
                let awant = if aphase != 0 && a_pre + 1 < 20 {
                    eval(a_pre) + (eval(a_pre + 1) - eval(a_pre)) * aphase as f32 / *hz as f32
                } else {
                    eval(a_pre)
                };
                let agot = f32::from_bits(c.bus.read32(w1 + 0x44));
                assert!(
                    (agot - awant).abs() < 1e-3,
                    "{hz} Hz own tick {t}: {agot} != {awant}"
                );
            }
        }
        // Reverse at 120: back down the authored line, blending between.
        {
            let base = DATA + 0x6_0000;
            let mut c = crate::difftest::fresh();
            let (screen, thread) = full_path_fixture(&mut c, base, 120, -1, &[(0, 0, 20)]);
            c.bus.write16(thread + 0xc, 6);
            c.bus.write16(thread + 0xe, 5);
            let w0 = base + 0x1000;
            let (mut f, mut last) = (6u32, 5u32);
            let mut tclock = OracleClock {
                frame: 0,
                credit: 0,
                live: false,
            };
            let mut tapplied = 0xffffu32;
            for t in 0..10u32 {
                update_tick(&mut c, screen);
                if f != last && tclock.due(120, f) {
                    last = f;
                    f -= 1;
                    tclock.ran(f);
                }
                assert_eq!(c.bus.read16(thread + 0xc), f as u16, "rev tick {t}");
                let mut want = 10.0 * f as f32;
                if tapplied == f {
                    let phase = tclock.phase(f);
                    if phase != 0 && f >= 1 {
                        want = 10.0 * (f as f32 - phase as f32 / 120.0);
                    }
                }
                tapplied = f;
                let got = f32::from_bits(c.bus.read32(w0 + 0x44));
                assert!((got - want).abs() < 1e-3, "rev tick {t}: {got} != {want}");
            }
        }
        // Stop, restart, jump and wrap at 120: held frames keep their exact
        // values, and no blend ever spans a jump or a wrap.
        {
            let base = DATA + 0x7_0000;
            let mut c = crate::difftest::fresh();
            let (screen, thread) = full_path_fixture(&mut c, base, 120, 1, &[(0, 0, 20)]);
            let w0 = base + 0x1000;
            // Five ticks: the thread steps to 3 on the last one, so the
            // widget holds an exact frame value to freeze.
            for _ in 0..5u32 {
                update_tick(&mut c, screen);
            }
            let held = c.bus.read16(thread + 0xc);
            assert_eq!(held, 3, "thread steps first");
            let frozen = f32::from_bits(c.bus.read32(w0 + 0x44));
            assert!((frozen - 30.0).abs() < 1e-3, "step tick is exact: {frozen}");
            // Hold the thread: the widget freezes at its frame's value.
            c.bus.write16(thread + 0xe, held);
            for t in 0..3u32 {
                update_tick(&mut c, screen);
                assert_eq!(c.bus.read16(thread + 0xc), held, "held tick {t}");
                let got = f32::from_bits(c.bus.read32(w0 + 0x44));
                assert!((got - 30.0).abs() < 1e-3, "held tick {t}: {got}");
            }
            // Restart one frame back: motion resumes through the blend.
            c.bus.write16(thread + 0xe, held - 1);
            update_tick(&mut c, screen);
            assert_eq!(c.bus.read16(thread + 0xc), held, "restart holds the step");
            let got = f32::from_bits(c.bus.read32(w0 + 0x44));
            assert!((got - 35.0).abs() < 1e-3, "restart blends: {got}");
            // Jump the thread (as a goto would): the widget takes the new
            // frame's value at once, with no in-between.
            c.bus.write16(thread + 0xc, 7);
            c.bus.write16(thread + 0xe, 7);
            update_tick(&mut c, screen);
            assert_eq!(c.bus.read16(thread + 0xc), 7);
            let got = f32::from_bits(c.bus.read32(w0 + 0x44));
            assert!((got - 70.0).abs() < 1e-3, "jump: {got}");
            // Reverse past frame 0: the thread wraps to 0xffff, the values
            // hold, nothing blends across the boundary.
            c.bus.write16(thread + 0xc, 1);
            c.bus.write16(thread + 0xe, 0);
            c.bus.write8(thread + 0x12, (-1i8) as u8);
            update_tick(&mut c, screen);
            assert_eq!(c.bus.read16(thread + 0xc), 0, "steps to 0");
            update_tick(&mut c, screen);
            update_tick(&mut c, screen);
            // Either it wraps to 0xffff or it holds at 0: nothing blends
            // across the boundary either way.
            let wrapped = c.bus.read16(thread + 0xc);
            assert!(wrapped == 0 || wrapped == 0xffff, "boundary: {wrapped}");
            let got = f32::from_bits(c.bus.read32(w0 + 0x44));
            assert!((got - 0.0).abs() < 1e-3, "wrap holds 0: {got}");
        }
        // Parameter 12 is never blended: the integer bits step exactly.
        {
            let base = DATA + 0x8_0000;
            let mut c = crate::difftest::fresh();
            let (screen, thread) = full_path_fixture(&mut c, base, 120, 1, &[(0, 0, 20)]);
            // Retarget the one channel at parameter 12.
            c.bus.write16(base + 0x3500 + 2, 12);
            let w0 = base + 0x1000;
            for t in 0..8u32 {
                update_tick(&mut c, screen);
                let f = c.bus.read16(thread + 0xc) as u32;
                let bits = (c.bus.read32(w0 + 0x14) >> 13) & 0x3f;
                assert_eq!(bits, 10 * f.min(10), "param 12 tick {t}");
            }
        }
    }

    /// An event unwinding a screen update before its widgets are applied, with
    /// the port re-entered at the walk (as `drive` does; in the game the
    /// generated body resumes), does not lose the screen: after the resume
    /// the timeline blend still finds it, so resumed ticks match
    /// uninterrupted ones exactly.
    #[test]
    fn ui_screen_update_resumed_before_widgets() {
        use crate::difftest::events::drive;
        let _slots = Slots::install();
        let base = DATA + 0x9_0000;
        let mut c = crate::difftest::fresh();
        let (screen, thread) = full_path_fixture(&mut c, base, 120, 1, &[(0, 0, 20)]);
        let w0 = base + 0x1000;
        let (mut f, mut last) = (0u32, 0xffffu32);
        let mut tclock = OracleClock {
            frame: 0,
            credit: 0,
            live: false,
        };
        let mut tapplied = 0xffffu32;
        for t in 0..6u32 {
            // Start the update with a VBlank already due, so the walk's
            // back edge unwinds before any thread runs; drive the update
            // through the resume to completion.
            c.g[A0][0] = screen as u64;
            c.g[RA][0] = sx(crate::difftest::RA);
            c.pc = 0x0039_d778;
            let first = c.cycles;
            c.next_event = c.cycles;
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 0;
            drive(&mut c, ui_screen_update);
            assert_ne!(c.next_event, first, "tick {t}: an event delivered");
            if f != last && tclock.due(120, f) {
                last = f;
                f += 1;
                tclock.ran(f);
            }
            assert_eq!(c.bus.read16(thread + 0xc), f as u16, "tick {t}");
            let mut want = 10.0 * f as f32;
            if tapplied == f {
                let phase = tclock.phase(f);
                if phase != 0 && f + 1 < 20 {
                    want = 10.0 * (f as f32 + phase as f32 / 120.0);
                }
            }
            tapplied = f;
            let got = f32::from_bits(c.bus.read32(w0 + 0x44));
            assert!((got - want).abs() < 1e-3, "tick {t}: {got} != {want}");
        }
    }

    /// In the game an update that an event unwinds is not re-entered: the
    /// generated body finishes it (the unwind form's slot stub runs every
    /// entry but the start there), so the port never clears its screen. The
    /// screen counts within the frame (a thread switch resumes it there) and,
    /// in the unwind form, never after the VSync that unwound it, however
    /// many updates were cut off; a whole update afterwards still blends and
    /// leaves no screen.
    #[test]
    fn ui_screen_update_unwound_leaves_no_stale_screen() {
        use crate::difftest::fakes::{FAKE, LOG_COUNT, LOG_UNWIND_AT};
        let _slots = Slots::install();
        let base = DATA + 0xa_0000;
        let mut c = crate::difftest::fresh();
        let (screen, thread) = full_path_fixture(&mut c, base, 120, 1, &[(0, 0, 20)]);
        // The app manager (`tick::set_for_test`) holds the VSync count.
        let app = c.bus.read32(crate::tick::APPMAN_PTR);
        // The widget's parameter setter: a fake that unwinds on the first
        // fake call, inside the group update.
        let vt = base + 0x2000;
        let setter = c.bus.read32(vt + 0x64);
        c.bus.write32(vt + 0x64, FAKE);
        let anim = base + 0x3000;
        for t in 0..4u32 {
            // A frame not yet applied: the original's channel loop runs.
            c.bus.write16(anim + 0x8, 0xffff);
            c.bus.write16(anim + 0xa, 0xffff);
            c.bus.write32(LOG_COUNT, 0);
            c.bus.write32(LOG_UNWIND_AT, 0);
            c.g[A0][0] = screen as u64;
            c.g[RA][0] = sx(crate::difftest::RA);
            c.pc = 0x0039_d778;
            ui_screen_update(&mut c);
            assert_eq!(c.pc & 0xffff_0000, 0xdead_0000, "tick {t}: unwound");
            assert_eq!(
                updating_screen(&mut c),
                Some(screen),
                "tick {t}: same frame"
            );
            // The VBlank that unwinds the thread runs `App_VSyncTick` first.
            let vsync = c.bus.read32(app + crate::tick::APPMAN_VSYNC) + 1;
            c.bus.write32(app + crate::tick::APPMAN_VSYNC, vsync);
            let unwind_form = slot_screen(UPDATING.get(), true, vsync);
            assert_eq!(unwind_form, None, "tick {t}: stale screen");
        }
        c.bus.write32(vt + 0x64, setter);
        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        // Between two steps the widget takes an in-between value again.
        let mut blended = false;
        for _ in 0..4u32 {
            update_tick(&mut c, screen);
            assert_eq!(updating_screen(&mut c), None, "after a whole update");
            let f = c.bus.read16(thread + 0xc) as f32;
            let got = f32::from_bits(c.bus.read32(base + 0x1000 + 0x44));
            blended |= (got - 10.0 * f).abs() > 1e-3;
        }
        assert!(blended, "the timeline widget blends again");
    }

    #[test]
    fn uiscreen_find_keyframe_matches() {
        check(
            ee::s13::f_0039c728__generated,
            uiscreen_find_keyframe,
            80,
            &|c, seed| {
                let mut r = Rng(0x39c7_2811 ^ seed as u64);
                scramble(c, &mut r);
                let screen = DATA;
                let list = DATA + 0x100;
                let n = 1 + seed % 4;
                c.bus.write32(list, n);
                let mut at = list + 4;
                for i in 0..n {
                    c.bus.write32(at + 4, 0x10);
                    c.bus.write16(at + 8, (i * 7 % 5) as u16);
                    fill(c, &mut r, at, 4);
                    at += 0x10;
                }
                c.bus.write32(screen + 0x34, list);
                c.g[A0][0] = sx(screen);
                // 0..4 hits an entry, 5 never does.
                c.g[A1][0] = ((seed / 4) % 6) as u64;
            },
        );
    }

    // ---------------------------------------------------------------- UI

    #[test]
    fn ui_screen_cmd_goto_label_matches() {
        check(
            ssx3_ee::s13::f_0039cbe0__generated,
            ui_screen_cmd_goto_label,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x39cbe0, 0);
                let screen = DATA;
                let list = DATA + 0x100;
                let key = DATA + 0x200;
                let out = DATA + 0x300;
                let n = (seed % 4) as usize;
                c.bus.write32(screen + 0x34, list);
                c.bus.write32(list, n as u32);
                let mut labels = [0u32; 4];
                for i in 0..n {
                    labels[i] = 0x1000 + 0x11 * i as u32;
                    let e = list + 4 + 0x20 * i as u32;
                    c.bus.write32(e, labels[i]);
                    c.bus.write32(e + 4, 0x20);
                    c.bus.write16(e + 8, 0x200 + 3 * i as u16);
                }
                let k = if n > 0 && seed % 2 == 0 {
                    labels[(seed as usize / 2) % n]
                } else {
                    0xdead_0000
                };
                c.bus.write32(key + 4, k);
                c.g[A0][0] = screen as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = key as u64;
                c.g[A3][0] = out as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI.

    #[test]
    fn ui_screen_find_object_matches() {
        check(
            ee::s13::f_0039d860__generated,
            ui_screen_find_object,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x39d8_60);
                let screen = DATA + 0x100;
                clear(c, screen, 0x40);
                let count = DATA + 0x200;
                let objs = DATA + 0x300;
                clear(c, count, 4);
                clear(c, objs, 0x40);
                let hash = r.next_u32();
                if seed % 7 != 0 {
                    let n = if seed % 5 == 0 { 0 } else { 1 + seed % 4 } as u32;
                    w(c, count, n);
                    w(c, screen + 0x38, count as u32);
                    w(c, screen + 0x3c, objs as u32);
                    for i in 0..n {
                        let o = DATA + 0x400 + 0x40 * i;
                        clear(c, o, 0x20);
                        if (seed >> (i + 1)) & 1 == 0 {
                            w(c, objs + 4 * i, o);
                            w(
                                c,
                                o + 0x38,
                                if i == n - 1 && seed % 3 == 0 {
                                    hash
                                } else {
                                    r.next_u32()
                                },
                            );
                        } else {
                            w(c, objs + 4 * i, 0);
                        }
                    }
                }
                c.g[A0][0] = sx(screen);
                c.g[A1][0] = hash as u64;
            },
        );
    }

    #[test]
    fn ui_screen_get_primary_thread_matches() {
        check(
            ssx3_ee::s13::f_0039c7c0__generated,
            ui_screen_get_primary_thread,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x39c7c0, 0);
                let l = OBJ + 0x18;
                if seed % 3 == 0 {
                    w(c, l + 4, 0);
                } else {
                    let n0 = OBJ2;
                    let n1 = OBJ2 + 0x40;
                    let sentinel = OBJ2 + 0x80;
                    w(c, l + 4, n0);
                    w(c, n0, n0.wrapping_add(0x100));
                    w(c, n0 + 4, n1);
                    w(c, n1, n1.wrapping_add(0x100));
                    w(c, n1 + 4, sentinel);
                    w(c, sentinel, sentinel);
                    w(c, sentinel + 4, sentinel);
                    let hidden = if seed % 2 == 0 { 1u32 << 25 } else { 0 };
                    w(c, n0 + 0x10, hidden);
                    w(c, n1 + 0x10, if seed % 5 == 1 { 1 << 25 } else { 0 });
                }
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn ui_screen_cmd_set_object_param_matches() {
        check(
            ssx3_ee::s13::f_0039cd30__generated,
            ui_screen_cmd_set_object_param,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39cd30, 0);
                let name = OBJ3 + 0x100;
                let count = OBJ2 + 0x800;
                for (i, b) in b"button\0".iter().enumerate() {
                    c.bus.write8(name + i as u32, *b);
                }
                if seed % 2 == 0 {
                    // One entry whose name is `name`: the find loop matches on
                    // its first iteration (`screen+0x3c` is the entry array,
                    // `screen+0x38` holds the count and the first entry).
                    w(c, OBJ + 0x3c, OBJ2);
                    w(c, OBJ2, OBJ3);
                    w(c, OBJ + 0x38, count);
                    w(c, count, 1);
                    w(c, OBJ3 + 0x38, name);
                } else {
                    // No entry array at all: the find returns null at once.
                    w(c, OBJ + 0x3c, 0);
                    w(c, OBJ + 0x38, 0);
                }
                w(c, OBJ3 + 0x8, VT);
                slot(c, VT, 0x68, 0);
                w(c, OBJ4 + 0x4, name);
                w(c, OBJ4 + 0x8, r.next_u32());
                w(c, OBJ4 + 0xc, r.next_u32());
                c.g[A0][0] = OBJ as u64;
                c.g[A2][0] = OBJ4 as u64;
            },
        );
    }

    #[test]
    fn ui_screen_cmd_start_thread_matches() {
        check(
            ee::s13::f_0039cc38__generated,
            ui_screen_cmd_start_thread,
            60,
            &|c, seed| {
                alloc_base(c, seed, 0x39cc_38);
                let list = DATA + 0x300;
                c.g[A0][0] = sx(DATA);
                c.g[A2][0] = sx(DATA + 0x200);
                let count = seed % 3;
                c.bus.write32(DATA + 0x34, list);
                c.bus.write32(list, count);
                c.bus.write32(DATA + 0x200 + 4, 1);
                for i in 0..count {
                    let e = list + 4 + i * 0x20;
                    let label = if seed % 2 == 0 { 1 } else { 2 };
                    c.bus.write32(e, label);
                    c.bus.write32(e + 4, 0x20);
                    c.bus.write16(e + 8, 1);
                }
            },
        );
    }

    #[test]
    fn ui_object_set_anim_matches() {
        check(
            ee::s13::f_0039fcc8__generated,
            ui_object_set_anim,
            60,
            &|c, seed| {
                machine(c, seed, 0x39fc_c8, 0);
                let anim = DATA + 0x2000;
                let old = DATA + 0x2400;
                let chain = DATA + 0x3000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(anim);
                c.g[A2][0] = (seed % 4) as u64;
                c.g[A3][0] = rng_word(seed) as u64 & 0xffff;
                c.bus
                    .write32(DATA + 0xc, if seed % 2 == 0 { old } else { anim });
                c.bus.write32(anim, chain);
                c.bus.write16(chain + 2, 5);
            },
        );
    }

    #[test]
    fn ui_state_ctor_matches() {
        check(
            ee::s13::f_0039e2a0__generated,
            ui_state_ctor,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x39_e2a0, 2);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.pc = 0x0039_e2a0;
            },
        );
    }

    #[test]
    fn ui_state_ctor_2_matches() {
        check(
            ee::s13::f_0039e318__generated,
            ui_state_ctor_2,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x39e318, 2);
                be_library_2(c);
                world(c);
                heap_init_2(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0x900) as u64;
            },
        );
    }

    /// `UIListNode_IsSentinel`: returns 0 for a node whose `+4` and `+0` are
    /// not the node itself. Used as a "returns 0" stub behind a vtable slot_2.
    pub(crate) const RET_ZERO: u32 = 0x0039_7718;

    // -----------------------------------------------------------------------
    // UI screens and states.

    #[test]
    fn uiscreen_cmd_send_event_matches() {
        covered(
            ee::s13::f_0039cda0__generated,
            uiscreen_cmd_send_event,
            SEEDS_6,
            &|c, seed| {
                base_3(c, seed, 0xcda0, 2);
                let s0 = DATA + 0x100;
                let s1 = DATA + 0x180;
                let ctrl = DATA + 0x200;
                c.g[A0][0] = sx(s0);
                c.g[A2][0] = sx(s1);
                c.bus.write32(s1 + 4, 0x99);
                c.bus.write32(s0 + 0xd0, ctrl);
                c.bus.write32(ctrl, 0);
                let vt1 = DATA + 0x400;
                slot_2(c, vt1, 0x80, 0, if seed % 2 == 0 { FAKE } else { RET_ZERO });
                c.bus.write32(ctrl + 8, vt1);
                let holder = DATA + 0x300;
                c.bus.write32(ctrl + 0x10, holder);
                let obj2 = DATA + 0x340;
                c.bus.write32(holder, if seed % 3 == 0 { 0 } else { obj2 });
                let vt2 = DATA + 0x440;
                slot_2(c, vt2, 0x18, 0, FAKE);
                c.bus.write32(obj2 + 4, vt2);
            },
        );
    }

    // ------------------------------------------------------------------
    // UI commands.

    #[test]
    fn ui_screen_cmd_play_anim_matches() {
        check(
            ssx3_ee::s13::f_0039cca8__generated,
            ui_screen_cmd_play_anim,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39cca8, 4);
                let screen = DATA + 0x100;
                let p1 = DATA + 0x200;
                let msg = DATA + 0x300;
                c.g[A0][0] = screen as u64;
                c.g[A1][0] = p1 as u64;
                c.g[A2][0] = msg as u64;
                let hash = r.next_u32();
                w_3(c, msg + 8, hash);
                w_3(c, msg + 4, r.next_u32());
                b(c, msg + 0xc, (r.next_u32() % 4) as u8);
                h(c, p1 + 8, (r.next_u32() % 0x10000) as u16);
                // The screen's object array.
                if seed % 3 == 0 {
                    w_3(c, screen + 0x38, 0);
                    w_3(c, screen + 0x3c, 0);
                } else {
                    let obj = DATA + 0x400;
                    let objs = DATA + 0x500;
                    w_3(c, objs, obj);
                    w_3(c, screen + 0x38, objs);
                    w_3(c, screen + 0x3c, 1);
                    w_3(c, obj + 0x38, hash);
                }
                // The animation bank: an empty list makes CreateAnim return null.
                let anim_set = DATA + 0x600;
                let bank = DATA + 0x650;
                w_3(c, screen + 0xd0, anim_set);
                w_3(c, anim_set + 0x10, bank - 0x50);
                w_3(c, bank, DATA + 0x700);
                w_3(c, DATA + 0x700, 0);
            },
        );
    }

    #[test]
    fn c_ui_scroll_dtor_matches() {
        check(
            ssx3_ee::s13::f_0039d8c0__generated,
            c_ui_scroll_dtor,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39d8c0, 3);
                be_library_3(c);
                world(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                w_3(c, obj + 0x74, 0);
                w_3(c, obj + 0x78, 0);
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
    // UI.

    #[test]
    fn ui_screen_set_data_matches() {
        check(
            ssx3_ee::s13::f_0039c688__generated,
            ui_screen_set_data,
            10,
            &|c, seed| {
                let _r = base_4(c, seed, 0x39c688, 3);
                let this = DATA + 0x1000;
                let data = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = data as u64;
                c.bus.write8(data, 0);
                c.bus.write8(data + 1, 0);
            },
        );
    }

    #[test]
    fn ui_state_render_matches() {
        check(
            ssx3_ee::s13::f_0039e6b8__generated,
            ui_state_render,
            20,
            &|c, seed| {
                let _r = base_4(c, seed, 0x39e6b8, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                let list = this + 0x24;
                let sentinel = DATA + 0x5000;
                w(c, sentinel, sentinel);
                w(c, sentinel + 4, sentinel);
                if seed % 3 == 0 {
                    w(c, list + 4, sentinel);
                } else {
                    let node = DATA + 0x2000;
                    w(c, list + 4, node);
                    w(c, node, sentinel);
                    w(c, node + 4, sentinel);
                    w(c, node + 0x54, if seed % 2 == 0 { 0 } else { 1 << 6 });
                    w(c, node + 0x48, DATA + 0x3000);
                    w(c, DATA + 0x3000, DATA + 0x3100);
                    c.bus.write16(DATA + 0x3100 + 0x88, 0x40);
                    w(c, DATA + 0x3100 + 0x8c, 0);
                }
                w(c, this + 8, DATA + 0x4000);
                w(c, DATA + 0x4000, DATA + 0x4100);
                c.bus.write16(DATA + 0x4100 + 0x70, 0);
                w(c, DATA + 0x4100 + 0x74, 0);
            },
        );
    }

    // ------------------------------------------------------------- UI dtor

    #[test]
    fn ui_state_dtor_matches() {
        check(
            ee::s13::f_0039e390__generated,
            ui_state_dtor,
            SEEDS_8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x39e3_90, 4);
                // A real heap object, so the `flags & 1` free is valid.
                let obj = alloc(c, 0x60);
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                // The two lists and the node destructor state.
                w_4(c, obj + 0x24, 0);
                w_4(c, obj + 0x28, 0);
                w_4(c, obj + 0x30, 0);
                w_4(c, obj + 0x34, 0);
            },
        );
    }

    // ------------------------------------------------------------------- UI

    #[test]
    fn cui_scroll_render_matches() {
        check(
            ee::s13::f_0039d968__generated,
            cui_scroll_render,
            8,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x0039_d968, 2);
                c.g[A0][0] = OBJ_2 as u64;
                let thumb = seed & 1 != 0;
                w_5(c, OBJ_2 + 0x74, if thumb { OBJ2_2 } else { 0 });
                w_5(c, OBJ_2 + 0x78, if seed % 3 == 0 { 0 } else { OBJ3_2 });
                w_5(c, OBJ_2 + 0x44, r.edge_f32());
                w_5(c, OBJ_2 + 0x48, r.edge_f32());
                for (obj, off) in [(OBJ2_2, 0x14u32), (OBJ3_2, 0x14u32)] {
                    let flags = if seed % 4 == 0 { 0 } else { 0x40 };
                    w_5(c, obj + off, flags);
                    let vt = vtable(c, &mut r, &[0x88]);
                    w_5(c, obj + 8, vt);
                }
            },
        );
    }

    #[test]
    fn ui_scroll_set_parts_visible_matches() {
        strict(
            ee::s13::f_0039de68__generated,
            ui_scroll_set_parts_visible,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39de, 12);
                let this = DATA;
                let vt = DATA + 0x1000;
                slot_5(c, vt, 0x48, FAKE);
                for (i, off) in [0x7c, 0x80, 0x84, 0x78, 0x74].into_iter().enumerate() {
                    let obj = DATA + 0x2000 + 0x100 * i as u32;
                    c.bus.write32(
                        this + off,
                        if (seed as usize >> i) & 1 == 0 {
                            0
                        } else {
                            obj
                        },
                    );
                    c.bus.write32(obj + 8, vt);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 1);
            },
        );
    }

    #[test]
    fn ui_screen_dtor_matches() {
        check3(
            ee::s13::f_0039c558__generated,
            ui_screen_dtor,
            40,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x39c558);
                let obj = DATA;
                // Zeroed screen: no names to free, empty lists.
                for i in 0..0x100u32 {
                    c.bus.write32(obj + 4 * i, 0);
                }
                c.bus.write32(obj + 0x14, (r.next_u32() % 4) as u32);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    pub(crate) use crate::game::object::movenode::tests::oc383_vtable;
    pub(crate) use crate::game::ui::uiengine::tests::oc383_ui_list;
    use std::sync::atomic::AtomicUsize;

    /// Point vtable slot `off` at a real leaf returning 0 (fakes always
    /// return non-zero).
    pub(crate) fn oc383_slot_zero(c: &mut Cpu, vt: u32, off: u32) {
        c.bus.write32(vt + off + 4, 0x0014_06c0);
    }

    #[test]
    fn ui_state_update_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s13::f_0039e510__generated,
            ui_state_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0xe510, 8);
                c.pc = 0x0039_e510;
                let this = DATA;
                let (owner, pad) = (DATA + 0x100, DATA + 0x200);
                let (state_vt, pad_vt, screen_vt) = (DATA + 0x800, DATA + 0x900, DATA + 0xa00);
                fill(c, &mut r, this, 0x10);
                c.bus.write32(this + 8, state_vt);
                c.bus.write32(this + 0x10, owner);
                c.bus.write32(owner + 0xc, pad);
                c.bus.write32(pad + 8, pad_vt);
                oc383_vtable(c, &mut r, state_vt, &[0x40, 0x48]);
                oc383_vtable(c, &mut r, pad_vt, &[0x160, 0x168, 0x148, 0x140]);
                oc383_vtable(c, &mut r, screen_vt, &[0x20]);
                let bits = r.next_u32();
                for (i, off) in [0x168u32, 0x148, 0x140].into_iter().enumerate() {
                    if bits >> i & 1 == 0 {
                        oc383_slot_zero(c, pad_vt, off);
                    }
                }
                if bits & 8 == 0 {
                    oc383_slot_zero(c, state_vt, 0x40);
                }
                if bits & 16 == 0 {
                    oc383_slot_zero(c, screen_vt, 0x20);
                }
                // Screens: 0..3 nodes, some active (+0x14 bit 0).
                let n = (seed % 4) as usize;
                let nodes: Vec<u32> = (0..n as u32).map(|i| DATA + 0x1000 + 0x100 * i).collect();
                for &node in &nodes {
                    fill(c, &mut r, node, 8);
                    c.bus.write32(node + 8, screen_vt);
                }
                oc383_ui_list(c, &mut r, this + UI_STATE_SCREENS, &nodes, DATA + 0x1800);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// Gate word for this test's fakes, and the find results they return.
    pub(crate) const UCS_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const UCS_MAGIC: u32 = 0x0c38_5d07;

    pub(crate) const UCS_FIND_NEXT: u32 = UCS_CTRL + 4;

    pub(crate) const UCS_FIND_RESULTS: u32 = UCS_CTRL + 0x10;

    /// Where the fake `operator new` puts every object.
    pub(crate) const UCS_NEW_OBJ: u32 = DATA + 0x1_0000;

    pub(crate) const UCS_REC: u32 = DATA + 0x2_0000;

    pub(crate) const UCS_CHILDREN: u32 = DATA + 0x3_0000;

    pub(crate) const UCS_SCREEN: u32 = DATA + 0x4_0000;

    /// The vtable the faked constructors leave in the object.
    pub(crate) const UCS_FAKE_VT: u32 = DATA + 0x5_0000;

    /// The jump table's real entries (types 0x10..=0x21).
    pub(crate) const UCS_JUMP: [u32; 18] = [
        0x0039_d124,
        0x0039_d0cc,
        0x0039_d1c4,
        0x0039_d1f4,
        0x0039_d730,
        0x0039_d2c4,
        0x0039_d554,
        0x0039_d60c,
        0x0039_d6e4,
        0x0039_d3d4,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d63c,
        0x0039_d488,
    ];

    pub(crate) const UCS_K_PLAIN: u32 = 0;

    pub(crate) const UCS_K_ALLOC: u32 = 1;

    pub(crate) const UCS_K_THIS: u32 = 2;

    pub(crate) const UCS_K_FIND: u32 = 3;

    /// The logging fake, then a result by kind: `new` returns the fixed
    /// object, constructors return `this`, finds the next fixture result.
    pub(crate) fn ucs_fake(c: &mut Cpu, addr: u32, kind: u32) {
        let this = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match kind {
            UCS_K_ALLOC => c.g[V0][0] = sx(UCS_NEW_OBJ),
            UCS_K_THIS => c.g[V0][0] = this,
            UCS_K_FIND => {
                let i = c.bus.read32(UCS_FIND_NEXT);
                c.bus.write32(UCS_FIND_NEXT, i + 1);
                let found = c.bus.read32(UCS_FIND_RESULTS + 4 * (i & 0xf));
                c.g[V0][0] = sx(found);
            }
            _ => {}
        }
    }

    macro_rules! ucs_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident $kind:ident;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(UCS_CTRL) != UCS_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    ucs_fake(c, $addr, $kind)
                }
            )*
            fn ucs_install_fakes() {
                $(
                    fakes::hook(std::ptr::addr_of_mut!(ee::$shard::$slot), $name, &$prev);
                )*
            }
        };
    }

    ucs_fakes! {
        ucs_f_alloc UCS_P_ALLOC 0x0031_7d70 s10 OVERRIDE_00317D70 f_00317d70__generated UCS_K_ALLOC;
        ucs_f_list_ctor UCS_P_LIST_CTOR 0x0039_77e8 s13 OVERRIDE_003977E8 f_003977e8__generated UCS_K_PLAIN;
        ucs_f_find UCS_P_FIND 0x0039_d860 s13 OVERRIDE_0039D860 f_0039d860__generated UCS_K_FIND;
        ucs_f_unlink UCS_P_UNLINK 0x0039_7740 s13 OVERRIDE_00397740 f_00397740__generated UCS_K_PLAIN;
        ucs_f_push_back UCS_P_PUSH_BACK 0x0039_7910 s13 OVERRIDE_00397910 f_00397910__generated UCS_K_PLAIN;
        ucs_f_menu_flags UCS_P_MENU_FLAGS 0x003a_5338 s13 OVERRIDE_003A5338 f_003a5338__generated UCS_K_PLAIN;
        ucs_f_pair_flags UCS_P_PAIR_FLAGS 0x003a_5508 s13 OVERRIDE_003A5508 f_003a5508__generated UCS_K_PLAIN;
        ucs_f_text UCS_P_TEXT 0x003a_0348 s13 OVERRIDE_003A0348 f_003a0348__generated UCS_K_THIS;
        ucs_f_text_scroll UCS_P_TEXT_SCROLL 0x003a_1148 s13 OVERRIDE_003A1148 f_003a1148__generated UCS_K_THIS;
        ucs_f_vector UCS_P_VECTOR 0x003a_3280 s13 OVERRIDE_003A3280 f_003a3280__generated UCS_K_THIS;
    }

    /// Every record type (and unknown ones), child loops of 0..4 entries,
    /// finds that miss, hit children, or hit a pair's own part slots (so the
    /// old part's deleting destructor runs), and unwinds at random calls.
    /// `new` and the constructors are fakes; `UIObject_ctor`, the list box
    /// constructor (with its real memset) and the icon handle run for real.
    #[test]
    fn ui_screen_create_object_matches() {
        ucs_install_fakes();
        check(
            ee::s13::f_0039d078__generated,
            ui_screen_create_object,
            300,
            &|c, seed| {
                ucs_install_fakes();
                let mut r = fakes::machine(c, seed, 0x0039_d078, 14);
                c.bus.write32(UCS_CTRL, UCS_MAGIC);
                c.bus.write32(UCS_FIND_NEXT, 0);
                for i in 0..16u32 {
                    let found = match r.next_u32() % 7 {
                        0 | 1 => 0,
                        2..=5 => UCS_CHILDREN + 0x100 * (r.next_u32() % 4),
                        _ => UCS_NEW_OBJ + 0x38 + 4 * (i & 1),
                    };
                    c.bus.write32(UCS_FIND_RESULTS + 4 * i, found);
                }
                for (i, &e) in UCS_JUMP.iter().enumerate() {
                    c.bus.write32(UCS_TABLE + 4 * i as u32, e);
                }
                // initFromRec and the deleting destructor in every vtable
                // the object can carry.
                let vts = [
                    UCS_VT_ICON,
                    UCS_VT_GROUP,
                    UCS_VT_MENU,
                    UCS_VT_PAIR,
                    UCS_VT_PROGRESS,
                    UCS_VT_21,
                    UCS_VT_16,
                    0x0049_4798,
                    UCS_FAKE_VT,
                ];
                for (i, &vt) in vts.iter().enumerate() {
                    let i = i as u32;
                    c.bus
                        .write16(vt + UCS_VT_INIT, (r.next_u32() & 0xfc) as u16);
                    c.bus.write32(vt + UCS_VT_INIT + 4, FAKE + 0x20 * i);
                    c.bus
                        .write16(vt + UCS_VT_DELETE, (r.next_u32() & 0xfc) as u16);
                    c.bus
                        .write32(vt + UCS_VT_DELETE + 4, FAKE + 0x20 * i + 0x10);
                }
                fill(c, &mut r, UCS_NEW_OBJ, 0xd0);
                c.bus.write32(UCS_NEW_OBJ + UCS_OBJ_VTABLE, UCS_FAKE_VT);
                fill(c, &mut r, UCS_CHILDREN, 0x100);
                fill(c, &mut r, UCS_REC, 0x10);
                let ty: u32 = match seed % 21 {
                    k @ 0..=17 => 0x10 + k,
                    18 => 0x22,
                    19 => 0x3,
                    _ => 0xfff0,
                };
                c.bus.write16(UCS_REC, ty as u16);
                c.bus.write32(UCS_REC + 0x20, (seed / 21) % 5);
                if ty == 0x20 {
                    c.bus.write8(UCS_REC + 0x22, ((seed / 7) % 5) as u8);
                }
                c.g[A0][0] = sx(UCS_SCREEN);
                c.g[A1][0] = sx(UCS_REC);
                c.pc = 0x0039_d078;
            },
        );
    }
}
