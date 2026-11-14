//! `src/fe/fememcardcallbacks.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tagsunk::vslot;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at_m2;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::fe::feasyncfile::lh_3 as lh_2;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_3;
pub(crate) use crate::game::fe::feasyncfile::STR_HASH;
pub(crate) use crate::game::fe::fepopupmisc::gp_2 as gp;
pub(crate) use crate::game::fe::fepopupmisc::lh;
pub(crate) use crate::game::fe::fepopupmisc::lw_2;
pub(crate) use crate::game::fe::fepopupmisc::p_2;
pub(crate) use crate::game::fe::fepopupmisc::sw;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_5dd8,
        c_fe_popup_confirm_rebuild,
        "cFEPopupConfirm_Rebuild",
    ),
    (
        0x001d_72a8,
        memcard_ui_on_load_aborted,
        "MemCardUI_OnLoadAborted",
    ),
    (
        0x001d_8160,
        mem_card_ui_try_next_slot,
        "MemCardUI_TryNextSlot",
    ),
    (
        0x001d_8358,
        mem_card_ui_show_file_display,
        "MemCardUI_ShowFileDisplay",
    ),
    (0x001d_8670, mem_card_state_base_v46, "MemCardStateBase_v46"),
    (0x001d_86b0, mem_card_state_base_v45, "MemCardStateBase_v45"),
    (
        0x001d_8be0,
        mem_card_state_base_reset_state,
        "MemCardStateBase_ResetState",
    ),
    (
        0x001d_8c50,
        memcard_state_base_handle_input,
        "MemCardStateBase_HandleInput",
    ),
    (
        0x001d_8d68,
        mem_card_state_base_show_status,
        "MemCardStateBase_ShowStatus",
    ),
    (
        0x001d_9258,
        memcard_state_base_sync_menu_selection,
        "MemCardStateBase_SyncMenuSelection",
    ),
    (
        0x001d_9308,
        mem_card_state_base_set_use_save_title,
        "MemCardStateBase_SetUseSaveTitle",
    ),
    (
        0x001d_9368,
        mem_card_state_base_set_mode_menu,
        "MemCardStateBase_SetModeMenu",
    ),
    (
        0x001d_93d8,
        mem_card_state_base_transition_out,
        "MemCardStateBase_TransitionOut",
    ),
    (
        0x001d_93e8,
        mem_card_state_base_play_widget_anim,
        "MemCardStateBase_PlayWidgetAnim",
    ),
    (
        0x001d_97e0,
        mem_card_state_base_step_slot_dir,
        "MemCardStateBase_StepSlotDir",
    ),
    (
        0x001d_9818,
        mem_card_state_base_on_activate,
        "MemCardStateBase_OnActivate",
    ),
    (
        0x001d_9880,
        memcard_state_base_is_card_ready,
        "MemCardStateBase_IsCardReady",
    ),
    (
        0x001d_9948,
        mem_card_state_base_alloc_read_buffer,
        "MemCardStateBase_AllocReadBuffer",
    ),
    (0x001d_99c8, cfe_popup_confirm_ctor, "cFEPopupConfirm_ctor"),
    (0x001d_9ca8, c_fe_popup_confirm_v27, "cFEPopupConfirm_v27"),
    (0x001d_9d18, c_fe_popup_confirm_v25, "cFEPopupConfirm_v25"),
    (
        0x001d_66b0,
        mem_card_ui_on_save_done,
        "MemCardUI_OnSaveDone",
    ),
    (
        0x001d_67c8,
        mem_card_ui_on_load_done,
        "MemCardUI_OnLoadDone",
    ),
    (
        0x001d_68e8,
        mem_card_ui_confirm_overwrite,
        "MemCardUI_ConfirmOverwrite",
    ),
    (
        0x001d_69e0,
        mem_card_state_base_show_create_edit_msg,
        "MemCardStateBase_ShowCreateEditMsg",
    ),
    (
        0x001d_6ad0,
        mem_card_ui_on_load_failed,
        "MemCardUI_OnLoadFailed",
    ),
    (
        0x001d_6c28,
        mem_card_ui_on_save_failed,
        "MemCardUI_OnSaveFailed",
    ),
    (
        0x001d_6d88,
        mem_card_ui_on_card_not_detected,
        "MemCardUI_OnCardNotDetected",
    ),
    (
        0x001d_7010,
        mem_card_ui_on_save_corrupt,
        "MemCardUI_OnSaveCorrupt",
    ),
    (
        0x001d_7190,
        mem_card_ui_show_corrupt_save,
        "MemCardUI_ShowCorruptSave",
    ),
    (
        0x001d_7318,
        mem_card_ui_confirm_format,
        "MemCardUI_ConfirmFormat",
    ),
    (
        0x001d_7420,
        mem_card_ui_on_format_done,
        "MemCardUI_OnFormatDone",
    ),
    (
        0x001d_7548,
        mem_card_ui_on_format_failed,
        "MemCardUI_OnFormatFailed",
    ),
    (
        0x001d_7688,
        mem_card_ui_confirm_delete,
        "MemCardUI_ConfirmDelete",
    ),
    (
        0x001d_77e8,
        mem_card_ui_on_card_not_detected2,
        "MemCardUI_OnCardNotDetected2",
    ),
    (
        0x001d_7998,
        mem_card_ui_on_not_enough_space,
        "MemCardUI_OnNotEnoughSpace",
    ),
    (
        0x001d_7b18,
        mem_card_ui_on_not_enough_space2,
        "MemCardUI_OnNotEnoughSpace2",
    ),
    (
        0x001d_7c68,
        mem_card_ui_on_name_exists,
        "MemCardUI_OnNameExists",
    ),
    (
        0x001d_7d70,
        mem_card_ui_on_different_card,
        "MemCardUI_OnDifferentCard",
    ),
    (
        0x001d_7ed0,
        mem_card_ui_on_delete_done,
        "MemCardUI_OnDeleteDone",
    ),
    (
        0x001d_8020,
        mem_card_ui_on_delete_failed,
        "MemCardUI_OnDeleteFailed",
    ),
    (0x001d_8240, mem_card_ui_on_no_cards, "MemCardUI_OnNoCards"),
    (
        0x001d_83a8,
        mem_card_ui_check_cards_present,
        "MemCardUI_CheckCardsPresent",
    ),
    (
        0x001d_8530,
        mem_card_state_base_show_message,
        "MemCardStateBase_ShowMessage",
    ),
    (
        0x001d_8c38,
        mem_card_state_base_reset_after_save,
        "MemCardStateBase_ResetAfterSave",
    ),
    (
        0x001d_9470,
        mem_card_state_base_set_mode,
        "MemCardStateBase_SetMode",
    ),
    (
        0x001d_9a80,
        c_fe_popup_confirm_init_buttons,
        "cFEPopupConfirm_InitButtons",
    ),
    (0x001d_9bd0, c_fe_popup_confirm_v26, "cFEPopupConfirm_v26"),
    (
        0x001d_9d48,
        c_fe_state_net_help_popup_ctor,
        "cFEStateNetHelpPopup_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

pub(crate) mod callee {

    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;

    pub const MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE: Callee = Callee {
        addr: 0x001d_9308,
        f: ee::s04::f_001d9308,
    };
}

/// `void cFEPopupConfirm_Rebuild(self)` (0x001c5dd8): re-lay out the confirm
/// popup when it has a background, then scale and show it.
pub fn c_fe_popup_confirm_rebuild(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S0, 0x2bc)).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x20));
            c.cycles += 3;
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 21;
        callee::FE_POPUP_CONFIRM_RESET_LAYOUT.call(c, 0x001c_5dfc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(word(c, S0).wrapping_add(0xbc));
        callee::FE_POPUP_CONFIRM_APPLY_CONFIG.call(c, 0x001c_5e08)?;
        c.g[A0][0] = c.g[S0][0];
        c.ldl(2, at(c, S0, 0x27f));
        c.ldr(2, at(c, S0, 0x278));
        c.g[V1][0] = Addr(at(c, S0, 0x280)).lw(c);
        c.sdl(2, at(c, SP, 0x7));
        c.sdr(2, at(c, SP, 0));
        Addr(at(c, SP, 0x8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[SP][0];
        callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE.call(c, 0x001c_5e2c)?;
        c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(at(c, V0, 0x30)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x34)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V1), 0x001c_5e44)?;
        c.lq(16, frame.at(c, 0x20));
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
}

/// `cFEPopupConfirm_...` (v24) 0x1d9d18: store `a1` at_m2 +0x168 and, when the
/// popup has a live object at_m2 +0x40, apply its layout (`cFEStateNetHelpPopup`'s
/// sibling hook).
pub fn c_fe_popup_confirm_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let value = c.g[A1][0];
        c.sw(this.wrapping_add(0x168), value);
        let obj = c.lw(this.wrapping_add(0x40));
        c.g[V0][0] = obj;
        let idle = obj == 0;
        frame.restore_ra(c, 0);
        if !idle {
            c.cycles += 3;
            call_ptr(c, 0x001c_5dd8, 0x001d_9d38)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int MemCardStateBase_StepSlotDir(MemCardStateBase* self, void* list, int idx,
/// int back)` (0x1d97e0): step `idx` by ±1 and report whether it is still in
/// the list (1) or out of range (-1).
pub fn mem_card_state_base_step_slot_dir(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 1;
    c.g[V1][0] = u64::MAX;
    if c.g[A3][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let negative = (c.g[A2][0] as i64) < 0;
    c.g[V0][0] = u64::MAX;
    if negative {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x424)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let too_big = c.g[V1][0] != 0;
    c.g[V0][0] = 1;
    if too_big {
        c.cycles += 1;
        c.g[V0][0] = u64::MAX;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MemCardStateBase_SetModeMenu(State* s)` (0x1d9368): enter mode 1
/// once (`+0x214`), calling the state's own `v` slot (+0x118) with 1.
pub fn mem_card_state_base_set_mode_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let this = crate::abi::r32(c, A0);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        c.g[V0][0] = lw(c, this, 0x214);
        let already = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = 1;
        if !already {
            c.cycles += 6;
            Addr(p(this, 0x214)).sw(c, c.g[V1][0]);
            c.g[V0][0] = lw(c, this, 8);
            let vt = c.g[V0][0];
            let m = VSlot::read(c, vt, 0x118);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(c.g[A2][0]);
            crate::call::call_ptr(c, m.func as u32, 0x001d_939c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_ShowFileDisplay(self)` (0x1d8358): move the front end into
/// the file-display state (mode 2) unless it is already there or a save is in
/// progress, and tell its vtable slot 0x118/0x11c.
pub fn mem_card_ui_show_file_display(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x10c8)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x2;
        let fe = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(fe).off(0x214).lw(c);
        let in_mode = c.g[V0][0] == c.g[V1][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        if !in_mode {
            c.cycles += 3;
            c.g[V0][0] = Addr(fe).off(0x1c0).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[A1][0] = 0x2;
            if !busy {
                c.cycles += 7;
                Addr(fe).off(0x214).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(fe).off(8).lw(c);
                let vt = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(vt).off(0x118).lh(c);
                c.g[V1][0] = Addr(vt).off(0x11c).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001d_8398);
                c.g[A0][0] = sx(fe.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001d_8398)?;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_v45(self)` (0x1d86b0): enable the save title, set
/// the "check cards" mode (5) and call the state's vtable slot 0x178/0x17c.
pub fn mem_card_state_base_v45(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE.call(c, 0x001d_86c8)?;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = 0x5;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x1a8).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(this).off(8).lw(c);
        Addr(this).off(0x1c0).sw(c, c.g[V1][0]);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x178).lh(c);
        c.g[V1][0] = Addr(vt).off(0x17c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_86ec);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_86ec)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_ResetState(self)` (0x1d8be0): clear the save-title
/// and dialog fields and set the default use-save-title mode.
pub fn mem_card_state_base_reset_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        Addr(p(this, 0x1a8)).sw(c, 0);
        Addr(p(this, 0x1f0)).sw(c, 0);
        Addr(p(this, 0x1ec)).sw(c, 0);
        Addr(p(this, 0x1b0)).sw(c, 0);
        c.g[RA][0] = sx(0x001d_8c10);
        Addr(p(this, 0x1e4)).sw(c, c.g[V0][0]);
        callee::MEM_CARD_SET_USE_SAVE_TITLE.call(c, 0x001d_8c10)?;
        c.g[V0][0] = 2;
        Addr(p(this, 0x1dc)).sw(c, 0);
        Addr(p(this, 0x1c0)).sw(c, c.g[V0][0]);
        Addr(p(this, 0x1e0)).sw(c, 0);
        Addr(p(this, 0x1e8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[inline(always)]
pub(crate) fn sh(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sh(c, v);
}

/// `void MemCardStateBase_SetUseSaveTitle(MemCardStateBase* self, int on)`
/// (0x1d9308): store the flag and, when on, let the overlay manager's current
/// movie player know that the state now owns the title area.
pub fn mem_card_state_base_set_use_save_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let off = c.g[A1][0] == 0;
        sw(c, p_2(c.g[S0][0], 0x1f4), c.g[A1][0]);
        if !off {
            c.cycles += 12;
            c.g[A0][0] = lw_2(c, gp(c, -0x848));
            call_ptr(c, 0x0022_7f80, 0x001d_9328)?;
            c.g[A3][0] = lw_2(c, p_2(c.g[V0][0], 0x434));
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            c.g[A1][0] = lw_2(c, p_2(c.g[V0][0], 0xf8));
            c.g[V0][0] = lw_2(c, p_2(c.g[A3][0], 0));
            c.g[A0][0] = lh(c, p_2(c.g[V0][0], 0x70));
            c.g[V1][0] = lw_2(c, p_2(c.g[V0][0], 0x74));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_9348)?;
        } else {
            c.cycles += 2;
            sh(c, p_2(c.g[S0][0], 0x9c), 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_OnActivate(MemCardStateBase* self)` (0x1d9818):
/// activate the UI screen, then show the "busy" widget found by its name hash.
pub fn mem_card_state_base_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e4c0, 0x001d_982c)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x78b0));
        call_ptr(c, 0x0031_7670, 0x001d_9838)?;
        c.g[A0][0] = lw_2(c, p_2(c.g[S0][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001d_9844)?;
        c.g[A2][0] = c.g[V0][0];
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = lw_2(c, p_2(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, p_2(c.g[V0][0], 0x48));
            c.g[V1][0] = lw_2(c, p_2(c.g[V0][0], 0x4c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_9864)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card and front-end overlay states.

/// `void MemCardStateBase_v46(void*)` (0x1d8670): restart the front-end's
/// state machine if it is in state 4, then clear the two progress words of
/// the memory-card screen (the FE singleton at_m2 `gp-0x10c8`).
pub fn mem_card_state_base_v46(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    c.g[V1][0] = 0x4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c0));
    let not4 = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    if !not4 {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
        c.g[V0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    }
    c.cycles += 3;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), 0);
    abi::ret(c);
}

/// `int MemCardStateBase_TransitionOut(MemCardStateBase* state)` (0x1d93d8):
/// `state->+0x1c0 == 6`.
pub fn mem_card_state_base_transition_out(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x1c0)).lw(c);
    c.g[V0][0] ^= 0x6;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// The ports.

/// `void MemCardUI_OnLoadAborted(MemCardUI* self)` (0x1d72a8): abort the
/// front-end's memory-card load: notify the FE v-table entry 41, reset the
/// card manager mode and tear down its object's v-table entry 1.
pub fn memcard_ui_on_load_aborted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = Addr(gpr(c, -0x10c8)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_m2(c, A1, 8)).lw(c);
        c.g[A0][0] = Addr(at_m2(c, V0, 0x148)).lh(c);
        c.g[V1][0] = Addr(at_m2(c, V0, 0x14c)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_m2(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001d_72cc)?;

        c.g[V0][0] = Addr(gpr(c, -0x10c8)).lw(c);
        c.g[A0][0] = Addr(gpr(c, -0x848)).lw(c);
        Addr(at_m2(c, V0, 0x1e4)).sw(c, 0);
        call_ptr(c, 0x0022_7f80, 0x001d_72dc)?;
        c.g[S0][0] = c.g[V0][0];

        c.g[A1][0] = 0x24;
        c.g[V0][0] = Addr(at_m2(c, S0, 0x748)).lw(c);
        c.g[A0][0] = Addr(at_m2(c, V0, 8)).lh(c);
        c.g[V1][0] = Addr(at_m2(c, V0, 0xc)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_m2(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001d_72f8)?;

        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0023_d570, 0x001d_7304)?;

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_v27(cFEPopupConfirm* self, const u16* src)`
/// (0x1d9ca8): set the popup's text from a source UTF-16 string.
pub fn c_fe_popup_confirm_v27(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        let no_src = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !no_src {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x002c_24d0, 0x001d_9cd0)?;
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x360));
            if !empty {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                call::call_ptr(c, 0x002c_2508, 0x001d_9ce4)?;
                c.g[V1][0] = 1;
                c.g[V0][0] = Addr(p_3(r32(c, S2), 0x40)).lw(c);
                Addr(p_3(r32(c, S2), 0xc4)).sw(c, c.g[S0][0]);
                let no_refresh = c.g[V0][0] == 0;
                Addr(p_3(r32(c, S2), 0xc0)).sw(c, c.g[V1][0]);
                if !no_refresh {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S2][0];
                    call::call_ptr(c, 0x001c_5dd8, 0x001d_9d00)?;
                }
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `MemCardStateBase_ShowStatus` (0x1D8D68): show the memory card status text and
/// hand it to the state's UI object.
pub fn mem_card_state_base_show_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x660);
        frame.save(c, S0, 0x650);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x640);
        c.g[A0][0] = Addr(gp_2(c, -0x848)).lw(c);
        call_ptr(c, addr::WORLD_GET, 0x001d_8d80)?;
        c.g[V1][0] = 0x16;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[SP][0];
        sw_at(c, S0, 0x19c, c.g[V1][0]);
        c.g[A2][0] = 0x320;
        call_ptr(c, addr::MC_STATUS_TEXT, 0x001d_8d98)?;
        c.g[V0][0] = lw_at(c, S0, 0x8);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 1;
        c.g[A3][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x120)).lh(c);
        c.g[T0][0] = 0;
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x124)).lw(c);
        c.g[T1][0] = 0;
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        let show = r32(c, V1);
        c.g[T2][0] = 1;
        call_ptr(c, show, 0x001d_8dc4)?;
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(frame.at(c, 0x640)).ld(c);
        sw_at(c, S0, 0x210, c.g[V0][0]);
        frame.restore(c, S0, 0x650);
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x660));
        c.pc = ret;
        Ok(())
    })
}

