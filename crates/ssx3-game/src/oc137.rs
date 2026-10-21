//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path audio announcer lines, replay hooks, menu page
//! navigation, camera teardown, memory-card states, script/WScript loading,
//! bolt cache, rider finish-time estimate, front-end screen helpers and a
//! handful of renderer/rail/vram helpers, each with a differential test
//! against the generated body.
//!
//! Bodies are transcribed from the generated Rust (the authority for
//! registers, cycles and call order). Calls go through [`crate::call`] with
//! the original return address, so a replaced callee and the fake hooks the
//! differential tests install are both reached. Structured loops make the
//! original's back-edge event checks with [`crate::events::event_due`] and
//! [`crate::events::deliver_event_here`]; the resumable (dispatcher-shaped)
//! functions accept every resume pc the original can be re-entered at.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_2188,
        ssx_audio_system_dj_say_20ca,
        "SSXAudioSystem_DJSay20CA",
    ),
    (
        0x002a_2860,
        ssx_audio_system_dj_say_20cf,
        "SSXAudioSystem_DJSay20CF",
    ),
    (
        0x002a_2938,
        ssx_audio_system_dj_say_2102,
        "SSXAudioSystem_DJSay2102",
    ),
    (
        0x0027_b5d8,
        script_engine_on_bank_loaded,
        "ScriptEngine_OnBankLoaded",
    ),
    (
        0x002c_ab08,
        menu_sys_page_select_next,
        "MenuSysPage_SelectNext",
    ),(
        0x0027_0ab0,
        replay_on_rider_checkpoint,
        "Replay_OnRiderCheckpoint",
    ),
    (0x0015_dd88, game_camera_dtor, "GameCamera_dtor"),
    (
        0x0024_0ab0,
        mcmanager_state27_format,
        "MCManager_State27_Format",
    ),
    (
        0x0030_bee8,
        wscriptman_load_process_list,
        "WScriptMan_LoadProcessList",
    ),
    (0x0019_ce68, boltfile_cache_init, "BoltFile_CacheInit"),
    (
        0x0012_2d78,
        rider_estimate_finish_time,
        "Rider_EstimateFinishTime",
    ),(
        0x0026_cc48,
        playbackman_start_playback,
        "PlaybackMan_StartPlayback",
    ),
    (0x0022_9e58, crowdman2d_load_state, "CrowdMan2d_LoadState"),
    (0x003e_cb00, tag_field_merge, "TagFieldMerge"),
    (0x0039_9e28, uilistbox_refresh_text, "UIListBox_RefreshText"),
    (
        0x0035_a250,
        multisplinemodifier_v01,
        "MultiSplineModifier_v01",
    ),
    (
        0x0019_7bc8,
        cfestaterequestline_update_song_names,
        "cFEStateRequestLine_UpdateSongNames",
    ),
    (
        0x0014_54f8,
        benewraceint_get_event_target,
        "BENewRaceInt_GetEventTarget",
    ),
    (0x0019_48a8, cfestatetitle_update, "cFEStateTitle_Update"),
    (
        0x0019_4dd0,
        cfestatemainmenu_update,
        "cFEStateMainMenu_Update",
    ),
    (
        0x0018_3d08,
        cfestateubertrick_set_button_icon,
        "cFEStateUberTrick_SetButtonIcon",
    ),
    (0x0034_f048, floating_v49, "Floating_v49"),
    (
        0x001e_3680,
        messagecenter_get_list_index,
        "MessageCenter_GetListIndex",
    ),
    (
        0x0035_e770,
        rollermodifier_get_pose,
        "RollerModifier_GetPose",
    ),
    (0x0036_6618, vramheap_free_all, "VramHeap_FreeAll"),
    (
        0x002c_abe0,
        menu_sys_page_select_prev,
        "MenuSysPage_SelectPrev",
    ),
    (
        0x002c_6280,
        cps2memcard_mark_slots_without_space,
        "cPS2MemCard_MarkSlotsWithoutSpace",
    ),
    (0x0024_0688, mcmanager_state18, "MCManager_State18"),(
        0x001c_c6e8,
        ckeyboardpopup_handle_key_char,
        "cKeyboardPopup_HandleKeyChar",
    ),
    (
        0x0018_09b0,
        keyboardscreen_get_key_object_name,
        "KeyboardScreen_GetKeyObjectName",
    ),
    (
        0x001d_0510,
        cfestaterewardgallery_open_buy_popup,
        "cFEStateRewardGallery_OpenBuyPopup",
    ),
    (
        0x001f_b378,
        ovscreeningameerror_show_continue_button,
        "OVScreenInGameError_ShowContinueButton",
    ),
    (0x0014_59b8, benewplayerint_v01, "BENewPlayerInt_v01"),
    (
        0x0032_ca78,
        spheretree_collide_triangle,
        "SphereTree_CollideTriangle",
    ),(0x003a_be40, cps2chunkresolve_v05, "cPS2ChunkResolve_v05"),
    (
        0x003a_c358,
        cps2chunkresolve_relocate_instance_dma,
        "cPS2ChunkResolve_RelocateInstanceDma",
    ),
    (
        0x001f_4830,
        cfestatebuyattrib_init,
        "cFEStateBuyAttrib_Init",
    ),
    (0x0035_8588, railman_ctor_2, "RailMan_ctor_2"),
    (
        0x0019_8118,
        cfestaterequestline_update_song_count,
        "cFEStateRequestLine_UpdateSongCount",
    ),
    (
        0x0019_85f0,
        cfestaterequestline_update_slot_icons,
        "cFEStateRequestLine_UpdateSlotIcons",
    ),
    (
        0x0039_f320,
        uistatemgr_switch_state_immediate,
        "UIStateMgr_SwitchStateImmediate",
    ),
    (0x0030_ea80, rider_model_hide_part, "RiderModel_HidePart"),(0x0014_99a8, belibrary_dtor, "BELibrary_dtor"),
];

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Audio announcer lines.

/// `void SSXAudioSystem_DJSay20CA(SSXAudioSystem* self)` (0x2a2188): check the
/// DJ speech switch, post event 0x20CA with the language and DJ-voice masks,
/// then hand it to the speech manager with the category.
pub fn ssx_audio_system_dj_say_20ca(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_21a4)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(p(r32(c, S0), 0x5560));
        let mut rejected = false;
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20ca;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_21cc)?;
            if c.g[V0][0] != 0 {
                c.cycles += 29;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0029_f198, 0x002a_21dc)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1d8, 0x002a_21e4)?;
                c.g[A1][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1d48, 0x002a_21f0)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1e8, 0x002a_21f8)?;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x002a_2204)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_4bc0, 0x002a_220c)?;
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1e20, 0x002a_2218)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20ca;
                call_ptr(c, 0x003d_8008, 0x002a_222c)?;
                c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S0][0];
                let post = r32(c, V1);
                c.g[A1][0] = 3;
                call_ptr(c, post, 0x002a_2248)?;
            } else {
                rejected = true;
            }
        }
        if !rejected {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_DJSay20CF(SSXAudioSystem* self, int key)` (0x2a2860):
/// the same for event 0x20CF, but picks one bit from the language/voice mask
/// pair (or from `key` when it is a mask index) before posting.
pub fn ssx_audio_system_dj_say_20cf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_2884)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(p(r32(c, S1), 0x5560));
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cf;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_28ac)?;
            let rejected = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !rejected {
                c.cycles += 5;
                c.g[S3][0] = 0;
                call_ptr(c, 0x0029_f198, 0x002a_28bc)?;
                c.g[S2][0] = 0;
                c.g[S1][0] = c.g[V0][0];
                if (c.g[S0][0] as i64) >= 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3e7;
                    let is_999 = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 100i64) as u64;
                    if !is_999 {
                        c.cycles += 2;
                        let high = c.g[V0][0] == 0;
                        c.g[V0][0] = 1;
                        if high {
                            c.cycles += 2;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x64));
                            c.g[S2][0] = sx(r32(c, V0) << (r32(c, S0) & 31));
                        } else {
                            c.cycles += 2;
                            c.g[S3][0] = sx(r32(c, V0) << (r32(c, S0) & 31));
                        }
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20cf;
                call_ptr(c, 0x003d_8008, 0x002a_28fc)?;
                c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S3][0];
                c.g[T1][0] = c.g[S2][0];
                c.g[A1][0] = 4;
                c.g[A3][0] = 3;
                let post = r32(c, V1);
                call_ptr(c, post, 0x002a_291c)?;
            }
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

/// `void SSXAudioSystem_DJSay2102(SSXAudioSystem* self)` (0x2a2938): the same
/// as [`ssx_audio_system_dj_say_20ca`] for event 0x2102.
pub fn ssx_audio_system_dj_say_2102(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_2954)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(p(r32(c, S0), 0x5560));
        let mut rejected = false;
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x2102;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_297c)?;
            if c.g[V0][0] != 0 {
                c.cycles += 29;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0029_f198, 0x002a_298c)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1d8, 0x002a_2994)?;
                c.g[A1][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1d48, 0x002a_29a0)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1e8, 0x002a_29a8)?;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x002a_29b4)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_4bc0, 0x002a_29bc)?;
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1e20, 0x002a_29c8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x2102;
                call_ptr(c, 0x003d_8008, 0x002a_29dc)?;
                c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S0][0];
                let post = r32(c, V1);
                c.g[A1][0] = 3;
                call_ptr(c, post, 0x002a_29f8)?;
            } else {
                rejected = true;
            }
        }
        if !rejected {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_OnBankLoaded(ScriptEngine* self, int bank)` (0x27b5d8):
/// walk the ten script-bank slots; when one holds the loaded bank, read its
/// length, clear the slot, hand the data to the instance manager and record
/// the new data id.
pub fn script_engine_on_bank_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S1, 0xd0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0xa0);
        c.g[S3][0] = 0x480000;
        frame.save_ra(c, 0x90);
        c.g[S4][0] = 0xa;
        c.g[S0][0] = sx(p(r32(c, S1), 0x628));
        let mut via_clear = false;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            if c.g[V0][0] != c.g[A0][0] {
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            } else {
                c.cycles += 8;
                c.g[A1][0] = sx(p(r32(c, SP), 0x80));
                c.g[A2][0] = sx(p(r32(c, SP), 0x84));
                call_ptr(c, 0x003d_f9d8, 0x0027_b628)?;
                c.g[V1][0] = Addr(p(r32(c, S1), 0x620)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, SP), 0x84)).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                let pending = c.g[V0][0] != 0;
                Addr(p(r32(c, S1), 0x620)).sw(c, c.g[V1][0]);
                if !pending {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0027_7f08, 0x0027_b644)?;
                    via_clear = true;
                    break;
                }
                c.cycles += 14;
                c.g[A2][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(p(r32(c, S3), 0x1eb0));
                call_ptr(c, 0x0041_7828, 0x0027_b65c)?;
                Addr(p(r32(c, S0), 8)).sw(c, 0);
                c.g[A0][0] = sx(p(r32(c, S1), 0x51c));
                Addr(p(r32(c, S1), 0xa48)).sw(c, c.g[S2][0]);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = Addr(p(r32(c, SP), 0x80)).lw(c);
                c.g[A3][0] = Addr(p(r32(c, SP), 0x84)).lw(c);
                call_ptr(c, 0x0027_5920, 0x0027_b678)?;
                Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                Addr(p(r32(c, S1), 0xa48)).sw(c, c.g[S4][0]);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S0), 0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_b610);
            }
        }
        c.lq(S0, frame.at(c, 0xe0));
        if !via_clear {
            c.cycles += 1;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore_ra(c, 0x90);
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
// Replay hooks.

/// `void Replay_OnRiderEvent(Replay* self, int rider, int kind)` (0x2709d8):
/// ignore the event while the replay is busy; otherwise find the rider and
/// highlight the matching checkpoint bit, or empty the rider hook.
pub fn replay_on_rider_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x61c)).lw(c);
        let busy = c.g[V0][0] != 0;
        let mut empty = false;
        let mut highlight = false;
        let mut ret = false;
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x620)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x610)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x60c)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[V0][0] == 0 {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0027_0730, 0x0027_0a38)?;
                            c.g[A3][0] = c.g[V0][0];
                            let negative = (c.g[A3][0] as i64) < 0;
                            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
                            if negative {
                                ret = true;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = sx(r32(c, S1) << 2);
                                c.g[V1][0] = Addr(p(r32(c, A0), 0x84)).lw(c);
                                c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
                                c.g[V0][0] = sx(p(r32(c, V0), r32(c, A1)));
                                let cond = c.g[S2][0] == 0;
                                c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
                                if cond {
                                    empty = true;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 1;
                                    if c.g[S2][0] == c.g[V0][0] {
                                        c.g[A2][0] = Addr(p(r32(c, A0), 0x790)).lw(c);
                                        highlight = true;
                                    } else {
                                        c.cycles += 2;
                                        ret = true;
                                    }
                                }
                            }
                        } else {
                            ret = true;
                        }
                    } else {
                        ret = true;
                    }
                } else {
                    ret = true;
                }
            } else {
                ret = true;
            }
        } else {
            ret = true;
        }
        if empty {
            c.cycles += 6;
            c.g[A1][0] = Addr(p(r32(c, A0), 0x790)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p(r32(c, A1), 0xfc));
            call_ptr(c, 0x0027_05a8, 0x0027_0a80)?;
            ret = true;
        } else if highlight {
            c.cycles += 4;
            c.g[A1][0] = c.g[A3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx(p(r32(c, A2), 0xfc));
            call_ptr(c, 0x0027_0b88, 0x0027_0a98)?;
            ret = true;
        }
        if !ret {
            // (every remaining arm already loaded s0 and returned)
        }
        if busy || highlight {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_OnRiderCheckpoint(Replay* self, int rider, int bit)` (0x270ab0):
/// set the rider's checkpoint bit unless the replay is busy, the rider is
/// invalid or the bit is already set; clears the "pending" flag.
pub fn replay_on_rider_checkpoint(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x61c)).lw(c);
        c.g[S1][0] = c.g[A2][0];
        let mut plus_one = false;
        'checks: {
            if c.g[V0][0] != 0 {
                plus_one = true;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x620)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x610)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x60c)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 5;
            call_ptr(c, 0x0027_0730, 0x0027_0b0c)?;
            c.g[A2][0] = c.g[V0][0];
            if (c.g[A2][0] as i64) < 0 {
                break 'checks;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break 'checks;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A2) << 2);
            if !in_range {
                break 'checks;
            }
            c.cycles += 8;
            c.g[V1][0] = sx(p(r32(c, S0), 0x5fc));
            c.g[A3][0] = sx(p(r32(c, V1), r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, A3)).lw(c);
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> (r32(c, S1) & 31)) as u32);
            c.g[V0][0] = c.g[V0][0] ^ 1;
            c.g[V0][0] = c.g[V0][0] & 1;
            let clear = c.g[V0][0] == 0;
            c.g[A0][0] = 1;
            if clear {
                break 'checks;
            }
            c.cycles += 9;
            c.g[A1][0] = Addr(p(r32(c, S0), 0x3b4)).lw(c);
            c.g[V1][0] = sx(r32(c, A0) << (r32(c, A2) & 31));
            c.g[V0][0] = Addr(p(r32(c, A1), 0x1f)).lbu(c);
            c.g[A0][0] = sx(r32(c, A0) << (r32(c, S1) & 31));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            Addr(p(r32(c, A1), 0x1f)).sb(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
            Addr(r32(c, A3)).sw(c, c.g[V1][0]);
            plus_one = true;
        }
        if plus_one {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x20));
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera.

/// `GameCamera::~GameCamera(this, int flags)` (0x15dd88): clear the camera
/// algorithms, destroy the three camera parts, tear down the animation
/// controller and either free the object or leave the base vtable.
pub fn game_camera_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x460000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4778));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(r32(c, S0), 0xa4)).lw(c);
        call_ptr(c, 0x0015_cd60, 0x0015_ddb4)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0xa8)).lw(c);
        call_ptr(c, 0x0015_cd60, 0x0015_ddbc)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
        call_ptr(c, 0x0015_cd60, 0x0015_ddc4)?;
        c.g[A2][0] = Addr(p(r32(c, S0), 0xa4)).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let destroy = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, destroy, 0x0015_dde4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0xa8)).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let destroy = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, destroy, 0x0015_de04)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let destroy = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, destroy, 0x0015_de24)?;
        }
        c.cycles += 8;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc0));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0017_6da8, 0x0015_de30)?;
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = c.g[S1][0] & 1;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4750));
        let free = c.g[V1][0] == 0;
        Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
        if !free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0015_de4c)?;
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
// Memory-card manager.

/// `void MCManager_State27_Format(MCManager* self)` (0x240ab0): state 27;
/// poll the card slot and step the format operation.
pub fn mcmanager_state27_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // slot 0x58 on the card interface
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x5c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_0ad8)?;
        if c.g[V0][0] != 0 {
            c.lq(S0, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        // slot 0x1a8
        c.cycles += 8;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x1a8)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x1ac)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_0af8)?;
        let mut join_b20 = false;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(r32(c, S0), 0x88)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 4;
            join_b20 = true;
        }
        if !join_b20 {
            c.cycles += 4;
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0b14)?;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
                join_b20 = true;
            } else {
                c.g[A1][0] = 0x28;
            }
        }
        if join_b20 {
            c.cycles += 6;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x0024_0b30)?;
            c.lq(S0, frame.at(c, 0x10));
        } else {
            // poll the 0x6c callback
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x6c)).lw(c);
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0b44)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0024_0b54)?;
                c.lq(S0, frame.at(c, 0x10));
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x84)).lw(c);
                let f = r32(c, V0);
                call_ptr(c, f, 0x0024_0b68)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0024_0b78)?;
                }
                c.lq(S0, frame.at(c, 0x10));
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScript.

/// `void WScriptMan_LoadProcessList(WScriptMan* self, WScriptList* list,
/// Stream* stream)` (0x30bee8): read a count and that many process ids from
/// the stream, move each process from the old list state into the new one.
pub fn wscriptman_load_process_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[A2][0] = 4;
        frame.save(c, S4, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[S2][0] = 0;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let read = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, read, 0x0030_bf30)?;
        c.g[V0][0] = Addr(r32(c, SP)).lw(c);
        c.g[S3][0] = sx(p(r32(c, S0), 0x2b4));
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 8;
            c.lq(S0, frame.at(c, 0x60));
            frame.restore(c, S1, 0x50);
            frame.restore(c, S2, 0x40);
            frame.restore(c, S3, 0x30);
            frame.restore(c, S4, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        loop {
            c.cycles += 24;
            c.g[A1][0] = sx(p(r32(c, SP), 4));
            c.g[A2][0] = 4;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
            let read = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, read, 0x0030_bf5c)?;
            c.g[A1][0] = Addr(p(r32(c, SP), 4)).lw(c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0030_b320, 0x0030_bf68)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b2d0, 0x0030_bf78)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0030_8118, 0x0030_bf84)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0030_b270, 0x0030_bf90)?;
            c.g[V0][0] = Addr(r32(c, SP)).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_bf40);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt cache.

