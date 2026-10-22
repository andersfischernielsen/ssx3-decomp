//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! The functions here are cold-path UI, script, audio and rider helpers. Every
//! replacement copies the generated body's block cycle charges, the registers
//! each call leaves, the values left in `v0`/`v1` at return, and the stack
//! frame shape (save slots, offsets, order). Calls go through
//! [`crate::call::call_ptr`] with the same return address the generated body
//! uses, which also reproduces the unwind check after the call.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Frame, Flow};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_a670, ui_list_box_clear, "UIListBox_Clear"),
    (0x0017_a0b8, debug_menu_prev_enabled_item, "DebugMenu_PrevEnabledItem"),
    (0x002f_59d0, world_light_man_evict_oldest_instance, "WorldLightMan_EvictOldestInstance"),
    (0x0030_e9e0, rider_model_show_part, "RiderModel_ShowPart"),
    (0x0031_a268, vu0_set_signed_unit_vector, "VU0_SetSignedUnitVector"),
    (0x0031_1a50, anim_controller_reset, "AnimController_Reset"),
    (0x001a_8ae8, net_fe_state_queue_net_event, "NetFEState_QueueNetEvent"),
    (0x001a_9a18, net_fe_state_fill_yes_no_list, "NetFEState_FillYesNoList"),
    (0x001a_9ab8, net_fe_state_fill_on_off_list, "NetFEState_FillOnOffList"),
    (0x0028_75d0, ssx_audio_system_set_sfx_sub_channels_enabled, "SSXAudioSystem_SetSfxSubChannelsEnabled"),
    (0x0028_f328, ssx_audio_system_on_music_post_event, "SSXAudioSystem_OnMusicPostEvent"),
    (0x001c_1e08, c_fe_state_net_ssx_world_on_widget_created, "cFEStateNetSSXWorld_OnWidgetCreated"),
    (0x0025_fb80, lobby_man_send_private_message, "LobbyMan_SendPrivateMessage"),(0x0027_b370, script_engine_load_bank_async, "ScriptEngine_LoadBankAsync"),
    (0x0027_d170, script_engine_clear_obj_binding_filter, "ScriptEngine_ClearObjBindingFilter"),
    (0x001c_bc48, c_keyboard_popup_move_down_from_top_row, "cKeyboardPopup_MoveDownFromTopRow"),
    (0x001c_bf20, c_keyboard_popup_move_up_from_bottom_row, "cKeyboardPopup_MoveUpFromBottomRow"),(0x0030_b658, wscript_man_request_mission_cancel, "WScriptMan_RequestMissionCancel"),(0x0030_b7f8, wscript_man_deactivate_all, "WScriptMan_DeactivateAll"),
    (0x0027_d400, script_engine_activate_binding, "ScriptEngine_ActivateBinding"),
    (0x002c_be68, menu_sys_page_set_selection, "MenuSysPage_SetSelection"),(0x001a_3308, c_fe_state_mp_circuit_on_activate, "cFEStateMPCircuit_OnActivate"),
    (0x001a_9438, net_fe_state_on_mem_card_login_popup_event, "NetFEState_OnMemCardLoginPopupEvent"),(0x001f_b218, ov_screen_in_game_error_update_opponent_disconnected_text, "OVScreenInGameError_UpdateOpponentDisconnectedText"),
    (0x0015_8be0, be_reward_int_get_next_points_threshold, "BERewardInt_GetNextPointsThreshold"),
    (0x0011_9958, rider_metrix_chain_uber_grind, "RiderMetrix_ChainUberGrind"),
    (0x0011_9a38, rider_metrix_finish_board_press, "RiderMetrix_FinishBoardPress"),
    (0x002e_42d0, c_transition_man_dtor, "cTransitionMan_dtor"),
    (0x001d_1b68, c_fe_state_preview_reward_dtor, "cFEStatePreviewReward_dtor"),(0x0031_8448, cbxstring_assign, "cBXString_assign"),
    (0x0031_8630, cbxstring_op_plus_str_cstr, "cBXString_OpPlusStrCStr"),
    (0x003e_bf28, tag_field_set_flags, "TagFieldSetFlags"),
    (0x003a_1958, c_ui_text_scroll_grow_lines, "cUITextScroll_GrowLines"),
    (0x0022_4d00, luno_table_dtor, "LunoTable_dtor"),
    (0x0034_d650, fence_v40, "Fence_v40"),
    (0x0018_6b50, c_fe_state_mem_card_init, "cFEStateMemCard_Init"),
    (0x003a_6740, world_dtor, "World_dtor"),
    (0x001e_9a30, c_ov_state_hud_1p_ctor, "cOVStateHUD1P_ctor"),
    (0x001f_90e8, ov_screen_pause_freeride_init_mode_6, "OVScreenPauseFreeride_InitMode6"),
    (0x0028_3dc0, script_cmd_queue_pop, "ScriptCmdQueue_Pop"),
    (0x0013_6d40, rider_wipeout_state_enable_root_pose_override, "RiderWipeoutState_EnableRootPoseOverride"),
    (0x0014_dfa8, be_library_load_state, "BELibrary_LoadState"),
    (0x001a_7ed8, c_fe_state_read_message_popup_add_sent_message, "cFEStateReadMessagePopup_AddSentMessage"),
];

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `sp + off` (a frame slot).
#[inline(always)]
fn frame_at(c: &Cpu, off: u32) -> u32 {
    r32(c, SP).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw` (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// `sw`.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v);
}

// ---------------------------------------------------------------------------
// UI list box.

/// `void UIListBox_Clear(UIListBox* box)` (0x39a670): empty the list. When the
/// box owns its item storage (bit 0x10 of `+0x74`) it frees the single data
/// block at `+0x9c`; otherwise it frees the data of every item (0x14 bytes
/// each, count at `+0x318`). Then zero the count and the flag at `+0x319`.
pub fn ui_list_box_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);

        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x74));
        c.g[V0][0] &= 0x10;
        let mut done = false;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x318)).lbu(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x9c));
            if c.g[A0][0] == 0 {
                Addr(p(c.g[S1][0] as u32, 0x318)).sb(c, 0);
                done = true;
            } else {
                c.cycles += 4;
                call_ptr(c, 0x0031_7e98, 0x0039_a6ac)?; // EA_MemFree
                Addr(p(c.g[S1][0] as u32, 0x318)).sb(c, 0);
                done = true;
            }
        }
        if !done {
            c.cycles += 2;
            let empty = c.g[V0][0] == 0;
            c.g[S2][0] = 0;
            if !empty {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c));
                loop {
                    c.cycles += 3;
                    c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                    let count;
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x318)).lbu(c);
                        count = c.g[V0][0];
                    } else {
                        c.cycles += 3;
                        call_ptr(c, 0x0031_7e98, 0x0039_a6d4)?; // EA_MemFree
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x318)).lbu(c);
                        count = c.g[V0][0];
                    }
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (count as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_a6c0);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 1;
            Addr(p(c.g[S1][0] as u32, 0x318)).sb(c, 0);
        }
        c.cycles += 7;
        Addr(p(c.g[S1][0] as u32, 0x319)).sb(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Debug menu.

/// `int DebugMenu_PrevEnabledItem(DebugMenu* menu, int start)` (0x17a0b8):
/// walk backwards from `start` over the menu's item table, wrapping with the
/// item count at `menu+0`, and return the first item whose `+0x118` word is
/// non-zero; -1 when the menu is disabled (`menu+0x8808` is zero) or none is
/// enabled.
pub fn debug_menu_prev_enabled_item(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x8000;
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc08));
    if c.g[V1][0] != 0 {
        c.g[A2][0] = lw(c, c.g[A3][0] as u32);
        c.cycles += 5;
        c.g[A0][0] = 0x8c;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8c));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.div(0, 2, 6);
        c.g[V1][0] = c.hi[0];
        c.g[A1][0] = c.g[V1][0];
        c.mult(0, 4, 5, 4);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = lw(c, c.g[A0][0] as u32);
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A2][0];
        if !found {
            c.cycles += 4;
            c.g[T0][0] = 0;
            c.g[A2][0] = 0x8c;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            loop {
                c.cycles += 2;
                // `if (a0 == t0) break; else break;` — both branches leave the block.
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.div(0, 2, 4);
                c.g[V1][0] = c.hi[0];
                c.g[A1][0] = c.g[V1][0];
                c.mult(0, 3, 5, 6);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = lw(c, c.g[V0][0] as u32);
                let stop = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if stop {
                    if event_due(c) {
                        deliver_event_here(c, 0x0017_a120);
                    }
                    continue;
                }
                break;
            }
        }
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World light manager.

/// `WorldLightInstance* WorldLightMan_EvictOldestInstance(WorldLightMan* man,
/// int threshold)` (0x2f59d0): scan the 32 light instances at `+0x210`
/// (0x180 bytes each), keep the one with the smallest stamp at `+4` that is
/// below `threshold`, unlink it and move it to the free slot named by its low
/// five bits (`+0x3210`), then return it (or the last scanned instance).
pub fn world_light_man_evict_oldest_instance(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[T0][0] = 0;
    c.g[A3][0] = 0;
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x210));
    loop {
        c.cycles += 4;
        c.g[V1][0] = lw(c, p(c.g[A2][0] as u32, 4));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let skip = c.g[V0][0] == 0;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        if !skip {
            c.cycles += 2;
            c.g[A1][0] = c.g[V1][0];
            c.g[T0][0] = c.g[A2][0];
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[A3][0] < 0x20) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x180));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002f_59e0);
            }
            continue;
        }
        c.cycles += 6;
        c.g[V0][0] = c.ld(c.g[T0][0] as u32);
        c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 0x174));
        c.g[V0][0] = c.g[V0][0] << 24;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        let has_next = c.g[V1][0] != 0;
        c.g[V0][0] &= 0x1f;
        if has_next {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            if c.g[V0][0] != 0 {
                sw(c, p(c.g[V1][0] as u32, 0x170), c.g[V0][0]);
            } else {
                c.cycles += 2;
                sw(c, p(c.g[V1][0] as u32, 0x170), 0);
                c.cycles += 3;
                c.g[V0][0] = c.g[T0][0];
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            c.g[V0][0] = lw(c, p(c.g[T0][0] as u32, 0x174));
            sw(c, p(c.g[V1][0] as u32, 0x174), c.g[V0][0]);
        } else {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            sw(c, p(c.g[V0][0] as u32, 0x3210), c.g[V1][0]);
            c.g[V0][0] = lw(c, p(c.g[T0][0] as u32, 0x170));
            if c.g[V0][0] != 0 {
                sw(c, p(c.g[V0][0] as u32, 0x174), 0);
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[T0][0];
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[T0][0];
        abi::ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// Rider model.

/// `void RiderModel_ShowPart(RiderModel* model, int part, int slot)` (0x30e9e0):
/// reveal one part. Parts past the count at `+0x14` are ignored; a part whose
/// table (`+0x1c`) entry is negative is ignored. Otherwise set the part's
/// visibility word (`+0x18`) on the part block (`+0xc + 88*part`), or, when
/// `slot` is negative, all four (`+0x30..+0x18`), and OR its 64-bit mask at
/// `+0x150`.
pub fn rider_model_show_part(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = lw(c, p(c.g[A3][0] as u32, 0x14));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[A3][0] as u32, 0x1c));
        let no_table = c.g[A1][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        if !no_table {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = lw(c, c.g[V0][0] as u32);
            let negative = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = 0x58;
            if !negative {
                c.cycles += 8;
                c.g[A1][0] = lw(c, p(c.g[A3][0] as u32, 0xc));
                c.mult(0, 2, 3, 2);
                c.g[V1][0] = c.ld(p(c.g[A3][0] as u32, 0x150));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.ld(p(c.g[A1][0] as u32, 0x10));
                c.g[V1][0] |= c.g[A0][0];
                let all = (c.g[A2][0] as i64) < 0;
                c.sd(p(c.g[A3][0] as u32, 0x150), c.g[V1][0]);
                if !all {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
                    c.g[V1][0] = 1;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    sw(c, p(c.g[V0][0] as u32, 0x18), c.g[V1][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = 1;
                    c.g[V1][0] = 3;
                    loop {
                        c.cycles += 7;
                        sw(c, c.g[V0][0] as u32, c.g[A0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                        let again = (c.g[V1][0] as i64) >= 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
                        if again {
                            if event_due(c) {
                                deliver_event_here(c, 0x0030_ea40);
                            }
                            continue;
                        }
                        break;
                    }
                }
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// VU0 helper.

/// `void VU0_SetSignedUnitVector(int flags, float* out)` (0x31a268): write four
/// components of ±1 or a table value to `out[0..3]`, choosing each component's
/// value from `flags` bits 0xb/0x3 (0x8800), bit 3 (± from the tables at
/// `gp-0x2ee0`/`gp-0x2edc`) and bit 7 (negate). Then re-pack the four floats in
/// VU0 VF1/VF2 and multiply them (`mul.xyzw`), restoring the VI registers.
pub fn vu0_set_signed_unit_vector(c: &mut Cpu) {
    c.cycles += 4;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[A2][0] = 0;
    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.g[SP][0];
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0] & 0x8800;
        let hi_bits = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[A0][0] & 0x8;
        let bit3 = c.g[V0][0] != 0;
        if hi_bits {
            c.cycles += 3;
            c.lwc1(0, gp(c, -0x2ee0));
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            if !bit3 {
                c.cycles += 1;
                c.lwc1(0, gp(c, -0x2edc));
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0] & 0x80;
        } else {
            c.cycles += 3;
            c.mtc1(0, 0);
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            if bit3 {
                c.cycles += 1;
                c.g[V0][0] = c.g[A0][0] & 0x80;
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.g[A0][0] & 0x80;
            }
        }
        c.cycles += 2;
        let positive = c.g[V0][0] == 0;
        c.swc1(0, c.g[A1][0] as u32);
        if !positive {
            c.cycles += 1;
            c.neg_s(0, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.swc1(0, c.g[V0][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0031_a278);
            }
            continue;
        }
        break;
    }
    c.cycles += 10;
    c.qmfc2(8, 1);
    c.qmfc2(9, 2);
    c.lqc2(1, c.g[SP][0] as u32);
    c.lqc2(2, p(c.g[SP][0] as u32, 0x10));
    c.vu0().mul(0b1111, 1, 1, Src::Vec(2));
    c.qmtc2(8, 1);
    c.qmtc2(9, 2);
    c.vu0().vnop();
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Animation controller.

/// `void AnimController_Reset(AnimController* self)` (0x311a50): clear the six
/// animation sequence lists at `+0x50` (8 bytes each), set each current
/// animation id (`+0..+0x14`) to 0x1b6, and restore the defaults: `+0x18` = 0,
/// `+0x1c` = 1.0, `+0x20` = -1 and the two 16-byte constants at `0x4ff230`
/// copied to `+0x30`/`+0x40`.
pub fn anim_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[S1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x1b6;
        loop {
            c.cycles += 9;
            c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x50));
            c.g[A0][0] = sx((c.g[S2][0] as u32) << 3);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x0031_4668, 0x0031_1a8c)?; // AnimSeqList_Clear
            sw(c, c.g[S0][0] as u32, c.g[S3][0]);
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_1a78);
                }
                continue;
            }
            break;
        }
        c.cycles += 19;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = 0x0050_0000;
        sw(c, p(c.g[S1][0] as u32, 0x18), 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdd0));
        c.swc1(0, p(c.g[S1][0] as u32, 0x1c));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S0, 0x40);
        c.lq(2, c.g[A0][0] as u32);
        frame.restore(c, S2, 0x20);
        c.sq(2, p(c.g[S1][0] as u32, 0x30));
        frame.restore(c, S3, 0x10);
        c.lq(3, p(c.g[A0][0] as u32, 0x10));
        Addr(p(c.g[S1][0] as u32, 0x20)).sd(c, c.g[A1][0]);
        c.sq(3, p(c.g[S1][0] as u32, 0x40));
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Network front end.

/// `void NetFEState_QueueNetEvent(NetFEState* self, NetEvent* event)`
/// (0x1a8ae8): turn the event object into a queued entry through its virtual
/// conversion at vtable slot 0x10 (returning the payload), allocate a 12-byte
/// queue node from `self+0x6cc`'s list and push it on both the node list and
/// the pending list.
pub fn net_fe_state_queue_net_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        let no_event = c.g[A1][0] == 0;
        frame.save_ra(c, 0x20);
        if !no_event {
            c.cycles += 8;
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 4));
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x10)).lh(c);
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x14));
            c.g[V0][0] = sx(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x001a_8b14)?;
            c.g[S0][0] = c.g[V0][0];
            let skip = c.g[S0][0] == 0;
            c.g[A1][0] = 0x0046_0000;
            if !skip {
                c.cycles += 11;
                c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x6cc));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3218));
                c.g[A0][0] = 0xc;
                sw(c, p(c.g[SP][0] as u32, 0x10), c.g[V0][0]);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001a_8b3c)?; // EA_MemAllocNamed
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    sw(c, p(c.g[A1][0] as u32, 8), c.g[S0][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = lw(c, p(c.g[SP][0] as u32, 0x10));
                sw(c, c.g[A1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 4));
                sw(c, p(c.g[A1][0] as u32, 4), c.g[V1][0]);
                c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 4));
                sw(c, c.g[A0][0] as u32, c.g[A1][0]);
                c.g[V0][0] = lw(c, p(c.g[SP][0] as u32, 0x10));
                sw(c, p(c.g[V0][0] as u32, 4), c.g[A1][0]);
                sw(c, c.g[SP][0] as u32, c.g[A1][0]);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void NetFEState_FillYesNoList(NetFEState* self, UIListBox* box, int mode)`
/// (0x1a9a18): mark the box as having a background (bit 0x80 of `+0x14`), add
/// the "yes" and "no" string ids (the first as the initial item) and select
/// the mode, then refresh the box's parent list.
pub fn net_fe_state_fill_yes_no_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x80;
        sw(c, p(c.g[S0][0] as u32, 0x14), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001a_9a50)?; // StrHash
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        call_ptr(c, 0x0039_a568, 0x001a_9a60)?; // UIListBox_AddItemStrId
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
        call_ptr(c, 0x0031_7670, 0x001a_9a6c)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_a568, 0x001a_9a7c)?; // UIListBox_AddItemStrId
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = (c.g[S1][0] < 1) as u64;
        call_ptr(c, 0x0039_a7a8, 0x001a_9a88)?; // UIListBox_SetSelection
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x001a_9aa0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void NetFEState_FillOnOffList(NetFEState* self, UIListBox* box, int mode)`
/// (0x1a9ab8): the same fill as `NetFEState_FillYesNoList` with the "on"/"off"
/// string ids.
pub fn net_fe_state_fill_on_off_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x80;
        sw(c, p(c.g[S0][0] as u32, 0x14), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x001a_9af0)?; // StrHash
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        call_ptr(c, 0x0039_a568, 0x001a_9b00)?; // UIListBox_AddItemStrId
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
        call_ptr(c, 0x0031_7670, 0x001a_9b0c)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_a568, 0x001a_9b1c)?; // UIListBox_AddItemStrId
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = (c.g[S1][0] < 1) as u64;
        call_ptr(c, 0x0039_a7a8, 0x001a_9b28)?; // UIListBox_SetSelection
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x001a_9b40)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Audio system.

