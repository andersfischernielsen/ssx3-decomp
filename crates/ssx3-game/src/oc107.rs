//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold front-end, UI, audio and animation helpers, each
//! with a differential test against the generated body.
//!
//! - **Memory-card / rider state**: `MemCardUI_OnLoadAborted`,
//!   `Rider_SetTimedEvent`, `BENewPlayerInt_ShareSlot0CharRecord`.
//! - **Script objects**: `ScriptObj_dtor`, `ScriptList_GetEntryType`.
//! - **FX / particle objects**: the lens-FX and particle-modifier
//!   destructors, `MultiSplineModifier_v19`.
//! - **Animation objects**: the `AnimObject` / `AnimTeeter` constructors.
//! - **Strings**: `cBXString_AssignCopy`, `cBXString_ReleaseBuffer`.
//! - **Replay**: `Replay_FreeBuffers`.
//! - **UI screens/states**: `UIScreen_CmdStartThread`, the OV screen
//!   initialisers, `UIObject_SetAnim`, `UIStateMgr_CloseTopStates`,
//!   `cUIGroup_v18`, `TexMan_Shutdown` and the `cFEState*` helpers.
//! - **Net / lobby**: `NetFEState_*`, `cFEStateCreate*`.
//! - **Audio**: `SSXAudioSystem_StopTrickMeterSound`,
//!   `SSXAudioSystem_IsTrackGroup1Event`, `CrowdAudio_FreeSampleList`,
//!   `IniParser_FindSection`.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers, callees and event handlers see the same values), follow the
//! call protocol of [`crate::call`] and the event rules of
//! [`crate::events`].

use crate::abi;
use crate::call::{call_ptr, guest, sp, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

#[rustfmt::skip]
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_72a8, memcard_ui_on_load_aborted, "MemCardUI_OnLoadAborted"),
    (0x0010_e028, rider_set_timed_event, "Rider_SetTimedEvent"),
    (0x0028_3228, script_obj_dtor, "ScriptObj_dtor"),
    (0x002e_3060, lens_fx_base_dtor, "LensFXBase_dtor"),
    (0x0014_7828, be_new_player_int_share_slot0_char_record, "BENewPlayerInt_ShareSlot0CharRecord"),
    (0x0034_1c80, anim_object_ctor_2, "AnimObject_ctor_2"),
    (0x0034_22e8, anim_teeter_ctor_2, "AnimTeeter_ctor_2"),
    (0x0014_c2b0, bolts_index_bolts_by_char, "Bolts_IndexBoltsByChar"),
    (0x0027_6998, script_list_get_entry_type, "ScriptList_GetEntryType"),
    (0x0039_cc38, ui_screen_cmd_start_thread, "UIScreen_CmdStartThread"),
    (0x0031_83d8, cbx_string_assign_copy, "cBXString_AssignCopy"),
    (0x0031_8a88, cbx_string_release_buffer, "cBXString_ReleaseBuffer"),
    (0x0026_f428, replay_free_buffers, "Replay_FreeBuffers"),
    (0x001a_0598, fe_module_render_rider_previews, "FEModule_RenderRiderPreviews"),
    (0x0039_87f8, c_ui_group_v18, "cUIGroup_v18"),
    (0x0034_5ad0, particle_modifier_dtor, "ParticleModifier_dtor"),
    (0x0034_5e88, dynamic_particle_modifier_dtor, "DynamicParticleModifier_dtor"),
    (0x0035_aaf0, multi_spline_modifier_v19, "MultiSplineModifier_v19"),
    (0x0020_8bb8, ov_screen_session_hide_map_pic, "OVScreenSession_HideMapPic"),
    (0x0020_9300, ov_screen_session_post_input, "OVScreenSession_PostInput"),
    (0x0036_7360, tex_man_shutdown, "TexMan_Shutdown"),
    (0x001f_73a8, ov_screen_bc_start_init, "OVScreenBCStart_Init"),
    (0x0021_2198, ov_screen_autosave_init, "OVScreenAutosave_Init"),
    (0x001a_1d50, fe_module_dtor, "FEModule_dtor"),
    (0x0019_2740, c_fe_state_bragging_rights_set_rows_visible, "cFEStateBraggingRights_SetRowsVisible"),
    (0x0039_fcc8, ui_object_set_anim, "UIObject_SetAnim"),
    (0x0018_1238, c_fe_state_char_select_shutdown, "cFEStateCharSelect_Shutdown"),
    (0x0019_5b70, fe_locale_load_common_table, "FELocale_LoadCommonTable"),
    (0x0015_ccf0, cam_controller_reset, "CamController_Reset"),
    (0x0018_3238, c_fe_state_rider_detail_ctor, "cFEStateRiderDetail_ctor"),
    (0x0039_f190, ui_state_mgr_close_top_states, "UIStateMgr_CloseTopStates"),
    (0x0019_5880, fe_locale_ctor, "FELocale_ctor"),
    (0x0020_0218, ov_screen_rewards_list_popup_init, "OVScreenRewardsListPopup_Init"),
    (0x001a_9268, net_fe_state_on_messenger_popup_event, "NetFEState_OnMessengerPopupEvent"),
    (0x001a_b108, net_fe_state_show_wait_player_popup, "NetFEState_ShowWaitPlayerPopup"),
    (0x001a_ebf0, c_fe_state_challenge_popup_v32, "cFEStateChallengePopup_v32"),
    (0x001b_0670, c_fe_state_challenge_received_popup_transition_in, "cFEStateChallengeReceivedPopup_TransitionIn"),
    (0x001b_88f0, c_fe_state_create_step1_ctor, "cFEStateCreateStep1_ctor"),
    (0x001b_9740, c_fe_state_create_step1_set_inform_selection, "cFEStateCreateStep1_SetInformSelection"),
    (0x001b_97b0, c_fe_state_create_step1_set_share_info_selection, "cFEStateCreateStep1_SetShareInfoSelection"),
    (0x001b_b4b0, c_fe_state_create_step2_set_legal_selection, "cFEStateCreateStep2_SetLegalSelection"),
    (0x001b_c290, c_fe_state_create_duplicate_submit_account, "cFEStateCreateDuplicate_SubmitAccount"),
    (0x001b_e038, c_fe_state_lobby_menu_clamp_room_selections, "cFEStateLobbyMenu_ClampRoomSelections"),
    (0x001c_5098, c_fe_state_net_news_init, "cFEStateNetNews_Init"),
    (0x001e_25d8, message_center_on_bc_finish, "MessageCenter_OnBCFinish"),
    (0x0036_1f98, intc_table_add_handler, "IntcTable_AddHandler"),
    (0x0029_b3c0, ssx_audio_system_stop_trick_meter_sound, "SSXAudioSystem_StopTrickMeterSound"),
    (0x002a_40e8, ssx_audio_system_is_track_group1_event, "SSXAudioSystem_IsTrackGroup1Event"),
    (0x002a_4cf8, crowd_audio_free_sample_list, "CrowdAudio_FreeSampleList"),
    (0x002a_7bf8, ini_parser_find_section, "IniParser_FindSection"),
];

// ---------------------------------------------------------------------------
// Small register helpers.

/// The address `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gpr(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
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
        c.g[V0][0] = Addr(at(c, A1, 8)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x148)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x14c)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001d_72cc)?;

        c.g[V0][0] = Addr(gpr(c, -0x10c8)).lw(c);
        c.g[A0][0] = Addr(gpr(c, -0x848)).lw(c);
        Addr(at(c, V0, 0x1e4)).sw(c, 0);
        call_ptr(c, 0x0022_7f80, 0x001d_72dc)?;
        c.g[S0][0] = c.g[V0][0];

        c.g[A1][0] = 0x24;
        c.g[V0][0] = Addr(at(c, S0, 0x748)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 8)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
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