/// `void BoltFile_CacheInit(void* table)` (0x19ce68): clear the ten 256-entry
/// bolt rows and their counts, then create (or reset) the global index
/// manager.
pub fn boltfile_cache_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.g[V0][0] = 0xb0000;
        c.g[T2][0] = c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.g[A2][0] = 0x10000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[T0][0] = sx(p(r32(c, T2), r32(c, V0)));
        c.g[A0][0] = 0;
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = 1;
        c.g[A2][0] = c.g[A2][0] | 0x2000;
        loop {
            c.cycles += 6;
            c.mult(0, 2, 4, 6);
            c.g[A1][0] = sx(r32(c, A0) << 2);
            c.g[V1][0] = sx(p(r32(c, V0), r32(c, T2)));
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[A0][0] = 0xff;
            loop {
                c.cycles += 10;
                c.sw(r32(c, V1), 0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                Addr(p(r32(c, V1), 4)).sb(c, 0);
                Addr(p(r32(c, V1), 0x104)).sw(c, 0);
                Addr(p(r32(c, V1), 0x118)).sw(c, 0);
                Addr(p(r32(c, V1), 0x108)).sw(c, 0);
                Addr(p(r32(c, V1), 0x114)).sw(c, 0);
                Addr(p(r32(c, V1), 0x110)).sw(c, c.g[A3][0]);
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx(p(r32(c, V1), 0x120));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_ceb0);
                }
            }
            c.cycles += 5;
            c.g[V1][0] = sx(p(r32(c, T0), r32(c, A1)));
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            Addr(r32(c, V1)).sw(c, c.g[T1][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_ce98);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = 0xb0000;
        c.g[A0][0] = Addr(gp(c, -0x1788)).lw(c);
        c.g[V0][0] = sx(p(r32(c, T2), r32(c, V0)));
        Addr(p(r32(c, V0), 0x4000)).sw(c, 0);
        let have = c.g[A0][0] != 0;
        Addr(p(r32(c, V0), 0x4004)).sw(c, 0);
        if have {
            c.cycles += 2;
            call_ptr(c, 0x0019_cb60, 0x0019_cf34)?;
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x1f44;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x9c0));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0019_cf1c)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_cb38, 0x0019_cf24)?;
            Addr(gp(c, -0x1788)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(r32(c, SP));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider finish time.

/// `int Rider_EstimateFinishTime(Rider* self)` (0x122d78): estimate the
/// remaining time as `n + dist / rate`, where `rate` is the recent distance
/// rate clamped to at least `30 - aiIndex`.
pub fn rider_estimate_finish_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, frame.at(c, 0x48));
        c.swc1(20, frame.at(c, 0x40));
        c.g[S1][0] = sx(p(r32(c, S0), 0x6c0));
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x41f00000;
        c.mtc1(1, 21);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
        c.g[S2][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        call_ptr(c, 0x0011_3130, 0x0012_2db8)?;
        c.lwc1(1, p(r32(c, S0), 0x4d0));
        c.mtc1(18, 2);
        c.cvt_s_w(2, 2);
        c.g[V1][0] = Addr(p(r32(c, S0), 0x6c0)).lw(c);
        c.sub_s(0, 0, 1);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
        c.div_s(20, 0, 2);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x0012_2de0)?;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(0, 21, 0);
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.cycles += 9;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6c0)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, f, 0x0012_2e10)?;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.sub_s(20, 21, 0);
            c.lwc1(0, p(r32(c, S0), 0x4d0));
        } else {
            c.lwc1(0, p(r32(c, S0), 0x4d0));
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x30);
        c.div_s(0, 0, 20);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0);
        c.lwc1(21, frame.at(c, 0x48));
        c.lwc1(20, frame.at(c, 0x40));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
        frame.restore(c, S2, 0x10);
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Reward thresholds.

/// The shared body of the two reward threshold helpers: pick the tier, then
/// find the smallest table value that is below the current best.
fn reward_threshold(c: &mut Cpu, tier_fn: u32, table: u32, ret: u32, head: u32) -> Flow {
    c.cycles += 22;
    let frame = Frame::open(c, 0x90);
    frame.save(c, S4, 0x40);
    frame.save(c, S5, 0x30);
    c.g[S4][0] = c.g[A3][0];
    c.g[S5][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    frame.save(c, S1, 0x70);
    frame.save(c, S2, 0x60);
    frame.save(c, S3, 0x50);
    c.g[S2][0] = 0;
    frame.save(c, S6, 0x20);
    frame.save(c, S7, 0x10);
    frame.save_ra(c, 0);
    c.g[S3][0] = 0;
    call_ptr(c, tier_fn, ret)?;
    c.g[S7][0] = c.g[V0][0];
    c.g[V1][0] = sx(r32(c, S4) << 4);
    c.g[V0][0] = 0x460000;
    c.g[S6][0] = sx(r32(c, V0).wrapping_sub(table));
    c.g[S1][0] = sx(p(r32(c, V1), r32(c, S6)));
    loop {
        c.cycles += 6;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x0015_6a60, ret + 0x28)?;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        } else {
            c.cycles += 2;
            let empty = c.g[S2][0] == 0;
            c.g[V1][0] = sx(r32(c, S4) << 4);
            if !empty {
                c.cycles += 4;
                c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(p(r32(c, S3), r32(c, V1)));
                    c.g[V0][0] = sx(p(r32(c, V0), r32(c, S6)));
                    c.g[S2][0] = Addr(r32(c, V0)).lw(c);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                }
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(p(r32(c, S3), r32(c, V1)));
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, S6)));
                c.g[S2][0] = Addr(r32(c, V0)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            }
        }
        c.cycles += 4;
        c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    c.cycles += 12;
    c.g[V0][0] = c.g[S2][0];
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
}

/// `int BERewardInt_GetNextCollectibleThreshold(self, a, b, kind)` (0x158c80).
pub fn berewardint_get_next_collectible_threshold(c: &mut Cpu) {
    guest(c, |c| {
        reward_threshold(c, 0x0015_89b0, 0x5018, 0x0015_8cc0, 0x0015_8cd8)
    })
}

/// `int BERewardInt_GetNextMissionThreshold(self, a, b, kind)` (0x158d58).
pub fn berewardint_get_next_mission_threshold(c: &mut Cpu) {
    guest(c, |c| {
        reward_threshold(c, 0x0015_8a50, 0x4fe8, 0x0015_8d98, 0x0015_8db0)
    })
}

// ---------------------------------------------------------------------------
// Font.

/// `Font* Font_ctor(Font* self, char* name, int flags)` (0x3915e8): set the
/// default UV transforms and colours, then load the font.
pub fn font_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.sq(S0, p(r32(c, SP), 0x30));
        c.sw(r32(c, SP), 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(r32(c, SP), 4)).sw(c, 0);
        c.sd(p(r32(c, SP), 0x20), c.g[RA][0]);
        Addr(p(r32(c, S0), 8)).sw(c, 0);
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.sdl(2, p(r32(c, S0), 0x2f));
        c.sdr(2, p(r32(c, S0), 0x28));
        c.swc1(0, r32(c, SP));
        c.swc1(0, p(r32(c, SP), 4));
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.sdl(2, p(r32(c, S0), 0x37));
        c.sdr(2, p(r32(c, S0), 0x30));
        c.swc1(0, p(r32(c, SP), 0x10));
        c.swc1(0, p(r32(c, SP), 0x14));
        c.swc1(0, p(r32(c, SP), 0x18));
        c.swc1(0, p(r32(c, SP), 0x1c));
        c.ldl(2, p(r32(c, S0), 0x37));
        c.ldr(2, p(r32(c, S0), 0x30));
        c.sdl(2, p(r32(c, S0), 0x3f));
        c.sdr(2, p(r32(c, S0), 0x38));
        c.ldl(2, p(r32(c, SP), 0x17));
        c.ldr(2, p(r32(c, SP), 0x10));
        c.ldl(3, p(r32(c, SP), 0x1f));
        c.ldr(3, p(r32(c, SP), 0x18));
        c.sdl(2, p(r32(c, S0), 0x47));
        c.sdr(2, p(r32(c, S0), 0x40));
        c.sdl(3, p(r32(c, S0), 0x4f));
        c.sdr(3, p(r32(c, S0), 0x48));
        c.swc1(0, r32(c, SP));
        Addr(p(r32(c, SP), 4)).sw(c, 0);
        Addr(p(r32(c, SP), 8)).sw(c, 0);
        Addr(p(r32(c, SP), 0xc)).sw(c, 0);
        c.ldl(2, p(r32(c, SP), 7));
        c.ldr(2, r32(c, SP));
        c.ldl(3, p(r32(c, SP), 0xf));
        c.ldr(3, p(r32(c, SP), 8));
        c.sdl(2, p(r32(c, S0), 0x57));
        c.sdr(2, p(r32(c, S0), 0x50));
        c.sdl(3, p(r32(c, S0), 0x5f));
        c.sdr(3, p(r32(c, S0), 0x58));
        Addr(p(r32(c, S0), 0x64)).sb(c, 0);
        call_ptr(c, 0x0039_1708, 0x0039_16ac)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(p(r32(c, SP), 0x20));
        c.lq(S0, p(r32(c, SP), 0x30));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Playback manager.