/// `void SSXAudioSystem_SetSfxSubChannelsEnabled(AudioSystem* self, int on)`
/// (0x2875d0): mute sound-effect channels 9 and 10 (volume 0) when `on` is
/// zero; otherwise restore each to the channel-5 volume and remember the
/// setting at `+0x62b4`.
pub fn ssx_audio_system_set_sfx_sub_channels_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let off = c.g[S1][0] == 0;
        frame.save_fpr(c, 20, 0x30);
        if off {
            c.cycles += 10;
            c.mtc1(0, 20);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 9;
            c.mov_s(12, 20);
            call_ptr(c, 0x0028_7700, 0x0028_7640)?; // SetChannelVolume
            c.mov_s(12, 20);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0028_7700, 0x0028_7650)?;
            sw(c, p(c.g[S0][0] as u32, 0x62b4), c.g[S1][0]);
        } else {
            c.cycles += 15;
            c.g[A1][0] = 5;
            call_ptr(c, 0x0028_7920, 0x0028_75f8)?; // GetChannelVolume
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 9;
            call_ptr(c, 0x0028_7700, 0x0028_7608)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 5;
            call_ptr(c, 0x0028_7920, 0x0028_7614)?;
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0028_7700, 0x0028_7624)?;
            sw(c, p(c.g[S0][0] as u32, 0x62b4), c.g[S1][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_OnMusicPostEvent(AudioSystem* self, int event)`
/// (0x28f328): post the event to the embedded music player, then, when the
/// current song tag came back as none, track the restart events (10, 11, 18)
/// in `+0x530` and clear it (stopping the song through the player's vtable
/// slot 0x18 for event 18).
pub fn ssx_audio_system_on_music_post_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_3b88, 0x0028_f350)?; // MusicPlayer_PostEvent
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_49e0, 0x0028_f358)?; // MusicPlayer_GetSongTag
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        let mut early = false;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x30);
            early = true;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            let restart = c.g[V0][0] != 0;
            c.g[V1][0] = lw(c, gp(c, 0x530));
            if !restart {
                c.cycles += 3;
                c.g[V0][0] = 0x12;
                let other = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if other {
                    early = true;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = lw(c, gp(c, 0x530));
                }
            }
        }
        if !early {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                sw(c, p(c.g[S2][0] as u32, 0x530), 0);
            } else {
                c.cycles += 7;
                c.g[V0][0] = lw(c, p(c.g[S2][0] as u32, 0x5558));
                c.g[A1][0] = 8;
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x18)).lh(c);
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x1c));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0028_f3a8)?;
                sw(c, p(c.g[S2][0] as u32, 0x530), 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Front-end widgets.

/// `void cFEStateNetSSXWorld_OnWidgetCreated(cFEState*, void* widget)`
/// (0x1c1e08): compare the widget's name (`widget+0x38`) against the four
/// known names at `0x465a80/90/a8/c0`; the matching one selects the page
/// stored at `widget+0x18` (0..3), otherwise the field is left alone.
pub fn c_fe_state_net_ssx_world_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a80));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
        call_ptr(c, 0x0031_7670, 0x001c_1e2c)?; // StrHash
        let different = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        if !different {
            c.cycles += 2;
            sw(c, p(c.g[S1][0] as u32, 0x18), 0);
            c.cycles += 1;
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a90));
            call_ptr(c, 0x0031_7670, 0x001c_1e48)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[A0][0] = 0x0046_0000;
                c.cycles += 5;
                c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5aa8));
                call_ptr(c, 0x0031_7670, 0x001c_1e64)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x0046_0000;
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x38));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5ac0));
                    call_ptr(c, 0x0031_7670, 0x001c_1e80)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.cycles += 1;
                        c.g[V0][0] = 3;
                        c.cycles += 1;
                        sw(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                        c.cycles += 1;
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    c.cycles += 1;
                    sw(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                    c.cycles += 1;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.cycles += 1;
                sw(c, p(c.g[S1][0] as u32, 0x18), c.g[V0][0]);
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_SendPrivateMessage(Lobby*, char* to, char* message)`
/// (0x25fb80): build a 0x100-byte request tag from `to` (field 0x4a3258) and
/// `message` (field 0x4a3010) with the 0x40004000 flags, then send the request
/// (`0x6d657367`) to the lobby name at `lobby+0x54`.
pub fn lobby_man_send_private_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x130);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x120);
        c.g[A1][0] = 0x100;
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = 0x004a_0000;
        frame.save_ra(c, 0x100);
        c.g[A0][0] = c.g[SP][0];
        Addr(c.g[SP][0] as u32).sb(c, 0);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3258));
        call_ptr(c, 0x003e_c180, 0x0025_fbb4)?; // TagFieldSetString
        c.g[A2][0] = 0x004a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3010));
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003e_c180, 0x0025_fbcc)?;
        c.g[A2][0] = 0x004a_0000;
        c.g[A3][0] = 0x4000_4000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3260));
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003e_bf28, 0x0025_fbe8)?; // TagFieldSetFlags
        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x54));
        c.g[A3][0] = 0x0027_0000;
        c.g[A1][0] = 0x6d65_0000;
        c.g[T0][0] = c.g[S1][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7138));
        c.g[A1][0] |= 0x7367;
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003e_8b10, 0x0025_fc08)?; // LobbyApi_Send
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `int BuddyMan_FindRosterEntryNoCase(BuddyMan* man, char* name)`
/// (0x2624f8): case-insensitive search of the roster list at `man+0xcc`;
/// returns the entry index or -1.
pub fn buddy_man_find_roster_entry_no_case(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        let no_name = c.g[S2][0] == 0;
        frame.save_ra(c, 0);
        if no_name {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S3][0];
            c.g[S0][0] = 0;
            call_ptr(c, 0x0026_2598, 0x0026_2530)?; // BuddyMan_GetRosterCount
            c.g[S1][0] = c.g[V0][0];
            let empty = (c.g[S1][0] as i64) <= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if !empty {
                c.cycles += 1;
                c.g[A0][0] = c.g[S3][0];
                loop {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0026_25c8, 0x0026_2548)?; // BuddyMan_GetRosterEntry
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = lw(c, c.g[S2][0] as u32);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0041_aa88, 0x0026_255c)?; // strcasecmp
                        let same = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[S0][0];
                        if same {
                            break;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S3][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_2540);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break;
                }
            }
        }
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Crowd audio.

