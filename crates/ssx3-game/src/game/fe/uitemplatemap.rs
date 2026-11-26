//! `src/fe/uitemplatemap.cpp`: hand-written replacements (moved from the
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
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_m2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::gp;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::festateload::event_select_notify;
pub(crate) use crate::game::fe::festateruleselect::POPUP_ACTIVE;
pub(crate) use crate::game::fe::ovstatehud::oc738_copy_blocks;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::ssxappdtor::p;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_0ac0,
        event_select_on_activate,
        "EventSelect_OnActivate",
    ),
    (0x0020_0af0, event_select_shutdown, "EventSelect_Shutdown"),
    (
        0x0020_0d00,
        event_select_is_event_unlocked,
        "EventSelect_IsEventUnlocked",
    ),
    (
        0x0020_1868,
        event_select_hide_map_pic,
        "EventSelect_HideMapPic",
    ),
    (
        0x0020_18a8,
        event_select_on_widget_action,
        "EventSelect_OnWidgetAction",
    ),
    (
        0x0020_2738,
        event_select_on_widget_notify,
        "EventSelect_OnWidgetNotify",
    ),
    (
        0x0020_4ff0,
        event_select_get_medal_text,
        "EventSelect_GetMedalText",
    ),
    (
        0x0020_83d8,
        event_select_set_highlight_color,
        "EventSelect_SetHighlightColor",
    ),
    (
        0x0020_8438,
        event_select_set_unlocked_color,
        "EventSelect_SetUnlockedColor",
    ),
    (0x0020_8498, ov_screen_map_init, "OVScreenMap_Init"),
    (
        0x0020_8558,
        ov_screen_map_on_activate,
        "OVScreenMap_OnActivate",
    ),
    (0x0020_8588, ov_screen_map_shutdown, "OVScreenMap_Shutdown"),
    (
        0x0020_8610,
        ov_screen_map_on_widget_notify,
        "OVScreenMap_OnWidgetNotify",
    ),
    (
        0x0020_8638,
        ov_screen_map_handle_input,
        "OVScreenMap_HandleInput",
    ),
    (0x0020_8678, ov_screen_map_update, "OVScreenMap_Update"),
    (
        0x0020_3990,
        event_select_load_map_image,
        "EventSelect_LoadMapImage",
    ),
    (
        0x0020_3cb8,
        event_select_set_map_tab_mode,
        "EventSelect_SetMapTabMode",
    ),
    (
        0x0020_09d0,
        event_select_on_widget_created,
        "EventSelect_OnWidgetCreated",
    ),
    (0x0020_2828, event_select_update, "EventSelect_Update"),
    (
        0x0020_2ad8,
        event_select_update_mode_panels,
        "EventSelect_UpdateModePanels",
    ),
    (0x0020_30a0, event_select_set_mode, "EventSelect_SetMode"),
    (
        0x0020_3d40,
        event_select_refresh_peak_list,
        "EventSelect_RefreshPeakList",
    ),
    (
        0x0020_0b50,
        event_select_is_event_completed,
        "EventSelect_IsEventCompleted",
    ),
    (0x0020_0700, event_select_init, "EventSelect_Init"),
    (
        0x0020_0a70,
        event_select_transition_in,
        "EventSelect_TransitionIn",
    ),
    (
        0x0020_1720,
        event_select_reset_flags,
        "EventSelect_ResetFlags",
    ),
    (
        0x0020_1750,
        event_select_init_peak_locks,
        "EventSelect_InitPeakLocks",
    ),
    (
        0x0020_2770,
        event_select_post_input,
        "EventSelect_PostInput",
    ),
    (
        0x0020_3a58,
        event_select_update_confirm_popup,
        "EventSelect_UpdateConfirmPopup",
    ),
];

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

pub(crate) mod callee {

    pub(crate) use crate::game::bx::bxstringctor::callee::C_OV_STATE_OVERLAY_UPDATE;

    pub(crate) use crate::game::bx::bxstringctor::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::bx::bxstringctor::callee::EVENT_SELECT_UPDATE;

    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_ON_EVENT_SELECT_ENTER;

    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SET_MODE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SHUTDOWN;

    pub(crate) use crate::game::main::ssxappdtor::callee::OV_STATE_OVERLAY_SHUTDOWN;

    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;

    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;

    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;

    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
}

/// Cross-shard calls the generator emitted as `call_indirect` (no alias).
pub(crate) mod table {

    pub(crate) use crate::game::movie::movieplayer::table::UI_STATE_ON_ACTIVATE;
}

// ---------------------------------------------------------------------------
// Menus / race progress / crowd.

/// `int EventSelect_OnWidgetNotify(EventSelect *this, int unused, int notify)`
/// (0x202738): maps widget notifications 6/9/8 to 0, anything else to 0x101.
pub fn event_select_on_widget_notify(c: &mut Cpu) {
    let notify = c.g[A2][0];
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = notify == c.g[V0][0];
    c.g[V0][0] = 9;
    if !is_six {
        c.cycles += 2;
        let is_nine = notify == c.g[V0][0];
        c.g[V0][0] = 8;
        if !is_nine {
            c.cycles += 2;
            if notify != c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0x101;
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void OVScreenMap_HandleInput(this)` (0x00208638):
/// `EventSelect_HandleInput(this + 0x9c)`.
pub fn ov_screen_map_handle_input(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c)),
        callee::EVENT_SELECT_HANDLE_INPUT,
        0x0020_8648,
    )
}

/// `int OVScreenMap_OnWidgetNotify(self, a1, notify)` (0x208610): the same
/// notify hook on the screen map's event-select member at `self+0x9c`.
pub fn ov_screen_map_on_widget_notify(c: &mut Cpu) {
    event_select_notify(c, 0x9c, 0x0020_8620)
}

// ---------------------------------------------------------------------------
// Overlay screens.

/// `OVScreenMap_OnActivate(this)` 0x208558: activate the screen's widget list
/// and switch the event select into map mode.
pub fn ov_screen_map_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        call_ptr(c, table::UI_STATE_ON_ACTIVATE, 0x0020_856c)?;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
        callee::EVENT_SELECT_ON_ACTIVATE.call(c, 0x0020_8574)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenMap_Update(this)` 0x208678: update the overlay's own state, then
/// the event-select widget embedded at +0x9c.
pub fn ov_screen_map_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::C_OV_STATE_OVERLAY_UPDATE.call(c, 0x0020_868c)?;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
        callee::EVENT_SELECT_UPDATE.call(c, 0x0020_8694)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_OnActivate(self)` (0x200ac0): put the map widget into
/// its current mode, then let the audio system react to entering event
/// select.
pub fn event_select_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(abi::r32(c, GP).wrapping_sub(0xb54));
        frame.save_ra(c, 0);
        call_ptr(c, callee::EVENT_SELECT_SET_MODE, 0x0020_0ad4)?;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x0020_0adc)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::AUDIO_ON_EVENT_SELECT_ENTER, 0x0020_0ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMap_Shutdown(self)` (0x208588): shut the map's event-select
/// widget (`self+0x9c`) down, then the overlay state.
pub fn ov_screen_map_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x9c));
        call_ptr(c, callee::EVENT_SELECT_SHUTDOWN, 0x0020_85a0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::OV_STATE_OVERLAY_SHUTDOWN, 0x0020_85a8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_HideMapPic(EventSelect* s)` (0x201868): hide the map
/// picture (`+0x2dc = 0`), flag `+0x2f0`, and call the picture object's
/// "hide" slot (+0x48) when there is one.
pub fn event_select_hide_map_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let this = crate::abi::arg(c, 0);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        Addr(p(this, 0x2f0)).sw(c, c.g[V0][0]);
        c.g[A2][0] = lw(c, this, 0x2e4);
        let missing = c.g[A2][0] == 0;
        Addr(p(this, 0x2dc)).sw(c, 0);
        if !missing {
            c.cycles += 6;
            let obj = c.g[A2][0];
            let vt = lw(c, obj as u32, 8);
            c.g[V0][0] = vt;
            c.g[A1][0] = 0;
            let m = VSlot::read(c, vt, 0x48);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            crate::call::call_ptr(c, m.func as u32, 0x0020_189c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMap_OnWidgetAction(OVScreenMap* s, ...)` (0x2085b8): forward
/// to the event-select widget and, when it accepted, show the map legend.
pub fn ov_screen_map_on_widget_action(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
    if callee::EVENT_SELECT_ON_WIDGET_ACTION
        .call(c, 0x0020_85d0)
        .is_err()
    {
        return;
    }
    c.g[V1][0] = 1;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 10;
        c.g[A0][0] = Addr(p_2(c, S0, 0x10)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        if call_ptr(c, 0x0039_f190, 0x0020_85ec).is_err() {
            return;
        }
        c.g[V1][0] = Addr(p_2(c, S0, 0x1c)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | 0x780;
        c.sw(p_2(c, S0, 0x1c), c.g[V1][0]);
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void EventSelect_Shutdown(EventSelect* self)` (0x200af0): close the
/// event-select screen and its PDA template.
pub fn event_select_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let self_ = c.g[A0][0];
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((self_ as u32).wrapping_add(0x2d8)).lw(c);
        c.g[A0][0] = Addr(r32_2(c, V0)).off(0x11c).lw(c);
        call_ptr(c, 0x001a_38c0, 0x0020_0b08)?;
        c.g[V1][0] = Addr(gp(c, -0xb5c)).lw(c);
        c.g[V0][0] = 0x1;
        let first = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        let mut close = first;
        if !first {
            c.cycles += 2;
            let second = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            close = second;
            if !second {
                c.cycles += 2;
                close = c.g[V1][0] == c.g[V0][0];
            }
        }
        if close {
            c.cycles += 2;
            call_ptr(c, 0x0020_d190, 0x0020_0b30)?;
        }
        c.cycles += 8;
        call_ptr(c, 0x0028_b180, 0x0020_0b38)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_f678, 0x0020_0b44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_SetHighlightColor(EventSelect* self, void* obj, void* c)`
/// (0x2083d8): tell the row object to use the highlighted colour.
pub fn event_select_set_highlight_color(c: &mut Cpu) {
    event_select_set_color(c, 0x74c8, 0x74d8, 0x0020_8408, 0x0020_842c)
}

/// `void EventSelect_SetUnlockedColor(EventSelect* self, void* obj, void* c)`
/// (0x208438): tell the row object to use the unlocked colour.
pub fn event_select_set_unlocked_color(c: &mut Cpu) {
    event_select_set_color(c, 0x74a8, 0x74b8, 0x0020_8468, 0x0020_848c)
}

/// The shared body of the two `EventSelect_Set*Color` functions: call the
/// row's virtual colour setter with the colour object (or with no argument
/// when it is null).
pub(crate) fn event_select_set_color(c: &mut Cpu, with: u32, without: u32, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let obj = c.g[A1][0];
        c.g[A3][0] = obj;
        let none = c.g[A3][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            {
                c.cycles += 2;
                let no_colour = c.g[A2][0] == 0;
                c.g[A1][0] = 0x4d_0000;
                if !no_colour {
                    c.cycles += 8;
                    c.g[V1][0] = Addr(at(c, A3, 0x8)).lw(c);
                    c.g[A1][0] = sx(0x004d_0000u32.wrapping_sub(with));
                    c.g[A0][0] = Addr(r32_2(c, V1)).off(0x58).lh(c);
                    c.g[V0][0] = Addr(r32_2(c, V1)).off(0x5c).lw(c);
                    let target = r32_2(c, V0);
                    c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
                    call_ptr(c, target, ret1)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
            }
            c.cycles += 7;
            c.g[V1][0] = Addr(at(c, A3, 0x8)).lw(c);
            c.g[A1][0] = 0x4d_0000;
            c.g[A1][0] = sx(0x004d_0000u32.wrapping_sub(without));
            c.g[A0][0] = Addr(r32_2(c, V1)).off(0x58).lh(c);
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0x5c).lw(c);
            let target = r32_2(c, V0);
            c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
            call_ptr(c, target, ret2)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void OVScreenMap_Init(self)` (0x00208498): create the map screen and
/// initialise the overlay's event select.
pub fn ov_screen_map_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x4a0000);
        c.g[A0][0] = sx(0x4a0000u32.wrapping_add(0x26e8));
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x0020_84bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x0020_84d0)?;
        let absent = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x0020_84e8)?;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x0020_84f0)?;
        c.g[A1][0] = Addr(at_2(c, S1, 0x40)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(word(c, S1).wrapping_add(0x9c));
        callee::EVENT_SELECT_INIT.call(c, 0x0020_8500)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* EventSelect_GetMedalText(int peak)` (0x204ff0): the medal label for
/// a peak, hashed into the world's UI text table.
pub fn event_select_get_medal_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let is1 = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let mut world: u64;
        let text: u64;
        if is1 {
            c.cycles += 4;
            world = Addr(gp(c, -0x848)).lw(c);
            text = sx(0x0047_0e20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
            let lt2 = c.g[V0][0] != 0;
            world = Addr(gp(c, -0x848)).lw(c);
            if lt2 {
                c.cycles += 3;
                text = sx(0x0047_0e10);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let is2 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if is2 {
                    c.cycles += 4;
                    world = Addr(gp(c, -0x848)).lw(c);
                    text = sx(0x0047_0e30);
                } else {
                    c.cycles += 2;
                    let is3 = c.g[A1][0] == c.g[V0][0];
                    world = Addr(gp(c, -0x848)).lw(c);
                    if is3 {
                        c.cycles += 2;
                        text = sx(0x0047_0e40);
                    } else {
                        c.cycles += 3;
                        text = sx(0x0047_0e10);
                    }
                }
            }
        }
        c.g[V0][0] = world;
        c.g[A0][0] = text;
        c.cycles += 14;
        c.g[S0][0] = Addr(p(r32(c, V0), 0x8c)).lw(c);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x20)).lh(c);
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V0)));
        call_ptr(c, 0x0031_7670, 0x00205070)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x24)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let get = r32(c, V1);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, get, 0x00205080)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EventSelect_IsEventUnlocked (0x0000200d00).

/// `EventSelect_IsEventUnlocked`: report whether the event-select entry is unlocked.
pub fn event_select_is_event_unlocked(c: &mut Cpu) {
    guest(c, |c| {
        'b_200dd0: {
            'b_200dcc: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S2][0] = c.g[A2][0];
                call_ptr(c, 0x0014dc80, 0x00200d20)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00200d2c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_200dcc;
                } else {
                    'b_200d74: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x4;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_200d74;
                        } else {
                            c.cycles += 10;
                            c.g[A0][0] = 0u64;
                            call_ptr(c, 0x0014a080, 0x00200d54)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A3][0] = c.g[S2][0];
                            c.g[A1][0] = 0u64;
                            c.g[T0][0] = 0x1;
                            call_ptr(c, 0x001464d0, 0x00200d6c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_200dd0;
                            } else {
                                break 'b_200d74;
                            }
                        }
                    }
                    'b_200da4: {
                        c.cycles += 2;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6));
                        if t {
                            break 'b_200da4;
                        } else {
                            c.cycles += 10;
                            c.g[A0][0] = 0u64;
                            call_ptr(c, 0x0014a080, 0x00200d84)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A3][0] = c.g[S2][0];
                            c.g[A1][0] = 0u64;
                            c.g[T0][0] = 0u64;
                            call_ptr(c, 0x001464d0, 0x00200d9c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_200dd0;
                            } else {
                                break 'b_200da4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_200dcc;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014a080, 0x00200db8)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A3][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00146a70, 0x00200dcc)?;
                        break 'b_200dcc;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_200dd0;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// EventSelect_OnWidgetAction (0x2018a8).

/// Callees of `EventSelect_OnWidgetAction`.
mod action {
    use super::ee;
    use super::Callee;
    pub(super) const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub(super) const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub(super) const BE_LIBRARY_SET_PHASE: u32 = 0x0014_de68;
    pub(super) const BE_NEW_RACE_INT_GET_TRACK_INFO: u32 = 0x0014_4bc0;
    pub(super) const BE_NEW_RACE_INT_SET_EVENT_TYPE: u32 = 0x0014_4df0;
    pub(super) const BE_NEW_RACE_INT_SET_EVENT: u32 = 0x0014_51e8;
    pub(super) const BE_NEW_PLAYER_INT_IS_CHAR_PROFILE_FLAG_CLEAR: u32 = 0x0014_5d38;
    pub(super) const GAME_MODULE_QUIT_TO_LOAD_SCREEN: u32 = 0x0023_0488;
    pub(super) const GAME_MODULE_REQUEST_STATE: u32 = 0x0023_1250;
    pub(super) const GAME_MODE_MAN_START_MODE: u32 = 0x0023_8160;
    pub(super) const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub(super) const AUDIO_STOP_FE_MUSIC: u32 = 0x0028_f520;
    pub(super) const AUDIO_SET_SELECTED_PEAK: u32 = 0x0028_f558;
    pub(super) const AUDIO_ON_EVENT_SELECT_EXIT: u32 = 0x0028_f678;
    pub(super) const STR_HASH: u32 = 0x0031_7670;
    pub(super) const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
    pub(super) const UI_STATE_HIDE_OBJECT: u32 = 0x0039_e8b8;
    pub(super) const UI_STATE_SHOW_OBJECT: u32 = 0x0039_e948;
    pub(super) const EA_MEM_SET: u32 = 0x003e_6448;
    pub(super) const SPRINTF: u32 = 0x0041_7828;
    pub(super) const GET_EVENT_RECORD: Callee = Callee {
        addr: 0x0020_0388,
        f: ee::s05::f_00200388,
    };
    pub(super) const HIDE_MAP_PIC: Callee = Callee {
        addr: 0x0020_1868,
        f: ee::s05::f_00201868,
    };
    pub(super) const UPDATE_MODE_PANELS: Callee = Callee {
        addr: 0x0020_2ad8,
        f: ee::s05::f_00202ad8,
    };
    pub(super) const SET_MODE: Callee = Callee {
        addr: 0x0020_30a0,
        f: ee::s05::f_002030a0,
    };
    pub(super) const LOAD_MAP_IMAGE: Callee = Callee {
        addr: 0x0020_3990,
        f: ee::s05::f_00203990,
    };
    pub(super) const UPDATE_CONFIRM_POPUP: Callee = Callee {
        addr: 0x0020_3a58,
        f: ee::s05::f_00203a58,
    };
    pub(super) const SET_MAP_TAB_MODE: Callee = Callee {
        addr: 0x0020_3cb8,
        f: ee::s05::f_00203cb8,
    };
    pub(super) const REFRESH_PEAK_LIST: Callee = Callee {
        addr: 0x0020_3d40,
        f: ee::s05::f_00203d40,
    };
    pub(super) const SET_HIGHLIGHT_COLOR: Callee = Callee {
        addr: 0x0020_83d8,
        f: ee::s05::f_002083d8,
    };
    pub(super) const SET_UNLOCKED_COLOR: Callee = Callee {
        addr: 0x0020_8438,
        f: ee::s05::f_00208438,
    };
}

/// `gp`-relative globals of the event-select screen.
mod es {
    /// The panel shown: 0 peaks, 1 a peak's tabs, 2 a tab's events.
    pub(super) const MODE: i32 = -0xb54;
    /// Selected tab (`gp-0xb50`) and peak (`gp-0xb4c`).
    pub(super) const TAB: i32 = -0xb50;
    pub(super) const PEAK: i32 = -0xb4c;
    /// Which flow event select runs in (0/1 front end, 2+ challenge
    /// variants, 3/4 from the in-game overlay).
    pub(super) const FLOW: i32 = -0xb5c;
    /// The world singleton.
    pub(super) const WORLD: i32 = -0x848;
    /// The selected peak handed to the race (`gp-0x1f34`).
    pub(super) const RACE_PEAK: i32 = -0x1f34;
    /// Set when an event is started from the event list (`gp-0x1f38`).
    pub(super) const EVENT_STARTED: i32 = -0x1f38;
    /// The game state to load next (`gp-0x9f4`).
    pub(super) const NEXT_STATE: i32 = -0x9f4;
    /// The loading-screen history: a depth (`gp-0x69c`) into
    /// `D_005366E8`, and the screen picked from `D_004428F0` (`gp-0x6a0`).
    pub(super) const LOAD_DEPTH: i32 = -0x69c;
    pub(super) const LOAD_SCREEN: i32 = -0x6a0;
}

/// The new-race settings block (`BENewRaceInt` state).
const NEW_RACE_INFO: u32 = 0x0053_5bc8;
/// `D_005366E8`, `D_004428F0`: the loading-screen stacks.
const LOAD_SCREEN_STACK: u32 = 0x0053_66e8;
const LOAD_SCREEN_TABLE: u32 = 0x0044_28f0;
/// `D_004C8B38`: the empty colour argument of a row's colour setter.
const NO_COLOUR: u32 = 0x004c_8b38;

/// Where `EventSelect_OnWidgetAction` leaves for its epilogue (the
/// original's three exit labels).
#[derive(Clone, Copy)]
enum Exit {
    /// `s0` already restored (0x202714): +8.
    Restored,
    /// Return v0 (0x202710): +1, restore s0, +8.
    Value,
    /// Return 0 (0x20270c): +1, v0 = 0, then as `Value`.
    Zero,
}

type Next = Result<Exit, crate::call::Unwound>;

/// `gp + off`.
#[inline(always)]
fn es_global(c: &Cpu, off: i32) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_add(off as u32))
}