/// `void MemCardStateBase_AllocReadBuffer(self, size)` (0x001d9948):
/// (re)allocate the memcard state's read buffer.
pub fn mem_card_state_base_alloc_read_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at_2(c, S0, 0x228)).lw(c);
        let fresh = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if fresh {
            c.cycles += 15;
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(at_2(c, S0, 0x1d4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                callee::MEM_FREE_2.call(c, 0x001d_9980)?;
            }
            Addr(at_2(c, S0, 0x1d4)).sw(c, 0);
            c.cycles += 1;
            Addr(at_2(c, S0, 0x228)).sw(c, 0);
            c.cycles += 15;
        }
        c.g[A1][0] = sx(0x460000);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x460000u32.wrapping_add(0x7990));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW_NAMED.call(c, 0x001d_99a0)?;
        c.g[V1][0] = 1;
        Addr(at_2(c, S0, 0x1d4)).sw(c, c.g[V0][0]);
        Addr(at_2(c, S0, 0x228)).sw(c, c.g[V1][0]);
        Addr(at_2(c, S0, 0x1d0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_PlayWidgetAnim(MemCardStateBase* self, char* name,
/// char* anim)` (0x1d93e8): find the widget named `name` and play the named
/// animation on it (animations come from the anim set at_3
/// `self->screen->something + 0x50`).
pub fn mem_card_state_base_play_widget_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(16, at_3(sp0, 0x30));
        c.sq(18, at_3(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at_3(sp0, 0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::STR_HASH, 0x001d_940c)?;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at_2(c, this, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_2, 0x001d_9418)?;
        c.g[S1][0] = c.g[V0][0];
        let found = c.g[S1][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if found {
            c.cycles += 8;
            c.g[S0][0] = lw_at_2(c, this, 0x10);
            c.g[S0][0] = sx(at_3(c.g[S0][0] as u32, 0x50));
            call_ptr(c, callee::STR_HASH, 0x001d_9430)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_ANIM_SET_CREATE_ANIM, 0x001d_943c)?;
            let anim = c.g[V0][0];
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !none {
                c.cycles += 5;
                c.g[A1][0] = anim;
                c.g[A2][0] = 3;
                c.g[A3][0] = 0;
                c.g[T0][0] = 1;
                call_ptr(c, callee::UI_OBJECT_SET_ANIM, 0x001d_9458)?;
            }
        }
        c.cycles += 6;
        c.lq(16, at_3(sp0, 0x30));
        c.lq(17, at_3(sp0, 0x20));
        c.lq(18, at_3(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MemCardStateBase_SyncMenuSelection` (0x1d9258): look up the memory-card
// menu by name hash, choose the selected item (first selectable when there is
// no current item or it is disabled), apply it, and remember its index.
pub fn memcard_state_base_sync_menu_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1600));
        call_ptr(c, 0x00317670, 0x001d_927c)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x001d_9288)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
            let no_item = c.g[V0][0] == 0;
            c.g[S1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
            let mut find_first = no_item;
            if !no_item {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                find_first = c.g[V0][0] == 0;
                c.g[V0][0] = 0xff;
            }
            if find_first {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0039b7b0, 0x001d_92c0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = 0xff;
            }
            c.cycles += 2;
            let current = c.g[S1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if !current {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0] & 0xff;
                call_ptr(c, 0x0039b960, 0x001d_92d8)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c4), c.g[S1][0]);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0039b960, 0x001d_92e8)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c4), 0u64);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEPopupConfirm_ctor(self, engine, owner, text, a4..a8)` (0x1d99c8):
/// construct a confirmation popup, choosing the button count from the global
/// popup mode.
pub fn cfe_popup_confirm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T0][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[T2][0];
        c.g[S5][0] = c.g[T3][0];
        call_ptr(c, 0x001c_58e8, 0x001d_9a04)?;
        c.g[V1][0] = 0x470000;
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_ef4c)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3770));
        let already = c.g[V0][0] != 0;
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V1][0]);
        if already {
            c.cycles += 4;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = 0x4;
            c.sw(p(c.g[S0][0] as u32, 0x17c), c.g[V0][0]);
            c.sw(p(c.g[S0][0] as u32, 0x14c), c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x17c), c.g[V0][0]);
        }
        c.cycles += 18;
        c.g[T2][0] = Addr(p(c.g[SP][0] as u32, 0x70)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[T1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_9a80, 0x001d_9a54)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card / load screens.

/// `bool MemCardStateBase_IsCardReady(MemCardStateBase *self)` (0x1d9880):
/// the card is ready when the state is neither "none" (0) nor checking (6),
/// the menu entry is set, the dialog is idle and the port check passes.
pub fn memcard_state_base_is_card_ready(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d9930: {
            'b_1d992c: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[RA][0] = sx(0x001d98a4);
                c.g[S2][0] = 0u64;
                call_ptr(c, 0x00227f80, 0x001d98a4)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                let t = c.g[V1][0] == 0u64;
                c.g[S1][0] = c.g[V0][0];
                if t {
                    break 'b_1d992c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        break 'b_1d9930;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x340));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[S2][0];
                        if t {
                            break 'b_1d9930;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.g[S2][0];
                            if t {
                                break 'b_1d9930;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                vslot(c, c.g[V1][0], 0x58, V0);
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001d98ec);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001d98ec)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1d9930;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x130));
                                    c.g[V0][0] = c.g[V1][0] ^ 0x6;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[V1][0] ^ 0x1;
                                    if t {
                                        break 'b_1d992c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_1d9930;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                            vslot(c, c.g[V1][0], 0x258, V0);
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001d9924);
                                            c.g[A0][0] =
                                                sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001d9924)?;
                                            c.g[V1][0] = 0x1;
                                            if c.g[V0][0] != 0 {
                                                c.g[S2][0] = c.g[V1][0];
                                            }
                                            break 'b_1d992c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
            break 'b_1d9930;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MemCardUI_TryNextSlot (0x00001d8160).

/// `MemCardUI_TryNextSlot`: advance the memory-card screen to the next usable slot.
pub fn mem_card_ui_try_next_slot(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d8234: {
            'b_1d8210: {
                'b_1d8208: {
                    'b_1d81f0: {
                        'b_1d81a8: {
                            c.cycles += 13;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            call_ptr(c, 0x00227f80, 0x001d8178)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x424));
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[A0][0] = c.g[S0][0];
                                break 'b_1d81a8;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x0023fb18, 0x001d819c)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                let t = true;
                                c.g[A1][0] = 0x31;
                                if t {
                                    break 'b_1d81f0;
                                } else {
                                    break 'b_1d81a8;
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0023fb18, 0x001d81b0)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                        c.g[A1][0] = 0u64;
                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1a4));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[RA][0] = sx(0x001d81cc);
                        c.pc = t;
                        ssx3_ee_rt::funcs::call_indirect(c, t);
                        if c.pc != 0x001d81cc {
                            return Err(crate::call::Unwound);
                        }
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_1d8210;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x208));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x6;
                            if t {
                                break 'b_1d8208;
                            } else {
                                c.cycles += 3;
                                c.g[A1][0] = 0x2;
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                break 'b_1d81f0;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[RA][0] = sx(0x001d8200);
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x001d8200 {
                        return Err(crate::call::Unwound);
                    }
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1d8234;
                    } else {
                        break 'b_1d8208;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00241400, 0x001d8210)?;
                break 'b_1d8210;
            }
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023fae0, 0x001d8218)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A1][0] = 0x36;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[RA][0] = sx(0x001d8230);
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x001d8230 {
                return Err(crate::call::Unwound);
            }
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1d8234;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