/// `int CrowdAudio_AddSource(CrowdAudio* self, void* source)` (0x2a7678): add
/// `source` to the crowd instance at `self+0` (a 0x190-byte source description
/// appended at `+0x1d8`), returning the source bit or -1, then decrement the
/// instance's pending count at `+0x1f4`.
pub fn crowd_audio_add_source(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[T0][0] = lw(c, p(c.g[S0][0] as u32, 0x18));
        if c.g[T0][0] != 0 {
            c.g[V1][0] = lw(c, c.g[S0][0] as u32);
            c.cycles += 14;
            c.g[A3][0] = 0x190;
            c.g[A2][0] = c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x1c));
            c.g[A1][0] = c.g[V1][0];
            c.g[A0][0] = lw(c, p(c.g[V1][0] as u32, 0x64));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = lw(c, c.g[V0][0] as u32);
            c.mult(0, 2, 4, 7);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            call_ptr(c, 0x002a_5d08, 0x002a_76cc)?; // CrowdAudio_InstanceAddSource
            let failed = (c.g[V0][0] as i64) < 0;
            c.g[A1][0] = sx((c.g[V0][0] as u32) << 8);
            if !failed {
                c.cycles += 8;
                c.g[V1][0] = lw(c, c.g[S0][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x1c));
                c.g[A0][0] = lw(c, p(c.g[V1][0] as u32, 0x64));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = lw(c, c.g[V0][0] as u32);
                c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 4;
        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
        c.g[V1][0] = lw(c, p(c.g[A0][0] as u32, 0x1f4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        sw(c, p(c.g[A0][0] as u32, 0x1f4), c.g[V1][0]);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_LoadBankAsync(ScriptEngine* self, int bank, char* path,
/// void* into, int size, int offset)` (0x27b370): store the offset in the
/// bank slot at `self + 0x628 + 16*bank`, start a load (into `into` when given,
/// else a plain async load), then account for a queued bank: bump `+0x620` if
/// the slot got a handle, else clear the slot.
pub fn script_engine_load_bank_async(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x628));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        c.g[A1][0] = c.g[T1][0];
        c.g[A2][0] = c.g[T2][0];
        let plain = c.g[A1][0] == 0;
        sw(c, p(c.g[S0][0] as u32, 4), c.g[T0][0]);
        if !plain {
            c.cycles += 5;
            call_ptr(c, 0x003d_f748, 0x0027_b3ac)?; // AsyncFile_LoadInto
            sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
            sw(c, p(c.g[S0][0] as u32, 0xc), 0);
        } else {
            c.cycles += 5;
            c.g[A1][0] = c.g[A3][0];
            call_ptr(c, 0x003d_f690, 0x0027_b3c0)?; // AsyncFile_Load
            c.g[V1][0] = 1;
            sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
            sw(c, p(c.g[S0][0] as u32, 0xc), c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0027_7f08, 0x0027_b3f8)?; // ScriptBankSlot_Clear
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x620));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            sw(c, p(c.g[S1][0] as u32, 0x620), c.g[V0][0]);
            c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 8));
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0027_7f08, 0x0027_b3f8)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ScriptEngine_ClearObjBindingFilter(ScriptEngine* self, int filter)`
/// (0x27d170): for every binding in the container at `self + 4*filter`,
/// clear the filter slot through `ScriptEngine_ClearBindingFilter`.
pub fn script_engine_clear_obj_binding_filter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[S1][0] = lw(c, c.g[A1][0] as u32);
        c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0xc));
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x30)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x34));
        c.g[V0][0] = sx(c.g[V0][0] as u32);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0027_d1b0)?;
        c.g[S2][0] = c.g[V0][0];
        if (c.g[S2][0] as i64) <= 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0xc));
            loop {
                c.cycles += 12;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x38)).lh(c);
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x3c));
                c.g[V0][0] = sx(c.g[V0][0] as u32);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x0027_d1d8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0027_d558, 0x0027_d1e4)?; // ScriptEngine_ClearBindingFilter
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0xc));
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_d1c0);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break;
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// On-screen keyboard.

/// `int cKeyboardPopup_MoveDownFromTopRow(KeyboardPopup* popup, int arg)`
/// (0x1cbc48): from the top row (state 4) or the row below it (state 5), move
/// down to the first enabled key: if the key below the current one is enabled
/// select it directly, otherwise scan with `StepDownToEnabledKey`. Returns 1
/// when the selection moved.
pub fn c_keyboard_popup_move_down_from_top_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0x140));
        let not_top = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut step = false;
        let mut matched = !not_top;
        if !not_top {
            c.cycles += 5;
            c.g[A1][0] = 0x10;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x001c_e560, 0x001c_bc78)?; // IsKeyEnabled
            let disabled = c.g[V0][0] == 0;
            c.g[V0][0] = 0x10;
            if disabled {
                step = true;
            } else {
                c.cycles += 2;
                sw(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let below = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            matched = !below;
            if !below {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x12;
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001c_e560, 0x001c_bca4)?;
                let disabled = c.g[V0][0] == 0;
                c.g[V0][0] = 0x12;
                if disabled {
                    step = true;
                } else {
                    c.cycles += 2;
                    sw(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
                }
            }
        }
        if step {
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[SP][0] as u32);
            let out_empty = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[S1][0] < 1) as u64;
            if !out_empty {
                c.cycles += 1;
                sw(c, p(c.g[S0][0] as u32, 0x144), c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001c_c190, 0x001c_bccc)?; // StepDownToEnabledKey
        }
        if matched {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int cKeyboardPopup_MoveUpFromBottomRow(KeyboardPopup* popup, int arg)`
/// (0x1cbf20): the mirror of `MoveDownFromTopRow`, from state 0x4c/0x4f.
pub fn c_keyboard_popup_move_up_from_bottom_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x4c;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0x140));
        let not_bottom = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        let mut matched = !not_bottom;
        if !not_bottom {
            c.cycles += 5;
            c.g[A1][0] = 0x42;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x001c_e560, 0x001c_bf50)?;
            let enabled = c.g[V0][0] != 0;
            c.g[V0][0] = 0x42;
            if enabled {
                c.cycles += 2;
                sw(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, c.g[SP][0] as u32);
                c.cycles += 2;
                let out_empty = c.g[V0][0] == 0;
                c.g[V0][0] = (c.g[S1][0] < 1) as u64;
                if !out_empty {
                    c.cycles += 1;
                    sw(c, p(c.g[S0][0] as u32, 0x144), c.g[V0][0]);
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001c_c140, 0x001c_bfa4)?;
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4f;
            let other = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            matched = !other;
            if !other {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x42;
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001c_e560, 0x001c_bf7c)?;
                let enabled = c.g[V0][0] != 0;
                c.g[V0][0] = 0x42;
                if enabled {
                    c.cycles += 2;
                    sw(c, p(c.g[S0][0] as u32, 0x140), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = lw(c, c.g[SP][0] as u32);
                    c.cycles += 2;
                    let out_empty = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S1][0] < 1) as u64;
                    if !out_empty {
                        c.cycles += 1;
                        sw(c, p(c.g[S0][0] as u32, 0x144), c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001c_c140, 0x001c_bfa4)?;
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
            }
        }
        let _ = matched;
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScript process manager.

/// `WScriptProcess* WScriptMan_CreateProcess(WScriptMan* self, char* name,
/// char* code, int flag)` (0x3094c8): allocate and construct a 0x60-byte
/// process, then push it on the mission list (`+0x2bc`) when `flag` is set,
/// else on the generic list (`+0x2b4`).
pub fn wscript_man_create_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x60;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x66f8));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0030_9508)?; // EA_MemAllocNamed
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0030_7598, 0x0030_951c)?; // WScriptProcess_ctor
        let plain = c.g[S2][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !plain {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2bc));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b260, 0x0030_9530)?; // PushFront
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b260, 0x0030_9544)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_RequestMissionEnd(WScriptMan* self, int rider)` (0x30b758)
/// (0x30b658 is the cancel variant): stop playback, clear its saved world
/// state, reset the replay to state 10, force-reset the rider and mark the
/// mission state as cancelled (3) or ended (4).
pub fn wscript_man_request_mission(c: &mut Cpu, state: u64) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = lw(c, gp(c, -0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x84));
        c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x34));
        call_ptr(c, 0x0026_cdf8, 0x0030_b678)?; // PlaybackMan_Stop
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw(c, p(c.g[V1][0] as u32, 0x34));
        call_ptr(c, 0x0026_cbb0, 0x0030_b688)?; // PlaybackMan_ClearWorldStateSaved
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw(c, p(c.g[V1][0] as u32, 0x28));
        call_ptr(c, 0x0026_f228, 0x0030_b698)?; // Replay_Reset
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw(c, p(c.g[V1][0] as u32, 0x28));
        call_ptr(c, 0x0026_f7b8, 0x0030_b6a8)?; // Replay_AdvanceFromState10
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.g[V1][0] = lw(c, c.g[S0][0] as u32);
        c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x28));
        call_ptr(c, 0x0012_35f8, 0x0030_b6c8)?; // Rider_ForceReset
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x2a0));
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = state;
        c.g[V0][0] |= 0xffff;
        sw(c, p(c.g[S0][0] as u32, 0x2a0), c.g[V0][0]);
        sw(c, p(c.g[S0][0] as u32, 0x2ac), c.g[V1][0]);
        sw(c, p(c.g[S0][0] as u32, 0x2b0), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_RequestMissionCancel(WScriptMan* self, int rider)`
/// (0x30b658).
pub fn wscript_man_request_mission_cancel(c: &mut Cpu) {
    wscript_man_request_mission(c, 3)
}

/// `void WScriptMan_RequestMissionEnd(WScriptMan* self, int rider)` (0x30b758).
pub fn wscript_man_request_mission_end(c: &mut Cpu) {
    wscript_man_request_mission(c, 4)
}

/// `void WScriptMan_DeactivateAll(WScriptMan* self)` (0x30b7f8): terminate the
/// current mission process, clear the active fields, move every queued process
/// back to the idle list and reset the event queue.
pub fn wscript_man_deactivate_all(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0030_b7f8 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, frame_at(c, 0x30));
                    c.sq(17, frame_at(c, 0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, frame_at(c, 0x10));
                    c.g[S1][0] = 0xffff_ffff_ffff_0000;
                    c.sd(frame_at(c, 0), c.g[RA][0]);
                    c.g[S1][0] |= 0xffff;
                    c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x2a0));
                    if c.g[A1][0] == c.g[S1][0] {
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
                        pc = 0x0030_b850;
                        continue;
                    }
                    pc = 0x0030_b824;
                    continue;
                }
                0x0030_b824 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
                    call_ptr(c, 0x0030_b388, 0x0030_b82c)?; // FindMissionById
                    if c.g[V0][0] == 0 {
                        sw(c, p(c.g[S0][0] as u32, 0x2a0), c.g[S1][0]);
                        pc = 0x0030_b840;
                        continue;
                    }
                    pc = 0x0030_b834;
                    continue;
                }
                0x0030_b834 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0030_7d78, 0x0030_b83c)?; // WScriptProcess_Terminate
                    sw(c, p(c.g[S0][0] as u32, 0x2a0), c.g[S1][0]);
                    pc = 0x0030_b840;
                    continue;
                }
                0x0030_b840 => {
                    c.cycles += 4;
                    sw(c, p(c.g[S0][0] as u32, 0x2ac), 0);
                    sw(c, p(c.g[S0][0] as u32, 0x2b0), c.g[S1][0]);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
                    pc = 0x0030_b850;
                    continue;
                }
                0x0030_b850 => {
                    c.cycles += 2;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
                    pc = 0x0030_b864;
                    continue;
                }
                0x0030_b858 => {
                    c.cycles += 3;
                    sw(c, c.g[V0][0] as u32, 0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
                    call_ptr(c, 0x0030_b260, 0x0030_b864)?; // PushFront
                    pc = 0x0030_b864;
                    continue;
                }
                0x0030_b864 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0030_b208, 0x0030_b86c)?; // PopFront
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            c.pc = 0x0030_b858;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0030_b858;
                        continue;
                    }
                    pc = 0x0030_b874;
                    continue;
                }
                0x0030_b874 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0030_c790, 0x0030_b87c)?; // EventQueueReset
                    c.lq(16, frame_at(c, 0x30));
                    c.lq(17, frame_at(c, 0x20));
                    c.lq(18, frame_at(c, 0x10));
                    c.g[RA][0] = c.ld(frame_at(c, 0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030_b7f8, 0x0030_b894);
                    return Ok(());
                }
            }
        }
    });
}

/// `void ScriptEngine_ActivateBinding(ScriptEngine* self, Binding* binding,
/// int value)` (0x27d400): run the binding's start method; when it returns 0,
/// also run the value table entry's start, then store the value and the
/// success flag in the binding's slot.
pub fn script_engine_activate_binding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x14;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 4));
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x2a8));
        c.mult(0, 3, 3, 2);
        c.g[A0][0] = Addr(p(c.g[A1][0] as u32, 0x20)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x24));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx(c.g[V0][0] as u32);
        c.g[A1][0] = lw(c, p(c.g[S1][0] as u32, 4));
        call_ptr(c, c.g[V0][0] as u32, 0x0027_d448)?;
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !nonzero {
            c.cycles += 11;
            c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 4));
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = lw(c, c.g[V0][0] as u32);
            c.g[A2][0] = c.g[V0][0];
            c.g[V1][0] = lw(c, p(c.g[A2][0] as u32, 0xc));
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x20)).lh(c);
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x24));
            c.g[V0][0] = sx(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0027_d478)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 8;
        sw(c, p(c.g[S1][0] as u32, 8), c.g[S2][0]);
        sw(c, p(c.g[S1][0] as u32, 0xc), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
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

// ---------------------------------------------------------------------------
// Sound platform.

/// `int PATH_PlatSetAuxSend(int platform, int send, int level)` (0x2af4e0):
/// set the auxiliary send on the platform's sound stream (or its channel when
/// the platform has a direct voice), returning the previous value.
pub fn path_plat_set_aux_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x004d_0000;
        frame.save(c, S3, 0x30);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save(c, S2, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = lw(c, c.g[A0][0] as u32);
        let absent = c.g[S0][0] == 0;
        c.g[S1][0] = 0xffff_ffff_ffff_fff8;
        if !absent {
            c.cycles += 5;
            call_ptr(c, 0x003b_58a0, 0x002a_f524)?; // SND_Lock
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x64)).lb(c);
            let direct = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S3][0];
            if direct {
                c.cycles += 5;
                c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x003b_88c8, 0x002a_f53c)?; // SND_SetAuxSend
            } else {
                c.cycles += 3;
                c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x003b_8050, 0x002a_f550)?; // SNDSTRM_SetAuxSend
            }
            c.cycles += 3;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x003b_58d8, 0x002a_f558)?; // SND_Unlock
            sw(c, p(c.g[S0][0] as u32, 0x10), c.g[S2][0]);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `void cFEStateMPCircuit_OnActivate(cFEState*, void* arg)` (0x1a3308): find