/// `Addr(reg + off)`.
#[inline(always)]
fn reg_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// gcc's inline block move of `iters` 32-byte chunks from register `src` to
/// register `dst` (`ld`/`sd` when both are 8-aligned, else `ldl`/`ldr` and
/// `sdl`/`sdr`), the doublewords passing through `regs`, `cycles` per
/// iteration and the back-edge event check at `head`.
#[allow(clippy::too_many_arguments)]
fn es_block_move(
    c: &mut Cpu,
    src: usize,
    dst: usize,
    regs: [usize; 4],
    cycles: u64,
    head: u32,
    aligned: bool,
    iters: u32,
) {
    for i in 0..iters {
        c.cycles += cycles;
        let s = c.g[src][0] as u32;
        let d = c.g[dst][0] as u32;
        let mut v = [0u64; 4];
        for (k, w) in v.iter_mut().enumerate() {
            let a = Addr(s.wrapping_add(8 * k as u32));
            *w = if aligned {
                a.ld(c)
            } else {
                a.read_u64_unaligned(c)
            };
        }
        for (k, &w) in v.iter().enumerate() {
            let a = Addr(d.wrapping_add(8 * k as u32));
            if aligned {
                a.sd(c, w)
            } else {
                a.write_u64_unaligned(c, w)
            }
            c.g[regs[k]][0] = w;
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        c.g[dst][0] = sx(d.wrapping_add(0x20));
        if i + 1 < iters && event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The 12-byte tail of a 0x6c-byte record copy: `ldl/ldr` a doubleword
/// into `dw`, `lw` the last word into `word`, store both.
fn es_move_tail(c: &mut Cpu, src: usize, dst: usize, dw: usize, word: usize) {
    let s = c.g[src][0] as u32;
    let d = c.g[dst][0] as u32;
    c.g[dw][0] = Addr(s).read_u64_unaligned(c);
    c.g[word][0] = Addr(s.wrapping_add(8)).lw(c);
    Addr(d).write_u64_unaligned(c, c.g[dw][0]);
    Addr(d.wrapping_add(8)).sw(c, c.g[word][0]);
}

/// Play front-end sound `n` through the sound player in `s3`
/// (`player->vtbl[0x10](n)`; `swapped`: the original holds the vtable in v1
/// and the method in v0).
fn es_sound(c: &mut Cpu, n: u64, swapped: bool, ret: u32) -> Flow {
    let player = c.g[S3][0] as u32;
    let vt = Addr(player).lw(c);
    let m = VSlot::read(c, vt, 0x10);
    let (vt_reg, fn_reg) = if swapped { (V1, V0) } else { (V0, V1) };
    c.g[vt_reg][0] = vt;
    c.g[A1][0] = n;
    c.g[A0][0] = m.delta;
    c.g[fn_reg][0] = m.func;
    c.g[A0][0] = sx(player.wrapping_add(m.delta as u32));
    call_ptr(c, m.func as u32, ret)
}

/// Commit the new-race interface in `s0` (`iface->vtbl[+0xc][8]()`), the
/// method passing through `fn_reg`.
fn es_commit_race(c: &mut Cpu, fn_reg: usize, ret: u32) -> Flow {
    let iface = c.g[S0][0] as u32;
    c.g[V1][0] = Addr(iface.wrapping_add(0xc)).lw(c);
    let m = VSlot::read(c, c.g[V1][0], 8);
    c.g[A0][0] = m.delta;
    c.g[fn_reg][0] = m.func;
    c.g[A0][0] = sx(iface.wrapping_add(m.delta as u32));
    call_ptr(c, m.func as u32, ret)
}

/// `void EventSelect_OnWidgetAction(EventSelect* self, UIWidget* w, int action)`
/// (0x2018a8): the event-select screen's widget actions (`w->id` at +0x18 is
/// the row): 1/2 colour the row as highlighted/unlocked, 5 accepts (pick a
/// peak, a tab or an event, or confirm a challenge), 6 goes back, 7
/// toggles the map tab. Returns 1 when the screen is left, else 0.
pub fn event_select_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        c.g[S3][0] = sx(es_global(c, 0x2968).0);
        frame.save(c, S5, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S2][0] = c.g[A0][0];
        let this = c.g[S2][0] as u32;
        let w = c.g[S1][0] as u32;
        // The new-race (0) and new-player (1) interfaces.
        call_ptr(c, action::BE_LIBRARY_GET_INSTANCE, 0x0020_18dc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_18e8)?;
        c.g[S4][0] = c.g[V0][0];
        call_ptr(c, action::BE_LIBRARY_GET_INSTANCE, 0x0020_18f0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_18fc)?;
        c.g[S5][0] = c.g[V0][0];
        // The screen state's sound player, else the default one.
        c.g[V0][0] = Addr(this.wrapping_add(0x2ec)).lw(c);
        c.g[A0][0] = reg_at(c, V0, 0x10).lw(c);
        c.g[V1][0] = reg_at(c, A0, 0x14).lw(c);
        if c.g[V1][0] != 0 {
            c.g[S3][0] = c.g[V1][0];
        }
        let exit = if c.g[S1][0] == 0 {
            Exit::Zero
        } else {
            es_dispatch(c, this, w)?
        };
        match exit {
            Exit::Zero => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                c.cycles += 9;
            }
            Exit::Value => c.cycles += 9,
            Exit::Restored => c.cycles += 8,
        }
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// The action switch (jump table `D_00470780`, actions 1..7).
fn es_dispatch(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
    let in_range = c.g[V1][0] < 7;
    c.g[V0][0] = 0x0047_0000;
    if !in_range {
        return Ok(Exit::Zero);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx(0x0047_0780);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x0047_0780));
    c.g[A0][0] = reg_at(c, V1, 0).lw(c);
    match c.g[A0][0] as u32 {
        0x0020_23d4 => es_highlight(c, this, w),
        0x0020_261c => es_unlocked(c, this, w),
        0x0020_193c => es_accept(c, this, w),
        0x0020_21d8 => es_back(c, this),
        0x0020_22bc => es_toggle_tab(c, this),
        0x0020_270c => Ok(Exit::Zero),
        0x0020_2020 => {
            es_store_load_screen(c);
            es_start_special(c, this)
        }
        target => {
            c.jump_out(target, 0x0020_18a8, 0x0020_2734);
            Err(crate::call::Unwound)
        }
    }
}

// ----------------------------------------------------------- shared tails

/// 0x2021b8 / 0x201d60: "denied" sound (4), return 0.
fn es_deny(c: &mut Cpu, ret: u32) -> Next {
    c.cycles += 8;
    es_sound(c, 4, false, ret)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x2021e4: drop the confirm popup, then refresh the mode (a1 = mode).
fn es_clear_confirm(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    Addr(this.wrapping_add(0x2cc)).sw(c, 0);
    es_refresh_mode(c)
}

/// 0x2021ec: `SetMode(self, a1)`, refresh the panels and the confirm popup.
fn es_refresh_mode(c: &mut Cpu) -> Next {
    c.cycles += 8;
    action::SET_MODE.call(c, 0x0020_21f4)?;
    c.g[A0][0] = c.g[S2][0];
    action::UPDATE_MODE_PANELS.call(c, 0x0020_21fc)?;
    c.g[A0][0] = c.g[S2][0];
    action::UPDATE_CONFIRM_POPUP.call(c, 0x0020_2204)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x2022ac: hide the map picture and report the screen left (1).
fn es_leave(c: &mut Cpu) -> Next {
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    action::HIDE_MAP_PIC.call(c, 0x0020_22b4)?;
    c.g[V0][0] = 1;
    Ok(Exit::Value)
}

/// 0x2023bc: refresh the panels and load the map image, return 0.
fn es_show_mode(c: &mut Cpu) -> Next {
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    action::UPDATE_MODE_PANELS.call(c, 0x0020_23c4)?;
    c.g[A0][0] = c.g[S2][0];
    action::LOAD_MAP_IMAGE.call(c, 0x0020_23cc)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x202060: stop the front-end music and tell the audio system the peak
/// (`self+0x260`), return 0.
fn es_audio_peak(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 11;
    call_ptr(c, action::AUDIO_GET_INSTANCE, 0x0020_2068)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, action::AUDIO_STOP_FE_MUSIC, 0x0020_2070)?;
    call_ptr(c, action::AUDIO_GET_INSTANCE, 0x0020_2078)?;
    c.g[A1][0] = Addr(this.wrapping_add(0x260)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, action::AUDIO_SET_SELECTED_PEAK, 0x0020_2084)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x202020: pop the loading-screen history (registers from the block
/// before: a2 = new depth, v1 = stack entry * 4, a0 = table, a3 = state,
/// t1 = request, t0 = 1).
fn es_store_load_screen(c: &mut Cpu) {
    c.cycles += 6;
    es_global(c, es::NEXT_STATE).sw(c, c.g[A3][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    es_global(c, es::LOAD_DEPTH).sw(c, c.g[A2][0]);
    c.g[V0][0] = reg_at(c, V1, 0).lw(c);
    es_global(c, es::LOAD_SCREEN).sw(c, c.g[V0][0]);
    reg_at(c, T1, 8).sw(c, c.g[T0][0]);
}

/// 0x202038: event types 5/6 start their game mode; hide the map picture
/// and hand the peak to the audio system.
fn es_start_special(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 5;
    c.g[V0][0] = Addr(this.wrapping_add(0x264)).lw(c);
    let special = sx((c.g[V0][0] as u32).wrapping_sub(5)) < 2;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    if special {
        c.cycles += 3;
        c.g[A1][0] = Addr(this.wrapping_add(0x268)).lw(c);
        c.g[A0][0] = reg_at(c, V0, 0xc0).lw(c);
        call_ptr(c, action::GAME_MODE_MAN_START_MODE, 0x0020_2058)?;
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    action::HIDE_MAP_PIC.call(c, 0x0020_2060)?;
    es_audio_peak(c, this)
}

/// A row's colour setter with no colour (`w->vtbl(v1)[0x58]`, 0x2025fc /
/// 0x2026f4).
fn es_no_colour(c: &mut Cpu, w: u32, cycles: u64, ret: u32) -> Flow {
    c.cycles += cycles;
    c.g[A1][0] = sx(NO_COLOUR);
    let m = VSlot::read(c, c.g[V1][0], 0x58);
    c.g[A0][0] = m.delta;
    c.g[V0][0] = m.func;
    c.g[A0][0] = sx(w.wrapping_add(m.delta as u32));
    call_ptr(c, m.func as u32, ret)
}

/// `self + 4*w->id + off` (v0), loaded into a2: the row's flag.
fn es_row_flag(c: &mut Cpu, this: u32, w: u32, off: u32) {
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    es_row_flag_loaded(c, this, off);
}

/// As [`es_row_flag`] with `w->id` already in v0.
fn es_row_flag_loaded(c: &mut Cpu, this: u32, off: u32) {
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = reg_at(c, V0, off).lw(c);
}

/// `self + peak*0x60 + tab*0x20 + 4*w->id` (v0) for the event rows: `scaled`
/// holds `gp-0xb4c` and becomes `peak*0x60` (multiplied by `by`, which
/// holds 0x60).
fn es_event_row(c: &mut Cpu, this: u32, w: u32, scaled: usize, by: usize) {
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.mult(0, scaled, scaled, by);
    c.g[V1][0] = es_global(c, es::TAB).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[scaled][0] as u32));
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
}

// ------------------------------------------------- 1, 2: row colours

/// Action 1: colour the row as highlighted (with the confirm popup up, its
/// yes/no buttons follow the row).
fn es_highlight(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_23ec)?;
        c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
        c.g[A0][0] = Addr(this.wrapping_add(0x2ec)).lw(c);
        use self::action::{UI_STATE_HIDE_OBJECT as HIDE, UI_STATE_SHOW_OBJECT as SHOW};
        // ConfirmMenu_{Yes,No}_{x,a,na}: show "yes" on row 0, else "no".
        let steps: [(u32, u32, u32); 6] = if c.g[V0][0] == 0 {
            [
                (SHOW, 0x0047_06e8, 0x0020_2404),
                (HIDE, 0x0047_0700, 0x0020_2414),
                (SHOW, 0x0047_0718, 0x0020_2424),
                (HIDE, 0x0047_0730, 0x0020_2434),
                (SHOW, 0x0047_0748, 0x0020_2444),
                (HIDE, 0x0047_0760, 0x0020_2454),
            ]
        } else {
            [
                (SHOW, 0x0047_0700, 0x0020_2468),
                (HIDE, 0x0047_06e8, 0x0020_2478),
                (SHOW, 0x0047_0730, 0x0020_2488),
                (HIDE, 0x0047_0718, 0x0020_2498),
                (SHOW, 0x0047_0760, 0x0020_24a8),
                (HIDE, 0x0047_0748, 0x0020_24b8),
            ]
        };
        c.cycles += 25;
        for (i, (f, name, ret)) in steps.into_iter().enumerate() {
            if i > 0 {
                c.g[A0][0] = Addr(this.wrapping_add(0x2ec)).lw(c);
            }
            c.g[A1][0] = sx(name);
            call_ptr(c, f, ret)?;
        }
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 4;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    let mode = c.g[V1][0];
    c.g[V0][0] = ((mode as i64) < 2) as u64;
    if mode == 1 {
        // A tab row: its flag at +0x10; the tab highlight ("pghl") grows.
        c.cycles += 10;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        es_row_flag(c, this, w, 0x10);
        action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_2534)?;
        c.g[A0][0] = sx(0x004a_25f8);
        return es_grow_highlight(c, this, w);
    }
    c.cycles += 2;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if below {
        c.cycles += 2;
        if mode == 0 {
            // A peak row: its flag at +4.
            c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            es_row_flag_loaded(c, this, 4);
            action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_2510)?;
            c.g[V0][0] = 0;
            return Ok(Exit::Value);
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
        es_no_colour(c, w, 8, 0x0020_2614)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    c.g[A2][0] = es_global(c, es::PEAK).lw(c);
    if mode == 2 {
        // An event row; the event highlight ("ehl") grows.
        c.cycles += 15;
        c.g[A0][0] = 0x60;
        c.g[A1][0] = c.g[S1][0];
        es_event_row(c, this, w, A2, A0);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = reg_at(c, V0, 0x20).lw(c);
        action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_2574)?;
        c.g[A0][0] = sx(0x004a_2600);
        return es_grow_highlight(c, this, w);
    }
    c.cycles += 2;
    c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
    es_no_colour(c, w, 8, 0x0020_2614)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x20257c: find the highlight object named by a0 in the screen and set
/// its y (+0x48) to the row's y + 100.
fn es_grow_highlight(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 7;
    call_ptr(c, action::STR_HASH, 0x0020_2584)?;
    c.g[A0][0] = Addr(this.wrapping_add(0x2e8)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, action::UI_SCREEN_FIND_OBJECT, 0x0020_2590)?;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 25;
    let obj = c.g[V0][0] as u32;
    let s = sp(c);
    // The row's and the highlight's position (x, y, z at +0x44) copied to
    // the stack, the highlight moved to the row's y + 100.
    c.g[A3][0] = Addr(w.wrapping_add(0x44)).read_u64_unaligned(c);
    c.g[T0][0] = Addr(w.wrapping_add(0x4c)).lw(c);
    Addr(s).write_u64_unaligned(c, c.g[A3][0]);
    Addr(s + 8).sw(c, c.g[T0][0]);
    c.g[A1][0] = Addr(obj.wrapping_add(0x44)).read_u64_unaligned(c);
    c.g[A2][0] = Addr(obj.wrapping_add(0x4c)).lw(c);
    Addr(s + 0x10).write_u64_unaligned(c, c.g[A1][0]);
    Addr(s + 0x18).sw(c, c.g[A2][0]);
    const HUNDRED: u32 = 0x42c8_0000;
    c.g[AT][0] = HUNDRED as u64;
    c.f[1] = HUNDRED;
    let y = Addr(s + 4).read_fpu(c) + FpuF32(HUNDRED);
    c.f[0] = y.0;
    Addr(s + 0x14).write_fpu(c, y);
    c.g[V1][0] = Addr(s + 0x10).read_u64_unaligned(c);
    c.g[A0][0] = Addr(s + 0x18).lw(c);
    Addr(obj.wrapping_add(0x44)).write_u64_unaligned(c, c.g[V1][0]);
    Addr(obj.wrapping_add(0x4c)).sw(c, c.g[A0][0]);
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// Action 2: colour the row as unlocked.
fn es_unlocked(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        action::SET_UNLOCKED_COLOR.call(c, 0x0020_2634)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 4;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    let mode = c.g[V1][0];
    c.g[V0][0] = ((mode as i64) < 2) as u64;
    if mode == 1 {
        c.cycles += 9;
        c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        es_row_flag_loaded(c, this, 0x10);
        action::SET_UNLOCKED_COLOR.call(c, 0x0020_26b0)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if below {
        c.cycles += 2;
        if mode == 0 {
            c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            es_row_flag_loaded(c, this, 4);
            action::SET_UNLOCKED_COLOR.call(c, 0x0020_268c)?;
            c.g[V0][0] = 0;
            return Ok(Exit::Value);
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
        es_no_colour(c, w, 6, 0x0020_270c)?;
        return Ok(Exit::Zero);
    }
    c.cycles += 2;
    c.g[A2][0] = es_global(c, es::PEAK).lw(c);
    if mode == 2 {
        c.cycles += 15;
        c.g[A1][0] = 0x60;
        c.g[A0][0] = c.g[S2][0];
        es_event_row(c, this, w, A2, A1);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = reg_at(c, V0, 0x20).lw(c);
        action::SET_UNLOCKED_COLOR.call(c, 0x0020_26ec)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
    es_no_colour(c, w, 6, 0x0020_270c)?;
    Ok(Exit::Zero)
}

// ------------------------------------------------------------ 5: accept

/// Action 5 (accept) on the current panel.
fn es_accept(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    c.g[S4][0] = 1;
    let mode = c.g[V1][0];
    c.g[V0][0] = ((mode as i64) < 2) as u64;
    if mode == 1 {
        return es_accept_tab(c, this, w);
    }
    c.cycles += 2;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if below {
        c.cycles += 2;
        c.g[V0][0] = 0;
        if mode == 0 {
            return es_accept_peak(c, this, w);
        }
        c.cycles += 2;
        return Ok(Exit::Restored);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    if mode == 2 {
        return es_accept_event(c, this, w);
    }
    c.cycles += 2;
    Ok(Exit::Restored)
}

/// Accept on the peak panel: with the confirm popup up, "yes" (row 0)
/// starts the peak's free ride; else pick the peak (rows 0..2, if
/// unlocked), or in the challenge flows (`gp-0xb5c` >= 2) take a
/// challenge record (peaks 1 and 2 need their character profile flag).
fn es_accept_peak(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = reg_at(c, S3, 0).lw(c);
        return es_accept_confirm(c, this, w);
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(w.wrapping_add(0x18)).lw(c);
    let row = c.g[V1][0];
    let small = (row as i64) < 3;
    c.g[V0][0] = sx((row as u32) << 2);
    if !small {
        return es_deny(c, 0x0020_21d0);
    }
    c.cycles += 4;
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = reg_at(c, V0, 4).lw(c);
    c.g[A1][0] = 6;
    if c.g[V1][0] == 0 {
        return es_deny(c, 0x0020_21d0);
    }
    c.cycles += 9;
    es_sound(c, 6, false, 0x0020_19b4)?;
    c.g[V1][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A0][0] = 0;
    c.g[A2][0] = 0xf;
    if c.g[V1][0] == c.g[S4][0] {
        c.cycles += 2;
        c.g[A0][0] = 1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 2;
        if c.g[V1][0] == 2 {
            c.cycles += 2;
            c.g[A0][0] = 1;
            c.g[A2][0] = 0x10;
        }
    }
    c.cycles += 2;
    let locked_peak = c.g[A0][0] != 0;
    c.g[A0][0] = c.g[S5][0];
    if locked_peak {
        // BENewPlayerInt_IsCharProfileFlagClear(iface1, 0, 0xf / 0x10).
        c.cycles += 4;
        c.g[A1][0] = 0;
        call_ptr(
            c,
            action::BE_NEW_PLAYER_INT_IS_CHAR_PROFILE_FLAG_CLEAR,
            0x0020_19f0,
        )?;
        let clear = c.g[V0][0] != 0;
        c.g[V0][0] = es_global(c, es::FLOW).lw(c);
        if clear {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
                return es_pick_peak(c);
            }
            return es_take_challenge(c, this, w);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    es_pick_peak(c)
}

/// 0x201bbc: the peak (v0) picked: show its tabs.
fn es_pick_peak(c: &mut Cpu) -> Next {
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 1;
    es_global(c, es::PEAK).sw(c, c.g[V0][0]);
    action::SET_MODE.call(c, 0x0020_1bcc)?;
    es_show_mode(c)
}

/// The challenge flows: copy the challenge's 0x6c-byte event record (peak 1
/// from `D_00479248`, else `D_00478D38 + 0x360*peak`) to `self+0x260`, raise
/// the confirm popup and refresh the peak list.
fn es_take_challenge(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[A1][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[V0][0] = 1;
    c.g[A0][0] = 0x360;
    if c.g[A1][0] == 1 {
        c.cycles += 8;
        c.g[V1][0] = sx(this.wrapping_add(0x260));
        c.g[A1][0] = sx(0x0047_8ee8);
        c.g[A0][0] = sx(0x0047_9248);
        let aligned = (c.g[V1][0] | c.g[A0][0]) & 7 == 0;
        c.g[V0][0] = sx(0x0047_92a8);
        if aligned {
            es_block_move(c, A0, V1, [A1, A2, A3, T0], 11, 0x0020_1a88, true, 3);
        } else {
            es_block_move(c, A0, V1, [A1, A2, A3, T0], 19, 0x0020_1a34, false, 3);
            c.cycles += 2;
        }
        c.cycles += 7;
        es_move_tail(c, A0, V1, V0, A1);
    } else {
        c.cycles += 9;
        c.g[V1][0] = 0x0048_0000;
        c.mult(0, A0, A1, A0);
        c.g[V1][0] = sx(0x0047_8d38);
        c.g[A2][0] = sx(this.wrapping_add(0x260));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x0047_8d38));
        let aligned = (c.g[A0][0] | c.g[A2][0]) & 7 == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
        if aligned {
            es_block_move(c, A0, A2, [T0, V1, A1, A3], 11, 0x0020_1b4c, true, 3);
        } else {
            es_block_move(c, A0, A2, [T0, V1, A1, A3], 20, 0x0020_1af4, false, 3);
            c.cycles += 2;
        }
        c.cycles += 6;
        es_move_tail(c, A0, A2, T0, V0);
    }
    c.cycles += 10;
    c.g[V0][0] = 1;
    c.g[A1][0] = 0;
    Addr(this.wrapping_add(0x2cc)).sw(c, 1);
    c.g[A0][0] = c.g[S2][0];
    action::SET_MAP_TAB_MODE.call(c, 0x0020_1ba4)?;
    c.g[A0][0] = c.g[S2][0];
    action::REFRESH_PEAK_LIST.call(c, 0x0020_1bac)?;
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    c.g[A0][0] = c.g[S2][0];
    es_refresh_mode(c)
}

/// The confirm popup on the peak panel: "yes" (row 0) starts the peak's free
/// ride (event type 4, event 12) and leaves; "no" closes the popup.
fn es_accept_confirm(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 8;
    es_sound(c, 6, false, 0x0020_1be8)?;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[A1][0] = es_global(c, es::MODE).lw(c);
        c.cycles += 9;
        c.g[A0][0] = c.g[S2][0];
        Addr(this.wrapping_add(0x2cc)).sw(c, 0);
        action::SET_MODE.call(c, 0x0020_1d00)?;
        c.g[A0][0] = c.g[S2][0];
        action::UPDATE_MODE_PANELS.call(c, 0x0020_1d08)?;
        c.g[A0][0] = c.g[S2][0];
        action::UPDATE_CONFIRM_POPUP.call(c, 0x0020_1d10)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 31;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
    call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1c04)?;
    c.g[V1][0] = es_global(c, es::WORLD).lw(c);
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0;
    c.g[S1][0] = reg_at(c, V1, 0xc0).lw(c);
    call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1c18)?;
    c.g[V0][0] = Addr(this.wrapping_add(0x260)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 4;
    es_global(c, es::RACE_PEAK).sw(c, c.g[V0][0]);
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1c2c)?;
    c.g[A1][0] = 0xc;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1c38)?;
    es_commit_race(c, V0, 0x0020_1c4c)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0xc;
    call_ptr(c, action::GAME_MODE_MAN_START_MODE, 0x0020_1c58)?;
    // The game module: into state 0xe (event select -> race), flagged.
    c.g[V1][0] = es_global(c, es::WORLD).lw(c);
    c.g[A0][0] = 0xe;
    c.g[V1][0] = reg_at(c, V1, 0x84).lw(c);
    c.g[V0][0] = reg_at(c, V1, 0x214).lw(c);
    let in_state = c.g[V0][0] == c.g[A0][0];
    c.g[A0][0] = c.g[V1][0];
    if in_state {
        c.cycles += 5;
        c.g[V1][0] = reg_at(c, V1, 0x200).lw(c);
        reg_at(c, V1, 8).sw(c, c.g[S4][0]);
        c.g[V0][0] = Addr(this.wrapping_add(0x260)).lw(c);
        reg_at(c, V1, 0x18).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 8;
        c.g[A1][0] = 0xe;
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        call_ptr(c, action::GAME_MODULE_REQUEST_STATE, 0x0020_1c94)?;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        c.g[V1][0] = reg_at(c, V0, 0x84).lw(c);
        c.g[A0][0] = reg_at(c, V1, 0x200).lw(c);
        reg_at(c, A0, 8).sw(c, c.g[S4][0]);
    }
    c.cycles += 5;
    c.g[V0][0] = 3;
    c.g[A0][0] = es_global(c, es::FLOW).lw(c);
    c.g[V1][0] = 2;
    es_global(c, es::NEXT_STATE).sw(c, c.g[V0][0]);
    if c.g[A0][0] == c.g[V1][0] {
        // Pop the loading-screen history.
        c.cycles += 15;
        c.g[V0][0] = es_global(c, es::LOAD_DEPTH).lw(c);
        c.g[A0][0] = sx(LOAD_SCREEN_STACK);
        c.g[A1][0] = sx(LOAD_SCREEN_TABLE);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
        es_global(c, es::LOAD_DEPTH).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(LOAD_SCREEN_STACK));
        c.g[V0][0] = reg_at(c, V1, 0).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(LOAD_SCREEN_TABLE));
        c.g[V1][0] = reg_at(c, V0, 0).lw(c);
        es_global(c, es::LOAD_SCREEN).sw(c, c.g[V1][0]);
    }
    es_audio_peak(c, this)
}

/// Accept on the tab panel: an unlocked tab (`self[4 + id]`) opens its
/// event list, else the "denied" sound.
fn es_accept_tab(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 6;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = reg_at(c, V0, 0x10).lw(c);
    c.g[A1][0] = 6;
    if c.g[V1][0] == 0 {
        return es_deny(c, 0x0020_1d78);
    }
    c.cycles += 12;
    es_sound(c, 6, false, 0x0020_1d44)?;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 2;
    es_global(c, es::TAB).sw(c, c.g[V0][0]);
    action::SET_MODE.call(c, 0x0020_1d58)?;
    es_show_mode(c)
}

/// Accept on the event panel.
fn es_accept_event(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    let front_end = c.g[V0][0] < 2;
    c.g[V0][0] = front_end as u64;
    c.g[A0][0] = es_global(c, es::PEAK).lw(c);
    if front_end {
        c.cycles += 12;
        c.g[A1][0] = 0x60;
        es_event_row(c, this, w, A0, A1);
        c.g[V1][0] = reg_at(c, V0, 0x20).lw(c);
        let unlocked = c.g[V1][0] != 0;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        if unlocked {
            return es_start_event(c, w);
        }
    }
    es_event_confirm(c, this, w)
}

/// Front end, an unlocked event: set up the race from the event record and
/// leave (flow 1 also quits to the load screen, clearing the rule flags when
/// the game mode changed).
fn es_start_event(c: &mut Cpu, w: u32) -> Next {
    c.cycles += 13;
    c.g[A1][0] = 0;
    c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
    call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1dcc)?;
    c.g[S0][0] = c.g[V0][0];
    es_sound(c, 6, false, 0x0020_1de8)?;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    if c.g[V0][0] == c.g[S4][0] {
        c.cycles += 3;
        c.g[A0][0] = 0;
        call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1dfc)?;
    }
    c.g[A2][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.cycles += 24;
    c.g[A0][0] = c.g[SP][0];
    c.g[A3][0] = es_global(c, es::TAB).lw(c);
    c.g[A1][0] = es_global(c, es::PEAK).lw(c);
    action::GET_EVENT_RECORD.call(c, 0x0020_1e10)?;
    let record = sp(c);
    c.g[V1][0] = Addr(record).lw(c);
    c.g[V0][0] = 0x0053_0000;
    c.g[A1][0] = Addr(record + 4).lw(c);
    c.g[S1][0] = sx(NEW_RACE_INFO);
    Addr(NEW_RACE_INFO + 0x40).sw(c, c.g[V1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.g[S3][0] = Addr(NEW_RACE_INFO + 0x48).lb(c);
    es_global(c, es::EVENT_STARTED).sw(c, c.g[S4][0]);
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1e34)?;
    c.g[A1][0] = Addr(record + 8).lw(c);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1e40)?;
    es_commit_race(c, A1, 0x0020_1e54)?;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    if c.g[V0][0] == c.g[S4][0] {
        c.cycles += 3;
        c.g[V0][0] = reg_at(c, S1, 0x48).lb(c);
        let same_mode = c.g[S3][0] == c.g[V0][0];
        c.g[A0][0] = 0x0053_0000;
        if !same_mode {
            c.cycles += 4;
            c.g[A1][0] = 0;
            c.g[A0][0] = sx(0x0053_08b8);
            c.g[A2][0] = 0x20;
            call_ptr(c, action::EA_MEM_SET, 0x0020_1e7c)?;
        }
        c.cycles += 6;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = reg_at(c, V0, 0x84).lw(c);
        call_ptr(c, action::GAME_MODULE_QUIT_TO_LOAD_SCREEN, 0x0020_1e8c)?;
    }
    es_leave(c)
}

/// The event panel otherwise: with the confirm popup up, "go" (row 0)
/// starts the chosen event; else an unlocked event raises the popup.
fn es_event_confirm(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A1][0] = 6;
    if c.g[V0][0] == 0 {
        return es_choose_event(c, this, w);
    }
    c.cycles += 8;
    es_sound(c, 6, false, 0x0020_1eb4)?;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    if c.g[V0][0] != 0 {
        return es_clear_confirm(c, this);
    }
    // Event types 5/6 set race type and event from the record at +0x264;
    // otherwise a free ride (4, 12) unless the game mode is missions.
    c.cycles += 5;
    c.g[V0][0] = Addr(this.wrapping_add(0x264)).lw(c);
    let special = sx((c.g[V0][0] as u32).wrapping_sub(5)) < 2;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    if special {
        c.cycles += 19;
        c.g[A1][0] = 0;
        c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
        call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1ee0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1eec)?;
        c.g[A1][0] = Addr(this.wrapping_add(0x264)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1ef8)?;
        c.g[A1][0] = Addr(this.wrapping_add(0x268)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1f04)?;
        es_commit_race(c, V0, 0x0020_1f18)?;
        c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    } else {
        c.cycles += 5;
        c.g[V1][0] = 0x0053_0000;
        let missions = Addr(NEW_RACE_INFO + 0x48).lb(c) ^ 4 == 0;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        if !missions {
            c.cycles += 17;
            c.g[A1][0] = 0;
            c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
            call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1f40)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0;
            call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1f4c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1f58)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xc;
            call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1f64)?;
            es_commit_race(c, V0, 0x0020_1f78)?;
        }
        c.cycles += 1;
        c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    }
    // The overlay flows (3, 4) hand the peak to a game module already in
    // state 0xe; the front end requests that state.
    c.cycles += 4;
    let overlay = sx((c.g[V0][0] as u32).wrapping_sub(3)) < 2;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    if overlay {
        c.cycles += 5;
        c.g[A0][0] = 0xe;
        c.g[V0][0] = reg_at(c, V0, 0x84).lw(c);
        c.g[V1][0] = reg_at(c, V0, 0x214).lw(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 8;
            c.g[A0][0] = reg_at(c, V0, 0x200).lw(c);
            c.g[V0][0] = 1;
            reg_at(c, A0, 8).sw(c, 1);
            c.g[V1][0] = Addr(this.wrapping_add(0x260)).lw(c);
            reg_at(c, A0, 0x18).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(this.wrapping_add(0x260)).lw(c);
            es_global(c, es::RACE_PEAK).sw(c, c.g[V0][0]);
        }
        c.g[V0][0] = 3;
        c.cycles += 2;
        es_global(c, es::NEXT_STATE).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 22;
        c.g[A1][0] = 0xe;
        c.g[V1][0] = Addr(this.wrapping_add(0x260)).lw(c);
        c.g[A2][0] = 1;
        c.g[A0][0] = reg_at(c, V0, 0x84).lw(c);
        c.g[A3][0] = 0;
        es_global(c, es::RACE_PEAK).sw(c, c.g[V1][0]);
        call_ptr(c, action::GAME_MODULE_REQUEST_STATE, 0x0020_1fe4)?;
        c.g[A2][0] = es_global(c, es::LOAD_DEPTH).lw(c);
        c.g[V1][0] = sx(LOAD_SCREEN_STACK);
        c.g[A3][0] = es_global(c, es::WORLD).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A0][0] = sx(LOAD_SCREEN_TABLE);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(LOAD_SCREEN_STACK));
        c.g[A1][0] = reg_at(c, A3, 0x84).lw(c);
        c.g[V1][0] = reg_at(c, V0, 0).lw(c);
        c.g[A3][0] = 3;
        c.g[T1][0] = reg_at(c, A1, 0x200).lw(c);
        c.g[T0][0] = 1;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        es_store_load_screen(c);
    }
    es_start_special(c, this)
}

/// No popup yet: an unlocked event row raises the confirm popup with the
/// event's record copied to `self+0x260`; a locked one is denied.
fn es_choose_event(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 13;
    c.g[A0][0] = es_global(c, es::PEAK).lw(c);
    c.g[A1][0] = 0x60;
    es_event_row(c, this, w, A0, A1);
    c.g[V1][0] = reg_at(c, V0, 0x20).lw(c);
    c.g[A1][0] = 6;
    if c.g[V1][0] == 0 {
        return es_deny(c, 0x0020_21d0);
    }
    c.cycles += 19;
    c.g[S0][0] = sx(this.wrapping_add(0x260));
    es_sound(c, 6, true, 0x0020_20d8)?;
    c.g[V1][0] = 1;
    c.g[A0][0] = c.g[S2][0];
    Addr(this.wrapping_add(0x2cc)).sw(c, 1);
    c.g[A1][0] = 0;
    action::SET_MAP_TAB_MODE.call(c, 0x0020_20ec)?;
    c.g[A2][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = es_global(c, es::PEAK).lw(c);
    c.g[A3][0] = es_global(c, es::TAB).lw(c);
    action::GET_EVENT_RECORD.call(c, 0x0020_2100)?;
    c.g[V0][0] = c.g[S0][0] & 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V1][0] = c.g[SP][0];
    c.cycles += 1;
    c.g[V0][0] = sx(sp(c).wrapping_add(0x60));
    if aligned {
        es_block_move(c, V1, S0, [A1, A2, A3, T0], 11, 0x0020_2168, true, 3);
    } else {
        es_block_move(c, V1, S0, [A2, A3, T0, A0], 19, 0x0020_2110, false, 3);
        c.cycles += 2;
    }
    c.cycles += 9;
    es_move_tail(c, V1, S0, V0, A0);
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    c.g[A0][0] = c.g[S2][0];
    es_refresh_mode(c)
}

// -------------------------------------------------------------- 6: back