/// `MemCardStateBase_HandleInput(self, input)` (0x001d8c50): let the input
/// object's "confirm" and "cancel" slots drive the two save/load actions and
/// play the matching screen frame. Returns whether input was consumed.
pub fn memcard_state_base_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        c.g[S0][0] = c.g[A1][0]; // input
        c.g[S1][0] = c.g[A0][0]; // self
        let input = c.g[S0][0];
        let this = c.g[S1][0];
        c.g[S2][0] = 0;
        c.g[V1][0] = lw_4(c, this, 0x1a8);
        let busy = c.g[V1][0] != 1;
        let mut tail = false;
        if busy {
            // Confirm slot: `input->vt[19]()`.
            c.cycles += 7;
            let vt = lw_4(c, input, 8);
            c.g[V1][0] = vt;
            let delta = lh_2(c, vt, 0x98);
            let func = lw_4(c, vt, 0x9c);
            c.g[V0][0] = func;
            c.g[A0][0] = sx((input as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x001d_8c98)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S2][0] = 1;
                c.g[V0][0] = lw_4(c, this, 0x40);
                let no_screen = c.g[V0][0] == 0;
                sw_3(c, this, 0x1fc, c.g[S2][0]);
                if no_screen {
                    tail = true;
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = sx(0x0046_7940);
                    call_ptr(c, STR_HASH, 0x001d_8cbc)?;
                    c.g[A0][0] = lw_4(c, this, 0x40);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_c778, 0x001d_8cc8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let no_frame = c.g[A1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if !no_frame {
                        c.cycles += 5;
                        c.g[A0][0] = lw_4(c, this, 0x40);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x001d_8ce4)?;
                        c.g[V0][0] = c.g[S2][0];
                    }
                }
            } else {
                // Cancel slot: `input->vt[20]()`.
                c.g[V1][0] = lw_4(c, input, 8);
                c.cycles += 6;
                c.g[A0][0] = lh_2(c, c.g[V1][0], 0xa0);
                c.g[V0][0] = lw_4(c, c.g[V1][0], 0xa4);
                c.g[A0][0] = sx((input as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x001d_8cfc)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S2][0];
                if !none {
                    c.cycles += 4;
                    c.g[S2][0] = 1;
                    c.g[V0][0] = lw_4(c, this, 0x40);
                    let no_screen = c.g[V0][0] == 0;
                    sw_3(c, this, 0x1f8, c.g[S2][0]);
                    if no_screen {
                        tail = true;
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = sx(0x0046_7950);
                        call_ptr(c, STR_HASH, 0x001d_8d20)?;
                        c.g[A0][0] = lw_4(c, this, 0x40);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001d_8d2c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = c.g[S2][0];
                        if no_frame {
                            tail = true;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = lw_4(c, this, 0x40);
                            c.g[A2][0] = 1;
                            call_ptr(c, 0x0039_c870, 0x001d_8d48)?;
                            tail = true;
                        }
                    }
                }
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        if tail {
            c.cycles += 1;
        }
        c.g[V0][0] = c.g[S2][0];
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The memory-card front end the callbacks drive.
//
// The card manager calls these when an operation finishes (`cFEMemCard_callback*`
// in the reference). Each one finds the active memory-card state through
// `gp - 0x10c8`, looks its texts up in the application's text database by the
// `StrHash` of a text id, and shows them through the state's virtual methods:
// the message popup (a `cFEPopupConfirm`) and its option texts.

/// The active memory-card front-end state (`D_004A2028`), at `gp - 0x10c8`.
pub(crate) const MEMCARD_FE_GP: u32 = 0x10c8u32.wrapping_neg();

/// The application (`D_004A28A8`), at `gp - 0x848`: `MCManager_Get`'s argument,
/// with the text database at `+0x8c`.
pub(crate) const MEMCARD_APP_GP: u32 = 0x848u32.wrapping_neg();

/// The state's vtable (and the popup's).
pub(crate) const MEMCARD_FE_VTABLE: u32 = 0x8;

/// Which message the state is showing (its input handler reads it back).
pub(crate) const MEMCARD_FE_MESSAGE: u32 = 0x19c;

/// The message popup (a `cFEPopupConfirm`, created by
/// `MemCardStateBase_ShowMessage`); its vtable is at `+0x8` too.
pub(crate) const MEMCARD_FE_POPUP: u32 = 0x218;

/// The progress flag `MemCardStateBase_ResetAfterSave` clears with the state.
pub(crate) const MEMCARD_FE_BUSY: u32 = 0x1a8;

/// Set while a save or load was started from the in-game pause menu: the
/// reply closes the screen instead of showing a message.
pub(crate) const MEMCARD_FE_SAVE_FROM_PAUSE: u32 = 0x208;

pub(crate) const MEMCARD_FE_LOAD_FROM_PAUSE: u32 = 0x204;

/// State vtable slots (`{delta, fn}` pairs): show a message in the popup
/// (`MemCardStateBase_ShowMessage(text, single, a3, t0, selected, t2)`), and
/// close the screen.
pub(crate) const MEMCARD_FE_VT_SHOW_MESSAGE: u32 = 0x120;

pub(crate) const MEMCARD_FE_VT_CLOSE: u32 = 0x140;

/// Popup vtable slots: `cFEPopupConfirm_v25` (sets the popup's `+0x168` flag
/// and re-lays it out) and `cFEPopupConfirm_v26` (sets the text of option 0,
/// 1 or 2).
pub(crate) const MEMCARD_POPUP_VT_V25: u32 = 0xc8;

pub(crate) const MEMCARD_POPUP_VT_SET_OPTION: u32 = 0xd0;

/// The application's text database (`+0x8c`); its vtable is at `+0x4` and slot
/// `0x20` finds a text by the hash of its id.
pub(crate) const MEMCARD_APP_TEXT_DB: u32 = 0x8c;

pub(crate) const MEMCARD_TEXT_DB_VT_FIND: u32 = 0x20;

/// `MCManager_Get` (0x227f80): the application's card manager. Its card object
/// is at `+0x434`, the selected save slot at `+0xf8` and the current card's
/// slot index at `+0x428`.
pub(crate) const MEMCARD_MC_MANAGER_GET: u32 = 0x0022_7f80;

pub(crate) const MEMCARD_MC_CARD: u32 = 0x434;

pub(crate) const MEMCARD_MC_SLOT: u32 = 0xf8;

pub(crate) const MEMCARD_MC_CARD_SLOT: u32 = 0x428;

/// Card vtable slot `0x70`: the display name of a save slot (UTF-16).
pub(crate) const MEMCARD_CARD_VT_SLOT_NAME: u32 = 0x70;

/// `MCManager_FormatSlotName(manager, out, slot)` (0x241dc8): the UTF-16 name
/// of the card in `slot`.
pub(crate) const MEMCARD_FORMAT_SLOT_NAME: u32 = 0x0024_1dc8;

/// `EA_MemFree` (0x317e98), `UStr_Copy` (0x2c2508) and `UStr_Sprintf`
/// (0x2c26d0); `StrHash` and `operator_new_named` are the unit's `STR_HASH`
/// and `callee::OPERATOR_NEW_NAMED`.
pub(crate) const MEMCARD_MEM_FREE: u32 = 0x0031_7e98;

pub(crate) const MEMCARD_USTR_COPY: u32 = 0x002c_2508;

pub(crate) const MEMCARD_USTR_SPRINTF: u32 = 0x002c_26d0;

/// `"Temp String"`: the allocation tag of the message buffers; flags 0x100.
pub(crate) const MEMCARD_TEMP_STRING_TAG: u32 = 0x0045_e2a0;

pub(crate) const MEMCARD_TEMP_STRING_FLAGS: u64 = 0x100;

/// The size of most message buffers.
pub(crate) const MEMCARD_TEMP_STRING_SIZE: u64 = 0x7d0;
/// The size the game uses for the buffers of the messages that format a slot or
/// card name in (overwrite, different card, checking card), and for the stack
/// buffer `OnNotEnoughSpace2` builds its text in (the card name follows it).
pub(crate) const MEMCARD_NAME_STRING_SIZE: u32 = 0x640;

// Text ids (the format strings take the card's or the slot's name).
/// `kT_CMNContinue`.
pub(crate) const MEMCARD_TEXT_CONTINUE: u32 = 0x0045_f7e0;

/// `kT_MEMOVRetry`.
pub(crate) const MEMCARD_TEXT_RETRY: u32 = 0x0045_dcd8;

/// `kT_BTNCancel`.
pub(crate) const MEMCARD_TEXT_CANCEL: u32 = 0x0046_2c88;

/// `kT_MEMOVSaveDone`.
pub(crate) const MEMCARD_TEXT_SAVE_DONE: u32 = 0x0046_77b8;

/// `kT_MEMOVLoadDone`.
pub(crate) const MEMCARD_TEXT_LOAD_DONE: u32 = 0x0046_77d0;

/// `kT_MEMOVOverWriteFile`.
pub(crate) const MEMCARD_TEXT_OVERWRITE_FILE: u32 = 0x0046_42f8;

/// `kT_POPUP200CreateEdit`.
pub(crate) const MEMCARD_TEXT_CREATE_EDIT: u32 = 0x0046_41f0;

/// `kT_MEMOVLoadFailedPS2NGC`.
pub(crate) const MEMCARD_TEXT_LOAD_FAILED: u32 = 0x0046_4608;

/// `kT_MEMOVSaveFailed`.
pub(crate) const MEMCARD_TEXT_SAVE_FAILED: u32 = 0x0046_4628;

/// `kT_MEMOVNotDetectedPS2`.
pub(crate) const MEMCARD_TEXT_NOT_DETECTED: u32 = 0x0046_44d8;

/// `kT_MEMOVGameCorruptPS2`.
pub(crate) const MEMCARD_TEXT_GAME_CORRUPT: u32 = 0x0046_77e8;

/// `kT_MEMCorruptSavePS2`.
pub(crate) const MEMCARD_TEXT_CORRUPT_SAVE: u32 = 0x0046_7800;

/// `kT_MEMFormatCardPS2`.
pub(crate) const MEMCARD_TEXT_FORMAT_CARD: u32 = 0x0046_4330;

/// `kT_MEMOVFormatComplete`.
pub(crate) const MEMCARD_TEXT_FORMAT_COMPLETE: u32 = 0x0046_7818;

/// `kT_MEMFEFormatFailed`.
pub(crate) const MEMCARD_TEXT_FORMAT_FAILED: u32 = 0x0046_44f0;

/// `kT_FENameAlreadyExists`.
pub(crate) const MEMCARD_TEXT_NAME_EXISTS: u32 = 0x0046_7860;

/// `kT_MEMDifferentMemcardPS2`.
pub(crate) const MEMCARD_TEXT_DIFFERENT_CARD: u32 = 0x0046_7878;

/// `kT_MEMDeleteSuccess`.
pub(crate) const MEMCARD_TEXT_DELETE_DONE: u32 = 0x0046_7898;

/// `kT_MEMOVDeleteFailedError`.
pub(crate) const MEMCARD_TEXT_DELETE_FAILED: u32 = 0x0046_4640;

/// `kT_MEMConfirmDeletePS2`.
pub(crate) const MEMCARD_TEXT_CONFIRM_DELETE: u32 = 0x0046_7830;

/// `kT_MEMFETooBigPS2` (takes the card's name and the space the save needs).
pub(crate) const MEMCARD_TEXT_TOO_BIG: u32 = 0x0046_7848;

// Message ids the callbacks store at [`MEMCARD_FE_MESSAGE`].
pub(crate) const MEMCARD_MSG_CONFIRM_OVERWRITE: u32 = 1;

pub(crate) const MEMCARD_MSG_CREATE_EDIT: u32 = 2;

pub(crate) const MEMCARD_MSG_CONFIRM_DELETE: u32 = 3;

pub(crate) const MEMCARD_MSG_SAVE_DONE: u64 = 5;

pub(crate) const MEMCARD_MSG_LOAD_DONE: u64 = 6;

pub(crate) const MEMCARD_MSG_NOT_DETECTED: u64 = 9;

pub(crate) const MEMCARD_MSG_LOAD_FAILED: u32 = 0xa;

pub(crate) const MEMCARD_MSG_SAVE_FAILED: u32 = 0xb;

pub(crate) const MEMCARD_MSG_GAME_CORRUPT: u32 = 0xc;

pub(crate) const MEMCARD_MSG_CORRUPT_SAVE: u32 = 0xd;

pub(crate) const MEMCARD_MSG_CONFIRM_FORMAT: u32 = 0xe;

pub(crate) const MEMCARD_MSG_FORMAT_DONE: u32 = 0x10;

pub(crate) const MEMCARD_MSG_FORMAT_FAILED: u32 = 0x11;

pub(crate) const MEMCARD_MSG_DELETE_DONE: u32 = 7;

pub(crate) const MEMCARD_MSG_DELETE_FAILED: u32 = 8;

pub(crate) const MEMCARD_MSG_NO_CARD: u32 = 0x12;

pub(crate) const MEMCARD_MSG_NAME_EXISTS: u32 = 0x17;

pub(crate) const MEMCARD_MSG_DIFFERENT_CARD: u32 = 0x18;

pub(crate) const MEMCARD_MSG_NOT_ENOUGH_SPACE: u32 = 0x13;

/// The state's state machine (`MemCardStateBase_TransitionOut` tests 6); the
/// delete reply with no saves on the card clears the busy and `+0x1e0` words
/// when it is 4.
pub(crate) const MEMCARD_FE_STATE: u32 = 0x1c0;

/// Card vtable slots: the number of saves on the card, and the space (in KB)
/// a new save needs.
pub(crate) const MEMCARD_CARD_VT_SAVE_COUNT: u32 = 0x68;

pub(crate) const MEMCARD_CARD_VT_SAVE_SIZE: u32 = 0xb8;

/// Card-manager fields that let the "not detected" reply go on without a card:
/// two flags, and a hook (a function pointer) asked when both are clear.
pub(crate) const MEMCARD_MC_CONTINUE_A: u32 = 0xbc;

pub(crate) const MEMCARD_MC_CONTINUE_B: u32 = 0xc0;

pub(crate) const MEMCARD_MC_CAN_CONTINUE: u32 = 0x78;

/// The state's display mode (`MemCardStateBase_SetModeMenu` enters 1) and the
/// vtable slot that switches it (`v(+0x118)(mode)`).
pub(crate) const MEMCARD_FE_MODE: u32 = 0x214;

pub(crate) const MEMCARD_FE_VT_SET_MODE: u32 = 0x118;

pub(crate) const MEMCARD_MODE_MENU: u64 = 1;

pub(crate) const MEMCARD_MODE_FILES: u64 = 3;

/// The state's UI screen, and its "text box message" object (found by the
/// `StrHash` of that name) the "no cards" text goes into.
pub(crate) const MEMCARD_FE_SCREEN: u32 = 0x40;

pub(crate) const MEMCARD_TEXT_BOX_NAME: u32 = 0x0046_78b0;

/// `kT_MEMNoCardsPS2`.
pub(crate) const MEMCARD_TEXT_NO_CARDS: u32 = 0x0046_78c8;

/// `UIText_SetWrappedWideString(text, str)` (0x3a0d00; `UIScreen_FindObject` is
/// `callee::UI_SCREEN_FIND_OBJECT_2`); UI-object vtable slot `0x48` shows or
/// hides it.
pub(crate) const MEMCARD_UI_TEXT_SET_WRAPPED: u32 = 0x003a_0d00;

pub(crate) const MEMCARD_UI_VT_SHOW: u32 = 0x48;

/// The card manager's state machine (`+0x748`, slot 8 sets its state) and the
/// card vtable slot that says whether any card is in.
pub(crate) const MEMCARD_MC_STATE_MACHINE: u32 = 0x748;

pub(crate) const MEMCARD_CARD_VT_ANY_CARD: u32 = 0x1a8;

/// Popup vtable slot `0xe0`: re-initialise an existing popup with a new
/// text and flags (the arguments `ShowMessage` takes; the ctor calls
/// `cFEPopupConfirm_InitButtons` with them).
pub(crate) const MEMCARD_POPUP_VT_REINIT: u32 = 0xe0;

/// The state's UI engine (`+0x10`): the popup's owner; `+0x18` in it is the UI
/// state manager the new popup is pushed on.
pub(crate) const MEMCARD_FE_ENGINE: u32 = 0x10;

/// `"cFEPopupConfirm"` (the allocation tag) and a popup's size.
pub(crate) const MEMCARD_POPUP_TAG: u32 = 0x0045_e130;

pub(crate) const MEMCARD_POPUP_SIZE: u64 = 0xbfc;

/// `UIStateMgr_PushPopup` (0x39f290; `EA_MemAllocNamed` is
/// `callee::EA_MEM_ALLOC_NAMED_2`).

pub(crate) const MEMCARD_UI_PUSH_POPUP: u32 = 0x0039_f290;

/// The message whose popup keeps the `+0x210` flag (`MemCardStateBase_ShowStatus`
/// sets the flag with message 0x16).
pub(crate) const MEMCARD_MSG_STATUS: u64 = 0x16;

/// `cFEPopupConfirm_ctor` (0x1d99c8), through its override slot.
pub(crate) const MEMCARD_POPUP_CTOR: Callee = Callee {
    addr: 0x001d_99c8,
    f: ee::s04::cFEPopupConfirm_ctor,
};

// cFEPopupConfirm: the option list at `+0xbc` (its count there, the first
// option's flag at `+0xbc + 0`, the selected option at `+0xbc + 0x98`), the
// option count at `+0xbf8`, the message text at `+0x360` and the three
// options' texts at `+0x9a0`, `+0xa68` and `+0xb30`; `+0x40` is its live UI
// object (a rebuild is due when set).
pub(crate) const POPUP_OPTIONS: u32 = 0xbc;

pub(crate) const POPUP_OPTION_COUNT: u32 = 0xbf8;

pub(crate) const POPUP_TEXT: u32 = 0x360;

pub(crate) const POPUP_OPTION_TEXT: [u32; 3] = [0x9a0, 0xa68, 0xb30];

pub(crate) const POPUP_LIVE: u32 = 0x40;

/// `kT_CMNYes` / `kT_CMNNo`, the default option texts.
pub(crate) const POPUP_TEXT_YES: u32 = 0x0045_dcc8;

pub(crate) const POPUP_TEXT_NO: u32 = 0x0045_fcd8;

/// The default message text (an empty ASCII string, `D_004A1408`).
pub(crate) const POPUP_EMPTY_TEXT: u32 = 0x004a_1408;

/// `cFEPopupConfirm_CfgSetOptionWideString(options, i, str)` (0x1c57a0),
/// `cFEPopupConfirm_CfgSetOptionTextId(options, i, hash)` (0x1c57c0),
/// `cFEPopupConfirm_Rebuild(self)` (0x1c5dd8) and `UStr_FromAscii_2` (0x2c2540).
pub(crate) const POPUP_SET_OPTION_STRING: u32 = 0x001c_57a0;

pub(crate) const POPUP_SET_OPTION_TEXT_ID: u32 = 0x001c_57c0;

pub(crate) const POPUP_REBUILD: u32 = 0x001c_5dd8;

pub(crate) const POPUP_USTR_FROM_ASCII: u32 = 0x002c_2540;

/// `cFEStateNetHelpPopup`'s vtable, its seven button slots' count and
/// `UIState_ctor_2` (0x39e318); `cFEStateNetHelpPopup_ClearFields` (0x1da4a8).
pub(crate) const NET_HELP_POPUP_VTABLE: u32 = 0x0046_a618;

pub(crate) const UI_STATE_CTOR_2: u32 = 0x0039_e318;

pub(crate) const NET_HELP_POPUP_CLEAR_FIELDS: Callee = Callee {
    addr: 0x001d_a4a8,
    f: ee::s04::cFEStateNetHelpPopup_ClearFields,
};

/// `MemCardStateBase_SetMode`'s screen objects and values, all found by the
/// `StrHash` of their names: the "onmenu" / "offmenu" objects (UI-object slot
/// `0x38` shows or hides one), and the "off" / "file display" / "on" values
/// (`UIScreen_LookupValue`, 0x39c778: a thread id, or 0xffff when absent) the
/// screen's thread is replaced with (`UIScreen_ReplaceThread`, 0x39c820).
pub(crate) const MEMCARD_NAME_ON_MENU: u32 = 0x004a_1600;

pub(crate) const MEMCARD_NAME_OFF_MENU: u32 = 0x004a_1608;

pub(crate) const MEMCARD_NAME_OFF: u32 = 0x004a_2030;

pub(crate) const MEMCARD_NAME_ON: u32 = 0x004a_2038;

pub(crate) const MEMCARD_NAME_FILE_DISPLAY: u32 = 0x0046_7980;

pub(crate) const MEMCARD_UI_VT_SET_VISIBLE: u32 = 0x38;

pub(crate) const MEMCARD_UI_SCREEN_LOOKUP_VALUE: u32 = 0x0039_c778;

pub(crate) const MEMCARD_UI_SCREEN_REPLACE_THREAD: u32 = 0x0039_c820;

pub(crate) const MEMCARD_NO_VALUE: u64 = 0xffff;

/// `kT_MEMFECheckingCardPS2` (takes the card's name).
pub(crate) const MEMCARD_TEXT_CHECKING_CARD: u32 = 0x0046_3f58;

/// `MemCardStateBase_ResetAfterSave` (0x1d8c38), through its override slot.
pub(crate) const MEMCARD_RESET_AFTER_SAVE: Callee = Callee {
    addr: 0x001d_8c38,
    f: ee::s04::MemCardStateBase_ResetAfterSave,
};

// ---------------------------------------------------------------------------
// The steps the callbacks are made of. None charges cycles: the callers charge
// them where the original does. Registers a step leaves are the original's;
// where the callbacks load a value into different scratch registers that are
// overwritten before anything reads them, the step uses one form.

/// `gp + off`.
pub(crate) fn memcard_gp(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_add(off)
}

/// The active memory-card state.
pub(crate) fn memcard_fe(c: &mut Cpu) -> u32 {
    Addr(memcard_gp(c, MEMCARD_FE_GP)).lw(c) as u32
}

/// Record which message the state shows.
pub(crate) fn memcard_set_message(c: &mut Cpu, msg: u32) {
    let fe = memcard_fe(c);
    Addr(fe.wrapping_add(MEMCARD_FE_MESSAGE)).sw(c, msg as u64);
}

/// The localised text for the id string at `name`, in `v0`:
/// `app->text_db->Find(StrHash(name))`, keeping the database (with the slot's
/// `this` adjustment) in `s0` and its vtable in `s1`.
pub(crate) fn memcard_text(c: &mut Cpu, name: u32, rets: [u32; 2]) -> Flow {
    c.g[V0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
    c.g[A0][0] = sx(name);
    c.g[S0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_APP_TEXT_DB)).lw(c);
    c.g[S1][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(MEMCARD_TEXT_DB_VT_FIND)).lh(c);
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    call_ptr(c, STR_HASH, rets[0])?;
    c.g[V1][0] = Addr(r32(c, S1).wrapping_add(MEMCARD_TEXT_DB_VT_FIND + 4)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, r32(c, V1), rets[1])
}

/// `fe->ShowMessage(text, single, 0, t0, selected, 0)`, the state in `t3`, its
/// vtable in `v0` and the method in `v1` (the usual register choice).
/// `single`: one option instead of Yes/No; `selected`: the option selected at
/// first; the "not enough space" replies pass 3 in `t0` (the popup's `+0x14c`).
pub(crate) fn memcard_show_message(
    c: &mut Cpu,
    text: u64,
    single: u64,
    t0: u64,
    selected: u64,
    ret: u32,
) -> Flow {
    c.g[T3][0] = memcard_fe(c) as u64;
    c.g[A1][0] = text;
    c.g[A2][0] = single;
    c.g[A3][0] = 0;
    c.g[V0][0] = Addr(r32(c, T3).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[T0][0] = t0;
    c.g[T1][0] = selected;
    c.g[T2][0] = 0;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, T3).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// `popup->SetOption(option, v0)` for the text just looked up, the popup in
/// `a3`.
pub(crate) fn memcard_popup_set_option(c: &mut Cpu, option: u64, ret: u32) -> Flow {
    let fe = memcard_fe(c);
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = option;
    c.g[A3][0] = Addr(fe.wrapping_add(MEMCARD_FE_POPUP)).lw(c);
    c.g[V0][0] = Addr(r32(c, A3).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_SET_OPTION)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_SET_OPTION + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// `popup->SetOption(option, text(name))` (`SetLine` in the reference) in the
/// interleaved order most callbacks use: the popup (with its slot's `this`
/// adjustment) and its vtable in `s1`/`s3`, the text database and its vtable
/// in `s0`/`s2`. The state is loaded into `fe_reg` (`a1` or `v1`, as the
/// original chose: `a1` is still live at the `StrHash` call).
pub(crate) fn memcard_popup_option(
    c: &mut Cpu,
    fe_reg: usize,
    name: u32,
    option: u64,
    rets: [u32; 3],
) -> Flow {
    c.g[fe_reg][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
    c.g[A0][0] = sx(name);
    c.g[S1][0] = Addr(r32(c, fe_reg).wrapping_add(MEMCARD_FE_POPUP)).lw(c);
    c.g[S0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_APP_TEXT_DB)).lw(c);
    c.g[S3][0] = Addr(r32(c, S1).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[S2][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(MEMCARD_POPUP_VT_SET_OPTION)).lh(c);
    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(MEMCARD_TEXT_DB_VT_FIND)).lh(c);
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
    call_ptr(c, STR_HASH, rets[0])?;
    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(MEMCARD_TEXT_DB_VT_FIND + 4)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, r32(c, V1), rets[1])?;
    c.g[A3][0] = Addr(r32(c, S3).wrapping_add(MEMCARD_POPUP_VT_SET_OPTION + 4)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = option;
    call_ptr(c, r32(c, A3), rets[2])
}

/// `popup->v25(flag)` (`cFEPopupConfirm_v25`).
pub(crate) fn memcard_popup_v25(c: &mut Cpu, flag: u64, ret: u32) -> Flow {
    let fe = memcard_fe(c);
    c.g[A1][0] = flag;
    c.g[A2][0] = Addr(fe.wrapping_add(MEMCARD_FE_POPUP)).lw(c);
    c.g[V0][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_V25)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_V25 + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// Call the state's close slot; the state is in `a1`.
pub(crate) fn memcard_close(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V0][0] = Addr(r32(c, A1).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_CLOSE)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_CLOSE + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// A "Temp String" message buffer of `size` bytes, in `v0`.
pub(crate) fn memcard_new_temp_string(c: &mut Cpu, size: u64, ret: u32) -> Flow {
    c.g[A1][0] = sx(MEMCARD_TEMP_STRING_TAG);
    c.g[A2][0] = MEMCARD_TEMP_STRING_FLAGS;
    c.g[A3][0] = 0;
    c.g[A0][0] = size;
    callee::OPERATOR_NEW_NAMED.call(c, ret)
}

/// Free the message buffer in register `buf` if there is one; `s0` is
/// restored from `s0_slot` on both paths at the point the original does it.
pub(crate) fn memcard_free_temp_string(
    c: &mut Cpu,
    frame: Frame,
    buf: usize,
    s0_slot: u32,
    ret: u32,
) -> Flow {
    let none = c.g[buf][0] == 0;
    frame.restore(c, S0, s0_slot);
    if !none {
        c.cycles += 3;
        c.g[A0][0] = c.g[buf][0];
        call_ptr(c, MEMCARD_MEM_FREE, ret)?;
        frame.restore(c, S0, s0_slot);
    }
    Ok(())
}

/// The current card's name into `out`:
/// `MCManager_FormatSlotName(manager, out, manager->card_slot)`.
pub(crate) fn memcard_card_name(c: &mut Cpu, manager: u64, out: u64, ret: u32) -> Flow {
    c.g[A0][0] = manager;
    c.g[A1][0] = out;
    c.g[A2][0] = Addr(r32(c, A0).wrapping_add(MEMCARD_MC_CARD_SLOT)).lw(c);
    call_ptr(c, MEMCARD_FORMAT_SLOT_NAME, ret)
}

/// The selected save slot's name into `out`:
/// `manager->card->SlotName(manager->slot, out)`.
pub(crate) fn memcard_slot_name(c: &mut Cpu, manager: u32, out: u64, ret: u32) -> Flow {
    c.g[A3][0] = Addr(manager.wrapping_add(MEMCARD_MC_CARD)).lw(c);
    c.g[A2][0] = out;
    c.g[A1][0] = Addr(manager.wrapping_add(MEMCARD_MC_SLOT)).lw(c);
    c.g[V0][0] = Addr(r32(c, A3)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_CARD_VT_SLOT_NAME)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_CARD_VT_SLOT_NAME + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// `UStr_Sprintf(buf, v0, arg)`: format the text just looked up.
pub(crate) fn memcard_sprintf(c: &mut Cpu, buf: u64, arg: u64, ret: u32) -> Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = buf;
    c.g[A2][0] = arg;
    call_ptr(c, MEMCARD_USTR_SPRINTF, ret)
}

/// `UStr_Copy(buf, v0)`: copy the text just looked up.
pub(crate) fn memcard_copy(c: &mut Cpu, buf: u64, ret: u32) -> Flow {
    c.g[A0][0] = buf;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, MEMCARD_USTR_COPY, ret)
}

// ---------------------------------------------------------------------------
// Replies to finished card operations.

/// The tail the "done" replies share: show the text in `v0` as message `msg`
/// as a single option (this register choice: the method in `t3`), then label the
/// popup's option 0 "Continue". `rets` are the four calls' returns.
pub(crate) fn memcard_show_done_message(c: &mut Cpu, msg: u64, rets: [u32; 4]) -> Flow {
    c.g[A0][0] = memcard_fe(c) as u64;
    c.g[A1][0] = c.g[V0][0];
    c.g[V1][0] = msg;
    c.g[A3][0] = 0;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[A2][0] = 1;
    Addr(r32(c, A0).wrapping_add(MEMCARD_FE_MESSAGE)).sw(c, c.g[V1][0]);
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE)).lh(c);
    c.g[T3][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    call_ptr(c, r32(c, T3), rets[0])?;
    memcard_text(c, MEMCARD_TEXT_CONTINUE, [rets[1], rets[2]])?;
    memcard_popup_set_option(c, 0, rets[3])
}

/// `void cFEMemCard_callbackSaveSuccess(void)` (`MemCardUI_OnSaveDone`,
/// 0x1d66b0): the save finished. Reset the state; a save started from the
/// pause menu closes the screen, otherwise show "save done" with "Continue".
pub fn mem_card_ui_on_save_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = memcard_fe(c) as u64;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        MEMCARD_RESET_AFTER_SAVE.call(c, 0x001d_66cc)?;
        c.g[A1][0] = memcard_fe(c) as u64;
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(MEMCARD_FE_SAVE_FROM_PAUSE)).lw(c);
        let from_pause = c.g[V0][0] != 0;
        Addr(r32(c, A1).wrapping_add(0x1f0)).sw(c, 0);
        if from_pause {
            c.cycles += 6;
            memcard_close(c, 0x001d_67b4)?;
        } else {
            c.cycles += 49;
            memcard_text(c, MEMCARD_TEXT_SAVE_DONE, [0x001d_66fc, 0x001d_670c])?;
            memcard_show_done_message(
                c,
                MEMCARD_MSG_SAVE_DONE,
                [0x001d_6744, 0x001d_6764, 0x001d_6774, 0x001d_6798],
            )?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEMemCard_callbackReadSuccess(void)` (`MemCardUI_OnLoadDone`,
/// 0x1d67c8): the load finished. Drop the save title and mark the profile
/// loaded (`+0x1b4`); a load started from the pause menu closes the screen,
/// otherwise show "load done" with "Continue".
pub fn mem_card_ui_on_load_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = memcard_fe(c) as u64;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE.call(c, 0x001d_67e4)?;
        c.g[A1][0] = memcard_fe(c) as u64;
        c.g[V0][0] = 1;
        c.g[V1][0] = Addr(r32(c, A1).wrapping_add(MEMCARD_FE_LOAD_FROM_PAUSE)).lw(c);
        Addr(r32(c, A1).wrapping_add(0x1b4)).sw(c, c.g[V0][0]);
        let from_pause = c.g[V1][0] != 0;
        Addr(r32(c, A1).wrapping_add(MEMCARD_FE_BUSY)).sw(c, 0);
        if from_pause {
            c.cycles += 6;
            memcard_close(c, 0x001d_68d4)?;
        } else {
            c.cycles += 49;
            memcard_text(c, MEMCARD_TEXT_LOAD_DONE, [0x001d_681c, 0x001d_682c])?;
            memcard_show_done_message(
                c,
                MEMCARD_MSG_LOAD_DONE,
                [0x001d_6864, 0x001d_6884, 0x001d_6894, 0x001d_68b8],
            )?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_ConfirmOverwrite(void)` (0x1d68e8): ask whether to
/// overwrite the selected slot's file: `kT_MEMOVOverWriteFile` with the slot's
/// name, as a Yes/No message (1) with No selected.
pub fn mem_card_ui_confirm_overwrite(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        // 0x00..0x50: the slot name.
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save_ra(c, 0x50);
        memcard_new_temp_string(c, MEMCARD_NAME_STRING_SIZE as u64, 0x001d_6914)?;
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_6920)?;
        let sp = c.g[SP][0];
        let v0 = r32(c, V0);
        memcard_slot_name(c, v0, sp, 0x001d_6940)?;
        memcard_text(c, MEMCARD_TEXT_OVERWRITE_FILE, [0x001d_6960, 0x001d_6970])?;
        let (buf, sp) = (c.g[S2][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_6980)?;
        // fe->ShowMessage(buf, 0, 0, 0, 1, 0), the state in v1 here.
        c.g[V1][0] = memcard_fe(c) as u64;
        c.g[V0][0] = MEMCARD_MSG_CONFIRM_OVERWRITE as u64;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        Addr(r32(c, V1).wrapping_add(MEMCARD_FE_MESSAGE)).sw(c, c.g[V0][0]);
        c.g[A3][0] = 0;
        c.g[T3][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
        c.g[T0][0] = 0;
        c.g[T1][0] = 1;
        c.g[T2][0] = 0;
        c.g[A0][0] = Addr(r32(c, T3).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE)).lh(c);
        c.g[V0][0] = Addr(r32(c, T3).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE + 4)).lw(c);
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        call_ptr(c, r32(c, V0), 0x001d_69b8)?;
        memcard_free_temp_string(c, frame, S2, 0x80, 0x001d_69c8)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_ShowCreateEditMsg(void)` (0x1d69e0): show
/// `kT_POPUP200CreateEdit` as message 2 (Yes/No), with the popup's `v25` flag.
pub fn mem_card_state_base_show_create_edit_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 51;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_6a0c)?;
        c.g[S2][0] = c.g[V0][0];
        memcard_text(c, MEMCARD_TEXT_CREATE_EDIT, [0x001d_6a30, 0x001d_6a40])?;
        let s2 = c.g[S2][0];
        memcard_copy(c, s2, 0x001d_6a4c)?;
        memcard_set_message(c, MEMCARD_MSG_CREATE_EDIT);
        let s2 = c.g[S2][0];
        memcard_show_message(c, s2, 0, 0, 0, 0x001d_6a84)?;
        memcard_popup_v25(c, 1, 0x001d_6aa4)?;
        memcard_free_temp_string(c, frame, S2, 0x30, 0x001d_6ab4)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_OnLoadFailed(void)` (0x1d6ad0): the load failed; show
/// `kT_MEMOVLoadFailedPS2NGC` with the card's name (message 0xa, stored just before it
/// is shown) as a single "Continue" option.
pub fn mem_card_ui_on_load_failed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 76;
        // 0x000..0x200: the card name.
        let frame = Frame::open(c, 0x260);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x250);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S3, 0x220);
        frame.save(c, S4, 0x210);
        frame.save_ra(c, 0x200);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_6af8)?;
        c.g[S0][0] = c.g[V0][0];
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_6b14)?;
        c.g[S4][0] = c.g[V0][0];
        let (manager, sp) = (c.g[S0][0], c.g[SP][0]);
        memcard_card_name(c, manager, sp, 0x001d_6b28)?;
        memcard_text(c, MEMCARD_TEXT_LOAD_FAILED, [0x001d_6b48, 0x001d_6b58])?;
        let buf = c.g[S4][0];
        memcard_sprintf(c, buf, sp, 0x001d_6b68)?;
        memcard_set_message(c, MEMCARD_MSG_LOAD_FAILED);
        memcard_show_message(c, buf, 1, 0, 0, 0x001d_6ba0)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_CONTINUE,
            0,
            [0x001d_6bd4, 0x001d_6be4, 0x001d_6bf8],
        )?;
        memcard_free_temp_string(c, frame, S4, 0x250, 0x001d_6c08)?;
        c.cycles += 7;
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore(c, S3, 0x220);
        frame.restore(c, S4, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_OnSaveFailed(void)` (0x1d6c28): the save failed; show
/// `kT_MEMOVSaveFailed` with the card's name (message 0xb, stored before the
/// name is formatted) as a single "Continue" option.
pub fn mem_card_ui_on_save_failed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 77;
        // 0x000..0x200: the card name.
        let frame = Frame::open(c, 0x260);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x250);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S3, 0x220);
        frame.save(c, S4, 0x210);
        frame.save_ra(c, 0x200);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_6c50)?;
        c.g[S0][0] = c.g[V0][0];
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_6c6c)?;
        c.g[S4][0] = c.g[V0][0];
        memcard_set_message(c, MEMCARD_MSG_SAVE_FAILED);
        let (manager, sp) = (c.g[S0][0], c.g[SP][0]);
        memcard_card_name(c, manager, sp, 0x001d_6c8c)?;
        memcard_text(c, MEMCARD_TEXT_SAVE_FAILED, [0x001d_6cac, 0x001d_6cbc])?;
        let buf = c.g[S4][0];
        memcard_sprintf(c, buf, sp, 0x001d_6ccc)?;
        memcard_show_message(c, buf, 1, 0, 0, 0x001d_6cfc)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_CONTINUE,
            0,
            [0x001d_6d30, 0x001d_6d40, 0x001d_6d54],
        )?;
        memcard_free_temp_string(c, frame, S4, 0x250, 0x001d_6d64)?;
        c.cycles += 7;
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore(c, S3, 0x220);
        frame.restore(c, S4, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `fe->ShowMessage(buf, a2, 0, 0, 0, 0)` with the state in `t3`, the buffer
/// in `s4`, the vtable in `v1` and the method in `v0` (`a2` is the caller's).
pub(crate) fn memcard_show_s4_message(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A3][0] = 0;
    c.g[A1][0] = c.g[S4][0];
    c.g[V1][0] = Addr(r32(c, T3).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, T3).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V0), ret)
}

/// `void MemCardUI_OnCardNotDetected(void)` (0x1d6d88): no card in the slot.
/// Format `kT_MEMOVNotDetectedPS2` with the slot's name (message 9). If the
/// manager lets the player go on without a card, show it with a single
/// "Continue" option; otherwise as "Retry" / "Cancel".
pub fn mem_card_ui_on_card_not_detected(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        // 0x000..0x200: the card name.
        let frame = Frame::open(c, 0x260);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x250);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S4, 0x210);
        frame.save(c, S3, 0x220);
        frame.save_ra(c, 0x200);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_6db0)?;
        c.g[S2][0] = c.g[V0][0];
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_6dcc)?;
        c.g[S4][0] = c.g[V0][0];
        let s2 = c.g[S2][0];
        memcard_card_name(c, s2, c.g[SP][0], 0x001d_6de0)?;
        memcard_text(c, MEMCARD_TEXT_NOT_DETECTED, [0x001d_6e00, 0x001d_6e10])?;
        let (buf, sp) = (c.g[S4][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_6e20)?;
        // a0 (the state) and v0 (the id) stay live into the hook call.
        c.g[A0][0] = memcard_fe(c) as u64;
        c.g[V0][0] = MEMCARD_MSG_NOT_DETECTED;
        Addr(r32(c, A0).wrapping_add(MEMCARD_FE_MESSAGE)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, S2).wrapping_add(MEMCARD_MC_CONTINUE_A)).lw(c);
        let flag_a = c.g[V1][0] != 0;
        c.g[T3][0] = memcard_fe(c) as u64;
        let can_continue = if flag_a {
            c.cycles += 1;
            c.g[A2][0] = 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(MEMCARD_MC_CONTINUE_B)).lw(c);
            c.g[A2][0] = 1;
            if c.g[V0][0] != 0 {
                true
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(MEMCARD_MC_CAN_CONTINUE)).lw(c);
                call_ptr(c, r32(c, V0), 0x001d_6e50)?;
                let hook = c.g[V0][0] != 0;
                c.g[T3][0] = memcard_fe(c) as u64;
                if hook {
                    c.cycles += 1;
                    c.g[A2][0] = 1;
                }
                hook
            }
        };
        if can_continue {
            c.cycles += 32;
            memcard_show_s4_message(c, 0x001d_6f88)?;
            memcard_popup_option(
                c,
                V1,
                MEMCARD_TEXT_CONTINUE,
                0,
                [0x001d_6fbc, 0x001d_6fcc, 0x001d_6fe0],
            )?;
        } else {
            c.cycles += 65;
            c.g[A2][0] = 0;
            memcard_show_s4_message(c, 0x001d_6e84)?;
            memcard_popup_option(
                c,
                V1,
                MEMCARD_TEXT_RETRY,
                1,
                [0x001d_6eb8, 0x001d_6ec8, 0x001d_6edc],
            )?;
            memcard_popup_v25(c, 1, 0x001d_6efc)?;
            memcard_popup_option(
                c,
                A1,
                MEMCARD_TEXT_CANCEL,
                0,
                [0x001d_6f30, 0x001d_6f40, 0x001d_6f54],
            )?;
        }
        c.cycles += 2;
        memcard_free_temp_string(c, frame, S4, 0x250, 0x001d_6ff0)?;
        c.cycles += 7;
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore(c, S3, 0x220);
        frame.restore(c, S4, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_OnSaveCorrupt(void)` (0x1d7010): the card's save is
/// corrupt; show `kT_MEMOVGameCorruptPS2` formatted with the
/// card's name, as a single "Continue" option.
pub fn mem_card_ui_on_save_corrupt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 85;
        // 0x000..0x200: the card name; 0x200..0x400: the slot name.
        let frame = Frame::open(c, 0x460);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x450);
        frame.save(c, S1, 0x440);
        frame.save(c, S2, 0x430);
        frame.save(c, S3, 0x420);
        frame.save(c, S4, 0x410);
        frame.save_ra(c, 0x400);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_7038)?;
        c.g[S0][0] = c.g[V0][0];
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_7054)?;
        c.g[S4][0] = c.g[V0][0];
        let s0 = c.g[S0][0];
        memcard_card_name(c, s0, c.g[SP][0], 0x001d_7068)?;
        let slot_name = sx(r32(c, SP).wrapping_add(0x200));
        let s0 = r32(c, S0);
        memcard_slot_name(c, s0, slot_name, 0x001d_7088)?;
        // The state stays in a1 into the StrHash call.
        c.g[A1][0] = memcard_fe(c) as u64;
        memcard_set_message(c, MEMCARD_MSG_GAME_CORRUPT);
        memcard_text(c, MEMCARD_TEXT_GAME_CORRUPT, [0x001d_70b4, 0x001d_70c4])?;
        let (buf, sp) = (c.g[S4][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_70d4)?;
        let s4 = c.g[S4][0];
        memcard_show_message(c, s4, 1, 0, 1, 0x001d_7104)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_CONTINUE,
            0,
            [0x001d_7138, 0x001d_7148, 0x001d_715c],
        )?;
        memcard_free_temp_string(c, frame, S4, 0x450, 0x001d_716c)?;
        c.cycles += 7;
        frame.restore(c, S1, 0x440);
        frame.restore(c, S2, 0x430);
        frame.restore(c, S3, 0x420);
        frame.restore(c, S4, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_ShowCorruptSave(void)` (0x1d7190): the selected save is
/// corrupt; ask (Yes/No, No selected) with `kT_MEMCorruptSavePS2` and the
/// card's name, and set the popup's `v25` flag.
pub fn mem_card_ui_show_corrupt_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        let frame = Frame::open(c, 0x240);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x230);
        frame.save(c, S1, 0x220);
        frame.save(c, S2, 0x210);
        frame.save_ra(c, 0x200);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_71b0)?;
        c.g[S0][0] = c.g[V0][0];
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_71cc)?;
        c.g[S2][0] = c.g[V0][0];
        let s0 = c.g[S0][0];
        memcard_card_name(c, s0, c.g[SP][0], 0x001d_71e0)?;
        memcard_set_message(c, MEMCARD_MSG_CORRUPT_SAVE);
        // The application stays in a1 into the StrHash call.
        c.g[A1][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        memcard_text(c, MEMCARD_TEXT_CORRUPT_SAVE, [0x001d_720c, 0x001d_721c])?;
        let (buf, sp) = (c.g[S2][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_722c)?;
        let s2 = c.g[S2][0];
        memcard_show_message(c, s2, 0, 0, 1, 0x001d_725c)?;
        memcard_popup_v25(c, 1, 0x001d_727c)?;
        memcard_free_temp_string(c, frame, S2, 0x230, 0x001d_728c)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x220);
        frame.restore(c, S2, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// The start the format replies share: a message buffer in `s2`, the card
/// manager, and the current card's name at `sp`.
pub(crate) fn memcard_buffer_and_card_name(c: &mut Cpu, rets: [u32; 3]) -> Flow {
    memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, rets[0])?;
    c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
    c.g[S2][0] = c.g[V0][0];
    call_ptr(c, MEMCARD_MC_MANAGER_GET, rets[1])?;
    let v0 = c.g[V0][0];
    memcard_card_name(c, v0, c.g[SP][0], rets[2])
}

/// `void MemCardUI_ConfirmFormat(void)` (0x1d7318): the card is unformatted;
/// ask (Yes/No, No selected) with `kT_MEMFormatCardPS2` and the card's name,
/// and set the popup's `v25` flag.
pub fn mem_card_ui_confirm_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        let frame = Frame::open(c, 0x240);
        frame.save(c, S0, 0x230);
        frame.save(c, S1, 0x220);
        frame.save(c, S2, 0x210);
        frame.save_ra(c, 0x200);
        memcard_buffer_and_card_name(c, [0x001d_7344, 0x001d_7350, 0x001d_7360])?;
        memcard_text(c, MEMCARD_TEXT_FORMAT_CARD, [0x001d_7380, 0x001d_7390])?;
        let (buf, sp) = (c.g[S2][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_73a0)?;
        memcard_set_message(c, MEMCARD_MSG_CONFIRM_FORMAT);
        let s2 = c.g[S2][0];
        memcard_show_message(c, s2, 0, 0, 1, 0x001d_73d8)?;
        memcard_popup_v25(c, 1, 0x001d_73f8)?;
        memcard_free_temp_string(c, frame, S2, 0x230, 0x001d_7408)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x220);
        frame.restore(c, S2, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEMemCard_callbackConfirmFormatDone(void)` (`MemCardUI_OnFormatDone`,
/// 0x1d7420): the card is formatted; show `kT_MEMOVFormatComplete` as a single
/// "Continue" option.
pub fn mem_card_ui_on_format_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 66;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_744c)?;
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_7458)?;
        memcard_text(c, MEMCARD_TEXT_FORMAT_COMPLETE, [0x001d_7478, 0x001d_7488])?;
        let s2 = c.g[S2][0];
        memcard_copy(c, s2, 0x001d_7494)?;
        memcard_set_message(c, MEMCARD_MSG_FORMAT_DONE);
        let s2 = c.g[S2][0];
        memcard_show_message(c, s2, 1, 0, 0, 0x001d_74cc)?;
        memcard_text(c, MEMCARD_TEXT_CONTINUE, [0x001d_74ec, 0x001d_74fc])?;
        memcard_popup_set_option(c, 0, 0x001d_7520)?;
        memcard_free_temp_string(c, frame, S2, 0x30, 0x001d_7530)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_OnFormatFailed(void)` (0x1d7548): formatting failed; show
/// `kT_MEMFEFormatFailed` with the card's name and a single "Continue".
pub fn mem_card_ui_on_format_failed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 71;
        let frame = Frame::open(c, 0x240);
        frame.save(c, S0, 0x230);
        frame.save(c, S1, 0x220);
        frame.save(c, S2, 0x210);
        frame.save_ra(c, 0x200);
        memcard_buffer_and_card_name(c, [0x001d_7574, 0x001d_7580, 0x001d_7590])?;
        memcard_text(c, MEMCARD_TEXT_FORMAT_FAILED, [0x001d_75b0, 0x001d_75c0])?;
        let (buf, sp) = (c.g[S2][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_75d0)?;
        memcard_set_message(c, MEMCARD_MSG_FORMAT_FAILED);
        let s2 = c.g[S2][0];
        memcard_show_message(c, s2, 1, 0, 0, 0x001d_7608)?;
        memcard_text(c, MEMCARD_TEXT_CONTINUE, [0x001d_7628, 0x001d_7638])?;
        memcard_popup_set_option(c, 0, 0x001d_765c)?;
        memcard_free_temp_string(c, frame, S2, 0x230, 0x001d_766c)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x220);
        frame.restore(c, S2, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEMemCard_callbackConfirmDelete(void)` (`MemCardUI_ConfirmDelete`,
/// 0x1d7688): ask (Yes/No, No selected) whether to delete the selected save
/// (`kT_MEMConfirmDeletePS2` with its name, message 3), with the `v25` flag.
/// With no saves on the card, end the operation instead (state 4 clears the
/// busy words).
pub fn mem_card_ui_confirm_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        // 0x000..0x200: the slot name.
        let frame = Frame::open(c, 0x240);
        frame.save(c, S0, 0x230);
        frame.save(c, S1, 0x220);
        frame.save(c, S2, 0x210);
        frame.save_ra(c, 0x200);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_76a8)?;
        c.g[S0][0] = c.g[V0][0];
        // manager->card->SaveCount()
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_MC_CARD)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_CARD_VT_SAVE_COUNT)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_CARD_VT_SAVE_COUNT + 4)).lw(c);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, r32(c, V0), 0x001d_76c4)?;
        let no_saves = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0x0046_0000;
        if !no_saves {
            c.cycles += 55;
            memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_76e0)?;
            c.g[S2][0] = c.g[V0][0];
            memcard_set_message(c, MEMCARD_MSG_CONFIRM_DELETE);
            let (manager, sp) = (r32(c, S0), c.g[SP][0]);
            memcard_slot_name(c, manager, sp, 0x001d_7710)?;
            memcard_text(c, MEMCARD_TEXT_CONFIRM_DELETE, [0x001d_7730, 0x001d_7740])?;
            let (buf, sp) = (c.g[S2][0], c.g[SP][0]);
            memcard_sprintf(c, buf, sp, 0x001d_7750)?;
            memcard_show_message(c, buf, 0, 0, 1, 0x001d_7780)?;
            memcard_popup_v25(c, 1, 0x001d_77a0)?;
            let none = c.g[S2][0] == 0;
            frame.restore(c, S0, 0x230);
            if !none {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, MEMCARD_MEM_FREE, 0x001d_77b0)?;
                frame.restore(c, S0, 0x230);
            }
        } else {
            c.cycles += 5;
            c.g[A0][0] = memcard_fe(c) as u64;
            c.g[V1][0] = 4;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(MEMCARD_FE_STATE)).lw(c);
            let ending = c.g[V0][0] == c.g[V1][0];
            frame.restore(c, S0, 0x230);
            if ending {
                c.cycles += 2;
                Addr(r32(c, A0).wrapping_add(0x1e0)).sw(c, 0);
                Addr(r32(c, A0).wrapping_add(MEMCARD_FE_BUSY)).sw(c, 0);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x220);
        frame.restore(c, S2, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_OnCardNotDetected2(void)` (0x1d77e8): no card, with no
/// "continue without a card" check: show `kT_MEMOVNotDetectedPS2` with the
/// card's name (message 0x12) as a two-option popup labelled "Retry" (0) and
/// "Cancel" (1).
pub fn mem_card_ui_on_card_not_detected2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0x260);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x250);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S3, 0x220);
        frame.save(c, S4, 0x210);
        frame.save_ra(c, 0x200);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_7810)?;
        c.g[S0][0] = c.g[V0][0];
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_782c)?;
        c.g[S4][0] = c.g[V0][0];
        let (manager, sp) = (c.g[S0][0], c.g[SP][0]);
        memcard_card_name(c, manager, sp, 0x001d_7840)?;
        memcard_text(c, MEMCARD_TEXT_NOT_DETECTED, [0x001d_7860, 0x001d_7870])?;
        let (buf, sp) = (c.g[S4][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_7880)?;
        memcard_set_message(c, MEMCARD_MSG_NO_CARD);
        memcard_show_message(c, buf, 0, 0, 0, 0x001d_78b8)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_RETRY,
            0,
            [0x001d_78ec, 0x001d_78fc, 0x001d_7910],
        )?;
        memcard_popup_option(
            c,
            V1,
            MEMCARD_TEXT_CANCEL,
            1,
            [0x001d_7944, 0x001d_7954, 0x001d_7968],
        )?;
        memcard_free_temp_string(c, frame, S4, 0x250, 0x001d_7978)?;
        c.cycles += 7;
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore(c, S3, 0x220);
        frame.restore(c, S4, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// The "not enough space" text into `buf`: `kT_MEMFETooBigPS2` formatted with
/// the card's name (at `name`, register `s3` or `sp`) and the size a new save
/// needs (`manager->card->SaveSize()`, the manager in `s2`).
pub(crate) fn memcard_too_big_text(c: &mut Cpu, buf: u64, name: u64, rets: [u32; 4]) -> Flow {
    memcard_text(c, MEMCARD_TEXT_TOO_BIG, [rets[0], rets[1]])?;
    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(MEMCARD_MC_CARD)).lw(c);
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_CARD_VT_SAVE_SIZE)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_CARD_VT_SAVE_SIZE + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V0), rets[2])?;
    c.g[A3][0] = c.g[V0][0];
    c.g[A2][0] = name;
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = buf;
    call_ptr(c, MEMCARD_USTR_SPRINTF, rets[3])
}

/// `void MemCardUI_OnNotEnoughSpace(void)` (0x1d7998): the card is too full
/// for the save; say how much space it needs (message 0x13) with a single
/// "Continue" option.
pub fn mem_card_ui_on_not_enough_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 85;
        let frame = Frame::open(c, 0x260);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x250);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S3, 0x220);
        frame.save(c, S4, 0x210);
        frame.save_ra(c, 0x200);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_79c0)?;
        c.g[S2][0] = c.g[V0][0];
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_79dc)?;
        c.g[S4][0] = c.g[V0][0];
        memcard_set_message(c, MEMCARD_MSG_NOT_ENOUGH_SPACE);
        let (manager, sp) = (c.g[S2][0], c.g[SP][0]);
        memcard_card_name(c, manager, sp, 0x001d_79fc)?;
        let buf = c.g[S4][0];
        memcard_too_big_text(
            c,
            buf,
            sp,
            [0x001d_7a1c, 0x001d_7a2c, 0x001d_7a48, 0x001d_7a5c],
        )?;
        memcard_show_message(c, buf, 1, 3, 1, 0x001d_7a8c)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_CONTINUE,
            0,
            [0x001d_7ac0, 0x001d_7ad0, 0x001d_7ae4],
        )?;
        memcard_free_temp_string(c, frame, S4, 0x250, 0x001d_7af4)?;
        c.cycles += 7;
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore(c, S3, 0x220);
        frame.restore(c, S4, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_OnNotEnoughSpace2(void)` (0x1d7b18): the same message
/// built in a stack buffer instead of a "Temp String" (no free): the text at
/// `sp`, the card's name after it (in `s3`).
pub fn mem_card_ui_on_not_enough_space2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 83;
        let frame = Frame::open(c, 0x6b0);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x6a0);
        frame.save(c, S1, 0x690);
        frame.save(c, S2, 0x680);
        frame.save(c, S3, 0x670);
        frame.save_ra(c, 0x660);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_7b3c)?;
        memcard_set_message(c, MEMCARD_MSG_NOT_ENOUGH_SPACE);
        c.g[S2][0] = c.g[V0][0];
        c.g[S3][0] = sx(r32(c, SP).wrapping_add(MEMCARD_NAME_STRING_SIZE));
        let (manager, name) = (c.g[S2][0], c.g[S3][0]);
        memcard_card_name(c, manager, name, 0x001d_7b60)?;
        let sp = c.g[SP][0];
        memcard_too_big_text(
            c,
            sp,
            name,
            [0x001d_7b80, 0x001d_7b90, 0x001d_7bac, 0x001d_7bc0],
        )?;
        memcard_show_message(c, sp, 1, 3, 1, 0x001d_7bf0)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_CONTINUE,
            0,
            [0x001d_7c24, 0x001d_7c34, 0x001d_7c48],
        )?;
        frame.restore(c, S0, 0x6a0);
        frame.restore(c, S1, 0x690);
        frame.restore(c, S2, 0x680);
        frame.restore(c, S3, 0x670);
        frame.restore_ra(c, 0x660);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEMemCard_callbackFileExists(void)` (`MemCardUI_OnNameExists`,
/// 0x1d7c68): a save with that name exists; show `kT_FENameAlreadyExists`
/// as a single "Continue" option, and put the state back to 1.
pub fn mem_card_ui_on_name_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S2, 0x20);
        c.g[V0][0] = MEMCARD_MSG_NAME_EXISTS as u64;
        c.g[S2][0] = memcard_fe(c) as u64;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(MEMCARD_TEXT_NAME_EXISTS);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        // The text lookup, interleaved with the state's ShowMessage slot
        // (the state with its `this` adjustment in s2, its vtable in s3).
        Addr(r32(c, S2).wrapping_add(MEMCARD_FE_MESSAGE)).sw(c, c.g[V0][0]);
        c.g[S3][0] = Addr(r32(c, S2).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
        c.g[S0][0] = Addr(r32(c, A1).wrapping_add(MEMCARD_APP_TEXT_DB)).lw(c);
        c.g[V1][0] = Addr(r32(c, S3).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE)).lh(c);
        c.g[S1][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V1)));
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(MEMCARD_TEXT_DB_VT_FIND)).lh(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        call_ptr(c, STR_HASH, 0x001d_7cb8)?;
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(MEMCARD_TEXT_DB_VT_FIND + 4)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, r32(c, V1), 0x001d_7cc8)?;
        // fe->ShowMessage(text, 1, 0, 0, 0, 0)
        c.g[V1][0] = Addr(r32(c, S3).wrapping_add(MEMCARD_FE_VT_SHOW_MESSAGE + 4)).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = 1;
        call_ptr(c, r32(c, V1), 0x001d_7cec)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_CONTINUE,
            0,
            [0x001d_7d20, 0x001d_7d30, 0x001d_7d44],
        )?;
        c.g[V1][0] = memcard_fe(c) as u64;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V1).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_OnDifferentCard(void)` (0x1d7d70): the card was swapped;