/// the "circuit" widget on the state's screen, and if it is the selected list
/// box play its frame from the selected item's name; then run the empty hook
/// and the base `UIState_OnActivate`.
pub fn c_fe_state_mp_circuit_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x004a_0000;
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19e8));
        call_ptr(c, 0x0031_7670, 0x001a_332c)?; // StrHash
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001a_3338)?; // UIScreen_FindObject
        if c.g[V0][0] != 0 {
            c.cycles += 11;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_a708, 0x001a_3348)?; // UIListBox_GetSelectedData
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0031_7670, 0x001a_3350)?; // StrHash
            c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001a_335c)?; // UIScreen_LookupValue
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            let no_frame = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] &= 0xffff;
            if !no_frame {
                c.cycles += 3;
                c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x40));
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x001a_3378)?; // UIScreen_StartThread
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a_34a8, 0x001a_3380)?; // cFEStateMPCircuit_EmptyHook
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e4c0, 0x001a_338c)?; // UIState_OnActivate
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void NetFEState_OnMemCardLoginPopupEvent(NetFEState*, void* event, int id)`
/// (0x1a9438): on the login-popup result event (0x16), create a login state
/// through the vtable at `self+8`, switch to it and forward the event;
/// otherwise forward the event to the base handler.
pub fn net_fe_state_on_mem_card_login_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let other = c.g[A2][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if other {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_93d0, 0x001a_94c0)?; // NetFEState_OnChildEventDefault
        } else {
            c.cycles += 23;
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x100)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x104));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001a_946c)?;
            c.g[A1][0] = 0x0046_0000;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ca0));
            c.g[A0][0] = 0x6d4;
            call_ptr(c, 0x0031_7d70, 0x001a_9484)?; // EA_MemAllocNamed
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x10));
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_ee30, 0x001a_9490)?; // cFEStateLogin_ctor
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_8a40, 0x001a_949c)?; // NetFEState_GotoNextState
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x16;
            call_ptr(c, 0x001a_93d0, 0x001a_94ac)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cFEStateOptions_OnWidgetNotify(cFEState*, void* widget, int event,
/// int index)` (0x188c68): map the widget events 6/7 onto the page's flag
/// word, or return 0x100/0x101 for the other ids.
pub fn c_fe_state_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S1][0] < 0xa) as u64;
        let big = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut done = false;
        if !big {
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < 8) as u64;
            let low = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if !low {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
                done = true;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 6;
        }
        if !done {
            c.cycles += 2;
            let not_six = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 7;
            let mut hit = false;
            if !not_six {
                c.cycles += 9;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                c.g[A1][0] = c.g[A3][0];
                call_ptr(c, 0x0039_7870, 0x0018_8cb8)?; // UIList_GetAt
                c.g[A0][0] = 0x004a_0000;
                c.g[S0][0] = lw(c, p(c.g[V0][0] as u32, 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
                call_ptr(c, 0x0031_7670, 0x0018_8cc8)?; // StrHash
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[A0][0] = lw(c, p(c.g[S2][0] as u32, 0x4c));
                    hit = true;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 7;
                }
            }
            if hit {
                c.cycles += 3;
                c.g[V1][0] = 0x101;
                c.g[V0][0] = 1;
                if lw(c, c.g[A0][0] as u32) != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
            } else {
                c.cycles += 2;
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x101;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = lw(c, p(c.g[S2][0] as u32, 0x4c));
                    c.cycles += 3;
                    c.g[V1][0] = 0x101;
                    c.g[V0][0] = 1;
                    if lw(c, c.g[A0][0] as u32) != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                }
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

/// `void OVScreenInGameError_UpdateOpponentDisconnectedText(Screen* self)`
/// (0x1fb218): find the disconnected-text object on the screen, set its text
/// id to the opponent-disconnected string and show the continue button.
pub fn ov_screen_in_game_error_update_opponent_disconnected_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x40));
        let no_screen = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0047_0000;
        if !no_screen {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
            call_ptr(c, 0x0031_7670, 0x001f_b240)?; // StrHash
            c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_b24c)?; // UIScreen_FindObject
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[V0][0] = lw(c, 8);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0xa0));
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = 0x0047_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x478));
                    call_ptr(c, 0x0031_7670, 0x001f_b270)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a_1000, 0x001f_b27c)?; // UIText_SetTextId
                    c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
                }
            }
            c.cycles += 8;
            c.g[A1][0] = lw(c, p(c.g[S1][0] as u32, 0xa0));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001f_b294)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x001f_b378, 0x001f_b2a0)?; // ShowContinueButton
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int BERewardInt_GetNextPointsThreshold(BEReward* self, int a, int tier)`
/// (0x158be0): return the points threshold of the first tier (up to three)
/// that the reward has points in and that is better than `a`.
pub fn be_reward_int_get_next_points_threshold(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4fb8));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        loop {
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S0][0];
            call_ptr(c, 0x0015_8af0, 0x0015_8c2c)?; // HasPointsTier
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            call_ptr(c, 0x0015_6a60, 0x0015_8c3c)?; // IsBetterPlace
            let better = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            if !better {
                c.cycles += 2;
                c.g[V0][0] = lw(c, c.g[S2][0] as u32);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0015_8c18);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break;
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
    });
}

// ---------------------------------------------------------------------------
// Rider metrics.

/// `void RiderMetrix_ChainUberGrind(RiderMetrix* self, int points)` (0x119958):
/// count the chained uber, finish the trick with no extra bonus and reset it,
/// then record the points as the trick's value.
pub fn rider_metrix_chain_uber_grind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[A1][0] = 0;
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        c.g[T0][0] = 0;
        c.lwc1(21, p(c.g[S0][0] as u32, 0x30));
        c.g[T1][0] = 0;
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0x54));
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.lwc1(22, p(c.g[S0][0] as u32, 0x24));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        sw(c, p(c.g[S0][0] as u32, 0x54), c.g[V1][0]);
        sw(c, p(c.g[S0][0] as u32, 0x74), c.g[V0][0]);
        call_ptr(c, 0x0011_a228, 0x0011_99b4)?; // RiderMetrix_FinishTrick
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_99c0)?; // RiderMetrix_ResetTrick
        c.swc1(21, p(c.g[S0][0] as u32, 0x30));
        c.mov_s(0, 20);
        sw(c, p(c.g[S0][0] as u32, 0x20), c.g[S1][0]);
        c.swc1(22, p(c.g[S0][0] as u32, 0x24));
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderMetrix_FinishBoardPress(RiderMetrix* self)` (0x119a38): finish
/// the board press trick (with a zero bonus unless the press value at
/// `+0x30` is negative) and reset it.
pub fn rider_metrix_finish_board_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.mtc1(0, 0);
        frame.save(c, S0, 0x20);
        c.g[T1][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 20, 0x30);
        frame.save_fpr(c, 21, 0x38);
        c.lwc1(21, p(c.g[S0][0] as u32, 0x30));
        c.lwc1(22, p(c.g[S0][0] as u32, 0x24));
        c.c_le_s(0, 21);
        let not_negative = c.fcc();
        c.g[S1][0] = lw(c, p(c.g[S0][0] as u32, 0x20));
        if !not_negative {
            c.cycles += 1;
            c.g[T1][0] = 0;
        }
        c.cycles += 22;
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0011_a228, 0x0011_9a94)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_7838, 0x0011_9aa0)?;
        c.swc1(21, p(c.g[S0][0] as u32, 0x30));
        c.mov_s(0, 20);
        sw(c, p(c.g[S0][0] as u32, 0x20), c.g[S1][0]);
        c.swc1(22, p(c.g[S0][0] as u32, 0x24));
        sw(c, p(c.g[S0][0] as u32, 0xc), c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Transitions.

/// `void cTransitionMan_dtor(TransitionMan* self, int flags)` (0x2e42d0): stop
/// the transition, release the pending effect through the global vtable, then
/// unregister and destroy the embedded script object and the game process.
pub fn c_transition_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 0x0049_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f00));
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7ea8));
        sw(c, p(c.g[S0][0] as u32, 0x1c), c.g[V0][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sw(p(c.g[S0][0] as u32, 0xc), c.g[V1][0]);
        call_ptr(c, 0x002e_4578, 0x002e_4308)?; // cTransitionMan_Stop
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x7c));
        let no_effect = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        if !no_effect {
            c.cycles += 8;
            c.g[A2][0] = lw(c, gp(c, 0x2a90));
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x68));
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x1e0)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x1e4));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x002e_4330)?;
            sw(c, p(c.g[S0][0] as u32, 0x7c), 0);
        }
        c.cycles += 14;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_32d8, 0x002e_433c)?; // ScriptObj_Unregister
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_3228, 0x002e_4348)?; // ScriptObj_dtor
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0035_46c8, 0x002e_4354)?; // GameProcess_dtor
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Board equipment.

/// `void BEBoltInt_RestoreAllEquipFlags(void)` (0x14af10): for the three board
/// slots and their ten equipment entries, restore the "owned" flag bit 0x10
/// on entries whose bit 4 was set, and clear it elsewhere.
pub fn be_bolt_int_restore_all_equip_flags(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x004a_0000;
    c.g[A0][0] = 0;
    c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[T5][0] = 0x9b50;
    c.g[T3][0] = 0xf88;
    c.mult(0, 9, 4, 13);
    loop {
        c.cycles += 4;
        c.g[V1][0] = 0;
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.mult(0, 4, 3, 11);
        loop {
            c.cycles += 6;
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x28c));
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            if !empty {
                c.cycles += 2;
                c.g[A2][0] = c.g[V1][0];
                c.g[T0][0] = 0xffff_ffff_ffff_ffef;
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 2)).lhu(c);
                    c.g[A1][0] = c.g[V0][0] | 0x10;
                    c.g[V1][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let clear = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    if clear {
                        c.cycles += 1;
                        c.sh(p(c.g[A0][0] as u32, 2), c.g[V1][0]);
                    } else {
                        c.cycles += 2;
                        c.sh(p(c.g[A0][0] as u32, 2), c.g[A1][0]);
                    }
                    c.cycles += 2;
                    let more = c.g[A2][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_af58);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A3][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.mult(0, 4, 3, 11);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af38);
                }
                continue;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[T2][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            let more_outer = c.g[V0][0] != 0;
            c.mult(0, 9, 4, 13);
            if more_outer {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af28);
                }
                continue;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// Strings.

/// `cBXString& cBXString::operator=(const cBXString& other)` (0x318448):
/// share the other string when both headers are writable, otherwise copy it.
pub fn cbxstring_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
        let same = c.g[V1][0] == c.g[A0][0];
        c.g[V0][0] = c.g[S1][0];
        if same {
            c.cycles += 5;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xffff_fff4));
        let mut copy = false;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0xffff_fff4));
            c.cycles += 2;
            if (c.g[V0][0] as i64) < 0 {
                c.cycles += 1;
                c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                copy = true;
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x0048_dcd4;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                copy = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0xffff_fff4));
                c.cycles += 2;
                copy = (c.g[V0][0] as i64) < 0;
            }
        }
        if copy {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw(c, p(c.g[A2][0] as u32, 0xffff_fff8));
            call_ptr(c, 0x0031_83d8, 0x0031_84a8)?; // AssignCopy
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_80b8, 0x0031_84b8)?; // Release
            c.g[V1][0] = lw(c, c.g[S0][0] as u32);
            sw(c, c.g[S1][0] as u32, c.g[V1][0]);
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xffff_fff4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            sw(c, p(c.g[V1][0] as u32, 0xffff_fff4), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cBXString cBXString::operator+(const char* rhs)` (0x318630): concatenate
/// `rhs` after this string and return the result object.
pub fn cbxstring_op_plus_str_cstr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = lw(c, gp(c, 0xda0));
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = lw(c, c.g[S1][0] as u32);
        sw(c, c.g[SP][0] as u32, c.g[V0][0]);
        let nil = c.g[S0][0] == 0;
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(8));
        let mut len = 0u64;
        if !nil {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6810, 0x0031_8670)?; // strlen
            len = c.g[V0][0];
        } else {
            c.cycles += 2;
            len = 0;
        }
        c.g[A2][0] = lw(c, c.g[S1][0] as u32);
        c.cycles += 19;
        c.g[T0][0] = c.g[S0][0];
        c.g[A1][0] = lw(c, c.g[S2][0] as u32);
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = len;
        call_ptr(c, 0x0031_8540, 0x0031_8694)?; // ConcatCopy
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0031_7fe8, 0x0031_86a0)?; // ctor_copy
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0031_86ac)?; // dtor
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void TagFieldSetFlags(Tag tag, int flags, void* format, int value)`
/// (0x3ebf28): copy `tag` to a stack buffer, append the value's base-32 digits
/// from the table at `0x4965c0` and set the flags field with the result.
pub fn tag_field_set_flags(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x150);
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x120);
        c.g[S0][0] = c.g[A3][0];
        call_ptr(c, 0x003e_b198, 0x003e_bf50)?; // TagFieldCopyTag
        c.g[A1][0] = 0x0049_0000;
        c.g[V1][0] = c.g[V0][0];
        let no_value = c.g[S0][0] == 0;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x65c0));
        // `zero` means the common tail (A0 = S1) runs; `add_one` is its +1.
        let mut zero = no_value;
        let mut add_one = no_value;
        if !no_value {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
            if c.g[V0][0] == 0 {
                zero = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0] & 1;
                loop {
                    c.cycles += 2;
                    let bit_clear = c.g[V0][0] == 0;
                    c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
                    if !bit_clear {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                        c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    }
                    c.cycles += 2;
                    let end = c.g[S0][0] == 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    if end {
                        zero = true;
                        add_one = true;
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                    let cont = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0] & 1;
                    if !cont {
                        zero = true;
                        add_one = true;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_bf70);
                    }
                }
            }
        }
        if zero {
            if add_one {
                c.cycles += 1;
            }
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 10;
        c.g[A1][0] = c.g[S2][0];
        c.sb(c.g[V1][0] as u32, 0);
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003e_b1e0, 0x003e_bfac)?; // TagFieldSetRaw
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Text scroll.

/// `int cUITextScroll_GrowLines(cUITextScroll* self)` (0x3a1958): grow the
/// line-pointer array (`+0xbc`, count `+0xc8`) by one, keep the old entries,
/// and return the old count.
pub fn c_ui_text_scroll_grow_lines(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40a8));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x100;
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S3][0] = Addr(p(c.g[S1][0] as u32, 0xc8)).lhu(c);
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        c.sh(p(c.g[S1][0] as u32, 0xc8), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0] & 0xffff;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        call_ptr(c, 0x0031_7e30, 0x003a_199c)?; // operator new
        c.g[A1][0] = lw(c, p(c.g[S1][0] as u32, 0xbc));
        let none = c.g[A1][0] == 0;
        c.g[S2][0] = c.g[V0][0];
        if !none {
            c.cycles += 8;
            c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_605c, 0x003a_19b8)?; // memcpy
            c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0xbc));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            let gone = c.g[A0][0] == 0;
            sw(c, c.g[S0][0] as u32, 0);
            if !gone {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x003a_19d0)?; // EA_MemFree
            }
        }
        c.cycles += 9;
        sw(c, p(c.g[S1][0] as u32, 0xbc), c.g[S2][0]);
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Luno table.