/// `void PlaybackMan_StartPlayback(PlaybackMan* self, int frame, int a2)`
/// (0x26cc48): if not already playing, purge the replay riders, reset the
/// replay, save the world state and start frame stepping.
pub fn playbackman_start_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x70)).lw(c);
        let mut done = false;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            if (c.g[V1][0] as i64) > 0 {
                c.cycles += 33;
                c.g[A0][0] = Addr(p(r32(c, S0), 0x60)).lw(c);
                call_ptr(c, 0x0012_af68, 0x0026_cc7c)?;
                c.g[A0][0] = Addr(p(r32(c, S0), 0x60)).lw(c);
                call_ptr(c, 0x0012_8ac0, 0x0026_cc84)?;
                c.g[V1][0] = Addr(p(r32(c, S0), 0x5c)).lw(c);
                c.g[A1][0] = 0xe;
                c.g[V0][0] = Addr(p(r32(c, V1), 0x28)).lw(c);
                c.g[A2][0] = Addr(r32(c, V0)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                Addr(p(r32(c, V0), 4)).sw(c, c.g[A2][0]);
                call_ptr(c, 0x0026_f4a0, 0x0026_cca0)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_cbb8, 0x0026_cca8)?;
                c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = 0x10000;
                c.g[A0][0] = c.g[A0][0] | 0xc214;
                Addr(p(r32(c, V1), 8)).sw(c, 0);
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = sx(p(r32(c, V0), r32(c, A0)));
                call_ptr(c, 0x0031_7908, 0x0026_ccc4)?;
                c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[V0][0] = 0x10000;
                c.g[V0][0] = c.g[V0][0] | 0x8000;
                c.g[A1][0] = Addr(p(r32(c, S0), 0x60)).lw(c);
                c.g[V1][0] = sx(p(r32(c, V1), r32(c, V0)));
                c.g[A0][0] = 1;
                c.g[V0][0] = Addr(p(r32(c, V1), 0x4210)).lw(c);
                Addr(p(r32(c, A1), 8)).sw(c, c.g[V0][0]);
                Addr(p(r32(c, S0), 0x70)).sw(c, c.g[A0][0]);
                c.g[V0][0] = Addr(p(r32(c, S0), 0x60)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, V0), 0x70)).lw(c);
                let no_cur = c.g[V0][0] == 0;
                c.g[A2][0] = 0;
                if !no_cur {
                    c.cycles += 1;
                    c.g[A2][0] = Addr(p(r32(c, V0), 0x18)).lw(c);
                }
                c.cycles += 6;
                c.g[V1][0] = Addr(p(r32(c, S0), 0x5c)).lw(c);
                c.g[A1][0] = c.g[A2][0];
                c.g[V0][0] = Addr(p(r32(c, V1), 0x84)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 4)).lw(c);
                call_ptr(c, 0x0015_edc8, 0x0026_cd10)?;
                done = true;
            }
        }
        if !done {
            // both early exits did `lq s0` and jumped straight to the tail
        }
        c.lq(S0, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd manager.

/// `void CrowdMan2d_LoadState(CrowdMan2d* self, CrowdState* state)` (0x229e58):
/// clear the crowd, fetch the id table and re-create every live slot.
pub fn crowdman2d_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x250);
        frame.save(c, S3, 0x210);
        frame.save(c, S0, 0x240);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x230);
        frame.save(c, S2, 0x220);
        frame.save_ra(c, 0x200);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0022_9398, 0x0022_9e7c)?;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0022_92e0, 0x0022_9e84)?;
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.g[SP][0];
        c.g[A0][0] = 0x7f;
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            let more = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9e98);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x200;
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[S1][0] = 0;
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        c.g[S2][0] = c.g[S2][0] | 0xffff;
        let read = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, read, 0x0022_9edc)?;
        c.g[S0][0] = c.g[SP][0];
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        loop {
            c.cycles += 6;
            c.g[A1][0] = c.g[S1][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[V0][0];
            let empty = c.g[V0][0] == c.g[S2][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !empty {
                c.cycles += 2;
                call_ptr(c, 0x0022_9910, 0x0022_9f08)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0022_9ee8);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x240);
        frame.restore(c, S1, 0x230);
        frame.restore(c, S2, 0x220);
        frame.restore(c, S3, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tag fields.

/// `int TagFieldMerge(void* record, void* len, char* data)` (0x3ecb00): merge
/// whitespace-separated `name=value` fields from `data` into the tag record,
/// returning the number of fields accepted.
pub fn tag_field_merge(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x003e_cb00 => {
                c.cycles += 15;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
                c.sq(S0, p(r32(c, SP), 0x60));
                c.sq(S1, p(r32(c, SP), 0x50));
                c.g[S0][0] = c.g[A2][0];
                c.sq(S2, p(r32(c, SP), 0x40));
                c.g[S1][0] = 0;
                c.sq(S3, p(r32(c, SP), 0x30));
                c.g[S2][0] = c.g[A1][0];
                c.sq(S4, p(r32(c, SP), 0x20));
                c.g[S3][0] = c.g[A0][0];
                c.sq(S5, p(r32(c, SP), 0x10));
                c.g[S4][0] = 0x3a;
                c.sd(r32(c, SP), c.g[RA][0]);
                c.g[S5][0] = 0x3d;
                pc = 0x003e_cb84;
            }
            0x003e_cb3c => {
                c.cycles += 1;
                pc = 0x003e_cb40;
            }
            0x003e_cb40 => {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                if call_ptr(c, 0x003e_b1e0, 0x003e_cb4c).is_err() {
                    return;
                }
                c.g[V1][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = (c.g[V1][0] < 0x20u64) as u64;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                if c.g[V1][0] != 0 {
                    pc = 0x003e_cb84;
                } else {
                    pc = 0x003e_cb60;
                }
            }
            0x003e_cb60 => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                pc = 0x003e_cb68;
            }
            0x003e_cb68 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    if back_edge_unwinds(c, 0x003e_cb68) {
                        return;
                    }
                    pc = 0x003e_cb68;
                } else {
                    pc = 0x003e_cb84;
                }
            }
            0x003e_cb84 => {
                c.cycles += 3;
                c.g[V1][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = (c.g[V1][0] < 0x21u64) as u64;
                if c.g[V1][0] == 0 {
                    pc = 0x003e_cbb0;
                } else {
                    pc = 0x003e_cb90;
                }
            }
            0x003e_cb90 => {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    pc = 0x003e_cba0;
                } else {
                    pc = 0x003e_cb98;
                }
            }
            0x003e_cb98 => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                if back_edge_unwinds(c, 0x003e_cb84) {
                    return;
                }
                pc = 0x003e_cb84;
            }
            0x003e_cba0 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[S1][0];
                if c.g[V1][0] == c.g[S5][0] {
                    pc = 0x003e_cbb4;
                } else {
                    pc = 0x003e_cba8;
                }
            }
            0x003e_cba8 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[V1][0] != c.g[S4][0] {
                    if back_edge_unwinds(c, 0x003e_cb40) {
                        return;
                    }
                    pc = 0x003e_cb40;
                } else {
                    pc = 0x003e_cbb0;
                }
            }
            0x003e_cbb0 => {
                c.cycles += 1;
                c.g[V0][0] = c.g[S1][0];
                pc = 0x003e_cbb4;
            }
            0x003e_cbb4 => {
                c.cycles += 9;
                c.lq(S0, p(r32(c, SP), 0x60));
                c.lq(S1, p(r32(c, SP), 0x50));
                c.lq(S2, p(r32(c, SP), 0x40));
                c.lq(S3, p(r32(c, SP), 0x30));
                c.lq(S4, p(r32(c, SP), 0x20));
                c.lq(S5, p(r32(c, SP), 0x10));
                c.g[RA][0] = c.ld(r32(c, SP));
                let ra = r32(c, RA);
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x70));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003e_cb00, 0x003e_cbd8);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// List box.

/// `void UIListBox_RefreshText(UIListBox* self)` (0x399e28): set the text of
/// every row from the localised string table, then relayout the box.
pub fn uilistbox_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x5c)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0xd0)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lw(c);
        c.g[S2][0] = Addr(p(r32(c, A0), 0x10)).lw(c);
        let no_storage = c.g[S2][0] == 0;
        c.lq(S0, frame.at(c, 0x50));
        if !no_storage {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S1), 0x318)).lbu(c);
            let none = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 3;
                c.g[S4][0] = 0x14;
                c.g[S3][0] = sx(p(r32(c, S1), 0x98));
                c.mult(0, 3, 16, 20);
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx(p(r32(c, V1), r32(c, S3)));
                    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                    let empty = c.g[A1][0] == 0;
                    c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                    let mut next = 0u64;
                    if !empty {
                        c.cycles += 7;
                        c.g[V1][0] = Addr(p(r32(c, S2), 4)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
                        c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
                        let get = r32(c, V0);
                        c.g[A0][0] = sx(p(r32(c, S2), r32(c, A0)));
                        call_ptr(c, get, 0x0039_9e9c)?;
                        let is_null = c.g[V0][0] == 0;
                        c.g[A2][0] = c.g[V0][0];
                        if is_null {
                            c.cycles += 1;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0039_9d80, 0x0039_9eb0)?;
                            c.cycles += 1;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                    }
                    next = c.g[V1][0];
                    let _ = next;
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x318)).lbu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.mult(0, 3, 16, 20);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_9e78);
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
        let relayout = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, relayout, 0x0039_9ee0)?;
        c.lq(S0, frame.at(c, 0x50));
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spline modifier.

/// `MultiSplineModifier::~MultiSplineModifier(this, int flags)` (0x35a250):
/// remove the node instances, free the node/data/instance arrays, then free
/// the object when asked.
pub fn multisplinemodifier_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x490000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe98));
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0035_a780, 0x0035_a27c)?;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x38)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(p(r32(c, S1), 0x3c)).lw(c);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0035_a290)?;
            c.g[A0][0] = Addr(p(r32(c, S1), 0x3c)).lw(c);
        }
        c.cycles += 2;
        let have_data = c.g[A0][0] != 0;
        c.g[S2][0] = c.g[S0][0] & 1;
        if have_data {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x0035_a2a4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
        let mut none = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !none {
            loop {
                c.cycles += 13;
                call_ptr(c, 0x002d_1ce0, 0x0035_a2b8)?;
                c.g[A1][0] = Addr(p(r32(c, S1), 0x44)).lw(c);
                c.g[V1][0] = sx(r32(c, S0) << 2);
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V1][0] = sx(p(r32(c, V1), r32(c, A1)));
                c.g[A1][0] = Addr(r32(c, V1)).lw(c);
                call_ptr(c, 0x0035_1260, 0x0035_a2d4)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a2b0);
                }
            }
            none = true;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x44)).lw(c);
        let no_insts = c.g[A0][0] == 0;
        c.g[V0][0] = 0x490000;
        if !no_insts {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0035_a2f8)?;
            c.g[V0][0] = 0x490000;
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x13f8));
        let keep = c.g[S2][0] == 0;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0035_a310)?;
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

// ---------------------------------------------------------------------------
// Front end: request line.

/// `void cFEStateRequestLine_UpdateSongNames(self)` (0x197bc8): fill the
/// eight visible text rows from the message list (or hide the empty ones).
pub fn cfestaterequestline_update_song_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = sx(p(r32(c, S2), 0x174));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save(c, S0, 0x50);
        c.g[S4][0] = sx(p(r32(c, S2), 0x4c));
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S2), 0x150)).lw(c);
        loop {
            c.cycles += 8;
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(p(r32(c, S2), 0x14c)).lw(c);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, S3)));
            c.g[V1][0] = sx(r32(c, V0) << 2);
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S4), r32(c, V1)));
            if in_range {
                c.cycles += 15;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let show = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, show, 0x0019_7c34)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0019_8a88, 0x0019_7c40)?;
                c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x0019_7c4c)?;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            } else {
                c.cycles += 8;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let show = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, show, 0x0019_7c70)?;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p(r32(c, S2), 0x150)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_7bf8);
            }
        }
        c.cycles += 8;
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

// ---------------------------------------------------------------------------
// Front end: race event target.

/// `int BENewRaceInt_GetEventTarget(BENewRaceInt* self, int kind, int a2)`
/// (0x1454f8): special-field lookup by mode, else scan the 30-entry event
/// table for the current key and kind.
pub fn benewraceint_get_event_target(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x0014_54f8 => {
                c.cycles += 10;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
                c.g[V0][0] = 0x530000;
                c.sd(r32(c, SP), c.g[RA][0]);
                c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x5bc8));
                c.g[A2][0] = c.g[A1][0];
                c.g[A1][0] = Addr(p(r32(c, A3), 0x4a)).lbu(c);
                c.g[V0][0] = sx(r32(c, A1).wrapping_sub(6));
                c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                let special = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1) << 24);
                if special {
                    pc = 0x0014_5520;
                } else {
                    pc = 0x0014_5538;
                }
            }
            0x0014_5520 => {
                c.cycles += 4;
                c.g[A1][0] = sx(((r32(c, A1) as i32) >> 24) as u32);
                if call_ptr(c, 0x0014_5600, 0x0014_5528).is_err() {
                    return;
                }
                c.g[RA][0] = c.ld(r32(c, SP));
                pc = 0x0014_55c8;
            }
            0x0014_5530 => {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r32(c, V1), 0xe)).lh(c);
                pc = 0x0014_55c4;
            }
            0x0014_5538 => {
                c.cycles += 8;
                c.g[V1][0] = 0x440000;
                c.g[V0][0] = 1;
                c.g[T0][0] = sx(r32(c, V1).wrapping_add(0xb38));
                c.g[A3][0] = Addr(p(r32(c, A3), 0x40)).lw(c);
                if c.g[A2][0] == 0 {
                    c.g[A2][0] = c.g[V0][0];
                }
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[T0][0];
                pc = 0x0014_5558;
            }
            0x0014_5558 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).lh(c);
                let key_ok = c.g[V0][0] == c.g[A3][0];
                c.g[V1][0] = sx(r32(c, A1) << 4);
                if key_ok {
                    pc = 0x0014_5564;
                } else {
                    pc = 0x0014_5570;
                }
            }
            0x0014_5564 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, A0), 2)).lh(c);
                if c.g[V0][0] == c.g[A2][0] {
                    c.g[V1][0] = sx(p(r32(c, T0), r32(c, V1)));
                    if back_edge_unwinds(c, 0x0014_5530) {
                        return;
                    }
                    pc = 0x0014_5530;
                } else {
                    pc = 0x0014_5570;
                }
            }
            0x0014_5570 => {
                c.cycles += 4;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < 30i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(p(r32(c, A0), 0x10));
                if more {
                    if back_edge_unwinds(c, 0x0014_5558) {
                        return;
                    }
                    pc = 0x0014_5558;
                } else {
                    pc = 0x0014_5580;
                }
            }
            0x0014_5580 => {
                c.cycles += 6;
                c.g[V0][0] = 0x440000;
                c.g[A3][0] = Addr(gp(c, -0x1f34)).lw(c);
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(0xb38));
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[T0][0];
                pc = 0x0014_5598;
            }
            0x0014_5598 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).lh(c);
                let key_ok = c.g[V0][0] == c.g[A3][0];
                c.g[V1][0] = sx(r32(c, A1) << 4);
                if key_ok {
                    pc = 0x0014_55a4;
                } else {
                    pc = 0x0014_55b0;
                }
            }
            0x0014_55a4 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, A0), 2)).lh(c);
                if c.g[V0][0] == c.g[A2][0] {
                    c.g[V1][0] = sx(p(r32(c, T0), r32(c, V1)));
                    if back_edge_unwinds(c, 0x0014_5530) {
                        return;
                    }
                    pc = 0x0014_5530;
                } else {
                    pc = 0x0014_55b0;
                }
            }
            0x0014_55b0 => {
                c.cycles += 4;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < 30i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(p(r32(c, A0), 0x10));
                if more {
                    if back_edge_unwinds(c, 0x0014_5598) {
                        return;
                    }
                    pc = 0x0014_5598;
                } else {
                    pc = 0x0014_55c0;
                }
            }
            0x0014_55c0 => {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                pc = 0x0014_55c4;
            }
            0x0014_55c4 => {
                c.cycles += 1;
                c.g[RA][0] = c.ld(r32(c, SP));
                pc = 0x0014_55c8;
            }
            0x0014_55c8 => {
                c.cycles += 2;
                let ra = r32(c, RA);
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0014_54f8, 0x0014_55d0);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Front end: title / main menu.