/// show `kT_MEMDifferentMemcardPS2` with the card's name and a single
/// "Continue" option, and put the state back to 1.
pub fn mem_card_ui_on_different_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 77;
        let frame = Frame::open(c, 0x260);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x250);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S3, 0x220);
        frame.save(c, S4, 0x210);
        frame.save_ra(c, 0x200);
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_7d98)?;
        let (manager, sp) = (c.g[V0][0], c.g[SP][0]);
        memcard_card_name(c, manager, sp, 0x001d_7da8)?;
        memcard_new_temp_string(c, MEMCARD_NAME_STRING_SIZE as u64, 0x001d_7dc0)?;
        c.g[S4][0] = c.g[V0][0];
        memcard_text(c, MEMCARD_TEXT_DIFFERENT_CARD, [0x001d_7de4, 0x001d_7df4])?;
        let buf = c.g[S4][0];
        memcard_sprintf(c, buf, sp, 0x001d_7e04)?;
        memcard_set_message(c, MEMCARD_MSG_DIFFERENT_CARD);
        memcard_show_message(c, buf, 1, 0, 0, 0x001d_7e3c)?;
        memcard_popup_option(
            c,
            A1,
            MEMCARD_TEXT_CONTINUE,
            0,
            [0x001d_7e70, 0x001d_7e80, 0x001d_7e94],
        )?;
        c.g[V1][0] = memcard_fe(c) as u64;
        c.g[V0][0] = 1;
        let none = c.g[S4][0] == 0;
        Addr(r32(c, V1).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, MEMCARD_MEM_FREE, 0x001d_7eac)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x250);
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore(c, S3, 0x220);
        frame.restore(c, S4, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEMemCard_callbackDeleteDone(void)` (`MemCardUI_OnDeleteDone`,
/// 0x1d7ed0): the save was deleted. Unless the delete came from the pause menu,
/// show `kT_MEMDeleteSuccess` with a single "Continue"; a state of 4
/// goes back to 1. Returns 1.
pub fn mem_card_ui_on_delete_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = memcard_fe(c) as u64;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_LOAD_FROM_PAUSE)).lw(c);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = memcard_fe(c) as u64;
        } else {
            c.cycles += 59;
            memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, 0x001d_7f0c)?;
            c.g[S2][0] = c.g[V0][0];
            memcard_text(c, MEMCARD_TEXT_DELETE_DONE, [0x001d_7f30, 0x001d_7f40])?;
            let buf = c.g[S2][0];
            memcard_copy(c, buf, 0x001d_7f4c)?;
            memcard_set_message(c, MEMCARD_MSG_DELETE_DONE);
            memcard_show_message(c, buf, 1, 0, 0, 0x001d_7f84)?;
            memcard_text(c, MEMCARD_TEXT_CONTINUE, [0x001d_7fa4, 0x001d_7fb4])?;
            memcard_popup_set_option(c, 0, 0x001d_7fd8)?;
            let none = c.g[S2][0] == 0;
            c.g[A0][0] = memcard_fe(c) as u64;
            if !none {
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, MEMCARD_MEM_FREE, 0x001d_7fe8)?;
                c.g[A0][0] = memcard_fe(c) as u64;
            }
        }
        c.cycles += 4;
        c.g[V1][0] = 4;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(MEMCARD_FE_STATE)).lw(c);
        let back = c.g[V0][0] == c.g[V1][0];
        frame.restore(c, S0, 0x30);
        if back {
            c.cycles += 2;
            c.g[V0][0] = 1;
            Addr(r32(c, A0).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MemCardUI_OnDeleteFailed(void)` (0x1d8020): the delete failed; show
/// `kT_MEMOVDeleteFailedError` with the card's name and a single
/// "Continue" option. Returns 0.
pub fn mem_card_ui_on_delete_failed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 71;
        let frame = Frame::open(c, 0x240);
        frame.save(c, S0, 0x230);
        frame.save(c, S1, 0x220);
        frame.save(c, S2, 0x210);
        frame.save_ra(c, 0x200);
        memcard_buffer_and_card_name(c, [0x001d_804c, 0x001d_8058, 0x001d_8068])?;
        memcard_text(c, MEMCARD_TEXT_DELETE_FAILED, [0x001d_8088, 0x001d_8098])?;
        let (buf, sp) = (c.g[S2][0], c.g[SP][0]);
        memcard_sprintf(c, buf, sp, 0x001d_80a8)?;
        memcard_set_message(c, MEMCARD_MSG_DELETE_FAILED);
        memcard_show_message(c, buf, 1, 0, 0, 0x001d_80e0)?;
        memcard_text(c, MEMCARD_TEXT_CONTINUE, [0x001d_8100, 0x001d_8110])?;
        memcard_popup_set_option(c, 0, 0x001d_8134)?;
        memcard_free_temp_string(c, frame, S2, 0x230, 0x001d_8144)?;
        c.cycles += 6;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x220);
        frame.restore(c, S2, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_ResetAfterSave(MemCardStateBase* self)` (0x1d8c38):
/// back to state 1 with the `+0x1dc` and busy words cleared.
pub fn mem_card_state_base_reset_after_save(c: &mut Cpu) {
    c.cycles += 5;
    let state = r32(c, A0);
    c.g[V0][0] = 1;
    Addr(state.wrapping_add(0x1dc)).sw(c, 0);
    Addr(state.wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
    Addr(state.wrapping_add(MEMCARD_FE_BUSY)).sw(c, 0);
    ret(c);
}

/// Enter display mode `a1` (already set by the caller, and in register `mode`)
/// through the state's set-mode slot; the state is in `a2`.
pub(crate) fn memcard_enter_mode(c: &mut Cpu, mode: usize, ret: u32) -> Flow {
    Addr(r32(c, A2).wrapping_add(MEMCARD_FE_MODE)).sw(c, c.g[mode][0]);
    c.g[V0][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_SET_MODE)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_VT_SET_MODE + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// `box->v(+0x48)(1)` for the text box in `s3` (after its text is set).
pub(crate) fn memcard_box_show(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_UI_VT_SHOW)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_UI_VT_SHOW + 4)).lw(c);
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// The tail `MemCardUI_OnNoCards` and `MemCardUI_CheckCardsPresent` share
/// (with its own cycle charges): put `kT_MEMNoCardsPS2` into the screen's
/// "text box message" object, if it has one, and show it. `s0` is restored
/// here; `rets` are the nine calls' returns.
pub(crate) fn memcard_no_cards_box(c: &mut Cpu, frame: Frame, rets: [u32; 9]) -> Flow {
    c.cycles += 10;
    c.g[A0][0] = sx(MEMCARD_TEXT_BOX_NAME);
    c.g[S0][0] = memcard_fe(c) as u64;
    call_ptr(c, STR_HASH, rets[0])?;
    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_2, rets[1])?;
    c.g[S3][0] = c.g[V0][0];
    let found = c.g[S3][0] != 0;
    c.g[A1][0] = 0x0046_0000;
    if found {
        c.cycles += 32;
        memcard_new_temp_string(c, MEMCARD_TEMP_STRING_SIZE, rets[2])?;
        c.g[S2][0] = c.g[V0][0];
        memcard_text(c, MEMCARD_TEXT_NO_CARDS, [rets[3], rets[4]])?;
        let buf = c.g[S2][0];
        memcard_copy(c, buf, rets[5])?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = buf;
        call_ptr(c, MEMCARD_UI_TEXT_SET_WRAPPED, rets[6])?;
        memcard_box_show(c, rets[7])?;
        let none = c.g[S2][0] == 0;
        frame.restore(c, S0, 0x40);
        if none {
            return Ok(());
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, MEMCARD_MEM_FREE, rets[8])?;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// `void MemCardUI_OnNoCards(void)` (0x1d8240): no memory card at all. Back
/// to state 1 and the menu mode, and put `kT_MEMNoCardsPS2` in the text box.
pub fn mem_card_ui_on_no_cards(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = memcard_fe(c) as u64;
        c.g[V1][0] = 1;
        Addr(r32(c, A2).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_MODE)).lw(c);
        let in_menu = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = MEMCARD_MODE_MENU;
        if !in_menu {
            c.cycles += 6;
            memcard_enter_mode(c, V1, 0x001d_8288)?;
        }
        memcard_no_cards_box(
            c,
            frame,
            [
                0x001d_8298,
                0x001d_82a4,
                0x001d_82c4,
                0x001d_82e8,
                0x001d_82f8,
                0x001d_8304,
                0x001d_8310,
                0x001d_8328,
                0x001d_8338,
            ],
        )?;
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_CheckCardsPresent(void)` (0x1d83a8): poll the cards.
/// Back to state 1, ask the manager's state machine for state 2; with a card
/// in, enter the file mode (3), otherwise the menu mode and the "no cards"
/// text box.
pub fn mem_card_ui_check_cards_present(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;
        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_83cc)?;
        // manager->state_machine->SetState(2)
        c.g[V1][0] = memcard_fe(c) as u64;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        Addr(r32(c, V1).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[S1][0]);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_MC_STATE_MACHINE)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, r32(c, V1), 0x001d_83f0)?;
        // manager->card->AnyCard()
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_MC_CARD)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_CARD_VT_ANY_CARD)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_CARD_VT_ANY_CARD + 4)).lw(c);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, r32(c, V0), 0x001d_8408)?;
        let any_card = c.g[V0][0] != 0;
        c.g[A2][0] = memcard_fe(c) as u64;
        if any_card {
            c.cycles += 4;
            c.g[V1][0] = MEMCARD_MODE_FILES;
            c.g[V0][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_MODE)).lw(c);
            let in_files = c.g[V0][0] == c.g[V1][0];
            c.g[A1][0] = MEMCARD_MODE_FILES;
            if in_files {
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
            } else {
                c.cycles += 8;
                memcard_enter_mode(c, V1, 0x001d_8438)?;
                frame.restore(c, S0, 0x40);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_MODE)).lw(c);
            let in_menu = c.g[V0][0] == c.g[S1][0];
            c.g[A1][0] = MEMCARD_MODE_MENU;
            if !in_menu {
                c.cycles += 6;
                memcard_enter_mode(c, S1, 0x001d_8464)?;
            }
            memcard_no_cards_box(
                c,
                frame,
                [
                    0x001d_8474,
                    0x001d_8480,
                    0x001d_84a0,
                    0x001d_84c4,
                    0x001d_84d4,
                    0x001d_84e0,
                    0x001d_84ec,
                    0x001d_8504,
                    0x001d_8514,
                ],
            )?;
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