/// `void LunoTable_dtor(LunoTable* self, int flags)` (0x224d00): call the
/// destructor of every entry in the block at `+0x10` (0x20 bytes each, count at
/// block-0x10), free it, and delete the table when `flags & 1`.
pub fn luno_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
        let empty = c.g[V1][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xffff_fff0));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if c.g[V1][0] == c.g[S0][0] {
                c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                loop {
                    c.cycles += 9;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x1c));
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 8)).lh(c);
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                    c.g[V1][0] = sx(c.g[V1][0] as u32);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0022_4d58)?;
                    c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_4d40);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0022_4d70)?; // EA_MemFree
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0022_4d84)?; // __builtin_delete
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Fences.

/// `void Fence_v40(Fence* self, Vec4* impulse)` (0x34d650): when the fence is
/// below full strength, add a scaled impulse to it and count the hit (up to
/// four in each of two counters).
pub fn fence_v40(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.sq(16, frame_at(c, 0x10));
    c.sd(frame_at(c, 0), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.lwc1(0, p(c.g[S0][0] as u32, 0x30));
    c.c_le_s(1, 0);
    if c.fcc() {
        c.lq(16, frame_at(c, 0x10));
    } else {
        c.cycles += 17;
        c.lwc1(1, gp(c, -0x2ab8));
        c.lqc2(3, p(c.g[A1][0] as u32, 0x20));
        c.lqc2(5, p(c.g[A1][0] as u32, 0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 12);
        c.lwc1(0, p(c.g[A1][0] as u32, 0x30));
        c.mul_s(12, 12, 1);
        c.mul_s(12, 12, 0);
        if call_ptr(c, 0x0034_d6f0, 0x0034_d6b4).is_err() { return; } // Fence_AddImpulse
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x38));
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x3c));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            sw(c, p(c.g[S0][0] as u32, 0x38), c.g[V0][0]);
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x3c));
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.lq(16, frame_at(c, 0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            sw(c, p(c.g[S0][0] as u32, 0x3c), c.g[V0][0]);
            c.lq(16, frame_at(c, 0x10));
        }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(frame_at(c, 0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// Memory card front end.

/// `void cFEStateMemCard_Init(cFEStateMemCard* self)` (0x186b50): create the
/// screen named by the state's engine string, start it, then find the "text"
/// object and call its setup method.
pub fn c_fe_state_mem_card_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2390));
        c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
        call_ptr(c, 0x0031_7670, 0x0018_6b74)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_6b88)?; // UIEngine_CreateScreen
        let failed = c.g[V0][0] == 0;
        sw(c, p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_6ba0)?; // UIScreen_StartThread
        }
        c.cycles += 19;
        c.g[A0][0] = lw(c, gp(c, -0x848));
        call_ptr(c, 0x0022_7f80, 0x0018_6ba8)?; // MCManager_Get
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2380));
        call_ptr(c, 0x0031_7670, 0x0018_6bb4)?;
        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_6bc0)?; // UIScreen_FindObject
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 8));
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x48)).lh(c);
        c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, 0x4c));
        c.g[A2][0] = sx(c.g[A2][0] as u32);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[A2][0] as u32, 0x0018_6bd8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// World.

/// `void World_dtor(World* self, int flags)` (0x3a6740): stop the global game
/// module, destroy the world cache, script cache and painter, free the sphere
/// tree cache and delete the world when `flags & 1`.
pub fn world_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = lw(c, gp(c, 0x2a90));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x358)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x35c));
        c.g[V1][0] = sx(c.g[V1][0] as u32);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x003a_6770)?;
        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_7e38, 0x003a_6784)?; // cWorldCache_dtor
        }
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 4));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x003a_c8f0, 0x003a_6798)?; // cWScriptCache_dtor
        }
        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002b_b140, 0x003a_67ac)?; // cWorldPainterMan_dtor
        }
        c.cycles += 5;
        call_ptr(c, 0x0032_de20, 0x003a_67b4)?; // World_FreeSphereTreeCache
        c.g[V0][0] = c.g[S1][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        sw(c, gp(c, 0x16c8), 0);
        if !no_delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x003a_67c8)?; // __builtin_delete
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// HUD and pause screens.

/// `void cOVStateHUD1P_ctor(cOVStateHUD1P* self)` (0x1e9a30): construct the
/// base state and its two HUD meters at `+0x50`/`+0xdc`, and default the
/// display fields.
pub fn c_ov_state_hud_1p_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48));
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;
        call_ptr(c, 0x0039_e2a0, 0x001e_9a58)?; // UIState_ctor
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 0x0047_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4160));
        sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x0020_eaa0, 0x001e_9a74)?; // HUDMeter_ctor
            let more = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001e_9a68);
                }
                continue;
            }
            break;
        }
        c.cycles += 20;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        sw(c, p(c.g[S0][0] as u32, 0x474), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.swc1(0, p(c.g[S0][0] as u32, 0x48c));
        sw(c, p(c.g[S0][0] as u32, 0x3d4), 0);
        sw(c, p(c.g[S0][0] as u32, 0x3d8), 0);
        c.swc1(0, p(c.g[S0][0] as u32, 0x478));
        c.swc1(0, p(c.g[S0][0] as u32, 0x47c));
        sw(c, p(c.g[S0][0] as u32, 0x480), 0);
        sw(c, p(c.g[S0][0] as u32, 0x484), 0);
        c.swc1(0, p(c.g[S0][0] as u32, 0x488));
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenPauseFreeride_InitMode6(OVScreenPauseFreeride* self)`
/// (0x1f90e8): pick the item table (two when a restart is possible), copy it
/// into `+0x9c`, set the page to 6 and run the screen init.
pub fn ov_screen_pause_freeride_init_mode_6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x0048_0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e40));
        sw(c, p(c.g[S0][0] as u32, 0xbc), c.g[V0][0]);
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
        call_ptr(c, 0x0023_1ab8, 0x001f_9118)?; // GameModule_CanRestartFromPause
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xbc));
        } else {
            c.cycles += 4;
            c.g[V0][0] = 2;
            c.g[S1][0] = sx(gp(c, -0x888));
            sw(c, p(c.g[S0][0] as u32, 0xbc), c.g[V0][0]);
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xbc));
        }
        c.cycles += 2;
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[V1][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            loop {
                c.cycles += 8;
                c.g[V0][0] = lw(c, c.g[V1][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                sw(c, c.g[A0][0] as u32, c.g[V0][0]);
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xbc));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_9140);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        c.g[V0][0] = 6;
        c.g[A0][0] = c.g[S0][0];
        sw(c, p(c.g[S0][0] as u32, 0x110), c.g[V0][0]);
        call_ptr(c, 0x001f_8168, 0x001f_9170)?; // OVScreenPauseFreeride_Init
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Script command queue.

/// `int ScriptCmdQueue_Pop(ScriptCmdQueue* self, ScriptCmd* out)` (0x283dc0):
/// pop the record at `+0x8c` into `out`, advance the ring index and return 1;
/// 0 when the queue is empty.
pub fn script_cmd_queue_pop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0028_3da0, 0x0028_3ddc)?; // IsEmpty
        let empty = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            let no_out = c.g[S1][0] == 0;
            sw(c, p(c.g[S0][0] as u32, 0x94), 0);
            if !no_out {
                c.cycles += 18;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x8c));
                c.g[V1][0] = 0x1c;
                c.mult(0, 4, 2, 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.ldl(5, p(c.g[V0][0] as u32, 7));
                c.ldr(5, c.g[V0][0] as u32);
                c.ldl(6, p(c.g[V0][0] as u32, 0xf));
                c.ldr(6, p(c.g[V0][0] as u32, 8));
                c.ldl(3, p(c.g[V0][0] as u32, 0x17));
                c.ldr(3, p(c.g[V0][0] as u32, 0x10));
                c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x18));
                c.sdl(5, p(c.g[S1][0] as u32, 7));
                c.sdr(5, c.g[S1][0] as u32);
                c.sdl(6, p(c.g[S1][0] as u32, 0xf));
                c.sdr(6, p(c.g[S1][0] as u32, 8));
                c.sdl(3, p(c.g[S1][0] as u32, 0x17));
                c.sdr(3, p(c.g[S1][0] as u32, 0x10));
                sw(c, p(c.g[S1][0] as u32, 0x18), c.g[A0][0]);
            }
            c.cycles += 5;
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x8c));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0028_3c60, 0x0028_3e40)?; // NextIndex
            sw(c, p(c.g[S0][0] as u32, 0x8c), c.g[V0][0]);
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Rider wipeout.

/// `void RiderWipeoutState_EnableRootPoseOverride(Rider* rider, Vec4* pose,
/// Vec4* offset, Vec4* root)` (0x136d40): store the pose/offset/root and mark
/// the model's root pose overridden.
pub fn rider_wipeout_state_enable_root_pose_override(c: &mut Cpu) {
    c.cycles += 16;
    c.g[T1][0] = c.g[A0][0];
    c.lq(3, c.g[A3][0] as u32);
    c.g[A0][0] = lw(c, p(c.g[T1][0] as u32, 0x40));
    c.g[T0][0] = 1;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.sq(3, p(c.g[A0][0] as u32, 0x130));
    c.lq(2, p(c.g[A3][0] as u32, 0x10));
    c.sq(2, p(c.g[A0][0] as u32, 0x140));
    c.g[V1][0] = lw(c, p(c.g[T1][0] as u32, 0x40));
    sw(c, p(c.g[V1][0] as u32, 0x150), c.g[T0][0]);
    c.lq(4, c.g[A2][0] as u32);
    c.g[V1][0] = lw(c, c.g[T1][0] as u32);
    c.sq(4, p(c.g[T1][0] as u32, 0x20));
    c.lq(2, c.g[A1][0] as u32);
    let no_override = c.g[V1][0] != 0;
    c.sq(2, p(c.g[T1][0] as u32, 0x10));
    if !no_override {
        c.cycles += 21;
        c.g[V0][0] = lw(c, p(c.g[T1][0] as u32, 0x40));
        c.lqc2(3, p(c.g[V0][0] as u32, 0x370));
        c.lqc2(5, c.g[A1][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.mfc1(4, 0);
        c.lqc2(4, p(c.g[V0][0] as u32, 0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame_at(c, 0x10));
        c.lq(3, frame_at(c, 0x10));
        c.sq(3, frame_at(c, 0));
        c.lqc2(3, p(c.g[T1][0] as u32, 0x10));
        c.lqc2(4, frame_at(c, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(c.g[T1][0] as u32, 0x10));
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// BE library.

/// `void BELibrary_LoadState(BELibrary* library, void* request)` (0x14dfa8):
/// run the request's start method, then load the 14 interfaces through
/// `BELibrary_GetInterface`, calling each interface's load callback with the
/// request.
pub fn be_library_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 4;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = sx(gp(c, -0x1efc));
        c.g[V0][0] = lw(c, c.g[S2][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x14));
        c.g[V1][0] = sx(c.g[V1][0] as u32);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x0014_dfe4)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0014_dd58, 0x0014_dff0)?; // GetInterface
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            let missing = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            if !missing {
                c.cycles += 8;
                call_ptr(c, 0x0014_dd58, 0x0014_e008)?; // GetInterface
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x20)).lh(c);
                c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, 0x24));
                c.g[A2][0] = sx(c.g[A2][0] as u32);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[A2][0] as u32, 0x0014_e020)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 14) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_dfe8);
                }
                continue;
            }
            break;
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

// ---------------------------------------------------------------------------
// Read-message popup.

/// `void cFEStateReadMessagePopup_AddSentMessage(cFEState*, char* text)`
/// (0x1a7ed8): build a 16-byte message (sender from `gp-0xc8`, the text and a
/// timestamp string) and add it to the popup's message ring.
pub fn c_fe_state_read_message_popup_add_sent_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, S0, 0x40);
        c.g[A0][0] = 0x10;
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a08));
        call_ptr(c, 0x0031_7d70, 0x001a_7f10)?; // EA_MemAllocNamed
        c.g[A0][0] = lw(c, gp(c, -0xc8));
        c.g[V1][0] = 0x0047_0000;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2740));
        c.g[A1][0] = lw(c, p(c.g[A0][0] as u32, 0x80));
        c.g[S0][0] = 0x004a_0000;
        sw(c, p(c.g[S1][0] as u32, 0xc), c.g[V1][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_82d8, 0x001a_7f38)?; // cBXString_ctor_cstr
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        call_ptr(c, 0x0031_82d8, 0x001a_7f44)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        call_ptr(c, 0x0031_82d8, 0x001a_7f50)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001a_7dd8, 0x001a_7f5c)?; // AddMessage
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Reward preview.