/// `void cFEStateTitle_Update(self)` (0x1948a8): tick the idle timer and drop
/// back to the attract mode after 30 seconds; any active list item resets it.
pub fn cfestatetitle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e510, 0x0019_48bc)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x48)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = ((c.g[V0][0] as i64) < 1801i64) as u64;
        let ticking = c.g[V1][0] != 0;
        Addr(p(r32(c, S0), 0x48)).sw(c, c.g[V0][0]);
        if !ticking {
            c.cycles += 5;
            c.g[V0][0] = Addr(gp(c, -0x1724)).lw(c);
            Addr(p(r32(c, S0), 0x48)).sw(c, 0);
            c.g[V0][0] = c.g[V0][0] | 4;
            Addr(gp(c, -0x1724)).sw(c, c.g[V0][0]);
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A3][0] = 0;
        c.g[V1][0] = 0;
        c.g[T0][0] = 0x1c;
        c.g[T1][0] = sx(p(r32(c, V0), 0xb0));
        c.g[V0][0] = sx(r32(c, V1) << 2);
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[V0][0] = sx(p(r32(c, T1), r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32(c, V1)).lw(c);
            c.g[A1][0] = Addr(r32(c, V1)).lw(c);
            if c.g[A1][0] == 0 {
                c.g[V1][0] = c.g[A2][0];
            } else {
                c.cycles += 1;
                c.g[A0][0] = 0;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    c.mult(0, 10, 4, 8);
                    let mut found = 0u64;
                    if out_of_range {
                        c.cycles += 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx(p(r32(c, T2), r32(c, V1)));
                        found = Addr(p(r32(c, V0), 0x10)).lw(c);
                    }
                    c.cycles += 2;
                    if found == 0 {
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_4920);
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A3][0] = 1;
                        break;
                    }
                }
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    Addr(p(r32(c, S0), 0x48)).sw(c, 0);
                }
                c.cycles += 1;
                c.g[V1][0] = c.g[A2][0];
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, V1) << 2);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_4900);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMainMenu_Update(self)` (0x194dd0): select the two menu
/// columns and pick the first entry if both are valid.
pub fn cfestatemainmenu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0039_e510, 0x0019_4df0)?;
        c.g[V0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        c.g[S0][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        Addr(p(r32(c, S1), 0x55)).sb(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        loop {
            c.cycles += 12;
            c.g[A1][0] = c.g[S2][0] & 0xff;
            c.g[A0][0] = Addr(p(r32(c, V0), 0x160)).lh(c);
            c.g[A2][0] = Addr(p(r32(c, V0), 0x164)).lw(c);
            let select = r32(c, A2);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, select, 0x0019_4e14)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x168)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x16c)).lw(c);
            let valid = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, valid, 0x0019_4e28)?;
            let not_valid = c.g[V0][0] == 0;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            if !not_valid {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x55)).lbu(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                Addr(p(r32(c, S1), 0x55)).sb(c, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_4e00);
            }
        }
        c.cycles += 9;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1398));
        call_ptr(c, 0x0031_7670, 0x0019_4e54)?;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_4e60)?;
        c.g[A0][0] = c.g[V0][0];
        let no_menu = c.g[A0][0] == 0;
        c.g[V0][0] = 2;
        if no_menu {
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
            c.cycles += 1;
            c.lq(S1, frame.at(c, 0x20));
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(r32(c, A0), 0x95)).lbu(c);
            let not_sel = c.g[V1][0] != c.g[V0][0];
            c.lq(S0, frame.at(c, 0x30));
            if not_sel {
                c.cycles += 1;
                c.lq(S1, frame.at(c, 0x20));
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x55)).lb(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                let few = c.g[V0][0] == 0;
                c.lq(S1, frame.at(c, 0x20));
                if !few {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0039_b960, 0x0019_4e90)?;
                    c.cycles += 1;
                    c.lq(S0, frame.at(c, 0x30));
                    c.cycles += 1;
                    c.lq(S1, frame.at(c, 0x20));
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateUberTrick_SetButtonIcon(self, Rect* rect)` (0x183d08): pass
/// the rect's size (scaled) to the icon widget, or hide it when there is no
/// rect.
pub fn cfestateubertrick_set_button_icon(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.swc1(20, frame.at(c, 0x40));
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let no_rect = c.g[S0][0] == 0;
        c.mov_s(20, 12);
        if !no_rect {
            c.cycles += 33;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = Addr(p(r32(c, V0), 0xa0)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xa4)).lw(c);
            let getpos = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, getpos, 0x0018_3d44)?;
            c.lwc1(4, p(r32(c, S0), 0xc));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(2, p(r32(c, S0), 0x14));
            c.lwc1(3, p(r32(c, S0), 0x10));
            c.lwc1(1, p(r32(c, S0), 0x18));
            c.g[AT][0] = 0x43800000;
            c.mtc1(1, 0);
            c.sub_s(2, 2, 3);
            c.sub_s(1, 1, 4);
            c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.mul_s(0, 20, 0);
            c.mul_s(1, 1, 0);
            c.mul_s(2, 2, 0);
            c.swc1(1, p(r32(c, SP), 4));
            c.swc1(2, r32(c, SP));
            c.g[A0][0] = Addr(p(r32(c, V1), 0xa8)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0xac)).lw(c);
            let setpos = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, setpos, 0x0018_3d90)?;
            c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
            let show = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, show, 0x0018_3da8)?;
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let show = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, show, 0x0018_3dc8)?;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(20, frame.at(c, 0x40));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Floating modifier.

/// `void Floating_v49(Floating* self)` (0x34f048): refresh the bound box from
/// the instance's min/max and notify the collision system, or run the base
/// update when there is no box info.
pub fn floating_v49(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S0, 0x70);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x50);
        c.g[A1][0] = Addr(p(r32(c, S1), 0x78)).lw(c);
        let no_info = c.g[A1][0] == 0;
        if no_info {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0035_67e0, 0x0034_f10c)?;
            c.lq(S0, frame.at(c, 0x70));
        } else {
            c.cycles += 39;
            c.lq(2, r32(c, A1));
            c.g[V1][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            c.sq(2, p(r32(c, SP), 0x20));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 2);
            c.lq(2, p(r32(c, A1), 0x10));
            c.sq(2, p(r32(c, SP), 0x30));
            c.lwc1(0, p(r32(c, V1), 0x60));
            c.swc1(0, p(r32(c, SP), 0x40));
            c.lwc1(1, p(r32(c, V1), 0x64));
            c.swc1(1, p(r32(c, SP), 0x44));
            c.lwc1(0, p(r32(c, V1), 0x68));
            c.swc1(2, p(r32(c, SP), 0x4c));
            c.swc1(0, p(r32(c, SP), 0x48));
            c.lq(4, p(r32(c, SP), 0x40));
            c.sq(4, r32(c, SP));
            c.lwc1(0, p(r32(c, V1), 0x6c));
            c.swc1(0, p(r32(c, SP), 0x40));
            c.lwc1(1, p(r32(c, V1), 0x70));
            c.swc1(1, p(r32(c, SP), 0x44));
            c.lwc1(0, p(r32(c, V1), 0x74));
            c.swc1(0, p(r32(c, SP), 0x48));
            c.lq(2, p(r32(c, SP), 0x40));
            c.sq(2, p(r32(c, SP), 0x10));
            c.sq(4, r32(c, A1));
            c.lq(2, p(r32(c, SP), 0x10));
            c.sq(2, p(r32(c, A1), 0x10));
            c.g[S0][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            call_ptr(c, 0x002d_1be0, 0x0034_f0dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(p(r32(c, SP), 0x20));
            call_ptr(c, 0x0032_91e0, 0x0034_f0f4)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0034_ef08, 0x0034_f0fc)?;
            c.lq(S0, frame.at(c, 0x70));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Message center.

/// `int MessageCenter_GetListIndex(MessageCenter* self, int id)` (0x1e3680):
/// find the index of message `id` in the visible and grouped message lists
/// (0xff when absent and no list is active).
pub fn messagecenter_get_list_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(gp(c, -0x102c)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        let active = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if active {
            c.cycles += 10;
            c.g[A0][0] = 4;
            c.g[A1][0] = 0;
            call_ptr(c, 0x001e_2bb8, 0x001e_36c0)?;
            c.g[S0][0] = 0;
            c.g[S4][0] = c.g[V0][0];
            call_ptr(c, 0x001e_33e8, 0x001e_36cc)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V0][0] = sx(p(r32(c, S4), r32(c, S3)));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S5][0] = c.g[V0][0];
            if !none {
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                loop {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r32(c, S3).wrapping_sub(r32(c, V0)));
                    let negative = (c.g[A0][0] as i64) < 0;
                    c.g[S1][0] = c.g[V0][0];
                    let mut matched = false;
                    if !negative {
                        c.cycles += 4;
                        call_ptr(c, 0x001e_3620, 0x001e_36f4)?;
                        if c.g[V0][0] == c.g[S2][0] {
                            c.cycles += 2;
                            c.g[V0][0] = c.g[S0][0] & 0xff;
                            matched = true;
                        } else {
                            c.g[S0][0] = c.g[S1][0];
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx(p(r32(c, A0), r32(c, S4)));
                        let group_neg = (c.g[V0][0] as i64) < 0;
                        c.g[A1][0] = !(0u64 | c.g[A0][0]);
                        if !group_neg {
                            c.cycles += 5;
                            c.g[A2][0] = 1;
                            c.g[A0][0] = 4;
                            call_ptr(c, 0x001e_2c80, 0x001e_371c)?;
                            if c.g[V0][0] == c.g[S2][0] {
                                c.g[V0][0] = c.g[S0][0] & 0xff;
                                matched = true;
                            }
                        }
                        if !matched {
                            c.cycles += 1;
                            c.g[S0][0] = c.g[S1][0];
                        }
                    }
                    if matched {
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
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001e_36e0);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0xff;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[S2][0] & 0xff;
        }
        // (the inactive path returns above without the 0xff arm)
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
    })
}

// ---------------------------------------------------------------------------
// Roller modifier.

/// `sRmXform RollerModifier_GetPose(self, sRmXform* in)` (0x35e770): compose
/// the roller's rotation quaternion with the incoming transform (VU0), scale
/// it by -1 and add the translation, writing the result through a0.
pub fn rollermodifier_get_pose(c: &mut Cpu) {
    c.cycles += 55;
    c.lq(3, p(r32(c, A1), 0x40));
    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
    c.lq(6, p(r32(c, A1), 0x30));
    c.g[V0][0] = c.g[A0][0];
    c.sq(3, p(r32(c, SP), 0x10));
    c.sq(6, r32(c, SP));
    c.lqc2(4, p(r32(c, A1), 0x40));
    c.lqc2(5, p(r32(c, A1), 0x2b0));
    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().opmula(4, 6);
    c.vu0().opmsub(7, 6, 4);
    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
    c.sqc2(8, p(r32(c, SP), 0x40));
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 0);
    c.lq(3, p(r32(c, SP), 0x40));
    c.sq(3, p(r32(c, SP), 0x30));
    c.mfc1(5, 0);
    c.lqc2(4, p(r32(c, SP), 0x30));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(r32(c, SP), 0x40));
    c.lq(3, p(r32(c, SP), 0x40));
    c.sq(3, p(r32(c, SP), 0x20));
    c.lqc2(4, p(r32(c, SP), 0x10));
    c.lqc2(5, p(r32(c, SP), 0x20));
    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().opmula(4, 6);
    c.vu0().opmsub(7, 6, 4);
    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
    c.sqc2(8, p(r32(c, SP), 0x60));
    c.lq(3, p(r32(c, SP), 0x60));
    c.sq(3, p(r32(c, SP), 0x50));
    c.lqc2(3, r32(c, SP));
    c.lqc2(4, p(r32(c, SP), 0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, r32(c, SP));
    c.lq(3, r32(c, SP));
    c.lq(5, p(r32(c, SP), 0x10));
    c.sq(3, r32(c, A0));
    c.sq(5, p(r32(c, A0), 0x10));
    let ra = r32(c, RA);
    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x70));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Vram heap.

/// `void VramHeap_FreeAll(VramHeap* self)` (0x366618): walk the free list and
/// release every marked block.
pub fn vramheap_free_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p(r32(c, S1), 0x23c0)).lw(c);
        let done = (c.g[A1][0] as i64) < 0;
        c.g[S2][0] = 0x1c;
        if done {
            c.cycles += 7;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[S3][0] = 0xffff_ffff_ffff_fffd;
        c.mult(0, 3, 5, 18);
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S1), 0x1ff0)).lw(c);
            c.g[A3][0] = sx(p(r32(c, V0), r32(c, V1)));
            c.g[S0][0] = Addr(p(r32(c, A3), 0x10)).lw(c);
            let leaf = (c.g[S0][0] as i64) < 0;
            c.g[T0][0] = c.g[V0][0];
            let mut need_next = false;
            if !leaf {
                c.cycles += 6;
                c.mult(0, 2, 16, 18);
                c.g[V1][0] = sx(p(r32(c, V0), r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 1;
                if c.g[V0][0] != 0 {
                    // break out with V1 already loaded from A3 below
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = c.g[T0][0];
                    c.g[A0][0] = 0x1c;
                    c.mult(0, 3, 16, 4);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = sx(p(r32(c, V1), r32(c, A2)));
                        c.g[S0][0] = Addr(p(r32(c, V0), 0x10)).lw(c);
                        let end = (c.g[S0][0] as i64) < 0;
                        c.mult(0, 2, 16, 4);
                        if end {
                            need_next = true;
                            break;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = sx(p(r32(c, V0), r32(c, T0)));
                        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                        c.g[V0][0] = c.g[V0][0] & 1;
                        let more = c.g[V0][0] == 0;
                        c.mult(0, 3, 16, 4);
                        if !more {
                            need_next = true;
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0036_6680);
                        }
                    }
                }
                if need_next {
                    c.cycles += 1;
                }
                c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            } else {
                c.cycles += 1;
                c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            let clean = c.g[V0][0] == 0;
            c.g[V0][0] = sx(((r32(c, V1) as i32) >> 1) as u32);
            if !clean {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let no_clear = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & c.g[S3][0];
                if !no_clear {
                    c.cycles += 1;
                    Addr(r32(c, A3)).sw(c, c.g[V0][0]);
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0036_63d8, 0x0036_66cc)?;
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            let more = (c.g[A1][0] as i64) >= 0;
            c.mult(0, 3, 5, 18);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_6648);
            }
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

// ---------------------------------------------------------------------------
// Memory card: slot marking.

/// `void cPS2MemCard_MarkSlotsWithoutSpace(self)` (0x2c6280): mark every slot
/// that cannot hold another save, given the remaining space and save size.
pub fn cps2memcard_mark_slots_without_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0xa0)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xa4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x002c_62bc)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = Addr(p(r32(c, S1), 0x24)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0xb0)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xb4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x002c_62d8)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = Addr(p(r32(c, S1), 0x14)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S5][0] = 1;
        if none {
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
        c.cycles += 2;
        c.g[S0][0] = sx(p(r32(c, S1), 0x1ac));
        loop {
            c.cycles += 9;
            Addr(r32(c, S0)).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x218)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x21c)).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, f, 0x002c_630c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(r32(c, S1), 0x14)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    Addr(r32(c, S0)).sw(c, c.g[S5][0]);
                } else {
                    c.cycles += 1;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(r32(c, S4)));
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x14)).lw(c);
            }
            c.cycles += 4;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S0), 0x78));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_62f0);
            }
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
    })
}

/// `void MCManager_State18(MCManager* self)` (0x240688): state 18; poll the
/// card slot, then the pending-operation and completion callbacks.
pub fn mcmanager_state18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x5c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_06b0)?;
        if c.g[V0][0] != 0 {
            c.lq(S0, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x1a8)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x1ac)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_06d0)?;
        let mut join = false;
        if c.g[V0][0] != 0 {
            c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 4;
            join = true;
        }
        if !join {
            c.cycles += 7;
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x248)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x24c)).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
            call_ptr(c, f, 0x0024_06f8)?;
            let zero = c.g[V0][0] == 0;
            c.g[A1][0] = 0x1c;
            if !zero {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
                join = true;
            }
        }
        if !join {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x68)).lw(c);
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0714)?;
            let zero = c.g[V0][0] == 0;
            c.g[A1][0] = 0x19;
            if !zero {
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
                join = true;
            }
        }
        if join {
            c.cycles += 6;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x0024_0730)?;
            c.lq(S0, frame.at(c, 0x10));
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x6c)).lw(c);
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0744)?;
            c.g[A0][0] = c.g[S0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0024_0754)?;
            }
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Popup confirm.