/// `void MemCardStateBase_ShowMessage(MemCardStateBase* self, const u16* text,
/// int single, int a3, int t0, int selected, int t2)` (0x1d8530): show `text`
/// in the state's popup. An existing popup is re-initialised with the same
/// arguments; otherwise a `cFEPopupConfirm` is created and pushed on the UI
/// state manager. Clears `+0x210` unless the message is the status one, and
/// sets the popup's `v25` flag for message 1.
pub fn mem_card_state_base_show_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = memcard_fe(c) as u64;
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        // self, text, single, a3, t0, selected, t2.
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A3][0];
        c.g[S6][0] = c.g[T0][0];
        c.g[S1][0] = c.g[T1][0];
        c.g[S5][0] = c.g[T2][0];
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
        c.g[V0][0] = MEMCARD_MSG_STATUS;
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 1;
            Addr(r32(c, S0).wrapping_add(0x210)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[T2][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_POPUP)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        if c.g[T2][0] != 0 {
            c.cycles += 12;
            c.g[V0][0] = Addr(r32(c, T2).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_REINIT)).lh(c);
            c.g[T0][0] = c.g[S6][0];
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_REINIT + 4)).lw(c);
            c.g[T1][0] = c.g[S1][0];
            c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
            c.g[T2][0] = c.g[S5][0];
            call_ptr(c, r32(c, V1), 0x001d_85b8)?;
        } else {
            c.cycles += 22;
            c.g[A1][0] = sx(MEMCARD_POPUP_TAG);
            c.g[A0][0] = MEMCARD_POPUP_SIZE;
            c.g[A2][0] = MEMCARD_TEMP_STRING_FLAGS;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED_2, 0x001d_85d8)?;
            // new cFEPopupConfirm(engine, fe, text, single, a3, t0, selected, t2)
            c.g[A2][0] = memcard_fe(c) as u64;
            c.g[A0][0] = c.g[V0][0];
            c.g[T3][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_ENGINE)).lw(c);
            c.g[T0][0] = c.g[S3][0];
            Addr(r32(c, SP)).sw(c, c.g[S5][0]);
            c.g[T1][0] = c.g[S4][0];
            c.g[T2][0] = c.g[S6][0];
            MEMCARD_POPUP_CTOR.call(c, 0x001d_8600)?;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_ENGINE)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_POPUP)).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
            call_ptr(c, MEMCARD_UI_PUSH_POPUP, 0x001d_8614)?;
        }
        c.g[V0][0] = memcard_fe(c) as u64;
        c.cycles += 4;
        c.g[A0][0] = 1;
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 8;
            c.g[A2][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_POPUP)).lw(c);
            c.g[A1][0] = 1;
            c.g[V0][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_V25)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_POPUP_VT_V25 + 4)).lw(c);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, r32(c, V1), 0x001d_8644)?;
        }
        frame.restore(c, S0, 0x80);
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Rebuild the popup in `s1` if it is live; `s0` is restored from `s0_slot`
/// on both paths at the point the original does it. `live` is the `+0x40`
/// word already in `v0`.
pub(crate) fn popup_rebuild_if_live(c: &mut Cpu, frame: Frame, s0_slot: u32, ret: u32) -> Flow {
    let live = c.g[V0][0] != 0;
    frame.restore(c, S0, s0_slot);
    if live {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, POPUP_REBUILD, ret)?;
        frame.restore(c, S0, s0_slot);
    }
    Ok(())
}

