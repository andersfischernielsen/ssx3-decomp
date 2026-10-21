//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small script, UI, world-cache, FX and front-end helpers,
//! each with a differential test against the generated body.
//!
//! The ports keep the guest-visible interface of the originals: every
//! register the generated body leaves set at return is reproduced, calls go
//! through the same function-table entries (`call_ptr`, exactly what the
//! generated cross-shard / indirect calls use), and loops that check events
//! deliver them at the same back edges as the generated code.

use crate::abi;
use crate::call::{call_ptr, guest, sp, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_73a0, script_list_on_seq_end, "ScriptList_OnSeqEnd"),
    (0x0039_d860, ui_screen_find_object, "UIScreen_FindObject"),
    (0x003a_7e38, world_cache_dtor, "cWorldCache_dtor"),
    (0x0034_37c0, counter_ctor_2, "Counter_ctor_2"),
    (0x0026_cbb8, playback_man_save_world_state, "PlaybackMan_SaveWorldState"),
    (0x0039_8738, ui_group_refresh_text, "cUIGroup_RefreshText"),
    (0x0039_8798, ui_group_handle_input, "cUIGroup_HandleInput"),
    (0x0034_57c8, particle_modifier_base_ctor_2, "ParticleModifier_BaseCtor_2"),
    (0x0034_90b0, float_rail_v01, "FloatRail_v01"),
    (0x0034_9eb8, spring_rail_v01, "SpringRail_v01"),
    (0x0035_8fd0, anim_combo_v36, "AnimCombo_v36"),
    (0x0036_7260, tex_man_alloc_btex_info, "TexMan_AllocBTexInfo"),
    (0x0034_cde8, fence_kick_from_neighbour, "Fence_KickFromNeighbour"),
    (0x0030_eb60, rider_model_hide_all_parts, "RiderModel_HideAllParts"),
    (0x0028_b528, snd_bank_unload, "SndBank_Unload"),
    (0x0034_fb90, world_obj_ctor_2, "WorldObj_ctor_2"),
    (0x0035_48c0, sync_process_ctor_2, "SyncProcess_ctor_2"),
    (0x001a_3fe0, fe_state_net_messenger_update, "cFEStateNetMessenger_Update"),
    (
        0x001a_4fb0,
        fe_state_net_messenger_on_find_user_result,
        "cFEStateNetMessenger_OnFindUserResult",
    ),
    (
        0x001a_6c40,
        fe_state_net_messenger_set_player_menu_selection,
        "cFEStateNetMessenger_SetPlayerMenuSelection",
    ),
    (0x001a_8770, net_fe_state_update, "NetFEState_Update"),
    (0x001a_9090, net_fe_state_open_keyboard_str, "NetFEState_OpenKeyboardStr"),
    (0x001a_9150, net_fe_state_open_keyboard_text_id, "NetFEState_OpenKeyboardTextId"),
    (0x001b_5268, fe_state_net_config_start_card_scan, "cFEStateNetConfig_StartCardScan"),
    (
        0x001b_8890,
        fe_state_net_account_mem_popup_id_to_state,
        "cFEStateNetAccountMem_PopupIdToState",
    ),
    (
        0x001b_96e0,
        fe_state_create_step1_set_email_text,
        "cFEStateCreateStep1_SetEmailText",
    ),
    (0x001e_2518, message_center_post_once_3c, "MessageCenter_PostOnce3C"),
    (0x001e_2578, message_center_post_once_38, "MessageCenter_PostOnce38"),
    (0x001e_3268, message_center_remove_message_by_id, "MessageCenter_RemoveMessageById"),
    (0x0035_f688, tex_flip_modifier_save, "TexFlipModifier_Save"),
    (0x0025_a850, lobby_man_release_snapshot_lists, "LobbyMan_ReleaseSnapshotLists"),
    (0x0025_aae0, lobby_man_send_chat, "LobbyMan_SendChat"),
    (0x0026_8048, chat_event_dtor, "ChatEvent_dtor"),
    (0x0026_81b0, net_event_e2_clone, "NetEventE2_Clone"),
    (0x0026_82a8, net_event102_lock_dtor, "NetEvent102Lock_dtor"),
    (0x0026_8460, net_event_e3_dtor, "NetEventE3_dtor"),
    (0x0027_b578, script_engine_find_bank_slot_by_id, "ScriptEngine_FindBankSlotById"),
    (0x002a_de88, sound_manager_ref_ctor, "SoundManagerRef_ctor"),
    (0x0027_3a00, in_seq_channel_release, "InSeq_ChannelRelease"),
    (0x002c_46c0, ps2_mem_card_v38, "cPS2MemCard_v38"),
    (0x002c_5278, ps2_mem_card_clear_dir_cache, "cPS2MemCard_ClearDirCache"),
    (0x0016_22b0, rider_camera_algorithm_ctor, "RiderCameraAlgorithm_ctor"),
    (0x003a_51d0, ui_group_v03, "cUIGroup_v03"),
    (
        0x001c_c0e0,
        c_keyboard_popup_find_enabled_in_column_from_bottom,
        "cKeyboardPopup_FindEnabledInColumnFromBottom",
    ),
    (0x001c_e408, c_keyboard_popup_set_digit_keys_disabled, "cKeyboardPopup_SetDigitKeysDisabled"),
    (
        0x001c_e468,
        c_keyboard_popup_set_function_keys_enabled,
        "cKeyboardPopup_SetFunctionKeysEnabled",
    ),
    (0x0030_b6f8, w_script_man_request_mission_restart, "WScriptMan_RequestMissionRestart"),
];

// ---------------------------------------------------------------------------
// Small register helpers.

/// `base + off` as a 32-bit address.
#[inline(always)]
fn p(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `lw a` through the CPU's load path.
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

// ---------------------------------------------------------------------------
// Script data manager.

/// `void ScriptList_OnSeqEnd(ScriptList* self, int a1, void* obj)` (0x2773a0):
/// if the list's current command is a movie entry that is not playing, hand
/// over to the next ready script; a paused script is resumed first.
pub fn script_list_on_seq_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, 0x0027_71c8, 0x0027_73c0)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 0xb8));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0028_26d8, 0x0027_73e0)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0027_76e0, 0x0027_73e8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// UI screens and groups.

/// `void* UIScreen_FindObject(UIScreen* self, uint hash)` (0x39d860): walk the
/// screen's object array and return the first live object whose hash matches.
pub fn ui_screen_find_object(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = lw(c, p(c.g[A0][0], 0x3c));
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = lw(c, p(c.g[A0][0], 0x38));
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
                let mut more;
                if c.g[V1][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = (c.g[A3][0] < c.g[A0][0]) as u64;
                    more = c.g[V0][0] != 0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p(c.g[V1][0], 0x38));
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

/// `void cUIGroup_RefreshText(cUIGroup* self)` (0x398738): call every element
/// of the group's UI list to re-read its text.
pub fn ui_group_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x0039_7840, 0x0039_874c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(p(c.g[V0][0], 0x80)).lh(c);
                c.g[V1][0] = lw(c, p(c.g[V0][0], 0x84));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_8770)?;
                c.g[S0][0] = lw(c, p(c.g[S0][0], 4));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039_7718, 0x0039_877c)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                    break;
                }
                c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8760);
                }
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cUIGroup_HandleInput(cUIGroup* self, void* arg)` (0x398798): unless the
/// group is a passive display, offer the input to every list node through a
/// small functor; true when a node claimed it.
pub fn ui_group_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = lw(c, p(c.g[A0][0], 0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 11;
            c.g[V0][0] = 0x0049_0000;
            Addr(p(c.g[SP][0], 4)).sw(c, c.g[A1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4bb8));
            c.g[S0][0] = 0x0049_0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4ce8));
            Addr(p(c.g[SP][0], 0)).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0039_79f8, 0x0039_87dc)?;
            Addr(p(c.g[SP][0], 0)).sw(c, c.g[S0][0]);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// World cache.

/// `void cWorldCache_dtor(cWorldCache* self, int flags)` (0x3a7e38): release
/// the cache's hulls/sections and both streamers, then delete it when owned.
pub fn world_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x003a_7e98, 0x003a_7e54)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x300));
        c.g[A1][0] = 2;
        call_ptr(c, 0x003a_6e20, 0x003a_7e60)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 2;
        call_ptr(c, 0x003a_8fb8, 0x003a_7e6c)?;
        c.g[S0][0] &= 1;
        let owns = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !owns {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x003a_7e80)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cWorldCache_FreeSectionTable(cWorldCache* self, int i)` (0x3a8230):
/// destroy section table `i` and clear its slot.
pub fn world_cache_free_section_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0xc));
        call_ptr(c, 0x003a_7b98, 0x003a_8250)?;
        c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = lw(c, c.g[V0][0] as u32);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_7d80, 0x003a_826c)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
        frame.restore_ra(c, 0);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Particles, FX and world objects.