/// `void cFEStatePreviewReward_dtor(cFEStatePreviewReward* self, int flags)`
/// (0x1d1b68): release the previewed bolt file when one is set, reset the two
/// handles and run the base state destructor.
pub fn c_fe_state_preview_reward_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6608));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0x70));
        let no_bolt = (c.g[V1][0] as i64) < 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_bolt {
            c.cycles += 7;
            c.g[A2][0] = lw(c, gp(c, -0x854));
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x190)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001d_1bb0)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw(c, p(c.g[S0][0] as u32, 0x6c));
        let no_rider = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = lw(c, gp(c, -0x848));
        if !no_rider {
            c.cycles += 4;
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x64));
            c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x7c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            call_ptr(c, 0x0019_dc20, 0x001d_1bcc)?; // BoltFile_Release
        }
        c.cycles += 14;
        c.g[V0][0] = 0x0047_0000;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        sw(c, p(c.g[S0][0] as u32, 0x70), c.g[V1][0]);
        sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        sw(c, p(c.g[S0][0] as u32, 0x6c), c.g[V1][0]);
        call_ptr(c, 0x0039_e390, 0x001d_1bf0)?; // UIState_dtor
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn clear(c: &mut Cpu, a: u32, len: u32) {
        for i in 0..len / 4 {
            c.bus.write32(a + 4 * i, 0);
        }
    }

    const BE: u32 = DATA + 0x10_0000;
    const WORLD: u32 = DATA + 0x11_0000;
    const GAME: u32 = WORLD + 0x1000;

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = GAME;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals(c);
        r
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for the
    /// paths that allocate.
    fn heap(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0x0012_3450;
        for (i, a) in [MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000]
            .into_iter()
            .enumerate()
        {
            c.g[[A0, A1, A2][i]][0] = a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_ff60);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus four
    /// free nodes per size class.
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
        let nodes = DATA + 0x80_0000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x40_0000 + 0x4_0000 * i as u32;
            let base = nodes + 0x4000 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

    // ------------------------------------------------------------------ UI

    #[test]
    fn ui_list_box_clear_matches() {
        check(ssx3_ee::s13::f_0039a670__generated, ui_list_box_clear, 24, &|c, seed| {
            let mut r = base(c, seed, 0x0039_a670);
            let list_box = DATA + 0x100;
            clear(c, list_box, 0x340);
            c.g[A0][0] = sx(list_box);
            if seed % 3 == 0 {
                // Owned storage: bit 0x10 set, no data block to free.
                w(c, list_box + 0x74, 0x10);
                w(c, list_box + 0x9c, 0);
                w(c, list_box + 0x318, (seed % 7) as u32);
            } else {
                let count = seed % 5;
                w(c, list_box + 0x74, r.next_u32() & !0x10);
                w(c, list_box + 0x318, count);
                for i in 0..count {
                    // Null item data: the clear must not call the allocator.
                    w(c, list_box + 0x9c + 0x14 * i, 0);
                }
            }
            w(c, list_box + 0x319, r.next_u32() as u32);
        });
    }

    // ------------------------------------------------------------ debug menu

    #[test]
    fn debug_menu_prev_enabled_item_matches() {
        check(ssx3_ee::s02::f_0017a0b8__generated, debug_menu_prev_enabled_item, 24, &|c, seed| {
            let mut r = base(c, seed, 0x0017_a0b8);
            let menu = DATA + 0x100;
            let n = 1 + (seed % 6);
            clear(c, menu, 0x130);
            w(c, menu, n);
            w(c, menu + 0x8808, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
            c.g[A0][0] = sx(menu);
            c.g[A1][0] = (seed % n) as u64;
            // The first candidate: sometimes enabled (returns at once), else
            // leave the wrap-around scan one enabled entry so it terminates.
            w(c, menu + 0x118, if seed % 2 == 0 { 1 } else { 0 });
            let hit = (seed % n) as u32;
            for i in 0..n {
                w(c, menu + 0x8c + 4 * i, (i == hit) as u32);
            }
        });
    }

    // ----------------------------------------------------------- world light

    #[test]
    fn world_light_man_evict_oldest_instance_matches() {
        check(
            ssx3_ee::s09::f_002f59d0__generated,
            world_light_man_evict_oldest_instance,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x002f_59d0);
                let man = DATA + 0x100;
                let scratch = DATA + 0x9000;
                clear(c, man + 0x210, 0x20 * 0x180);
                clear(c, scratch, 0x400);
                c.g[A0][0] = sx(man);
                c.g[A1][0] = (seed % 64) as u64;
                for i in 0..32u32 {
                    let node = man + 0x210 + 0x180 * i;
                    w(c, node, r.next_u32());
                    w(c, node + 4, if seed % 4 == 0 { 0x1000 } else { r.next_u32() % 64 });
                    let prev = match seed % 3 {
                        0 => 0,
                        1 => scratch,
                        _ => node.wrapping_sub(0x180),
                    };
                    let next = if seed % 2 == 0 { 0 } else { scratch + 4 };
                    w(c, node + 0x170, prev);
                    w(c, node + 0x174, next);
                }
            },
        );
    }

    // ----------------------------------------------------------- rider model

    #[test]
    fn rider_model_show_part_matches() {
        check(ssx3_ee::s10::f_0030e9e0__generated, rider_model_show_part, 24, &|c, seed| {
            let mut r = base(c, seed, 0x0030_e9e0);
            let model = DATA + 0x100;
            let table = DATA + 0x2000;
            let parts = DATA + 0x3000;
            clear(c, model, 0x160);
            clear(c, table, 0x80);
            clear(c, parts, 0x400);
            let part = (seed % 8) as u32;
            w(c, model + 0x14, if seed % 6 == 0 { part } else { 8 });
            w(c, model + 0x1c, table);
            w(c, model + 0xc, parts);
            for i in 0..8 {
                w(c, table + 4 * i, if (seed as u32 + i) % 4 == 0 { 0xffff_ffff } else { i });
            }
            w(c, model + 0x150, r.next_u32());
            w(c, model + 0x154, r.next_u32());
            c.g[A0][0] = sx(model);
            c.g[A1][0] = part as u64;
            c.g[A2][0] = if seed % 3 == 0 { 0xffff_ffff_ffff_ffff } else { (seed % 4) as u64 };
        });
    }

    // ------------------------------------------------------------- VU0 unit

    #[test]
    fn vu0_set_signed_unit_vector_matches() {
        check(ssx3_ee::s10::f_0031a268__generated, vu0_set_signed_unit_vector, 24, &|c, seed| {
            let mut r = base(c, seed, 0x0031_a268);
            let t0 = gp(c, -0x2ee0);
            c.bus.write32(t0, r.edge_f32());
            let t1 = gp(c, -0x2edc);
            c.bus.write32(t1, r.edge_f32());
            c.g[A0][0] = if seed % 4 == 0 {
                (r.next_u32() & 0xffff_0000) as u64
            } else {
                r.next_u32() as u64
            };
            c.g[A1][0] = sx(DATA + 0x4000);
        });
    }
    // --------------------------------------------------- animation controller

    #[test]
    fn anim_controller_reset_matches() {
        check(ssx3_ee::s10::f_00311a50__generated, anim_controller_reset, 12, &|c, seed| {
            let mut r = base(c, seed, 0x0031_1a50);
            let anim = DATA + 0x100;
            let lists = if seed % 7 == 0 { DATA + 0x2000 } else { DATA + 0x2000 };
            clear(c, anim, 0x60);
            clear(c, lists, 0x30);
            w(c, anim + 0x50, lists);
            c.g[A0][0] = sx(anim);
            for off in [0u32, 0x18, 0x1c, 0x20, 0x30, 0x40] {
                w(c, anim + off, r.next_u32());
            }
        });
    }

    // --------------------------------------------------------------- net events

    #[test]
    fn net_fe_state_queue_net_event_matches() {
        check(ssx3_ee::s03::f_001a8ae8__generated, net_fe_state_queue_net_event, 16, &|c, seed| {
            let mut r = base(c, seed, 0x001a_8ae8);
            elf_image(c);
            heap(c);
            pool(c);
            let state = DATA + 0x100;
            let event = DATA + 0x900;
            let vtable = DATA + 0xa00;
            let node = DATA + 0x3000;
            clear(c, state, 0x700);
            clear(c, node, 0x20);
            w(c, state + 0x6cc, node);
            w(c, node + 4, 0);
            clear(c, event, 0x40);
            clear(c, vtable, 0x40);
            w(c, event + 4, vtable);
            if seed % 3 == 0 {
                // Virtual conversion returns the payload.
                c.bus.write16(vtable + 0x10, 0);
                w(c, vtable + 0x14, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 0x4242);
            } else {
                // Virtual conversion returns null (MenuSys_IsActive on a
                // zeroed first word), so nothing is queued.
                c.bus.write16(vtable + 0x10, 0);
                w(c, vtable + 0x14, 0x002c_c098);
                w(c, event, 0);
            }
            c.g[A0][0] = sx(state);
            c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(event) };
        });
    }

    fn fill_setup(c: &mut Cpu, seed: u32, salt: u64) -> (Rng, u32) {
        let mut r = base(c, seed, salt);
        let list_box = DATA + 0x100;
        let p1 = DATA + 0x2000;
        let p2 = DATA + 0x2100;
        let p3 = DATA + 0x2200;
        let node = DATA + 0x3000;
        clear(c, list_box, 0x340);
        clear(c, p1, 0x100);
        clear(c, p2, 0x100);
        clear(c, p3, 0x100);
        clear(c, node, 0x60);
        // The add-item path stops at the null object chain: no allocation.
        w(c, list_box + 0x5c, p1);
        w(c, p1 + 0xd0, p2);
        w(c, p2 + 0x10, p3);
        w(c, p3 + 0x10, 0);
        // The parent list's node: delta 0 and a miss-hook target.
        w(c, list_box + 8, node);
        c.bus.write16(node + 0x48, 0);
        w(c, node + 0x4c, 0);
        w(c, list_box + 0x14, r.next_u32());
        w(c, list_box + 0x318, (seed % 4) as u32);
        (r, list_box)
    }

    #[test]
    fn net_fe_state_fill_yes_no_list_matches() {
        check(ssx3_ee::s03::f_001a9a18__generated, net_fe_state_fill_yes_no_list, 12, &|c, seed| {
            let (_, list_box) = fill_setup(c, seed, 0x001a_9a18);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(list_box);
            c.g[A2][0] = (seed % 3) as u64;
        });
    }

    #[test]
    fn net_fe_state_fill_on_off_list_matches() {
        check(ssx3_ee::s03::f_001a9ab8__generated, net_fe_state_fill_on_off_list, 12, &|c, seed| {
            let (_, list_box) = fill_setup(c, seed, 0x001a_9ab8);
            c.g[A0][0] = sx(DATA + 0x1000);
            c.g[A1][0] = sx(list_box);
            c.g[A2][0] = (seed % 3) as u64;
        });
    }

    // ------------------------------------------------------------------ audio

    #[test]
    fn ssx_audio_system_set_sfx_sub_channels_enabled_matches() {
        check(
            ssx3_ee::s07::f_002875d0__generated,
            ssx_audio_system_set_sfx_sub_channels_enabled,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x0028_75d0);
                let audio = DATA + 0x100;
                clear(c, audio, 0x6400);
                for k in 0..11 {
                    w(c, audio + 0x62bc + 0x10 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_music_post_event_matches() {
        check(
            ssx3_ee::s07::f_0028f328__generated,
            ssx_audio_system_on_music_post_event,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x0028_f328);
                let audio = DATA + 0x100;
                let player = audio + 0x118;
                let song = DATA + 0x3000;
                let queue = DATA + 0x4000;
                clear(c, audio, 0x6000);
                clear(c, song, 0x80);
                clear(c, queue, 0x40);
                // A live song object whose tag is "none" (-1).
                w(c, player + 0x418, 1);
                w(c, player + 0x408, song);
                w(c, song + 0x58, 0xffff_ffff);
                w(c, audio + 0x5558, queue);
                w(c, queue + 0x1c, 0);
                let event = match seed % 5 {
                    0 => 10,
                    1 => 11,
                    2 => 18,
                    3 => 9,
                    _ => (seed % 32) as u32,
                };
                w(c, GAME_GP.wrapping_add(0x530), (seed % 3) as u32);
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = event as u64;
            },
        );
    }

    fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().chain([0u8]).enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
    }

    // ------------------------------------------------------- front-end widget

    #[test]
    fn c_fe_state_net_ssx_world_on_widget_created_matches() {
        check(
            ssx3_ee::s03::f_001c1e08__generated,
            c_fe_state_net_ssx_world_on_widget_created,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x001c_1e08);
                let widget = DATA + 0x100;
                clear(c, widget, 0x40);
                put_str(c, 0x0046_5a80, "widget_a");
                put_str(c, 0x0046_5a90, "widget_b");
                put_str(c, 0x0046_5aa8, "widget_c");
                put_str(c, 0x0046_5ac0, "widget_d");
                put_str(c, DATA + 0x2000, "other");
                let name = match seed % 5 {
                    0 => 0x0046_5a80,
                    1 => 0x0046_5a90,
                    2 => 0x0046_5aa8,
                    3 => 0x0046_5ac0,
                    _ => DATA + 0x2000,
                };
                w(c, widget + 0x38, name);
                w(c, widget + 0x18, r.next_u32());
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn lobby_man_send_private_message_matches() {
        check(
            ssx3_ee::s06::f_0025fb80__generated,
            lobby_man_send_private_message,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0025_fb80);
                elf_image(c);
                heap(c);
                pool(c);
                let lobby = DATA + 0x100;
                let service = DATA + 0x4000;
                clear(c, lobby, 0x60);
                clear(c, service, 0x600);
                put_str(c, DATA + 0x3000, "player_one");
                put_str(c, DATA + 0x3100, "hello there");
                w(c, lobby + 0x54, service);
                w(c, service + 0xc, 0x6964_6c65); // "idle"
                c.g[A0][0] = sx(lobby);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3100);
            },
        );
    }

    // ----------------------------------------------------------------- roster

        // ------------------------------------------------------------ crowd audio

        // -------------------------------------------------------- script engine

    #[test]
    fn script_engine_load_bank_async_matches() {
        check(ssx3_ee::s07::f_0027b370__generated, script_engine_load_bank_async, 12, &|c, seed| {
            let mut r = base(c, seed, 0x0027_b370);
            let engine = DATA + 0x100;
            let bank = (seed % 4) as u32;
            let slot = engine + 0x628 + 16 * bank;
            clear(c, engine, 0x700);
            put_str(c, DATA + 0x3000, "bank.bin");
            w(c, slot + 4, r.next_u32());
            w(c, slot + 8, if seed % 3 == 0 { 1 } else { 0 });
            w(c, slot + 0xc, if seed % 3 == 0 { 0 } else { 1 });
            w(c, engine + 0x620, r.next_u32());
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = bank as u64;
            c.g[A2][0] = sx(DATA + 0x3000);
            c.g[A3][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x3100) };
            c.g[T0][0] = (seed % 0x1000) as u64;
            c.g[T1][0] = if seed % 4 == 0 { sx(DATA + 0x3200) } else { 0 };
            c.g[T2][0] = 0;
        });
    }

    #[test]
    fn script_engine_clear_obj_binding_filter_matches() {
        check(
            ssx3_ee::s07::f_0027d170__generated,
            script_engine_clear_obj_binding_filter,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x0027_d170);
                let engine = DATA + 0x100;
                let list = DATA + 0x2000;
                let vtable = DATA + 0x3000;
                let filter = (seed % 4) as u32;
                clear(c, engine, 0x1000);
                clear(c, list, 0x80);
                clear(c, vtable, 0x80);
                w(c, engine + 4 * filter, list);
                w(c, list + 0xc, vtable);
                c.bus.write16(vtable + 0x30, 0);
                w(c, vtable + 0x34, crate::difftest::fakes::FAKE);
                c.bus.write16(vtable + 0x38, 0);
                w(c, vtable + 0x3c, crate::difftest::fakes::FAKE + 0x10);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                w(c, crate::difftest::fakes::LOG_RESULTS + 4, 3);
                c.g[A0][0] = sx(engine);
                c.g[A1][0] = filter as u64;
                c.g[A2][0] = 0;
            },
        );
    }

    // ------------------------------------------------------------ keyboard

    /// A keyboard popup whose key tables are all enabled except the given key.
    fn keyboard(c: &mut Cpu, state: u32, disable: &[u32]) -> u32 {
        let popup = DATA + 0x100;
        clear(c, popup, 0x700);
        for i in 0..((0x700 - 0x148) / 4) {
            w(c, popup + 0x148 + 4 * i, 1);
        }
        for &k in disable {
            w(c, popup + 0x148 + 4 * k, 0);
            w(c, popup + 0x298 + 4 * k, 0);
        }
        w(c, popup + 0x140, state);
        w(c, popup + 0x144, 0x55);
        popup
    }

    #[test]
    fn c_keyboard_popup_move_down_from_top_row_matches() {
        check(
            ssx3_ee::s04::f_001cbc48__generated,
            c_keyboard_popup_move_down_from_top_row,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x001c_bc48);
                let state = [4u32, 5, 4, 5, 0x40, 0x4c][seed as usize % 6];
                let disable: &[u32] = if seed % 2 == 0 { &[] } else { &[0x10, 0x12] };
                let popup = keyboard(c, state, disable);
                c.g[A0][0] = sx(popup);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn c_keyboard_popup_move_up_from_bottom_row_matches() {
        check(
            ssx3_ee::s04::f_001cbf20__generated,
            c_keyboard_popup_move_up_from_bottom_row,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x001c_bf20);
                let state = [0x4cu32, 0x4f, 0x4c, 0x4f, 0x40, 4][seed as usize % 6];
                let disable: &[u32] = if seed % 2 == 0 { &[] } else { &[0x42] };
                let popup = keyboard(c, state, disable);
                c.g[A0][0] = sx(popup);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    // ------------------------------------------------------ WScript manager

        fn mission_world(c: &mut Cpu, seed: u32, salt: u64) -> (Rng, u32) {
        let r = base(c, seed, salt);
        let playback = DATA + 0x6000;
        let replay = DATA + 0x7000;
        let riders = DATA + 0x3000;
        let rider = DATA + 0x4000;
        let state = DATA + 0x5000;
        clear(c, playback, 0x100);
        clear(c, replay, 0x100);
        clear(c, riders, 0x100);
        clear(c, rider, 0x800);
        clear(c, state, 0x400);
        w(c, GAME + 0x34, playback);
        w(c, GAME + 0x28, replay);
        w(c, replay, 0xe);
        w(c, GAME + 0xc, riders);
        for i in 0..3 {
            w(c, riders + 0x28 + 4 * i, rider);
        }
        w(c, rider + 0x77c, state);
        let self_obj = DATA + 0x100;
        clear(c, self_obj, 0x300);
        w(c, self_obj, (seed % 3) as u32);
        (r, self_obj)
    }

    #[test]
    fn wscript_man_request_mission_cancel_matches() {
        check(ssx3_ee::s10::f_0030b658__generated, wscript_man_request_mission_cancel, 8, &|c, seed| {
            let (mut r, obj) = mission_world(c, seed, 0x0030_b658);
            w(c, obj + 0x2a0, r.next_u32());
            w(c, obj + 0x2ac, r.next_u32());
            w(c, obj + 0x2b0, r.next_u32());
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 3) as u64;
        });
    }

        #[test]
    fn wscript_man_deactivate_all_matches() {
        check(ssx3_ee::s10::f_0030b7f8__generated, wscript_man_deactivate_all, 8, &|c, seed| {
            let _ = base(c, seed, 0x0030_b7f8);
            let man = DATA + 0x100;
            let mission = DATA + 0x2000;
            let idle = DATA + 0x2100;
            let queue = DATA + 0x2200;
            clear(c, man, 0x400);
            clear(c, mission, 0x40);
            clear(c, idle, 0x40);
            clear(c, queue, 0x100);
            w(c, man + 0x2b8, mission);
            w(c, man + 0x2b4, idle);
            w(c, man + 0x1c4, queue);
            w(c, man + 0x2a0, if seed % 2 == 0 { 0xffff_ffff } else { (1 + seed % 4) as u32 });
            c.pc = 0x0030_b7f8;
            c.g[A0][0] = sx(man);
        });
    }

    #[test]
    fn script_engine_activate_binding_matches() {
        check(ssx3_ee::s07::f_0027d400__generated, script_engine_activate_binding, 12, &|c, seed| {
            let _ = base(c, seed, 0x0027_d400);
            let engine = DATA + 0x100;
            let def = DATA + 0x2000;
            let slot = DATA + 0x3000;
            clear(c, engine, 0x400);
            clear(c, def, 0x80);
            clear(c, slot, 0x80);
            w(c, engine + 0x2a8, def);
            c.bus.write16(def + 0x20, 0);
            if seed % 2 == 0 {
                w(c, def + 0x24, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            } else {
                // Run the second path: the binding's start returns 0.
                w(c, def + 0x24, 0x002c_c098);
            }
            w(c, engine + 0x54, 0);
            if seed % 2 != 0 {
                // The binding start returns 0 (`MenuSys_IsActive` on a zeroed
                // first word), so the value table path runs and reads slot 0.
                w(c, engine, 0);
            } else {
                w(c, engine, slot);
            }
            c.bus.write16(slot + 0x20, 0);
            w(c, slot + 0x24, 0);
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = sx(DATA + 0x3100);
            c.g[A2][0] = (seed % 5) as u64;
        });
    }

    fn str_hash(c: &mut Cpu, addr: u32) -> u64 {
        const RET: u32 = 0x0012_3450;
        c.g[A0][0] = sx(addr);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7670);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    // ----------------------------------------------------------- menu pages

    #[test]
    fn menu_sys_page_set_selection_matches() {
        check(ssx3_ee::s09::f_002cbe68__generated, menu_sys_page_set_selection, 16, &|c, seed| {
            let _ = base(c, seed, 0x002c_be68);
            let page = DATA + 0x100;
            let items = DATA + 0x2000;
            clear(c, page, 0x100);
            let count = 1 + (seed % 4);
            w(c, page, count);
            w(c, page + 4, (seed % count) as u32);
            // The item array starts at +0xc but is indexed with `(index >> 2)`
            // by the original's own convention; fill both layouts with valid
            // item pointers.
            for i in 0..count {
                let item = DATA + 0x3000 + 0x80 * i;
                let vt = DATA + 0x4000 + 0x80 * i;
                clear(c, item, 0x40);
                clear(c, vt, 0x40);
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
        });
    }

    // --------------------------------------------------------------- sound

        // ------------------------------------------------------ front-end state

    #[test]
    fn c_fe_state_mp_circuit_on_activate_matches() {
        check(ssx3_ee::s03::f_001a3308__generated, c_fe_state_mp_circuit_on_activate, 12, &|c, seed| {
            let (mut r, state) = (base(c, seed, 0x001a_3308), DATA + 0x100);
            clear(c, state, 0x100);
            let screen = DATA + 0x2000;
            let objects = DATA + 0x2100;
            let obj = DATA + 0x2200;
            clear(c, screen, 0x80);
            clear(c, objects, 0x20);
            clear(c, obj, 0x340);
            if seed % 3 != 0 {
                w(c, state + 0x40, screen);
                w(c, screen + 0x3c, 1);
                w(c, screen + 0x38, objects);
                w(c, objects, obj);
                let hv = str_hash(c, 0x004a_19e8) as u32;
                w(c, obj + 0x38, hv);
            }
            w(c, state + 0x18, if seed % 6 == 0 { obj } else { r.next_u32() });
            w(c, obj + 0x318, 0);
            w(c, obj + 0x319, r.next_u32() % 4);
            c.g[A0][0] = sx(state);
            c.g[A1][0] = sx(DATA + 0x9000);
        });
    }

    #[test]
    fn net_fe_state_on_mem_card_login_popup_event_matches() {
        check(
            ssx3_ee::s03::f_001a9438__generated,
            net_fe_state_on_mem_card_login_popup_event,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x001a_9438);
                elf_image(c);
                heap(c);
                pool(c);
                let state = DATA + 0x100;
                let vtable = DATA + 0x2000;
                let event_obj = DATA + 0x4000;
                clear(c, state, 0x800);
                clear(c, vtable, 0x200);
                clear(c, event_obj, 0x40);
                w(c, state + 8, vtable);
                w(c, state + 0x10, DATA + 0x3000);
                c.bus.write16(vtable + 0x100, 0);
                w(c, vtable + 0x104, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(event_obj);
                c.g[A2][0] = if seed % 3 == 0 { 0x16 } else { (seed % 0x20) as u64 };
            },
        );
    }

        #[test]
    fn ov_screen_in_game_error_update_opponent_disconnected_text_matches() {
        check(
            ssx3_ee::s04::f_001fb218__generated,
            ov_screen_in_game_error_update_opponent_disconnected_text,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x001f_b218);
                let self_obj = DATA + 0x100;
                let screen = DATA + 0x2000;
                let objects = DATA + 0x2100;
                let obj = DATA + 0x2200;
                let vtable = DATA + 0x3000;
                clear(c, self_obj, 0x100);
                clear(c, screen, 0x80);
                clear(c, objects, 0x20);
                clear(c, obj, 0x100);
                clear(c, vtable, 0x80);
                w(c, self_obj + 0xa0, r.next_u32());
                if seed % 4 != 0 {
                    w(c, self_obj + 0x40, screen);
                    w(c, screen + 0x3c, 1);
                    w(c, screen + 0x38, objects);
                    w(c, objects, obj);
                    let hv2 = str_hash(c, 0x0046_fb18) as u32;
                    w(c, obj + 0x38, hv2);
                    w(c, obj + 8, vtable);
                    c.bus.write16(vtable + 0x48, 0);
                    w(c, vtable + 0x4c, crate::difftest::fakes::FAKE);
                    w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                }
                c.g[A0][0] = sx(self_obj);
            },
        );
    }

    #[test]
    fn be_reward_int_get_next_points_threshold_matches() {
        check(
            ssx3_ee::s01::f_00158be0__generated,
            be_reward_int_get_next_points_threshold,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x0015_8be0);
                let reward = DATA + 0x100;
                clear(c, reward, 0x100);
                for i in 0..3u32 {
                    w(c, 0x0045_b048 + 4 * i, r.next_u32() % 100000);
                }
                c.g[A0][0] = sx(reward);
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed % 7) as u64;
            },
        );
    }

    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        const RET: u32 = 0x0012_3450;
        c.g[A0][0] = size as u64;
        c.g[A1][0] = 0x004a_0000;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7d70;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7d70);
        assert_eq!(c.pc, RET, "EA_MemAllocNamed did not return");
        c.g[V0][0] as u32
    }

    /// A cBXString block at `data`: header (refcount, length, capacity) and
    /// text.
    fn string_block(c: &mut Cpu, data: u32, refcount: i32, text: &str) {
        clear(c, data - 0x10, 0x10);
        w(c, data - 0xc, refcount as u32);
        w(c, data - 8, text.len() as u32);
        w(c, data - 4, 0x100);
        put_str(c, data, text);
    }

    // ---------------------------------------------------------- rider metrics

    #[test]
    fn rider_metrix_chain_uber_grind_matches() {
        check(ssx3_ee::s00::f_00119958__generated, rider_metrix_chain_uber_grind, 8, &|c, seed| {
            let mut r = base(c, seed, 0x0011_9958);
            let m = DATA + 0x100;
            clear(c, m, 0x200);
            w(c, m + 0x54, r.next_u32() % 8);
            w(c, m + 0x74, r.next_u32() % 8);
            w(c, m + 0x30, r.f32(4.0));
            w(c, m + 0x24, r.f32(4.0));
            w(c, m + 0x20, r.next_u32());
            w(c, m + 0xc, r.next_u32());
            c.g[A0][0] = sx(m);
            c.g[A1][0] = (seed % 500) as u64;
        });
    }

    #[test]
    fn rider_metrix_finish_board_press_matches() {
        check(ssx3_ee::s00::f_00119a38__generated, rider_metrix_finish_board_press, 8, &|c, seed| {
            let mut r = base(c, seed, 0x0011_9a38);
            let m = DATA + 0x100;
            clear(c, m, 0x200);
            w(c, m + 0x30, if seed % 2 == 0 { r.f32(1.0) } else { r.next_u32() | 0x8000_0000 });
            w(c, m + 0x24, r.f32(4.0));
            w(c, m + 0x20, r.next_u32());
            w(c, m + 0xc, r.next_u32());
            c.g[A0][0] = sx(m);
        });
    }

    // ------------------------------------------------------------ transitions

    #[test]
    fn c_transition_man_dtor_matches() {
        check(ssx3_ee::s09::f_002e42d0__generated, c_transition_man_dtor, 8, &|c, seed| {
            let _ = base(c, seed, 0x002e_42d0);
            let man = DATA + 0x100;
            let mgr = DATA + 0x5000;
            let vtable = DATA + 0x6000;
            clear(c, man, 0x100);
            clear(c, mgr, 0x1200);
            clear(c, vtable, 0x200);
            w(c, man + 0x7c, if seed % 3 == 0 { 0 } else { DATA + 0x7000 });
            w(c, GAME_GP.wrapping_add(0x2a90), mgr);
            w(c, mgr + 0x10d8, vtable);
            c.bus.write16(vtable + 0x1e0, 0);
            w(c, vtable + 0x1e4, crate::difftest::fakes::FAKE);
            w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            c.g[A0][0] = sx(man);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    // ------------------------------------------------------------- equipment

        // ---------------------------------------------------------------- strings

    #[test]
    fn cbxstring_assign_matches() {
        check(ssx3_ee::s10::f_00318448__generated, cbxstring_assign, 12, &|c, seed| {
            let _ = base(c, seed, 0x0031_8448);
            heap(c);
            let a = DATA + 0x100;
            let b = DATA + 0x110;
            let da = alloc(c, 0x120);
            let da = if da == 0 { DATA + 0x2000 } else { da + 0x10 };
            string_block(c, da + 0x10, if seed % 3 == 2 { -1 } else { 2 }, "left");
            let db = DATA + 0x3000;
            string_block(c, db, 2, "right");
            w(c, a, if seed % 5 == 0 { db } else { da + 0x10 });
            w(c, b, db);
            c.g[A0][0] = sx(a);
            c.g[A1][0] = sx(b);
        });
    }

    #[test]
    fn cbxstring_op_plus_str_cstr_matches() {
        check(ssx3_ee::s10::f_00318630__generated, cbxstring_op_plus_str_cstr, 12, &|c, seed| {
            let _ = base(c, seed, 0x0031_8630);
            heap(c);
            let src_obj = DATA + 0x100;
            let dst_obj = DATA + 0x110;
            let da = alloc(c, 0x120);
            let data = if da == 0 { DATA + 0x2000 } else { da + 0x10 };
            string_block(c, data, 2, "hello");
            w(c, src_obj, data);
            w(c, dst_obj, 0);
            put_str(c, DATA + 0x3000, " world");
            w(c, GAME_GP.wrapping_add(0xda0), 0x0048_dcd4);
            c.g[A0][0] = sx(dst_obj);
            c.g[A1][0] = sx(src_obj);
            c.g[A2][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x3000) };
        });
    }

    #[test]
    fn tag_field_set_flags_matches() {
        check(ssx3_ee::s14::f_003ebf28__generated, tag_field_set_flags, 12, &|c, seed| {
            let _ = base(c, seed, 0x003e_bf28);
            heap(c);
            let tag = DATA + 0x100;
            clear(c, tag, 0x120);
            put_str(c, 0x0049_65c0, "0123456789abcdefghijklmnopqrstuv");
            c.g[A0][0] = sx(tag);
            c.g[A1][0] = (seed % 0x10000) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff,
                _ => (seed * 2654435761) as u64,
            };
        });
    }

    // ------------------------------------------------------------- text scroll

    #[test]
    fn c_ui_text_scroll_grow_lines_matches() {
        check(ssx3_ee::s13::f_003a1958__generated, c_ui_text_scroll_grow_lines, 12, &|c, seed| {
            let _ = base(c, seed, 0x003a_1958);
            heap(c);
            pool(c);
            let obj = DATA + 0x100;
            clear(c, obj, 0x100);
            let old = if seed % 3 == 0 { 0 } else { DATA + 0x40_0400 };
            w(c, obj + 0xbc, old);
            c.bus.write16(obj + 0xc8, (seed % 4) as u16);
            c.g[A0][0] = sx(obj);
        });
    }

    // ------------------------------------------------------------------ luno

    #[test]
    fn luno_table_dtor_matches() {
        check(ssx3_ee::s05::f_00224d00__generated, luno_table_dtor, 8, &|c, seed| {
            let _ = base(c, seed, 0x0022_4d00);
            heap(c);
            let table = DATA + 0x100;
            clear(c, table, 0x100);
            let n = seed % 3;
            let block = alloc(c, 0x20 * (n + 1) + 0x10);
            let block = if block == 0 { DATA + 0x2000 } else { block };
            clear(c, block, 0x20 * (n + 1) + 0x10);
            w(c, block, n);
            let vtable = DATA + 0x3000;
            clear(c, vtable, 0x40);
            c.bus.write16(vtable + 8, 0);
            w(c, vtable + 0xc, crate::difftest::fakes::FAKE);
            for i in 0..n {
                let e = block + 0x10 + 0x20 * (n - 1 - i);
                w(c, e + 0x1c, vtable);
            }
            w(c, table + 0x10, block + 0x10);
            c.g[A0][0] = sx(table);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }

    // ----------------------------------------------------------------- fence

    #[test]
    fn fence_v40_matches() {
        check(ssx3_ee::s11::f_0034d650__generated, fence_v40, 12, &|c, seed| {
            let mut r = base(c, seed, 0x0034_d650);
            let fence = DATA + 0x100;
            let impulse = DATA + 0x2000;
            clear(c, fence, 0x100);
            for k in 0..16 {
                w(c, impulse + 4 * k, r.edge_f32());
            }
            w(c, fence + 0x30, if seed % 3 == 0 { 1.5f32.to_bits() } else { 0.25f32.to_bits() });
            w(c, fence + 0x38, if seed % 2 == 0 { 0 } else { 4 });
            w(c, fence + 0x3c, if seed % 2 == 0 { 0 } else { 4 });
            let cst = gp(c, -0x2ab8);
            c.bus.write32(cst, r.edge_f32());
            c.g[A0][0] = sx(fence);
            c.g[A1][0] = sx(impulse);
        });
    }

    // ------------------------------------------------------- reward preview

    #[test]
    fn c_fe_state_preview_reward_dtor_matches() {
        check(
            ssx3_ee::s04::f_001d1b68__generated,
            c_fe_state_preview_reward_dtor,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x001d_1b68);
                let state = DATA + 0x100;
                let mgr = DATA + 0x5000;
                let vtable = DATA + 0x6000;
                clear(c, state, 0x100);
                clear(c, mgr, 0x1200);
                clear(c, vtable, 0x200);
                w(c, state + 0x70, if seed % 3 == 0 { 0xffff_ffff } else { (seed % 4) as u32 });
                w(c, state + 0x6c, if seed % 4 == 0 { 0xffff_ffff } else { (seed % 4) as u32 });
                w(c, state + 0x64, DATA + 0x7000);
                w(c, GAME_GP.wrapping_sub(0x854), mgr);
                w(c, mgr + 0x10d8, vtable);
                c.bus.write16(vtable + 0x190, 0);
                w(c, vtable + 0x194, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                w(c, WORLD + 0x7c, 0x10_0000);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    // ----------------------------------------------------------- mem card

    #[test]
    fn c_fe_state_mem_card_init_matches() {
        check(ssx3_ee::s02::f_00186b50__generated, c_fe_state_mem_card_init, 8, &|c, seed| {
            let _ = base(c, seed, 0x0018_6b50);
            elf_image(c);
            heap(c);
            pool(c);
            let state = DATA + 0x100;
            let engine = DATA + 0x2000;
            clear(c, state, 0x100);
            clear(c, engine, 0x400);
            put_str(c, DATA + 0x3000, "memcard");
            w(c, state + 0x10, engine);
            c.g[A0][0] = sx(state);
        });
    }

    // -------------------------------------------------------------- world

    #[test]
    fn world_dtor_matches() {
        check(ssx3_ee::s13::f_003a6740__generated, world_dtor, 8, &|c, seed| {
            let _ = base(c, seed, 0x003a_6740);
            heap(c);
            pool(c);
            let world = DATA + 0x100;
            let mgr = DATA + 0x5000;
            let vtable = DATA + 0x6000;
            clear(c, world, 0x100);
            clear(c, mgr, 0x1200);
            clear(c, vtable, 0x200);
            clear(c, DATA + 0x2000, 0x400);
            clear(c, DATA + 0x3000, 0x400);
            clear(c, DATA + 0x4000, 0x400);
            if seed % 2 == 0 {
                w(c, world, DATA + 0x2000);
                w(c, world + 4, DATA + 0x3000);
                w(c, world + 8, DATA + 0x4000);
            }
            w(c, GAME_GP.wrapping_add(0x2a90), mgr);
            w(c, mgr + 0x10d8, vtable);
            c.bus.write16(vtable + 0x358, 0);
            w(c, vtable + 0x35c, crate::difftest::fakes::FAKE);
            w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            c.g[A0][0] = sx(world);
            c.g[A1][0] = (seed % 3) as u64;
        });
    }

    // ----------------------------------------------------------------- HUD

    #[test]
    fn c_ov_state_hud_1p_ctor_matches() {
        check(ssx3_ee::s04::f_001e9a30__generated, c_ov_state_hud_1p_ctor, 8, &|c, seed| {
            let _ = base(c, seed, 0x001e_9a30);
            let state = DATA + 0x100;
            clear(c, state, 0x600);
            c.g[A0][0] = sx(state);
        });
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode_6_matches() {
        check(
            ssx3_ee::s04::f_001f90e8__generated,
            ov_screen_pause_freeride_init_mode_6,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x001f_90e8);
                elf_image(c);
                heap(c);
                pool(c);
                let screen = DATA + 0x100;
                let engine = DATA + 0x2000;
                clear(c, screen, 0x300);
                clear(c, engine, 0x400);
                w(c, screen + 0x10, engine);
                w(c, screen + 0xbc, (seed % 4) as u32);
                c.g[A0][0] = sx(screen);
            },
        );
    }

    // -------------------------------------------------------- command queue

    #[test]
    fn script_cmd_queue_pop_matches() {
        check(ssx3_ee::s07::f_00283dc0__generated, script_cmd_queue_pop, 12, &|c, seed| {
            let mut r = base(c, seed, 0x0028_3dc0);
            let q = DATA + 0x100;
            let out = DATA + 0x2000;
            clear(c, q, 0x200);
            clear(c, out, 0x40);
            if seed % 2 == 0 {
                w(c, q + 0x8c, (seed % 5) as u32);
                w(c, q + 0x90, (seed % 5) as u32);
                w(c, q + 0x94, 0);
            } else {
                w(c, q + 0x8c, (seed % 4) as u32);
                w(c, q + 0x90, (seed % 4 + 1) as u32);
                w(c, q + 0x94, 1);
            }
            for k in 0..8 {
                w(c, q + 0x1c * k, r.next_u32());
            }
            c.g[A0][0] = sx(q);
            c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(out) };
        });
    }

    // ------------------------------------------------------ rider wipeout

    #[test]
    fn rider_wipeout_state_enable_root_pose_override_matches() {
        check(
            ssx3_ee::s01::f_00136d40__generated,
            rider_wipeout_state_enable_root_pose_override,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x0013_6d40);
                let rider = DATA + 0x100;
                let model = DATA + 0x2000;
                let pose = DATA + 0x3000;
                let offset = DATA + 0x3100;
                let root = DATA + 0x3200;
                clear(c, rider, 0x100);
                clear(c, model, 0x400);
                for a in [pose, offset, root] {
                    for k in 0..8 {
                        w(c, a + 4 * k, r.next_u32());
                    }
                }
                w(c, rider + 0x40, model);
                w(c, rider, (seed % 3 == 0) as u32);
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(offset);
                c.g[A2][0] = sx(root);
                c.g[A3][0] = sx(pose);
            },
        );
    }

    // -------------------------------------------------------- BE library

    #[test]
    fn be_library_load_state_matches() {
        check(ssx3_ee::s01::f_0014dfa8__generated, be_library_load_state, 8, &|c, seed| {
            let _ = base(c, seed, 0x0014_dfa8);
            let lib = DATA + 0x100;
            let request = DATA + 0x2000;
            let vtable = DATA + 0x3000;
            clear(c, lib, 0x100);
            clear(c, request, 0x100);
            clear(c, vtable, 0x80);
            w(c, request, vtable);
            c.bus.write16(vtable + 0x10, 0);
            w(c, vtable + 0x14, crate::difftest::fakes::FAKE);
            w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            c.g[A0][0] = sx(lib);
            c.g[A1][0] = sx(request);
        });
    }

    // ---------------------------------------------------- read-message popup

    #[test]
    fn c_fe_state_read_message_popup_add_sent_message_matches() {
        check(
            ssx3_ee::s03::f_001a7ed8__generated,
            c_fe_state_read_message_popup_add_sent_message,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x001a_7ed8);
                elf_image(c);
                heap(c);
                pool(c);
                let popup = DATA + 0x100;
                let player = DATA + 0x2000;
                clear(c, popup, 0x800);
                clear(c, player, 0x100);
                put_str(c, DATA + 0x3000, "sender");
                put_str(c, DATA + 0x3100, "message body");
                w(c, player + 0x80, DATA + 0x3000);
                w(c, GAME_GP.wrapping_sub(0xc8), player);
                c.g[A0][0] = sx(popup);
                c.g[A1][0] = sx(DATA + 0x3100);
            },
        );
    }

}