/// Action 6 (back): close the confirm popup, else step back a panel; from
/// the peak panel leave the screen (in the overlay flow 3 also handing the
/// race's track to the game module and telling the audio system).
fn es_back(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    if c.g[V0][0] != 0 {
        return es_clear_confirm(c, this);
    }
    c.cycles += 4;
    c.g[A0][0] = es_global(c, es::FLOW).lw(c);
    c.g[V0][0] = 3;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    let mode = c.g[V1][0];
    if c.g[A0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 1;
        if mode == 0 {
            c.cycles += 15;
            c.g[V1][0] = es_global(c, es::WORLD).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[S4][0];
            c.g[V0][0] = reg_at(c, V1, 0x84).lw(c);
            c.g[S0][0] = reg_at(c, V0, 0x200).lw(c);
            reg_at(c, S0, 8).sw(c, c.g[A1][0]);
            call_ptr(c, action::BE_NEW_RACE_INT_GET_TRACK_INFO, 0x0020_2240)?;
            c.g[V1][0] = reg_at(c, V0, 0).lw(c);
            reg_at(c, S0, 0x18).sw(c, c.g[V1][0]);
            call_ptr(c, action::AUDIO_GET_INSTANCE, 0x0020_224c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, action::AUDIO_ON_EVENT_SELECT_EXIT, 0x0020_2258)?;
            return es_leave(c);
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.g[V0][0] = 2;
    if mode == 1 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0;
        action::SET_MODE.call(c, 0x0020_2278)?;
        return es_show_mode(c);
    }
    c.cycles += 2;
    c.g[V0][0] = 4;
    if mode == 2 {
        c.cycles += 7;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        action::SET_MODE.call(c, 0x0020_2294)?;
        c.g[A0][0] = c.g[S2][0];
        action::UPDATE_MODE_PANELS.call(c, 0x0020_229c)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    let stay = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = 0;
    if stay {
        return Ok(Exit::Value);
    }
    es_leave(c)
}

// ---------------------------------------------------------- 7: map tab

/// Action 7 (challenge flows only, no popup): toggle the map tab, unless
/// the peak list (mode 0) or the peak's goal list (mode 1) is still
/// animating in (its object's state byte +0x95 is 3).
fn es_toggle_tab(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 4;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    let front_end = c.g[V0][0] < 2;
    c.g[V0][0] = 0;
    if front_end {
        return Ok(Exit::Value);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    let confirm = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if confirm {
        return Ok(Exit::Value);
    }
    c.cycles += 3;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    c.g[V0][0] = 1;
    if c.g[V1][0] == 0 {
        // "SP_PeakList".
        c.cycles += 9;
        c.g[A0][0] = sx(0x0047_0600);
        c.g[S0][0] = 0;
        call_ptr(c, action::STR_HASH, 0x0020_22f4)?;
        c.g[A0][0] = Addr(this.wrapping_add(0x2e8)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, action::UI_SCREEN_FIND_OBJECT, 0x0020_2300)?;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = reg_at(c, V0, 0x95).lbu(c);
        }
        c.cycles += 3;
        c.g[V0][0] = 3;
        if c.g[S0][0] == c.g[V0][0] {
            return es_show_mode(c);
        }
        c.cycles += 2;
        c.g[A1][0] = Addr(this).lw(c);
        return es_flip_tab(c, 11, 0x0020_2378, 0x0020_2390);
    }
    c.cycles += 2;
    if c.g[V1][0] != c.g[V0][0] {
        c.g[A1][0] = Addr(this).lw(c);
        return es_flip_tab(c, 9, 0x0020_23a4, 0x0020_23bc);
    }
    // "SPG_Peak%dGoals" for the selected peak.
    c.cycles += 13;
    c.g[A2][0] = es_global(c, es::PEAK).lw(c);
    c.g[A1][0] = sx(0x0047_06d8);
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    call_ptr(c, action::SPRINTF, 0x0020_233c)?;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, action::STR_HASH, 0x0020_2344)?;
    c.g[A0][0] = Addr(this.wrapping_add(0x2e8)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, action::UI_SCREEN_FIND_OBJECT, 0x0020_2350)?;
    let found = c.g[V0][0] != 0;
    c.g[V1][0] = 0;
    if found {
        c.cycles += 1;
        c.g[V1][0] = reg_at(c, V0, 0x95).lbu(c);
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    if c.g[V1][0] == c.g[V0][0] {
        return es_show_mode(c);
    }
    c.cycles += 1;
    c.g[A1][0] = Addr(this).lw(c);
    es_flip_tab(c, 11, 0x0020_2378, 0x0020_2390)
}

/// `SetMapTabMode(self, self->tab ^ 1)` (a1 = `self->tab`), the "select"
/// sound, then refresh.
fn es_flip_tab(c: &mut Cpu, cycles: u64, ret1: u32, ret2: u32) -> Next {
    c.cycles += cycles;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] ^= 1;
    action::SET_MAP_TAB_MODE.call(c, ret1)?;
    es_sound(c, 6, false, ret2)?;
    es_show_mode(c)
}

/// `void EventSelect_SetMapTabMode(EventSelect* self, int mode)` (0x203cb8):
/// store the map-tab mode and label the tab from the new mode.
pub fn event_select_set_map_tab_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at_m2(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at_m2(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[A0][0] = 0x470000;
        let this = c.g[S0][0];
        c.sw(this as u32, c.g[A1][0]);
        c.g[A0][0] = sx(at_m2(c.g[A0][0] as u32, 0xab8));
        call_ptr(c, callee::STR_HASH, 0x0020_3cdc)?;
        c.g[A0][0] = lw_at(c, this, 0x2e8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0020_3ce8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = lw_at(c, this, 0);
        let mode_zero = c.g[V0][0] == 0;
        c.g[A0][0] = 0x470000;
        if mode_zero {
            c.cycles += 6;
            c.g[A0][0] = sx(at_m2(c.g[A0][0] as u32, 0xad8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d20)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d2c)?;
            c.lq(16, at_m2(sp0, 0x20));
        } else {
            c.cycles += 8;
            c.g[A0][0] = sx(at_m2(c.g[A0][0] as u32, 0xac8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d04)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d10)?;
            c.lq(16, at_m2(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at_m2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_m2(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void EventSelect_LoadMapImage(EventSelect *self)` (0x203990): request the
/// selected event's map texture and latch which variant was loaded.
pub fn event_select_load_map_image(c: &mut Cpu) {
    guest(c, |c| {
        'b_203a44: {
            'b_203a40: {
                'b_2039f0: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                    if t {
                        break 'b_2039f0;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d4));
                        c.g[S2][0] = 0x1;
                        if c.g[V0][0] == c.g[S2][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_203a44;
                        } else {
                            c.cycles += 10;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x002039d4);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001a38c0, 0x002039d4)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x002039e4);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x001a36e8, 0x002039e4)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), c.g[S2][0]);
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), 0u64);
                            if t {
                                break 'b_203a40;
                            } else {
                                break 'b_2039f0;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                if t {
                    break 'b_203a40;
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                    if t {
                        break 'b_203a40;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        if c.g[V1][0] == c.g[V0][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_203a44;
                        } else {
                            c.cycles += 12;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x00203a1c);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x001a38c0, 0x00203a1c)?;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                            c.g[RA][0] = sx(0x00203a30);
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x001a3768, 0x00203a30)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb4c));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), 0u64);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), c.g[V0][0]);
                            break 'b_203a40;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_203a44;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `cUIState_hideObjSafe(state, name)`.
pub(crate) const ES_HIDE_OBJECT: u32 = 0x0039_e8b8;

/// `cUIState_showObjSafe(state, name)`.
pub(crate) const ES_SHOW_OBJECT: u32 = 0x0039_e948;

/// `GetHashValue32(str)`.
pub(crate) const ES_STR_HASH: u32 = 0x0031_7670;

/// `UIScreen_FindObject(screen, hash)`.
pub(crate) const ES_FIND_OBJECT: u32 = 0x0039_d860;

/// `UIText_SetTextId(text, id)`.
pub(crate) const ES_SET_TEXT_ID: u32 = 0x003a_1000;

/// `sprintf(buf, fmt, ...)`.
pub(crate) const ES_SPRINTF: u32 = 0x0041_7828;

/// `EventSelect_RefreshEventList(self)` (location info of the event list).
pub(crate) const ES_REFRESH_EVENT_LIST: u32 = 0x0020_7430;

/// Panel owner fields (the event-select screen object, `s1`).
pub(crate) const ES_HAS_FOCUS: u32 = 0x0;

pub(crate) const ES_LOCKS_FLAG: u32 = 0x2cc;

pub(crate) const ES_SCREEN: u32 = 0x2e8;

pub(crate) const ES_UI_STATE: u32 = 0x2ec;

/// Selected-list state byte of a list-menu object.
pub(crate) const ES_LIST_STATE: u32 = 0x95;

/// gp-relative map globals: state (0 peaks, 1 peak goals, 2 events), selected
/// event kind, peak number, and map style.
pub(crate) const ES_GP_MAP_STATE: u32 = 0xb54;

pub(crate) const ES_GP_SELECTED_EVENT: u32 = 0xb50;

pub(crate) const ES_GP_PEAK: u32 = 0xb4c;

pub(crate) const ES_GP_MAP_STYLE: u32 = 0xb5c;

/// An address in the string table the code builds with `lui 0x47`.
#[inline]
pub(crate) fn es_str47(off: u32) -> u64 {
    sx(0x0047_0000u32.wrapping_add(off))
}

/// An address in the string table the code builds with `lui 0x4a`.
#[inline]
pub(crate) fn es_str4a(off: u32) -> u64 {
    sx(0x004a_0000u32.wrapping_add(off))
}

/// `a0 = self->uiState` (the screen's `cUIState`, read again before each call).
#[inline]
pub(crate) fn es_load_ui_state(c: &mut Cpu) {
    let s1 = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(s1.wrapping_add(ES_UI_STATE));
}

/// `showObjSafe(a0, a1)` returning to `ret`.
#[inline]
pub(crate) fn es_show(c: &mut Cpu, a1: u64, ret: u32) -> Flow {
    c.g[A1][0] = a1;
    call_ptr(c, ES_SHOW_OBJECT, ret)
}

/// `hideObjSafe(a0, a1)` returning to `ret`.
#[inline]
pub(crate) fn es_hide(c: &mut Cpu, a1: u64, ret: u32) -> Flow {
    c.g[A1][0] = a1;
    call_ptr(c, ES_HIDE_OBJECT, ret)
}

/// `a1 = GetHashValue32(a0)`; `UIText_SetTextId(title (s0), a1)`: set the
/// sub-title text from the string id named by `a0`.
#[inline]
pub(crate) fn es_set_title(c: &mut Cpu, hash_ret: u32, text_ret: u32) -> Flow {
    call_ptr(c, ES_STR_HASH, hash_ret)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, ES_SET_TEXT_ID, text_ret)
}

/// `v0 = UIScreen_FindObject(self->screen, GetHashValue32(a0))`.
#[inline]
pub(crate) fn es_find_object(c: &mut Cpu, hash_ret: u32, find_ret: u32) -> Flow {
    call_ptr(c, ES_STR_HASH, hash_ret)?;
    let s1 = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(s1.wrapping_add(ES_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_FIND_OBJECT, find_ret)
}

/// The six pda map-tab objects, hidden (`hide = true`) or shown. The hide
/// form is entered with `a0` already loaded; the show form loads it first.
pub(crate) fn es_map_tab_objects(c: &mut Cpu, hide: bool) -> Flow {
    let names: [u64; 6] = [
        es_str4a(0x25b0),
        es_str47(0x808),
        es_str47(0x818),
        es_str4a(0x2618),
        es_str4a(0x2620),
        es_str4a(0x2628),
    ];
    let rets: [u32; 6] = if hide {
        [
            0x0020_2e14,
            0x0020_2e24,
            0x0020_2e34,
            0x0020_2e44,
            0x0020_2e54,
            0x0020_2e64,
        ]
    } else {
        [
            0x0020_2e88,
            0x0020_2e98,
            0x0020_2ea8,
            0x0020_2eb8,
            0x0020_2ec8,
            0x0020_2ed8,
        ]
    };
    for i in 0..6 {
        if i > 0 {
            es_load_ui_state(c);
        }
        if hide {
            es_hide(c, names[i], rets[i])?;
        } else {
            es_show(c, names[i], rets[i])?;
        }
    }
    Ok(())
}

/// Hide the map tab objects (entered with `a0` loaded), restore `s0`.
pub(crate) fn es_hide_section(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 25;
    es_map_tab_objects(c, true)?;
    frame.restore(c, S0, 0xd0);
    Ok(())
}

/// Hide section entered after one cycle and a reload of `a0`.
pub(crate) fn es_hide_section_reload(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    es_load_ui_state(c);
    es_hide_section(c, frame)
}

/// Show the map tab objects when `v0` (map style `< 2`) is 0, then restore
/// `s0`: the tail shared by the peak and goal screens.
pub(crate) fn es_show_section(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let skip = c.g[V0][0] != 0;
    frame.restore(c, S0, 0xd0);
    if !skip {
        c.cycles += 26;
        es_load_ui_state(c);
        es_map_tab_objects(c, false)?;
        frame.restore(c, S0, 0xd0);
    }
    Ok(())
}

/// `v0 = map style` on entry: `v0 = (style < 2)`, then the show section.
pub(crate) fn es_style_section(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    es_show_section(c, frame)
}

/// Map state 0: "Select Peak".
pub(crate) fn es_state_peaks(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 25;
    c.g[A0][0] = es_str47(0x7c0);
    es_set_title(c, 0x0020_2b5c, 0x0020_2b68)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7d8), 0x0020_2b78)?;
    c.g[V0][0] = 0x4a0000;
    es_load_ui_state(c);
    c.g[S2][0] = es_str4a(0x2608);
    let a1 = c.g[S2][0];
    es_hide(c, a1, 0x0020_2b8c)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7e8), 0x0020_2b9c)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7f8), 0x0020_2bac)?;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_LOCKS_FLAG));
    let locked = c.g[V0][0] != 0;
    es_load_ui_state(c);
    if !locked {
        c.cycles += 4;
        es_show(c, es_str4a(0x2610), 0x0020_2bc4)?;
        es_load_ui_state(c);
    }
    c.cycles += 15;
    c.g[S0][0] = 0;
    es_hide(c, es_str47(0x7a0), 0x0020_2bd8)?;
    es_load_ui_state(c);
    let a1 = c.g[S2][0];
    es_hide(c, a1, 0x0020_2be4)?;
    c.g[A0][0] = es_str47(0x600);
    call_ptr(c, ES_STR_HASH, 0x0020_2bf0)?;
    let s1 = c.g[S1][0] as u32;
    c.g[A0][0] = c.lw(s1.wrapping_add(ES_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_FIND_OBJECT, 0x0020_2bfc)?;
    if c.g[V0][0] != 0 {
        let list = c.g[V0][0] as u32;
        c.g[S0][0] = c.lbu(list.wrapping_add(ES_LIST_STATE));
    }
    c.cycles += 3;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_HAS_FOCUS));
    let no_focus = c.g[V0][0] == 0;
    c.g[V0][0] = 3;
    if !no_focus {
        c.cycles += 2;
        let is_earnings = c.g[S0][0] == c.g[V0][0];
        es_load_ui_state(c);
        if is_earnings {
            return es_hide_section(c, frame);
        }
        c.cycles += 2;
        let a1 = c.g[S2][0];
        es_show(c, a1, 0x0020_2c20)?;
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    let is_earnings = c.g[S0][0] == c.g[V0][0];
    let s_gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(s_gp.wrapping_sub(ES_GP_MAP_STYLE));
    if is_earnings {
        return es_hide_section_reload(c, frame);
    }
    c.cycles += 2;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    es_show_section(c, frame)
}

/// Map state 1: "Select Peak Goal".
pub(crate) fn es_state_goals(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 81;
    c.g[S2][0] = 0;
    c.g[A0][0] = es_str47(0x828);
    es_set_title(c, 0x0020_2c44, 0x0020_2c50)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7d8), 0x0020_2c60)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7e8), 0x0020_2c70)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7f8), 0x0020_2c80)?;
    es_load_ui_state(c);
    es_hide(c, es_str4a(0x2610), 0x0020_2c90)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7a0), 0x0020_2ca0)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x840), 0x0020_2cb0)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x850), 0x0020_2cc0)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x860), 0x0020_2cd0)?;
    // sprintf(name, "peak%d indicators", mPeak)
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[A1][0] = es_str47(0x870);
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, ES_SPRINTF, 0x0020_2ce4)?;
    es_load_ui_state(c);
    let name = c.g[SP][0];
    es_show(c, name, 0x0020_2cf0)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S6][0] = es_str47(0x888);
    let a1 = c.g[S6][0];
    es_hide(c, a1, 0x0020_2d04)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S5][0] = es_str47(0x8a0);
    let a1 = c.g[S5][0];
    es_hide(c, a1, 0x0020_2d18)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S4][0] = es_str47(0x8b8);
    let a1 = c.g[S4][0];
    es_hide(c, a1, 0x0020_2d2c)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S3][0] = es_str47(0x8d0);
    let a1 = c.g[S3][0];
    es_hide(c, a1, 0x0020_2d40)?;
    // sprintf(next, "SPG_Peak%d_Goals", mPeak + 1); find its list menu
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A1][0] = es_str47(0x6d8);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    call_ptr(c, ES_SPRINTF, 0x0020_2d5c)?;
    c.g[A0][0] = c.g[S0][0];
    es_find_object(c, 0x0020_2d64, 0x0020_2d70)?;
    if c.g[V0][0] != 0 {
        let list = c.g[V0][0] as u32;
        c.g[S2][0] = c.lbu(list.wrapping_add(ES_LIST_STATE));
    }
    // Info back panel and the per-state goal panel.
    c.cycles += 3;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_HAS_FOCUS));
    if c.g[V0][0] != 0 {
        es_load_ui_state(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        let not_earnings = c.g[S2][0] != c.g[V0][0];
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(gp.wrapping_sub(ES_GP_MAP_STYLE));
        if not_earnings {
            return es_style_section(c, frame);
        }
        c.cycles += 1;
        es_load_ui_state(c);
    }
    c.cycles += 5;
    let a1 = c.g[S3][0];
    es_show(c, a1, 0x0020_2d9c)?;
    c.g[V0][0] = 2;
    let is_freeride = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[S2][0] as i64) < 3) as u64;
    if is_freeride {
        c.cycles += 5;
        es_load_ui_state(c);
        let a1 = c.g[S5][0];
        es_show(c, a1, 0x0020_2de8)?;
        c.g[V0][0] = 3;
    } else {
        c.cycles += 2;
        let below_3 = c.g[V0][0] != 0;
        c.g[V0][0] = 3;
        if below_3 {
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.cycles += 5;
                es_load_ui_state(c);
                let a1 = c.g[S6][0];
                es_show(c, a1, 0x0020_2dd4)?;
                c.g[V0][0] = 3;
            }
        } else {
            c.cycles += 2;
            if c.g[S2][0] == c.g[V0][0] {
                es_load_ui_state(c);
                c.cycles += 3;
                let a1 = c.g[S4][0];
                es_show(c, a1, 0x0020_2df8)?;
                c.g[V0][0] = 3;
            } else {
                c.cycles += 2;
            }
        }
    }
    c.cycles += 2;
    let not_earnings = c.g[S2][0] != c.g[V0][0];
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_sub(ES_GP_MAP_STYLE));
    if not_earnings {
        es_style_section(c, frame)
    } else {
        es_hide_section_reload(c, frame)
    }
}

/// Map state 2: "Select Event". Enters with `v0` = selected event kind and
/// `a0` = 1 (left by the dispatch).
pub(crate) fn es_state_events(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let not_race = c.g[V0][0] != 0;
    // Title string for race (0), free-style (1) or free-ride (other) events.
    if !not_race {
        c.cycles += 6;
        es_load_ui_state(c);
        es_hide(c, es_str4a(0x2610), 0x0020_2ef4)?;
        c.cycles += 7;
        c.g[A0][0] = es_str47(0x8e0);
        es_set_title(c, 0x0020_2f24, 0x0020_2f30)?;
    } else {
        c.cycles += 2;
        let not_fs = c.g[V0][0] != c.g[A0][0];
        es_load_ui_state(c);
        if !not_fs {
            c.cycles += 5;
            es_hide(c, es_str4a(0x2610), 0x0020_2f14)?;
            c.g[A0][0] = es_str47(0x8f8);
            c.cycles += 7;
            es_set_title(c, 0x0020_2f24, 0x0020_2f30)?;
        } else {
            c.cycles += 10;
            es_hide(c, es_str4a(0x2610), 0x0020_2f44)?;
            c.g[A0][0] = es_str47(0x908);
            es_set_title(c, 0x0020_2f50, 0x0020_2f5c)?;
        }
    }
    es_load_ui_state(c);
    c.cycles += 47;
    es_hide(c, es_str47(0x7d8), 0x0020_2f6c)?;
    es_load_ui_state(c);
    es_hide(c, es_str47(0x7e8), 0x0020_2f7c)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7f8), 0x0020_2f8c)?;
    es_load_ui_state(c);
    es_show(c, es_str47(0x7a0), 0x0020_2f9c)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S5][0] = es_str47(0x840);
    let a1 = c.g[S5][0];
    es_hide(c, a1, 0x0020_2fb0)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S4][0] = es_str47(0x850);
    let a1 = c.g[S4][0];
    es_hide(c, a1, 0x0020_2fc4)?;
    c.g[V0][0] = 0x470000;
    es_load_ui_state(c);
    c.g[S3][0] = es_str47(0x860);
    let a1 = c.g[S3][0];
    es_hide(c, a1, 0x0020_2fd8)?;
    c.g[V0][0] = 0x470000;
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[S2][0] = es_str47(0x870);
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, ES_SPRINTF, 0x0020_2ff0)?;
    es_load_ui_state(c);
    let name = c.g[SP][0];
    es_show(c, name, 0x0020_2ffc)?;
    c.g[V0][0] = 0x4a0000;
    es_load_ui_state(c);
    c.g[S0][0] = es_str4a(0x2630);
    let a1 = c.g[S0][0];
    es_hide(c, a1, 0x0020_3010)?;
    let s1 = c.g[S1][0] as u32;
    c.g[V0][0] = c.lw(s1.wrapping_add(ES_HAS_FOCUS));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        es_load_ui_state(c);
        let a1 = c.g[S0][0];
        es_show(c, a1, 0x0020_3028)?;
    }
    c.cycles += 20;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, ES_REFRESH_EVENT_LIST, 0x0020_3030)?;
    es_load_ui_state(c);
    let a1 = c.g[S5][0];
    es_hide(c, a1, 0x0020_303c)?;
    es_load_ui_state(c);
    let a1 = c.g[S4][0];
    es_hide(c, a1, 0x0020_3048)?;
    es_load_ui_state(c);
    let a1 = c.g[S3][0];
    es_hide(c, a1, 0x0020_3054)?;
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    let gp = c.g[GP][0] as u32;
    c.g[A2][0] = c.lw(gp.wrapping_sub(ES_GP_PEAK));
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, ES_SPRINTF, 0x0020_3068)?;
    es_load_ui_state(c);
    let a1 = c.g[S0][0];
    es_show(c, a1, 0x0020_3074)?;
    frame.restore(c, S0, 0xd0);
    Ok(())
}

/// `void EventSelect_UpdateModePanels(EventSelect* self)` (0x202ad8): show and
/// hide the panels of the map screen by the map state (`gp-0xb54`): 0 select
/// a peak, 1 select a peak goal, 2 select an event. Sets the sub-title text,
/// toggles the objects of the screen's UI state by name, builds the
/// "peakN indicators" names with sprintf, and, for the event list, refreshes
/// the event list.
pub fn event_select_update_mode_panels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xd0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x60);
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x620));
        // title = FindObject(self->screen, GetHashValue32("SubTitle"))
        es_find_object(c, 0x0020_2b0c, 0x0020_2b18)?;
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = c.lw(gp.wrapping_sub(ES_GP_MAP_STATE));
        c.g[A0][0] = 1;
        c.g[S0][0] = c.g[V0][0];
        let state = c.g[V1][0];
        if state == 1 {
            es_state_goals(c, frame)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((state as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let is_events = state == c.g[V0][0];
                c.g[V0][0] = c.lw(gp.wrapping_sub(ES_GP_SELECTED_EVENT));
                if is_events {
                    es_state_events(c, frame)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xd0);
                }
            } else {
                c.cycles += 2;
                let is_peaks = state == 0;
                c.g[A0][0] = 0x470000;
                if is_peaks {
                    es_state_peaks(c, frame)?;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0xd0);
                }
            }
        }
        // Epilogue.
        c.cycles += 9;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `EventSelect_SetMapTabMode`, `EventSelect_RefreshPeakList`,
/// `EventSelect_RefreshPeakGoals` (direct calls through their generated path).
pub(crate) const ES_SETMODE_SET_MAP_TAB_MODE: Callee = Callee {
    addr: 0x0020_3cb8,
    f: ee::s05::f_00203cb8,
};

pub(crate) const ES_SETMODE_REFRESH_PEAK_LIST: Callee = Callee {
    addr: 0x0020_3d40,
    f: ee::s05::f_00203d40,
};

pub(crate) const ES_SETMODE_REFRESH_PEAK_GOALS: Callee = Callee {
    addr: 0x0020_5098,
    f: ee::s05::f_00205098,
};

/// `StrHash`, `UIScreen_FindObject`, `UIState_HideObject` (hideObjSafe),
/// `UIText_SetTextID` (0x3a1000).
pub(crate) const ES_SETMODE_STR_HASH: u32 = 0x0031_7670;

pub(crate) const ES_SETMODE_FIND_OBJECT: u32 = 0x0039_d860;

pub(crate) const ES_SETMODE_HIDE_OBJ_SAFE: u32 = 0x0039_e8b8;

pub(crate) const ES_SETMODE_SET_TEXT_ID: u32 = 0x003a_1000;

/// Base of the screen's name strings (`str_*` at 0x470000 + offset).
pub(crate) const ES_SETMODE_STR_BASE: u32 = 0x0047_0000;

/// `EventSelect` fields.
pub(crate) const ES_SETMODE_POPUP: u32 = 0x2cc;

pub(crate) const ES_SETMODE_NO_SELECTED: u32 = 0x2d0;

pub(crate) const ES_SETMODE_SCREEN: u32 = 0x2e8;

pub(crate) const ES_SETMODE_STATE: u32 = 0x2ec;

/// Widget vtable slots (`{delta: i16, pad, fn: u32}` at `8 * index`): v07
/// (set visible) at 0x38, v08 at 0x40, v09 at 0x48.
pub(crate) const ES_SETMODE_V07: u32 = 0x38;

pub(crate) const ES_SETMODE_V08: u32 = 0x40;

pub(crate) const ES_SETMODE_V09: u32 = 0x48;

/// `gp`-relative globals: map state (panel), selected tab, peak, flow.
pub(crate) const ES_SETMODE_G_MAP_STATE: u32 = 0xb54;

pub(crate) const ES_SETMODE_G_FLOW: u32 = 0xb5c;

pub(crate) const ES_SETMODE_G_PEAK: u32 = 0xb4c;

pub(crate) const ES_SETMODE_G_EVENT: u32 = 0xb50;

/// `gp - off`.
#[inline(always)]
pub(crate) fn es_setmode_gp(c: &Cpu, off: u32) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_sub(off))
}

/// `sp + off`.
#[inline(always)]
pub(crate) fn es_setmode_sp(c: &Cpu, off: u32) -> Addr {
    Addr(sp(c).wrapping_add(off))
}

/// Which panel `EventSelect_SetMode` configures.
#[derive(Clone, Copy)]
pub(crate) enum EsSetModePanel {
    Peaks,
    Goals,
    Events,
}

/// `obj->vtbl[slot](a1)`: `a0 = obj + vt[slot].delta`, target `vt[slot].fn`.
/// The compiled code keeps the vtable pointer in `vt_reg` and the target in
/// `fn_reg` (v0/v1 in either order).
pub(crate) fn es_setmode_vcall(
    c: &mut Cpu,
    obj: u32,
    vt_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> Flow {
    let vt = Addr(obj).off(8).lw(c);
    c.g[vt_reg][0] = vt;
    c.g[A0][0] = Addr(vt as u32).off(slot).lh(c);
    c.g[fn_reg][0] = Addr(vt as u32).off(slot + 4).lw(c);
    let target = c.g[fn_reg][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `hash = StrHash(str_base + name); obj = UIScreen_FindObject(self->screen,
/// hash)` (returns in v0). `ra` is the first call's return address, the
/// second returns 12 bytes later.
pub(crate) fn es_setmode_find_widget(c: &mut Cpu, name: u32, ra: u32) -> Flow {
    c.g[A0][0] = sx(ES_SETMODE_STR_BASE + name);
    call_ptr(c, ES_SETMODE_STR_HASH, ra)?;
    let this = c.g[S2][0] as u32;
    c.g[A0][0] = Addr(this).off(ES_SETMODE_SCREEN).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_SETMODE_FIND_OBJECT, ra + 0xc)
}

/// `hideObj(self, name)` with the widget kept in the saved register `dest`:
/// find the widget; if it exists, `v07(0)` and `v09(0)`. `pre` are the block's
/// own cycles (the first widget's are in the prologue).
pub(crate) fn es_setmode_hide_reg(c: &mut Cpu, pre: u64, ra: u32, name: u32, dest: usize) -> Flow {
    c.cycles += pre;
    es_setmode_find_widget(c, name, ra)?;
    c.g[dest][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if c.g[dest][0] != 0 {
        c.cycles += 11;
        let obj = c.g[dest][0] as u32;
        es_setmode_vcall(c, obj, V0, V1, ES_SETMODE_V07, ra + 0x2c)?;
        let obj = c.g[dest][0] as u32;
        c.g[A1][0] = 0;
        es_setmode_vcall(c, obj, V0, V1, ES_SETMODE_V09, ra + 0x44)?;
    }
    Ok(())
}

/// As [`es_setmode_hide_reg`] with the widget spilled to `sp + slot` (the
/// compiled code reloads it from there for every use, leaving it in v0).
pub(crate) fn es_setmode_hide_stack(c: &mut Cpu, ra: u32, name: u32, slot: u32) -> Flow {
    c.cycles += 8;
    es_setmode_find_widget(c, name, ra)?;
    let found = c.g[V0][0];
    es_setmode_sp(c, slot).sw(c, found);
    if found != 0 {
        c.cycles += 15;
        let vt = Addr(found as u32).off(8).lw(c);
        c.g[V0][0] = vt;
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(vt as u32).off(ES_SETMODE_V07).lh(c);
        c.g[V1][0] = Addr(vt as u32).off(ES_SETMODE_V07 + 4).lw(c);
        let obj = es_setmode_sp(c, slot).lw(c);
        c.g[V0][0] = obj;
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ra + 0x30)?;
        let obj = es_setmode_sp(c, slot).lw(c);
        c.g[V1][0] = obj;
        c.g[A1][0] = 0;
        let vt = Addr(obj as u32).off(8).lw(c);
        c.g[V0][0] = vt;
        c.g[A0][0] = Addr(vt as u32).off(ES_SETMODE_V09).lh(c);
        c.g[V1][0] = Addr(vt as u32).off(ES_SETMODE_V09 + 4).lw(c);
        let obj = es_setmode_sp(c, slot).lw(c);
        c.g[V0][0] = obj;
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ra + 0x50)?;
    }
    Ok(())
}

/// Show the widget in saved register `obj_reg`: set bit 0 of its flags word
/// (`+0x14`), then call the vtable slots `first` and `second` with a1 = 1
/// (the callers' delay slot already loaded a1 = 1).
pub(crate) fn es_setmode_show_widget(
    c: &mut Cpu,
    obj_reg: usize,
    first: u32,
    second: u32,
    rets: [u32; 2],
) -> Flow {
    let obj = c.g[obj_reg][0] as u32;
    c.g[V0][0] = Addr(obj).off(0x14).lw(c);
    c.g[V1][0] = Addr(obj).off(8).lw(c);
    c.g[V0][0] |= 1;
    let flags = c.g[V0][0];
    Addr(obj).off(0x14).sw(c, flags);
    es_setmode_vcall(c, obj, V1, V0, first, rets[0])?;
    let obj = c.g[obj_reg][0] as u32;
    c.g[A1][0] = 1;
    es_setmode_vcall(c, obj, V1, V0, second, rets[1])
}

/// `s0 = UIScreen_FindObject(screen, StrHash("title_title"))`; the title
/// widget name is at `str_base + 0x518`.
pub(crate) fn es_setmode_find_title(c: &mut Cpu, ra: u32) -> Flow {
    es_setmode_find_widget(c, 0x518, ra)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// `title->setTextID(StrHash(str_base + text))` (0x3a1000).
pub(crate) fn es_setmode_set_title(c: &mut Cpu, text: u32, ra_hash: u32, ra_set: u32) -> Flow {
    c.g[A0][0] = sx(ES_SETMODE_STR_BASE + text);
    call_ptr(c, ES_SETMODE_STR_HASH, ra_hash)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, ES_SETMODE_SET_TEXT_ID, ra_set)
}

/// `void EventSelect_SetMode(EventSelect* self, int focus)` (0x2030a0):
/// `cUITemplate_MAP_setupMenuFocus`. Sets the map tab (`SetMapTabMode(self,
/// 0)`), stores `focus` as the panel (`gp-0xb54`), hides the 14 panel widgets
/// (peak list, three goal lists, nine event lists, the confirm popup) and then
///
/// - with the confirm popup open (`self+0x2cc`): hides the "Locks" objects
///   through the UI state, shows the popup and highlights its "no" or "yes"
///   button by `self+0x2d0`;
/// - else by panel: 0 refreshes the peak list and shows the peak list, 1
///   refreshes the peak goals and shows the selected peak's goal list, 2 shows
///   the event list of the selected peak and tab; panels 0 to 2 also set the
///   screen title unless the flow (`gp-0xb5c`) is 2 or more.
///
/// The widgets live in saved registers (peak list, goals, last three event
/// lists, popup) and in stack slots `sp+0..0x14` (the other event lists), as
/// the compiled code spilled them; the scratch registers it leaves on each
/// path (a1, v0/v1, a0) are reproduced.
pub fn event_select_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0xc0 frame, ra at 0x20, s0..s7 and fp saved.
        c.cycles += 25;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S2, 0x90);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, FP, 0x30);
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save_ra(c, 0x20);

        // SetMapTabMode(self, 0); mMapState = focus.
        c.g[A1][0] = 0;
        ES_SETMODE_SET_MAP_TAB_MODE.call(c, 0x0020_30dc)?;
        c.g[A0][0] = 0x0047_0000;
        let focus = c.g[S0][0];
        es_setmode_gp(c, ES_SETMODE_G_MAP_STATE).sw(c, focus);

        // Hide the widgets: peak list (fp), goal lists (s7, s6, s4), event
        // lists (stack, stack, stack, stack, stack, stack, s5, s3, s1), popup
        // (s0).
        es_setmode_hide_reg(c, 0, 0x0020_30ec, 0x600, FP)?;
        es_setmode_hide_reg(c, 9, 0x0020_313c, 0x918, S7)?;
        es_setmode_hide_reg(c, 9, 0x0020_318c, 0x928, S6)?;
        es_setmode_hide_reg(c, 9, 0x0020_31dc, 0x938, S4)?;
        es_setmode_hide_stack(c, 0x0020_322c, 0x948, 0x00)?;
        es_setmode_hide_stack(c, 0x0020_3288, 0x960, 0x04)?;
        es_setmode_hide_stack(c, 0x0020_32e4, 0x978, 0x08)?;
        es_setmode_hide_stack(c, 0x0020_3340, 0x990, 0x0c)?;
        es_setmode_hide_stack(c, 0x0020_339c, 0x9a8, 0x10)?;
        es_setmode_hide_stack(c, 0x0020_33f8, 0x9c0, 0x14)?;
        es_setmode_hide_reg(c, 9, 0x0020_3454, 0x9d8, S5)?;
        es_setmode_hide_reg(c, 9, 0x0020_34a4, 0x9f0, S3)?;
        es_setmode_hide_reg(c, 9, 0x0020_34f4, 0xa08, S1)?;
        es_setmode_hide_reg(c, 9, 0x0020_3544, 0xa20, S0)?;

        'epilogue: {
            'restored_s0: {
                // The confirm popup.
                c.cycles += 3;
                let this = c.g[S2][0] as u32;
                c.g[V0][0] = Addr(this).off(ES_SETMODE_POPUP).lw(c);
                c.g[A1][0] = 0x004a_0000;
                if c.g[V0][0] != 0 {
                    c.cycles += 18;
                    let state = Addr(this).off(ES_SETMODE_STATE).lw(c);
                    c.g[A0][0] = state;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2610));
                    call_ptr(c, ES_SETMODE_HIDE_OBJ_SAFE, 0x0020_35a0)?;
                    // popup->v07(1); popup->v09(1).
                    let popup = c.g[S0][0] as u32;
                    c.g[A1][0] = 1;
                    es_setmode_vcall(c, popup, V0, V1, ES_SETMODE_V07, 0x0020_35b8)?;
                    let popup = c.g[S0][0] as u32;
                    c.g[A1][0] = 1;
                    es_setmode_vcall(c, popup, V0, V1, ES_SETMODE_V09, 0x0020_35d0)?;
                    c.g[V0][0] = Addr(this).off(ES_SETMODE_NO_SELECTED).lw(c);
                    let no_selected = c.g[V0][0] != 0;
                    c.g[A0][0] = 0x0047_0000;
                    if !no_selected {
                        // "No" button: v09(1), then v08(0).
                        c.cycles += 15;
                        es_setmode_find_widget(c, 0x6c8, 0x0020_35e4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 1;
                        let no = c.g[S0][0] as u32;
                        es_setmode_vcall(c, no, V0, V1, ES_SETMODE_V09, 0x0020_360c)?;
                        let no = c.g[S0][0] as u32;
                        c.g[V0][0] = Addr(no).off(8).lw(c);
                        c.g[A1][0] = 0;
                    } else {
                        // "No" button: v09(0), then v08(1).
                        c.cycles += 14;
                        es_setmode_find_widget(c, 0x6c8, 0x0020_3620)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        let no = c.g[S0][0] as u32;
                        es_setmode_vcall(c, no, V0, V1, ES_SETMODE_V09, 0x0020_3648)?;
                        let no = c.g[S0][0] as u32;
                        c.g[V0][0] = Addr(no).off(8).lw(c);
                        c.g[A1][0] = 1;
                    }
                    c.cycles += 6;
                    let no = c.g[S0][0] as u32;
                    es_setmode_vcall(c, no, V0, V1, ES_SETMODE_V08, 0x0020_3660)?;
                    frame.restore(c, S0, 0xb0);
                    break 'restored_s0;
                }

                // The panel switch.
                'panel_done: {
                    c.cycles += 4;
                    c.g[V1][0] = es_setmode_gp(c, ES_SETMODE_G_MAP_STATE).lw(c);
                    c.g[A0][0] = 1;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                    let panel = if c.g[V1][0] == c.g[A0][0] {
                        EsSetModePanel::Goals
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            // 2 or more: only 2 is a panel.
                            c.g[V0][0] = 2;
                            c.cycles += 2;
                            let is_events = c.g[V1][0] == c.g[V0][0];
                            frame.restore(c, S0, 0xb0);
                            if !is_events {
                                c.cycles += 2;
                                frame.restore(c, S1, 0xa0);
                                break 'epilogue;
                            }
                            EsSetModePanel::Events
                        } else {
                            // Below 2: 0 is the peak panel, negative none.
                            c.cycles += 2;
                            let is_peaks = c.g[V1][0] == 0;
                            frame.restore(c, S0, 0xb0);
                            if !is_peaks {
                                c.cycles += 2;
                                frame.restore(c, S1, 0xa0);
                                break 'epilogue;
                            }
                            EsSetModePanel::Peaks
                        }
                    };

                    match panel {
                        EsSetModePanel::Peaks => {
                            // Refresh the peak list and show it.
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            ES_SETMODE_REFRESH_PEAK_LIST.call(c, 0x0020_36a8)?;
                            c.g[A1][0] = 1;
                            if c.g[FP][0] != 0 {
                                c.cycles += 14;
                                es_setmode_show_widget(
                                    c,
                                    FP,
                                    ES_SETMODE_V07,
                                    ES_SETMODE_V09,
                                    [0x0020_36d0, 0x0020_36e8],
                                )?;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            c.g[A0][0] = 0x0047_0000;
                            if c.g[V0][0] == 0 {
                                break 'panel_done;
                            }
                            c.cycles += 9;
                            es_setmode_find_title(c, 0x0020_3700)?;
                            c.cycles += 7;
                            es_setmode_set_title(c, 0x7c0, 0x0020_3844, 0x0020_3850)?;
                            frame.restore(c, S0, 0xb0);
                            break 'restored_s0;
                        }
                        EsSetModePanel::Goals => {
                            // Refresh the goals and show the selected peak's.
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S2][0];
                            ES_SETMODE_REFRESH_PEAK_GOALS.call(c, 0x0020_3724)?;
                            es_setmode_show_goals(c)?;
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            c.g[A0][0] = 0x0047_0000;
                            if c.g[V0][0] == 0 {
                                break 'panel_done;
                            }
                            c.cycles += 8;
                            es_setmode_find_title(c, 0x0020_3824)?;
                            c.cycles += 7;
                            es_setmode_set_title(c, 0xa30, 0x0020_3844, 0x0020_3850)?;
                            frame.restore(c, S0, 0xb0);
                            break 'restored_s0;
                        }
                        EsSetModePanel::Events => {
                            // Show the selected peak's and tab's event list.
                            es_setmode_select_event(c);
                            c.cycles += 2;
                            c.g[A1][0] = 1;
                            if c.g[S0][0] != 0 {
                                c.cycles += 14;
                                es_setmode_show_widget(
                                    c,
                                    S0,
                                    ES_SETMODE_V09,
                                    ES_SETMODE_V07,
                                    [0x0020_3904, 0x0020_391c],
                                )?;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            c.g[A0][0] = 0x0047_0000;
                            if c.g[V0][0] == 0 {
                                break 'panel_done;
                            }
                            c.cycles += 12;
                            es_setmode_find_title(c, 0x0020_3934)?;
                            es_setmode_set_title(c, 0xa48, 0x0020_3950, 0x0020_395c)?;
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0xb0);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0xa0);
        }

        // Epilogue.
        c.cycles += 10;
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Panel 1: show the goal list of the selected peak (`gp-0xb4c`: 0 is s7, 1 is
/// s6, anything else s4; a missing list falls through to the next). Leaves
/// v0 = the flow (`gp-0xb5c`).
pub(crate) fn es_setmode_show_goals(c: &mut Cpu) -> Flow {
    c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_PEAK).lw(c);
    c.g[V1][0] = es_setmode_gp(c, ES_SETMODE_G_PEAK).lw(c);
    'shown: {
        'goal_two: {
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[A1][0] = 1;
                if c.g[S7][0] != 0 {
                    c.cycles += 16;
                    es_setmode_show_widget(
                        c,
                        S7,
                        ES_SETMODE_V09,
                        ES_SETMODE_V07,
                        [0x0020_3758, 0x0020_3770],
                    )?;
                    c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
                    break 'shown;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                break 'goal_two;
            }
            c.cycles += 2;
            c.g[A1][0] = 1;
            if c.g[S6][0] == 0 {
                break 'goal_two;
            }
            c.cycles += 16;
            es_setmode_show_widget(
                c,
                S6,
                ES_SETMODE_V09,
                ES_SETMODE_V07,
                [0x0020_37ac, 0x0020_37c4],
            )?;
            c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
            break 'shown;
        }
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[S4][0] != 0 {
            c.cycles += 14;
            es_setmode_show_widget(
                c,
                S4,
                ES_SETMODE_V09,
                ES_SETMODE_V07,
                [0x0020_37f4, 0x0020_380c],
            )?;
        }
        c.cycles += 1;
        c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_FLOW).lw(c);
    }
    Ok(())
}