/// `Counter* Counter_ctor_2(Counter* self, int a1, Stream* stream)` (0x3437c0):
/// construct the base object, install the debounce vtable and register the
/// debounce record with the stream.
pub fn counter_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_fb90, 0x0034_37dc)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3f0));
        c.g[A2][0] = 8;
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_3804)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void* PlaybackMan_SaveWorldState(PlaybackMan* self)` (0x26cbb8): save one
/// world-state snapshot once (the `+0x10` latch).
pub fn playback_man_save_world_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = lw(c, p(c.g[S1][0], 0x10));
        let saved = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x30);
        if !saved {
            c.cycles += 11;
            c.g[S0][0] = 1;
            Addr(p(c.g[SP][0], 2)).sb(c, 0);
            Addr(p(c.g[SP][0], 0)).sh(c, c.g[S0][0]);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            Addr(p(c.g[SP][0], 3)).sb(c, 0);
            c.g[A2][0] = c.g[SP][0];
            c.g[V0][0] = lw(c, p(c.g[S1][0], 0x5c));
            c.g[A0][0] = lw(c, p(c.g[V0][0], 0x28));
            call_ptr(c, 0x0026_d7d0, 0x0026_cbfc)?;
            Addr(p(c.g[S1][0], 0x10)).sw(c, c.g[S0][0]);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `ParticleModifier* ParticleModifier_BaseCtor_2(ParticleModifier* self,
/// void* obj)` (0x3457c8): link-list init, base class from `obj`, and the
/// global modifier counter.
pub fn particle_modifier_base_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p(c.g[S0][0], 8)).sw(c, c.g[V0][0]);
        Addr(p(c.g[S0][0], 4)).sw(c, 0);
        c.g[V1][0] = lw(c, c.g[A1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x18)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[V1][0], 0x1c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_5800)?;
        c.g[V1][0] = lw(c, p(c.g[GP][0], 0xf04));
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(p(c.g[GP][0], 0xf04)).sw(c, c.g[V1][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Rails.

/// `void FloatRail_v01(FloatRail* self, int flags)` (0x3490b0): detach the
/// rail from the spatial tree and run the base destructor. The `v01` in the
/// name marks this as the vector-deleting variant.
pub fn float_rail_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0034_8c48, 0x0034_90d8)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x70));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0034_90ec)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_7f90, 0x0034_90f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SpringRail_v01(SpringRail* self, int flags)` (0x349eb8): the same
/// teardown for a spring rail (its own vtable, base `FlexRail_dtor`).
pub fn spring_rail_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0034_8c48, 0x0034_9ee0)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x70));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0034_9ef4)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_7f90, 0x0034_9f00)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Animation objects.

/// `void AnimCombo_v36(AnimCombo* self, int msg)` (0x358fd0): on the "snapshot"
/// message copy the current pose into the blend source; otherwise pass the
/// message to the animation object it embeds.
pub fn anim_combo_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x82;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let msg_is_snapshot = c.g[A1][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !msg_is_snapshot {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x14)).lhu(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 8;
                c.lwc1(0, p(c.g[S0][0], 0x38));
                c.swc1(0, p(c.g[S0][0], 0x10));
                call_ptr(c, 0x0035_9030, 0x0035_9000)?;
                c.lwc1(0, p(c.g[S0][0], 8));
                c.g[V0][0] = 1;
                Addr(p(c.g[S0][0], 0x14)).sb(c, c.g[V0][0]);
                c.swc1(0, p(c.g[S0][0], 4));
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        } else {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0034_1fe8, 0x0035_901c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `AnimDelta* AnimDelta_ctor_2(AnimDelta* self, int a1, Stream* stream)`
/// (0x359270): base animation object, its vtable and the stream registration.
pub fn anim_delta_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_1c80, 0x0035_928c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1590));
        c.g[A2][0] = 4;
        Addr(p(c.g[S0][0], 0x3c)).sw(c, c.g[V1][0]);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_92b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Textures and VRAM.

/// `void TexMan_AllocBTexInfo(TexMan* self, int id)` (0x367260): allocate one
/// 0x58-byte BTexInfo for slot `id` and bump the manager's counter.
pub fn tex_man_alloc_btex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = 0x0049_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2078));
        c.g[A0][0] = 0x58;
        c.g[A2][0] = 0x2100_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0036_7298)?;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void VramHeap_FreeTextureBlocks(VramHeap* self, int i)` (0x366e30): free
/// both texture blocks a VRAM slot owns.
pub fn vram_heap_free_texture_blocks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x23e8));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S1][0] = lw(c, c.g[A1][0] as u32);
        c.g[A1][0] = lw(c, p(c.g[S1][0], 0x30));
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 3;
            call_ptr(c, 0x0036_63d8, 0x0036_6e68)?;
        }
        c.g[A1][0] = lw(c, p(c.g[S1][0], 0x34));
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0036_63d8, 0x0036_6e7c)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Fences and riders.

/// `void Fence_KickFromNeighbour(Fence* self, int on)` (0x34cde8): give the
/// fence an impulse and arm the kick timer for whichever side was hit.
pub fn fence_kick_from_neighbour(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0034_d6f0, 0x0034_ce04)?;
        if c.g[S0][0] == 0 {
            c.g[V0][0] = lw(c, p(c.g[S1][0], 0x3c));
            c.cycles += 2;
            let armed = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
            if !armed {
                c.cycles += 2;
                c.g[V0][0] = 4;
                Addr(p(c.g[S1][0], 0x3c)).sw(c, c.g[V0][0]);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[S1][0], 0x38));
            let armed = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
            if !armed {
                c.cycles += 3;
                c.g[V0][0] = 4;
                Addr(p(c.g[S1][0], 0x38)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderModel_HideAllParts(RiderModel* self)` (0x30eb60): clear the
/// `used` flag of every LOD of every part and the model's visibility mask.
pub fn rider_model_hide_all_parts(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = lw(c, p(c.g[A2][0], 8));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if !empty {
        c.cycles += 2;
        c.g[T1][0] = 0x58;
        c.mult(0, 7, 3, 9);
        'outer: loop {
            c.cycles += 2;
            c.g[A1][0] = 0;
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 8;
                c.g[V0][0] = lw(c, p(c.g[A2][0], 0xc));
                c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let more = c.g[V1][0] != 0;
                Addr(p(c.g[V0][0], 0x18)).sw(c, 0);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_eb80);
                    }
                    continue;
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, p(c.g[A2][0], 8));
                c.g[V1][0] = c.g[T0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.mult(0, 7, 3, 9);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_eb78);
                    }
                    continue 'outer;
                }
                break 'outer;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    Addr(p(c.g[A2][0], 0x150)).sd(c, 0);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Sound.

/// `void SndBank_Unload(SndBank* self)` (0x28b528): unload a sound bank once.
pub fn snd_bank_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, c.g[S0][0] as u32);
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 7;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A0][0] = lw(c, p(c.g[S0][0], 4));
            call_ptr(c, 0x003b_6300, 0x0028_b554)?;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
            let has_owner = c.g[V0][0] != 0;
            Addr(p(c.g[S0][0], 4)).sw(c, c.g[V1][0]);
            if !has_owner {
                c.cycles += 4;
                c.g[A0][0] = lw(c, p(c.g[S0][0], 0x10));
                call_ptr(c, 0x0025_23a8, 0x0028_b56c)?;
                Addr(p(c.g[S0][0], 0x14)).sw(c, 0);
                Addr(p(c.g[S0][0], 0x10)).sw(c, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Process nodes.

/// `WorldObj* WorldObj_ctor_2(WorldObj* self, int a1, Stream* stream)`
/// (0x34fb90): construct the process, install the world-object vtable and ask
/// the stream for the object's id.
pub fn world_obj_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_48c0, 0x0034_fbac)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c80));
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V1][0]);
        c.g[A2][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[A2][0], 0x20)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[A2][0], 0x24));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_fbd0)?;
        Addr(p(c.g[S0][0], 0x18)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SyncProcess* SyncProcess_ctor_2(SyncProcess* self, int a, Stream* stream)`
/// (0x3548c0): typed-process base, its vtable and the stream registration.
pub fn sync_process_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_4788, 0x0035_48dc)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1e00));
        c.g[A2][0] = 4;
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_4904)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Front-end state: net messenger.