/// `void cFEPopupConfirm_InitButtons(cFEPopupConfirm* self, const u16* text,
/// int single, int none, int t0, int selected)` (0x1d9a80): Yes/No options,
/// the message text (or the empty default), and the option count: 0 with
/// `none` (the first option's flag cleared, option 1 selected), 1 with
/// `single`, else 2 with `selected`. Rebuilds a live popup.
pub fn c_fe_popup_confirm_init_buttons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[T0][0];
        c.g[S4][0] = c.g[T1][0];
        c.g[S3][0] = c.g[A1][0];
        c.g[S6][0] = c.g[A3][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[A0][0] = sx(POPUP_TEXT_YES);
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(POPUP_OPTIONS));
        call_ptr(c, STR_HASH, 0x001d_9acc)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, POPUP_SET_OPTION_TEXT_ID, 0x001d_9adc)?;
        c.g[A0][0] = sx(POPUP_TEXT_NO);
        call_ptr(c, STR_HASH, 0x001d_9ae8)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        call_ptr(c, POPUP_SET_OPTION_TEXT_ID, 0x001d_9af8)?;
        let this = r32(c, S1);
        c.g[V0][0] = 2;
        Addr(this.wrapping_add(0x14c)).sw(c, c.g[S0][0]);
        Addr(this.wrapping_add(POPUP_OPTIONS)).sw(c, c.g[V0][0]);
        Addr(this.wrapping_add(0x168)).sw(c, 0);
        Addr(this.wrapping_add(0x154)).sw(c, 0);
        Addr(this.wrapping_add(0x164)).sw(c, 0);
        // The message text: `text`, or the empty default.
        if c.g[S3][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = sx(this.wrapping_add(POPUP_TEXT));
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[V0][0];
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, MEMCARD_USTR_COPY, 0x001d_9b28)?;
        } else {
            c.cycles += 6;
            c.g[S0][0] = sx(this.wrapping_add(POPUP_TEXT));
            c.g[A1][0] = sx(POPUP_EMPTY_TEXT);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, POPUP_USTR_FROM_ASCII, 0x001d_9b44)?;
        }
        c.g[A1][0] = c.g[S3][0];
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, MEMCARD_USTR_COPY, 0x001d_9b50)?;
        // The option count and the selection.
        let options = r32(c, S2);
        let this = r32(c, S1);
        c.g[V0][0] = 1;
        if c.g[S6][0] != 0 {
            c.cycles += 4;
            Addr(this.wrapping_add(POPUP_OPTION_COUNT)).sw(c, 0);
            Addr(options.wrapping_add(0x98)).sw(c, c.g[V0][0]);
            Addr(options).sw(c, 0);
        } else {
            c.cycles += 2;
            if c.g[S5][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 1;
                Addr(this.wrapping_add(POPUP_OPTION_COUNT)).sw(c, c.g[V0][0]);
            } else {
                c.cycles += 3;
                Addr(this.wrapping_add(POPUP_OPTION_COUNT)).sw(c, c.g[V0][0]);
                Addr(options).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            Addr(options.wrapping_add(0x98)).sw(c, c.g[S4][0]);
        }
        c.cycles += 6;
        c.g[V1][0] = 1;
        Addr(options.wrapping_add(8)).sw(c, c.g[S0][0]);
        Addr(options.wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(this.wrapping_add(POPUP_LIVE)).lw(c);
        popup_rebuild_if_live(c, frame, 0x70, 0x001d_9ba4)?;
        c.cycles += 9;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Copy the text in `a1` into the popup's option text `i` (register `s0`
/// points at it) and set option `i` to it.
pub(crate) fn popup_set_option_text(c: &mut Cpu, i: usize, rets: [u32; 2]) -> Flow {
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(POPUP_OPTION_TEXT[i]));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, MEMCARD_USTR_COPY, rets[0])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = i as u64;
    call_ptr(c, POPUP_SET_OPTION_STRING, rets[1])
}