/// Panel 2: pick the event list widget for the selected peak (`gp-0xb4c`) and
/// tab (`gp-0xb50`) into s0 (null for anything out of range): peak 0 and
/// tab 0, 1, 2 are the first three stack slots, peak 1 the next three, peak 2
/// is s5, s3, s1. Leaves v0/a0 as the compiled branches do.
pub(crate) fn es_setmode_select_event(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_PEAK).lw(c);
    c.g[S0][0] = 0;
    let state = c.g[V1][0];
    'chosen: {
        'tail: {
            'peak_not_zero: {
                if c.g[V0][0] != 0 {
                    break 'peak_not_zero;
                }
                // Peak 0: slots 0 (tab 0), 4 (tab 1), 8 (tab 2).
                c.cycles += 3;
                c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_EVENT).lw(c);
                c.g[S0][0] = es_setmode_sp(c, 0).lw(c);
                if c.g[V0][0] == 0 {
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[S0][0] = es_setmode_sp(c, 8).lw(c);
                if c.g[V0][0] == c.g[A0][0] {
                    c.cycles += 2;
                    c.g[S0][0] = es_setmode_sp(c, 4).lw(c);
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[V0][0] ^= 2;
                break 'tail;
            }
            'peak_not_one: {
                c.cycles += 2;
                if c.g[V0][0] != c.g[A0][0] {
                    break 'peak_not_one;
                }
                // Peak 1: slots 0xc, 0x10, 0x14.
                c.cycles += 3;
                c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_EVENT).lw(c);
                c.g[S0][0] = es_setmode_sp(c, 0xc).lw(c);
                if c.g[V0][0] == 0 {
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[S0][0] = es_setmode_sp(c, 0x14).lw(c);
                if c.g[V0][0] == c.g[A0][0] {
                    c.cycles += 2;
                    c.g[S0][0] = es_setmode_sp(c, 0x10).lw(c);
                    break 'chosen;
                }
                c.cycles += 2;
                c.g[V0][0] ^= 2;
                break 'tail;
            }
            // Peak 2: s5, s3, s1 (v1 is the panel, 2).
            c.cycles += 2;
            let not_two = c.g[V0][0] != state;
            c.g[V0][0] = es_setmode_gp(c, ES_SETMODE_G_EVENT).lw(c);
            if not_two {
                break 'chosen;
            }
            c.cycles += 2;
            c.g[S0][0] = c.g[S5][0];
            if c.g[V0][0] == 0 {
                break 'chosen;
            }
            c.cycles += 2;
            let not_one = c.g[V0][0] != c.g[A0][0];
            c.g[V0][0] ^= 2;
            if !not_one {
                c.cycles += 2;
                c.g[S0][0] = c.g[S3][0];
                break 'chosen;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[S1][0];
        }
        // Tab 2 (v0 = tab ^ 2 is zero) keeps the slot, any other tab clears it.
        c.cycles += 1;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = 0;
        }
    }
}

// ---------------------------------------------------------------------------
// EventSelect_RefreshPeakList (`cUITemplate_MAP_setupPeakInfo`).

/// `self` (the map template, `s3`) field offsets: unlocked flags of the three
/// peaks (`+4 + 4 * peak`), popup flag, screen and the state owner.
pub(crate) const PK_UNLOCKED: u32 = 0x4;

pub(crate) const PK_POPUP: u32 = 0x2cc;

pub(crate) const PK_SCREEN: u32 = 0x2e8;

pub(crate) const PK_OWNER: u32 = 0x2ec;

/// Peak selection object: the selected peak at `+0x18`; the list object keeps
/// it at `+0xa0`.
pub(crate) const PK_SEL_PEAK: u32 = 0x18;

pub(crate) const PK_LIST_SEL: u32 = 0xa0;

/// Stack frame of the function (`sp` relative).
pub(crate) const PK_FRAME: u32 = 0x3b0;

/// `name1`/`name2` string buffers (`sp+0`, `sp+0x20`), the wide/ascii text
/// buffer (`sp+0x40`), the copied goal records (`sp+0x1d0`, `sp+0x240`; 0x6c
/// bytes each: word 0 is the track, word 2 the goal kind).
pub(crate) const PK_NAME2: u32 = 0x20;

pub(crate) const PK_BUF: u32 = 0x40;

pub(crate) const PK_GOAL_A: u32 = 0x1d0;

pub(crate) const PK_GOAL_B: u32 = 0x240;

/// Spill slots: the mission, economy and score interfaces, goal count, done
/// count, the next peak, the goal-B pointer and `t1`.
pub(crate) const PK_MIS: u32 = 0x2b0;

pub(crate) const PK_ECON: u32 = 0x2b4;

pub(crate) const PK_SCORE: u32 = 0x2b8;

pub(crate) const PK_CNT: u32 = 0x2bc;

pub(crate) const PK_DONE: u32 = 0x2c0;

pub(crate) const PK_NEXT: u32 = 0x2c4;

pub(crate) const PK_GOAL_B_PTR: u32 = 0x2c8;

pub(crate) const PK_SPILL_T1: u32 = 0x2d0;

pub(crate) const PK_HIDE: u32 = 0x0039_e8b8;

pub(crate) const PK_SHOW: u32 = 0x0039_e948;

pub(crate) const PK_HASH: u32 = 0x0031_7670;

pub(crate) const PK_FIND: u32 = 0x0039_d860;

pub(crate) const PK_SET_TEXT_ID: u32 = 0x003a_1000;

pub(crate) const PK_SET_STRING: u32 = 0x003a_0c50;

pub(crate) const PK_SET_WIDE: u32 = 0x003a_0e90;

pub(crate) const PK_SPRINTF: u32 = 0x0041_7828;

pub(crate) const PK_SPRINTF1: u32 = 0x002c_26d0;

pub(crate) const PK_TRACK_MEDAL: u32 = 0x0015_8910;

pub(crate) const PK_TROPHY: u32 = 0x0015_7bf0;

/// `s3` (self); callee-saved, read from the register each time.
#[inline(always)]
pub(crate) fn pk_this(c: &Cpu) -> u32 {
    c.g[S3][0] as u32
}

/// `a0 = self->owner`.
#[inline(always)]
pub(crate) fn pk_owner_a0(c: &mut Cpu) {
    let a = pk_this(c).wrapping_add(PK_OWNER);
    c.g[A0][0] = c.lw(a);
}

/// `a0 = self->screen`.
#[inline(always)]
pub(crate) fn pk_screen_a0(c: &mut Cpu) {
    let a = pk_this(c).wrapping_add(PK_SCREEN);
    c.g[A0][0] = c.lw(a);
}

#[inline(always)]
pub(crate) fn pk_lw_sp(c: &mut Cpu, off: u32) -> u64 {
    let a = sp(c).wrapping_add(off);
    c.lw(a)
}

#[inline(always)]
pub(crate) fn pk_sw_sp(c: &mut Cpu, off: u32, v: u64) {
    let a = sp(c).wrapping_add(off);
    c.sw(a, v)
}

/// `sw reg, off(sp)`.
#[inline(always)]
pub(crate) fn pk_sw_sp_reg(c: &mut Cpu, off: u32, reg: usize) {
    let a = sp(c).wrapping_add(off);
    let v = c.g[reg][0];
    c.sw(a, v)
}

/// `lw (gp - off)`.
#[inline(always)]
pub(crate) fn pk_lw_gp(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lw(a)
}

/// `a0 = sp + off`-style address register value.
#[inline(always)]
pub(crate) fn pk_sp_addr(c: &Cpu, off: u32) -> u64 {
    sx(sp(c).wrapping_add(off))
}

/// Call of a guest function through the function table.
#[inline(always)]
pub(crate) fn pk_call(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    call_ptr(c, target, ret)
}

/// `obj->vtable[slot](obj, ...)`: `v0 = *(obj+8)`, `a0 = lh(v0+slot)`,
/// `v1 = lw(v0+slot+4)`, `a0 = obj + a0`, call `v1` (a1 is set by the caller).
pub(crate) fn pk_vcall(c: &mut Cpu, obj: usize, slot: u32, ret: u32) -> Flow {
    let o = c.g[obj][0] as u32;
    c.g[V0][0] = c.lw(o.wrapping_add(8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(slot));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(slot + 4));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(o.wrapping_add(c.g[A0][0] as u32));
    pk_call(c, t, ret)
}

/// The inlined copy of one 0x6c-byte goal record from `v1` to `a0`: 0x60 bytes
/// in 32-byte strides (`v0` is the end address of the strides), either with
/// aligned doublewords (`ld`/`sd`, 11 cycles a stride) or unaligned (`ldl`/
/// `ldr`, 19 cycles a stride and 2 more cycles at the exit). `regs` are the
/// four registers carrying the doublewords. The last 12 bytes follow in the
/// caller ([`pk_goal_tail`]).
pub(crate) fn pk_goal_strides(c: &mut Cpu, aligned: bool, regs: [usize; 4], head: u32) {
    loop {
        let src = c.g[V1][0] as u32;
        let dst = c.g[A0][0] as u32;
        if aligned {
            c.cycles += 11;
            for (i, r) in regs.iter().enumerate() {
                c.g[*r][0] = c.ld(src.wrapping_add(8 * i as u32));
            }
            for (i, r) in regs.iter().enumerate() {
                let v = c.g[*r][0];
                c.sd(dst.wrapping_add(8 * i as u32), v);
            }
        } else {
            c.cycles += 19;
            for (i, r) in regs.iter().enumerate() {
                let o = 8 * i as u32;
                c.ldl(*r, src.wrapping_add(o + 7));
                c.ldr(*r, src.wrapping_add(o));
            }
            for (i, r) in regs.iter().enumerate() {
                let o = 8 * i as u32;
                c.sdl(*r, dst.wrapping_add(o + 7));
                c.sdr(*r, dst.wrapping_add(o));
            }
        }
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = sx(dst.wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    if !aligned {
        c.cycles += 2;
    }
}

/// The last 12 bytes of a goal copy: a doubleword through `ldl`/`ldr` in
/// `r_dw`, then a word into `r_w` (stored with `sw`).
pub(crate) fn pk_goal_tail(c: &mut Cpu, r_dw: usize, r_w: usize) {
    let src = c.g[V1][0] as u32;
    c.ldl(r_dw, src.wrapping_add(7));
    c.ldr(r_dw, src);
    c.g[r_w][0] = c.lw(src.wrapping_add(8));
    let dst = c.g[A0][0] as u32;
    c.sdl(r_dw, dst.wrapping_add(7));
    c.sdr(r_dw, dst);
    let w = c.g[r_w][0];
    c.sw(dst.wrapping_add(8), w);
}

pub(crate) fn pk_resume_2(c: &mut Cpu) {
    ee::s05::f_00203d40__entries(c)
}

/// Epilogue: reload the callee-saved registers (`s0` first, as the early
/// exits do), `ra` and the float registers, and return.
pub(crate) fn pk_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 17;
    for (r, off) in [
        (S1, 0x360),
        (S2, 0x350),
        (S3, 0x340),
        (S4, 0x330),
        (S5, 0x320),
        (S6, 0x310),
        (S7, 0x300),
        (FP, 0x2f0),
    ] {
        frame.restore(c, r, off);
    }
    frame.restore_ra(c, 0x2e0);
    for (f, off) in [
        (25, 0x3a8),
        (24, 0x3a0),
        (23, 0x398),
        (22, 0x390),
        (21, 0x388),
        (20, 0x380),
    ] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
}

/// `lq s0` (the early exits reload `s0` in a branch delay slot) and leave.
pub(crate) fn pk_leave(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x370);
    pk_epilogue(c, frame);
}

/// `void EventSelect_RefreshPeakList(MapTemplate* self)` (0x203d40;
/// `cUITemplate_MAP_setupPeakInfo`): refresh the peak panel of the map screen.
/// Shows/hides the popup frame, the three trophy marks and the per-peak
/// "unlocked" markers (by map style and state), then looks at the selected
/// peak. For peaks 0..2 it sets the help text and, on map style >= 2, the goal
/// counters (slopestyle/race medal goals found, challenges and collectibles,
/// earnings) with their check marks. For the overall entry (peak >= 3) it
/// counts goals, gold medals, missions, collectibles and highlights and shows
/// the completion percentage.
///
/// Everything lives in registers across the calls (the generated body is
/// resumable at its call returns), so the loop counters stay in `s0`..`s7`.
pub fn event_select_refresh_peak_list(c: &mut Cpu) {
    if c.pc != 0x0020_3d40 {
        return pk_resume_2(c);
    }
    guest(c, pk_refresh_body)
}

pub(crate) fn pk_refresh_body(c: &mut Cpu) -> Flow {
    c.cycles += 44;
    let frame = Frame::open(c, PK_FRAME);
    frame.save(c, S3, 0x340);
    frame.save(c, S5, 0x320);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S7, 0x300);
    frame.save(c, S0, 0x370);
    frame.save(c, S1, 0x360);
    frame.save(c, S2, 0x350);
    frame.save(c, S4, 0x330);
    frame.save(c, S6, 0x310);
    frame.save(c, FP, 0x2f0);
    frame.save_ra(c, 0x2e0);
    for (f, off) in [
        (25, 0x3a8),
        (24, 0x3a0),
        (23, 0x398),
        (22, 0x390),
        (21, 0x388),
        (20, 0x380),
    ] {
        frame.save_fpr(c, f, off);
    }

    // The back-end interfaces: reward (s5), mission, economy, score; and the
    // rider character (s7).
    pk_call(c, 0x0014_dc80, 0x0020_3d8c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    pk_call(c, 0x0014_dd58, 0x0020_3d98)?;
    c.g[S5][0] = c.g[V0][0];
    pk_call(c, 0x0014_dc80, 0x0020_3da0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xa;
    pk_call(c, 0x0014_dd58, 0x0020_3dac)?;
    pk_sw_sp_reg(c, PK_MIS, V0);
    pk_call(c, 0x0014_dc80, 0x0020_3db4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xb;
    pk_call(c, 0x0014_dd58, 0x0020_3dc0)?;
    pk_sw_sp_reg(c, PK_ECON, V0);
    pk_call(c, 0x0014_dc80, 0x0020_3dc8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x8;
    pk_call(c, 0x0014_dd58, 0x0020_3dd4)?;
    pk_sw_sp_reg(c, PK_SCORE, V0);
    c.g[A0][0] = 0;
    pk_call(c, 0x0014_a080, 0x0020_3de0)?;
    c.g[S7][0] = c.g[V0][0];

    // Popup frame: hidden while the popup flag is set, else shown.
    let popup = pk_this(c).wrapping_add(PK_POPUP);
    c.g[V0][0] = c.lw(popup);
    let show_frame = c.g[V0][0] == 0;
    pk_owner_a0(c);
    if !show_frame {
        c.cycles += 13;
        c.g[A1][0] = 0x0047_0ae8;
        pk_call(c, PK_HIDE, 0x0020_3dfc)?;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2648;
        pk_call(c, PK_HIDE, 0x0020_3e0c)?;
        c.g[A1][0] = 0x004a_0000;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2610;
        pk_call(c, PK_HIDE, 0x0020_3e1c)?;
        pk_owner_a0(c);
    } else {
        c.cycles += 12;
        c.g[A1][0] = 0x0047_0ae8;
        pk_call(c, PK_SHOW, 0x0020_3e30)?;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2648;
        pk_call(c, PK_SHOW, 0x0020_3e40)?;
        c.g[A1][0] = 0x004a_0000;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2610;
        pk_call(c, PK_SHOW, 0x0020_3e50)?;
        pk_owner_a0(c);
    }

    c.cycles += 17;
    c.g[A1][0] = 0x0047_0af8;
    c.g[S0][0] = 0;
    pk_call(c, PK_HIDE, 0x0020_3e64)?;
    c.g[S2][0] = 0x0047_0000;
    pk_owner_a0(c);
    c.g[A1][0] = 0x0047_0b08;
    c.g[FP][0] = 0x0047_0000;
    pk_call(c, PK_HIDE, 0x0020_3e7c)?;
    c.g[A1][0] = 0x004a_0000;
    pk_owner_a0(c);
    c.g[A1][0] = 0x004a_2650;
    pk_call(c, PK_HIDE, 0x0020_3e8c)?;
    c.g[S4][0] = pk_sp_addr(c, PK_NAME2);
    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(1));

    // Trophy marks 1..3: shown when all four trophies of the peak are owned.
    loop {
        c.cycles += 12;
        c.g[A0][0] = c.g[SP][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb20));
        pk_call(c, PK_SPRINTF, 0x0020_3ea8)?;
        // `isTrophyOwned(rew, 0, ch, peak, i)` for i = 0..3. `None`: one of
        // the first three failed, `Some(false)`: the fourth failed.
        let mut owned: Option<bool> = None;
        'trophies: {
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 0;
            pk_call(c, PK_TROPHY, 0x0020_3ec0)?;
            let ok = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S5][0];
            if !ok {
                break 'trophies;
            }
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 1;
            pk_call(c, PK_TROPHY, 0x0020_3edc)?;
            let ok = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S5][0];
            if !ok {
                break 'trophies;
            }
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 2;
            pk_call(c, PK_TROPHY, 0x0020_3ef8)?;
            let ok = c.g[V0][0] != 0;
            c.g[A3][0] = c.g[S0][0];
            if !ok {
                break 'trophies;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[T0][0] = 3;
            pk_call(c, PK_TROPHY, 0x0020_3f14)?;
            let ok = c.g[V0][0] != 0;
            pk_owner_a0(c);
            owned = Some(ok);
        }
        match owned {
            Some(true) => {
                c.cycles += 4;
                c.g[A1][0] = c.g[SP][0];
                pk_call(c, PK_SHOW, 0x0020_3f24)?;
                c.g[S0][0] = c.g[S1][0];
            }
            other => {
                if other.is_none() {
                    c.cycles += 1;
                    pk_owner_a0(c);
                }
                c.cycles += 3;
                c.g[A1][0] = c.g[SP][0];
                pk_call(c, PK_HIDE, 0x0020_3f38)?;
                c.g[S0][0] = c.g[S1][0];
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_3e98);
        }
    }

    // Map style < 2: the "Mountain" object is made static and invisible.
    c.cycles += 4;
    c.g[V0][0] = pk_lw_gp(c, 0xb5c);
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let low_style = c.g[V0][0] != 0;
    c.g[A0][0] = 0x0047_0000;
    if low_style {
        c.cycles += 18;
        c.g[A0][0] = 0x0047_0610;
        pk_call(c, PK_HASH, 0x0020_3f60)?;
        pk_screen_a0(c);
        c.g[A1][0] = c.g[V0][0];
        pk_call(c, PK_FIND, 0x0020_3f6c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        pk_vcall(c, S0, 0x40, 0x0020_3f88)?;
        c.g[A1][0] = 0;
        pk_vcall(c, S0, 0x48, 0x0020_3fa0)?;
    }

    c.cycles += 16;
    pk_owner_a0(c);
    c.g[A1][0] = 0x004a_25d8;
    c.g[S1][0] = 0;
    pk_call(c, PK_HIDE, 0x0020_3fb4)?;
    c.g[S6][0] = sx(pk_this(c).wrapping_add(PK_UNLOCKED));
    pk_owner_a0(c);
    c.g[A1][0] = 0x0047_0548;
    pk_call(c, PK_HIDE, 0x0020_3fc8)?;
    c.g[A1][0] = 0x0047_0000;
    pk_owner_a0(c);
    c.g[A1][0] = 0x0047_0558;
    pk_call(c, PK_HIDE, 0x0020_3fd8)?;
    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));

    // The per-peak markers "PEAKn" (name1) and "...n" (name2).
    loop {
        c.cycles += 13;
        c.g[V0][0] = 0x004a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2658));
        c.g[A2][0] = c.g[S0][0];
        c.g[S2][0] = c.g[S0][0];
        pk_call(c, PK_SPRINTF, 0x0020_3ff8)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0xb38));
        c.g[A2][0] = c.g[S0][0];
        pk_call(c, PK_SPRINTF, 0x0020_4008)?;
        c.g[V1][0] = pk_lw_gp(c, 0xb54);
        let state_set = c.g[V1][0] != 0;
        pk_owner_a0(c);
        let mut hide_name2_only = false;
        if !state_set {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S4][0];
                pk_call(c, PK_HIDE, 0x0020_4030)?;
                c.g[S1][0] = c.g[S2][0];
                hide_name2_only = true;
            }
        }
        if !hide_name2_only {
            c.cycles += 6;
            c.g[A1][0] = c.g[SP][0];
            pk_call(c, PK_HIDE, 0x0020_4040)?;
            c.g[V0][0] = pk_lw_gp(c, 0xb5c);
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            let high_style = c.g[V0][0] == 0;
            c.g[S1][0] = c.g[S2][0];
            if !high_style {
                c.cycles += 4;
                pk_owner_a0(c);
                c.g[A1][0] = c.g[S4][0];
                pk_call(c, PK_HIDE, 0x0020_405c)?;
                c.g[S1][0] = c.g[S2][0];
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_3fe0);
        }
    }

    // Help text object hidden; then the peak list and its selection.
    c.cycles += 21;
    c.g[A0][0] = 0x0047_0578;
    pk_call(c, PK_HASH, 0x0020_4078)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4084)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    pk_vcall(c, S1, 0x48, 0x0020_40a0)?;
    c.g[A0][0] = 0x0047_0600;
    pk_call(c, PK_HASH, 0x0020_40ac)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_40b8)?;
    let no_list = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x370);
    if no_list {
        pk_epilogue(c, frame);
        return Ok(());
    }
    c.cycles += 3;
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(PK_LIST_SEL));
    if c.g[S0][0] == 0 {
        pk_leave(c, frame);
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PK_SEL_PEAK));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
    let single_peak = c.g[V0][0] != 0;
    pk_owner_a0(c);
    if single_peak {
        pk_peak_info(c, frame)
    } else {
        pk_total_info(c, frame)
    }
}