/// `void cFEStateNetMessenger_Update(self)` (0x1a3fe0): refresh the player
/// list when a second has passed since the last refresh, then poll the net
/// state.
pub fn fe_state_net_messenger_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = lw(c, p(c.g[GP][0], 0x2a74));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x6dc));
        c.g[V1][0] = lw(c, p(c.g[V1][0], 0x18));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 0x3bu64) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            Addr(p(c.g[S0][0], 0x6dc)).sw(c, c.g[V1][0]);
            call_ptr(c, 0x001a_6978, 0x001a_4028)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_8770, 0x001a_4030)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x001a_8770, 0x001a_4018)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateNetMessenger_OnFindUserResult(self, a1, a2)` (0x1a4fb0):
/// show the find-user error for the "user not found" reply, otherwise hand
/// the event to the base state.
pub fn fe_state_net_messenger_on_find_user_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(c.g[A3][0], 0x1c));
        c.g[V0][0] &= 6;
        let no_flags = c.g[V0][0] == 0;
        c.g[A0][0] = 0x807;
        if !no_flags {
            c.cycles += 4;
            c.g[V0][0] = lw(c, p(c.g[A3][0], 0x6a0));
            c.g[V1][0] = lw(c, p(c.g[V0][0], 0x18));
            let not_found = c.g[V1][0] == c.g[A0][0];
            c.g[A0][0] = c.g[A3][0];
            if not_found {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                call_ptr(c, 0x001a_4e28, 0x001a_4fe4)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                c.pc = c.g[RA][0] as u32;
                return Ok(());
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[A3][0];
            call_ptr(c, 0x001a_8e40, 0x001a_4ff4)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
            c.pc = c.g[RA][0] as u32;
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[A3][0];
        call_ptr(c, 0x001a_8e40, 0x001a_5004)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = c.g[RA][0] as u32;
        Ok(())
    });
}

/// `void cFEStateNetMessenger_SetPlayerMenuSelection(self, int idx)`
/// (0x1a6c40): select entry `idx` in the menu named by the messenger's hash.
pub fn fe_state_net_messenger_set_player_menu_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(c.g[S0][0], 0x40));
        let no_screen = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_screen {
            c.cycles += 2;
            let negative = (c.g[S1][0] as i64) < 0;
            c.g[A0][0] = 0x0046_0000;
            if !negative {
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1590));
                call_ptr(c, 0x0031_7670, 0x001a_6c70)?;
                c.g[A0][0] = lw(c, p(c.g[S0][0], 0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001a_6c7c)?;
                let no_menu = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[V0][0];
                if !no_menu {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0] & 0xff;
                    call_ptr(c, 0x0039_b960, 0x001a_6c8c)?;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void NetFEState_Update(NetFEState* self)` (0x1a8770): two virtual hooks,
/// then the shared UI update and the net event pump.
pub fn net_fe_state_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
        c.g[A0][0] = Addr(p(c.g[V0][0], 0xd8)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0xdc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001a_8794)?;
        c.g[V0][0] = lw(c, p(c.g[S0][0], 8));
        c.g[A0][0] = Addr(p(c.g[V0][0], 0xe0)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0xe4));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001a_87a8)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x001a_87b0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x114;
        c.g[A2][0] = 0;
        call_ptr(c, 0x001a_8b88, 0x001a_87c0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void NetFEState_OpenKeyboardStr(self, a1, str, a3, a4, a5)` (0x1a9090):
/// create the popup, then set its static text and its initial text.
pub fn net_fe_state_open_keyboard_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[T0][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[T2][0];
        c.g[A3][0] = c.g[T1][0];
        call_ptr(c, 0x001a_91b0, 0x001a_90c0)?;
        c.g[A0][0] = lw(c, p(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_cf00, 0x001a_90cc)?;
        c.g[A0][0] = lw(c, p(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001c_b418, 0x001a_90d8)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void NetFEState_OpenKeyboardTextId(self, a1, id, a3, a4, a5)` (0x1a9150):
/// the same popup setup with a static text id instead of a string.
pub fn net_fe_state_open_keyboard_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[T0][0];
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = c.g[T2][0];
        c.g[A3][0] = c.g[T1][0];
        call_ptr(c, 0x001a_91b0, 0x001a_9180)?;
        c.g[A0][0] = lw(c, p(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001c_d020, 0x001a_918c)?;
        c.g[A0][0] = lw(c, p(c.g[S1][0], 0x69c));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001c_b418, 0x001a_9198)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateNetConfig_StartCardScan(self)` (0x1b5268): reset the card
/// status widgets and open the "checking card" popup.
pub fn fe_state_net_config_start_card_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x6f4));
        call_ptr(c, 0x0039_b960, 0x001b_528c)?;
        Addr(p(c.g[GP][0], -0x1570i32 as u32)).sw(c, 0);
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x6e4));
        Addr(p(c.g[S0][0], 0x70c)).sw(c, c.g[S1][0]);
        call_ptr(c, 0x0039_a670, 0x001b_529c)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x6e8));
        call_ptr(c, 0x0039_a670, 0x001b_52a4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x001b_47c0, 0x001b_52b0)?;
        Addr(p(c.g[S0][0], 0x708)).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cFEStateNetAccountMem_PopupIdToState(int id)` (0x1b8890): map an
/// account-memory popup id to the front-end state that owns it.
pub fn fe_state_net_account_mem_popup_id_to_state(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xd;
    let thirteen = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < 14i64) as u64;
    if thirteen {
        c.cycles += 2;
        c.g[V0][0] = 0xf;
        abi::ret(c);
        return;
    }
    let below14 = c.g[V0][0] != 0;
    c.cycles += 2;
    c.g[V0][0] = 7;
    if below14 {
        c.cycles += 2;
        let seven = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1c;
        if seven {
            c.cycles += 2;
            c.g[V0][0] = 9;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x15;
    let twentyone = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = 0x19;
    if twentyone {
        c.cycles += 2;
        c.g[V0][0] = 0x17;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x1c;
    c.cycles += 2;
    abi::ret(c);
}

/// `void cFEStateCreateStep1_SetEmailText(self)` (0x1b96e0): find the email
/// widget and push the current email string into it.
pub fn fe_state_create_step1_set_email_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4838));
        call_ptr(c, 0x0031_7670, 0x001b_96fc)?;
        c.g[A0][0] = lw(c, p(c.g[S0][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001b_9708)?;
        c.g[A1][0] = c.g[V0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[S0][0], 0x6d0));
            c.g[A2][0] = lw(c, p(c.g[V0][0], 0xc));
            c.g[V1][0] = lw(c, (c.g[A2][0] as u32).wrapping_sub(8));
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001b_9950, 0x001b_9730)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Message center.

/// `void MessageCenter_PostOnce3C(void)` (0x1e2518): post the "friends" hint
/// once, when the buddy list is quiet.
pub fn message_center_post_once_3c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, p(c.g[GP][0], -0x848i32 as u32));
        frame.save_ra(c, 0);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x84));
        c.g[A0][0] = lw(c, p(c.g[V1][0], 0x28));
        c.g[A0][0] = lw(c, c.g[A0][0] as u32);
        let none = c.g[A0][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
        }
        c.cycles += 2;
        let busy = c.g[A1][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            c.cycles += 4;
            c.g[A0][0] = 0x3c;
            call_ptr(c, 0x001e_3a30, 0x001e_254c)?;
            let flagged = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !flagged {
                c.cycles += 7;
                c.g[A0][0] = 0x3c;
                c.g[A1][0] = 1;
                call_ptr(c, 0x001e_3a78, 0x001e_2560)?;
                c.g[A0][0] = 0xff;
                c.g[A1][0] = 0;
                call_ptr(c, 0x001e_2fe0, 0x001e_256c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void MessageCenter_PostOnce38(void)` (0x1e2578): post the "lodge" hint
/// once, when the buddy list is quiet.
pub fn message_center_post_once_38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, p(c.g[GP][0], -0x848i32 as u32));
        frame.save_ra(c, 0);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x84));
        c.g[A0][0] = lw(c, p(c.g[V1][0], 0x28));
        c.g[A0][0] = lw(c, c.g[A0][0] as u32);
        let none = c.g[A0][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
        }
        c.cycles += 2;
        let busy = c.g[A1][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            c.cycles += 4;
            c.g[A0][0] = 0x38;
            call_ptr(c, 0x001e_3a30, 0x001e_25ac)?;
            let flagged = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !flagged {
                c.cycles += 7;
                c.g[A0][0] = 0x38;
                c.g[A1][0] = 1;
                call_ptr(c, 0x001e_3a78, 0x001e_25c0)?;
                c.g[A0][0] = 0xfb;
                c.g[A1][0] = 0;
                call_ptr(c, 0x001e_2fe0, 0x001e_25cc)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void MessageCenter_RemoveMessageById(int id)` (0x1e3268): find the inbox
/// entry with `id` and remove it. A `match pc` dispatcher: the loop head
/// 0x1e3290 is a resume entry.
pub fn message_center_remove_message_by_id(c: &mut Cpu) {
    guest(c, |c| {
        let frame;
        let mut increment;
        if c.pc == 0x001e_3268 {
            c.cycles += 9;
            frame = Frame::open(c, 0x20);
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            c.g[RA][0] = sx(0x001e_327c);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x001e_39f8, 0x001e_327c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = 0;
            c.g[V1][0] = c.g[A1][0];
            increment = false;
        } else if c.pc == 0x001e_3290 {
            frame = Frame { size: 0x20 };
            increment = true;
        } else {
            let pc = c.pc;
            c.jump_out(pc, 0x001e_3268, 0x001e_32c8);
            return Ok(());
        }
        loop {
            if increment {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            }
            increment = true;
            c.cycles += 4;
            c.g[V0][0] = Addr(p(c.g[A1][0], 0xcc)).lbu(c);
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[V1][0] as u32);
            let differs = c.g[S0][0] != c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
            if differs {
                if back_edge_unwinds(c, 0x001e_3290) {
                    return Ok(());
                }
                continue;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001e_32b8);
            call_ptr(c, 0x001e_31b8, 0x001e_32b8)?;
            frame.restore(c, S0, 0x10);
            break;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Texture flip modifier.

/// `void TexFlipModifier_Save(TexFlipModifier* self, Stream* stream)`
/// (0x35f688): write the modifier's 0x18-byte record and its texture handle.
pub fn tex_flip_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x18;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = lw(c, c.g[S0][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V1][0], 8)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[V1][0], 0xc));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f6bc)?;
        c.g[V1][0] = lw(c, c.g[S0][0] as u32);
        c.g[A1][0] = lw(c, p(c.g[S1][0], 0x18));
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x28)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[V1][0], 0x2c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_f6d4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Lobby.

/// `void LobbyMan_ReleaseSnapshotLists(LobbyMan* self)` (0x25a850): destroy
/// all 29 snapshot lists hanging off `self+0x1ec`.
pub fn lobby_man_release_snapshot_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ec));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1c;
        loop {
            c.cycles += 5;
            c.g[V0][0] = lw(c, c.g[S0][0] as u32);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[A1][0] = 4;
            let empty = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[V0][0];
            if !empty {
                c.cycles += 3;
                c.g[A0][0] = lw(c, p(c.g[S1][0], 0x54));
                call_ptr(c, 0x003e_8688, 0x0025_a88c)?;
                Addr(c.g[S0][0] as u32).sw(c, 0);
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0025_a870);
            }
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