/// `void cFEPopupConfirm_LayoutTitle(self, Desc* desc)` (0x1c8678): lay out
/// the confirm dialog's title / body from the descriptor and relayout it.
pub fn cfepopupconfirm_layout_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
        if c.g[A3][0] == 0 {
            c.lq(S0, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, A1), 0xc)).lw(c);
            let no_desc = c.g[V0][0] == 0;
            c.g[S1][0] = sx(p(r32(c, A1), 0xc));
            if !no_desc {
                c.cycles += 26;
                c.g[A1][0] = c.g[A3][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x001c_8050, 0x001c_86b0)?;
                c.g[A2][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                c.g[A1][0] = 1;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let f = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, f, 0x001c_86cc)?;
                c.g[A1][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
                c.g[A3][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001c_8320, 0x001c_86e0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(p(r32(c, S0), 0x260));
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001c_9b68, 0x001c_86f0)?;
                c.lwc1(0, p(r32(c, S0), 0x264));
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 1);
                c.g[V0][0] = Addr(p(r32(c, S0), 0x2cc)).lw(c);
                c.add_s(0, 0, 1);
                let no_align = c.g[V0][0] == 0;
                c.swc1(0, p(r32(c, S0), 0x264));
                if !no_align {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.mtc1(0, 12);
                    c.g[A1][0] = Addr(p(r32(c, A0), 0x7c)).lw(c);
                    c.g[A2][0] = 0x11;
                    call_ptr(c, 0x001c_8930, 0x001c_8720)?;
                    c.lq(S0, frame.at(c, 0x30));
                }
            } else {
                c.cycles += 6;
                c.g[V0][0] = Addr(p(r32(c, A3), 8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let f = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A3), r32(c, A0)));
                call_ptr(c, f, 0x001c_8740)?;
            }
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `int cKeyboardPopup_HandleKeyChar(self, char ch)` (0x1cc6e8): map the
/// character to a key index and select it when enabled.
pub fn ckeyboardpopup_handle_key_char(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x001c_c6e8 => {
                c.cycles += 9;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                c.g[A1][0] = sx(r32(c, A1) << 24);
                c.sq(S1, p(r32(c, SP), 0x10));
                c.g[A1][0] = sx(((r32(c, A1) as i32) >> 24) as u32);
                c.sq(S0, p(r32(c, SP), 0x20));
                c.g[S1][0] = c.g[A0][0];
                c.g[V0][0] = 0x60;
                let is_shift = c.g[A1][0] != c.g[V0][0];
                c.sd(r32(c, SP), c.g[RA][0]);
                if is_shift {
                    pc = 0x001c_c728;
                } else {
                    pc = 0x001c_c70c;
                }
            }
            0x001c_c70c => {
                c.cycles += 4;
                c.g[V1][0] = 0xe;
                c.g[V0][0] = 1;
                Addr(p(r32(c, S1), 0x140)).sw(c, c.g[V1][0]);
                pc = 0x001c_c7b0;
            }
            0x001c_c71c => {
                c.cycles += 3;
                Addr(p(r32(c, S1), 0x140)).sw(c, c.g[S0][0]);
                c.g[V0][0] = 1;
                pc = 0x001c_c7b0;
            }
            0x001c_c728 => {
                c.cycles += 8;
                c.g[V0][0] = 0x440000;
                c.g[V1][0] = 0x440000;
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(0x1390));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x13e8));
                c.g[S0][0] = 0;
                c.g[A3][0] = 0x2a;
                c.g[A2][0] = 0x40;
                pc = 0x001c_c748;
            }
            0x001c_c748 => {
                c.cycles += 4;
                c.g[V0][0] = sx(p(r32(c, S0), r32(c, T0)));
                c.g[A0][0] = Addr(r32(c, V0)).lb(c);
                if c.g[A0][0] == c.g[A1][0] {
                    pc = 0x001c_c764;
                } else {
                    pc = 0x001c_c758;
                }
            }
            0x001c_c758 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    pc = 0x001c_c7a0;
                } else {
                    pc = 0x001c_c764;
                }
            }
            0x001c_c764 => {
                c.cycles += 2;
                if c.g[A0][0] == c.g[A3][0] {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    pc = 0x001c_c7a0;
                } else {
                    pc = 0x001c_c76c;
                }
            }
            0x001c_c76c => {
                c.cycles += 2;
                if c.g[A0][0] != c.g[A2][0] {
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x001c_c780;
                } else {
                    pc = 0x001c_c774;
                }
            }
            0x001c_c774 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                let same = c.g[V0][0] == c.g[A0][0];
                c.g[A0][0] = c.g[S1][0];
                if same {
                    pc = 0x001c_c79c;
                } else {
                    pc = 0x001c_c780;
                }
            }
            0x001c_c780 => {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0;
                if call_ptr(c, 0x001c_e560, 0x001c_c78c).is_err() {
                    return;
                }
                let enabled = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if enabled {
                    if back_edge_unwinds(c, 0x001c_c71c) {
                        return;
                    }
                    pc = 0x001c_c71c;
                } else {
                    pc = 0x001c_c794;
                }
            }
            0x001c_c794 => {
                c.cycles += 2;
                c.lq(S0, p(r32(c, SP), 0x20));
                pc = 0x001c_c7b4;
            }
            0x001c_c79c => {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                pc = 0x001c_c7a0;
            }
            0x001c_c7a0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 84i64) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    if back_edge_unwinds(c, 0x001c_c748) {
                        return;
                    }
                    pc = 0x001c_c748;
                } else {
                    pc = 0x001c_c7ac;
                }
            }
            0x001c_c7ac => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                pc = 0x001c_c7b0;
            }
            0x001c_c7b0 => {
                c.cycles += 1;
                c.lq(S0, p(r32(c, SP), 0x20));
                pc = 0x001c_c7b4;
            }
            0x001c_c7b4 => {
                c.cycles += 4;
                c.lq(S1, p(r32(c, SP), 0x10));
                c.g[RA][0] = c.ld(r32(c, SP));
                let ra = r32(c, RA);
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x001c_c6e8, 0x001c_c7c4);
                return;
            }
        }
    }
}

/// `void KeyboardScreen_GetKeyObjectName(char* dst, int key, char* out)`
/// (0x1809b0): copy the key's object name (or its printable character) into
/// `out`.
pub fn keyboardscreen_get_key_object_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
        c.g[A0][0] = c.g[A2][0];
        c.sd(p(r32(c, SP), 0x10), c.g[RA][0]);
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = ((c.g[A1][0] as i64) < 10i64) as u64;
        c.g[T1][0] = sx(r32(c, V0).wrapping_sub(0x2ab0));
        c.ldl(6, p(r32(c, T1), 7));
        c.ldr(6, r32(c, T1));
        c.g[A3][0] = Addr(p(r32(c, T1), 8)).lb(c);
        c.g[T0][0] = Addr(p(r32(c, T1), 9)).lb(c);
        c.sdl(6, p(r32(c, A0), 7));
        c.sdr(6, r32(c, A0));
        Addr(p(r32(c, A0), 8)).sb(c, c.g[A3][0]);
        Addr(p(r32(c, A0), 9)).sb(c, c.g[T0][0]);
        let digit = c.g[V1][0] != 0;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x30));
        if !digit {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 36i64) as u64;
            let alphabet = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x37));
            if !alphabet {
                self::ks_special_chain(c)?;
                return Ok(());
            }
        }
        c.cycles += 6;
        Addr(p(r32(c, SP), 1)).sb(c, 0);
        Addr(r32(c, SP)).sb(c, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0041_62d0, 0x0018_0a0c)?;
        ks_finish(c);
        Ok(())
    })
}

/// The `@ : * #` branch of [`keyboardscreen_get_key_object_name`].
fn ks_special_chain(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = 0x24;
    let not_at = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x25;
    if !not_at {
        c.cycles += 5;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1350));
        call_ptr(c, 0x0041_62d0, 0x0018_0a2c)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_colon = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x26;
    if !not_colon {
        c.cycles += 5;
        c.g[A1][0] = 0x460000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2aa0));
        call_ptr(c, 0x0041_62d0, 0x0018_0a48)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_star = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x27;
    if !not_star {
        c.cycles += 5;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1358));
        call_ptr(c, 0x0041_62d0, 0x0018_0a64)?;
        ks_finish(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_hash = c.g[A1][0] != c.g[V0][0];
    c.g[RA][0] = c.ld(p(r32(c, SP), 0x10));
    if not_hash {
        ks_close(c);
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = 0x4a0000;
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1360));
    call_ptr(c, 0x0041_62d0, 0x0018_0a80)?;
    ks_finish(c);
    Ok(())
}

/// The shared return of [`keyboardscreen_get_key_object_name`]: `ra` is
/// already restored, add the trailing cycles and leave.
fn ks_close(c: &mut Cpu) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x20));
    c.pc = ra;
}

/// `ra = ld(sp+0x10); cycles += 2; sp += 0x20; return`.
fn ks_finish(c: &mut Cpu) {
    c.cycles += 2;
    c.g[RA][0] = c.ld(p(r32(c, SP), 0x10));
    ks_close(c);
}

// ---------------------------------------------------------------------------
// Reward gallery.

/// `void cFEStateRewardGallery_OpenBuyPopup(self)` (0x1d0510): create the
/// buy popup for the selected reward and its item parameters.
pub fn cfestaterewardgallery_open_buy_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 55;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x110)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x114)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, f, 0x001d_0540)?;
        c.g[A1][0] = 0x460000;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x790));
        c.g[A0][0] = 0x70;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x001d_055c)?;
        c.g[A3][0] = Addr(p(r32(c, S0), 0x44)).lb(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_aba8, 0x001d_0570)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr(p(r32(c, V1), 0x128)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x12c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, f, 0x001d_058c)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr(p(r32(c, V1), 0x118)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x11c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, f, 0x001d_05a8)?;
        c.g[T0][0] = Addr(p(r32(c, S0), 0x54)).lw(c);
        c.g[A0][0] = sx(p(r32(c, S3), 0x48));
        c.g[A1][0] = Addr(p(r32(c, S0), 0x4c)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[V0][0];
        call_ptr(c, 0x001c_aa18, 0x001d_05c0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = sx(p(r32(c, A0), 0x18));
        call_ptr(c, 0x0039_f290, 0x001d_05d0)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenInGameError_ShowContinueButton(self, int visible)`
/// (0x1fb378): find the error screen's buttons and show/hide the continue
/// button.
pub fn ovscreeningameerror_show_continue_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A0][0] = 0x470000;
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x4b8));
        call_ptr(c, 0x0031_7670, 0x001f_b3a0)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_b3ac)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 21;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x450));
            call_ptr(c, 0x0031_7670, 0x001f_b3c4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003a_1000, 0x001f_b3d0)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A1][0] = c.g[S1][0] ^ 1;
            c.g[A0][0] = Addr(p(r32(c, V0), 0x40)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x44)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x001f_b3e8)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x001f_b400)?;
            c.g[V0][0] = 4;
            Addr(p(r32(c, S0), 0x18)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2470));
        call_ptr(c, 0x0031_7670, 0x001f_b414)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_d860, 0x001f_b420)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.lq(S0, frame.at(c, 0x30));
        } else {
            c.cycles += 2;
            let hide = c.g[S1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !hide {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0039_b960, 0x001f_b43c)?;
            }
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// New player defaults.

/// `void BENewPlayerInt_v01(NewPlayer* self, int a1)` (0x1459b8): copy the
/// default player block into the global slot, or save the defaults when there
/// is no global slot.
pub fn benewplayerint_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = Addr(gp(c, -0x1f30)).lw(c);
        c.sd(r32(c, SP), c.g[RA][0]);
        c.g[V0][0] = Addr(p(r32(c, V1), 8)).lw(c);
        let save = c.g[V0][0] == 0;
        c.g[V0][0] = 0x530000;
        if save {
            c.cycles += 2;
            call_ptr(c, 0x0014_5a98, 0x0014_5a88)?;
        } else {
            c.cycles += 7;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = sx(p(r32(c, V0), 0x5b20));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x4fe0));
            c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
            c.g[V0][0] = c.g[V0][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[V0][0] = sx(p(r32(c, V1), 0xa0));
            if !aligned {
                loop {
                    c.cycles += 19;
                    c.ldl(5, p(r32(c, V1), 7));
                    c.ldr(5, r32(c, V1));
                    c.ldl(6, p(r32(c, V1), 0xf));
                    c.ldr(6, p(r32(c, V1), 8));
                    c.ldl(7, p(r32(c, V1), 0x17));
                    c.ldr(7, p(r32(c, V1), 0x10));
                    c.ldl(8, p(r32(c, V1), 0x1f));
                    c.ldr(8, p(r32(c, V1), 0x18));
                    c.sdl(5, p(r32(c, A0), 7));
                    c.sdr(5, r32(c, A0));
                    c.sdl(6, p(r32(c, A0), 0xf));
                    c.sdr(6, p(r32(c, A0), 8));
                    c.sdl(7, p(r32(c, A0), 0x17));
                    c.sdr(7, p(r32(c, A0), 0x10));
                    c.sdl(8, p(r32(c, A0), 0x1f));
                    c.sdr(8, p(r32(c, A0), 0x18));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0014_59ec);
                    }
                }
            }
            loop {
                c.cycles += 11;
                c.g[A1][0] = c.ld(r32(c, V1));
                c.g[A2][0] = c.ld(p(r32(c, V1), 8));
                c.g[A3][0] = c.ld(p(r32(c, V1), 0x10));
                c.g[T0][0] = c.ld(p(r32(c, V1), 0x18));
                c.sd(r32(c, A0), c.g[A1][0]);
                c.sd(p(r32(c, A0), 8), c.g[A2][0]);
                c.sd(p(r32(c, A0), 0x10), c.g[A3][0]);
                c.sd(p(r32(c, A0), 0x18), c.g[T0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_5a40);
                }
            }
            c.cycles += 5;
            c.ldl(2, p(r32(c, V1), 7));
            c.ldr(2, r32(c, V1));
            c.sdl(2, p(r32(c, A0), 7));
            c.sdr(2, r32(c, A0));
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(r32(c, SP));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sphere tree.

/// `int SphereTree_CollideTriangle(self, a1, a2, a3, t0, t1, t2)` (0x32ca78):
/// refresh the tree's cached bound, collide the triangle and report whether
/// the closest hit is in front.
pub fn spheretree_collide_triangle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[T1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[T2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S6][0] = c.g[A3][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0x98)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A0), 8)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0032_df28, 0x0032_cacc)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x98)).lw(c);
            Addr(p(r32(c, V1), 0x28)).sw(c, c.g[V0][0]);
        }
        c.cycles += 21;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0032_db40, 0x0032_cae0)?;
        Addr(p(r32(c, S0), 0x94)).sw(c, 0);
        c.g[A1][0] = c.g[S4][0];
        c.sw(r32(c, SP), c.g[S2][0]);
        c.g[A2][0] = c.g[S5][0];
        c.sw(p(r32(c, SP), 8), c.g[S3][0]);
        c.g[A3][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[T1][0] = sx(p(r32(c, S0), 0x80));
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        call_ptr(c, 0x0032_d470, 0x0032_cb10)?;
        c.lwc1(1, p(r32(c, S0), 0x94));
        c.mtc1(0, 0);
        c.c_lt_s(1, 0);
        let hit = c.fcc();
        c.g[V0][0] = 1;
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
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
    })
}

// ---------------------------------------------------------------------------
// Replay start.

/// `void Replay_BeginPlayback(Replay* self, int a1, int a2, int a3)`
/// (0x26f8a0): start the replay from a snapshot unless one is already
/// running, then switch the replay cameras and tell the audio system.
pub fn replay_begin_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let busy = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A3][0];
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x608)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?;
            }
            c.cycles += 13;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?;
            Addr(p(r32(c, S0), 0x618)).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            Addr(p(r32(c, S0), 0x61c)).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
            Addr(p(r32(c, S0), 0x620)).sw(c, c.g[S1][0]);
            call_ptr(c, 0x0027_02f8, 0x0026_f914)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0027_03f0, 0x0026_f920)?;
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ef30, 0x0026_f930)?;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ee68, 0x0026_f940)?;
            }
            c.cycles += 2;
            let have_a2 = c.g[S1][0] != 0;
            c.lq(S0, frame.at(c, 0x40));
            if have_a2 {
                c.cycles += 1;
                c.lq(S1, frame.at(c, 0x30));
            } else {
                c.cycles += 2;
                let have_a3 = c.g[S3][0] != 0;
                c.lq(S1, frame.at(c, 0x30));
                if !have_a3 {
                    c.cycles += 4;
                    call_ptr(c, 0x0028_b180, 0x0026_f958)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028_f200, 0x0026_f960)?;
                    c.cycles += 1;
                    c.lq(S0, frame.at(c, 0x40));
                    c.cycles += 1;
                    c.lq(S1, frame.at(c, 0x30));
                }
            }
        } else {
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x40));
            c.cycles += 1;
            c.lq(S1, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chunk resolver.

/// `int cPS2ChunkResolve_v05(self, void* chunk)` (0x3abe40): relocate the
/// chunk's two offsets and every pointer table entry, then finish the mesh
/// DMA relocation.
pub fn cps2chunkresolve_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, S0)));
        c.g[T0][0] = sx(p(r32(c, V1), r32(c, S0)));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[T0][0]);
        c.g[V0][0] = Addr(r32(c, T0)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = sx(p(r32(c, T0), 4));
        if !none {
            loop {
                c.cycles += 10;
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                c.g[A1][0] = Addr(r32(c, A2)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = c.g[A1][0] & 0xff;
                c.g[V1][0] = Addr(p(r32(c, A0), 8)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let empty = c.g[V1][0] == 0;
                c.g[V0][0] = sx(r32(c, A1) >> 8);
                let mut resolved = 0u64;
                if !empty {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(p(r32(c, V1), 4)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    if c.g[V0][0] != 0 {
                        resolved = sx(r32(c, V0) << 2);
                    }
                }
                c.cycles += 6;
                Addr(r32(c, A2)).sw(c, resolved);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, T0)).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(p(r32(c, A2), 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_be80);
                }
            }
        }
        c.cycles += 15;
        c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x34)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x003a_bef8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003a_bf20, 0x003a_bf04)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2ChunkResolve_RelocateInstanceDma(self, void* chunk)` (0x3ac358):
/// relocate the instance-DMA chain for every instance in the chunk.
pub fn cps2chunkresolve_relocate_instance_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S7, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[A0][0] = 0;
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S2), 0x98)).lw(c);
        c.g[S5][0] = Addr(p(r32(c, S2), 0x80)).lw(c);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, S2)));
        Addr(p(r32(c, S2), 0x98)).sw(c, c.g[V0][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[S6][0] = Addr(p(r32(c, S5), 4)).lw(c);
        let none = (c.g[S6][0] as i64) <= 0;
        c.g[V1][0] = Addr(p(r32(c, S5), 8)).lw(c);
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[S1][0] = Addr(p(r32(c, V1), 4)).lw(c);
                c.g[S4][0] = sx(r32(c, A0).wrapping_add(1));
                let no_obj = c.g[S1][0] == 0;
                c.g[S3][0] = sx(p(r32(c, V1), 0x10));
                let mut count_le = false;
                if !no_obj {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                    c.g[S0][0] = 0;
                    count_le = (c.g[V0][0] as i64) <= 0;
                } else {
                    count_le = true;
                }
                if !count_le {
                    c.cycles += 1;
                    loop {
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S7][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S5][0];
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        call_ptr(c, 0x003a_c508, 0x003a_c3e4)?;
                        c.g[V1][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                        c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let more = c.g[V1][0] != 0;
                        c.g[A3][0] = c.g[V0][0];
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_c3d0);
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.g[S3][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_c3b0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x80);
        c.g[V0][0] = 1;
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

// ---------------------------------------------------------------------------
// Buy attributes screen.

/// `void cFEStateBuyAttrib_Init(self)` (0x1f4830): create the screen, fetch
/// the character's attributes and refresh every displayed value.
pub fn cfestatebuyattrib_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x1008));
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_4854)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_4868)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x48)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_4880)?;
        }
        c.cycles += 35;
        call_ptr(c, 0x0014_dc80, 0x001f_4888)?;
        c.g[A1][0] = 0xb;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001f_4894)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x001f_48a0)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0015_0928, 0x001f_48b0)?;
        Addr(p(r32(c, S1), 0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(p(r32(c, S1), 0xa0)).sw(c, 0);
        Addr(p(r32(c, S1), 0xa4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x001f_5240, 0x001f_48c4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_4ea8, 0x001f_48cc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_4fc0, 0x001f_48d4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_50b0, 0x001f_48dc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_52a0, 0x001f_48e4)?;
        call_ptr(c, 0x0028_b180, 0x001f_48ec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0028_f140, 0x001f_48f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rail manager.

/// `RailMan* RailMan_ctor_2(RailMan* self, Stream* stream, void* arg)`
/// (0x358588): init the sync process, clear the 128 rail handles, read the
/// handle table and disable every used rail.
pub fn railman_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0035_48c0, 0x0035_85ac)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1220));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p(r32(c, S2), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(p(r32(c, S2), 0x18));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_85d0);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A3][0] = sx(p(r32(c, S2), 0x18));
        Addr(gp(c, 0xf50)).sw(c, c.g[S2][0]);
        c.g[A1][0] = c.g[A3][0];
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[A2][0] = 0x200;
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        let read = r32(c, V1);
        c.g[S1][0] = c.g[A3][0];
        call_ptr(c, read, 0x0035_8618)?;
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            let empty = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0035_8780, 0x0035_8634)?;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 128i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(p(r32(c, S1), 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8620);
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
    })
}