/// The panel of one peak (0..2): help text, then (map style >= 2) the goal
/// counters of that peak.
pub(crate) fn pk_peak_info(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 15;
    c.g[A1][0] = 0x0047_0568;
    pk_call(c, PK_HIDE, 0x0020_40e8)?;
    c.g[A1][0] = 1;
    c.g[FP][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PK_SEL_PEAK));
    pk_vcall(c, S1, 0x48, 0x0020_4104)?;
    c.g[V0][0] = sx((c.g[FP][0] as u32) << 2);
    c.g[V0][0] = sx(pk_this(c).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(PK_UNLOCKED));
    let locked = c.g[V1][0] == 0;
    c.g[V0][0] = pk_lw_gp(c, 0xb5c);
    // Which help string, as the original's block ladder: `Some(string)` with
    // the cycles of the string choice, then `SetTextId(title, hash(string))`
    // charging `set_cycles`.
    let set_cycles;
    if !locked {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = 0x0047_0000;
            c.cycles += 2;
            c.g[A0][0] = 0x0047_0b68;
        } else {
            c.cycles += 3;
            c.g[A0][0] = 0x0047_0b50;
        }
        set_cycles = 7;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let not_one = c.g[FP][0] != c.g[V0][0];
        c.g[V0][0] = pk_lw_gp(c, 0xb5c);
        if !not_one {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = 0x0047_0000;
                c.cycles += 2;
                c.g[A0][0] = 0x0047_0b98;
            } else {
                c.cycles += 3;
                c.g[A0][0] = 0x0047_0b80;
            }
            set_cycles = 7;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            c.g[A0][0] = 0x0047_0000;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = 0x0047_0bc8;
                set_cycles = 6;
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0x0047_0bb0;
                set_cycles = 7;
            }
        }
    }
    c.cycles += set_cycles;
    pk_call(
        c,
        PK_HASH,
        if set_cycles == 7 {
            0x0020_4180
        } else {
            0x0020_419c
        },
    )?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(
        c,
        PK_SET_TEXT_ID,
        if set_cycles == 7 {
            0x0020_418c
        } else {
            0x0020_41a8
        },
    )?;
    c.g[V0][0] = pk_lw_gp(c, 0xb5c);

    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    if c.g[V0][0] != 0 {
        pk_leave(c, frame);
        return Ok(());
    }

    // Map style >= 2: goal counters of the peak.
    c.cycles += 41;
    c.g[V1][0] = 4;
    c.g[A0][0] = 0x0047_0be0;
    pk_sw_sp_reg(c, PK_CNT, V1);
    pk_call(c, PK_HASH, 0x0020_41cc)?;
    c.g[S4][0] = 0;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_41dc)?;
    c.g[V1][0] = pk_lw_gp(c, 0x848);
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = 0x0047_0000;
    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbf8));
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    pk_call(c, PK_HASH, 0x0020_4200)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let t = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, t, 0x0020_4210)?;
    c.g[S0][0] = pk_sp_addr(c, PK_BUF);
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(1));
    c.g[A0][0] = c.g[S0][0];
    pk_call(c, PK_SPRINTF1, 0x0020_4224)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    pk_call(c, PK_SET_WIDE, 0x0020_4230)?;
    c.g[A1][0] = pk_lw_sp(c, PK_CNT);
    c.g[V1][0] = 3;
    c.g[V0][0] = c.g[FP][0] ^ 2;
    c.g[A0][0] = pk_sp_addr(c, PK_GOAL_B);
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    c.g[S6][0] = pk_sp_addr(c, PK_GOAL_A);
    pk_sw_sp_reg(c, PK_CNT, A1);
    pk_sw_sp_reg(c, PK_GOAL_B_PTR, A0);
    c.g[A2][0] = pk_lw_sp(c, PK_CNT);
    let no_goals = c.g[A2][0] == 0;
    c.g[A1][0] = 0;
    if !no_goals {
        // Goal records of the first table (0x4781d0, 0x1b0 a peak, 0x6c a
        // goal): count the medals found.
        c.cycles += 3;
        c.g[V0][0] = 0x1b0;
        c.mult(0, S2, FP, V0);
        c.g[V0][0] = 0x6c;
        loop {
            c.cycles += 8;
            c.g[A2][0] = 0x0048_0000;
            c.mult(0, V0, A1, V0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7e30));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = c.g[V1][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S6][0];
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
            if aligned {
                pk_goal_strides(c, true, [A3, T0, A1, A2], 0x0020_42f4);
            } else {
                pk_goal_strides(c, false, [A3, T0, A1, A2], 0x0020_4294);
            }
            c.cycles += 19;
            pk_goal_tail(c, A3, T0);
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = pk_lw_sp(c, PK_GOAL_A + 8);
            c.g[A1][0] = 0;
            c.g[T0][0] = pk_lw_sp(c, PK_GOAL_A);
            c.g[A0][0] = c.g[S5][0];
            pk_call(c, PK_TRACK_MEDAL, 0x0020_4350)?;
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            c.g[A2][0] = pk_lw_sp(c, PK_CNT);
            c.g[A1][0] = c.g[S1][0];
            if c.g[V0][0] != 0 {
                c.g[S4][0] = c.g[S0][0];
            }
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0x6c;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_4268);
            }
        }
    }

    // The medal-goal count for this peak: shown with its "x of n" text.
    c.cycles += 20;
    c.g[A0][0] = 0x0047_0c10;
    pk_call(c, PK_HASH, 0x0020_4378)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4384)?;
    c.g[A3][0] = 0x004a_0000;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2398));
    c.g[A0][0] = c.g[S6][0];
    c.g[A3][0] = pk_lw_sp(c, PK_CNT);
    c.g[A2][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_43a0)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_43ac)?;
    c.g[T0][0] = pk_lw_sp(c, PK_CNT);
    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
    let short = c.g[V0][0] != 0;
    c.g[V1][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc20));
        pk_call(c, PK_HIDE, 0x0020_43e0)?;
    } else {
        c.cycles += 6;
        c.g[V0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc20));
        pk_call(c, PK_SHOW, 0x0020_43cc)?;
    }
    c.g[V0][0] = 0x21c;

    // Goal records of the second table (0x4786e0, 0x21c a peak): count.
    c.cycles += 5;
    c.g[S4][0] = 0;
    c.mult(0, S2, FP, V0);
    c.g[A1][0] = 0;
    c.g[V0][0] = 0x6c;
    loop {
        c.cycles += 8;
        c.g[A2][0] = 0x0048_0000;
        c.mult(0, V0, A1, V0);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7920));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = c.g[V1][0] & 7;
        let aligned = c.g[V0][0] == 0;
        c.g[A0][0] = pk_lw_sp(c, PK_GOAL_B_PTR);
        c.cycles += 3;
        c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
        if aligned {
            pk_goal_strides(c, true, [A3, T0, A1, A2], 0x0020_4484);
        } else {
            pk_goal_strides(c, false, [A3, T0, A1, A2], 0x0020_4424);
        }
        c.cycles += 18;
        pk_goal_tail(c, A3, T0);
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = pk_lw_sp(c, PK_GOAL_B + 8);
        c.g[A1][0] = 0;
        c.g[T0][0] = pk_lw_sp(c, PK_GOAL_B);
        c.g[A0][0] = c.g[S5][0];
        pk_call(c, PK_TRACK_MEDAL, 0x0020_44e0)?;
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.g[S4][0] = c.g[S0][0];
        }
        c.g[V0][0] = ((c.g[A1][0] as i64) < 5) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0x6c;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_43f8);
        }
    }

    c.cycles += 18;
    c.g[A0][0] = 0x0047_0c30;
    pk_call(c, PK_HASH, 0x0020_4504)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4510)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2660));
    c.g[A0][0] = c.g[S6][0];
    c.g[A2][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_4528)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_4534)?;
    c.g[V0][0] = ((c.g[S4][0] as i64) < 5) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc48));
        pk_call(c, PK_HIDE, 0x0020_4564)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc48));
        pk_call(c, PK_SHOW, 0x0020_4550)?;
    }
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);

    // Challenges and collectibles: two medals at most.
    c.cycles += 39;
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, 0x0015_43e0, 0x0020_4570)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[FP][0];
    c.g[A2][0] = 3;
    pk_call(c, 0x0015_8890, 0x0020_4584)?;
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);
    c.g[S1][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[S1][0] ^= 1;
    c.g[A1][0] = 0;
    c.g[A2][0] = c.g[FP][0];
    pk_call(c, 0x0015_35c0, 0x0020_459c)?;
    c.g[A2][0] = 3;
    c.g[A1][0] = c.g[FP][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[S0][0] = c.g[V0][0];
    pk_call(c, 0x0015_8870, 0x0020_45b0)?;
    c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A0][0] = 0x0047_0000;
    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc60));
    if c.g[S0][0] == 0 {
        c.g[S1][0] = c.g[V0][0];
    }
    pk_call(c, PK_HASH, 0x0020_45c8)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_45d4)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[A2][0] = c.g[S1][0];
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2668));
    c.g[A0][0] = c.g[S6][0];
    pk_call(c, PK_SPRINTF, 0x0020_45ec)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_45f8)?;
    c.g[S1][0] = ((c.g[S1][0] as i64) < 2) as u64;
    let few = c.g[S1][0] != 0;
    c.g[V0][0] = 0x0047_0000;
    if few {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc70));
        pk_call(c, PK_HIDE, 0x0020_4628)?;
    } else {
        c.cycles += 6;
        c.g[T0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc70));
        pk_call(c, PK_SHOW, 0x0020_4614)?;
    }
    c.g[A0][0] = pk_lw_sp(c, PK_ECON);

    // Earnings: total money earned against the peak's target.
    c.cycles += 24;
    c.g[A1][0] = 0;
    pk_call(c, 0x0015_09f8, 0x0020_4634)?;
    c.g[A1][0] = c.g[FP][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[S0][0] = c.g[V0][0];
    pk_call(c, 0x0015_88b0, 0x0020_4644)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc88));
    pk_call(c, PK_HASH, 0x0020_4654)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4660)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2670));
    c.g[A0][0] = c.g[S6][0];
    c.g[A2][0] = c.g[S1][0];
    pk_call(c, PK_SPRINTF, 0x0020_4678)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_4684)?;
    let not_met = (c.g[S1][0] as i64) <= 0;
    c.g[V1][0] = 0x0047_0000;
    if not_met {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc98));
        pk_call(c, PK_HIDE, 0x0020_46b0)?;
    } else {
        c.cycles += 5;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc98));
        pk_call(c, PK_SHOW, 0x0020_4698)?;
    }
    pk_leave(c, frame);
    Ok(())
}

/// The overall entry of the peak list (peak >= 3): the game-wide completion
/// panel. Counts the goals done over all peaks, the gold medals, the missions
/// and collectibles and the highlight levels, shows each with its check mark
/// and ends with the completion percentage.
pub(crate) fn pk_total_info(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 24;
    c.g[A1][0] = 0x0047_0568;
    pk_sw_sp(c, PK_DONE, 0);
    pk_call(c, PK_SHOW, 0x0020_46c8)?;
    c.g[S6][0] = 0;
    pk_owner_a0(c);
    c.g[A1][0] = 0x004a_2608;
    pk_call(c, PK_HIDE, 0x0020_46dc)?;
    c.g[A0][0] = 0x0047_0cb0;
    pk_call(c, PK_HASH, 0x0020_46e8)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_46f4)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xcc8));
    pk_call(c, PK_HASH, 0x0020_4704)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_SET_TEXT_ID, 0x0020_4710)?;
    c.g[FP][0] = pk_sp_addr(c, PK_BUF);
    c.g[S1][0] = 4;

    // Goals done over the three peaks (s6): table A (4 goals, 3 on the last
    // peak) and table B (5 goals), skipping the medal-only kinds.
    loop {
        c.cycles += 7;
        c.g[V1][0] = 3;
        c.g[V0][0] = c.g[S6][0] ^ 2;
        c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
        c.g[A1][0] = 0;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[V1][0];
        }
        let skip = c.g[S1][0] == 0;
        pk_sw_sp_reg(c, PK_NEXT, A3);
        if !skip {
            c.cycles += 7;
            c.g[V0][0] = 0x1b0;
            c.g[V1][0] = 0x0048_0000;
            c.mult(0, S4, S6, V0);
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e30));
            c.g[S2][0] = 1;
            c.g[V0][0] = 0x6c;
            loop {
                c.cycles += 6;
                c.mult(0, V0, A1, V0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.g[V0][0] = c.g[V1][0] & 7;
                let aligned = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[FP][0];
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                if aligned {
                    pk_goal_strides(c, true, [A1, A2, A3, T0], 0x0020_47cc);
                } else {
                    pk_goal_strides(c, false, [A1, A2, A3, T0], 0x0020_4770);
                }
                c.cycles += 10;
                pk_goal_tail(c, V0, A1);
                c.g[V1][0] = pk_lw_sp(c, PK_BUF + 8);
                c.g[V0][0] = c.g[V1][0] ^ 4;
                let t = c.g[V0][0] == 0;
                c.g[A1][0] = 0;
                if !t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] ^ 6;
                    let t = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[V1][0];
                    if !t {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[A0][0] ^ 7;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[A0][0] ^ 8;
                        if !t {
                            c.cycles += 1;
                            if c.g[V0][0] != 0 {
                                c.g[A1][0] = c.g[S2][0];
                            }
                        }
                    }
                }
                c.cycles += 2;
                let skip_goal = c.g[A1][0] == 0;
                c.g[T0][0] = pk_lw_sp(c, PK_BUF);
                if !skip_goal {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A3][0] = pk_lw_sp(c, PK_BUF + 8);
                    c.g[A1][0] = 0;
                    frame.save(c, T1, PK_SPILL_T1);
                    c.g[A2][0] = c.g[S7][0];
                    pk_call(c, PK_TRACK_MEDAL, 0x0020_485c)?;
                    c.g[T0][0] = pk_lw_sp(c, PK_DONE);
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    frame.restore(c, T1, PK_SPILL_T1);
                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                    if c.g[V0][0] == 0 {
                        c.g[T0][0] = c.g[V1][0];
                    }
                    pk_sw_sp_reg(c, PK_DONE, T0);
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = 0x6c;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_4750);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x21c;
        c.g[V1][0] = 0x0048_0000;
        c.mult(0, S2, S6, V0);
        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7920));
        c.g[A1][0] = 0;
        c.g[S1][0] = 1;
        c.g[V0][0] = 0x6c;
        loop {
            c.cycles += 6;
            c.mult(0, V0, A1, V0);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
            c.g[V0][0] = c.g[V1][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[FP][0];
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
            if aligned {
                pk_goal_strides(c, true, [A1, A2, A3, T0], 0x0020_491c);
            } else {
                pk_goal_strides(c, false, [A1, A2, A3, T0], 0x0020_48c0);
            }
            c.cycles += 10;
            pk_goal_tail(c, V0, A1);
            c.g[V1][0] = pk_lw_sp(c, PK_BUF + 8);
            c.g[V0][0] = c.g[V1][0] ^ 5;
            let t = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] ^ 9;
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[V1][0];
                if !t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] ^ 0xa;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[A0][0] ^ 0xb;
                    if !t {
                        c.cycles += 1;
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = c.g[S1][0];
                        }
                    }
                }
            }
            c.cycles += 2;
            let skip_goal = c.g[A1][0] == 0;
            c.g[T0][0] = pk_lw_sp(c, PK_BUF);
            if !skip_goal {
                c.cycles += 10;
                c.g[A0][0] = c.g[S5][0];
                c.g[A3][0] = pk_lw_sp(c, PK_BUF + 8);
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S7][0];
                pk_call(c, PK_TRACK_MEDAL, 0x0020_49a8)?;
                c.g[T0][0] = pk_lw_sp(c, PK_DONE);
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                if c.g[V0][0] == 0 {
                    c.g[T0][0] = c.g[V1][0];
                }
                pk_sw_sp_reg(c, PK_DONE, T0);
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = ((c.g[A1][0] as i64) < 5) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0x6c;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_48a0);
            }
        }
        c.cycles += 4;
        c.g[S6][0] = pk_lw_sp(c, PK_NEXT);
        c.g[V0][0] = ((c.g[S6][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = 4;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_4718);
        }
    }

    // "done / 14" and its check mark.
    c.cycles += 19;
    c.g[A0][0] = 0x0047_0cd8;
    pk_call(c, PK_HASH, 0x0020_49e8)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_49f4)?;
    c.g[A2][0] = pk_lw_sp(c, PK_DONE);
    c.g[A1][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2678));
    c.g[A0][0] = c.g[FP][0];
    pk_call(c, PK_SPRINTF, 0x0020_4a0c)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4a18)?;
    c.g[V1][0] = pk_lw_sp(c, PK_DONE);
    c.g[V0][0] = ((c.g[V1][0] as i64) < 14) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xce8));
        pk_call(c, PK_HIDE, 0x0020_4a4c)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xce8));
        pk_call(c, PK_SHOW, 0x0020_4a38)?;
    }
    c.g[A0][0] = 0x0047_0000;

    // Gold medals: twelve track medals (kind, track), "gold / 12".
    c.cycles += 124;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd00));
    pk_call(c, PK_HASH, 0x0020_4a58)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4a64)?;
    c.g[S1][0] = c.g[V0][0];
    const GOLD: [(u64, u64, u32); 12] = [
        (4, 0xe, 0x0020_4a80),
        (6, 0xe, 0x0020_4aa0),
        (5, 0xe, 0x0020_4ac4),
        (9, 0xe, 0x0020_4ae8),
        (4, 0xf, 0x0020_4b0c),
        (7, 0xf, 0x0020_4b30),
        (5, 0xf, 0x0020_4b54),
        (0xa, 0xf, 0x0020_4b78),
        (4, 0x10, 0x0020_4b9c),
        (8, 0x10, 0x0020_4bc0),
        (5, 0x10, 0x0020_4be4),
        (0xb, 0x10, 0x0020_4c08),
    ];
    for (i, (a3, t0, ret)) in GOLD.into_iter().enumerate() {
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = a3;
        c.g[T0][0] = t0;
        pk_call(c, PK_TRACK_MEDAL, ret)?;
        if i == 0 {
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
        } else {
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V1][0];
            }
        }
    }
    c.g[A1][0] = 0x004a_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2680));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S0][0];
    pk_call(c, PK_SPRINTF, 0x0020_4c28)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4c34)?;
    c.g[V0][0] = ((c.g[S0][0] as i64) < 12) as u64;
    let short = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd10));
        pk_call(c, PK_HIDE, 0x0020_4c64)?;
    } else {
        c.cycles += 6;
        c.g[T0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0xd10));
        pk_call(c, PK_SHOW, 0x0020_4c50)?;
    }
    c.g[A0][0] = 0x0047_0000;

    // Missions done / total over the three peaks.
    c.cycles += 20;
    c.lwc1(24, sp(c).wrapping_add(PK_DONE));
    c.cvt_s_w(24, 24);
    c.mtc1(S0, 25);
    c.cvt_s_w(25, 25);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd28));
    pk_call(c, PK_HASH, 0x0020_4c80)?;
    c.g[S2][0] = 0;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    c.g[S4][0] = 0;
    c.g[S0][0] = 0;
    pk_call(c, PK_FIND, 0x0020_4c98)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd40));
    pk_call(c, PK_HASH, 0x0020_4ca8)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_SET_TEXT_ID, 0x0020_4cb4)?;
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);
    loop {
        c.cycles += 11;
        c.g[A1][0] = c.g[S0][0];
        pk_call(c, 0x0015_43e0, 0x0020_4cc0)?;
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        c.g[A1][0] = c.g[S0][0];
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        pk_call(c, 0x0015_4368, 0x0020_4cd4)?;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        if event_due(c) {
            deliver_event_here(c, 0x0020_4cb8);
        }
    }
    c.cycles += 19;
    c.g[A0][0] = 0x0047_0d58;
    pk_call(c, PK_HASH, 0x0020_4cf0)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4cfc)?;
    c.g[V1][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2398));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_4d18)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4d24)?;
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 3;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xd68));
        pk_call(c, PK_HIDE, 0x0020_4d54)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xd68));
        pk_call(c, PK_SHOW, 0x0020_4d40)?;
    }

    // Collectibles found / total.
    c.cycles += 9;
    c.mtc1(S2, 23);
    c.cvt_s_w(23, 23);
    c.g[S0][0] = 0;
    c.mtc1(S4, 22);
    c.cvt_s_w(22, 22);
    c.g[S2][0] = 0;
    c.g[S4][0] = 0;
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);
    loop {
        c.cycles += 12;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        pk_call(c, 0x0015_35c0, 0x0020_4d84)?;
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        c.g[A1][0] = c.g[S0][0];
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        pk_call(c, 0x0015_3390, 0x0020_4d98)?;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        if event_due(c) {
            deliver_event_here(c, 0x0020_4d78);
        }
    }
    c.cycles += 19;
    c.g[A0][0] = 0x0047_0d80;
    pk_call(c, PK_HASH, 0x0020_4db4)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4dc0)?;
    c.g[T0][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2398));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_4ddc)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4de8)?;
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
    let short = c.g[V0][0] != 0;
    c.g[V1][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xd90));
        pk_call(c, PK_HIDE, 0x0020_4e18)?;
    } else {
        c.cycles += 6;
        c.g[V0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd90));
        pk_call(c, PK_SHOW, 0x0020_4e04)?;
    }
    c.g[A0][0] = 0x0047_0000;

    // Highlight levels (8 slots), "highlights / 24".
    c.cycles += 19;
    c.mtc1(S2, 21);
    c.cvt_s_w(21, 21);
    c.mtc1(S4, 20);
    c.cvt_s_w(20, 20);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xda8));
    pk_call(c, PK_HASH, 0x0020_4e34)?;
    c.g[S2][0] = 0;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    c.g[S0][0] = 0;
    pk_call(c, PK_FIND, 0x0020_4e48)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdc0));
    pk_call(c, PK_HASH, 0x0020_4e58)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_SET_TEXT_ID, 0x0020_4e64)?;
    c.g[A0][0] = pk_lw_sp(c, PK_SCORE);
    loop {
        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        pk_call(c, 0x0015_52b0, 0x0020_4e74)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 8) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = pk_lw_sp(c, PK_SCORE);
        if event_due(c) {
            deliver_event_here(c, 0x0020_4e68);
        }
    }
    c.cycles += 17;
    c.g[A1][0] = 0x004a_0000;
    c.g[A2][0] = c.g[S2][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2688));
    c.g[A0][0] = c.g[FP][0];
    pk_call(c, PK_SPRINTF, 0x0020_4e98)?;
    c.g[A0][0] = 0x0047_0dd8;
    pk_call(c, PK_HASH, 0x0020_4ea4)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4eb0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4ebc)?;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 24) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 3;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xde8));
        pk_call(c, PK_HIDE, 0x0020_4eec)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xde8));
        pk_call(c, PK_SHOW, 0x0020_4ed8)?;
    }

    // Completion percentage: (done/14 + gold/12 + missions + collectibles +
    // highlights/24) * 20, 100 only when exactly 100.0, else at most 99.
    c.cycles += 23;
    let a = (c.g[GP][0] as u32).wrapping_sub(0x552c);
    c.lwc1(0, a);
    c.div_s(7, 23, 22);
    c.lwc1(2, a.wrapping_add(4));
    c.div_s(6, 21, 20);
    c.mul_s(0, 24, 0);
    c.lwc1(3, a.wrapping_add(8));
    c.mul_s(2, 25, 2);
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(AT, 5);
    c.mtc1(S2, 1);
    c.cvt_s_w(1, 1);
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 4);
    c.add_s(0, 0, 2);
    c.mul_s(1, 1, 3);
    c.add_s(0, 0, 7);
    c.add_s(0, 0, 6);
    c.add_s(0, 0, 1);
    c.mul_s(1, 0, 5);
    c.c_eq_s(1, 4);
    let exactly_100 = c.fcc();
    c.g[S0][0] = 0x64;
    if !exactly_100 {
        c.cycles += 8;
        c.g[AT][0] = 0x42c6_0000;
        c.mtc1(AT, 0);
        c.cvt_w_s(2, 1);
        c.mfc1(S0, 2);
        c.c_le_s(0, 1);
        if c.fcc() {
            c.g[S0][0] = 0x63;
        }
    }
    c.cycles += 16;
    c.g[A0][0] = 0x0047_0e00;
    pk_call(c, PK_HASH, 0x0020_4f74)?;
    c.g[A0][0] = c.lw((pk_this(c)).wrapping_add(PK_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4f80)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2690));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S0][0];
    pk_call(c, PK_SPRINTF, 0x0020_4f98)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4fa4)?;
    pk_leave(c, frame);
    Ok(())
}