/// `void LobbyMan_SendChat(LobbyMan* self, void* value)` (0x25aae0): format the
/// chat field into a 0x200-byte buffer and send it on the lobby socket.
pub fn lobby_man_send_chat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x220);
        frame.save(c, S0, 0x210);
        frame.save_ra(c, 0x200);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, c.g[S0][0] as u32);
        let inactive = c.g[V0][0] == 0;
        c.g[A3][0] = c.g[A1][0];
        if !inactive {
            c.cycles += 13;
            c.g[A2][0] = 0x004a_0000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3010));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x200;
            Addr(c.g[SP][0] as u32).sb(c, 0);
            call_ptr(c, 0x0025_9288, 0x0025_ab14)?;
            c.g[A1][0] = 0x6d65_0000;
            c.g[A0][0] = lw(c, p(c.g[S0][0], 0x54));
            c.g[A1][0] |= 0x7367;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x003e_8b10, 0x0025_ab30)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Network events.

/// `void ChatEvent_dtor(ChatEvent* self, int flags)` (0x268048): destroy both
/// strings and delete the event when owned.
pub fn chat_event_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_806c)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8078)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[S0][0] &= 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        let owns = c.g[S0][0] == 0;
        Addr(p(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        if !owns {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_8094)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void* NetEventE2_Clone(NetEventE2* self)` (0x2681b0): allocate a 0xc-byte
/// copy, set its type/vtable and copy the string.
pub fn net_event_e2_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0xc;
        call_ptr(c, 0x0031_7d70, 0x0026_81d8)?;
        c.g[A1][0] = lw(c, p(c.g[S0][0], 8));
        c.g[V1][0] = 0x0048_0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1210));
        c.g[V0][0] = 0xe2;
        Addr(p(c.g[S0][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_81fc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void NetEvent102Lock_dtor(NetEvent102Lock* self, int flags)` (0x2682a8):
/// destroy both strings and delete the lock event when owned.
pub fn net_event102_lock_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_82cc)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_82d8)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[S0][0] &= 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        let owns = c.g[S0][0] == 0;
        Addr(p(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        if !owns {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_82f4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void NetEventE3_dtor(NetEventE3* self, int flags)` (0x268460): destroy
/// both strings and delete the event when owned.
pub fn net_event_e3_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8484)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8490)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[S0][0] &= 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        let owns = c.g[S0][0] == 0;
        Addr(p(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        if !owns {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_84ac)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int ScriptEngine_FindBankSlotById(BankSlot* slots, int key, int start)`
/// (0x27b578): scan the 10 bank slots from `start`, return the first free slot
/// whose key matches, or -1.
pub fn script_engine_find_bank_slot_by_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < 10i64) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
    if !out_of_range {
        c.cycles += 3;
        c.g[A3][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62c));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[A0][0], 4));
            let taken = c.g[V0][0] != 0;
            c.g[V1][0] = 0;
            if !taken {
                c.cycles += 3;
                c.g[V0][0] = lw(c, (c.g[A0][0] as u32).wrapping_sub(4));
                c.g[V1][0] = c.g[A3][0];
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = 0;
                }
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, c.g[A0][0] as u32);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A2][0];
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_b590);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    abi::ret(c);
}