/// `void Rider_SetTimedEvent(Rider* self, int mode, float seconds)`
/// (0x10e028): boost timer; `mode == 0` clears it to -1, otherwise the frame
/// counter plus `seconds * 60` truncated.
pub fn rider_set_timed_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        let mode = c.g[S2][0] != 0;
        frame.save_ra(c, 0);
        if !mode {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            Addr(at(c, S1, 0x358)).sw(c, 0);
        } else {
            c.cycles += 9;
            c.g[AT][0] = 0x4270_0000;
            c.mtc1(1, 0);
            c.mul_s(0, 12, 0);
            c.cvt_w_s(1, 0);
            c.mfc1(16, 1);
            call_ptr(c, 0x0012_98c8, 0x0010_e070)?;
            Addr(at(c, S1, 0x358)).sw(c, c.g[S2][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        }
        c.cycles += 7;
        Addr(at(c, S1, 0x354)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptObj_dtor(ScriptObj* self, int flags)` (0x283228): release the
/// script object (unregister it when its registration word is 0 or -1) and
/// optionally free it.
pub fn script_obj_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4825_58;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(at(c, S0, 4)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
        let t = c.g[V1][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !t {
            c.cycles += 4;
            call_ptr(c, 0x0028_32d8, 0x0028_3264)?;
            c.g[V0][0] = u64::MAX;
            Addr(at(c, S0, 4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0028_3280)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LensFXBase_dtor(LensFXBase* self, int flags)` (0x2e3060): unhook the
/// lens FX from the global render list, clear the pending-effect word and
/// optionally free the object.
pub fn lens_fx_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4886_b0;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A3][0] = Addr(gpr(c, 0x2a90)).lw(c);
        c.g[A1][0] = 0;
        Addr(at(c, S1, 0x649c)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[V0][0] = Addr(at(c, A3, 0x10d8)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x3a8)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x3ac)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c, A3, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002e_30a4)?;

        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        Addr(gpr(c, 0xa28)).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002e_30b8)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BENewPlayerInt_ShareSlot0CharRecord(int index)` (0x147828): copy
/// rider 0's character id into the player slot's record and set its shared
/// flag.
pub fn be_new_player_int_share_slot0_char_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1c;
        call_ptr(c, 0x0014_7798, 0x0014_7848)?;
        c.mult(0, 2, 2, 18);
        c.g[S0][0] = 0x534f_e0;
        c.g[A0][0] = c.g[S1][0];
        c.g[V1][0] = Addr(at(c, S0, 0x11)).lbu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        Addr(at(c, V0, 0x11)).sb(c, c.g[V1][0]);
        call_ptr(c, 0x0014_7798, 0x0014_7868)?;
        c.mult(0, 3, 2, 18);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = Addr(at(c, V0, 0x10)).lw(c);
        frame.restore(c, S0, 0x30);
        c.g[V1][0] = c.g[V1][0] | 4;
        Addr(at(c, V0, 0x10)).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// The shared tail of the two animation-node constructors: construct the
/// embedded base object at `self + base`, install the class v-table, run the
/// stream's node constructor for the base and set the object's flag bit.
fn anim_node_ctor(
    c: &mut Cpu,
    base: u32,
    class_vt: u32,
    vtable_arg: u32,
    flags_off: u32,
    ret_ctor: u32,
    ret_arg: u32,
) -> Flow {
    c.cycles += 27;
    let frame = Frame::open(c, 0x30);
    let self_ = c.g[A0][0] as u32;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = sx(self_);
    c.g[S1][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(self_.wrapping_add(base));
    call_ptr(c, 0x0034_dac8, ret_ctor)?;

    c.g[V1][0] = sx(class_vt);
    c.g[A1][0] = sx(self_);
    c.g[A2][0] = sx(vtable_arg);
    Addr(self_.wrapping_add(base).wrapping_add(0x20)).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(at(c, S1, 0)).lw(c);
    c.g[A0][0] = Addr(at(c, V0, 0x10)).lh(c);
    c.g[A3][0] = Addr(at(c, V0, 0x14)).lw(c);
    let t = c.g[A3][0] as u32;
    c.g[A0][0] = sx(at(c, S1, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret_arg)?;

    c.g[V1][0] = Addr(at(c, S0, flags_off)).lhu(c);
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0x10);
    c.g[V1][0] = c.g[V1][0] | 1;
    frame.restore_ra(c, 0);
    Addr(at(c, S0, flags_off)).sh(c, c.g[V1][0]);
    frame.restore(c, S0, 0x20);
    frame.close(c);
    Ok(())
}

/// `void* AnimObject_ctor_2(AnimObject* self, void* arg, Stream* stream)`
/// (0x341c80): construct the base node at +0x1c, install the class v-table,
/// let the stream build the node and set flag bit 0.
pub fn anim_object_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        anim_node_ctor(c, 0x1c, 0x490b_10, 0x1c, 0x42, 0x0034_1ca0, 0x0034_1cc8)
    })
}

/// `void* AnimTeeter_ctor_2(AnimTeeter* self, void* arg, Stream* stream)`
/// (0x3422e8): same for the teeter object at +0x30.
pub fn anim_teeter_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        anim_node_ctor(c, 0x30, 0x4908_f8, 0x30, 0x56, 0x0034_2308, 0x0034_2330)
    })
}

/// `void Bolts_IndexBoltsByChar(Bolts* self)` (0x14c2b0): count the bolts per
/// character group and remember the first index of each group.
pub fn bolts_index_bolts_by_char(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
    c.g[T1][0] = u64::MAX;
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !none {
        c.cycles += 4;
        c.g[T3][0] = sx(at(c, A0, 0x2c));
        c.g[T2][0] = sx(at(c, A0, 0xa4));
        c.g[T0][0] = 0;
        loop {
            c.cycles += 9;
            c.g[V0][0] = Addr(at(c, A0, 4)).lw(c);
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = Addr(at(c, V0, 0)).lb(c);
            c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = Addr(at(c, V1, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let same = c.g[T1][0] == c.g[A1][0];
            Addr(at(c, V1, 0)).sw(c, c.g[V0][0]);
            if !same {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[T1][0] = c.g[A1][0];
                Addr(at(c, V0, 0)).sw(c, c.g[A3][0]);
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x38));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_c2d0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int ScriptList_GetEntryType(ScriptList* self, int i)` (0x276998): the
/// command type at index `i` when it is in range and below 0x1c, else 0x1c.
pub fn script_list_get_entry_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx(at(c, A0, 0xc));
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_3d28, 0x0027_69b8)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        let mut result = 0x1c;
        if in_range {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0028_3d70, 0x0027_69cc)?;
            c.g[V1][0] = Addr(at(c, V0, 0)).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 28) as u64;
            let low = c.g[V1][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if low {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0028_3d70, 0x0027_69e4)?;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                result = c.g[V0][0] as u32;
            } else {
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = sx(result);
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
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
        c.g[V0][0] = Addr(at(c, T0, 0x34)).lw(c);
        c.g[V1][0] = Addr(at(c, V0, 0)).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[A1][0] = sx(at(c, V0, 4));
        if !empty {
            loop {
                c.cycles += 4;
                c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
                c.g[V0][0] = Addr(at(c, A2, 4)).lw(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = Addr(at(c, T0, 0x34)).lw(c);
                    break;
                }
                c.cycles += 6;
                c.g[A1][0] = Addr(at(c, A1, 8)).lhu(c);
                c.g[A0][0] = c.g[T0][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0039_cc78)?;
                c.g[RA][0] = Addr(at(c, SP, 0)).ld(c);
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                c.pc = ra;
                return Ok(());
            }
            loop {
                c.cycles += 6;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V1][0] = Addr(at(c, A1, 4)).lw(c);
                c.g[V0][0] = Addr(at(c, A0, 0)).lw(c);
                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(at(c, A1, 0).wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cc58);
                }
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
                    c.g[V0][0] = Addr(at(c, A2, 4)).lw(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = Addr(at(c, T0, 0x34)).lw(c);
                        break;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = Addr(at(c, A1, 8)).lhu(c);
                    c.g[A0][0] = c.g[T0][0];
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x0039_c870, 0x0039_cc78)?;
                    c.g[RA][0] = Addr(at(c, SP, 0)).ld(c);
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
            }
        }
        c.cycles += 1;
        c.g[RA][0] = Addr(at(c, SP, 0)).ld(c);
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `void cBXString_AssignCopy(cBXString* self, int len, const char* str)`
/// (0x3183d8): make room, copy `len` bytes and terminate, unless the string
/// is the shared empty buffer.
pub fn cbx_string_assign_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0031_8188, 0x0031_83fc)?;
        c.g[V0][0] = 0x48dc_d4;
        c.g[A0][0] = Addr(at(c, S0, 0)).lw(c);
        let shared = c.g[A0][0] == c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        if !shared {
            c.cycles += 7;
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_605c, 0x0031_8418)?;
            c.g[V1][0] = Addr(at(c, S0, 0)).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S1][0]);
            c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            Addr(c.g[V0][0] as u32).sb(c, 0);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_ReleaseBuffer(cBXString* self, int len)` (0x318a88):
/// detach the buffer, shrink it to `len` (`-1` uses the current length) and
/// terminate.
pub fn cbx_string_release_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_8120, 0x0031_8aa4)?;
        c.g[V0][0] = u64::MAX;
        let given = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = 0x4900_00;
        if !given {
            c.cycles += 4;
            c.g[A0][0] = Addr(at(c, S1, 0)).lw(c);
            call_ptr(c, 0x0041_6810, 0x0031_8ab8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x4900_00;
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(at(c, S1, 0)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x232c));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 5;
            Addr((c.g[V1][0] as u32).wrapping_sub(8)).sw(c, c.g[S0][0]);
            c.g[V0][0] = Addr(at(c, S1, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            Addr(c.g[V0][0] as u32).sb(c, 0);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_FreeBuffers(Replay* self)` (0x26f428): free every replay