/// `EventSelect_OnWidgetCreated` (0x002009d0): Widget-created hook of the event-select map: when the widget's label is the peak list, selects menu entry 2 (map style < 2) or `2 - location.peak`. `(self, menu)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn event_select_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_200a58: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x0014dc80, 0x002009ec)?; // call_indirect
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x002009f8)?; // call_indirect
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
            call_ptr(c, 0x00317670, 0x00200a0c)?; // call_indirect
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_200a58;
            } else {
                'b_200a34: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_200a34;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0039b960, 0x00200a2c)?; // call_indirect
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_200a58;
                        } else {
                            break 'b_200a34;
                        }
                    }
                }
                c.cycles += 9;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00144bc0, 0x00200a3c)?; // call_indirect
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x54));
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[A1][0] = c.g[A1][0] & 0xff;
                call_ptr(c, 0x0039b960, 0x00200a54)?; // call_indirect
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_200a58;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `EventSelect_Update` (0x00202828): Per-frame update of the event-select map widget: fades the map picture in, shows or hides the peak outlines / indicators and the "you are here" markers, and applies the async-loaded map texture once its slot is loaded. `(self)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn event_select_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_2029bc: {
            'b_20294c: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00202850)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x0020285c)?; // call_indirect
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2dc));
                if c.g[V0][0] == 0u64 {
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2e0), 0u64);
                    break 'b_20294c;
                } else {
                    'b_202898: {
                        c.cycles += 7;
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x2e0));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.c_lt_s(1, 0);
                        if !c.fcc() {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                            break 'b_202898;
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5530));
                            c.add_s(0, 1, 0);
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2e0));
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                            break 'b_202898;
                        }
                    }
                    'b_202938: {
                        c.cycles += 26;
                        c.g[A1][0] = c.g[SP][0];
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x2e0));
                        c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x23));
                        c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x2b));
                        c.ldr(3, (c.g[A0][0] as u32).wrapping_add(0x24));
                        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(2, c.g[SP][0] as u32);
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(0, c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[A2][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        call_ptr(c, t, 0x002028d8)?; // call_indirect
                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002028f4)?; // call_indirect
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        if c.g[V0][0] != 0u64 {
                            c.g[A1][0] = 0x470000;
                            break 'b_202938;
                        } else {
                            c.cycles += 14;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            c.g[A1][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x588));
                            call_ptr(c, 0x0039e948, 0x00202910)?; // call_indirect
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            c.g[A1][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x598));
                            call_ptr(c, 0x0039e948, 0x00202920)?; // call_indirect
                            c.g[A1][0] = 0x470000;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a8));
                            call_ptr(c, 0x0039e948, 0x00202930)?; // call_indirect
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
                            if t {
                                break 'b_2029bc;
                            } else {
                                break 'b_202938;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a0));
                    call_ptr(c, 0x0039e948, 0x00202944)?; // call_indirect
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
                    if t {
                        break 'b_2029bc;
                    } else {
                        break 'b_20294c;
                    }
                }
            }
            'b_2029ac: {
                c.cycles += 10;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00202968)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                if c.g[V0][0] != 0u64 {
                    c.g[A1][0] = 0x470000;
                    break 'b_2029ac;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x588));
                    call_ptr(c, 0x0039e8b8, 0x00202984)?; // call_indirect
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x598));
                    call_ptr(c, 0x0039e8b8, 0x00202994)?; // call_indirect
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a8));
                    call_ptr(c, 0x0039e8b8, 0x002029a4)?; // call_indirect
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
                    if t {
                        break 'b_2029bc;
                    } else {
                        break 'b_2029ac;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a0));
            call_ptr(c, 0x0039e8b8, 0x002029b8)?; // call_indirect
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2f0));
            break 'b_2029bc;
        }
        'b_202ab8: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_202ab8;
            } else {
                c.cycles += 5;
                c.g[S1][0] = 0u64;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S4][0] = 0x470000;
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                'l_2029d8: loop {
                    'b_202a4c: {
                        'b_202a40: {
                            'b_202a3c: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[S3][0] = c.g[A2][0];
                                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7b0));
                                call_ptr(c, 0x00417828, 0x002029e8)?; // call_indirect
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2dc));
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                                if t {
                                    break 'b_202a3c;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                                        break 'b_202a40;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S5][0];
                                        call_ptr(c, 0x00144bc0, 0x00202a04)?; // call_indirect
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
                                        if c.g[S1][0] != c.g[V1][0] {
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                                            break 'b_202a40;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                                            if t {
                                                break 'b_202a40;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                                                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_202a40;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[A1][0] = c.g[S2][0];
                                                    call_ptr(c, 0x0039e948, 0x00202a34)?; // call_indirect
                                                    let t = true;
                                                    c.g[S1][0] = c.g[S3][0];
                                                    if t {
                                                        break 'b_202a4c;
                                                    } else {
                                                        break 'b_202a3c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                            break 'b_202a40;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x00202a48)?; // call_indirect
                        c.g[S1][0] = c.g[S3][0];
                        break 'b_202a4c;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002029d8);
                        }
                        continue 'l_2029d8;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x001a39c0, 0x00202a6c)?; // call_indirect
                        c.g[V1][0] = 0x3;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            break 'b_202ab8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2dc));
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                break 'b_202ab8;
                            } else {
                                'b_202ab4: {
                                    c.cycles += 3;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[V1][0] = 0x11c;
                                    if t {
                                        break 'b_202ab4;
                                    } else {
                                        c.cycles += 9;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2d8));
                                        c.g[A1][0] = 0x1;
                                        c.mult(0, 4, 2, 3);
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x110));
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x7c), 0u64);
                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x2dc), c.g[A1][0]);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x2e0), 0u64);
                                        break 'b_202ab4;
                                    }
                                }
                                c.cycles += 1;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                break 'b_202ab8;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `int EventSelect_IsEventCompleted(void* self, sPeakGoal goal, int peak)`
/// (0x200b50), the C++ `cUITemplate_MAP_isMapEventCompleted`. `goal` is a
/// 0x6c-byte struct passed by value (`location` at +0, `gameEvent` at +8).
/// Location 23 is never completed; event types 4 and 5 (the peak-1/2/3 mission
/// events) are completed when the player interface says the event for that
/// peak is not locked; every other event is completed when the character has a
/// track medal for it.
pub fn event_select_is_event_completed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[V0][0] = c.g[A1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        frame.save_ra(c, 0x70);
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
        if aligned {
            oc738_copy_blocks(c, V1, A0, V0, true, [A1, A2, A3, T0], 0x0020_0bd4);
        } else {
            oc738_copy_blocks(c, V1, A0, V0, false, [A1, A2, A3, T0], 0x0020_0b7c);
            c.cycles += 2;
        }

        // The trailing 12 bytes of the struct.
        c.cycles += 23;
        let src = c.g[V1][0] as u32;
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[A1][0] = Addr(src.wrapping_add(0x8)).lw(c);
        let dst = c.g[A0][0] as u32;
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[A1][0]);
        call_ptr(c, 0x0014_dc80, 0x0020_0c1c)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0020_0c28)?; // BELibrary_GetInterface(be, 1)
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0020_0c30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0020_0c3c)?; // BELibrary_GetInterface(be, 13)
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x0020_0c48)?; // BELibrary_GetCharacterID(0)
        c.g[A2][0] = c.g[V0][0];
        c.g[T0][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 0x17;
        let not_summit = c.g[T0][0] != c.g[V0][0];
        c.g[A3][0] = Addr(sp(c).wrapping_add(0x8)).lw(c);
        if !not_summit {
            c.cycles += 2;
            c.g[V0][0] = 0;
            oc738_event_completed_exit(c, frame);
            return Ok(());
        }

        c.cycles += 3;
        c.g[V0][0] = 0x4;
        let not_four = c.g[A3][0] != c.g[V0][0];
        c.g[V0][0] = 0x5;
        if !not_four {
            c.cycles += 7;
            c.g[V1][0] = c.g[S1][0] ^ 0x1;
            c.g[A0][0] = 0x7;
            c.g[S0][0] = 0x6;
            c.g[A1][0] = c.g[S1][0] ^ 0x2;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            c.g[V0][0] = 0x8;
        } else {
            c.cycles += 2;
            let is_five = c.g[A3][0] == c.g[V0][0];
            c.g[V1][0] = c.g[S1][0] ^ 0x1;
            if !is_five {
                // Any other event type: a track medal exists.
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0015_8910, 0x0020_0ca0)?; // getTrackMedal(rw, 0, cid, event, loc)
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                oc738_event_completed_exit(c, frame);
                return Ok(());
            }
            c.cycles += 5;
            c.g[A0][0] = 0xa;
            c.g[S0][0] = 0x9;
            c.g[A1][0] = c.g[S1][0] ^ 0x2;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            c.g[V0][0] = 0xb;
        }

        // Event types 4 and 5: the event index for the peak is in S0.
        c.cycles += 9;
        if c.g[A1][0] == 0 {
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x0020_0ccc)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_6a70, 0x0020_0ce0)?; // isEventLocked_b(np, 0, cid, idx)
        c.g[V0][0] ^= 0x1;
        oc738_event_completed_exit(c, frame);
        Ok(())
    });
}

/// Epilogue of `EventSelect_IsEventCompleted`.
pub(crate) fn oc738_event_completed_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore_ra(c, 0x70);
    frame.close(c);
}

/// `int EventSelect_TransitionIn(EventSelect* self, int enter)` (0x200a70):
/// when `enter != 0` sets the screen mode from the global at `gp-0xb54`,
/// builds the peak texts and the event list and sets the map tab mode from
/// `*self`; returns 0.
pub fn event_select_transition_in(c: &mut Cpu) {
    'b_200aa8: {
        c.cycles += 5;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let t = c.g[A1][0] == 0u64;
        c.g[S0][0] = c.g[A0][0];
        if t {
            break 'b_200aa8;
        } else {
            c.cycles += 9;
            c.g[RA][0] = sx(0x00200a8c);
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
            c.pc = 0x002030a0;
            ee::s05::f_002030a0(c);
            if c.pc != 0x00200a8c {
                return;
            }
            c.g[RA][0] = sx(0x00200a94);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x00200de8;
            ee::s05::f_00200de8(c);
            if c.pc != 0x00200a94 {
                return;
            }
            c.g[RA][0] = sx(0x00200a9c);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x00207430;
            ee::s05::f_00207430(c);
            if c.pc != 0x00200a9c {
                return;
            }
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00200aa8);
            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
            c.pc = 0x00203cb8;
            ee::s05::f_00203cb8(c);
            if c.pc != 0x00200aa8 {
                return;
            }
            break 'b_200aa8;
        }
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = 0u64;
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) const OC758_CTRL: u32 = crate::difftest::DATA + 0x20_0000;

#[cfg(test)]
pub(crate) const OC758_MAGIC: u32 = 0x0758_c0de;

/// `void EventSelect_Init(sMapTmpl* this, void* screen, void* owner)` (0x200700).
///
/// Sets up the map template of the event select screen: stores `screen`
/// (`+0x2e8`) and `owner` (`+0x2ec`); opens the PDA template for map styles
/// 1, 3 and 4; clears the map state unless the front-end start of style 0;
/// resets the mode, popup and file fields, looks up the `MapPic` widget (hash
/// of its name) and hides it; then sets up the peak locks, flags, mode panels,
/// map image, popup and (style >= 2) the peak list, and finally hides or shows
/// the named widgets of the owner screen, which ones depending on the map
/// style (`gp - 0xb5c`).
pub fn event_select_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_200744: {
            'b_200738: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x1;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2ec), c.g[A2][0]);
                let t = c.g[V1][0] == c.g[V0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2e8), c.g[A1][0]);
                if t {
                    break 'b_200738;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x4;
                    if t {
                        break 'b_200738;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                        if t {
                            break 'b_200744;
                        } else {
                            break 'b_200738;
                        }
                    }
                }
            }
            c.cycles += 3;
            call_ptr(c, 0x0020d110, 0x00200740)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
            break 'b_200744;
        }
        'b_200760: {
            'b_20075c: {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb54), 0u64);
                    break 'b_20075c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb58));
                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = 0u64;
                        break 'b_200760;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0xb54), 0u64);
                        break 'b_20075c;
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = 0u64;
            break 'b_200760;
        }
        'b_2007c4: {
            c.cycles += 19;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00203cb8, 0x00200768)?;
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2cc), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d0), c.g[V0][0]);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
            call_ptr(c, 0x001a3bf0, 0x00200780)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), 0u64);
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25a8));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d8), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x00200794)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e8));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002007a0)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2e4), c.g[A2][0]);
            if t {
                break 'b_2007c4;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002007c4)?;
                break 'b_2007c4;
            }
        }
        'b_2008b8: {
            'b_200898: {
                'b_200818: {
                    c.cycles += 15;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2f0), 0u64);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00201750, 0x002007d0)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00201720, 0x002007d8)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00202ad8, 0x002007e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00203990, 0x002007e8)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00203a58, 0x002007f0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                        break 'b_200818;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00203d40, 0x00200808)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                        if t {
                            break 'b_200898;
                        } else {
                            break 'b_200818;
                        }
                    }
                }
                'b_200854: {
                    c.cycles += 10;
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c8));
                    call_ptr(c, 0x0039e8b8, 0x00200824)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25b0));
                    call_ptr(c, 0x0039e8b8, 0x00200834)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    if t {
                        break 'b_200854;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25b8));
                        call_ptr(c, 0x0039e8b8, 0x0020084c)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                        if t {
                            break 'b_2008b8;
                        } else {
                            break 'b_200854;
                        }
                    }
                }
                c.cycles += 17;
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4d8));
                call_ptr(c, 0x0039e8b8, 0x00200860)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e8));
                call_ptr(c, 0x0039e8b8, 0x00200870)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4f8));
                call_ptr(c, 0x0039e8b8, 0x00200880)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x508));
                call_ptr(c, 0x0039e8b8, 0x00200890)?;
                let t = true;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                if t {
                    break 'b_2008b8;
                } else {
                    break 'b_200898;
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25c0));
            call_ptr(c, 0x0039e8b8, 0x002008a4)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x518));
            call_ptr(c, 0x0039e948, 0x002008b4)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
            break 'b_2008b8;
        }
        'b_200904: {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
            if t {
                break 'b_200904;
            } else {
                c.cycles += 16;
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x528));
                call_ptr(c, 0x0039e8b8, 0x002008d0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x538));
                call_ptr(c, 0x0039e8b8, 0x002008e0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25c8));
                call_ptr(c, 0x0039e8b8, 0x002008f0)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25d0));
                call_ptr(c, 0x0039e8b8, 0x00200900)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                break 'b_200904;
            }
        }
        c.cycles += 51;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25d8));
        call_ptr(c, 0x0039e8b8, 0x00200910)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x548));
        call_ptr(c, 0x0039e8b8, 0x00200920)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x558));
        call_ptr(c, 0x0039e8b8, 0x00200930)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x568));
        call_ptr(c, 0x0039e8b8, 0x00200940)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x578));
        call_ptr(c, 0x0039e8b8, 0x00200950)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x588));
        call_ptr(c, 0x0039e8b8, 0x00200960)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x598));
        call_ptr(c, 0x0039e8b8, 0x00200970)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a8));
        call_ptr(c, 0x0039e8b8, 0x00200980)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b8));
        call_ptr(c, 0x0039e8b8, 0x00200990)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5c8));
        call_ptr(c, 0x0039e8b8, 0x002009a0)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5d8));
        call_ptr(c, 0x0039e8b8, 0x002009b0)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5e8));
        call_ptr(c, 0x0039e8b8, 0x002009c0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `StrHash(name)` then `UIScreen_FindObject(screen, hash)` of the map UI
/// (`this` in s1, screen at `+0x2e8`); the object comes back in `v0`.
pub(crate) fn popup_find_by_name(c: &mut Cpu, name: u32, hash_ret: u32, find_ret: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, hash_ret)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, find_ret)
}

/// `cUIText_setUnicodeStringByID(obj (s2 or s0), StrHash(name))`: the hash
/// call, then the string set (0x3a1000) on the object in register `obj`.
pub(crate) fn popup_set_text(
    c: &mut Cpu,
    obj: usize,
    name: u32,
    hash_ret: u32,
    set_ret: u32,
) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, hash_ret)?;
    c.g[A0][0] = c.g[obj][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x003a_1000, set_ret)
}

/// Virtual call `obj->vt[off](obj + delta, a1)` on the object in s0 (`a1`
/// set by the caller).
pub(crate) fn popup_s0_vcall(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// `void EventSelect_UpdateConfirmPopup(cUITemplate_MAP* this)` (0x203a58)
/// (reference: `cUITemplate_MAP_setupPopup`).
///
/// Shows or hides the map screen's transport confirm popup, by the popup flag
/// at `+0x2cc`. Active: show "ConfirmPopup", select item 0 of "ConfirmMenu",
/// and (if still active) swap "Popup1"/"Popup2"/"maptab" and set the
/// "P2_TransportNow" text by the map state (gp-0xb54); then label the Yes
/// button and, by the popup type (`+0x2d0`), show or hide the No button.
/// Inactive: hide "ConfirmPopup", show "maptab".
pub fn event_select_update_confirm_popup(c: &mut Cpu) {
    if c.pc != 0x0020_3a58 {
        return ee::s05::f_00203a58__entries(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_ACTIVE));
        let inactive = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
        if inactive {
            c.cycles += 8;
            c.g[A1][0] = sx(0x0047_0a60);
            call_ptr(c, 0x0039_e8b8, 0x0020_3c8c)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
            c.g[A1][0] = sx(0x004a_25b0);
            call_ptr(c, 0x0039_e948, 0x0020_3c9c)?;
            frame.restore(c, 16, 0x40);
        } else {
            c.cycles += 11;
            c.g[A1][0] = sx(0x0047_0a60);
            call_ptr(c, 0x0039_e948, 0x0020_3a8c)?;
            popup_find_by_name(c, 0x0047_0a20, 0x0020_3a98, 0x0020_3aa4)?;
            let no_menu = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !no_menu {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0039_b960, 0x0020_3ab4)?;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_ACTIVE));
            let still_active = c.g[V0][0] != 0;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
            if still_active {
                c.cycles += 21;
                c.g[A1][0] = sx(0x004a_2638);
                call_ptr(c, 0x0039_e8b8, 0x0020_3acc)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = sx(0x004a_2640);
                call_ptr(c, 0x0039_e948, 0x0020_3adc)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = sx(0x004a_25b0);
                call_ptr(c, 0x0039_e8b8, 0x0020_3aec)?;
                popup_find_by_name(c, 0x0047_0a70, 0x0020_3af8, 0x0020_3b04)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                let map_state_set = c.g[V0][0] != 0;
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    // Map state 0: "kT_OVRCMNGoPeakNow", else "kT_MAPTransArea".
                    if map_state_set {
                        c.cycles += 7;
                        popup_set_text(c, S0, 0x0047_0a98, 0x0020_3b50, 0x0020_3b5c)?;
                    } else {
                        c.cycles += 8;
                        popup_set_text(c, S0, 0x0047_0a80, 0x0020_3b28, 0x0020_3b34)?;
                    }
                }
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
            }
            // The Yes/No buttons of the confirm menu.
            c.cycles += 22;
            c.g[A1][0] = sx(0x0047_0700);
            call_ptr(c, 0x0039_e8b8, 0x0020_3b6c)?;
            popup_find_by_name(c, 0x0047_06b8, 0x0020_3b78, 0x0020_3b84)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = 0x470000;
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c8));
            let no_button_name = c.g[S3][0] as u32;
            popup_find_by_name(c, no_button_name, 0x0020_3b98, 0x0020_3ba4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_TYPE));
            c.g[V0][0] = 0x1;
            let not_ok_only = c.g[V1][0] != c.g[V0][0];
            if !not_ok_only {
                // Type 1: "OK" label, hide and freeze the No button.
                c.cycles += 2;
                let no_title = c.g[S2][0] == 0;
                c.g[A0][0] = 0x470000;
                if !no_title {
                    c.cycles += 5;
                    popup_set_text(c, S2, 0x0046_faa0, 0x0020_3bc8, 0x0020_3bd4)?;
                }
                c.cycles += 2;
                let no_no_button = c.g[S0][0] == 0;
                c.g[A1][0] = 0;
                if !no_no_button {
                    c.cycles += 11;
                    popup_s0_vcall(c, 0x48, 0x0020_3bf0)?;
                    c.g[A1][0] = 0x1;
                    popup_s0_vcall(c, 0x40, 0x0020_3c08)?;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039_e8b8, 0x0020_3c14)?;
            } else {
                // Other types: "Yes" label, show the No button.
                c.cycles += 2;
                let no_title = c.g[S2][0] == 0;
                c.g[A0][0] = 0x470000;
                if !no_title {
                    c.cycles += 5;
                    popup_set_text(c, S2, 0x0047_0aa8, 0x0020_3c2c, 0x0020_3c38)?;
                }
                c.cycles += 2;
                let no_no_button = c.g[S0][0] == 0;
                c.g[A1][0] = 0x1;
                if !no_no_button {
                    c.cycles += 11;
                    popup_s0_vcall(c, 0x48, 0x0020_3c54)?;
                    c.g[A1][0] = 0;
                    popup_s0_vcall(c, 0x40, 0x0020_3c6c)?;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039_e948, 0x0020_3c78)?;
            }
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const POPUP_TYPE: u32 = 0x2d0;

pub(crate) const POPUP_SCREEN: u32 = 0x2e8;

pub(crate) const POPUP_STATE: u32 = 0x2ec;

/// `void cUITemplate_MAP::onInputAlways(cUITemplate_MAP* this, cUIInput* in)`
/// (0x202770, frame 0x30): the event-select map screen's per-frame input hook.
/// Asks the input object (`a1`) its two virtual predicates (`isUp`, `isDown`,
/// thiscall tables at +0x88/+0x8c and +0x90/+0x94 of its class); if neither is
/// set it does nothing. Otherwise, depending on the map state word at
/// `gp-0xb54` (0: peak list, 1: peak goals, 2: event list) it refreshes the
/// matching list (`EventSelect_RefreshPeakList` / `RefreshPeakGoals` +
/// `UpdateModePanels`, or `RefreshEventList`).
pub fn event_select_post_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_202814: {
            'b_202810: {
                'b_2027c0: {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0020279c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0020279c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        break 'b_2027c0;
                    } else {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002027b8);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002027b8)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        if t {
                            break 'b_202810;
                        } else {
                            break 'b_2027c0;
                        }
                    }
                }
                'b_2027dc: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        break 'b_2027dc;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x002027d0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00203d40, 0x002027d0)?;
                        c.g[RA][0] = sx(0x002027d8);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00202ad8, 0x002027d8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        break 'b_2027dc;
                    }
                }
                'b_2027fc: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        break 'b_2027fc;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x002027f0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00205098, 0x002027f0)?;
                        c.g[RA][0] = sx(0x002027f8);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00202ad8, 0x002027f8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        break 'b_2027fc;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_202814;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00202810);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00207430, 0x00202810)?;
                    break 'b_202810;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_202814;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `EventSelect_InitPeakLocks(self)` (0x00201750): mark the three peaks of
/// the event-select map unlocked (`+4/+8/+0xc = 1`), then for peaks 1 and 2
/// query the back end (interfaces 0 and 1 of `cBE`): in two-player path mode
/// (byte `0x535c11 == 2`) a peak stays unlocked unless both players have it
/// locked, otherwise the single-player lock query decides; a locked peak
/// clears `self->+8 + 4*(peak-1)`.
///
/// The resume entries (`pc != entry`) stay with the generated `__entries`.
pub fn event_select_init_peak_locks(c: &mut Cpu) {
    if c.pc != 0x0020_1750 {
        return ee::s05::f_00201750__entries(c);
    }
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        frame.save(c, S4, 0x20);
        c.g[S3][0] = 1;
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S5][0] = 2;
        call_ptr(c, 0x0014_dc80, 0x0020_1784)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0020_1790)?;
        call_ptr(c, 0x0014_dc80, 0x0020_1798)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0020_17a4)?;
        c.g[V1][0] = 1;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0x53_0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
        c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            let single = c.g[V0][0] != c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            let mut check_lock = true;
            if single {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_6008, 0x0020_1828)?;
            } else {
                c.cycles += 10;
                call_ptr(c, 0x0014_7488, 0x0020_17dc)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x0014_5f90, 0x0020_17f0)?;
                c.g[A0][0] = c.g[S1][0];
                let first_free = c.g[V0][0] == 0;
                c.g[A1][0] = 1;
                if first_free {
                    check_lock = false;
                } else {
                    c.cycles += 9;
                    call_ptr(c, 0x0014_7488, 0x0020_1804)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0014_5f90, 0x0020_1818)?;
                }
            }
            if check_lock {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(c.g[S2][0] as u32, 0);
                }
            }
            c.cycles += 4;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 3) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0020_17c0);
                }
                continue;
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
            return Ok(());
        }
    })
}

/// `EventSelect_ResetFlags(EventSelect* s)` (0x201720): clears +0x1c, sets
/// +0x10 and +0x14 to 1, and sets +0x18 to 1 when the gp word at -0xb5c is
/// below 2 (unsigned), else 0.
pub fn event_select_reset_flags(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(EVENT_SELECT_MODE_GP));
    c.g[V1][0] = 0x1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), 0u64);
    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
    let flag_zero = c.g[V0][0] == 0u64;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
    c.cycles += 2;
    if flag_zero {
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
    } else {
        c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
    }
    ret(c);
}

/// The gp-relative word (gp - this) `EventSelect_ResetFlags` tests.
pub(crate) const EVENT_SELECT_MODE_GP: u32 = 0xb5c;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc774_install_fakes;
    pub(crate) use crate::game::ui::uimenu::tests::oc776_install_plain_fakes;
    pub(crate) use crate::game::visualfx::renderstateman::tests::oc758_install_fakes;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;

    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::LIST;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::fememcard::tests::vtable_at;
    pub(crate) use crate::game::fe::festateload::tests::notify_setup;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    pub(crate) use crate::game::movie::movieplayer::tests::w as w_2;

    use ssx3_ee as ee;

    #[test]
    fn event_select_on_widget_notify_matches() {
        check(
            ssx3_ee::s05::f_00202738,
            event_select_on_widget_notify,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A2][0] = match seed % 4 {
                    0 => 6,
                    1 => 9,
                    2 => 8,
                    _ => 0x100,
                };
            },
        );
    }

    #[test]
    fn ov_screen_map_handle_input_matches() {
        check(
            ee::s05::f_00208638__generated,
            ov_screen_map_handle_input,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20_8638, 0);
                c.g[A0][0] = ptr(&mut r) as u64;
            },
        );
    }

    #[test]
    fn ov_screen_map_on_widget_notify_matches() {
        check(
            ssx3_ee::s05::f_00208610__generated,
            ov_screen_map_on_widget_notify,
            150,
            &|c, seed| notify_setup(c, seed, 0x18),
        );
    }

    #[test]
    fn ov_screen_map_on_activate_matches() {
        check(
            ssx3_ee::s05::f_00208558__generated,
            ov_screen_map_on_activate,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xad, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // EventSelect mode 0 (the map tab).
                c.bus.write32(GAME_GP - 0xb54, 0);
            },
        );
    }

    #[test]
    fn ov_screen_map_update_matches() {
        check(
            ssx3_ee::s05::f_00208678__generated,
            ov_screen_map_update,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xae, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP - 0xb54, 0);
            },
        );
    }

    #[test]
    fn event_select_on_activate_matches() {
        check(
            ee::s05::f_00200ac0__generated,
            event_select_on_activate,
            40,
            &|c, seed| {
                machine(c, seed, 0x200ac0, 4);
                globals_2(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                // The map mode the widget is put into (`D_004A259C`).
                c.bus
                    .write32(GAME_GP.wrapping_sub(0xb54), (seed % 4) as u32);
            },
        );
    }

    #[test]
    fn ov_screen_map_shutdown_matches() {
        check(
            ee::s05::f_00208588__generated,
            ov_screen_map_shutdown,
            40,
            &|c, seed| {
                machine(c, seed, 0x208588, 6);
                globals_2(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn event_select_hide_map_pic_matches() {
        check(
            ssx3_ee::s05::f_00201868__generated,
            event_select_hide_map_pic,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30, 1);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x2e4, 0);
                } else {
                    c.bus.write32(DATA + 0x2e4, DATA + 0x3000);
                    vtable_at(c, &mut r, DATA + 0x3000, 0x48);
                }
            },
        );
    }

    #[test]
    fn event_select_set_highlight_color_matches() {
        check(
            ssx3_ee::s05::f_002083d8__generated,
            event_select_set_highlight_color,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2083d8, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x58, 0);
                c.g[A0][0] = OBJ2 as u64;
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { OBJ as u64 };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn event_select_set_unlocked_color_matches() {
        check(
            ssx3_ee::s05::f_00208438__generated,
            event_select_set_unlocked_color,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x208438, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x58, 0);
                c.g[A0][0] = OBJ2 as u64;
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { OBJ as u64 };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn event_select_shutdown_matches() {
        check(
            ssx3_ee::s05::f_00200af0__generated,
            event_select_shutdown,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x200af0, 0);
                w(c, GAME_GP - 0x848, 0);
                w(
                    c,
                    GAME_GP - 0xb5c,
                    match seed % 4 {
                        0 => 1,
                        1 => 3,
                        2 => 4,
                        _ => seed,
                    },
                );
                w(c, OBJ + 0x2d8, OBJ2);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // ---------------------------------------------------------------- overlay

    #[test]
    fn ov_screen_map_init_matches() {
        check(
            ee::s05::f_00208498__generated,
            ov_screen_map_init,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2084_98, 4);
                let engine = DATA + 0x1000;
                w_2(c, DATA + 0x10, engine);
                w_2(c, engine, DATA + 0x1100);
                w_2(c, engine + 4, 0);
                w_2(c, DATA + 0x1100, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn event_select_get_medal_text_matches() {
        check(
            ee::s05::f_00204ff0__generated,
            event_select_get_medal_text,
            10,
            &|c, seed| {
                let r = base_2(c, seed, 0x204ff0, 2);
                c.g[A1][0] = (seed % 6) as u64;
                w_3(c, WORLD + 0x8c, OBJ2_2);
                w_3(c, OBJ2_2 + 4, LIST);
                w_3(c, LIST + 0x20, 0);
                w_3(c, LIST + 0x24, 0);
                let _ = r;
            },
        );
    }

    #[test]
    fn event_select_is_event_unlocked_matches() {
        diff(
            ee::s05::f_00200d00__generated,
            event_select_is_event_unlocked,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x20_0d00);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = (seed % 5) as u64;
                w_4(
                    c,
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { DATA + 0x3000 },
                );
                w_4(c, DATA + 0x2000, r.next_u32());
                w_4(c, DATA + 0x3000, r.next_u32());
            },
        );
    }

    /// `EventSelect_OnWidgetAction` against the generated body. Twenty
    /// setups (`seed % 20`) steer it into every action and panel: row
    /// colours (with/without the confirm popup, every mode, the highlight
    /// object found or not), accept on each panel (deny, pick peak/tab,
    /// the challenge record copies aligned and not, the free ride, starting
    /// an event with and without the rule-flag reset, the confirm popup's
    /// special/missions/free-ride and overlay/front-end flows), back, the
    /// map-tab toggle, a null widget, out-of-range actions and an unknown
    /// jump-table target. The real callees take their short paths: the
    /// screen (`+0x2e8`) is null except where the highlight/toggle needs an
    /// object, the UI state has an empty list, and the BELibrary interfaces,
    /// sound player, widget and object vtables end in fakes.
    #[test]
    fn event_select_on_widget_action_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::game::be::bedbreward::tests::alloc_ready;
        // Fixture objects (above the fake-pointer and heap ranges).
        const T_ES: u32 = 0x008a_0000;
        const T_WIDGET: u32 = 0x008a_1000;
        const T_STATE: u32 = 0x008a_2000;
        const T_SOUND: u32 = 0x008a_2100;
        const T_PLAYER: u32 = 0x008a_2200;
        const T_PVT: u32 = 0x008a_2300;
        const T_WVT: u32 = 0x008a_2400;
        const T_SCREEN: u32 = 0x008a_3000;
        const T_COUNT: u32 = 0x008a_3100;
        const T_LIST: u32 = 0x008a_3200;
        const T_OBJ: u32 = 0x008a_4000;
        const T_OVT: u32 = 0x008a_5000;
        const T_WORLD: u32 = 0x008a_6000;
        const T_GM: u32 = 0x008a_7000;
        const T_GM_STATE: u32 = 0x008a_7400;
        const T_REPLAY: u32 = 0x008a_7800;
        const T_MODE_MAN: u32 = 0x008a_8000;
        const T_ASYNC: u32 = 0x008a_9000;
        const T_AUDIO: u32 = 0x008b_0000;
        const T_BELIB: u32 = 0x008b_8000;
        const T_APP: u32 = 0x008b_9000;
        const NEG1: u32 = 0xffff_ffff;
        check(
            ssx3_ee::s05::f_002018a8__generated,
            event_select_on_widget_action,
            480,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_18a8, 6);
                alloc_ready(c);
                let pick = |r: &mut Rng, xs: &[u32]| xs[r.next_u32() as usize % xs.len()];
                let g = |off: i32| GAME_GP.wrapping_add(off as u32);

                // Globals: BELibrary dummy, world, audio system, default
                // sound player, the app (QuitToLoadScreen's next state).
                c.bus.write32(g(-0x1f04), T_BELIB);
                c.bus.write32(g(-0x848), T_WORLD);
                c.bus.write32(g(0x410), T_AUDIO);
                c.bus.write32(g(0x2968), T_PVT);
                c.bus.write32(0x004a_28a8, T_APP);
                c.bus.write32(T_WORLD + 0x78, T_BELIB);
                c.bus.write32(T_WORLD + 0x84, T_GM);
                c.bus.write32(T_WORLD + 0xc0, T_MODE_MAN);
                c.bus.write32(T_WORLD + 0x11c, T_ASYNC);
                c.bus.write32(T_GM + 0x28, T_REPLAY);
                c.bus.write32(T_GM + 0x200, T_GM_STATE);
                c.bus.write32(T_GM + 0x214, pick(&mut r, &[0xe, 0xe, 1]));
                // Music off (no StopSong), no loading-screen loop.
                c.bus.write32(T_AUDIO + 0x608c, 2);
                c.bus.write32(T_AUDIO + 0x62a8, NEG1);
                // Track ids that never equal the selected peak (no
                // RestartPeakMusic).
                for k in 0..4 {
                    c.bus.write32(0x0043_d950 + k * 100, 0x7fff_ffff);
                }
                // Loading-screen history.
                c.bus.write32(g(-0x69c), 1 + r.next_u32() % 3);
                for i in 0..4 {
                    c.bus.write32(0x0053_66e8 + 4 * i, i);
                    c.bus.write32(0x0044_28f0 + 4 * i, 0x100 + i);
                }
                // Character profile flags 15/16 (peaks 2 and 3).
                c.bus
                    .write32(0x004a_7774, pick(&mut r, &[0, 1 << 15, 1 << 16, 3 << 15]));
                // EventSelect_OnWidgetAction's jump table (`D_00470780`).
                let mut table = [
                    0x0020_23d4,
                    0x0020_261c,
                    0x0020_270c,
                    0x0020_270c,
                    0x0020_193c,
                    0x0020_21d8,
                    0x0020_22bc,
                ];

                // The case.
                let k = seed % 20;
                let mut action: u64 = 5;
                let mut mode: u32 = 0;
                let mut confirm = 0u32;
                let mut flow = 0u32;
                let mut row = r.next_u32() % 3;
                let mut screen = false;
                let mut widget = T_WIDGET;
                let mut event_flag = r.next_u32() % 2;
                let mut peak_flag = r.next_u32() % 4 != 0;
                let mut special = 0u32;
                let mut missions = false;
                let mut reset_rules = false;
                let modes = [0, 1, 2, NEG1, 3];
                match k {
                    0 => {
                        action = 1;
                        confirm = 1;
                        row = pick(&mut r, &[0, 0, 1, 2]);
                    }
                    1 => {
                        action = 1;
                        mode = pick(&mut r, &modes);
                        screen = r.next_u32() % 2 == 0;
                    }
                    2 => {
                        action = 1;
                        mode = pick(&mut r, &[1, 2]);
                        screen = true;
                    }
                    3 => {
                        action = 2;
                        confirm = (r.next_u32() % 3 == 0) as u32;
                        mode = pick(&mut r, &modes);
                    }
                    4 => {
                        action = 2;
                        mode = pick(&mut r, &[0, 1, 2, NEG1, 7]);
                    }
                    5 => {
                        mode = 1;
                        row = r.next_u32() % 4;
                    }
                    6 => {
                        confirm = 1;
                        row = pick(&mut r, &[0, 0, 1]);
                        flow = pick(&mut r, &[0, 1, 2, 2, 3]);
                    }
                    7 => row = r.next_u32() % 5,
                    8 | 9 => {
                        row = pick(&mut r, &[1, 2]);
                        peak_flag = true;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                    }
                    10 => {
                        mode = 2;
                        flow = pick(&mut r, &[0, 1, 1]);
                        event_flag = 1;
                        reset_rules = r.next_u32() % 2 == 0;
                    }
                    11 => {
                        mode = 2;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                    }
                    12 | 13 => {
                        mode = 2;
                        confirm = 1;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                        if flow < 2 {
                            event_flag = 0;
                        }
                        row = pick(&mut r, &[0, 0, 1]);
                        special = pick(&mut r, &[5, 6, 0, 3]);
                        missions = r.next_u32() % 2 == 0;
                    }
                    14 => mode = pick(&mut r, &[NEG1, 3, 7]),
                    15 => {
                        action = 6;
                        confirm = (r.next_u32() % 3 == 0) as u32;
                        mode = pick(&mut r, &[0, 1, 2, NEG1]);
                        flow = pick(&mut r, &[0, 1, 3, 4]);
                    }
                    16 => {
                        action = 6;
                        flow = 3;
                    }
                    17 => {
                        action = 7;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                        confirm = (r.next_u32() % 4 == 0) as u32;
                        mode = pick(&mut r, &[0, 1, 2, NEG1]);
                        screen = mode < 2 && r.next_u32() % 2 == 0;
                    }
                    18 => {
                        action = 7;
                        flow = pick(&mut r, &[2, 3, 4]);
                        mode = pick(&mut r, &[0, 1]);
                        screen = true;
                    }
                    _ => match seed / 20 % 5 {
                        0 => widget = 0,
                        1 => action = 0,
                        2 => action = 8,
                        3 => action = pick(&mut r, &[3, 4]) as u64,
                        _ => {
                            action = 4;
                            table[3] = FAKE + 0x100;
                        }
                    },
                }
                put_words(c, 0x0047_0780, &table);
                let peak = r.next_u32() % 3;
                let tab = if action == 5 {
                    r.next_u32() % 2
                } else {
                    r.next_u32() % 3
                };
                c.bus.write32(g(-0xb54), mode);
                c.bus.write32(g(-0xb5c), flow);
                c.bus.write32(g(-0xb4c), peak);
                c.bus.write32(g(-0xb50), tab);
                if missions {
                    c.bus.write8(0x0053_5c10, 4);
                }
                if reset_rules {
                    // SetEventType(3) is allowed: the game mode byte
                    // changes, so the rule flags are cleared.
                    c.bus.write8(0x0043_e700 + 3, 1);
                }
                // The event record GetEventRecord(peak, row, tab) copies.
                let rec = if tab == 0 {
                    0x0047_81d0 + peak * 0x1b0 + row * 0x6c
                } else {
                    0x0047_86e0 + peak * 0x21c + row * 0x6c
                };
                c.bus.write32(rec, r.next_u32() % 4);
                c.bus
                    .write32(rec + 4, if reset_rules { 3 } else { r.next_u32() % 3 });
                c.bus.write32(rec + 8, r.next_u32() % 16);

                // The event-select panel (4-aligned, so the record copies at
                // +0x260 take the unaligned loops too).
                let this = T_ES + 4 * (r.next_u32() % 2);
                c.bus.write32(this, r.next_u32() % 2);
                for i in 0..3 {
                    let f = if i == row {
                        peak_flag as u32
                    } else {
                        r.next_u32() % 2
                    };
                    c.bus.write32(this + 4 + 4 * i, f);
                    c.bus.write32(this + 0x10 + 4 * i, r.next_u32() % 2);
                }
                c.bus
                    .write32(this + 0x20 + peak * 0x60 + tab * 0x20 + 4 * row, event_flag);
                c.bus.write32(this + 0x260, r.next_u32() % 3);
                c.bus.write32(this + 0x264, special);
                c.bus.write32(this + 0x268, pick(&mut r, &[0, 1, 0xc, 3]));
                c.bus.write32(this + 0x2cc, confirm);
                c.bus.write32(this + 0x2d0, r.next_u32() % 2);
                c.bus
                    .write32(this + 0x2e8, if screen { T_SCREEN } else { 0 });
                c.bus.write32(this + 0x2ec, T_STATE);
                // Its UI state: the sound player (or the default one).
                c.bus.write32(T_STATE + 0x10, T_SOUND);
                c.bus.write32(
                    T_SOUND + 0x14,
                    if r.next_u32() % 4 == 0 { 0 } else { T_PLAYER },
                );
                c.bus.write32(T_PLAYER, T_PVT);
                c.bus.write16(T_PVT + 0x10, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(T_PVT + 0x14, FAKE);
                // The screen: one object (every name hashes to 0 in test
                // RAM), its state byte, position and an all-fake vtable.
                c.bus.write32(T_SCREEN + 0x38, T_COUNT);
                c.bus.write32(T_SCREEN + 0x3c, T_LIST);
                c.bus.write32(T_COUNT, 1);
                c.bus.write32(T_LIST, T_OBJ);
                c.bus.write32(T_OBJ + 8, T_OVT);
                c.bus
                    .write8(T_OBJ + 0x95, pick(&mut r, &[3, 3, 0, 1]) as u8);
                for o in 0..3 {
                    c.bus.write32(T_OBJ + 0x44 + 4 * o, r.f32(500.0));
                    c.bus.write32(T_WIDGET + 0x44 + 4 * o, r.f32(500.0));
                }
                for slot in (0..0x100).step_by(8) {
                    c.bus.write32(T_OVT + slot + 4, FAKE + 0x20);
                }
                // The widget: its row and colour-setter vtable.
                c.bus.write32(T_WIDGET + 8, T_WVT);
                c.bus.write32(T_WIDGET + 0x18, row);
                c.bus.write16(T_WVT + 0x58, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(T_WVT + 0x5c, FAKE + 0x10);

                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
                c.g[A2][0] = action;
                // `alloc_ready` leaves `pc` at the sentinel return address;
                // the generated body dispatches to its resume entries
                // unless entered at its start.
                c.pc = 0x0020_18a8;
            },
        );
    }

    use crate::difftest::fakes::LOG_RESULTS;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::screen_with;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;

    #[test]
    fn event_select_set_map_tab_mode_matches() {
        check(
            ee::s05::f_00203cb8__generated,
            event_select_set_map_tab_mode,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x203cb8, 4);
                let screen = DATA + 0x1000;
                let node = screen_with(c, &mut r, screen, 0);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus
                    .write32(DATA + 0x2e8, if seed % 4 == 0 { 0 } else { screen });
                let _ = node;
            },
        );
    }

    #[test]
    fn event_select_load_map_image_matches() {
        strict(
            ee::s05::f_00203990__generated,
            event_select_load_map_image,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2039);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(gp_at(-0xb54), [-1i32, 0, 2, 3][(seed % 4) as usize] as u32);
                c.bus.write32(gp_at(-0xb4c), (seed % 5) as u32);
                c.bus.write32(DATA + 0x2d4, (seed % 7) as u32);
                c.bus.write32(DATA + 0x2d8, seed as u32);
                c.bus.write32(DATA + 0x2dc, 0);
                c.bus.write32(W + 0x11c, DATA + 0x3000);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::fe::festateloadscreen::tests::oc444_run;

    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_ASYNC;

    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_L1;

    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_OBJ;
    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_OBJ2;

    use std::ptr::addr_of_mut;

    /// The owner screen object, its UI state (empty object list: the real
    /// show/hide callees return at once), the widget screen with one list-menu
    /// object, and that object's vtable.
    pub(crate) const OC440_SELF: u32 = DATA + 0x400;

    pub(crate) const OC440_UI_STATE: u32 = DATA + 0x2000;

    pub(crate) const OC440_SCREEN: u32 = DATA + 0x3000;

    pub(crate) const OC440_COUNT: u32 = DATA + 0x3100;

    pub(crate) const OC440_OBJECTS: u32 = DATA + 0x3200;

    pub(crate) const OC440_OBJ: u32 = DATA + 0x4000;

    pub(crate) const OC440_VTABLE: u32 = DATA + 0x4200;

    /// `GetHashValue32(addr)` as the game computes it (every string of the
    /// test RAM is empty, so all names hash alike and the one object matches
    /// every lookup).
    pub(crate) fn oc440_hash(c: &mut Cpu, s: u32) -> u32 {
        const BACK: u32 = 0x0012_3450;
        c.g[RA][0] = BACK as u64;
        c.g[A0][0] = sx(s);
        c.pc = ES_STR_HASH;
        ssx3_ee_rt::funcs::call_indirect(c, ES_STR_HASH);
        assert_eq!(c.pc, BACK, "GetHashValue32 did not return");
        c.g[V0][0] as u32
    }

    pub(crate) fn oc440_fixture(c: &mut Cpu, seed: u32, state: u32) {
        let mut r: Rng = machine(c, seed, 0x20_2ad8, 0);
        let gp = GAME_GP;
        c.bus.write32(gp - ES_GP_MAP_STATE, state);
        c.bus
            .write32(gp - ES_GP_SELECTED_EVENT, [0, 1, 2, 5][seed as usize % 4]);
        c.bus.write32(gp - ES_GP_PEAK, r.next_u32() % 4);
        c.bus.write32(gp - ES_GP_MAP_STYLE, r.next_u32() % 4);
        c.bus
            .write32(OC440_SELF + ES_HAS_FOCUS, (seed % 3 != 0) as u32);
        c.bus
            .write32(OC440_SELF + ES_LOCKS_FLAG, (seed % 5 < 2) as u32);
        c.bus.write32(OC440_SELF + ES_SCREEN, OC440_SCREEN);
        c.bus.write32(OC440_SELF + ES_UI_STATE, OC440_UI_STATE);
        // A list-menu object found by every name (its text-id slot at +0xb0
        // and a vtable whose virtual calls land in the fake).
        let h = oc440_hash(c, DATA + 0x100);
        c.bus.write32(OC440_OBJ + 0x38, h);
        c.bus
            .write8(OC440_OBJ + ES_LIST_STATE, (r.next_u32() % 5) as u8);
        c.bus.write32(OC440_OBJ + 8, OC440_VTABLE);
        for off in [0x28u32, 0x48] {
            c.bus.write16(OC440_VTABLE + off, 0);
            c.bus.write32(OC440_VTABLE + off + 4, FAKE);
        }
        c.bus.write32(OC440_COUNT, 1);
        c.bus.write32(OC440_OBJECTS, OC440_OBJ);
        c.bus.write32(OC440_SCREEN + 0x38, OC440_COUNT);
        // Every seventh seed has no objects: all lookups return null.
        c.bus.write32(
            OC440_SCREEN + 0x3c,
            if seed % 7 == 0 { 0 } else { OC440_OBJECTS },
        );
        c.g[A0][0] = sx(OC440_SELF);
        c.pc = 0x0020_2ad8;
    }

    /// Map states 0 (peaks) and 1 (peak goals), plus the states that only
    /// restore and return (negative, 3): focus, lock flag, list state 0..4,
    /// map style, peak number all vary.
    #[test]
    fn event_select_update_mode_panels_matches() {
        check(
            ee::s05::f_00202ad8__generated,
            event_select_update_mode_panels,
            210,
            &|c, seed| {
                let state = [0u32, 1, 0, 1, 0xffff_ffff, 3, 1][seed as usize % 7];
                oc440_fixture(c, seed, state);
            },
        );
    }

    /// Map state 2 (events): runs the real `EventSelect_RefreshEventList` on
    /// the fixture, so it needs the back-end library singleton.
    #[test]
    fn event_select_update_mode_panels_events_matches() {
        check(
            ee::s05::f_00202ad8__generated,
            event_select_update_mode_panels,
            24,
            &|c, seed| {
                oc440_fixture(c, seed, 2);
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x20_0000);
            },
        );
    }

    /// `EventSelect_SetMode` test fixture addresses (above the fake-call log
    /// and below the heap).
    pub(crate) const T_SM_SELF: u32 = DATA + 0x100;

    pub(crate) const T_SM_STATE: u32 = DATA + 0x1000;

    pub(crate) const T_SM_SCREEN: u32 = DATA + 0x2000;

    pub(crate) const T_SM_COUNT: u32 = DATA + 0x2100;

    pub(crate) const T_SM_LIST: u32 = DATA + 0x2200;

    pub(crate) const T_SM_VT: u32 = DATA + 0x2800;

    pub(crate) const T_SM_OBJS: u32 = DATA + 0x4000;

    /// Name offsets (from `0x470000`) of the 14 panel widgets in hide order,
    /// then the confirm "No" button, the title and the tab label.
    pub(crate) const T_SM_NAMES: [u32; 17] = [
        0x600, 0x918, 0x928, 0x938, 0x948, 0x960, 0x978, 0x990, 0x9a8, 0x9c0, 0x9d8, 0x9f0, 0xa08,
        0xa20, 0x6c8, 0x518, 0xab8,
    ];

    /// `StrHash(string at addr)` through the real function.
    pub(crate) fn t_sm_hash(c: &mut Cpu, addr: u32) -> u32 {
        const RET: u32 = 0x0012_3450;
        c.g[A0][0] = sx(addr);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7670);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0] as u32
    }

    /// A screen holding the widgets named in `T_SM_NAMES` (the first 14 of
    /// them only where `absent` has no bit set), every widget with the same
    /// all-fake vtable, and the event-select object `T_SM_SELF` pointing at
    /// it and at an empty UI state.
    pub(crate) fn t_sm_screen(c: &mut Cpu, r: &mut Rng, absent: u32, with_screen: bool) {
        for (i, off) in [0x38u32, 0x48, 0x40, 0x28].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(T_SM_VT + off, delta as u16);
            c.bus.write32(T_SM_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        let mut count = 0;
        for (k, off) in T_SM_NAMES.into_iter().enumerate() {
            let name = 0x0047_0000 + off;
            c.bus.write8(name, b'A' + k as u8);
            c.bus.write8(name + 1, b'x');
            c.bus.write8(name + 2, 0);
            if k < 14 && (absent >> k) & 1 != 0 {
                continue;
            }
            let hash = t_sm_hash(c, name);
            let obj = T_SM_OBJS + 0x100 * k as u32;
            c.bus.write32(obj + 8, T_SM_VT);
            c.bus.write32(obj + 0x14, r.next_u32());
            c.bus.write32(obj + 0x38, hash);
            c.bus.write32(T_SM_LIST + 4 * count, obj);
            count += 1;
        }
        c.bus.write32(T_SM_COUNT, count);
        c.bus.write32(T_SM_SCREEN + 0x38, T_SM_COUNT);
        c.bus.write32(T_SM_SCREEN + 0x3c, T_SM_LIST);
        c.bus.write32(
            T_SM_SELF + ES_SETMODE_SCREEN,
            if with_screen { T_SM_SCREEN } else { 0 },
        );
        c.bus.write32(T_SM_SELF + ES_SETMODE_STATE, T_SM_STATE);
    }

    /// The `gp`-relative globals the function reads.
    pub(crate) fn t_sm_globals(c: &mut Cpu, flow: u32, peak: u32, event: u32) {
        c.bus.write32(GAME_GP - ES_SETMODE_G_FLOW, flow);
        c.bus.write32(GAME_GP - ES_SETMODE_G_PEAK, peak);
        c.bus.write32(GAME_GP - ES_SETMODE_G_EVENT, event);
    }

    /// `EventSelect_SetMode` against the generated body: the confirm popup
    /// (with every focus: no panel code runs), the event panel (focus 2) for
    /// every peak, tab and flow, and focus values with no panel (negative,
    /// 3, 9); widgets present or missing, the screen null in some setups.
    /// The real callees take their short paths: the UI state has an empty
    /// widget list, every widget's vtable ends in fakes. (Panels 0 and 1 call
    /// the not yet ported `RefreshPeakList`/`RefreshPeakGoals`: see
    /// `event_select_set_mode_peak_panels_match`.)
    #[test]
    fn event_select_set_mode_matches() {
        check(
            ee::s05::f_002030a0__generated,
            event_select_set_mode,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_30a0, 24);
                let confirm = seed % 4 == 0;
                let focus: i32 = if confirm {
                    [0, 1, 2, 3, -1][(r.next_u32() % 5) as usize]
                } else {
                    [2, 2, 2, 2, 3, -1, 9][(r.next_u32() % 7) as usize]
                };
                let absent = if seed % 3 == 0 {
                    r.next_u32() & 0x3fff
                } else {
                    0
                };
                let with_screen = seed % 9 != 5;
                t_sm_screen(c, &mut r, absent, with_screen);
                t_sm_globals(c, r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.bus.write32(T_SM_SELF + ES_SETMODE_POPUP, confirm as u32);
                c.bus
                    .write32(T_SM_SELF + ES_SETMODE_NO_SELECTED, r.next_u32() % 2);
                c.bus.write32(T_SM_SELF, r.next_u32());
                c.g[A0][0] = sx(T_SM_SELF);
                c.g[A1][0] = sx(focus as u32);
            },
        );
    }

    /// `EventSelect_SetMode` on panels 0 and 1, which call the real
    /// `RefreshPeakList`/`RefreshPeakGoals`: one screen widget that every name
    /// finds (names hash to 0 in zeroed RAM), the BELibrary dummy and a world,
    /// every peak and flow, the screen null in some setups.
    #[test]
    fn event_select_set_mode_peak_panels_match() {
        const T_BELIB: u32 = 0x008b_8000;
        const T_WORLD: u32 = 0x008a_6000;
        const T_OBJ: u32 = 0x008a_4000;
        const T_OVT: u32 = 0x008a_5000;
        check(
            ee::s05::f_002030a0__generated,
            event_select_set_mode,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_30a1, 40);
                let g = |off: i32| GAME_GP.wrapping_add(off as u32);
                c.bus.write32(g(-0x1f04), T_BELIB);
                c.bus.write32(g(-0x848), T_WORLD);
                c.bus.write32(T_WORLD + 0x78, T_BELIB);
                c.bus.write32(
                    0x004a_7774,
                    [0, 1 << 15, 1 << 16, 3 << 15][(seed % 4) as usize],
                );
                c.bus.write32(T_SM_SCREEN + 0x38, T_SM_COUNT);
                c.bus.write32(T_SM_SCREEN + 0x3c, T_SM_LIST);
                c.bus.write32(T_SM_COUNT, 1);
                c.bus.write32(T_SM_LIST, T_OBJ);
                c.bus.write32(T_OBJ + 8, T_OVT);
                c.bus.write32(T_OBJ + 0x14, r.next_u32());
                for slot in (0..0x100).step_by(8) {
                    c.bus.write32(T_OVT + slot + 4, FAKE + 0x20);
                }
                c.bus.write32(
                    T_SM_SELF + ES_SETMODE_SCREEN,
                    if seed % 5 == 3 { 0 } else { T_SM_SCREEN },
                );
                c.bus.write32(T_SM_SELF + ES_SETMODE_STATE, T_SM_STATE);
                c.bus.write32(T_SM_SELF + ES_SETMODE_POPUP, 0);
                t_sm_globals(c, r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4);
                c.g[A0][0] = sx(T_SM_SELF);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    /// Gate of the fakes (magic word in the machine's RAM) and the knobs the
    /// setup writes for them.
    pub(crate) const OC442_CTRL: u32 = DATA + 0x8000;

    pub(crate) const OC442_MAGIC: u32 = 0x4f43_3434;

    pub(crate) const OC442_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC442_SCREEN: u32 = DATA + 0x2000;

    pub(crate) const OC442_OWNER: u32 = DATA + 0x3000;

    pub(crate) const OC442_NODE1: u32 = DATA + 0x3100;

    pub(crate) const OC442_NODE2: u32 = DATA + 0x3200;

    /// The one object every `UIScreen_FindObject(screen, ..)` returns (list,
    /// text, mountain: its vtable has a fake method in every slot).
    pub(crate) const OC442_OBJ: u32 = DATA + 0x4000;

    pub(crate) const OC442_VT: u32 = DATA + 0x5000;

    pub(crate) const OC442_SEL: u32 = DATA + 0x6000;

    pub(crate) const OC442_WORLD: u32 = DATA + 0x7000;

    pub(crate) const OC442_LOC: u32 = DATA + 0x7100;

    pub(crate) const OC442_LOCVT: u32 = DATA + 0x7200;

    /// A fake callee: logs the call (all registers, cycles) through
    /// `fakes::fake`, then sets the result the callee would return from a
    /// seeded hash and the knobs in the control block.
    pub(crate) fn oc442_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let seed = c.bus.read32(OC442_CTRL + 4) as u64;
        let trophy_pct = c.bus.read32(OC442_CTRL + 8) as u64;
        let medal_pct = c.bus.read32(OC442_CTRL + 0xc) as u64;
        let h = fakes::mix((seed << 32) ^ ((addr as u64) << 8) ^ n as u64 ^ 0x9e37_79b9);
        let v = match addr {
            // The rider character.
            0x0014_a080 => Some(h % 6),
            // isTrophyOwned.
            0x0015_7bf0 => Some((h % 100 < trophy_pct) as u64),
            // getTrackMedal: -1 (none) or a medal 0..3.
            0x0015_8910 => Some(if h % 100 < medal_pct {
                (h >> 8) % 4
            } else {
                u64::MAX
            }),
            // Mission, collectible, medal-target and money counters.
            0x0015_43e0 | 0x0015_4368 | 0x0015_35c0 | 0x0015_3390 | 0x0015_8890 | 0x0015_8870
            | 0x0015_09f8 => Some((h >> 8) % 5),
            // FindObject: the screen's objects (one lookup may fail), nothing
            // in the state owner's list nodes.
            0x0039_d860 => {
                if a0 == OC442_SCREEN {
                    let k = c.bus.read32(OC442_CTRL + 0x10);
                    c.bus.write32(OC442_CTRL + 0x10, k + 1);
                    let zero_at = c.bus.read32(OC442_CTRL + 0x14);
                    Some(if k == zero_at { 0 } else { OC442_OBJ as u64 })
                } else {
                    Some(0)
                }
            }
            _ => None,
        };
        if let Some(v) = v {
            c.g[V0][0] = v;
        }
    }

    macro_rules! oc442_fakes {
        ($(($fake:ident, $addr:expr, $gen:path, $slot:path)),* $(,)?) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC442_CTRL) != OC442_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    oc442_fake_call(c, $addr)
                }
            )*

            /// Hooks the fakes into the callees' override slots (once per
            /// process; with the gate clear they run what the slots held).
            pub(crate) fn oc442_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc442_fakes! {
        (oc442_fake_be, 0x0014_dc80, ee::s01::f_0014dc80__generated, ee::s01::OVERRIDE_0014DC80),
        (oc442_fake_interface, 0x0014_dd58, ee::s01::f_0014dd58__generated, ee::s01::OVERRIDE_0014DD58),
        (oc442_fake_rider_char, 0x0014_a080, ee::s01::f_0014a080__generated, ee::s01::OVERRIDE_0014A080),
        (oc442_fake_show, 0x0039_e948, ee::s13::f_0039e948__generated, ee::s13::OVERRIDE_0039E948),
        (oc442_fake_trophy, 0x0015_7bf0, ee::s01::f_00157bf0__generated, ee::s01::OVERRIDE_00157BF0),
        (oc442_fake_find, 0x0039_d860, ee::s13::f_0039d860__generated, ee::s13::OVERRIDE_0039D860),
        (oc442_fake_set_wide, 0x003a_0e90, ee::s13::f_003a0e90__generated, ee::s13::OVERRIDE_003A0E90),
        (oc442_fake_set_ascii, 0x003a_0c70, ee::s13::f_003a0c70__generated, ee::s13::OVERRIDE_003A0C70),
        (oc442_fake_medal, 0x0015_8910, ee::s01::f_00158910__generated, ee::s01::OVERRIDE_00158910),
        (oc442_fake_done_missions, 0x0015_43e0, ee::s01::f_001543e0__generated, ee::s01::OVERRIDE_001543E0),
        (oc442_fake_challenge_target, 0x0015_8890, ee::s01::f_00158890__generated, ee::s01::OVERRIDE_00158890),
        (oc442_fake_collect, 0x0015_35c0, ee::s01::f_001535c0__generated, ee::s01::OVERRIDE_001535C0),
        (oc442_fake_collect_target, 0x0015_8870, ee::s01::f_00158870__generated, ee::s01::OVERRIDE_00158870),
        (oc442_fake_money, 0x0015_09f8, ee::s01::f_001509f8__generated, ee::s01::OVERRIDE_001509F8),
        (oc442_fake_num_missions, 0x0015_4368, ee::s01::f_00154368__generated, ee::s01::OVERRIDE_00154368),
        (oc442_fake_total_collect, 0x0015_3390, ee::s01::f_00153390__generated, ee::s01::OVERRIDE_00153390),
    }

    /// One seed of `EventSelect_RefreshPeakList`: map style 0..3, map state,
    /// popup flag, unlocked peaks, the selected peak 0..4 (the overall entry
    /// from 3), missing list/selection, and the counters the fakes return
    /// (trophies, medals, missions) from a seeded hash. Random goal records
    /// (kinds and tracks, both 4-aligned and 8-aligned copies), earnings
    /// targets and highlight levels in RAM; the strings and format strings
    /// come from the executable image. Real callees: hash, sprintf, hide,
    /// SetTextId, SetString (the text vtable slots are fake methods), the
    /// earnings and highlight accessors.
    pub(crate) fn oc442_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0020_3d40, 200);
        elf_image(c);
        c.pc = 0x0020_3d40;
        c.g[A0][0] = sx(OC442_SELF);

        let style = seed % 4;
        let state = (seed / 4) % 2;
        let popup = (seed / 8) % 2;
        let peak = r.next_u32() % 5;
        c.bus.write32(GAME_GP - 0xb5c, style);
        c.bus.write32(GAME_GP - 0xb54, state);
        c.bus.write32(GAME_GP - 0x848, OC442_WORLD);

        // The map template.
        c.bus.write32(OC442_SELF + PK_SCREEN, OC442_SCREEN);
        c.bus.write32(OC442_SELF + PK_OWNER, OC442_OWNER);
        c.bus.write32(OC442_SELF + PK_POPUP, popup);
        for i in 0..3 {
            c.bus
                .write32(OC442_SELF + PK_UNLOCKED + 4 * i, r.next_u32() % 2);
        }
        // The state owner's object list: two nodes, so `UIState_HideObject`
        // looks the object up twice.
        c.bus.write32(OC442_OWNER + 0x28, OC442_NODE1);
        c.bus.write32(OC442_NODE1 + 4, OC442_NODE2);
        // The text/list object and its vtable: a fake method in every slot.
        c.bus.write32(OC442_OBJ + 8, OC442_VT);
        let no_sel = r.next_u32() % 8 == 0;
        c.bus
            .write32(OC442_OBJ + PK_LIST_SEL, if no_sel { 0 } else { OC442_SEL });
        c.bus.write32(OC442_SEL + PK_SEL_PEAK, peak);
        for off in (0..0x200).step_by(8) {
            c.bus.write16(OC442_VT + off, 0);
            c.bus
                .write32(OC442_VT + off + 4, fakes::FAKE + 0x10 * (off / 8));
        }
        // The localisation lookup through the world (`world+0x8c`).
        c.bus.write32(OC442_WORLD + 0x8c, OC442_LOC);
        c.bus.write32(OC442_LOC + 4, OC442_LOCVT);
        c.bus.write16(OC442_LOCVT + 0x20, 0);
        c.bus.write32(OC442_LOCVT + 0x24, fakes::FAKE + 0x1000);

        // Goal records: table A (3 peaks x 4 goals, 0x6c each; 0x1b0 a peak)
        // and table B (3 x 5; 0x21c a peak): random track and kind.
        for (base, stride, n) in [(0x0047_81d0u32, 0x1b0u32, 4u32), (0x0047_86e0, 0x21c, 5)] {
            for p in 0..3 {
                for k in 0..n {
                    let at = base + p * stride + k * 0x6c;
                    for w in 0..27 {
                        c.bus.write32(at + 4 * w, r.next_u32());
                    }
                    c.bus.write32(at, r.next_u32() % 20);
                    c.bus.write32(at + 8, r.next_u32() % 14);
                }
            }
        }
        // Earnings targets and highlight levels.
        for p in 0..6 {
            c.bus.write32(0x0045_b048 + 4 * p, r.next_u32() % 5);
        }
        let fill = r.next_u32() % 3;
        for o in 0..0x5d60u32 {
            let b = match fill {
                0 => 0,
                1 => 3,
                _ => r.next_u32() % 4,
            };
            c.bus.write8(0x004a_7860 + o, b as u8);
        }

        // The fakes' gate and answers.
        let trophy_pct = [0, 60, 100, 95][(r.next_u32() % 4) as usize];
        let medal_pct = [0, 30, 80, 100][(r.next_u32() % 4) as usize];
        c.bus.write32(OC442_CTRL, OC442_MAGIC);
        c.bus.write32(OC442_CTRL + 4, seed);
        c.bus.write32(OC442_CTRL + 8, trophy_pct);
        c.bus.write32(OC442_CTRL + 0xc, medal_pct);
        c.bus.write32(OC442_CTRL + 0x10, 0);
        // The k-th screen lookup that fails: the list lookup in 1 of 8 seeds
        // (third lookup with map style < 2, else the second).
        let list_lookup = if style < 2 { 2 } else { 1 };
        c.bus.write32(
            OC442_CTRL + 0x14,
            if r.next_u32() % 8 == 0 {
                list_lookup
            } else {
                u32::MAX
            },
        );
    }

    /// `EventSelect_RefreshPeakList` (0x203d40) against the generated body:
    /// map styles 0..3 and both map states, popup on/off, locked and unlocked
    /// peaks, trophies and medals (every combination of found and missing),
    /// the selected peak 0..2 (help text; goal counters on style >= 2) and the
    /// overall entry (peaks 3 and 4: done/gold/missions/collectibles/highlight
    /// counters and the percentage), a missing list or selection, and thread
    /// unwinds in the callees. The back-end accessors and the screen lookups
    /// are fakes; the formatting, hashing and state hide/show run real.
    #[test]
    fn event_select_refresh_peak_list_matches() {
        oc442_install_fakes();
        covered(
            ee::s05::f_00203d40__generated,
            event_select_refresh_peak_list,
            300,
            &oc442_setup,
        );
    }

    // ----------------------------------------- FEAsyncManager_LoadSlot.

    // --------------------------------------- EventSelect (event map UI).

    /// The hash the game computes for the `"SP_PeakList"` label.
    pub(crate) fn oc444_str_hash(addr: u32) -> u32 {
        use std::collections::HashMap;
        use std::sync::Mutex;
        static CACHE: Mutex<Option<HashMap<u32, u32>>> = Mutex::new(None);
        let mut cache = CACHE.lock().unwrap_or_else(|p| p.into_inner());
        *cache
            .get_or_insert_with(HashMap::new)
            .entry(addr)
            .or_insert_with(|| {
                let mut c = crate::difftest::fresh();
                elf_image(&mut c);
                c.g[A0][0] = addr as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0031_7670;
                let f = ssx3_ee_rt::funcs::lookup(0x0031_7670).expect("StrHash");
                f(&mut c);
                c.g[V0][0] as u32
            })
    }

    /// `EventSelect_OnWidgetCreated(self, menu)`: only a menu labelled
    /// `SP_PeakList` selects an entry, by map style.
    #[test]
    fn event_select_on_widget_created_matches() {
        oc444_run(
            ee::s05::f_002009d0__generated,
            event_select_on_widget_created,
            0x0020_09d0,
            0x20_09d0,
            80,
            &|c, r, seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                c.g[A1][0] = (OC444_OBJ2 + 0x100) as u64;
                let label = if seed % 4 == 0 {
                    r.next_u32() % 6
                } else {
                    oc444_str_hash(0x0047_0600)
                };
                c.bus.write32(OC444_OBJ2 + 0x100 + 0x38, label);
            },
        );
    }

    /// `EventSelect_Update(self)`: fade in or hide the map, the peak
    /// outlines (map state 0) or indicators, the "you are here" labels and the
    /// async-loaded texture.
    #[test]
    fn event_select_update_matches() {
        oc444_run(
            ee::s05::f_00202828__generated,
            event_select_update,
            0x0020_2828,
            0x20_2828,
            120,
            &|c, r, seed| {
                let slot = r.next_u32() % 3;
                c.g[A0][0] = OC444_OBJ as u64;
                c.bus.write32(OC444_OBJ, (seed % 5 == 0) as u32);
                c.bus.write32(OC444_OBJ + 0x2d8, slot);
                c.bus.write32(OC444_OBJ + 0x2dc, (seed % 2) as u32);
                let fade = [0u32, 0x3dcc_cccd, 0x3f66_6666, 0x3f80_0000][r.next_u32() as usize % 4];
                c.bus.write32(OC444_OBJ + 0x2e0, fade);
                c.bus.write32(OC444_OBJ + 0x2e4, OC444_L1 + 0x1000);
                c.bus.write32(OC444_OBJ + 0x2ec, OC444_L1 + 0x1400);
                c.bus.write32(OC444_OBJ + 0x2f0, (seed % 3 == 0) as u32);
                let state = if seed % 4 < 2 { 3 } else { r.next_u32() % 3 };
                c.bus.write32(OC444_ASYNC + 0x114 + 0x11c * slot, state);
            },
        );
    }

    use crate::difftest::fill;

    pub(crate) use crate::game::sound::asyncsys::tests::oc738_install_fakes;
    pub(crate) use crate::game::sound::asyncsys::tests::oc738_machine;
    pub(crate) use crate::game::sound::asyncsys::tests::OC738_CTRL;

    /// Control words `OC738_CTRL + 4, 8, 0xc`: non-zero makes the medal fake
    /// return -1, `isLoaded` and `IsActiveNISInTransition` return 0.
    pub(crate) const OC738_MEDAL_M1: u32 = OC738_CTRL + 4;

    /// An unaligned guest word store (byte stores).
    pub(crate) fn oc738_put32(c: &mut Cpu, a: u32, v: u32) {
        for i in 0..4 {
            c.bus.write8(a + i, (v >> (8 * i)) as u8);
        }
    }

    /// `EventSelect_IsEventCompleted`: the goal struct at an aligned or
    /// unaligned address; location 23, event types 4/5 (peaks 0..2) and
    /// other event types (medal present or -1).
    #[test]
    fn event_select_is_event_completed_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s05::f_00200b50__generated,
            event_select_is_event_completed,
            400,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x20_0b50, 6);
                let base = DATA + 0x1000;
                fill(c, &mut r, base, 0x20);
                let off = if seed % 3 == 0 { 0 } else { 1 + (seed / 3) % 7 };
                let location = if seed % 5 == 0 {
                    0x17
                } else {
                    r.next_u32() % 40
                };
                let event = [4u32, 5, 4, 5, 0, 1, 2, 3, 6][((seed / 5) % 9) as usize];
                oc738_put32(c, base + off, location);
                oc738_put32(c, base + off + 8, event);
                c.bus.write32(OC738_MEDAL_M1, ((seed / 45) % 2) as u32);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(base + off);
                c.g[A2][0] = (seed % 4) as u64;
                c.pc = 0x0020_0b50;
            },
        );
    }

    use crate::difftest::RA as DT_RA;

    pub(crate) use crate::game::fe::feridermanager::tests::oc786_arm;
    pub(crate) use crate::game::fe::feridermanager::tests::OC786_OBJ;
    pub(crate) use crate::game::fe::festatecharsetup::tests::oc774_arm;
    pub(crate) use crate::game::fe::festatecharsetup::tests::oc774_force_zero;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_OBJ;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_UI;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_WORLD;

    pub(crate) use crate::game::fe::festateruleselect::tests::oc776_install_fakes;

    pub(crate) use crate::game::fe::festateruleselect::tests::OC776_ROWS;
    pub(crate) use crate::game::fe::festateruleselect::tests::OC776_THIS;
    pub(crate) use crate::game::fe::festateruleselect::tests::OC776_VT;

    pub(crate) use crate::game::sound::bankmanager::tests::OC786_ZERO_MASK;

    pub(crate) use crate::game::ui::uimenu::tests::OC776_CTRL;
    pub(crate) use crate::game::ui::uimenu::tests::OC776_MAGIC;
    use std::sync::atomic::AtomicUsize;

    /// `EventSelect_TransitionIn`: `enter == 0` (nothing happens), and
    /// `enter != 0` with the fake unwinding at the mode call (0) or at the
    /// first call inside the real `EventSelect_SetupPeakTexts` (1). The
    /// full path (the real peak-text and event-list builders over fake
    /// pointers) is the next test.
    #[test]
    fn event_select_transition_in_matches() {
        covered(
            ee::s05::f_00200a70__generated,
            event_select_transition_in,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x200a70, 2);
                oc758_install_fakes();
                c.bus.write32(OC758_CTRL, OC758_MAGIC);
                c.pc = 0x0020_0a70;
                let this = DATA + 0x1000;
                c.bus.write32(this, r.next_u32());
                c.bus.write32(GAME_GP - 0xb54, r.next_u32());
                c.g[A0][0] = sx(this);
                if seed % 3 == 0 {
                    c.g[A1][0] = 0;
                } else {
                    c.g[A1][0] = sx(r.next_u32() | 1);
                    c.bus
                        .write32(fakes::LOG_UNWIND_AT, if seed % 3 == 1 { 0 } else { 1 });
                }
            },
        );
    }

    /// `EventSelect_TransitionIn`, `enter != 0`, no unwind: every callee runs
    /// (the real `SetupPeakTexts` / `RefreshEventList` over zeroed RAM and
    /// fake results). Dropped from the `OVERRIDES`-gate if this runs away.
    #[test]
    fn event_select_transition_in_full_matches() {
        check(
            ee::s05::f_00200a70__generated,
            event_select_transition_in,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x200a71, 0);
                oc758_install_fakes();
                c.bus.write32(OC758_CTRL, OC758_MAGIC);
                c.pc = 0x0020_0a70;
                let this = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0xb54, r.next_u32() % 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() | 1);
            },
        );
    }

    /// `EventSelect_Init`: map styles 0..5, the front-end start flag, the
    /// `MapPic` widget found or not, and the unwinds of its calls (the
    /// style-changed-by-refresh path needs a callee that changes `gp - 0xb5c`,
    /// which the fake does not).
    #[test]
    fn event_select_init_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s05::f_00200700__generated,
            event_select_init,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774e, 40);
                oc774_arm(c);
                c.pc = 0x0020_0700;
                c.g[A0][0] = sx(OC774_OBJ);
                c.g[A1][0] = sx(r.next_u32() | 1);
                c.g[A2][0] = sx(OC774_UI);
                c.bus.write32(GAME_GP - 0xb5c, (seed % 6) as u32);
                c.bus.write32(GAME_GP - 0xb58, ((seed / 6) % 2) as u32);
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x11c, r.next_u32());
                if (seed / 12) % 2 == 0 {
                    oc774_force_zero(c, 0x0039_d860);
                }
            },
        );
    }

    /// `EventSelect_UpdateConfirmPopup`: popup inactive/active, the menu
    /// found or not, the second activity check, map state 0 or not, the
    /// P2 text, Yes/No buttons present or missing, popup type 1 or not, and
    /// the unwinds of its calls.
    #[test]
    fn event_select_update_confirm_popup_matches() {
        oc776_install_fakes();
        oc776_install_plain_fakes();
        fakes::covered(
            ee::s05::f_00203a58__generated,
            event_select_update_confirm_popup,
            500,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7763, 30);
                c.pc = 0x0020_3a58;
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                // Objects by name: "1b" ConfirmMenu, "2b" P2_TransportNow,
                // "3b" ConfirmMenu_Yes, "4b" ConfirmMenu_No (object index = 2 * digit).
                for (name, digit) in [
                    (0x0047_0a20u32, b'1'),
                    (0x0047_0a70, b'2'),
                    (0x0047_06b8, b'3'),
                    (0x0047_06c8, b'4'),
                ] {
                    c.bus.write8(name, digit);
                    c.bus.write8(name + 1, b'b');
                }
                // Absent objects: one bit per object (2, 4, 6, 8).
                let absent = [0u32, 1 << 2, 1 << 4, 1 << 6, 1 << 8][((seed / 4) % 5) as usize]
                    | if seed % 7 == 0 { 1 << 6 | 1 << 8 } else { 0 };
                c.bus.write32(OC776_CTRL + 4, absent);
                c.g[A0][0] = sx(OC776_THIS);
                c.bus
                    .write32(OC776_THIS + POPUP_ACTIVE, if seed % 5 == 0 { 0 } else { 1 });
                // Finding the menu clears the popup flag (the popup closed meanwhile).
                c.bus.write32(OC776_CTRL + 8, (seed % 6 == 1) as u32);
                c.bus.write32(
                    OC776_THIS + POPUP_TYPE,
                    [1u32, 0, 2][((seed / 2) % 3) as usize],
                );
                c.bus.write32(OC776_THIS + POPUP_SCREEN, 0x4321);
                c.bus.write32(OC776_THIS + POPUP_STATE, 0x8765);
                c.bus.write32(
                    crate::difftest::GAME_GP - 0xb54,
                    if seed % 2 == 0 { 0 } else { 3 },
                );
                for (i, slot) in [0x48u32, 0x40].into_iter().enumerate() {
                    c.bus
                        .write16(OC776_VT + slot, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus
                        .write32(OC776_VT + slot + 4, fakes::FAKE + 0x10 * i as u32);
                }
                for n in 0..32u32 {
                    c.bus.write32(OC776_ROWS + n * 0x200 + 8, OC776_VT);
                }
            },
        );
    }

    /// The fakes' gate word and the fixture objects, clear of the fakes' call
    /// log (`DATA + 0x10_0000`) and pointer range (`DATA .. DATA + 0x4_0000`).
    pub(crate) const OC777_CTRL: u32 = DATA + 0x2c_0000;

    pub(crate) const OC777_MAGIC: u32 = 0x4f43_3737 + 0x77;

    /// Address whose gated fake unwinds (pc = 0xdead_xxxx) whenever it is
    /// called; 0 for none.
    pub(crate) const OC777_UNWIND_ADDR: u32 = OC777_CTRL + 4;

    /// Per fake address (index in `OC777_ADDRS`): result mask word and result
    /// value word. Call k (k < 32) of that address returns the value in `v0`
    /// when bit k of the mask is set.
    pub(crate) const OC777_PAT: u32 = DATA + 0x2c_1000;

    /// Per fake address: how many times it was called.
    pub(crate) const OC777_CNT: u32 = DATA + 0x2c_2000;

    pub(crate) const OC777_OBJ: u32 = DATA + 0x30_0000;

    /// The real leaf at 0x1406c0 returns 0.
    pub(crate) const OC777_ZERO_LEAF: u32 = 0x0014_06c0;

    /// Gated fakes for the callees that have an override slot. A fake logs
    /// the call (every register) like `fakes::fake_call`, then the fixture can
    /// script its result (`oc777_pat`) or make it unwind (`OC777_UNWIND_ADDR`).
    macro_rules! oc777_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            const OC777_ADDRS: &[u32] = &[$($addr),*];
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC777_CTRL) != OC777_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc777_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc777_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    pub(crate) fn oc777_index(addr: u32) -> u32 {
        OC777_ADDRS
            .iter()
            .position(|&a| a == addr)
            .expect("fake address") as u32
    }

    /// One gated fake call: the logged call of `fakes::fake_call`, then the
    /// scripted result and the scripted unwind.
    pub(crate) fn oc777_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let idx = oc777_index(addr);
        let k = c.bus.read32(OC777_CNT + 4 * idx);
        c.bus.write32(OC777_CNT + 4 * idx, k + 1);
        if k < 32 && (c.bus.read32(OC777_PAT + 8 * idx) >> k) & 1 != 0 {
            let v = c.bus.read32(OC777_PAT + 8 * idx + 4);
            c.g[V0][0] = sx(v);
        }
        if addr == c.bus.read32(OC777_UNWIND_ADDR) {
            let n = c.bus.read32(fakes::LOG_COUNT);
            c.pc = 0xdead_0000 | n << 4;
        }
    }

    /// Open the gate (both machines of a diff get their own).
    pub(crate) fn oc777_gate(c: &mut Cpu) {
        c.bus.write32(OC777_CTRL, OC777_MAGIC);
    }

    oc777_fakes!(
        (
            oc777_fake_00202ad8,
            OC777_PREV_00202AD8,
            ee::s05::OVERRIDE_00202AD8,
            ee::s05::f_00202ad8__generated,
            0x0020_2ad8
        ),
        (
            oc777_fake_00203d40,
            OC777_PREV_00203D40,
            ee::s05::OVERRIDE_00203D40,
            ee::s05::f_00203d40__generated,
            0x0020_3d40
        ),
        (
            oc777_fake_00317670,
            OC777_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc777_fake_00317e30,
            OC777_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc777_fake_00287920,
            OC777_PREV_00287920,
            ee::s07::OVERRIDE_00287920,
            ee::s07::f_00287920__generated,
            0x0028_7920
        ),
        (
            oc777_fake_00285be0,
            OC777_PREV_00285BE0,
            ee::s07::OVERRIDE_00285BE0,
            ee::s07::f_00285be0__generated,
            0x0028_5be0
        ),
        (
            oc777_fake_00287968,
            OC777_PREV_00287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc777_fake_00287f00,
            OC777_PREV_00287F00,
            ee::s07::OVERRIDE_00287F00,
            ee::s07::f_00287f00__generated,
            0x0028_7f00
        ),
        (
            oc777_fake_00289bb8,
            OC777_PREV_00289BB8,
            ee::s07::OVERRIDE_00289BB8,
            ee::s07::f_00289bb8__generated,
            0x0028_9bb8
        ),
        (
            oc777_fake_0028b830,
            OC777_PREV_0028B830,
            ee::s07::OVERRIDE_0028B830,
            ee::s07::f_0028b830__generated,
            0x0028_b830
        ),
        (
            oc777_fake_0028a728,
            OC777_PREV_0028A728,
            ee::s07::OVERRIDE_0028A728,
            ee::s07::f_0028a728__generated,
            0x0028_a728
        ),
        (
            oc777_fake_0028b320,
            OC777_PREV_0028B320,
            ee::s07::OVERRIDE_0028B320,
            ee::s07::f_0028b320__generated,
            0x0028_b320
        ),
        (
            oc777_fake_0028b1d8,
            OC777_PREV_0028B1D8,
            ee::s07::OVERRIDE_0028B1D8,
            ee::s07::f_0028b1d8__generated,
            0x0028_b1d8
        ),
        (
            oc777_fake_002b4388,
            OC777_PREV_002B4388,
            ee::s08::OVERRIDE_002B4388,
            ee::s08::f_002b4388__generated,
            0x002b_4388
        ),
        (
            oc777_fake_002a77f8,
            OC777_PREV_002A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8
        ),
        (
            oc777_fake_002a7890,
            OC777_PREV_002A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890
        ),
        (
            oc777_fake_002a7bf8,
            OC777_PREV_002A7BF8,
            ee::s08::OVERRIDE_002A7BF8,
            ee::s08::f_002a7bf8__generated,
            0x002a_7bf8
        ),
        (
            oc777_fake_002a7f90,
            OC777_PREV_002A7F90,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90
        ),
        (
            oc777_fake_002a7ff8,
            OC777_PREV_002A7FF8,
            ee::s08::OVERRIDE_002A7FF8,
            ee::s08::f_002a7ff8__generated,
            0x002a_7ff8
        ),
        (
            oc777_fake_002b49c0,
            OC777_PREV_002B49C0,
            ee::s08::OVERRIDE_002B49C0,
            ee::s08::f_002b49c0__generated,
            0x002b_49c0
        ),
        (
            oc777_fake_002a78f8,
            OC777_PREV_002A78F8,
            ee::s08::OVERRIDE_002A78F8,
            ee::s08::f_002a78f8__generated,
            0x002a_78f8
        ),
        (
            oc777_fake_002a78e0,
            OC777_PREV_002A78E0,
            ee::s08::OVERRIDE_002A78E0,
            ee::s08::f_002a78e0__generated,
            0x002a_78e0
        ),
        (
            oc777_fake_002a7a20,
            OC777_PREV_002A7A20,
            ee::s08::OVERRIDE_002A7A20,
            ee::s08::f_002a7a20__generated,
            0x002a_7a20
        ),
        (
            oc777_fake_002a7c68,
            OC777_PREV_002A7C68,
            ee::s08::OVERRIDE_002A7C68,
            ee::s08::f_002a7c68__generated,
            0x002a_7c68
        ),
        (
            oc777_fake_002aab98,
            OC777_PREV_002AAB98,
            ee::s08::OVERRIDE_002AAB98,
            ee::s08::f_002aab98__generated,
            0x002a_ab98
        ),
        (
            oc777_fake_002b0348,
            OC777_PREV_002B0348,
            ee::s08::OVERRIDE_002B0348,
            ee::s08::f_002b0348__generated,
            0x002b_0348
        ),
        (
            oc777_fake_0029f378,
            OC777_PREV_0029F378,
            ee::s08::OVERRIDE_0029F378,
            ee::s08::f_0029f378__generated,
            0x0029_f378
        ),
        (
            oc777_fake_003b8160,
            OC777_PREV_003B8160,
            ee::s13::OVERRIDE_003B8160,
            ee::s13::f_003b8160__generated,
            0x003b_8160
        ),
        (
            oc777_fake_004166f4,
            OC777_PREV_004166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc777_fake_00417828,
            OC777_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc777_fake_0041aa88,
            OC777_PREV_0041AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88
        ),
        (
            oc777_fake_00418ef8,
            OC777_PREV_00418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
        (
            oc777_fake_0014bdb8,
            OC777_PREV_0014BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc777_fake_0014d448,
            OC777_PREV_0014D448,
            ee::s01::OVERRIDE_0014D448,
            ee::s01::f_0014d448__generated,
            0x0014_d448
        ),
        (
            oc777_fake_0019cbe0,
            OC777_PREV_0019CBE0,
            ee::s03::OVERRIDE_0019CBE0,
            ee::s03::f_0019cbe0__generated,
            0x0019_cbe0
        ),
        (
            oc777_fake_0019cc30,
            OC777_PREV_0019CC30,
            ee::s03::OVERRIDE_0019CC30,
            ee::s03::f_0019cc30__generated,
            0x0019_cc30
        ),
        (
            oc777_fake_0019da10,
            OC777_PREV_0019DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc777_fake_0019dc20,
            OC777_PREV_0019DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc777_fake_003e1908,
            OC777_PREV_003E1908,
            ee::s14::OVERRIDE_003E1908,
            ee::s14::f_003e1908__generated,
            0x003e_1908
        ),
    );

    /// `EventSelect_PostInput` (the MAP screen's `onInputAlways`): the input
    /// object's two virtual predicates are the real zero leaf or a fake
    /// (non-zero), the map state at `gp-0xb54` takes 0..3. State 0 runs the
    /// gated list/panel fakes; states 1 and 2 enter the real
    /// `RefreshPeakGoals`/`RefreshEventList`, whose first call (`0x317670`,
    /// a gated fake) is made to unwind.
    #[test]
    fn event_select_post_input_matches() {
        oc777_install_fakes();
        check(
            ee::s05::f_00202770__generated,
            event_select_post_input,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777b, 4);
                c.pc = 0x0020_2770;
                oc777_gate(c);
                let obj = OC777_OBJ;
                let input = OC777_OBJ + 0x100;
                let vt = OC777_OBJ + 0x200;
                let up_fake = seed & 1 == 0;
                let down_fake = seed & 2 == 0;
                let state = (seed >> 2) % 4;
                c.bus.write32(input + 8, vt);
                c.bus.write16(vt + 0x88, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(
                    vt + 0x8c,
                    if up_fake {
                        fakes::FAKE
                    } else {
                        OC777_ZERO_LEAF
                    },
                );
                c.bus.write16(vt + 0x90, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(
                    vt + 0x94,
                    if down_fake {
                        fakes::FAKE + 0x10
                    } else {
                        OC777_ZERO_LEAF
                    },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0xb54), state);
                if state == 1 || state == 2 {
                    // One fake predicate call comes first, then the callee's own
                    // first fake call (0x317670) is the one that unwinds.
                    let before = (up_fake || down_fake) as u32;
                    c.bus.write32(fakes::LOG_UNWIND_AT, before);
                }
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(input);
            },
        );
    }

    /// `EventSelect_InitPeakLocks`: both game paths, all lock results.
    #[test]
    fn event_select_init_peak_locks_matches() {
        fakes::covered(
            ee::s05::f_00201750__generated,
            event_select_init_peak_locks,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x201750, 14);
                oc786_arm(c);
                c.pc = 0x0020_1750;
                c.g[A0][0] = sx(OC786_OBJ);
                c.bus.write8(0x0053_5c11, if seed % 2 == 0 { 2 } else { 1 });
                c.bus.write32(OC786_ZERO_MASK, r.next_u32());
                for i in 0..3 {
                    c.bus.write32(OC786_OBJ + 4 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// Arms oc797's gated callee fakes in a machine.
    pub(crate) const OC797_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC797_MAGIC: u32 = 0x0797_f4ce;

    /// Word the speech fake's forced result is read from (0: the fake's own).
    pub(crate) const OC797_MAIL: u32 = DATA + 0x30_0010;

    /// The fake callee: logs the call, then returns the forced `v0` of the
    /// mailbox (stored +1) unless it unwound.
    pub(crate) fn oc797_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC797_MAIL);
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc797_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC797_CTRL) != OC797_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc797_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc797_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc797_fakes!(
        (
            oc797_fk_39e390,
            OC797_PREV_39E390,
            ee::s13::OVERRIDE_0039E390,
            ee::s13::f_0039e390__generated,
            0x0039_e390
        ),
        (
            oc797_fk_3d7ec8,
            OC797_PREV_3D7EC8,
            ee::s14::OVERRIDE_003D7EC8,
            ee::s14::f_003d7ec8__generated,
            0x003d_7ec8
        ),
    );

    /// `EventSelect_ResetFlags`: the mode word below and at/above 2.
    #[test]
    fn event_select_reset_flags_matches() {
        check(
            ee::s05::f_00201720__generated,
            event_select_reset_flags,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797a, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[RA][0] = DT_RA as u64;
                let mode = [0u32, 1, 2, 3, 0xffff_ffff, 7][seed as usize % 6];
                c.bus.write32(GAME_GP - EVENT_SELECT_MODE_GP, mode);
            },
        );
    }
}