/// `SoundManagerRef* SoundManagerRef_ctor(SoundManagerRef* self, void* a1)`
/// (0x2ade88): install the reference vtable, create the sound manager and
/// pass the argument on.
pub fn sound_manager_ref_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c00));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
        Addr(p(c.g[GP][0], 0x68c)).sw(c, c.g[S0][0]);
        call_ptr(c, 0x002a_8fa0, 0x002a_deb4)?;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V1][0], 8)).lh(c);
        c.g[A2][0] = lw(c, p(c.g[V1][0], 0xc));
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002a_ded0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InSeq_ChannelRelease(InSeqChannel* self)` (0x273a00): stop and clear
/// the channel when one is bound.
pub fn in_seq_channel_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[S0][0], 4));
        if c.g[V0][0] != 0 {
            c.cycles += 14;
            call_ptr(c, 0x0027_3dc0, 0x0027_3a24)?;
            Addr(p(c.g[S0][0], 0x38)).sw(c, 0);
            Addr(p(c.g[S0][0], 4)).sw(c, 0);
            Addr(p(c.g[S0][0], 8)).sw(c, 0);
            Addr(p(c.g[S0][0], 0xc)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x10)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x14)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x18)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x1c)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x20)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x30)).sw(c, 0);
            Addr(p(c.g[S0][0], 0x34)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Memory card.

/// `void cPS2MemCard_v38(self, a1, a2, a3)` (0x2c46c0): build the save name
/// and call the virtual save slot.
pub fn ps2_mem_card_v38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A3][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x002c_54d0, 0x002c_46e8)?;
        c.g[V1][0] = lw(c, c.g[S0][0] as u32);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x120)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[V1][0], 0x124));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_4708)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// `void cPS2MemCard_ClearDirCache(cPS2MemCard* self)` (0x2c5278): wipe the
/// directory cache buffers and the current directory entry.
pub fn ps2_mem_card_clear_dir_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x40;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(c.g[S0][0], 0x50)).sw(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x480));
        Addr(p(c.g[S0][0], 0x54)).sw(c, 0);
        call_ptr(c, 0x003e_6448, 0x002c_52a0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a4));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x2d0;
        call_ptr(c, 0x003e_6448, 0x002c_52b0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c0));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x1000;
        call_ptr(c, 0x003e_6448, 0x002c_52c0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002c_52d8, 0x002c_52c8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Camera.

/// `RiderCameraAlgorithm* RiderCameraAlgorithm_ctor(RiderCameraAlgorithm* self)`
/// (0x1622b0): camera base ctor, then the algorithm's constants and vtable.
pub fn rider_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4220));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0], 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0016_7e30, 0x0016_22d0)?;
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 1);
        c.g[V1][0] = 0x0046_0000;
        c.lwc1(2, p(c.g[GP][0], -0x6b0c_i32 as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x43c8));
        c.g[A0][0] = 0x53;
        c.swc1(0, c.g[S0][0] as u32);
        c.swc1(1, p(c.g[S0][0], 4));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(2, p(c.g[S0][0], 8));
        Addr(p(c.g[S0][0], 0x10)).sw(c, c.g[V1][0]);
        Addr(p(c.g[S0][0], 0xc)).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
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
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x10)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[V1][0], 0x14));
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

/// `int cKeyboardPopup_FindEnabledInColumnFromBottom(cKeyboardPopup* self,
/// int start)` (0x1cc0e0): walk up a key column until an enabled key is
/// found; remember it at `self+0x140`. Dispatcher: 0x1cc110 is a resume entry.
pub fn c_keyboard_popup_find_enabled_in_column_from_bottom(c: &mut Cpu) {
    guest(c, |c| {
        let frame;
        let mut stage;
        if c.pc == 0x001c_c0e0 {
            c.cycles += 7;
            frame = Frame::open(c, 0x30);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            frame.save(c, S1, 0x10);
            frame.save(c, S0, 0x20);
            c.g[S1][0] = c.g[A0][0];
            let have_start = c.g[A1][0] != c.g[V0][0];
            frame.save_ra(c, 0);
            if have_start {
                c.cycles += 2;
                c.g[S0][0] = c.g[A1][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[S1][0], 0x140));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38));
            }
            stage = 0u32;
        } else if c.pc == 0x001c_c110 {
            frame = Frame { size: 0x30 };
            stage = 1;
        } else {
            let pc = c.pc;
            c.jump_out(pc, 0x001c_c0e0, 0x001c_c140);
            return Ok(());
        }
        loop {
            if stage == 1 {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe));
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x001c_e560, 0x001c_c124)?;
            let disabled = c.g[V0][0] == 0;
            frame.restore_ra(c, 0);
            if !disabled {
                break;
            }
            if back_edge_unwinds(c, 0x001c_c110) {
                return Ok(());
            }
            stage = 1;
        }
        c.cycles += 5;
        Addr(p(c.g[S1][0], 0x140)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cKeyboardPopup_SetDigitKeysDisabled(cKeyboardPopup* self, int on)`
/// (0x1ce408): enable/disable the ten digit keys (indices 0xf..0x18).
pub fn c_keyboard_popup_set_digit_keys_disabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0xf;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0] ^ 1;
            c.g[A3][0] = 1;
            call_ptr(c, 0x001c_e3c8, 0x001c_e440)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 25i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_e430);
            }
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

/// `void cKeyboardPopup_SetFunctionKeysEnabled(cKeyboardPopup* self, int on)`
/// (0x1ce468): enable/disable the fourteen function keys (indices 0..0xd).
pub fn c_keyboard_popup_set_function_keys_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = 2;
            call_ptr(c, 0x001c_e3c8, 0x001c_e4a0)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 14i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_e490);
            }
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

/// `void WScriptMan_RequestMissionRestart(WScriptMan* self)` (0x30b6f8):
/// reset the world/replay managers and arm the mission-restart request.
pub fn w_script_man_request_mission_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = lw(c, p(c.g[GP][0], -0x848i32 as u32));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[V1][0], 0x84));
        c.g[A0][0] = lw(c, p(c.g[V0][0], 0x34));
        call_ptr(c, 0x0026_cdf8, 0x0030_b718)?;
        c.g[V0][0] = lw(c, p(c.g[GP][0], -0x848i32 as u32));
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x84));
        c.g[A0][0] = lw(c, p(c.g[V1][0], 0x28));
        call_ptr(c, 0x0026_f228, 0x0030_b728)?;
        c.g[V0][0] = lw(c, p(c.g[GP][0], -0x848i32 as u32));
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x84));
        c.g[A0][0] = lw(c, p(c.g[V1][0], 0x28));
        call_ptr(c, 0x0026_f7b8, 0x0030_b738)?;
        c.g[V1][0] = lw(c, p(c.g[S0][0], 0x2a0));
        c.g[V0][0] = 2;
        Addr(p(c.g[S0][0], 0x2ac)).sw(c, c.g[V0][0]);
        Addr(p(c.g[S0][0], 0x2b0)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    const SEEDS: u32 = 60;

    /// A `gp`-relative global address.
    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// Zero `words` words at `at`.
    fn clear(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }

    /// Store a 32-bit word at `at`.
    fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    /// A vtable at `at` whose slots `(offset, target)` hold `{delta 0, target}`.
    fn vtable(c: &mut Cpu, at: u32, slots: &[(u32, u32)]) {
        clear(c, at, 0x40);
        for &(off, target) in slots {
            c.bus.write16(at + off, 0);
            c.bus.write32(at + off + 4, target);
        }
    }

    /// An inbox with `count` entries whose ids come from `ids`.
    fn inbox(c: &mut Cpu, at: u32, count: u8, ids: &[u32]) {
        clear(c, at, 0x40);
        c.bus.write8(at + 0xcc, count);
        for (i, &id) in ids.iter().enumerate() {
            w(c, at + 8 * i as u32, id);
        }
    }

    /// [`machine`] plus the BELibrary dummy at `gp-0x1f04` (without it
    /// `BELibrary_GetInstance` builds a library and waits on its rewards
    /// file), zeroed once per setup, and a working allocator.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        let lib = DATA + 0x7000;
        clear(c, lib, 0x100);
        w(c, gp_at(-0x1f04), lib as u32);
        // A small world whose race manager's spatial-hash root cell carries
        // level 11 (what `SpatialKey_FromAABB` produces for a zero box).
        // `SpatialHash_Remove` descends into a cell's children until the
        // levels match; with a null cell it recurses on null children forever.
        let world = DATA + 0x8000;
        let man = DATA + 0x8100;
        let cell = DATA + 0x8200;
        clear(c, world, 0x40);
        clear(c, man, 0x40);
        clear(c, cell, 0x40);
        w(c, cell, 11);
        w(c, gp_at(-0x848), world as u32);
        w(c, world + 0x84, man as u32);
        w(c, man + 0x20, cell as u32);
        // Allocations up to 0x200 bytes come from the size-class pools.
        // Without the `gp+0xdd8` flag, MEM_PoolAlloc <-> MEM_AllocInternal
        // recurse forever. (Tests that need larger blocks also call
        // `heap_init` for the dlmalloc arena.)
        pool_allocator(c);
        r
    }

    /// The `cBXString` static-empty-string marker (the value the string dtor
    /// compares against before touching the heap).
    const EMPTY_STRING: u32 = 0x0048_dcd4;

    /// The main heap's struct and a 1 MiB arena (clear of the test data and
    /// the fake-call log at 0x900000).
    const MEM_HEAP: u32 = 0x004f_f250;
    const ARENA: u32 = 0x0010_0000;
    const ARENA_SIZE: u32 = 0x0010_0000;

    /// Runs a generated function to completion as a setup step.
    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, crate::difftest::RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The engine's size-class pools (`MEM_PoolAlloc`): free lists hung off
    /// `gp + 0x2a60..`, live only when the "pools ready" flag at `gp+0xdd8`
    /// is set. Each list is 16 blocks, enough for the allocations these
    /// callees make.
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
        // Pool structs stay above the fake-call result pointers (which end
        // at DATA+0x3fff0) and the nodes below the fake-call log.
        let mut nodes = DATA + 0x44_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x40_000 + 0x800 * i as u32;
            let base_node = nodes;
            for n in 0..16u32 {
                let node = base_node + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base_node);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// A fresh dlmalloc arena for the main heap.
    fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, ARENA as u64, ARENA_SIZE as u64],
        );
    }

    /// One zeroed block from the dlmalloc arena (valid for `__builtin_delete`).
    fn heap_alloc(c: &mut Cpu, size: u32) -> u32 {
        let p = run(
            c,
            0x0032_0058,
            ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32;
        for i in (0..size).step_by(4) {
            c.bus.write32(p.wrapping_add(i), 0);
        }
        p
    }

    // ------------------------------------------------------------------
    // Script.

    #[test]
    fn script_list_on_seq_end_matches() {
        check(ee::s07::f_002773a0__generated, script_list_on_seq_end, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2773_a0);
            let list = DATA + 0x100;
            clear(c, list, 0x100);
            w(c, list + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
            w(c, list + 0xb8, if seed % 4 < 2 { 0 } else { 1 });
            w(c, gp_at(-0x84c), 0);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA);
        });
    }

    #[test]
    fn find_bank_slot_matches() {
        check(ee::s07::f_0027b578__generated, script_engine_find_bank_slot_by_id, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x27b5_78);
            let base = DATA + 0x100;
            clear(c, base, 0x300);
            let table = base + 0x62c;
            let key = r.next_u32();
            let target = if seed % 7 == 0 { -1i32 } else { (seed % 10) as i32 };
            for i in 0..10u32 {
                let s = table + 0x10 * i;
                w(c, s + 4, if (seed + i) % 3 == 0 { 1 } else { 0 });
                w(c, s, if i as i32 == target { key } else { r.next_u32() });
            }
            c.g[A0][0] = sx(base);
            c.g[A1][0] = key as u64;
            c.g[A2][0] = if seed % 5 == 0 { 0 } else { (seed % 12) as u64 };
        });
    }

    // ------------------------------------------------------------------
    // UI.

    #[test]
    fn ui_screen_find_object_matches() {
        check(ee::s13::f_0039d860__generated, ui_screen_find_object, SEEDS, &|c, seed| {
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
                        w(c, o + 0x38, if i == n - 1 && seed % 3 == 0 { hash } else { r.next_u32() });
                    } else {
                        w(c, objs + 4 * i, 0);
                    }
                }
            }
            c.g[A0][0] = sx(screen);
            c.g[A1][0] = hash as u64;
        });
    }

    #[test]
    fn ui_group_refresh_text_matches() {
        check(ee::s13::f_00398738__generated, ui_group_refresh_text, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x3987_38);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            let list = s + 0x74;
            let sent = DATA + 0x500;
            w(c, sent, sent as u32);
            w(c, sent + 4, sent as u32);
            if seed % 3 == 0 {
                w(c, list + 4, sent as u32);
            } else {
                let n = DATA + 0x540;
                clear(c, n, 0x20);
                w(c, n, list as u32);
                w(c, n + 4, sent as u32);
                let vt = DATA + 0x580;
                vtable(c, vt, &[(0x80, FAKE + 0x20)]);
                w(c, n + 8, vt as u32);
                w(c, list + 4, n as u32);
            }
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn ui_group_handle_input_matches() {
        check(ee::s13::f_00398798__generated, ui_group_handle_input, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3987_98);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, s + 0x14, if seed % 3 == 0 { 0x20 } else { 0 });
            let list = s + 0x74;
            let sent = DATA + 0x500;
            w(c, sent, sent as u32);
            w(c, sent + 4, sent as u32);
            if seed % 2 == 0 {
                w(c, list + 4, sent as u32);
            } else {
                let n = DATA + 0x540;
                clear(c, n, 0x20);
                w(c, n, list as u32);
                w(c, n + 4, sent as u32);
                let vt = DATA + 0x580;
                vtable(c, vt, &[(0x10, FAKE + 0x30)]);
                w(c, n + 8, vt as u32);
                w(c, list + 4, n as u32);
            }
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn ui_group_v03_matches() {
        check(ee::s13::f_003a51d0__generated, ui_group_v03, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3a51_d0);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            let vt = DATA + 0x580;
            vtable(c, vt, &[(0x10, FAKE + 0x40)]);
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
        });
    }

    // ------------------------------------------------------------------
    // World cache.

    #[test]
    fn world_cache_dtor_matches() {
        check(ee::s13::f_003a7e38__generated, world_cache_dtor, 24, &|c, seed| {
            let _ = base(c, seed, 0x3a7e_38);
            let s = DATA + 0x100;
            clear(c, s, 0x1000);
            w(c, s + 0x3e8, if seed % 5 == 0 { 0 } else { DATA + 0x4000 });
            clear(c, DATA + 0x4000, 0x40);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    

    // ------------------------------------------------------------------
    // World objects, FX, rails.

    #[test]
    fn counter_ctor_2_matches() {
        check(ee::s11::f_003437c0__generated, counter_ctor_2, 40, &|c, seed| {
            let mut r = base(c, seed, 0x3437_c0);
            let s = DATA + 0x100;
            clear(c, s, 0x100);
            let stream = DATA + 0x300;
            clear(c, stream, 0x20);
            let vt = DATA + 0x400;
            vtable(
                c,
                vt,
                &[(0x10, FAKE + 0x10), (0x14, FAKE + 0x20), (0x20, FAKE + 0x30), (0x24, FAKE + 0x40)],
            );
            w(c, stream, vt as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(stream);
        });
    }

    #[test]
    fn world_obj_ctor_2_matches() {
        check(ee::s11::f_0034fb90__generated, world_obj_ctor_2, 40, &|c, seed| {
            let mut r = base(c, seed, 0x34fb_90);
            let s = DATA + 0x100;
            clear(c, s, 0x100);
            let stream = DATA + 0x300;
            clear(c, stream, 0x20);
            let vt = DATA + 0x400;
            vtable(
                c,
                vt,
                &[(0x10, FAKE + 0x10), (0x14, FAKE + 0x20), (0x20, FAKE + 0x30), (0x24, FAKE + 0x40)],
            );
            w(c, stream, vt as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(stream);
        });
    }

    #[test]
    fn sync_process_ctor_2_matches() {
        check(ee::s11::f_003548c0__generated, sync_process_ctor_2, 40, &|c, seed| {
            let mut r = base(c, seed, 0x3548_c0);
            let s = DATA + 0x100;
            clear(c, s, 0x100);
            let stream = DATA + 0x300;
            clear(c, stream, 0x20);
            let vt = DATA + 0x400;
            vtable(c, vt, &[(0x10, FAKE + 0x10), (0x14, FAKE + 0x20)]);
            w(c, stream, vt as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(stream);
        });
    }

    

    #[test]
    fn particle_modifier_base_ctor_2_matches() {
        check(ee::s11::f_003457c8__generated, particle_modifier_base_ctor_2, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3457_c8);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            let obj = DATA + 0x300;
            clear(c, obj, 0x20);
            let vt = DATA + 0x400;
            vtable(c, vt, &[(0x18, FAKE + 0x10), (0x1c, FAKE + 0x20)]);
            w(c, obj, vt as u32);
            w(c, gp_at(0xf04), r.next_u32());
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(obj);
        });
    }

    #[test]
    fn float_rail_v01_matches() {
        check(ee::s11::f_003490b0__generated, float_rail_v01, 24, &|c, seed| {
            let _ = base(c, seed, 0x3490_b0);
            let s = DATA + 0x100;
            clear(c, s, 0x400);
            // `FlexRail_UnregisterSpaceNode` -> `World_GetNash()` and
            // `SpatialHash_Remove`: if the cell's level word does not match
            // the key's (11, from `SpatialKey_FromAABB`), the latter descends
            // into the (null) child forever. Give it a real cell whose level
            // matches, so the non-recursive path is taken.
            let world = DATA + 0x1000;
            let man = DATA + 0x2000;
            let cell = DATA + 0x3000;
            clear(c, world, 0x40);
            clear(c, man, 0x40);
            clear(c, cell, 0x40);
            w(c, cell, 11);
            w(c, gp_at(-0x848), world as u32);
            w(c, world + 0x84, man as u32);
            w(c, man + 0x20, cell as u32);
            // No node and no owned memory: no allocator at all on this path.
            w(c, s + 0x54, 0);
            w(c, s + 0x70, 0);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = 0;
        });
    }

    #[test]
    fn spring_rail_v01_matches() {
        check(ee::s11::f_00349eb8__generated, spring_rail_v01, 24, &|c, seed| {
            let _ = base(c, seed, 0x349e_b8);
            let s = DATA + 0x100;
            clear(c, s, 0x400);
            // See `float_rail_v01_matches`: match the spatial hash's level.
            let world = DATA + 0x1000;
            let man = DATA + 0x2000;
            let cell = DATA + 0x3000;
            clear(c, world, 0x40);
            clear(c, man, 0x40);
            clear(c, cell, 0x40);
            w(c, cell, 11);
            w(c, gp_at(-0x848), world as u32);
            w(c, world + 0x84, man as u32);
            w(c, man + 0x20, cell as u32);
            w(c, s + 0x54, 0);
            w(c, s + 0x70, 0);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = 0;
        });
    }

    #[test]
    fn anim_combo_v36_matches() {
        check(ee::s11::f_00358fd0__generated, anim_combo_v36, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x358f_d0);
            let s = DATA + 0x100;
            clear(c, s, 0x100);
            w(c, s + 0x14, if seed % 2 == 0 { 0 } else { 1 });
            w(c, s + 0x38, r.next_u32());
            w(c, s + 8, r.next_u32());
            w(c, s + 0x48, 0);
            w(c, s + 4, r.next_u32());
            c.f[12] = r.next_u32();
            c.g[A0][0] = sx(s);
            let msg = match seed % 6 {
                0 => 0x82,
                1 => 0x64,
                2 => 0x65,
                3 => 0x67,
                4 => 0x68,
                _ => 0xff,
            };
            c.g[A1][0] = msg as u64;
        });
    }

    #[test]
    fn fence_kick_from_neighbour_matches() {
        check(ee::s11::f_0034cde8__generated, fence_kick_from_neighbour, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x34cd_e8);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, s + 0x38, if seed % 3 == 0 { 0 } else { 1 });
            w(c, s + 0x3c, if seed % 4 == 0 { 0 } else { 1 });
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn rider_model_hide_all_parts_matches() {
        check_lohi(ee::s10::f_0030eb60__generated, rider_model_hide_all_parts, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x30eb_60);
            let s = DATA + 0x100;
            clear(c, s, 0x200);
            let np = (seed % 4) as u32;
            let parts = DATA + 0x400;
            clear(c, parts, 0x200);
            w(c, s + 8, np);
            w(c, s + 0xc, parts as u32);
            for j in 0..np {
                for k in 0..4 {
                    w(c, parts + 0x58 * j + 0x18 + 8 * k, 1);
                }
            }
            c.g[A0][0] = sx(s);
            c.g[A3][0] = if seed % 2 == 0 { 0 } else { 0x40 };
        });
    }

    #[test]
    fn tex_man_alloc_btex_info_matches() {
        check(ee::s12::f_00367260__generated, tex_man_alloc_btex_info, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3672_60);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, s, r.next_u32());
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 8) as u64;
        });
    }

    

    // ------------------------------------------------------------------
    // Sound.

    #[test]
    fn snd_bank_unload_matches() {
        check(ee::s07::f_0028b528__generated, snd_bank_unload, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x28b5_28);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, s, if seed % 3 == 0 { 1 } else { 0 });
            w(c, s + 4, if seed % 2 == 0 { 0 } else { DATA + 0x300 });
            w(c, s + 8, if seed % 4 < 2 { 0 } else { 1 });
            w(c, s + 0x10, DATA + 0x400);
            clear(c, DATA + 0x300, 0x40);
            clear(c, DATA + 0x400, 0x40);
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn in_seq_channel_release_matches() {
        check(ee::s07::f_00273a00__generated, in_seq_channel_release, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x273a_00);
            let s = DATA + 0x100;
            clear(c, s, 0x80);
            w(c, s + 4, if seed % 2 == 0 { 0 } else { 1 });
            // `InSeq_ChannelStop` returns at once with no bound stream.
            w(c, s + 0x28, 0);
            c.g[A0][0] = sx(s);
        });
    }

    // ------------------------------------------------------------------
    // Front-end.

    #[test]
    fn fe_state_net_messenger_update_matches() {
        check(ee::s03::f_001a3fe0__generated, fe_state_net_messenger_update, 24, &|c, seed| {
            let mut r = base(c, seed, 0x1a3f_e0);
            let s = DATA + 0x100;
            clear(c, s, 0x800);
            let obj = DATA + 0x1000;
            clear(c, obj, 0x40);
            let now = r.next_u32();
            w(c, obj + 0x18, now);
            w(c, gp_at(0x2a74), obj as u32);
            w(
                c,
                s + 0x6dc,
                if seed % 2 == 0 { now.wrapping_sub(1) } else { now.wrapping_sub(100) },
            );
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn fe_state_net_messenger_on_find_user_result_matches() {
        check(ee::s03::f_001a4fb0__generated, fe_state_net_messenger_on_find_user_result, 48, &|c, seed| {
            let mut r = base(c, seed, 0x1a4f_b0);
            let s = DATA + 0x100;
            clear(c, s, 0x800);
            w(c, s + 0x1c, if seed % 2 == 0 { 0 } else { 6 });
            let reply = DATA + 0x400;
            clear(c, reply, 0x40);
            w(c, s + 0x6a0, if seed % 3 == 0 { 0 } else { reply as u32 });
            w(c, reply + 0x18, if seed % 4 == 0 { 0x807 } else { r.next_u32() });
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA + 0x500);
            clear(c, DATA + 0x500, 0x40);
        });
    }

    #[test]
    fn fe_state_net_messenger_set_player_menu_selection_matches() {
        check(ee::s03::f_001a6c40__generated, fe_state_net_messenger_set_player_menu_selection, 48, &|c, seed| {
            let _ = base(c, seed, 0x1a6c_40);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            let screen = DATA + 0x300;
            clear(c, screen, 0x40);
            if seed % 3 != 0 {
                w(c, s + 0x40, screen as u32);
                let objs = DATA + 0x400;
                let count = DATA + 0x440;
                let menu = DATA + 0x480;
                clear(c, objs, 0x10);
                clear(c, count, 4);
                clear(c, menu, 0x100);
                w(c, screen + 0x3c, objs as u32);
                w(c, screen + 0x38, count as u32);
                if seed % 2 == 0 {
                    w(c, count, 0);
                } else {
                    w(c, count, 1);
                    w(c, objs, menu as u32);
                    w(c, menu + 0x38, 0);
                }
            }
            c.g[A0][0] = sx(s);
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(0xffff_ffff) };
        });
    }

    #[test]
    fn net_fe_state_update_matches() {
        check(ee::s03::f_001a8770__generated, net_fe_state_update, 24, &|c, seed| {
            let mut r = base(c, seed, 0x1a87_70);
            let s = DATA + 0x100;
            clear(c, s, 0x800);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn net_fe_state_open_keyboard_str_matches() {
        check(ee::s03::f_001a9090__generated, net_fe_state_open_keyboard_str, 32, &|c, seed| {
            let mut r = base(c, seed, 0x1a90_90);
            let s = DATA + 0x100;
            clear(c, s, 0x800);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA + 0x200);
            c.g[A3][0] = sx(DATA + 0x300);
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
            c.g[T2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn net_fe_state_open_keyboard_text_id_matches() {
        check(ee::s03::f_001a9150__generated, net_fe_state_open_keyboard_text_id, 32, &|c, seed| {
            let mut r = base(c, seed, 0x1a91_50);
            let s = DATA + 0x100;
            clear(c, s, 0x800);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA + 0x200);
            c.g[A3][0] = sx(DATA + 0x300);
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
            c.g[T2][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn fe_state_net_config_start_card_scan_matches() {
        check(ee::s03::f_001b5268__generated, fe_state_net_config_start_card_scan, 32, &|c, seed| {
            let mut r = base(c, seed, 0x1b52_68);
            let s = DATA + 0x100;
            clear(c, s, 0x800);
            w(c, s + 0x6e4, DATA + 0x400);
            w(c, s + 0x6e8, DATA + 0x480);
            w(c, s + 0x6f4, DATA + 0x500);
            clear(c, DATA + 0x400, 0x40);
            clear(c, DATA + 0x480, 0x40);
            clear(c, DATA + 0x500, 0x40);
            w(c, gp_at(-0x1570), r.next_u32());
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn fe_state_net_account_mem_popup_id_to_state_matches() {
        check(ee::s03::f_001b8890__generated, fe_state_net_account_mem_popup_id_to_state, 120, &|c, seed| {
            base(c, seed, 0x1b88_90);
            c.g[A1][0] = match seed % 8 {
                0 => 13,
                1 => 21,
                2 => 7,
                3 => 25,
                4 => 14,
                5 => 0,
                6 => 20,
                _ => sx(0xffff_fff0),
            };
        });
    }

    #[test]
    fn fe_state_create_step1_set_email_text_matches() {
        check(ee::s03::f_001b96e0__generated, fe_state_create_step1_set_email_text, 48, &|c, seed| {
            let mut r = base(c, seed, 0x1b96_e0);
            let s = DATA + 0x100;
            clear(c, s, 0x800);
            let screen = DATA + 0x300;
            clear(c, screen, 0x40);
            if seed % 3 != 0 {
                w(c, s + 0x40, screen as u32);
                let objs = DATA + 0x400;
                let count = DATA + 0x440;
                let text = DATA + 0x480;
                clear(c, objs, 0x10);
                clear(c, count, 4);
                clear(c, text, 0x40);
                w(c, screen + 0x3c, objs as u32);
                w(c, screen + 0x38, count as u32);
                w(c, count, 1);
                w(c, objs, text as u32);
                w(c, text + 0x38, r.next_u32());
                let holder = DATA + 0x500;
                clear(c, holder, 0x20);
                w(c, s + 0x6d0, holder as u32);
                w(c, holder + 0xc, DATA + 0x600);
                // A string with a non-zero length word at `s - 8`.
                w(c, DATA + 0x600, 0);
                w(c, DATA + 0x5f8, if seed % 2 == 0 { 0 } else { 4 });
            }
            c.g[A0][0] = sx(s);
        });
    }

    // ------------------------------------------------------------------
    // Message center.

    #[test]
    fn message_center_post_once_3c_matches() {
        check(ee::s04::f_001e2518__generated, message_center_post_once_3c, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x1e25_18);
            let world = DATA + 0x1000;
            let man = DATA + 0x2000;
            let list = DATA + 0x3000;
            clear(c, world, 0x100);
            clear(c, man, 0x80);
            clear(c, list, 0x40);
            w(c, gp_at(-0x848), world as u32);
            w(c, world + 0x84, man as u32);
            w(c, man + 0x28, list as u32);
            w(c, list, match seed % 4 { 0 => 0, 1 => 3, 2 => 12, _ => r.next_u32() });
        });
    }

    #[test]
    fn message_center_post_once_38_matches() {
        check(ee::s04::f_001e2578__generated, message_center_post_once_38, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x1e25_78);
            let world = DATA + 0x1000;
            let man = DATA + 0x2000;
            let list = DATA + 0x3000;
            clear(c, world, 0x100);
            clear(c, man, 0x80);
            clear(c, list, 0x40);
            w(c, gp_at(-0x848), world as u32);
            w(c, world + 0x84, man as u32);
            w(c, man + 0x28, list as u32);
            w(c, list, match seed % 4 { 0 => 0, 1 => 3, 2 => 12, _ => r.next_u32() });
        });
    }

    #[test]
    fn message_center_remove_message_by_id_matches() {
        check(ee::s04::f_001e3268__generated, message_center_remove_message_by_id, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x1e32_68);
            // `MessageCenter_GetInbox` walks the BELibrary singleton.
            let lib = DATA + 0x1000;
            clear(c, lib, 0x100);
            w(c, gp_at(-0x1f04), lib as u32);
            let box_ = DATA + 0x2000;
            let id = r.next_u32();
            let n = (seed % 5) as u8;
            inbox(
                c,
                box_,
                n,
                &[
                    r.next_u32(),
                    if seed % 3 == 0 { id } else { r.next_u32() },
                    r.next_u32(),
                    r.next_u32(),
                ],
            );
            // The inbox pointer the (faked) `MessageCenter_GetInbox` returns.
            w(c, LOG_RESULTS, box_);
            c.pc = 0x001e_3268;
            c.g[A0][0] = id as u64;
        });
    }

    // ------------------------------------------------------------------
    // Misc.

    #[test]
    fn tex_flip_modifier_save_matches() {
        check(ee::s12::f_0035f688__generated, tex_flip_modifier_save, 40, &|c, seed| {
            base(c, seed, 0x35f6_88);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            let stream = DATA + 0x300;
            clear(c, stream, 0x20);
            let vt = DATA + 0x400;
            vtable(
                c,
                vt,
                &[(0x8, FAKE + 0x10), (0xc, FAKE + 0x20), (0x28, FAKE + 0x30), (0x2c, FAKE + 0x40)],
            );
            w(c, stream, vt as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(stream);
        });
    }

    #[test]
    fn lobby_man_release_snapshot_lists_matches() {
        check(ee::s06::f_0025a850__generated, lobby_man_release_snapshot_lists, 40, &|c, seed| {
            let _ = base(c, seed, 0x25a8_50);
            // `LobbyApi_DestroyList` destroys and frees each non-null list.
            heap_init(c);
            let s = DATA + 0x100;
            clear(c, s, 0x400);
            let arr = s + 0x1ec;
            for i in 0..29u32 {
                if (seed + i) % 4 == 0 {
                    let list = heap_alloc(c, 0x40);
                    w(c, arr + 4 * i, list);
                } else {
                    w(c, arr + 4 * i, 0);
                }
            }
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn lobby_man_send_chat_matches() {
        check(ee::s06::f_0025aae0__generated, lobby_man_send_chat, 32, &|c, seed| {
            let mut r = base(c, seed, 0x25aa_e0);
            let s = DATA + 0x100;
            clear(c, s, 0x80);
            w(c, s, if seed % 3 == 0 { 0 } else { 1 });
            w(c, s + 0x54, DATA + 0x300);
            clear(c, DATA + 0x300, 0x40);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn chat_event_dtor_matches() {
        check(ee::s07::f_00268048__generated, chat_event_dtor, 48, &|c, seed| {
            let _ = base(c, seed, 0x2680_48);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            // Both embedded strings are the shared static empty string, so the
            // string dtors return without touching the heap; `flags = 0` keeps
            // the object owned by the caller.
            w(c, s, seed as u32);
            w(c, s + 8, EMPTY_STRING);
            w(c, s + 0xc, EMPTY_STRING);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = 0;
        });
    }

    #[test]
    fn net_event102_lock_dtor_matches() {
        check(ee::s07::f_002682a8__generated, net_event102_lock_dtor, 48, &|c, seed| {
            let _ = base(c, seed, 0x2682_a8);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, s, seed as u32);
            w(c, s + 0xc, EMPTY_STRING);
            w(c, s + 0x10, EMPTY_STRING);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = 0;
        });
    }

    #[test]
    fn net_event_e3_dtor_matches() {
        check(ee::s07::f_00268460__generated, net_event_e3_dtor, 48, &|c, seed| {
            let _ = base(c, seed, 0x2684_60);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, s, seed as u32);
            w(c, s + 8, EMPTY_STRING);
            w(c, s + 0xc, EMPTY_STRING);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = 0;
        });
    }

    #[test]
    fn net_event_e2_clone_matches() {
        check(ee::s07::f_002681b0__generated, net_event_e2_clone, 40, &|c, seed| {
            let mut r = base(c, seed, 0x2681_b0);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            // A null source string: the `cBXString` ctor then skips the
            // (faked) `strlen` and the buffer allocation.
            w(c, s + 8, 0);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn sound_manager_ref_ctor_matches() {
        check(ee::s08::f_002ade88__generated, sound_manager_ref_ctor, 32, &|c, seed| {
            let mut r = base(c, seed, 0x2ade_88);
            // `PS2SoundManager_Create` allocates 0x320 bytes (heap path).
            heap_init(c);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, gp_at(0x68c), r.next_u32());
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn ps2_mem_card_v38_matches() {
        check(ee::s08::f_002c46c0__generated, ps2_mem_card_v38, 32, &|c, seed| {
            let mut r = base(c, seed, 0x2c46_c0);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            let vt = DATA + 0x400;
            vtable(c, vt, &[(0x120, FAKE + 0x10), (0x124, FAKE + 0x20)]);
            w(c, s, vt as u32);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn ps2_mem_card_clear_dir_cache_matches() {
        check(ee::s08::f_002c5278__generated, ps2_mem_card_clear_dir_cache, 32, &|c, seed| {
            let _ = base(c, seed, 0x2c52_78);
            let s = DATA + 0x1000;
            clear(c, s, 0x1000);
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn rider_camera_algorithm_ctor_matches() {
        check(ee::s01::f_001622b0__generated, rider_camera_algorithm_ctor, 40, &|c, seed| {
            let mut r = base(c, seed, 0x1622_b0);
            let s = DATA + 0x100;
            clear(c, s, 0x40);
            w(c, gp_at(-0x6b0c), r.next_u32());
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn keyboard_popup_find_enabled_in_column_matches() {
        check(ee::s04::f_001cc0e0__generated, c_keyboard_popup_find_enabled_in_column_from_bottom, SEEDS, &|c, seed| {
            base(c, seed, 0x1cc0_e0);
            let s = DATA + 0x100;
            clear(c, s, 0x400);
            // Index 56 (start 0x38 from a zero base) is disabled, 42 enabled:
            // the loop takes two iterations.
            w(c, s + 0x378, 0);
            w(c, s + 0x340, 1);
            c.pc = 0x001c_c0e0;
            c.g[A0][0] = sx(s);
            c.g[A1][0] = if seed % 2 == 0 { 0xffff_ffff_ffff_ffff } else { 0x38 };
        });
    }

    #[test]
    fn keyboard_popup_set_digit_keys_disabled_matches() {
        check(ee::s04::f_001ce408__generated, c_keyboard_popup_set_digit_keys_disabled, 48, &|c, seed| {
            let _ = base(c, seed, 0x1ce4_08);
            let s = DATA + 0x100;
            clear(c, s, 0x400);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn keyboard_popup_set_function_keys_enabled_matches() {
        check(ee::s04::f_001ce468__generated, c_keyboard_popup_set_function_keys_enabled, 48, &|c, seed| {
            let _ = base(c, seed, 0x1ce4_68);
            let s = DATA + 0x100;
            clear(c, s, 0x400);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn w_script_man_request_mission_restart_matches() {
        check(ee::s10::f_0030b6f8__generated, w_script_man_request_mission_restart, 32, &|c, seed| {
            let mut r = base(c, seed, 0x30b6_f8);
            let world = DATA + 0x1000;
            let man = DATA + 0x2000;
            clear(c, world, 0x100);
            clear(c, man, 0x100);
            w(c, gp_at(-0x848), world as u32);
            w(c, world + 0x84, man as u32);
            w(c, man + 0x34, DATA + 0x3000);
            w(c, man + 0x28, DATA + 0x3400);
            clear(c, DATA + 0x3000, 0x40);
            clear(c, DATA + 0x3400, 0x40);
            let s = DATA + 0x4000;
            clear(c, s, 0x400);
            w(c, s + 0x2a0, r.next_u32());
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn playback_man_save_world_state_matches() {
        check(ee::s07::f_0026cbb8__generated, playback_man_save_world_state, 48, &|c, seed| {
            base(c, seed, 0x26cb_b8);
            let s = DATA + 0x100;
            clear(c, s, 0x100);
            w(c, s + 0x10, if seed % 3 == 0 { 0 } else { 1 });
            let holder = DATA + 0x300;
            clear(c, holder, 0x40);
            w(c, s + 0x5c, holder as u32);
            w(c, holder + 0x28, if seed % 4 == 0 { 0 } else { DATA + 0x400 });
            clear(c, DATA + 0x400, 0x40);
            c.g[A0][0] = sx(s);
        });
    }
}