/// stream buffer and clear both snapshot lists.
pub fn replay_free_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S2, 8)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(at(c, S2, 0x2c));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0026_e968, 0x0026_f45c)?;
                c.g[V0][0] = Addr(at(c, S2, 8)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f450);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx(at(c, S2, 0x3b0));
        call_ptr(c, 0x0026_e5c0, 0x0026_f474)?;
        c.g[A0][0] = sx(at(c, S2, 0x3bc));
        call_ptr(c, 0x0026_e5c0, 0x0026_f47c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RenderRiderPreviews(FEModule* self)` (0x1a0598): point the
/// render targets at the FE canvas and render both rider previews.
pub fn fe_module_render_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gpr(c, -0x854)).lw(c);
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x2e;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2f;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(at(c, V0, 0x24)).sw(c, c.g[V1][0]);
        c.g[S1][0] = sx(at(c, S0, 0x19c0));
        c.g[A1][0] = Addr(at(c, V0, 0x1008)).lw(c);
        Addr(at(c, V0, 0x84)).sw(c, c.g[A2][0]);
        Addr(at(c, V0, 0x10a8)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(at(c, S0, 0xcc8)).lw(c);
        loop {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = sx(at(c, S0, 0xce0));
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0019_ee88, 0x001a_05e0)?;
                c.g[S0][0] = sx(at(c, S0, 0xce0));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[V0][0] = Addr(at(c, S0, 0xcc8)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x001a_05d0);
            }
        }
    })
}

/// `void cUIGroup_Render(cUIGroup* self, int a1)` (0x3987f8): call every
/// child node's render method. The original's loop head is a resume entry
/// (an event at the back edge returns at 0x398828).
pub fn c_ui_group_v18(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        match c.pc {
            0x0039_87f8 => {
                c.cycles += 10;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
                c.g[A0][0] = sx(at(c, A0, 0x74));
                c.sq(17, frame.at(c, 0x10));
                c.sq(16, frame.at(c, 0x20));
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                call_ptr(c, 0x0039_7840, 0x0039_8814)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
            }
            0x0039_8820 => c.cycles += 2,
            0x0039_8828 => ui_group_dispatch(c)?,
            _ => {
                c.jump_out(c.pc, 0x0039_87f8, 0x0039_8864);
                return Ok(());
            }
        }
        loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_7718, 0x0039_8848)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V0][0] = Addr(at(c, S0, 8)).lw(c);
            if event_due(c) && back_edge_unwinds(c, 0x0039_8828) {
                return Ok(());
            }
            ui_group_dispatch(c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The render dispatch of one [`c_ui_group_v18`] node (0x398828).
fn ui_group_dispatch(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = Addr(at(c, V0, 0x90)).lh(c);
    c.g[V1][0] = Addr(at(c, V0, 0x94)).lw(c);
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0039_883c)?;
    c.g[S0][0] = Addr(at(c, S0, 4)).lw(c);
    Ok(())
}

/// `void ParticleModifier_dtor(ParticleModifier* self, int flags)`
/// (0x345ad0): free the modifier's point buffer, drop it from the emitter
/// list and run the base destructor.
pub fn particle_modifier_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4912_b0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at(c, S0, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S0, 0x1e0)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0034_5b04)?;
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(gpr(c, 0xf08)).lw(c);
        c.g[A0][0] = sx(at(c, S0, 0x50));
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(gpr(c, 0xf08)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0037_0758, 0x0034_5b1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_5828, 0x0034_5b28)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DynamicParticleModifier_dtor(DynamicParticleModifier* self, int
/// flags)` (0x345e88): the dynamic-particle variant of the same teardown.
pub fn dynamic_particle_modifier_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4912_68;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at(c, S0, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S0, 0x260)).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0034_5ebc)?;
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(gpr(c, 0xf0c)).lw(c);
        c.g[A0][0] = sx(at(c, S0, 0x60));
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(gpr(c, 0xf0c)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0037_0c08, 0x0034_5ed4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_5828, 0x0034_5ee0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiSplineModifier_v19(MultiSplineModifier* self, void* item)`
/// (0x35aaf0): unlink the spline object (when present) and notify the
/// current item when it is the modifier's own.
pub fn multi_spline_modifier_v19(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S0, 0x30)).lw(c);
        let no_spline = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_spline {
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x18)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0x1c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0035_ab24)?;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(at(c, S0, 0x40)).lw(c);
        let same = c.g[S1][0] == c.g[V0][0];
        c.g[V0][0] = sx(at(c, S1, 0x10));
        if same {
            c.cycles += 6;
            c.g[A1][0] = Addr(at(c, S1, 0xc)).lw(c);
            c.g[V1][0] = Addr(at(c, A1, 0xc)).lw(c);
            c.g[A0][0] = Addr(at(c, V1, 0xc0)).lh(c);
            c.g[V0][0] = Addr(at(c, V1, 0xc4)).lw(c);
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx(at(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0035_ab48)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_HideMapPic(OVScreenSession* self)` (0x208bb8):
/// drop both map pictures and hide the map.
pub fn ov_screen_session_hide_map_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(at(c, S0, 0xdc)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(at(c, S0, 0xd0)).lw(c);
        let none = c.g[A2][0] == 0;
        Addr(at(c, S0, 0xc8)).sw(c, 0);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(at(c, A2, 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(at(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0x4c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_8bf4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(at(c, S0, 0xe0)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, A2, 8)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0x4c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_8c14)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_PostInput(OVScreenSession* self, void* widget)`
/// (0x209300): when either of the widget's two status methods reports a
/// change, refresh the session's indicators.
pub fn ov_screen_session_post_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(at(c, S0, 8)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x88)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x8c)).lw(c);
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0020_932c)?;
        let first = c.g[V0][0] != 0;
        let mut update = first;
        if !first {
            c.cycles += 7;
            c.g[V1][0] = Addr(at(c, S0, 8)).lw(c);
            c.g[A0][0] = Addr(at(c, V1, 0x90)).lh(c);
            c.g[V0][0] = Addr(at(c, V1, 0x94)).lw(c);
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_9348)?;
            update = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
        }
        if update {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0020_9e78, 0x0020_9358)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_Shutdown(TexMan* self)` (0x367360): destroy both texture-info
/// arrays, free the VRAM heaps and invalidate the resident-texture table.
pub fn tex_man_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0036_7230, 0x0036_7374)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0036_7310, 0x0036_737c)?;
        c.g[A0][0] = sx(at(c, S0, 0x1f60));
        call_ptr(c, 0x0036_6548, 0x0036_7384)?;
        c.g[A0][0] = sx(at(c, S0, 0x4350));
        call_ptr(c, 0x0036_6548, 0x0036_738c)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx(at(c, S0, 0x1f50));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7398);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        frame.restore_ra(c, 0);
        Addr(at(c, S0, 0x1f54)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCStart_Init(OVScreenBCStart* self)` (0x1f73a8): create the
/// screen by name, start its thread and initialise the overlay state.
pub fn ov_screen_bc_start_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46f4_e0;
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_73cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_73e0)?;
        let t = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_73f8)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001f_7400)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenAutosave_Init(OVScreenAutosave* self)` (0x212198): the same
/// initialisation with the autosave screen's hash name.
pub fn ov_screen_autosave_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x471e_28;
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0021_21bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0021_21d0)?;
        let t = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0021_21e8)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0021_21f0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_dtor(FEModule* self, int flags)` (0x1a1d50): tear down the
/// two net hooks then the base front-end module.
pub fn fe_module_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x46d7_c0;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0xb5abc;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(at(c, S0, 0xb5abc));
        Addr(at(c, S0, 0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 2;
        call_ptr(c, 0x0026_39b0, 0x001a_1d88)?;
        c.g[A1][0] = 2;
        c.g[A0][0] = sx(at(c, S0, 0xb5ab0));
        call_ptr(c, 0x0026_3678, 0x001a_1d9c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0019_57d0, 0x001a_1da8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBraggingRights_SetRowsVisible(cFEState* self, int a1)`
/// (0x192740): tell up to eight child rows they are visible or not.
pub fn c_fe_state_bragging_rights_set_rows_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(at(c, A0, 8));
        frame.save_ra(c, 0);
        c.g[S1][0] = 7;
        Addr(at(c, A0, 0x28)).sw(c, c.g[S2][0]);
        c.g[A2][0] = Addr(at(c, S0, 0)).lw(c);
        loop {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = sx(at(c, S0, 4));
            let none = c.g[A2][0] == 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(at(c, A2, 8)).lw(c);
                c.g[A0][0] = Addr(at(c, V0, 0x40)).lh(c);
                c.g[V1][0] = Addr(at(c, V0, 0x44)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019_278c)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[A2][0] = Addr(at(c, S0, 0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_2768);
            }
        }
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
        c.g[V0][0] = Addr(at(c, S0, 0xc)).lw(c);
        let same = c.g[V0][0] == c.g[S1][0];
        c.g[S3][0] = c.g[A3][0] & 0xffff;
        if !same {
            c.cycles += 2;
            call_ptr(c, 0x0039_fd38, 0x0039_fd00)?;
        }
        c.cycles += 13;
        Addr(at(c, S0, 0x10)).sb(c, c.g[S2][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(at(c, S0, 0xc)).sw(c, c.g[S1][0]);
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

/// `void cFEStateCharSelect_Shutdown(cFEStateCharSelect* self)` (0x181238):
/// release the BE interface, hide the rider preview when riders exist and
/// shut the UI state down.
pub fn c_fe_state_char_select_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_124c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0018_1258)?;
        c.g[V1][0] = Addr(gpr(c, -0x848)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x7c)).lw(c);
        let no_riders = c.g[A0][0] == 0;
        c.g[V0][0] = 0x530000;
        let mut hide = false;
        if !no_riders {
            c.cycles += 3;
            c.g[V1][0] = Addr(at(c, V0, 0x5c11)).lb(c);
            hide = c.g[V1][0] != 0;
        }
        if hide {
            c.cycles += 6;
            c.g[A1][0] = Addr(at(c, S0, 0x44)).lb(c);
            c.g[A0][0] = sx(at(c, A0, 0xb0));
            call_ptr(c, 0x001a_0548, 0x0018_1280)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0019_e538, 0x0018_128c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e4a0, 0x0018_1294)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FELocale_LoadCommonTable(FELocale* self)` (0x195b70): map the
/// locale byte at +1 to a language index and load that table.
pub fn fe_locale_load_common_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        c.g[V1][0] = Addr(at(c, A0, 1)).lb(c);
        let byte = c.g[V1][0];
        c.g[A1][0] = u64::MAX;
        let mut lang = -1i64;
        if byte == 1 {
            // 0x195bbc falls into the tail that selects language 1.
            c.cycles += 2;
            lang = 1;
        } else if (byte as i64) < 2 {
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if byte == 0 {
                lang = 0;
            } else {
                c.cycles += 2;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.cycles += 2;
            c.g[V0][0] = 3;
            if byte == 2 {
                c.cycles += 1;
                lang = 2;
            } else {
                c.cycles += 2;
                if byte == 3 {
                    lang = 3;
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.g[A1][0] = sx(lang as u32);
        c.cycles += 5;
        call_ptr(c, 0x0019_59a8, 0x0019_5bd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CamController_Reset(CamController* self)` (0x15ccf0): reset every
/// camera node's override state and the current camera's chase/splash
/// managers.
pub fn cam_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S1, 8)).lw(c);
        c.g[S0][0] = Addr(at(c, V0, 0)).lw(c);
        let none = c.g[S0][0] == 0;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = Addr(at(c, S0, 0)).lw(c);
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(at(c, A1, 0x10)).lw(c);
                c.g[A0][0] = Addr(at(c, V0, 0x20)).lh(c);
                c.g[V1][0] = Addr(at(c, V0, 0x24)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0015_cd2c)?;
                c.g[S0][0] = Addr(at(c, S0, 0x14)).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(at(c, S0, 0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0015_cd18);
                }
            }
        }
        c.cycles += 9;
        c.g[A0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0015_e030, 0x0015_cd40)?;
        c.g[A0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x002f_41a8, 0x0015_cd48)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateRiderDetail_ctor(cFEStateRiderDetail* self, void* a1, int
/// index)` (0x183238): construct the char-setup base state and fill in the
/// rider-detail class and pad mask.
pub fn c_fe_state_rider_detail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0018_28c0, 0x0018_325c)?;
        c.g[V0][0] = 0xd;
        c.g[V1][0] = 0x46cd_90;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V0][0]);
        Addr(at(c, S0, 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(at(c, S0, 0x44)).sb(c, c.g[S1][0]);
        c.g[V0][0] = Addr(at(c, S0, 0x10)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0)).lw(c);
        call_ptr(c, 0x001a_1cd0, 0x0018_3284)?;
        Addr(at(c, S0, 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x4c)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* UIStateMgr_CloseTopStates(UIList* list, int count)` (0x39f190):
/// close (mark hidden) the first `count` UI states.
pub fn ui_state_mgr_close_top_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0039_7840, 0x0039_f1a4)?;
        c.g[A0][0] = c.g[V0][0];
        let none = (c.g[S0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        // The empty-list exit skips the one-cycle charge of the count block.
        let mut empty_exit = false;
        if !none {
            c.cycles += 2;
            c.g[A3][0] = 0xffff_ffff_ffff_c0ff;
            c.g[A2][0] = 0xffff_ffff_ffff_ff7f;
            loop {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    empty_exit = true;
                    break;
                }
                c.cycles += 10;
                c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | 0x600;
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0x80;
                Addr(at(c, A0, 0x1c)).sw(c, c.g[V0][0]);
                let more = c.g[V1][0] != 0;
                c.g[A0][0] = Addr(at(c, A0, 4)).lw(c);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_f1b8);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        if !empty_exit {
            c.cycles += 1;
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* FELocale_ctor(FELocale* self)` (0x195880): install both locale
/// v-tables, clear the flags and zero the 0x50-byte language table.
pub fn fe_locale_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x46d8_48;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0)).sb(c, 0);
        Addr(at(c, S0, 1)).sb(c, 0);
        Addr(at(c, S0, 2)).sb(c, 0);
        Addr(at(c, S0, 4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(at(c, S0, 3));
        call_ptr(c, 0x001d_cc40, 0x0019_58b0)?;
        c.g[V1][0] = 0x46bf_98;
        c.g[V0][0] = 4;
        Addr(at(c, S0, 0x58)).sb(c, 0);
        Addr(at(c, S0, 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(at(c, S0, 8));
        Addr(at(c, S0, 2)).sb(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x50;
        call_ptr(c, 0x0041_6210, 0x0019_58d8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenRewardsListPopup_Init(OVScreenRewardsListPopup* self)`
/// (0x200218): create the rewards-list popup screen and start its thread.
pub fn ov_screen_rewards_list_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x4704_b8;
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_023c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_0250)?;
        let t = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_0268)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0020_0270)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_OnMessengerPopupEvent(NetFEState* self, void* a1, int
/// event)` (0x1a9268): the messenger popup's confirm event allocates and
/// opens the online-character-select state; every other event is forwarded.
pub fn net_fe_state_on_messenger_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[A2][0] != c.g[V0][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_93d0, 0x001a_92c8)?;
        } else {
            c.cycles += 16;
            c.g[A2][0] = 0xf;
            call_ptr(c, 0x001a_93d0, 0x001a_9288)?;
            c.g[A1][0] = 0x461c_60;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x6e4;
            c.g[A2][0] = 0x100;
            call_ptr(c, 0x0031_7d70, 0x001a_92a0)?;
            c.g[A1][0] = Addr(at(c, S0, 0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_0f60, 0x001a_92ac)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001a_92b8)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_ShowWaitPlayerPopup(NetFEState* self)` (0x1ab108):
/// configure and show the "waiting for players" popup.
pub fn net_fe_state_show_wait_player_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x001a_9c50, 0x001a_b120)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x836;
        Addr(at(c, S0, 0xbc)).sw(c, 0);
        c.g[V0][0] = 6;
        Addr(at(c, S0, 0x170)).sw(c, 0);
        c.g[A0][0] = 0x4627_50;
        Addr(at(c, S0, 0x150)).sw(c, c.g[V1][0]);
        Addr(at(c, S0, 0x14c)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001a_b148)?;
        c.g[V1][0] = 3;
        Addr(at(c, S0, 0x164)).sw(c, 0);
        Addr(at(c, S0, 0xc4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(at(c, S0, 0xc0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001a_9bc0, 0x001a_b164)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengePopup_v32(cFEStateChallengePopup* self)` (0x1aebf0):
/// ensure the buddy manager exists and is connected, disconnect the lobby
/// when it vanished, and publish the player's presence.
pub fn c_fe_state_challenge_popup_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gpr(c, 0x238)).lw(c);
        let exists = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !exists {
            c.cycles += 2;
            call_ptr(c, 0x0026_0f80, 0x001a_ec08)?;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr(gpr(c, 0x238)).lw(c);
        call_ptr(c, 0x0026_1460, 0x001a_ec10)?;
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gpr(c, 0x238)).lw(c);
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = Addr(gpr(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_13c8, 0x001a_ec20)?;
            c.g[V0][0] = Addr(gpr(c, 0x238)).lw(c);
        }
        c.cycles += 2;
        let have_buddy = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gpr(c, -0xc8)).lw(c);
        if !have_buddy {
            c.cycles += 3;
            c.g[A0][0] = Addr(gpr(c, -0xc8)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001a_ec34)?;
            c.g[V0][0] = Addr(gpr(c, -0xc8)).lw(c);
        }
        c.cycles += 10;
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(gpr(c, 0x238)).lw(c);
        c.g[A2][0] = 7;
        c.g[A3][0] = Addr(at(c, V0, 0x98)).lw(c);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, 0x0026_2768, 0x001a_ec54)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengeReceivedPopup_TransitionIn(cFEState* self, void*
/// a1)` (0x1b0670): finish the popup transition and, when a challenge is
/// given, fill in its text.
pub fn c_fe_state_challenge_received_popup_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S1, 8)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x100)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x104)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c, S1, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001b_069c)?;
        let none = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !none {
            c.cycles += 10;
            call_ptr(c, 0x0014_dc80, 0x001b_06ac)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x001b_06b8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = Addr(at(c, A0, 0x40)).lw(c);
            call_ptr(c, 0x001b_0850, 0x001b_06c8)?;
        }
        c.cycles += 5;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateCreateStep1_ctor(cFEStateCreateStep1* self, void* a1, int
/// a2)` (0x1b88f0): construct the net front-end state and install the
/// create-step-1 class and mode.
pub fn c_fe_state_create_step1_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        call_ptr(c, 0x001a_8500, 0x001b_8910)?;
        c.g[V1][0] = Addr(gpr(c, 0xda0)).lw(c);
        c.g[V0][0] = 0x4686_c0;
        Addr(at(c, S0, 0x6f0)).sw(c, 0);
        Addr(at(c, S0, 8)).sw(c, c.g[V0][0]);
        Addr(at(c, S0, 0x6dc)).sw(c, c.g[V1][0]);
        Addr(at(c, S0, 0x6d4)).sw(c, c.g[V1][0]);
        Addr(at(c, S0, 0x6d8)).sw(c, c.g[V1][0]);
        let none = c.g[S1][0] == 0;
        c.g[V0][0] = 0x50;
        if !none {
            c.cycles += 1;
            c.g[V0][0] = 0x4d;
        }
        c.cycles += 9;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V0][0]);
        Addr(at(c, S0, 0x6ec)).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x6a8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateStep1_SetSelection(cFEState* self, u32 name_hash,
/// u32 field)` (0x1b9740): select the list-box item matching the state's
/// boolean flag at `field`.
fn create_step1_selection(
    c: &mut Cpu,
    name: u32,
    field: u32,
    ret_str: u32,
    ret_find: u32,
    ret_find2: u32,
) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    c.g[A0][0] = sx(name);
    frame.save_ra(c, 0x10);
    call_ptr(c, 0x0031_7670, ret_str)?;
    c.g[A0][0] = Addr(at(c, S1, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)?;
    c.g[S0][0] = c.g[V0][0];
    let none = c.g[S0][0] == 0;
    c.g[A2][0] = c.g[SP][0];
    if !none {
        c.cycles += 9;
        c.g[V0][0] = Addr(at(c, S1, 0x6d0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(at(c, SP, 0)).sb(c, 0);
        c.g[A1][0] = Addr(at(c, V0, field)).lw(c);
        c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
        call_ptr(c, 0x0039_a768, ret_find2)?;
        c.g[A1][0] = Addr(at(c, SP, 0)).lbu(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_a7a8, ret_find2 + 0xc)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void cFEStateCreateStep1_SetInformSelection(cFEState* self)` (0x1b9740).
pub fn c_fe_state_create_step1_set_inform_selection(c: &mut Cpu) {
    guest(c, |c| {
        create_step1_selection(c, 0x4647_80, 0x38, 0x001b_9760, 0x001b_976c, 0x001b_9790)
    })
}

/// `void cFEStateCreateStep1_SetShareInfoSelection(cFEState* self)`
/// (0x1b97b0).
pub fn c_fe_state_create_step1_set_share_info_selection(c: &mut Cpu) {
    guest(c, |c| {
        create_step1_selection(c, 0x4647_90, 0x3c, 0x001b_97d0, 0x001b_97dc, 0x001b_9800)
    })
}

/// `void cFEStateCreateStep2_SetLegalSelection(cFEState* self)` (0x1bb4b0).
pub fn c_fe_state_create_step2_set_legal_selection(c: &mut Cpu) {
    guest(c, |c| {
        create_step1_selection(c, 0x464a_08, 0x40, 0x001b_b4d0, 0x001b_b4dc, 0x001b_b500)
    })
}

/// `void cFEStateCreateDuplicate_SubmitAccount(cFEState* self)` (0x1bc290):
/// configure and show the account-exists popup, then create the account.
pub fn c_fe_state_create_duplicate_submit_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x001a_9c50, 0x001b_c2a8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x464a_38;
        Addr(at(c, S0, 0xbc)).sw(c, 0);
        call_ptr(c, 0x0031_7670, 0x001b_c2bc)?;
        c.g[V1][0] = 3;
        c.g[A2][0] = 0x820;
        Addr(at(c, S0, 0x164)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        Addr(at(c, S0, 0xc4)).sw(c, c.g[V0][0]);
        Addr(at(c, S0, 0xc0)).sw(c, c.g[V1][0]);
        Addr(at(c, S0, 0x150)).sw(c, c.g[A2][0]);
        call_ptr(c, 0x001a_9bc0, 0x001b_c2e0)?;
        c.g[A0][0] = Addr(gpr(c, -0xc8)).lw(c);
        c.g[A1][0] = sx(at(c, A0, 0x260));
        call_ptr(c, 0x0025_b9c8, 0x001b_c2ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLobbyMenu_ClampRoomSelections(cFEState* self)` (0x1be038):
/// keep each of the four room-list selections inside its item count.
pub fn c_fe_state_lobby_menu_clamp_room_selections(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(at(c, A0, 0x708));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V1][0] = Addr(at(c, S0, 0x48)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x318)).lbu(c);
            c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
            let in_range = c.g[V1][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            if !in_range {
                c.cycles += 1;
                Addr(at(c, S0, 0x48)).sw(c, c.g[A0][0]);
            }
            c.cycles += 4;
            c.g[V1][0] = Addr(at(c, S0, 0)).lw(c);
            c.g[V0][0] = Addr(at(c, V1, 0x318)).lbu(c);
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V1][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = Addr(at(c, S0, 0x48)).lbu(c);
                call_ptr(c, 0x0039_a7a8, 0x001b_e088)?;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < 0x4u64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_e050);
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

/// `void cFEStateNetNews_Init(cFEStateNetNews* self)` (0x1c5098): create the
/// net-news screen, start its thread and request the news list.
pub fn c_fe_state_net_news_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x4a1c_30;
        c.g[S1][0] = Addr(at(c, S0, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001c_50bc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001c_50d0)?;
        let t = c.g[V0][0] == 0;
        Addr(at(c, S0, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 7;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_50e8)?;
            c.g[A0][0] = Addr(gpr(c, -0xc8)).lw(c);
            c.g[A1][0] = 1;
            call_ptr(c, 0x0025_e0c8, 0x001c_50f4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MessageCenter_OnBCFinish(MessageCenter* self, int key, void* a2)`
/// (0x1e25d8): when no broadcast is running, dispatch the finish message by
/// key (4/5).
pub fn message_center_on_bc_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gpr(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, V0, 0x84)).lw(c);
        c.g[V0][0] = Addr(at(c, V1, 0x28)).lw(c);
        c.g[V0][0] = Addr(at(c, V0, 0)).lw(c);
        let no_bc = c.g[V0][0] == 0;
        c.g[A3][0] = 0;
        if !no_bc {
            c.cycles += 1;
            c.g[A3][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        let busy = c.g[A3][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            let mut key4 = false;
            c.cycles += 3;
            c.g[V0][0] = 4;
            key4 = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 5;
            if !key4 {
                c.cycles += 2;
                let key5 = c.g[A0][0] == c.g[V0][0];
                c.g[A0][0] = c.g[A1][0];
                if key5 {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[A2][0];
                    call_ptr(c, 0x001e_2828, 0x001e_263c)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                }
            }
            if key4 {
                c.cycles += 5;
                c.g[A0][0] = c.g[A1][0];
                c.g[A1][0] = c.g[A2][0];
                call_ptr(c, 0x001e_2648, 0x001e_262c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void IntcTable_AddHandler(IntcTable* self, int cause, void* handler,
/// void* arg)` (0x361f98): register an interrupt handler under the table
/// lock.
pub fn intc_table_add_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0042_4880, 0x0036_1fc8)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0042_3a80, 0x0036_1fd8)?;
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(at(c, S1, 0)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0042_48e8, 0x0036_1fec)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopTrickMeterSound(AudioSystem* self, void*
/// rider)` (0x29b3c0): stop the trick-meter handle when the rider is gone or
/// finished.
pub fn ssx_audio_system_stop_trick_meter_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_rider = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        let run;
        if no_rider {
            run = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(at(c, A1, 0x874)).lw(c);
            let gone = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !gone {
                c.cycles += 2;
                c.g[V0][0] = Addr(at(c, A1, 0x87c)).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            run = true;
        }
        if run {
            c.cycles += 4;
            c.g[A1][0] = Addr(at(c, S0, 0x5fdc)).lw(c);
            c.g[S1][0] = u64::MAX;
            let same = c.g[A1][0] == c.g[S1][0];
            c.g[A2][0] = 1;
            if !same {
                c.cycles += 6;
                c.g[V0][0] = Addr(at(c, S0, 0x118)).lw(c);
                c.mtc1(0, 12);
                c.g[A0][0] = Addr(at(c, V0, 0)).lw(c);
                c.g[A0][0] = sx(at(c, A0, 0x1d8));
                call_ptr(c, 0x002a_d5f0, 0x0029_b418)?;
                Addr(at(c, S0, 0x5fdc)).sw(c, c.g[S1][0]);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsTrackGroup1Event(AudioSystem* self, void* info)`
/// (0x2a40e8): whether the current track is in group 1, or the sound's
/// configured event is the group-1 event flag.
pub fn ssx_audio_system_is_track_group1_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x002a_40fc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x002a_4108)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_4110)?;
        c.g[A1][0] = Addr(at(c, V0, 0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4158, 0x002a_411c)?;
        let group1 = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        let mut result = 1u64;
        if !group1 {
            c.cycles += 5;
            c.g[V0][0] = 0x530000;
            c.g[A0][0] = 1;
            c.g[V1][0] = Addr(at(c, V0, 0x5c10)).lb(c);
            let is_one = c.g[V1][0] == c.g[A0][0];
            c.g[V1][0] = c.g[V1][0] ^ 6;
            if is_one {
                c.cycles += 1;
                result = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                result = c.g[V0][0];
            }
        }
        c.g[V0][0] = result;
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_FreeSampleList(SampleList* list)` (0x2a4cf8): free every
/// sample pointer in the list and clear it.
pub fn crowd_audio_free_sample_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S2, 0)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(at(c, S2, 4));
            loop {
                c.cycles += 3;
                c.g[A0][0] = Addr(at(c, S0, 0)).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(at(c, S0, 0)).sw(c, 0);
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0031_7e98, 0x002a_4d34)?;
                    Addr(at(c, S0, 0)).sw(c, 0);
                }
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(at(c, S2, 0)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_4d20);
                }
            }
        }
        c.cycles += 7;
        Addr(at(c, S2, 0)).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int IniParser_FindSection(IniParser* self, const char* name)` (0x2a7bf8):
/// walk the parser's sections and return whether one matches `name`
/// (case-insensitively). Its loop head is a resume entry after an event.
pub fn ini_parser_find_section(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        #[derive(Clone, Copy)]
        enum Stage {
            Head,
            Check,
            Get,
            Cmp,
        }
        let mut st = match c.pc {
            0x002a_7bf8 => {
                c.cycles += 7;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
                c.sq(16, frame.at(c, 0x20));
                c.sq(17, frame.at(c, 0x10));
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                Stage::Head
            }
            0x002a_7c14 => {
                c.cycles += 1;
                Stage::Cmp
            }
            0x002a_7c18 => Stage::Cmp,
            _ => {
                c.jump_out(c.pc, 0x002a_7bf8, 0x002a_7c68);
                return Ok(());
            }
        };
        let mut result = 0u64;
        loop {
            match st {
                Stage::Head => {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(at(c, S0, 8)).lw(c);
                    st = Stage::Check;
                }
                Stage::Check => {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(at(c, V0, 0)).lb(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[V0][0] = 0;
                    if empty {
                        result = 0;
                        break;
                    }
                    st = Stage::Get;
                }
                Stage::Get => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_7b08, 0x002a_7c48)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        result = 0;
                        break;
                    }
                    c.g[A0][0] = c.g[V0][0];
                    if event_due(c) && back_edge_unwinds(c, 0x002a_7c18) {
                        return Ok(());
                    }
                    st = Stage::Cmp;
                }
                Stage::Cmp => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041_aa88, 0x002a_7c20)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(at(c, S0, 8)).lw(c);
                        st = Stage::Check;
                    } else {
                        c.cycles += 2;
                        result = 1;
                        break;
                    }
                }
            }
        }
        c.g[V0][0] = result;
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP, RA as SENTINEL_RA, SP as STACK_TOP};
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    /// The main heap's dlmalloc arena over test RAM.
    const MEM_HEAP: u32 = 0x004f_f250;
    /// The test world / race manager / BE singleton live high in `DATA`.
    const W: u32 = DATA + 0x3d_0000;
    const RM: u32 = DATA + 0x3d_0100;
    const BE: u32 = DATA + 0x3d_2000;

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// A machine with a plausible game `gp`: the BE singleton, the world,
    /// race manager and frame counter.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP - 0x1f04, BE);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(GAME_GP - 0x848, W);
        c.bus.write32(W + 0x84, RM);
        c.bus.write32(RM + 0xc, DATA + 0x3d_0200);
        c.bus.write32(DATA + 0x3d_0200 + 8, r.next_u32());
        c.bus.write32(RM + 0x7c, 0);
        c.bus.write32(RM + 0x64, 0);
        c.bus.write32(RM + 0x84, 0);
        r
    }

    /// [`base`] plus a working pool allocator and dlmalloc arena.
    fn alloc_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base(c, seed, salt);
        pool_allocator(c);
        heap_init(c);
        r
    }

    /// A real heap block of `size` bytes.
    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    /// A v-table at `at` with `{delta = 0, fake}` at each slot.
    fn put_vtable(c: &mut Cpu, at: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(at + off, 0);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    fn put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
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
    fn rider_set_timed_event_matches() {
        check(
            ee::s00::f_0010e028__generated,
            rider_set_timed_event,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x10e0_28);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
                c.f[12] = if seed % 3 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(5.0)
                };
            },
        );
    }

    #[test]
    fn script_obj_dtor_matches() {
        check(ee::s07::f_00283228__generated, script_obj_dtor, 80, &|c, seed| {
            alloc_base(c, seed, 0x2832_28);
            let se = DATA + 0x2000;
            let vt = DATA + 0x2200;
            c.bus.write32(GAME_GP + 0x3ec, se);
            c.bus.write32(se + 0x2a8, vt);
            put_vtable(c, vt, &[0x18]);
            let obj = alloc(c, 0x30);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = [0u64, 2, 1][seed as usize % 3];
            c.bus
                .write32(obj + 4, [0u32, u32::MAX, 1, 5][seed as usize % 4]);
        });
    }

    #[test]
    fn lens_fx_base_dtor_matches() {
        check(ee::s09::f_002e3060__generated, lens_fx_base_dtor, 40, &|c, seed| {
            alloc_base(c, seed, 0x2e30_60);
            let obj = alloc(c, 0x6500);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = [0u64, 2, 1][seed as usize % 3];
            let man = DATA + 0x1000;
            let vt = DATA + 0x1200;
            c.bus.write32(GAME_GP + 0x2a90, man);
            c.bus.write32(man + 0x10d8, vt);
            put_vtable(c, vt, &[0x3a8]);
            c.bus.write32(GAME_GP + 0xa28, 0x1122_3344);
        });
    }

    #[test]
    fn be_new_player_int_share_slot0_char_record_matches() {
        check(
            ee::s01::f_00147828__generated,
            be_new_player_int_share_slot0_char_record,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1478_28);
                let index = seed % 4;
                let slot = (seed / 4) % 3;
                c.g[A0][0] = sx(index);
                c.bus.write32(0x5305_b0 + index * 4, slot);
                let entry = 0x534f_e0 + slot * 0x1c;
                c.bus.write32(0x534f_e0 + 0x11, r.next_u32());
                c.bus.write32(entry + 0x10, r.next_u32());
                c.bus.write8(entry + 0x11, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn anim_object_ctor_2_matches() {
        check(ee::s11::f_00341c80__generated, anim_object_ctor_2, 60, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x341c_80);
            let obj = DATA + 0x1000;
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = 0;
            c.bus.write32(STACK_TOP - 0xd0, 0);
        });
    }

    #[test]
    fn anim_teeter_ctor_2_matches() {
        check(ee::s11::f_003422e8__generated, anim_teeter_ctor_2, 60, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x3422_e8);
            let obj = DATA + 0x1000;
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = 0;
            c.bus.write32(STACK_TOP - 0xd0, 0);
        });
    }

    #[test]
    fn bolts_index_bolts_by_char_matches() {
        check(
            ee::s01::f_0014c2b0__generated,
            bolts_index_bolts_by_char,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x14c2_b0);
                let count = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x1c, count);
                c.bus.write32(DATA + 4, DATA + 0x1000);
                for i in 0..count {
                    c.bus
                        .write8(DATA + 0x1000 + i * 0x38, (r.next_u32() % 3) as u8);
                }
                fill(c, &mut r, DATA + 0x2c, 4);
                fill(c, &mut r, DATA + 0xa4, 4);
            },
        );
    }

    #[test]
    fn script_list_get_entry_type_matches() {
        check(
            ee::s07::f_00276998__generated,
            script_list_get_entry_type,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2769_98);
                let list = DATA + 0xc;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 7) as u64;
                c.bus
                    .write32(list + 0x94, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write32(list + 0x8c, (seed % 5) as u32);
                c.bus.write32(list + 0x90, 5);
                for slot in 0..5u32 {
                    let v = if seed % 3 == 0 {
                        r.next_u32() % 64
                    } else {
                        (r.next_u32() % 30) + 2
                    };
                    c.bus.write32(list + slot * 0x1c, v);
                }
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
    fn cbx_string_assign_copy_matches() {
        check(
            ee::s10::f_003183d8__generated,
            cbx_string_assign_copy,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x3183_d8);
                let len = 4 + (seed % 9);
                let buf = DATA + 0x1000;
                let empty = 0x0048_dcd4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, if seed % 2 == 0 { buf } else { empty });
                for b in [buf, empty] {
                    c.bus.write32(b - 0xc, 2);
                    c.bus.write32(b - 4, 0x100);
                }
                c.g[A1][0] = len as u64;
                c.g[A2][0] = sx(DATA + 0x400);
                fill(c, &mut r, DATA + 0x400, 8);
            },
        );
    }

    #[test]
    fn cbx_string_release_buffer_matches() {
        check(
            ee::s10::f_00318a88__generated,
            cbx_string_release_buffer,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x318a_88);
                let buf = DATA + 0x1000;
                let empty = 0x0048_dcd4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, if seed % 2 == 0 { buf } else { empty });
                for b in [buf, empty] {
                    c.bus.write32(b - 0xc, 2);
                    c.bus.write32(b - 4, 0x100);
                }
                put_str(c, buf, b"hello\0");
                c.g[A1][0] = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (seed % 12) as u64
                };
                fill(c, &mut r, DATA + 0x400, 4);
            },
        );
    }

    #[test]
    fn replay_free_buffers_matches() {
        check(ee::s07::f_0026f428__generated, replay_free_buffers, 60, &|c, seed| {
            let mut r = base(c, seed, 0x26f4_28);
            let count = seed % 4;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 8, count);
            for i in 0..count {
                let e = DATA + 0x2c + i * 0x38;
                c.bus.write32(e + 4, 0);
                fill(c, &mut r, e, 0xe);
            }
            fill(c, &mut r, DATA + 0x3b0, 3);
            fill(c, &mut r, DATA + 0x3bc, 3);
        });
    }

    #[test]
    fn fe_module_render_rider_previews_matches() {
        check(
            ee::s03::f_001a0598__generated,
            fe_module_render_rider_previews,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1a05_98);
                let canvas = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x854, canvas);
                fill(c, &mut r, canvas, 0x400);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xcc8, 0);
                c.bus.write32(DATA + 0xce0 + 0xcc8, 0);
            },
        );
    }

    #[test]
    fn c_ui_group_v18_matches() {
        check(ee::s13::f_003987f8__generated, c_ui_group_v18, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3987_f8, 0);
            let n0 = DATA + 0x1000;
            let n1 = DATA + 0x1200;
            let obj = DATA + 0x1300;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = r.next_u32() as u64;
            c.bus.write32(DATA + 0x78, n0);
            if seed % 2 == 0 {
                c.bus.write32(n0, n0);
                c.bus.write32(n0 + 4, n0);
            } else {
                c.bus.write32(n0, 0);
                c.bus.write32(n0 + 4, n1);
                c.bus.write32(n0 + 8, obj);
                c.bus.write32(n1, n1);
                c.bus.write32(n1 + 4, n1);
                put_vtable(c, obj, &[0x90]);
            }
            c.pc = 0x0039_87f8;
        });
    }

    #[test]
    fn particle_modifier_dtor_matches() {
        check(
            ee::s11::f_00345ad0__generated,
            particle_modifier_dtor,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x345a_d0);
                let obj = DATA + 0x1000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let buf = if seed % 2 == 0 { 0 } else { alloc(c, 0x20) };
                c.bus.write32(obj + 0x1e0, buf);
                c.bus.write32(obj, 0);
                c.bus.write32(obj + 4, 0);
                c.bus.write32(GAME_GP + 0xf08, rng_word(seed));
            },
        );
    }

    #[test]
    fn dynamic_particle_modifier_dtor_matches() {
        check(
            ee::s11::f_00345e88__generated,
            dynamic_particle_modifier_dtor,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x345e_88);
                let obj = DATA + 0x1000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
                let buf = if seed % 2 == 0 { 0 } else { alloc(c, 0x20) };
                c.bus.write32(obj + 0x260, buf);
                c.bus.write32(obj, 0);
                c.bus.write32(obj + 4, 0);
                c.bus.write32(GAME_GP + 0xf0c, rng_word(seed));
            },
        );
    }

    fn rng_word(seed: u32) -> u32 {
        0x9e37_79b9u32.wrapping_mul(seed.wrapping_add(1))
    }

    #[test]
    fn multi_spline_modifier_v19_matches() {
        check(
            ee::s11::f_0035aaf0__generated,
            multi_spline_modifier_v19,
            60,
            &|c, seed| {
                machine(c, seed, 0x35aa_f0, 0);
                let obj = DATA + 0x1000;
                let item = DATA + 0x2000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(item);
                let spline = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                c.bus.write32(obj + 0x30, spline);
                if spline != 0 {
                    put_vtable(c, spline, &[0x18]);
                }
                c.bus
                    .write32(obj + 0x40, if seed % 2 == 0 { item } else { 0 });
                let a1 = DATA + 0x2400;
                c.bus.write32(item + 0xc, a1);
                put_vtable(c, a1, &[0xc0]);
            },
        );
    }

    #[test]
    fn ov_screen_session_hide_map_pic_matches() {
        check(
            ee::s05::f_00208bb8__generated,
            ov_screen_session_hide_map_pic,
            60,
            &|c, seed| {
                machine(c, seed, 0x208b_b8, 0);
                c.g[A0][0] = sx(DATA);
                let a = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
                let b = if seed % 2 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(DATA + 0xd0, a);
                c.bus.write32(DATA + 0xe0, b);
                if a != 0 {
                    put_vtable(c, a, &[0x48]);
                }
                if b != 0 {
                    put_vtable(c, b, &[0x48]);
                }
            },
        );
    }

    #[test]
    fn ov_screen_session_post_input_matches() {
        check(
            ee::s05::f_00209300__generated,
            ov_screen_session_post_input,
            60,
            &|c, seed| {
                machine(c, seed, 0x2093_00, 0);
                let obj = DATA + 0x300;
                let vt = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 8, vt);
                put_vtable(c, vt, &[0x88, 0x90]);
                // The two status methods are answered by
                // UIListNode_IsSentinel (a real function returning 0/1), so
                // the zero-result branches are reachable.
                c.bus.write32(vt + 0x88, 0);
                c.bus.write16(vt + 0x88, 0);
                c.bus.write32(vt + 0x8c, 0x0039_7718);
                c.bus.write32(vt + 0x94, 0x0039_7718);
                if seed % 3 == 0 {
                    c.bus.write32(obj + 4, obj);
                } else {
                    c.bus.write32(obj, 0);
                    c.bus.write32(obj + 4, 0);
                }
                if seed % 3 == 2 {
                    c.bus.write16(vt + 0x90, 0x40);
                    let n = obj + 0x40;
                    c.bus.write32(n, n);
                    c.bus.write32(n + 4, n);
                }
                c.bus.write32(DATA + 0xe0, 0);
                c.bus.write32(DATA + 0xd4, 0);
                c.bus.write32(DATA + 0xc0, 0);
            },
        );
    }

    #[test]
    fn tex_man_shutdown_matches() {
        check(ee::s12::f_00367360__generated, tex_man_shutdown, 24, &|c, seed| {
            let mut r = base(c, seed, 0x3673_60);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x4320, u32::MAX);
            c.bus.write32(DATA + 0x6710, u32::MAX);
            fill(c, &mut r, DATA + 0x1f50, 4);
        });
    }

    #[test]
    fn ov_screen_bc_start_init_matches() {
        check(
            ee::s04::f_001f73a8__generated,
            ov_screen_bc_start_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x1f73_a8);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }

    #[test]
    fn ov_screen_autosave_init_matches() {
        check(
            ee::s05::f_00212198__generated,
            ov_screen_autosave_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x2121_98);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }

    #[test]
    fn fe_module_dtor_matches() {
        check(ee::s03::f_001a1d50__generated, fe_module_dtor, 24, &|c, seed| {
            alloc_base(c, seed, 0x1a1d_50);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 2][seed as usize % 2];
            c.bus.write32(DATA + 0xc, 0);
            c.bus.write32(GAME_GP - 0x854, 0);
            c.bus.write32(DATA + 0xb5aa0, 0);
        });
    }

    #[test]
    fn c_fe_state_bragging_rights_set_rows_visible_matches() {
        check(
            ee::s02::f_00192740__generated,
            c_fe_state_bragging_rights_set_rows_visible,
            60,
            &|c, seed| {
                machine(c, seed, 0x1927_40, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                for i in 0..8u32 {
                    let p = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x1000 + i * 0x100
                    };
                    c.bus.write32(DATA + 8 + i * 4, p);
                    if p != 0 {
                        put_vtable(c, p, &[0x40]);
                    }
                }
            },
        );
    }

    #[test]
    fn ui_object_set_anim_matches() {
        check(ee::s13::f_0039fcc8__generated, ui_object_set_anim, 60, &|c, seed| {
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
        });
    }

    #[test]
    fn c_fe_state_char_select_shutdown_matches() {
        check(
            ee::s02::f_00181238__generated,
            c_fe_state_char_select_shutdown,
            40,
            &|c, seed| {
                base(c, seed, 0x1812_38);
                let iface = DATA + 0x3000;
                let vt = DATA + 0x3200;
                c.bus.write32(0x5308_a8, iface);
                c.bus.write32(iface + 4, vt);
                put_vtable(c, vt, &[8]);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(W + 0x7c, 0);
            },
        );
    }

    #[test]
    fn fe_locale_load_common_table_matches() {
        check(
            ee::s02::f_00195b70__generated,
            fe_locale_load_common_table,
            40,
            &|c, seed| {
                base(c, seed, 0x195b_70);
                c.g[A0][0] = sx(DATA);
                c.bus.write8(DATA + 1, (seed % 6) as u8);
                for lang in 0..4u32 {
                    c.bus.write32(DATA + 8 + lang * 4, 1);
                }
            },
        );
    }

    #[test]
    fn cam_controller_reset_matches() {
        check(ee::s01::f_0015ccf0__generated, cam_controller_reset, 50, &|c, seed| {
            machine(c, seed, 0x15cc_f0, 0);
            c.g[A0][0] = sx(DATA);
            let n0 = DATA + 0x1000;
            let n1 = if seed % 2 == 0 { 0 } else { DATA + 0x1200 };
            c.bus.write32(DATA + 8, n0);
            c.bus
                .write32(n0, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
            c.bus.write32(n0 + 0x14, n1);
            if n1 != 0 {
                c.bus.write32(n1, DATA + 0x2200);
                c.bus.write32(n1 + 0x14, 0);
            }
            for p in [DATA + 0x2000u32, DATA + 0x2200] {
                c.bus.write32(p + 0x10, p + 0x100);
                put_vtable(c, p + 0x100, &[0x20]);
            }
            c.bus.write32(DATA + 0x10, 0);
        });
    }

    #[test]
    fn c_fe_state_rider_detail_ctor_matches() {
        check(
            ee::s02::f_00183238__generated,
            c_fe_state_rider_detail_ctor,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1832_38);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (1 + seed % 4) as u64;
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn ui_state_mgr_close_top_states_matches() {
        check(
            ee::s13::f_0039f190__generated,
            ui_state_mgr_close_top_states,
            60,
            &|c, seed| {
                base(c, seed, 0x39f1_90);
                let sentinel = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(DATA + 4, sentinel);
                if seed % 3 == 0 {
                    c.bus.write32(sentinel, sentinel);
                    c.bus.write32(sentinel + 4, sentinel);
                } else {
                    let n1 = DATA + 0x1100;
                    let n2 = DATA + 0x1200;
                    c.bus.write32(sentinel, sentinel);
                    c.bus.write32(sentinel + 4, n1);
                    c.bus.write32(n1, n1);
                    c.bus.write32(n1 + 4, n2);
                    c.bus.write32(n1 + 0x1c, 0x1234_5678);
                    c.bus.write32(n2, n2);
                    c.bus.write32(n2 + 4, sentinel);
                    c.bus.write32(n2 + 0x1c, 0x9abc_def0);
                }
            },
        );
    }

    #[test]
    fn fe_locale_ctor_matches() {
        check(ee::s02::f_00195880__generated, fe_locale_ctor, 30, &|c, seed| {
            base(c, seed, 0x1958_80);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn ov_screen_rewards_list_popup_init_matches() {
        check(
            ee::s05::f_00200218__generated,
            ov_screen_rewards_list_popup_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x2002_18);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }

    #[test]
    fn net_fe_state_on_messenger_popup_event_matches() {
        check(
            ee::s03::f_001a9268__generated,
            net_fe_state_on_messenger_popup_event,
            30,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x1a92_68);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 0xf) as u64
                };
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn net_fe_state_show_wait_player_popup_matches() {
        check(
            ee::s03::f_001ab108__generated,
            net_fe_state_show_wait_player_popup,
            20,
            &|c, seed| {
                alloc_base(c, seed, 0x1ab1_08);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_popup_v32_matches() {
        check(
            ee::s03::f_001aebf0__generated,
            c_fe_state_challenge_popup_v32,
            20,
            &|c, seed| {
                alloc_base(c, seed, 0x1aeb_f0);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP + 0x238, 0);
                c.bus.write32(GAME_GP - 0xc8, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_transition_in_matches() {
        check(
            ee::s03::f_001b0670__generated,
            c_fe_state_challenge_received_popup_transition_in,
            30,
            &|c, seed| {
                machine(c, seed, 0x1b06_70, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 8, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_ctor_matches() {
        check(
            ee::s03::f_001b88f0__generated,
            c_fe_state_create_step1_ctor,
            24,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x1b88_f0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }

    /// Set up a step-1 list-box lookup for `name_addr` and the boolean field.
    fn selection_setup(c: &mut Cpu, name_addr: u32, field: u32) {
        let screen = DATA + 0x1000;
        let cnt = DATA + 0x2000;
        let arr = DATA + 0x2100;
        let item = DATA + 0x2200;
        let flags = DATA + 0x2300;
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + 0x40, screen);
        c.bus.write32(DATA + 0x6d0, flags);
        c.bus.write32(flags + field, 1);
        c.bus.write32(screen + 0x38, cnt);
        c.bus.write32(screen + 0x3c, arr);
        c.bus.write32(cnt, 1);
        c.bus.write32(arr, item);
        let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[name_addr as u64]) as u32;
        c.bus.write32(item + 0x38, hash);
        c.bus.write8(item + 0x318, 3);
        for i in 0..3u32 {
            c.bus.write32(item + 0xa4 + i * 0x14, i);
        }
    }

    #[test]
    fn c_fe_state_create_step1_set_inform_selection_matches() {
        check(
            ee::s03::f_001b9740__generated,
            c_fe_state_create_step1_set_inform_selection,
            40,
            &|c, seed| {
                base(c, seed, 0x1b97_40);
                if seed % 2 == 0 {
                    c.g[A0][0] = sx(DATA);
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    selection_setup(c, 0x0046_4780, 0x38);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_set_share_info_selection_matches() {
        check(
            ee::s03::f_001b97b0__generated,
            c_fe_state_create_step1_set_share_info_selection,
            40,
            &|c, seed| {
                base(c, seed, 0x1b97_b0);
                if seed % 2 == 0 {
                    c.g[A0][0] = sx(DATA);
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    selection_setup(c, 0x0046_4790, 0x3c);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_create_step2_set_legal_selection_matches() {
        check(
            ee::s03::f_001bb4b0__generated,
            c_fe_state_create_step2_set_legal_selection,
            40,
            &|c, seed| {
                base(c, seed, 0x1bb4_b0);
                if seed % 2 == 0 {
                    c.g[A0][0] = sx(DATA);
                    c.bus.write32(DATA + 0x40, 0);
                } else {
                    selection_setup(c, 0x0046_4a08, 0x40);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_create_duplicate_submit_account_matches() {
        check(
            ee::s03::f_001bc290__generated,
            c_fe_state_create_duplicate_submit_account,
            16,
            &|c, seed| {
                alloc_base(c, seed, 0x1bc2_90);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP - 0xc8, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_lobby_menu_clamp_room_selections_matches() {
        check(
            ee::s03::f_001be038__generated,
            c_fe_state_lobby_menu_clamp_room_selections,
            50,
            &|c, seed| {
                base(c, seed, 0x1be0_38);
                c.g[A0][0] = sx(DATA);
                for i in 0..4u32 {
                    let row = DATA + 0x1000 + i * 0x100;
                    c.bus.write32(DATA + 0x708 + i * 4, row);
                    let count = (seed + i) % 4;
                    c.bus.write8(row + 0x318, count as u8);
                    c.bus.write32(row + 0x48, count + 2);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_net_news_init_matches() {
        check(
            ee::s03::f_001c5098__generated,
            c_fe_state_net_news_init,
            20,
            &|c, seed| {
                alloc_base(c, seed, 0x1c50_98);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(GAME_GP - 0xc8, 0);
            },
        );
    }

    #[test]
    fn message_center_on_bc_finish_matches() {
        check(
            ee::s04::f_001e25d8__generated,
            message_center_on_bc_finish,
            60,
            &|c, seed| {
                base(c, seed, 0x1e25_d8);
                let p = DATA + 0x2000;
                c.bus.write32(RM + 0x28, p);
                c.bus.write32(p, [0u32, 5, 15][seed as usize % 3]);
                c.g[A0][0] = (6 + seed % 3) as u64;
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3400);
            },
        );
    }

    #[test]
    fn intc_table_add_handler_matches() {
        check(
            ee::s12::f_00361f98__generated,
            intc_table_add_handler,
            40,
            &|c, seed| {
                base(c, seed, 0x361f_98);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x3000);
                c.g[A3][0] = sx(DATA + 0x3400);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_trick_meter_sound_matches() {
        check(
            ee::s08::f_0029b3c0__generated,
            ssx_audio_system_stop_trick_meter_sound,
            50,
            &|c, seed| {
                let mut r = base(c, seed, 0x29b3_c0);
                c.g[A0][0] = sx(DATA);
                let rider = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
                c.g[A1][0] = sx(rider);
                if rider != 0 {
                    c.bus
                        .write32(rider + 0x874, if seed % 2 == 0 { 0 } else { 1 });
                    c.bus.write32(rider + 0x87c, (seed % 2) as u32);
                }
                c.bus
                    .write32(DATA + 0x5fdc, if seed % 4 == 0 { u32::MAX } else { 0x1234 });
                let p = DATA + 0x2000;
                c.bus.write32(DATA + 0x118, p);
                c.bus.write32(p, DATA + 0x2100);
                fill(c, &mut r, DATA + 0x2100, 0x100);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_group1_event_matches() {
        check(
            ee::s08::f_002a40e8__generated,
            ssx_audio_system_is_track_group1_event,
            60,
            &|c, seed| {
                base(c, seed, 0x2a40_e8);
                let iface = DATA + 0x3000;
                let vt = DATA + 0x3200;
                c.bus.write32(0x5308_a8, iface);
                c.bus.write32(iface + 4, vt);
                put_vtable(c, vt, &[8]);
                c.g[A0][0] = sx(DATA);
                let index = seed % 3;
                c.bus.write32(0x535c_08, index);
                let track_id = if seed % 2 == 0 { 6 } else { 2 };
                c.bus.write32(0x43d9_50 + index * 0x64, track_id);
                c.bus.write8(0x535c_10, if seed % 3 == 0 { 1 } else { 6 });
            },
        );
    }

    #[test]
    fn crowd_audio_free_sample_list_matches() {
        check(
            ee::s08::f_002a4cf8__generated,
            crowd_audio_free_sample_list,
            50,
            &|c, seed| {
                alloc_base(c, seed, 0x2a4c_f8);
                let count = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, count);
                for i in 0..count {
                    let p = if (seed + i) % 2 == 0 {
                        0
                    } else {
                        alloc(c, 0x20)
                    };
                    c.bus.write32(DATA + 4 + i * 4, p);
                }
            },
        );
    }

    #[test]
    fn ini_parser_find_section_matches() {
        check(
            ee::s08::f_002a7bf8__generated,
            ini_parser_find_section,
            40,
            &|c, seed| {
                base(c, seed, 0x2a7b_f8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                put_str(c, DATA + 0x1000, b"section\0");
                // An empty first line makes the parser report "not found"
                // immediately, without walking the ini stream.
                put_str(c, DATA + 0x1100, b"\0");
                c.bus.write32(DATA + 8, DATA + 0x1100);
                c.pc = 0x002a_7bf8;
            },
        );
    }
}