// ---------------------------------------------------------------------------
// Request line: song count / slot icons.

/// `void cFEStateRequestLine_UpdateSongCount(self)` (0x198118): format the
/// number of set mask bits into the song-count label.
pub fn cfestaterequestline_update_song_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x460000;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5d8));
        c.g[S0][0] = Addr(p(r32(c, V0), 0x8c)).lw(c);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x20)).lh(c);
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V0)));
        call_ptr(c, 0x0031_7670, 0x0019_8150)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x24)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        let f = r32(c, V1);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, f, 0x0019_8160)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x003d_bca0, 0x0019_816c)?;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(5));
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(r32(c, V0) << 1);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x688));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0019_8188)?;
        c.g[A1][0] = c.ld(p(r32(c, S2), 0x158));
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.g[A0][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (r32(c, A0) & 63);
            c.g[V0][0] = c.g[V0][0] & 1;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[A2][0] = sx(p(r32(c, V0), r32(c, A2)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8198);
            }
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002c_26d0, 0x0019_81c0)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x1e4)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003a_0e90, 0x0019_81cc)?;
        if c.g[S0][0] == 0 {
            c.lq(S0, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e98, 0x0019_81dc)?;
            c.lq(S0, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRequestLine_UpdateSlotIcons(self)` (0x1985f0): show the
/// icons for the slots the bit mask selects, hide the rest.
pub fn cfestaterequestline_update_slot_icons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[A0][0] = 0;
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.ld(p(r32(c, S2), 0x160));
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (r32(c, A0) & 63);
            c.g[V0][0] = c.g[V0][0] & 1;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[S3][0] = sx(p(r32(c, V0), r32(c, S3)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8620);
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0;
        c.g[S4][0] = 6;
        c.g[S0][0] = sx(p(r32(c, S2), 0x1bc));
        loop {
            c.cycles += 3;
            c.g[A2][0] = Addr(r32(c, S0)).lw(c);
            let no_icon = c.g[A2][0] == 0;
            c.g[V0][0] = sx(r32(c, S4).wrapping_sub(r32(c, S1)));
            if no_icon {
                c.cycles += 1;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(p(r32(c, S2), 0x16c)).lw(c);
                c.g[A1][0] = 0;
                let disabled = c.g[V1][0] == 0;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if disabled {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                    c.cycles += 4;
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                    let f = r32(c, V1);
                    c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                    call_ptr(c, f, 0x0019_869c)?;
                    c.cycles += 1;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    let few = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                    if few {
                        c.cycles += 4;
                        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                        let f = r32(c, V1);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, f, 0x0019_869c)?;
                        c.cycles += 1;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = 1;
                        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                        let f = r32(c, V1);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, f, 0x0019_8680)?;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S0), 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8648);
            }
        }
        c.cycles += 8;
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

// ---------------------------------------------------------------------------
// UI state stack.

/// `void UIStateMgr_SwitchStateImmediate(mgr, st, a2, a3)` (0x39f320): close
/// the current state when it is a popup of id 5, then push `st` and mark its
/// flags.
pub fn uistatemgr_switch_state_immediate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        call_ptr(c, 0x0039_fa88, 0x0039_f34c)?;
        c.g[A1][0] = c.g[V0][0];
        let mut plus_one = false;
        if c.g[A1][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
            c.g[S3][0] = 1;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x200;
            Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
            plus_one = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = Addr(p(r32(c, A1), 0x1c)).lw(c);
            c.g[V0][0] = sx(((r32(c, A0) as i32) >> 5) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            let not_popup = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32(c, A0) >> 8);
            if not_popup {
                plus_one = true;
            } else {
                c.cycles += 4;
                c.g[V1][0] = 5;
                c.g[V0][0] = c.g[V0][0] & 0x3f;
                if c.g[V0][0] != c.g[V1][0] {
                    // not the buy popup: nothing to close
                } else {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[V1][0] = c.g[S1][0] & 1;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                    c.g[V1][0] = sx(r32(c, V1) << 7);
                    c.g[A0][0] = 0xffff_ffff_ffff_ff7f;
                    c.g[V0][0] = c.g[V0][0] | 0x700;
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    Addr(p(r32(c, A1), 0x1c)).sw(c, c.g[V0][0]);
                    plus_one = true;
                }
            }
        }
        if plus_one {
            c.cycles += 1;
        }
        c.g[A0][0] = c.g[S2][0];
        c.cycles += 16;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f3c4)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        c.g[V1][0] = c.g[S3][0] & 1;
        c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
        c.g[V1][0] = sx(r32(c, V1) << 5);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        frame.restore(c, S2, 0x20);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider model.

/// `void RiderModel_HidePart(self, int part, int channel)` (0x30ea80): clear
/// a part's channel LOD and recompute the visibility mask.
pub fn rider_model_hide_part(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = Addr(p(r32(c, A3), 0x14)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        let ra = r32(c, RA);
        c.pc = ra;
        return;
    }
    'outer: {
        c.cycles += 3;
        c.g[A1][0] = Addr(p(r32(c, A3), 0x1c)).lw(c);
        let have_index = c.g[A1][0] != 0;
        c.g[V0][0] = sx(r32(c, V1) << 2);
        if !have_index {
            break 'outer;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, A1)));
        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[V1][0] = 0x58;
        if negative {
            break 'outer;
        }
        c.cycles += 9;
        c.g[A1][0] = Addr(p(r32(c, A3), 0xc)).lw(c);
        c.mult(0, 3, 2, 3);
        c.g[A0][0] = c.ld(p(r32(c, A3), 0x150));
        c.g[A1][0] = sx(p(r32(c, A1), r32(c, V1)));
        c.g[V0][0] = c.ld(p(r32(c, A1), 0x10));
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        let channel_ok = (c.g[A2][0] as i64) >= 0;
        c.sd(p(r32(c, A3), 0x150), c.g[A0][0]);
        if channel_ok {
            'found: {
                c.cycles += 8;
                c.g[V0][0] = sx(r32(c, A2) << 3);
                c.g[V1][0] = sx(p(r32(c, A1), 0x18));
                c.g[A2][0] = c.g[V1][0];
                c.g[V1][0] = sx(p(r32(c, V1), r32(c, V0)));
                Addr(r32(c, V1)).sw(c, 0);
                c.g[V0][0] = Addr(p(r32(c, A1), 0x18)).lw(c);
                c.g[A0][0] = 0;
                if c.g[V0][0] != 0 {
                    break 'found;
                }
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    let stop = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, A0) << 3);
                    if stop {
                        break 'outer;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(p(r32(c, A2), r32(c, V0)));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                    if !empty {
                        break 'found;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_eb28);
                    }
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.ld(p(r32(c, A1), 0x10));
            c.g[V0][0] = c.ld(p(r32(c, A3), 0x150));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.sd(p(r32(c, A3), 0x150), c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(p(r32(c, A1), 0x30));
            c.g[V1][0] = 3;
            loop {
                c.cycles += 7;
                Addr(r32(c, V0)).sw(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_eae0);
                }
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Script animation bank manager.

/// `ScriptAnimBankMan* ScriptAnimBankMan_ctor(self, base, count, flags)`
/// (0x2753b8): allocate the bank array and initialise each entry.
pub fn script_anim_bank_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
        c.g[T0][0] = c.g[A2][0];
        c.sq(S1, p(r32(c, SP), 0x10));
        c.g[V1][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, p(r32(c, SP), 0x20));
        c.g[A0][0] = 0x14;
        c.g[V0][0] = 0x480000;
        c.mult(0, 0, 8, 4);
        c.sd(r32(c, SP), c.g[RA][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x25d8));
        c.g[S0][0] = c.g[A3][0];
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = c.g[V1][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1a80));
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.lo[0];
        Addr(p(r32(c, S1), 0xc)).sw(c, c.g[V0][0]);
        c.g[A3][0] = 0;
        Addr(p(r32(c, S1), 4)).sw(c, c.g[T0][0]);
        call_ptr(c, 0x0031_7e30, 0x0027_540c)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 4)).lw(c);
        c.g[A1][0] = 0;
        let none = (c.g[V1][0] as i64) <= 0;
        Addr(p(r32(c, S1), 8)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 3;
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = 0;
            loop {
                c.cycles += 21;
                c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = sx(p(r32(c, A0), r32(c, V1)));
                Addr(r32(c, V1)).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
                c.g[V0][0] = sx(p(r32(c, A0), r32(c, V0)));
                Addr(p(r32(c, V0), 4)).sw(c, c.g[S0][0]);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
                c.g[V0][0] = sx(p(r32(c, A0), r32(c, V0)));
                Addr(p(r32(c, V0), 8)).sw(c, 0);
                c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
                c.g[V1][0] = sx(p(r32(c, A0), r32(c, V1)));
                Addr(p(r32(c, V1), 0xc)).sw(c, 0);
                c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
                c.g[V0][0] = sx(p(r32(c, A0), r32(c, V0)));
                Addr(p(r32(c, V0), 0x10)).sw(c, 0);
                c.g[V0][0] = Addr(p(r32(c, S1), 4)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5428);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        c.lq(S0, p(r32(c, SP), 0x20));
        c.lq(S1, p(r32(c, SP), 0x10));
        c.g[RA][0] = c.ld(r32(c, SP));
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE library.

/// `BELibrary::~BELibrary(this, int flags)` (0x1499a8): free every singleton
/// the library owns and optionally the library itself.
pub fn belibrary_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x1f3c)).lw(c);
        c.sd(r32(c, SP), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_7e50, 0x0014_99c8)?;
        c.g[A0][0] = Addr(gp(c, -0x1f30)).lw(c);
        Addr(gp(c, -0x1f3c)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99d4)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee8)).lw(c);
        Addr(gp(c, -0x1f30)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99e0)?;
        c.g[A0][0] = Addr(gp(c, -0x1f2c)).lw(c);
        Addr(gp(c, -0x1ee8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99ec)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee4)).lw(c);
        Addr(gp(c, -0x1f2c)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_99f8)?;
        Addr(gp(c, -0x1ee4)).sw(c, 0);
        call_ptr(c, 0x0015_3050, 0x0014_9a00)?;
        c.g[A0][0] = Addr(gp(c, -0x1ee0)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0014_9a08)?;
        c.g[A0][0] = Addr(gp(c, -0x1eec)).lw(c);
        Addr(gp(c, -0x1ee0)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a14)?;
        c.g[A0][0] = Addr(gp(c, -0x1ea8)).lw(c);
        Addr(gp(c, -0x1eec)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a20)?;
        c.g[A0][0] = Addr(gp(c, -0x1f28)).lw(c);
        Addr(gp(c, -0x1ea8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a2c)?;
        c.g[A0][0] = Addr(gp(c, -0x1eb8)).lw(c);
        Addr(gp(c, -0x1f28)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a38)?;
        c.g[A0][0] = Addr(gp(c, -0x1edc)).lw(c);
        Addr(gp(c, -0x1eb8)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a44)?;
        c.g[A0][0] = Addr(gp(c, -0x1ea4)).lw(c);
        Addr(gp(c, -0x1edc)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a50)?;
        c.g[A0][0] = Addr(gp(c, -0x1e8c)).lw(c);
        Addr(gp(c, -0x1ea4)).sw(c, 0);
        call_ptr(c, 0x0031_7e50, 0x0014_9a5c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        Addr(gp(c, -0x1e8c)).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0014_9a70)?;
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
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::{check, check_lohi, put_vec, Rng, DATA, GAME_GP, SP};
    use ssx3_ee as ee;

    const BE: u32 = DATA + 0x3_0000;
    const WORLD: u32 = DATA + 0x8_0000;
    const SYS: u32 = DATA + 0x10_0000;
    const ARR: u32 = DATA + 0x11_0000;
    const SI: u32 = DATA + 0x11_1000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn wf(c: &mut Cpu, a: u32, f: f32) {
        c.bus.write32(a, f.to_bits());
    }

    fn clear(c: &mut Cpu, a: u32, n: u32) {
        for i in 0..n {
            c.bus.write32(a + 4 * i, 0);
        }
    }

    /// The BELibrary singleton (a zeroed dummy, so nothing reads the disc)
    /// and the world (`gp - 0x848`), game module and race manager.
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let p = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, p + head, base);
            w(c, GAME_GP + slot, p);
        }
    }

    /// A tiny trampoline for setup calls into guest code.
    fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, 0x0012_3450, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap arena over 256 KiB of test RAM, so constructors that
    /// allocate take their normal path.
    fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1);
    }

    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        alloc_ready(c);
        globals(c);
        pool(c);
        r
    }

    /// The audio system singleton and its speech-context list.
    fn audio_system(c: &mut Cpu, seed: u32) {
        w(c, SYS + 0x5560 + 4, ARR);
        w(c, ARR, SI);
        w(c, SI + 4, 0);
        w(c, GAME_GP.wrapping_add(0x6c4), 0);
        w(c, GAME_GP.wrapping_add(0x173c), 0);
        if seed % 4 == 0 {
            for ch in 0..4u32 {
                wf(c, SYS + 0x62bc + 0x10 * ch, 1.0);
            }
        }
        if seed % 3 == 0 {
            w(c, ARR, 0);
        }
        w(c, SYS + 0x5780, 0);
    }

    #[test]
    fn ssx_audio_system_dj_say_20ca_matches() {
        check(
            ee::s08::f_002a2188__generated,
            ssx_audio_system_dj_say_20ca,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x2a2188, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_20cf_matches() {
        check(
            ee::s08::f_002a2860__generated,
            ssx_audio_system_dj_say_20cf,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x2a2860, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = (seed % 200) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_2102_matches() {
        check(
            ee::s08::f_002a2938__generated,
            ssx_audio_system_dj_say_2102,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x2a2938, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
            },
        );
    }

    #[test]
    fn script_engine_on_bank_loaded_matches() {
        check(
            ee::s07::f_0027b5d8__generated,
            script_engine_on_bank_loaded,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x27b5d8, 2);
                let this = DATA;
                clear(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let bank = (seed % 5) as u64;
                c.g[A1][0] = bank;
                w(c, this + 0x620, 1);
                for i in 0..10u32 {
                    let slot = this + 0x628 + 0x10 * i;
                    w(c, slot, (i + 1) as u32);
                    w(c, slot + 4, i);
                    w(
                        c,
                        slot + 8,
                        if seed % 2 == 0 && i == (seed % 10) {
                            bank as u32
                        } else {
                            0x99
                        },
                    );
                }
                w(c, SP - 0xf0 + 0x84, 0);
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
                let mut r = base(c, seed, 0x2cab08, 3);
                let this = DATA;
                clear(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let count = 4u32;
                w(c, this, if seed % 2 == 0 { 0 } else { count - 1 });
                w(c, this + 4, count);
                let vt = vtable(c, &mut r, &[0x18, 0x30, 0x38]);
                for i in 0..count {
                    let item = DATA + 0x200 + 0x80 * i;
                    clear(c, item, 0x40);
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
                let mut r = base(c, seed, 0x2cabe0, 3);
                let this = DATA;
                clear(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let count = 4u32;
                w(c, this, if seed % 2 == 0 { 0 } else { 1 });
                w(c, this + 4, count);
                let vt = vtable(c, &mut r, &[0x18, 0x30, 0x38]);
                for i in 0..count {
                    let item = DATA + 0x200 + 0x80 * i;
                    clear(c, item, 0x40);
                    w(c, item + 0x10, vt);
                }
                for i in 0..count {
                    w(c, this + 0xc + 4 * i, DATA + 0x200 + 0x80 * i);
                }
            },
        );
    }

        #[test]
    fn replay_on_rider_checkpoint_matches() {
        check(
            ee::s07::f_00270ab0__generated,
            replay_on_rider_checkpoint,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x270ab0, 2);
                let this = DATA;
                clear(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = if seed % 4 == 0 {
                    0xffff_fffe
                } else {
                    (seed % 4) as u64
                };
                w(c, this + 0x61c, if seed % 5 == 0 { 1 } else { 0 });
                w(c, this + 0x620, if seed % 5 == 1 { 1 } else { 0 });
                w(c, this + 0x610, if seed % 5 == 2 { 1 } else { 0 });
                w(c, this + 0x60c, if seed % 5 == 3 { 1 } else { 0 });
                w(c, this + 0x10, 8);
                w(c, GAME_GP.wrapping_sub(0x848), WORLD);
            },
        );
    }

    #[test]
    fn game_camera_dtor_matches() {
        check(ee::s01::f_0015dd88__generated, game_camera_dtor, 12, &|c, seed| {
            let _ = base(c, seed, 0x15dd88, 2);
            let this = DATA;
            clear(c, this, 0x400);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0;
            w(c, this + 0xa4, if seed % 2 == 0 { 0 } else { DATA + 0x200 });
            w(c, this + 0xa8, 0);
            w(c, this + 0xac, 0);
            clear(c, DATA + 0x200, 0x100);
            w(c, DATA + 0x200 + 0x14, 0);
        });
    }

    #[test]
    fn mcmanager_state27_format_matches() {
        check(
            ee::s06::f_00240ab0__generated,
            mcmanager_state27_format,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x240ab0, 3);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let iface = DATA + 0x800;
                clear(c, iface, 0x100);
                w(c, this + 0x434, iface);
                let vt = vtable(c, &mut r, &[0x58, 0x1a8]);
                w(c, iface, vt);
                w(c, this + 0x88, 0);
                w(c, this + 0x748, 0);
                w(c, this + 0x6c, 0);
                w(c, this + 0x84, 0);
            },
        );
    }

    #[test]
    fn wscriptman_load_process_list_matches() {
        check(
            ee::s10::f_0030bee8__generated,
            wscriptman_load_process_list,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x30bee8, 2);
                let this = DATA;
                clear(c, this, 0x400);
                let stream = DATA + 0x800;
                clear(c, stream, 0x100);
                let list = DATA + 0xc00;
                clear(c, list, 0x100);
                let vt = vtable(c, &mut r, &[0x10]);
                w(c, stream, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = list as u64;
                c.g[A2][0] = stream as u64;
                // the stream read leaves the count where it is written
                w(c, SP - 0x70, 0);
            },
        );
    }

    #[test]
    fn boltfile_cache_init_matches() {
        check_lohi(ee::s03::f_0019ce68__generated, boltfile_cache_init, 8, &|c, seed| {
            let _ = base(c, seed, 0x19ce68, 2);
            // A large private region for the 10x256 entry table.
            let base_at = 0x0100_0000u32;
            c.g[A0][0] = base_at as u64;
            w(
                c,
                GAME_GP.wrapping_sub(0x1788),
                if seed % 2 == 0 { DATA + 0x2_0000 } else { 0 },
            );
            clear(c, DATA + 0x2_0000, 0x3000);
        });
    }

    #[test]
    fn rider_estimate_finish_time_matches() {
        check(
            ee::s00::f_00122d78__generated,
            rider_estimate_finish_time,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x122d78, 2);
                let this = DATA;
                clear(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let n = if seed % 3 == 0 { 0 } else { 60 };
                w(c, WORLD + 0x84, WORLD + 0x1000);
                w(c, WORLD + 0x1000 + 0xc, WORLD + 0x2000);
                w(c, WORLD + 0x2000 + 0xc, n);
                wf(c, this + 0x4d0, if seed % 4 == 0 { 0.0 } else { 100.0 });
                let vt = vtable(c, &mut r, &[0x38]);
                w(c, this + 0x6c0, vt);
                wf(c, this + 0x520, 30.0);
            },
        );
    }

                #[test]
    fn playbackman_start_playback_matches() {
        check(
            ee::s07::f_0026cc48__generated,
            playbackman_start_playback,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x26cc48, 2);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                match seed % 3 {
                    0 => w(c, this + 0x70, 1),
                    1 => {
                        w(c, this + 4, DATA + 0x200);
                        w(c, DATA + 0x200 + 0xc, 0);
                    }
                    _ => {
                        w(c, this + 4, DATA + 0x200);
                        w(c, DATA + 0x200 + 0xc, 0);
                    }
                }
            },
        );
    }

    #[test]
    fn crowdman2d_load_state_matches() {
        check(ee::s05::f_00229e58__generated, crowdman2d_load_state, 8, &|c, seed| {
            let mut r = base(c, seed, 0x229e58, 2);
            let this = DATA;
            clear(c, this, 0x1000);
            let state = DATA + 0x800;
            clear(c, state, 0x100);
            let vt = vtable(c, &mut r, &[0x10]);
            w(c, state, vt);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = state as u64;
            for i in 0..128u32 {
                w(c, SP - 0x250 + 4 * i, 0xffff_ffff);
            }
        });
    }

    #[test]
    fn cfestatetitle_update_matches() {
        check_lohi(ee::s02::f_001948a8__generated, cfestatetitle_update, 12, &|c, seed| {
            let _ = base(c, seed, 0x1948a8, 2);
            let this = DATA;
            clear(c, this, 0x200);
            c.g[A0][0] = this as u64;
            w(
                c,
                this + 0x48,
                match seed % 3 {
                    0 => 0,
                    1 => 1800,
                    _ => 1801,
                },
            );
            w(c, GAME_GP.wrapping_sub(0x1724), 0);
            // two empty lists under the world
            let l0 = DATA + 0x1000;
            let l1 = DATA + 0x2000;
            clear(c, l0, 0x40);
            clear(c, l1, 0x40);
            w(c, WORLD + 0xb0, l0);
            w(c, WORLD + 0xb4, l1);
            w(c, l0, 0);
            w(c, l1, 0);
        });
    }

    #[test]
    fn cfestatemainmenu_update_matches() {
        check(
            ee::s02::f_00194dd0__generated,
            cfestatemainmenu_update,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x194dd0, 3);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                w(c, this + 0x10, DATA + 0x200);
                w(c, DATA + 0x200 + 0xc, DATA + 0x300);
                let vt = vtable(c, &mut r, &[0x160, 0x168]);
                w(c, DATA + 0x300 + 8, vt);
                w(c, this + 0x55, (seed % 3) as u32);
                w(c, this + 0x40, 0);
            },
        );
    }

    #[test]
    fn cfestateubertrick_set_button_icon_matches() {
        check(
            ee::s02::f_00183d08__generated,
            cfestateubertrick_set_button_icon,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x183d08, 3);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let rect = DATA + 0x200;
                clear(c, rect, 0x100);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { rect as u64 };
                c.f[12] = if seed % 2 == 0 {
                    1.0f32.to_bits()
                } else {
                    0.5f32.to_bits()
                };
                wf(c, rect + 0xc, 1.0);
                wf(c, rect + 0x10, 2.0);
                wf(c, rect + 0x14, 3.0);
                wf(c, rect + 0x18, 4.0);
                let vt = vtable(c, &mut r, &[0xa0, 0xa8, 0x48]);
                w(c, this + 8, vt);
            },
        );
    }

    #[test]
    fn floating_v49_matches() {
        check(ee::s11::f_0034f048__generated, floating_v49, 12, &|c, seed| {
            let _ = base(c, seed, 0x34f048, 2);
            let this = DATA;
            clear(c, this, 0x400);
            c.g[A0][0] = this as u64;
            w(c, this + 0x78, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
            w(c, this + 0x18, DATA + 0x300);
            let info = DATA + 0x200;
            clear(c, info, 0x100);
            for k in 0..8u32 {
                wf(c, info + 4 * k, 1.0 + k as f32);
            }
            let inst = DATA + 0x300;
            clear(c, inst, 0x100);
            for k in 0..8u32 {
                wf(c, inst + 0x60 + 4 * k, 0.25 * k as f32);
            }
        });
    }

    #[test]
    fn messagecenter_get_list_index_matches() {
        check(
            ee::s04::f_001e3680__generated,
            messagecenter_get_list_index,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x1e3680, 2);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x102c),
                    if seed % 4 == 0 { 0 } else { 1 },
                );
            },
        );
    }

    #[test]
    fn rollermodifier_get_pose_matches() {
        check(
            ee::s11::f_0035e770__generated,
            rollermodifier_get_pose,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x35e770, 0);
                let dst = DATA;
                let src = DATA + 0x400;
                clear(c, dst, 0x100);
                clear(c, src, 0x400);
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                for k in 0..0x40u32 {
                    let v = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
                    put_vec(c, src + 4 * k, v);
                }
                c.bus
                    .plat
                    .set_acc([r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            },
        );
    }

    #[test]
    fn vramheap_free_all_matches() {
        check_lohi(ee::s12::f_00366618__generated, vramheap_free_all, 8, &|c, seed| {
            let _ = base(c, seed, 0x366618, 2);
            let this = DATA;
            clear(c, this, 0x4000);
            c.g[A0][0] = this as u64;
            w(c, this + 0x1ff0, DATA + 0x3000);
            let i = if seed % 2 == 0 { 0 } else { 0xffff_fffe };
            w(c, this + 0x23c0, i);
            // one node: next = -1, flags clear
            let node = DATA + 0x3000 + i.wrapping_mul(0x1c);
            w(c, node + 0x10, 0xffff_ffff);
            w(c, node, if seed % 2 == 0 { 0 } else { 2 });
        });
    }

    #[test]
    fn cps2memcard_mark_slots_without_space_matches() {
        check(
            ee::s08::f_002c6280__generated,
            cps2memcard_mark_slots_without_space,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x2c6280, 3);
                let this = DATA;
                clear(c, this, 0x2000);
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[0xa0, 0xb0, 0x218]);
                w(c, this, vt);
                w(c, vt + 0x21c, 0x002c_c098);
                w(c, this + 0x24, (seed % 8) as u32);
                w(c, this + 0x14, 2);
                w(c, this + 0x150, (seed % 3) as u32);
                wf(c, this + 0x100, 2.0);
                wf(c, this + 0x104, 1.0);
                for i in 0..2u32 {
                    let e = this + 0x1ac + 0x78 * i;
                    w(c, e, 0);
                }
            },
        );
    }

    #[test]
    fn mcmanager_state18_matches() {
        check(ee::s06::f_00240688__generated, mcmanager_state18, 12, &|c, seed| {
            let mut r = base(c, seed, 0x240688, 3);
            let this = DATA;
            clear(c, this, 0x400);
            c.g[A0][0] = this as u64;
            let iface = DATA + 0x800;
            clear(c, iface, 0x100);
            w(c, this + 0x434, iface);
            let vt = vtable(c, &mut r, &[0x58, 0x1a8, 0x248]);
            w(c, iface, vt);
            w(c, this + 0x88, 0);
            w(c, this + 0x748, 0);
            w(c, this + 0x68, 0);
            w(c, this + 0x6c, 0);
        });
    }

        #[test]
    fn ckeyboardpopup_handle_key_char_matches() {
        check(
            ee::s04::f_001cc6e8__generated,
            ckeyboardpopup_handle_key_char,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x1cc6e8, 2);
                let this = DATA;
                clear(c, this, 0x200);
                c.pc = 0x001c_c6e8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0x60,
                    1 => 0,
                    2 => 0x2a,
                    _ => (seed % 90) as u64,
                };
                for i in 0..84u32 {
                    w(c, 0x0044_1390 + 4 * i, 0);
                    w(c, 0x0044_13e8 + 4 * i, 0);
                }
            },
        );
    }

    #[test]
    fn keyboardscreen_get_key_object_name_matches() {
        check(
            ee::s02::f_001809b0__generated,
            keyboardscreen_get_key_object_name,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x1809b0, 2);
                let dst = DATA;
                let out = DATA + 0x100;
                clear(c, dst, 0x100);
                clear(c, out, 0x100);
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => 5,
                    2 => 10,
                    3 => 0x24,
                    4 => 0x25,
                    _ => (seed % 64) as u64,
                };
                c.g[A2][0] = out as u64;
            },
        );
    }

    #[test]
    fn ovscreeningameerror_show_continue_button_matches() {
        check(
            ee::s04::f_001fb378__generated,
            ovscreeningameerror_show_continue_button,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x1fb378, 2);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, this + 0x40, 0);
            },
        );
    }

    #[test]
    fn benewplayerint_v01_matches() {
        check(ee::s01::f_001459b8__generated, benewplayerint_v01, 8, &|c, seed| {
            let _ = base(c, seed, 0x1459b8, 2);
            let this = DATA;
            clear(c, this, 0x1000);
            c.g[A0][0] = this as u64;
            w(c, GAME_GP.wrapping_sub(0x1f30), this + 0x200);
            w(c, this + 0x208, if seed % 2 == 0 { 0 } else { 1 });
            clear(c, DATA + 0x535b20u32.wrapping_sub(DATA), 0x200);
        });
    }

    #[test]
    fn spheretree_collide_triangle_matches() {
        check(
            ee::s10::f_0032ca78__generated,
            spheretree_collide_triangle,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x32ca78, 2);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = (seed % 2) as u64;
                c.g[T1][0] = (seed % 3) as u64;
                c.g[T2][0] = (seed % 5) as u64;
                w(c, this + 0x98, if seed % 3 == 0 { 0 } else { DATA + 0x500 });
                clear(c, DATA + 0x500, 0x100);
                wf(c, this + 0x94, 0.0);
            },
        );
    }

        #[test]
    fn cps2chunkresolve_v05_matches() {
        check(ee::s13::f_003abe40__generated, cps2chunkresolve_v05, 8, &|c, seed| {
            let mut r = base(c, seed, 0x3abe40, 2);
            let this = DATA;
            let chunk = DATA + 0x200;
            clear(c, this, 0x400);
            clear(c, chunk, 0x400);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = chunk as u64;
            w(c, chunk + 8, 0x10);
            w(c, chunk + 0xc, 0x20);
            w(c, chunk + 0x20, if seed % 2 == 0 { 0 } else { 0 });
            let vt = vtable(c, &mut r, &[0x30]);
            w(c, this + 8, vt);
        });
    }

    #[test]
    fn cps2chunkresolve_relocate_instance_dma_matches() {
        check(
            ee::s13::f_003ac358__generated,
            cps2chunkresolve_relocate_instance_dma,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x3ac358, 2);
                let this = DATA;
                let chunk = DATA + 0x200;
                clear(c, this, 0x400);
                clear(c, chunk, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = chunk as u64;
                w(c, chunk + 0x98, 0x10);
                w(c, chunk + 0x80, DATA + 0x600);
                clear(c, DATA + 0x600, 0x100);
                w(
                    c,
                    DATA + 0x600 + 4,
                    if seed % 2 == 0 { 0 } else { 0xffff_ffff },
                );
                w(c, DATA + 0x600 + 8, 0);
            },
        );
    }

    #[test]
    fn cfestatebuyattrib_init_matches() {
        check(
            ee::s04::f_001f4830__generated,
            cfestatebuyattrib_init,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x1f4830, 3);
                let this = DATA;
                clear(c, this, 0x800);
                c.g[A0][0] = this as u64;
                w(c, this + 0x10, DATA + 0x400);
                w(c, this + 0x44, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn railman_ctor_2_matches() {
        check(ee::s11::f_00358588__generated, railman_ctor_2, 8, &|c, seed| {
            let mut r = base(c, seed, 0x358588, 2);
            let this = DATA;
            clear(c, this, 0x1000);
            let stream = DATA + 0x800;
            clear(c, stream, 0x100);
            let vt = vtable(c, &mut r, &[0x10]);
            w(c, stream, vt);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            c.g[A2][0] = (seed % 2) as u64;
            for i in 0..128u32 {
                w(c, this + 0x18 + 4 * i, 0xffff_ffff);
            }
        });
    }

    #[test]
    fn cfestaterequestline_update_song_count_matches() {
        check(
            ee::s02::f_00198118__generated,
            cfestaterequestline_update_song_count,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x198118, 3);
                let this = DATA;
                clear(c, this, 0x800);
                c.g[A0][0] = this as u64;
                let db = DATA + 0x400;
                clear(c, db, 0x100);
                w(c, WORLD + 0x8c, db);
                w(c, db + 4, DATA + 0x500);
                w(c, DATA + 0x500 + 0x20, 0);
                let vt = vtable(c, &mut r, &[0x20]);
                w(c, DATA + 0x500 + 8, vt);
                let _ = vt;
                w(c, this + 0x158, (seed % 4) as u32);
                w(c, this + 0x1e4, 0);
            },
        );
    }

    #[test]
    fn cfestaterequestline_update_slot_icons_matches() {
        check(
            ee::s03::f_001985f0__generated,
            cfestaterequestline_update_slot_icons,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x1985f0, 3);
                let this = DATA;
                clear(c, this, 0x800);
                c.g[A0][0] = this as u64;
                w(c, this + 0x160, (seed % 8) as u32);
                w(c, this + 0x16c, (seed % 2) as u32);
                let vt = vtable(c, &mut r, &[0x48]);
                for i in 0..6u32 {
                    let icon = DATA + 0x200 + 0x80 * i;
                    clear(c, icon, 0x40);
                    if (seed as u32 + i) % 3 != 0 {
                        w(c, this + 0x1bc + 4 * i, icon);
                        w(c, icon + 8, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn uistatemgr_switch_state_immediate_matches() {
        check(
            ee::s13::f_0039f320__generated,
            uistatemgr_switch_state_immediate,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x39f320, 2);
                let mgr = DATA;
                let st = DATA + 0x200;
                clear(c, mgr, 0x200);
                clear(c, st, 0x200);
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = st as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn rider_model_hide_part_matches() {
        check_lohi(
            ee::s10::f_0030ea80__generated,
            rider_model_hide_part,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x30ea80, 0);
                let this = DATA;
                clear(c, this, 0x800);
                c.g[A0][0] = this as u64;
                let part = (seed % 4) as u64;
                c.g[A1][0] = part;
                c.g[A2][0] = if seed % 3 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    (seed % 4) as u64
                };
                w(c, this + 0x14, if seed % 5 == 0 { 0 } else { 3 });
                w(c, this + 0x1c, DATA + 0x200);
                w(
                    c,
                    DATA + 0x200 + 4 * (part as u32),
                    if seed % 6 == 0 { 0xffff_ffff } else { 0 },
                );
                w(c, this + 0xc, DATA + 0x400);
                let e = DATA + 0x400 + 0x58;
                w(
                    c,
                    e + 0x10,
                    if seed % 2 == 0 {
                        0
                    } else {
                        0x0000_0000_0000_00ff
                    },
                );
                w(c, e + 0x18, 0);
                w(c, e + 0x1c, 0);
            },
        );
    }

        #[test]
    fn belibrary_dtor_matches() {
        check(ee::s01::f_001499a8__generated, belibrary_dtor, 8, &|c, seed| {
            let _ = base(c, seed, 0x1499a8, 2);
            let this = DATA;
            clear(c, this, 0x100);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0;
            for off in [
                -0x1f3ci32, -0x1f30, -0x1ee8, -0x1f2c, -0x1ee4, -0x1ee0, -0x1eec, -0x1ea8, -0x1f28,
                -0x1eb8, -0x1edc, -0x1ea4, -0x1e8c,
            ] {
                w(
                    c,
                    GAME_GP.wrapping_sub(off as u32),
                    if seed % 2 == 0 { 0 } else { 0 },
                );
            }
        });
    }

    #[test]
    fn tag_field_merge_matches() {
        check(ee::s14::f_003ecb00__generated, tag_field_merge, 16, &|c, seed| {
            let _ = base(c, seed, 0x3ecb00, 2);
            let rec = DATA;
            let data = DATA + 0x200;
            clear(c, rec, 0x200);
            clear(c, data, 0x200);
            c.pc = 0x003e_cb00;
            c.g[A0][0] = rec as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = data as u64;
            let text: &[u8] = match seed % 5 {
                0 => b"name=value other:2  \0",
                1 => b"\0",
                2 => b"   \0",
                3 => b"a=b\0",
                _ => b"key=1=2\0",
            };
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(data + i as u32, *b);
            }
        });
    }
    #[test]
    fn uilistbox_refresh_text_matches() {
        check(
            ee::s13::f_00399e28__generated,
            uilistbox_refresh_text,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x399e28, 3);
                let this = DATA;
                clear(c, this, 0x800);
                c.g[A0][0] = this as u64;
                let a = DATA + 0x200;
                let b = DATA + 0x300;
                let loc = DATA + 0x400;
                clear(c, a, 0x100);
                clear(c, b, 0x100);
                clear(c, loc, 0x100);
                w(c, this + 0x5c, a);
                w(c, a + 0xd0, b);
                w(c, b + 0x10, loc);
                w(c, loc + 0x10, DATA + 0x500);
                let get = vtable(c, &mut r, &[0x20]);
                w(c, DATA + 0x500 + 4, get);
                let count = if seed % 3 == 0 { 0 } else { 2 };
                w(c, this + 0x318, count);
                for i in 0..3u32 {
                    w(
                        c,
                        this + 0x98 + 0x14 * i,
                        if i < count { 0x100 + i * 0x10 } else { 0 },
                    );
                }
                let relayout = vtable(c, &mut r, &[0x28]);
                w(c, this + 8, relayout);
            },
        );
    }

    #[test]
    fn multisplinemodifier_v01_matches() {
        check(
            ee::s11::f_0035a250__generated,
            multisplinemodifier_v01,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x35a250, 2);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                w(c, this + 0x4, if seed % 2 == 0 { 0 } else { -1i32 as u32 });
                w(c, this + 0x38, 0);
                w(c, this + 0x3c, 0);
                w(c, this + 0x44, 0);
            },
        );
    }

    #[test]
    fn cfestaterequestline_update_song_names_matches() {
        check(
            ee::s02::f_00197bc8__generated,
            cfestaterequestline_update_song_names,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x197bc8, 3);
                let this = DATA;
                clear(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                w(c, this + 0x14c, 0);
                w(c, this + 0x150, 0);
                let vt = vtable(c, &mut r, &[0x48]);
                for i in 0..8u32 {
                    let text = DATA + 0x200 + 0x80 * i;
                    clear(c, text, 0x40);
                    w(c, text + 8, vt);
                    w(c, this + 0x174 + 4 * i, text);
                }
            },
        );
    }

    #[test]
    fn benewraceint_get_event_target_matches() {
        check(
            ee::s01::f_001454f8__generated,
            benewraceint_get_event_target,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x1454f8, 2);
                clear(c, DATA, 0x400);
                c.pc = 0x0014_54f8;
                c.g[A0][0] = DATA as u64;
                let mut kind = (seed % 3) as u64;
                let mode = if seed % 2 == 0 {
                    0
                } else {
                    6 + (seed % 6) as u32
                };
                c.g[A1][0] = kind;
                c.g[A2][0] = (seed % 2) as u64;
                w(c, 0x0053_5bc8 + 0x40, 0);
                w(c, 0x0053_5bc8 + 0x4a, mode);
                w(c, GAME_GP.wrapping_sub(0x1f34), 0);
                if kind == 0 {
                    kind = 1;
                }
                w(c, 0x0044_0b38, 0);
                c.bus.write16(0x0044_0b38 + 2, kind as u16);
                c.bus.write16(0x0044_0b38 + 0xe, 0x1234);
                for i in 1..30u32 {
                    w(c, 0x0044_0b38 + 0x10 * i, 0xffff_ffff);
                }
            },
        );
    }

    #[test]
    fn cfestaterewardgallery_open_buy_popup_matches() {
        check(
            ee::s04::f_001d0510__generated,
            cfestaterewardgallery_open_buy_popup,
            4,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d0510, 3);
                let this = DATA;
                clear(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[0x110, 0x128, 0x118]);
                w(c, this + 8, vt);
                w(c, this + 0x10, DATA + 0x200);
                w(c, this + 0x44, (seed % 4) as u32);
                w(c, this + 0x4c, 0);
                w(c, this + 0x54, 0);
            },
        );
    }
}