/// `void cFEPopupConfirm_v26(cFEPopupConfirm* self, const u16* text,
/// int option)` (0x1d9bd0): set the text of option 0, 1 (only when the popup
/// has two options) or 2 (which makes it a three-option popup), and rebuild a
/// live popup.
pub fn c_fe_popup_confirm_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(POPUP_OPTIONS));
        if c.g[A2][0] == 0 {
            c.cycles += 9;
            popup_set_option_text(c, 0, [0x001d_9bfc, 0x001d_9c0c])?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let first = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if first {
                c.cycles += 4;
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(POPUP_OPTION_COUNT)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 9;
                    popup_set_option_text(c, 1, [0x001d_9c3c, 0x001d_9c4c])?;
                }
            } else {
                c.cycles += 10;
                let this = r32(c, S1);
                Addr(this.wrapping_add(POPUP_OPTIONS)).sw(c, c.g[V0][0]);
                Addr(this.wrapping_add(POPUP_OPTION_COUNT)).sw(c, c.g[V0][0]);
                popup_set_option_text(c, 2, [0x001d_9c68, 0x001d_9c78])?;
            }
        }
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(POPUP_LIVE)).lw(c);
        c.cycles += 2;
        popup_rebuild_if_live(c, frame, 0x30, 0x001d_9c8c)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup* cFEStateNetHelpPopup_ctor(self, engine, owner)`
/// (0x1d9d48): the UI state base, the vtable, seven empty button slots (the
/// compiler's empty array-constructor loop), the two counters at `+0x4c`, the
/// slot size 0x4c at `+0xc`, then `ClearFields`. Returns `self`.
pub fn c_fe_state_net_help_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, UI_STATE_CTOR_2, 0x001d_9d5c)?;
        c.g[V0][0] = sx(NET_HELP_POPUP_VTABLE);
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 6;
        Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = 0x4c;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_9d70);
            }
        }
        c.cycles += 10;
        let this = r32(c, S0);
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        Addr(this.wrapping_add(0x50)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(this.wrapping_add(0xc)).sw(c, c.g[V0][0]);
        NET_HELP_POPUP_CLEAR_FIELDS.call(c, 0x001d_9da0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Show or hide the screen object named by the string at `name`
/// (`v(+0x38)(visible)`) if the screen (the state in `s4`) has it.
pub(crate) fn memcard_object_visible(c: &mut Cpu, name: u32, visible: u64, rets: [u32; 3]) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, STR_HASH, rets[0])?;
    c.g[A0][0] = Addr(r32(c, S4).wrapping_add(MEMCARD_FE_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_2, rets[1])?;
    c.g[A2][0] = c.g[V0][0];
    let found = c.g[A2][0] != 0;
    c.g[A1][0] = visible;
    if found {
        c.cycles += 5;
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_UI_VT_SET_VISIBLE)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_UI_VT_SET_VISIBLE + 4)).lw(c);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, r32(c, V1), rets[2])?;
    }
    Ok(())
}

/// The screen's value named by the string at `name`, into `s5`.
pub(crate) fn memcard_screen_value(c: &mut Cpu, name: u32, rets: [u32; 2]) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, STR_HASH, rets[0])?;
    c.g[A0][0] = Addr(r32(c, S4).wrapping_add(MEMCARD_FE_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, MEMCARD_UI_SCREEN_LOOKUP_VALUE, rets[1])?;
    c.g[S5][0] = c.g[V0][0];
    Ok(())
}

/// The screen's "text box message" object, into `s3`; true when there is one.
pub(crate) fn memcard_find_text_box(c: &mut Cpu, rets: [u32; 2]) -> Result<bool, Unwound> {
    c.g[A0][0] = sx(MEMCARD_TEXT_BOX_NAME);
    call_ptr(c, STR_HASH, rets[0])?;
    c.g[A0][0] = Addr(r32(c, S4).wrapping_add(MEMCARD_FE_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_2, rets[1])?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A3][0] = 0;
    Ok(c.g[S3][0] != 0)
}

/// `void cFEMemCard_triggerDisplayState(MemCardStateBase* self, int mode)`
/// (`MemCardStateBase_SetMode`, 0x1d9470): store the mode and lay the screen
/// out for it: 1 (menu) shows "offmenu" and switches the screen to its "off"
/// thread; 2 (file display) shows "offmenu" and switches to "file display";
/// 3 (files) shows "onmenu" and switches to "on". Modes 1 and 2 also put
/// `kT_MEMFECheckingCardPS2` with the card's name into the text box.
pub fn mem_card_state_base_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        // 0x000..0x200: the card name.
        let frame = Frame::open(c, 0x270);
        frame.save(c, S0, 0x260);
        frame.save(c, S1, 0x250);
        frame.save(c, S2, 0x240);
        frame.save(c, S3, 0x230);
        frame.save(c, S4, 0x220);
        frame.save(c, S5, 0x210);
        frame.save_ra(c, 0x200);
        c.g[V0][0] = 2;
        c.g[S4][0] = c.g[A0][0];
        c.g[S5][0] = MEMCARD_NO_VALUE;
        let mode = c.g[A1][0];
        Addr(r32(c, S4).wrapping_add(MEMCARD_FE_MODE)).sw(c, mode);
        // Whether the "checking card" text is due (the box is in s3, the
        // buffer in s2 and FormatSlotName's arguments are set).
        let checking = if mode == 2 {
            c.cycles += 12;
            c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
            call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_95bc)?;
            c.g[S0][0] = c.g[V0][0];
            memcard_object_visible(
                c,
                MEMCARD_NAME_ON_MENU,
                0,
                [0x001d_95cc, 0x001d_95d8, 0x001d_95f8],
            )?;
            c.cycles += 9;
            memcard_object_visible(
                c,
                MEMCARD_NAME_OFF_MENU,
                1,
                [0x001d_9604, 0x001d_9610, 0x001d_9630],
            )?;
            c.cycles += 16;
            memcard_screen_value(c, MEMCARD_NAME_FILE_DISPLAY, [0x001d_963c, 0x001d_9648])?;
            if memcard_find_text_box(c, [0x001d_9658, 0x001d_9664])? {
                c.cycles += 9;
                memcard_new_temp_string(c, MEMCARD_NAME_STRING_SIZE as u64, 0x001d_9684)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
                c.g[A2][0] = Addr(r32(c, A0).wrapping_add(MEMCARD_MC_CARD_SLOT)).lw(c);
                c.g[A1][0] = c.g[SP][0];
                true
            } else {
                c.cycles += 1;
                false
            }
        } else {
            c.cycles += 3;
            if (mode as i64) < 3 {
                c.cycles += 2;
                c.g[V0][0] = MEMCARD_NO_VALUE;
                if mode == 1 {
                    c.cycles += 9;
                    memcard_object_visible(
                        c,
                        MEMCARD_NAME_ON_MENU,
                        0,
                        [0x001d_94e0, 0x001d_94ec, 0x001d_950c],
                    )?;
                    c.cycles += 9;
                    memcard_object_visible(
                        c,
                        MEMCARD_NAME_OFF_MENU,
                        1,
                        [0x001d_9518, 0x001d_9524, 0x001d_9544],
                    )?;
                    c.cycles += 16;
                    memcard_screen_value(c, MEMCARD_NAME_OFF, [0x001d_9550, 0x001d_955c])?;
                    if memcard_find_text_box(c, [0x001d_956c, 0x001d_9578])? {
                        c.cycles += 12;
                        memcard_new_temp_string(c, MEMCARD_NAME_STRING_SIZE as u64, 0x001d_9598)?;
                        c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
                        c.g[S2][0] = c.g[V0][0];
                        call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_95a4)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = Addr(r32(c, A0).wrapping_add(MEMCARD_MC_CARD_SLOT)).lw(c);
                        true
                    } else {
                        c.cycles += 1;
                        false
                    }
                } else {
                    c.cycles += 2;
                    false
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = MEMCARD_NO_VALUE;
                if mode == 3 {
                    c.cycles += 9;
                    memcard_object_visible(
                        c,
                        MEMCARD_NAME_ON_MENU,
                        1,
                        [0x001d_9724, 0x001d_9730, 0x001d_9750],
                    )?;
                    c.cycles += 9;
                    memcard_object_visible(
                        c,
                        MEMCARD_NAME_OFF_MENU,
                        0,
                        [0x001d_975c, 0x001d_9768, 0x001d_9788],
                    )?;
                    c.cycles += 7;
                    memcard_screen_value(c, MEMCARD_NAME_ON, [0x001d_9794, 0x001d_97a0])?;
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                }
                false
            }
        };
        if checking {
            c.cycles += 29;
            call_ptr(c, MEMCARD_FORMAT_SLOT_NAME, 0x001d_969c)?;
            memcard_text(c, MEMCARD_TEXT_CHECKING_CARD, [0x001d_96bc, 0x001d_96cc])?;
            let (buf, sp) = (c.g[S2][0], c.g[SP][0]);
            memcard_sprintf(c, buf, sp, 0x001d_96dc)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = buf;
            call_ptr(c, MEMCARD_UI_TEXT_SET_WRAPPED, 0x001d_96e8)?;
            memcard_box_show(c, 0x001d_9700)?;
            c.g[V0][0] = MEMCARD_NO_VALUE;
            if c.g[S2][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, MEMCARD_MEM_FREE, 0x001d_9710)?;
            }
        }
        c.g[V0][0] = MEMCARD_NO_VALUE;
        // Switch the screen's thread when the mode has one.
        c.cycles += 2;
        let none = c.g[S5][0] == c.g[V0][0];
        frame.restore(c, S0, 0x260);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = Addr(r32(c, S4).wrapping_add(MEMCARD_FE_SCREEN)).lw(c);
            c.g[A1][0] = c.g[S5][0] & 0xffff;
            call_ptr(c, MEMCARD_UI_SCREEN_REPLACE_THREAD, 0x001d_97bc)?;
            frame.restore(c, S0, 0x260);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x250);
        frame.restore(c, S2, 0x240);
        frame.restore(c, S3, 0x230);
        frame.restore(c, S4, 0x220);
        frame.restore(c, S5, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::DATA;

    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;

    pub(crate) use crate::game::movie::movieplayer::tests::w;

    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;

    // ------------------------------------------------------------ front end II

    #[test]
    fn c_fe_popup_confirm_rebuild_matches() {
        check(
            ee::s03::f_001c5dd8__generated,
            c_fe_popup_confirm_rebuild,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1c5d_d8, 6);
                let self_ = DATA;
                let child = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0x2bc, (seed % 2) as u32);
                w(c, self_ + 0x8, child);
                ret_slot(c, child, 0x30, RET_ZERO);
                let _ = vtable;
                w(c, self_ + 0x278, 0x1122_3344);
                w(c, self_ + 0x27c, 0x5566_7788);
                w(c, self_ + 0x280, 0x99aa_bbcc);
                w(c, self_ + 0xbc, 0);
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3;
    pub(crate) use crate::game::bx::bxstringctor::tests::slot_2;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2 as world;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2 as check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_3 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::screen_with;
    pub(crate) use crate::game::fe::feasyncfile::tests::str_hash;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::fememcard::tests::vtable_at;
    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::fe::festateoptions::tests::setup as setup_2;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_m2;
    use ssx3_ee as ee;

    #[test]
    fn c_fe_popup_confirm_v25_matches() {
        check(
            ssx3_ee::s04::f_001d9d18__generated,
            c_fe_popup_confirm_v25,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xab, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed as u64) << 4 | 3;
                let live = (seed % 2) as u32;
                c.bus.write32(DATA + 0x40, live);
                // `cFEPopupConfirm_ResetLayout` (+0x2bc) is not up: quick return.
                c.bus.write32(DATA + 0x2bc, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_step_slot_dir_matches() {
        check(
            ee::s04::f_001d97e0__generated,
            mem_card_state_base_step_slot_dir,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d97_e0, 0);
                let list = DATA + 0x400;
                zero(c, list, 0x40);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx((seed % 6) as u32);
                c.g[A3][0] = sx((seed & 1) as u32);
                c.bus.write32(list + 0x424, (seed % 6) + 1);
            },
        );
    }

    #[test]
    fn mem_card_state_base_set_mode_menu_matches() {
        check(
            ssx3_ee::s04::f_001d9368__generated,
            mem_card_state_base_set_mode_menu,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26, 1);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x214, if seed % 2 == 0 { 0 } else { 1 });
                vtable_at(c, &mut r, DATA, 0x118);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d23e0 cFEStateVideoPlayer_Init.

    // -----------------------------------------------------------------------
    // 0x1d8358 MemCardUI_ShowFileDisplay.

    #[test]
    fn mem_card_ui_show_file_display_matches() {
        check(
            ee::s04::f_001d8358__generated,
            mem_card_ui_show_file_display,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d83_58, 1);
                let fe = DATA + 0x100;
                c.bus.write32(GAME_GP.wrapping_sub(0x10c8), fe);
                match seed % 3 {
                    0 => c.bus.write32(fe + 0x214, 2), // already in mode 2
                    1 => c.bus.write32(fe + 0x1c0, 1), // a save is in progress
                    _ => {
                        c.bus.write32(fe + 0x214, 0);
                        c.bus.write32(fe + 0x1c0, 0);
                        let vt = vtable(c, &mut r, &[0x118]);
                        c.bus.write32(fe + 8, vt);
                    }
                }
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d86b0 MemCardStateBase_v45.

    #[test]
    fn mem_card_state_base_v45_matches() {
        check(
            ee::s04::f_001d86b0__generated,
            mem_card_state_base_v45,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d86_b0, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let vt = vtable(c, &mut r, &[0x178]);
                c.bus.write32(this + 8, vt);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x2000);
            },
        );
    }

    #[test]
    fn mem_card_state_base_reset_state_matches() {
        check(
            ee::s04::f_001d8be0__generated,
            mem_card_state_base_reset_state,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1d8be0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn mem_card_state_base_set_use_save_title_matches() {
        check(
            ssx3_ee::s04::f_001d9308__generated,
            mem_card_state_base_set_use_save_title,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1d9308);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The overlay manager and its movie player.
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x700);
                c.bus.write32(DATA + 0x700 + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x70]);
                c.bus.write32(DATA + 0x900, vt);
                c.bus.write32(DATA + 0x700 + 0xf8, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_state_base_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001d9818__generated,
            mem_card_state_base_on_activate,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d9818);
                c.g[A0][0] = DATA as u64;
                // No UI engine: the by-hash lookup returns null and takes the
                // leave-the-widget-alone path.
                c.bus.write32(DATA + 0x40, 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // Memory-card and overlay states.

    #[test]
    fn mem_card_state_base_v46_matches() {
        check(
            ee::s04::f_001d8670__generated,
            mem_card_state_base_v46,
            200,
            &|c, seed| {
                let mut r = setup_2(c, seed, 31);
                gpf(c, -0x10c8, AUX);
                put(
                    c,
                    AUX + 0x1c0,
                    if seed % 2 == 0 { 4 } else { r.next_u32() % 8 },
                );
                put(c, AUX + 0x1a8, r.next_u32());
                put(c, AUX + 0x1e0, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_state_base_transition_out_matches() {
        check(
            ee::s04::f_001d93d8__generated,
            mem_card_state_base_transition_out,
            200,
            &|c, seed| {
                setup_2(c, seed, 32);
                c.g[A0][0] = AUX as u64;
                put(c, AUX + 0x1c0, [6u32, 5, 7, 0x100][(seed % 4) as usize]);
            },
        );
    }

    // -----------------------------------------------------------------------

    #[test]
    fn memcard_ui_on_load_aborted_matches() {
        check(
            ee::s04::f_001d72a8__generated,
            memcard_ui_on_load_aborted,
            60,
            &|c, seed| {
                base(c, seed, 0x1d72_a8);
                let fe = DATA + 0x1000;
                let vt = DATA + 0x1200;
                c.bus.write32(GAME_GP - 0x10c8, fe);
                c.bus.write32(fe + 8, vt);
                put_vtable(c, vt, &[0x148]);
                let mc = DATA + 0x1400;
                let vt2 = DATA + 0x1600;
                c.bus.write32(GAME_GP - 0x47c, mc);
                c.bus.write32(mc + 0x748, vt2);
                put_vtable(c, vt2, &[8]);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v27_matches() {
        check(
            ssx3_ee::s04::f_001d9ca8__generated,
            c_fe_popup_confirm_v27,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d9ca8, 0);
                let s = DATA + 0x1000;
                let src = DATA + 0x2000;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { src as u64 };
                // A short UTF-16 string.
                w16(c, src, if seed % 3 == 0 { 0 } else { 0x41 });
                w16(c, src + 2, 0x42);
                w16(c, src + 4, 0);
                w32(c, s + 0x40, if seed % 2 == 0 { 0 } else { 1 });
                w32(c, s + 0x2bc, 0); // ResetLayout path avoided
            },
        );
    }

    #[test]
    fn mem_card_state_base_show_status_matches() {
        check(
            ee::s04::f_001d8d68__generated,
            mem_card_state_base_show_status,
            12,
            &|c, seed| {
                machine(c, seed, 0x1d_8d68, 3);
                belib(c);
                world(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w_m2(c, DATA + 0x2000 + 0x8, DATA + 0x2400);
                slot(c, DATA + 0x2400, 0x120, 0, FAKE);
                c.pc = 0x001d_8d68;
            },
        );
    }

    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    #[test]
    fn mem_card_state_base_alloc_read_buffer_matches() {
        check(
            ee::s04::f_001d9948__generated,
            mem_card_state_base_alloc_read_buffer,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1d99_48, 4);
                let self_ = DATA;
                w_m2(c, self_ + 0x228, (seed % 2) as u32);
                let old = if seed % 3 == 0 { 0 } else { alloc(c, 0x100) };
                w_m2(c, self_ + 0x1d4, old);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = 0x100;
            },
        );
    }

    #[test]
    fn mem_card_state_base_play_widget_anim_matches() {
        check(
            ee::s04::f_001d93e8__generated,
            mem_card_state_base_play_widget_anim,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d93e8, 2);
                put_str(c, DATA + 0x400, b"widget");
                let h = str_hash(c, DATA + 0x400);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                // The animation set at_3 `*(self+0x10)+0x50` has no entries, so
                // CreateAnim answers 0.
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2050, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }

    /// A screen at_m2 `screen` whose object list holds `node` for the string at_m2
    /// `name` (the zeroed game string). `+0x38` points at_m2 the object count and
    /// `+0x3c` at_m2 the object array, as `cUIScreen_getObjectByHashName` reads
    /// them; `node+0x38` is that string's hash.
    pub(crate) fn hash_screen(c: &mut Cpu, screen: u32, node: u32, name: u32) {
        let h = str_hash(c, name);
        let list = DATA + 0x5_6000;
        let count = DATA + 0x5_6010;
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(screen + 0x3c, list);
        c.bus.write32(count, 1);
        c.bus.write32(list, node);
        c.bus.write32(node + 0x38, h);
    }

    #[test]
    fn memcard_state_base_sync_menu_selection_matches() {
        check(
            ee::s04::f_001d9258__generated,
            memcard_state_base_sync_menu_selection,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1d92);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x40, if seed % 6 == 0 { 0 } else { screen });
                hash_screen(c, screen, node, 0x004a_1600);
                let item = DATA + 0x3000;
                let variant = seed % 4;
                c.bus
                    .write32(node + 0xa0, if variant == 0 { 0 } else { item });
                c.bus.write8(node + 0x95, (seed % 5) as u8);
                c.bus.write32(item + 0x14, variant << 5);
                c.bus.write32(node + 8, DATA + 0x4000);
                slot_2(c, DATA + 0x4000, 0x48, 1);
                fill(c, &mut r, DATA + 0x1c4, 0x10);
            },
        );
    }

    // ------------------------------------------------------------ more screens

    #[test]
    fn cfe_popup_confirm_ctor_matches() {
        check(
            ee::s04::f_001d99c8__generated,
            cfe_popup_confirm_ctor,
            6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x001d_99c8, 2);
                w_m2(c, GAME_GP.wrapping_sub(0x10b4), (seed % 2) as u32);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = OBJ3 as u64;
                c.g[A3][0] = OBJ4 as u64;
                c.g[T0][0] = (seed % 5) as u64;
                c.g[T1][0] = (seed % 7) as u64;
                c.g[T2][0] = (seed % 9) as u64;
                c.g[T3][0] = (seed % 11) as u64;
                w_m2(c, OBJ + 0x17c, 0);
                w_m2(c, OBJ + 0x14c, 0);
            },
        );
    }

    #[test]
    fn memcard_state_base_is_card_ready_matches() {
        strict(
            ee::s04::f_001d9880__generated,
            memcard_state_base_is_card_ready,
            80,
            &|c, seed| {
                let _r = base_5(c, seed, 0x1d98);
                c.g[A0][0] = DATA as u64;
                let m = DATA + 0x3000;
                c.bus.write32(gp_at(-0x47c), m);
                c.bus
                    .write32(DATA + 0x1c0, [0u32, 6, 2][(seed % 3) as usize]);
                c.bus.write32(m + 0x340, (seed % 2) as u32);
                c.bus.write32(m + 0xe0, ((seed / 2) % 2) as u32);
                c.bus.write32(m + 0x130, (seed % 4) as u32);
                c.bus.write32(m + 0x434, m + 0x1000);
                c.bus.write32(m + 0x1000, m + 0x1100);
                c.bus.write16(m + 0x1100 + 0x58, 0);
                c.bus.write32(m + 0x1100 + 0x5c, FAKE);
                c.bus.write16(m + 0x1100 + 0x258, 0);
                c.bus.write32(m + 0x1100 + 0x25c, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn mem_card_ui_try_next_slot_matches() {
        diff(
            ee::s04::f_001d8160__generated,
            mem_card_ui_try_next_slot,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1d_8160);
                c.g[A0][0] = sx(DATA);
                w_m2(c, DATA + 0x424, (seed % 4) as u32);
                w_m2(c, DATA + 0x428, (seed % 3) as u32);
                w_m2(c, DATA + 0x434, r.next_u32());
                w_m2(c, DATA + 0x1a0, r.next_u32());
                w_m2(c, DATA + 0x1a4, r.next_u32());
                w_m2(c, DATA + 0x1c0, r.next_u32());
            },
        );
    }

    #[test]
    fn memcard_state_base_handle_input_matches() {
        check_all(
            ee::s04::f_001d8c50__generated,
            memcard_state_base_handle_input,
            60,
            &|c, seed| {
                machine(c, seed, 0x1d8c, 0);
                let this_ = DATA + 0x1000;
                let input = DATA + 0x2000;
                c.g[A0][0] = this_ as u64;
                c.g[A1][0] = input as u64;
                c.bus.write32(this_ + 0x1a8, 0);
                c.bus.write32(this_ + 0x40, 0);
                // `MenuSys_IsActive` stands in for the input slots: it returns
                // whether the object's first word is non-zero and +0x54 is too.
                let vt = DATA + 0x3000;
                c.bus.write16(vt + 0x98, 0);
                c.bus.write32(vt + 0x9c, 0x002c_c098);
                c.bus.write16(vt + 0xa0, 0x100);
                c.bus.write32(vt + 0xa4, 0x002c_c098);
                c.bus.write32(input + 8, vt);
                let first = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(input, first);
                c.bus.write32(input + 0x54, 1);
                let second = if seed % 3 == 1 { 1 } else { 0 };
                c.bus.write32(input + 0x100, second);
                c.bus.write32(input + 0x154, 1);
            },
        );
    }
    use crate::difftest::Rng;

    /// Where the fixtures' objects live: above the test heap
    /// (`DATA + 0x4_0000..+0x8_0000`) and the fakes' random pointers.
    pub(crate) const MEMCARD_T_FE: u32 = DATA + 0x20_0000;

    pub(crate) const MEMCARD_T_FE_VT: u32 = DATA + 0x20_1000;

    pub(crate) const MEMCARD_T_POPUP: u32 = DATA + 0x20_2000;

    pub(crate) const MEMCARD_T_POPUP_VT: u32 = DATA + 0x20_3000;

    pub(crate) const MEMCARD_T_TEXT_DB: u32 = DATA + 0x20_4000;

    pub(crate) const MEMCARD_T_TEXT_DB_VT: u32 = DATA + 0x20_5000;

    pub(crate) const MEMCARD_T_CARD: u32 = DATA + 0x20_6000;

    pub(crate) const MEMCARD_T_CARD_VT: u32 = DATA + 0x20_7000;

    pub(crate) const MEMCARD_T_MC: u32 = DATA + 0x20_8000;

    /// A vtable at `vt` whose `slots` call the fakes, each with a small random
    /// `this` delta. Like `difftest::fakes::vtable`, but at a fixed address:
    /// some tests then point one slot at a real function (`RET_ZERO`).
    pub(crate) fn memcard_t_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(vt + off, (r.next_u32() & 0x70) as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// The callbacks' world: a working heap, the memory-card state at
    /// `gp - 0x10c8` with its vtable and popup, the application's text
    /// database, and the card manager `MCManager_Get` returns (its instance at
    /// `gp - 0x47c`) with a card object. Every vtable slot is a fake; `calls`
    /// is the number of fake calls a seed may unwind at.
    pub(crate) fn memcard_t_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = alloc_base(c, seed, salt, calls);
        let app = c.bus.read32(GAME_GP.wrapping_sub(0x848));
        c.bus.write32(GAME_GP.wrapping_sub(0x10c8), MEMCARD_T_FE);
        c.bus
            .write32(MEMCARD_T_FE + MEMCARD_FE_VTABLE, MEMCARD_T_FE_VT);
        let fe_slots = [
            0x118,
            MEMCARD_FE_VT_SHOW_MESSAGE,
            MEMCARD_FE_VT_CLOSE,
            0x148,
            0x178,
        ];
        memcard_t_vtable(c, &mut r, MEMCARD_T_FE_VT, &fe_slots);
        c.bus
            .write32(MEMCARD_T_FE + MEMCARD_FE_POPUP, MEMCARD_T_POPUP);
        c.bus
            .write32(MEMCARD_T_POPUP + MEMCARD_FE_VTABLE, MEMCARD_T_POPUP_VT);
        let popup_slots = [
            MEMCARD_POPUP_VT_V25,
            MEMCARD_POPUP_VT_SET_OPTION,
            MEMCARD_POPUP_VT_REINIT,
        ];
        memcard_t_vtable(c, &mut r, MEMCARD_T_POPUP_VT, &popup_slots);
        c.bus.write32(app + MEMCARD_APP_TEXT_DB, MEMCARD_T_TEXT_DB);
        c.bus.write32(MEMCARD_T_TEXT_DB + 4, MEMCARD_T_TEXT_DB_VT);
        memcard_t_vtable(c, &mut r, MEMCARD_T_TEXT_DB_VT, &[MEMCARD_TEXT_DB_VT_FIND]);
        c.bus.write32(GAME_GP.wrapping_sub(0x47c), MEMCARD_T_MC);
        c.bus
            .write32(MEMCARD_T_MC + MEMCARD_MC_CARD, MEMCARD_T_CARD);
        c.bus
            .write32(MEMCARD_T_MC + MEMCARD_MC_SLOT, r.next_u32() % 3);
        c.bus
            .write32(MEMCARD_T_MC + MEMCARD_MC_CARD_SLOT, r.next_u32() % 2);
        c.bus.write32(MEMCARD_T_CARD, MEMCARD_T_CARD_VT);
        memcard_t_vtable(c, &mut r, MEMCARD_T_CARD_VT, &[MEMCARD_CARD_VT_SLOT_NAME]);
        c.bus
            .write32(MEMCARD_T_FE + MEMCARD_FE_MESSAGE, r.next_u32());
        r
    }

    /// A callback with no inputs beyond [`memcard_t_setup`]: compared over 60
    /// seeds, unwinding at any of its first `calls` fake calls.
    pub(crate) fn memcard_t_check(generated: GuestFn, hand: GuestFn, salt: u64, calls: u32) {
        check(generated, hand, 60, &|c, seed| {
            let _r = memcard_t_setup(c, seed, salt, calls);
        });
    }

    #[test]
    fn mem_card_ui_on_save_done_matches() {
        check(
            ee::s04::f_001d66b0__generated,
            mem_card_ui_on_save_done,
            60,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_66b0, 4);
                let from_pause = if seed % 3 == 0 { r.next_u32() | 1 } else { 0 };
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_SAVE_FROM_PAUSE, from_pause);
                c.bus.write32(MEMCARD_T_FE + 0x1f0, r.next_u32());
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_BUSY, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_ui_on_load_done_matches() {
        check(
            ee::s04::f_001d67c8__generated,
            mem_card_ui_on_load_done,
            60,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_67c8, 4);
                let from_pause = if seed % 3 == 0 { r.next_u32() | 1 } else { 0 };
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_LOAD_FROM_PAUSE, from_pause);
                c.bus.write32(MEMCARD_T_FE + 0x1b4, r.next_u32());
                c.bus.write32(MEMCARD_T_FE + 0x1f4, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_ui_confirm_overwrite_matches() {
        memcard_t_check(
            ee::s04::f_001d68e8__generated,
            mem_card_ui_confirm_overwrite,
            0x1d_68e8,
            3,
        );
    }

    #[test]
    fn mem_card_state_base_show_create_edit_msg_matches() {
        memcard_t_check(
            ee::s04::f_001d69e0__generated,
            mem_card_state_base_show_create_edit_msg,
            0x1d_69e0,
            3,
        );
    }

    #[test]
    fn mem_card_ui_on_load_failed_matches() {
        memcard_t_check(
            ee::s04::f_001d6ad0__generated,
            mem_card_ui_on_load_failed,
            0x1d_6ad0,
            4,
        );
    }

    #[test]
    fn mem_card_ui_on_save_failed_matches() {
        memcard_t_check(
            ee::s04::f_001d6c28__generated,
            mem_card_ui_on_save_failed,
            0x1d_6c28,
            4,
        );
    }

    #[test]
    fn mem_card_ui_on_card_not_detected_matches() {
        check(
            ee::s04::f_001d6d88__generated,
            mem_card_ui_on_card_not_detected,
            80,
            &|c, seed| {
                let _r = memcard_t_setup(c, seed, 0x1d_6d88, 8);
                // Which of the two flags and the hook let the player go on.
                let (a, b) = [(1, 0), (0, 1), (0, 0), (0, 0)][(seed % 4) as usize];
                c.bus.write32(MEMCARD_T_MC + MEMCARD_MC_CONTINUE_A, a);
                c.bus.write32(MEMCARD_T_MC + MEMCARD_MC_CONTINUE_B, b);
                // The hook: a real function returning 0, or a fake (a pointer:
                // "yes").
                let hook = if seed % 8 < 4 { RET_ZERO } else { FAKE + 0x100 };
                c.bus.write32(MEMCARD_T_MC + MEMCARD_MC_CAN_CONTINUE, hook);
            },
        );
    }

    #[test]
    fn mem_card_ui_on_save_corrupt_matches() {
        memcard_t_check(
            ee::s04::f_001d7010__generated,
            mem_card_ui_on_save_corrupt,
            0x1d_7010,
            5,
        );
    }

    #[test]
    fn mem_card_ui_show_corrupt_save_matches() {
        memcard_t_check(
            ee::s04::f_001d7190__generated,
            mem_card_ui_show_corrupt_save,
            0x1d_7190,
            3,
        );
    }

    #[test]
    fn mem_card_ui_confirm_format_matches() {
        memcard_t_check(
            ee::s04::f_001d7318__generated,
            mem_card_ui_confirm_format,
            0x1d_7318,
            3,
        );
    }

    #[test]
    fn mem_card_ui_on_format_done_matches() {
        memcard_t_check(
            ee::s04::f_001d7420__generated,
            mem_card_ui_on_format_done,
            0x1d_7420,
            4,
        );
    }

    #[test]
    fn mem_card_ui_on_format_failed_matches() {
        memcard_t_check(
            ee::s04::f_001d7548__generated,
            mem_card_ui_on_format_failed,
            0x1d_7548,
            4,
        );
    }

    #[test]
    fn mem_card_ui_confirm_delete_matches() {
        check(
            ee::s04::f_001d7688__generated,
            mem_card_ui_confirm_delete,
            80,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_7688, 4);
                memcard_t_vtable(
                    c,
                    &mut r,
                    MEMCARD_T_CARD_VT,
                    &[MEMCARD_CARD_VT_SLOT_NAME, MEMCARD_CARD_VT_SAVE_COUNT],
                );
                // The save count: none (a real function returning 0, or a
                // forced -1 is out of reach: the fakes return pointers), or some.
                if seed % 2 == 0 {
                    c.bus
                        .write32(MEMCARD_T_CARD_VT + MEMCARD_CARD_VT_SAVE_COUNT + 4, RET_ZERO);
                }
                let state = [4, 4, 2, 5][(seed / 2 % 4) as usize];
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_STATE, state);
                c.bus.write32(MEMCARD_T_FE + 0x1e0, r.next_u32());
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_BUSY, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_ui_on_card_not_detected2_matches() {
        memcard_t_check(
            ee::s04::f_001d77e8__generated,
            mem_card_ui_on_card_not_detected2,
            0x1d_77e8,
            6,
        );
    }

    /// The not-enough-space replies: the card's save-size method is a fake.
    pub(crate) fn memcard_t_space(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = memcard_t_setup(c, seed, salt, 5);
        memcard_t_vtable(
            c,
            &mut r,
            MEMCARD_T_CARD_VT,
            &[MEMCARD_CARD_VT_SLOT_NAME, MEMCARD_CARD_VT_SAVE_SIZE],
        );
    }

    #[test]
    fn mem_card_ui_on_not_enough_space_matches() {
        check(
            ee::s04::f_001d7998__generated,
            mem_card_ui_on_not_enough_space,
            60,
            &|c, seed| memcard_t_space(c, seed, 0x1d_7998),
        );
    }

    #[test]
    fn mem_card_ui_on_not_enough_space2_matches() {
        check(
            ee::s04::f_001d7b18__generated,
            mem_card_ui_on_not_enough_space2,
            60,
            &|c, seed| memcard_t_space(c, seed, 0x1d_7b18),
        );
    }

    #[test]
    fn mem_card_ui_on_name_exists_matches() {
        memcard_t_check(
            ee::s04::f_001d7c68__generated,
            mem_card_ui_on_name_exists,
            0x1d_7c68,
            5,
        );
    }

    #[test]
    fn mem_card_ui_on_different_card_matches() {
        memcard_t_check(
            ee::s04::f_001d7d70__generated,
            mem_card_ui_on_different_card,
            0x1d_7d70,
            5,
        );
    }

    #[test]
    fn mem_card_ui_on_delete_done_matches() {
        check(
            ee::s04::f_001d7ed0__generated,
            mem_card_ui_on_delete_done,
            80,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_7ed0, 4);
                let from_pause = if seed % 3 == 0 { r.next_u32() | 1 } else { 0 };
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_LOAD_FROM_PAUSE, from_pause);
                let state = [4, 4, 1, 6][(seed / 3 % 4) as usize];
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_STATE, state);
            },
        );
    }

    #[test]
    fn mem_card_ui_on_delete_failed_matches() {
        memcard_t_check(
            ee::s04::f_001d8020__generated,
            mem_card_ui_on_delete_failed,
            0x1d_8020,
            4,
        );
    }

    #[test]
    fn mem_card_state_base_reset_after_save_matches() {
        check(
            ee::s04::f_001d8c38__generated,
            mem_card_state_base_reset_after_save,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d_8c38, 0);
                c.g[A0][0] = sx(MEMCARD_T_FE);
                for off in [0x1a8, 0x1c0, 0x1dc] {
                    c.bus.write32(MEMCARD_T_FE + off, r.next_u32());
                }
            },
        );
    }

    pub(crate) const MEMCARD_T_SCREEN: u32 = DATA + 0x20_9000;

    pub(crate) const MEMCARD_T_BOX: u32 = DATA + 0x20_a000;

    pub(crate) const MEMCARD_T_BOX_VT: u32 = DATA + 0x20_b000;

    /// The state's UI screen with one child object, the "text box message"
    /// (its id the `StrHash` of that name) unless `found` is false; the box's
    /// vtable slots are fakes. Also the manager's state machine.
    pub(crate) fn memcard_t_screen(c: &mut Cpu, r: &mut Rng, found: bool) {
        let hash = crate::game::fe::feasyncfile::tests::str_hash(c, MEMCARD_TEXT_BOX_NAME);
        c.bus
            .write32(MEMCARD_T_FE + MEMCARD_FE_SCREEN, MEMCARD_T_SCREEN);
        c.bus
            .write32(MEMCARD_T_SCREEN + 0x38, MEMCARD_T_SCREEN + 0x10);
        c.bus
            .write32(MEMCARD_T_SCREEN + 0x3c, MEMCARD_T_SCREEN + 0x20);
        c.bus.write32(MEMCARD_T_SCREEN + 0x10, 1);
        c.bus.write32(MEMCARD_T_SCREEN + 0x20, MEMCARD_T_BOX);
        c.bus
            .write32(MEMCARD_T_BOX + MEMCARD_FE_VTABLE, MEMCARD_T_BOX_VT);
        c.bus
            .write32(MEMCARD_T_BOX + 0x38, if found { hash } else { hash ^ 1 });
        memcard_t_vtable(c, r, MEMCARD_T_BOX_VT, &[0x28, MEMCARD_UI_VT_SHOW]);
        let machine_vt = MEMCARD_T_MC + 0x800;
        c.bus
            .write32(MEMCARD_T_MC + MEMCARD_MC_STATE_MACHINE, machine_vt);
        memcard_t_vtable(c, r, machine_vt, &[8]);
    }

    #[test]
    fn mem_card_ui_on_no_cards_matches() {
        check(
            ee::s04::f_001d8240__generated,
            mem_card_ui_on_no_cards,
            80,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_8240, 4);
                memcard_t_screen(c, &mut r, seed % 4 != 0);
                let mode = [1, 0, 3, 2][(seed / 4 % 4) as usize];
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_MODE, mode);
            },
        );
    }

    #[test]
    fn mem_card_ui_check_cards_present_matches() {
        check(
            ee::s04::f_001d83a8__generated,
            mem_card_ui_check_cards_present,
            120,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_83a8, 6);
                memcard_t_screen(c, &mut r, seed % 5 != 0);
                // Any card in: a fake (a pointer, "yes") or a real 0.
                let any = if seed % 2 == 0 {
                    RET_ZERO
                } else {
                    FAKE + 0x100
                };
                c.bus
                    .write32(MEMCARD_T_CARD_VT + MEMCARD_CARD_VT_ANY_CARD + 4, any);
                let mode = [1, 3, 0, 2][(seed / 2 % 4) as usize];
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_MODE, mode);
            },
        );
    }

    pub(crate) const MEMCARD_T_ENGINE: u32 = DATA + 0x20_c000;

    pub(crate) const MEMCARD_T_TEXT: u32 = DATA + 0x20_d000;

    pub(crate) const POPUP_T: u32 = DATA + 0x21_0000;

    /// A short UTF-16 text at [`MEMCARD_T_TEXT`].
    pub(crate) fn memcard_t_text(c: &mut Cpu) {
        for (i, ch) in [0x48u16, 0x69, 0].iter().enumerate() {
            c.bus.write16(MEMCARD_T_TEXT + 2 * i as u32, *ch);
        }
    }

    #[test]
    fn mem_card_state_base_show_message_matches() {
        check(
            ee::s04::f_001d8530__generated,
            mem_card_state_base_show_message,
            80,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_8530, 3);
                memcard_t_text(c);
                c.g[A0][0] = sx(MEMCARD_T_FE);
                c.g[A1][0] = sx(MEMCARD_T_TEXT);
                c.g[A2][0] = (seed & 1) as u64;
                c.g[A3][0] = (seed >> 1 & 1) as u64;
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                c.g[T1][0] = (seed >> 2 & 1) as u64;
                c.g[T2][0] = (r.next_u32() % 4) as u64;
                let msg = [1, 0x16, 5, 1][(seed / 8 % 4) as usize];
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_MESSAGE, msg);
                c.bus.write32(MEMCARD_T_FE + 0x210, r.next_u32());
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_ENGINE, MEMCARD_T_ENGINE);
                // No popup yet: a new one is created and pushed.
                if seed % 3 == 0 {
                    c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_POPUP, 0);
                }
            },
        );
    }

    /// A popup at [`POPUP_T`]: random option count and live object, and the
    /// rebuild's own early-out (`+0x2bc` clear).
    pub(crate) fn popup_t_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = memcard_t_setup(c, seed, salt, 1);
        memcard_t_text(c);
        c.g[A0][0] = sx(POPUP_T);
        c.bus
            .write32(POPUP_T + POPUP_OPTION_COUNT, r.next_u32() % 4);
        c.bus.write32(
            POPUP_T + POPUP_LIVE,
            if seed % 2 == 0 { 0 } else { DATA + 0x21_4000 },
        );
        c.bus.write32(POPUP_T + 0x2bc, 0);
        r
    }

    #[test]
    fn c_fe_popup_confirm_init_buttons_matches() {
        check(
            ee::s04::f_001d9a80__generated,
            c_fe_popup_confirm_init_buttons,
            80,
            &|c, seed| {
                let mut r = popup_t_setup(c, seed, 0x1d_9a80);
                c.g[A1][0] = if seed % 4 == 1 { 0 } else { sx(MEMCARD_T_TEXT) };
                c.g[A2][0] = (seed >> 1 & 1) as u64;
                c.g[A3][0] = (seed / 4 % 3 == 0) as u64;
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                c.g[T1][0] = (r.next_u32() % 2) as u64;
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v26_matches() {
        check(
            ee::s04::f_001d9bd0__generated,
            c_fe_popup_confirm_v26,
            80,
            &|c, seed| {
                let _r = popup_t_setup(c, seed, 0x1d_9bd0);
                c.g[A1][0] = sx(MEMCARD_T_TEXT);
                c.g[A2][0] = (seed / 2 % 4) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_ctor_matches() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s04::f_001d9d48__generated,
            c_fe_state_net_help_popup_ctor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d_9d48, 0);
                c.g[A0][0] = sx(POPUP_T);
                c.g[A1][0] = sx(MEMCARD_T_ENGINE);
                c.g[A2][0] = sx(MEMCARD_T_FE);
                schedule(c, &mut r, seed, 0x001d_9d48)
            },
        );
    }

    /// A screen for `MemCardStateBase_SetMode`: up to three children
    /// ("onmenu", "offmenu", the text box; `children` of them), stand-in
    /// names written at the name addresses (any distinct strings do), and a
    /// one-entry value table when `value` names one.
    pub(crate) fn set_mode_t_screen(c: &mut Cpu, r: &mut Rng, children: u32, value: Option<u32>) {
        use crate::game::fe::feasyncfile::tests::str_hash;
        let names = [
            MEMCARD_NAME_ON_MENU,
            MEMCARD_NAME_OFF_MENU,
            MEMCARD_TEXT_BOX_NAME,
        ];
        for (i, at) in [
            MEMCARD_NAME_ON_MENU,
            MEMCARD_NAME_OFF_MENU,
            MEMCARD_TEXT_BOX_NAME,
            MEMCARD_NAME_OFF,
            MEMCARD_NAME_ON,
            MEMCARD_NAME_FILE_DISPLAY,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write8(at, b'a' + i as u8);
            c.bus.write8(at + 1, 0);
        }
        let screen = MEMCARD_T_SCREEN;
        c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_SCREEN, screen);
        c.bus.write32(screen + 0x38, screen + 0x10);
        c.bus.write32(screen + 0x3c, screen + 0x20);
        c.bus.write32(screen + 0x10, children);
        for (i, name) in names.into_iter().enumerate() {
            let node = MEMCARD_T_BOX + 0x100 * i as u32;
            let hash = str_hash(c, name);
            c.bus.write32(screen + 0x20 + 4 * i as u32, node);
            c.bus.write32(node + MEMCARD_FE_VTABLE, MEMCARD_T_BOX_VT);
            c.bus.write32(node + 0x38, hash);
        }
        memcard_t_vtable(
            c,
            r,
            MEMCARD_T_BOX_VT,
            &[0x28, MEMCARD_UI_VT_SET_VISIBLE, MEMCARD_UI_VT_SHOW],
        );
        let table = screen + 0x400;
        c.bus.write32(screen + 0x34, table);
        match value {
            Some(name) => {
                let hash = str_hash(c, name);
                c.bus.write32(table, 1);
                c.bus.write32(table + 4, hash);
                c.bus.write32(table + 8, 0xc);
                c.bus.write16(table + 0xc, (r.next_u32() % 8) as u16);
            }
            None => c.bus.write32(table, 0),
        }
    }

    #[test]
    fn mem_card_state_base_set_mode_matches() {
        check(
            ee::s04::f_001d9470__generated,
            mem_card_state_base_set_mode,
            160,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_9470, 6);
                let mode = [1u32, 2, 3, 0, 4, 1, 2, 3][(seed % 8) as usize];
                let value = match (seed / 8) % 3 {
                    0 => None,
                    _ => Some(
                        [
                            0,
                            MEMCARD_NAME_OFF,
                            MEMCARD_NAME_FILE_DISPLAY,
                            MEMCARD_NAME_ON,
                        ][mode.min(3) as usize],
                    ),
                };
                let value = value.filter(|&v| v != 0);
                set_mode_t_screen(c, &mut r, [3, 3, 2, 1][(seed / 24 % 4) as usize], value);
                c.g[A0][0] = sx(MEMCARD_T_FE);
                c.g[A1][0] = mode as u64;
            },
        );
    }
}
