//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for a batch of cold-path functions — DJ/arcade speech
//! wrappers, script-engine helpers, memory-card and front-end screen state
//! handlers, tag/ini parser helpers — each with a differential test against
//! the generated body.
//!
//! The generated code is the authority for cycles, registers and memory.
//! Calls use [`crate::call`] (a [`Callee`] for a same-shard direct call,
//! [`call_ptr`] for `jalr`/cross-shard calls) so that overridden callees and
//! host-thread unwinds behave exactly as the compiled caller. Every block
//! charge and every register the generated body leaves behind (including
//! `v0`/`v1` and the scratch values an indirect callee observes) is
//! reproduced.

use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_26f0,
        ssx_audio_system_dj_say20e5,
        "SSXAudioSystem_DJSay20E5",
    ),
    (
        0x002a_3b18,
        ssx_audio_system_arcade_say20a7,
        "SSXAudioSystem_ArcadeSay20A7",
    ),
    (
        0x002a_3c00,
        ssx_audio_system_arcade_say20a8,
        "SSXAudioSystem_ArcadeSay20A8",
    ),
    (
        0x0027_95d0,
        script_engine_spawn_script,
        "ScriptEngine_SpawnScript",
    ),(0x0035_5770, mod_obj_v40, "ModObj_v40"),
    (
        0x0017_e030,
        lobby_menu_create_account,
        "LobbyMenu_CreateAccount",
    ),
    (
        0x001c_ff10,
        cfe_state_reward_gallery_poll_shape_load,
        "cFEStateRewardGallery_PollShapeLoad",
    ),
    (
        0x001d_1ff0,
        cfe_state_preview_reward_handle_input,
        "cFEStatePreviewReward_HandleInput",
    ),
    (
        0x001f_af10,
        ovscreen_in_game_error_update,
        "OVScreenInGameError_Update",
    ),(0x003e_be40, tag_field_set_number, "TagFieldSetNumber"),
    (
        0x0016_8fe8,
        script_cam_controller_ctor,
        "ScriptCamController_ctor",
    ),(
        0x0021_0fa0,
        ovscreen_read_private_message_create_keyboard_popup,
        "OVScreenReadPrivateMessage_CreateKeyboardPopup",
    ),
    (
        0x0032_16e8,
        input_parser_lookup_button,
        "InputParser_LookupButton",
    ),
    (
        0x0019_3568,
        cfe_state_bragging_rights_on_activate,
        "cFEStateBraggingRights_OnActivate",
    ),
    (
        0x001f_8df8,
        ovscreen_pause_freeride_play_icon_anim,
        "OVScreenPauseFreeride_PlayIconAnim",
    ),
    (
        0x0014_9778,
        bechar_int_is_trick_score_above_rating,
        "BECharInt_IsTrickScoreAboveRating",
    ),
    (
        0x001e_2428,
        message_center_post_peak_unlock_message,
        "MessageCenter_PostPeakUnlockMessage",
    ),
    (
        0x0035_f598,
        texflip_modifier_handle_event,
        "TexFlipModifier_HandleEvent",
    ),
    (
        0x0025_f030,
        lobby_man_on_quick_play_reply,
        "LobbyMan_OnQuickPlayReply",
    ),
    (
        0x0015_9080,
        bereward_int_get_medal_reward_id,
        "BERewardInt_GetMedalRewardId",
    ),(0x0036_a2c0, ps2_graphics_man_v94, "PS2GraphicsMan_v94"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

mod callee {
    use super::*;

    pub const IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::SSXAudioSystem_IsDJSpeechOn,
    };
    pub const IS_ARCADE_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f160,
        f: ee::s08::SSXAudioSystem_IsArcadeSpeechOn,
    };
    pub const GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::SSXAudioSystem_GetLanguageMask,
    };
    pub const SPEECH_STOP_INSTANCE: Callee = Callee {
        addr: 0x002b_11b0,
        f: ee::s08::SpeechSystem_StopInstance,
    };
    pub const SPEECH_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };
    pub const SCRIPT_POP_FREE_NODE: Callee = Callee {
        addr: 0x0027_bd78,
        f: ee::s07::ScriptEngine_PopFreeNode,
    };
    pub const SCRIPT_PUSH_FREE_NODE: Callee = Callee {
        addr: 0x0027_bd90,
        f: ee::s07::ScriptEngine_PushFreeNode,
    };
    pub const SCRIPT_START_SCRIPT: Callee = Callee {
        addr: 0x0028_21a0,
        f: ee::s07::ScriptEngine_StartScript,
    };
    pub const MOD_OBJ_RUN_SCRIPT_EVENT: Callee = Callee {
        addr: 0x0034_fe00,
        f: ee::s11::ModObj_RunScriptEvent19b8,
    };
    pub const REWARD_GALLERY_REGISTER_TEXTURES: Callee = Callee {
        addr: 0x001c_fff8,
        f: ee::s04::cFEStateRewardGallery_RegisterTextures,
    };
    pub const GAME_ERROR_SERVER_TIMEOUT: Callee = Callee {
        addr: 0x001f_b180,
        f: ee::s04::OVScreenInGameError_UpdateServerTimeoutText,
    };
    pub const GAME_ERROR_OPPONENT_DISCONNECTED: Callee = Callee {
        addr: 0x001f_b218,
        f: ee::s04::OVScreenInGameError_UpdateOpponentDisconnectedText,
    };
    pub const GAME_ERROR_SHOW_CANCEL: Callee = Callee {
        addr: 0x001f_aff8,
        f: ee::s04::OVScreenInGameError_ShowCancelGame,
    };
    pub const BE_LIBRARY_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };
    pub const BE_LIBRARY_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };
    pub const BE_REWARD_SET_BIT: Callee = Callee {
        addr: 0x0015_73f0,
        f: ee::s01::BERewardInt_SetBit,
    };
    pub const BE_REWARD_GET_FILE: Callee = Callee {
        addr: 0x0015_a478,
        f: ee::s01::BERewardInt_GetRewardsFile,
    };
    pub const BE_ECON_SPEND_MONEY: Callee = Callee {
        addr: 0x0015_0b48,
        f: ee::s01::BEEconInt_SpendMoney,
    };
    pub const BE_ECON_GET_MONEY: Callee = Callee {
        addr: 0x0015_0928,
        f: ee::s01::BEEconInt_GetMoney,
    };
    pub const TAG_FIELD_COPY_TAG: Callee = Callee {
        addr: 0x003e_b198,
        f: ee::s14::TagFieldCopyTag,
    };
    pub const TAG_FIELD_SET_RAW: Callee = Callee {
        addr: 0x003e_b1e0,
        f: ee::s14::TagFieldSetRaw,
    };
    pub const BRAGGING_RIGHTS_UPDATE_ROW_COLORS: Callee = Callee {
        addr: 0x0019_27b0,
        f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
    };
    pub const BENEWPLAYER_IS_SHARING_CHAR_RECORD: Callee = Callee {
        addr: 0x0014_77e8,
        f: ee::s01::BENewPlayerInt_IsSharingCharRecord,
    };
    pub const BECHAR_GET_OVERALL_RATING: Callee = Callee {
        addr: 0x0014_8950,
        f: ee::s01::BECharInt_GetOverallRating,
    };
    pub const MESSAGE_IS_FLAG_SET: Callee = Callee {
        addr: 0x001e_3a30,
        f: ee::s04::MessageCenter_IsFlagSet,
    };
    pub const MESSAGE_SET_FLAG: Callee = Callee {
        addr: 0x001e_3a78,
        f: ee::s04::MessageCenter_SetFlag,
    };
    pub const MESSAGE_POST: Callee = Callee {
        addr: 0x001e_2fe0,
        f: ee::s04::MessageCenter_PostMessage,
    };
    pub const TEXFLIP_SET_FRAME: Callee = Callee {
        addr: 0x0035_f378,
        f: ee::s12::TexFlipModifier_SetFrame,
    };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };
    pub const BENEWPLAYER_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };
    pub const VFX_PLANTS_GET_SWAY_MATRIX: Callee = Callee {
        addr: 0x0038_f2a8,
        f: ee::s12::VfxPlants_GetSwayMatrix,
    };
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The low word of a GPR.
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

// ---------------------------------------------------------------------------
// DJ / arcade speech wrappers.

/// `void SSXAudioSystem_DJSay20E5(void* self, int voice, int lang)` (0x2a26f0):
/// when DJ speech is enabled, stop the instance's speech stream, set the
/// `0x20e5` event context and start the announcement through the speech
/// interface, clearing the "pending" word at `self + 0x5794`.
pub fn ssx_audio_system_dj_say20e5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        callee::IS_DJ_SPEECH_ON.call(c, 0x002a_2720)?;

        let speech = p(r(c, S1), 0x5560);
        let speaking = r(c, V0) != 0;
        c.g[S0][0] = sx(speech);
        if !speaking {
            return epilogue(c, frame);
        }

        c.cycles += 14;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(speech);
        callee::SPEECH_STOP_INSTANCE.call(c, 0x002a_2734)?;

        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20e5;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xa;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        callee::SPEECH_SET_EVENT_CONTEXT.call(c, 0x002a_2758)?;

        let started = r(c, V0) != 0;
        c.g[A0][0] = c.g[S1][0];
        if !started {
            return epilogue(c, frame);
        }

        // `s2` is the voice (1 or 2), `s0` the language index (1 or 2).
        c.cycles += 7;
        c.g[S2][0] = 2;
        callee::GET_LANGUAGE_MASK.call(c, 0x002a_2768)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[S0][0] = 1;
        c.g[V0][0] = Addr(p(r(c, S1), 0x5794)).lw(c);
        let pending = c.g[V0][0] == 0;
        if c.g[S4][0] != 0 {
            c.g[S2][0] = c.g[S0][0];
        }
        if !pending {
            c.cycles += 1;
            c.sw(p(r(c, S1), 0x5794), 0);
        }

        c.cycles += 13;
        c.g[V0][0] = 2;
        c.g[A0][0] = 1;
        if c.g[S5][0] != 0 {
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20e5;
        call_ptr(c, 0x003d_8008, 0x002a_2798)?;

        c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S0][0];
        let target = r(c, V1);
        c.g[A1][0] = 3;
        call_ptr(c, target, 0x002a_27b4)?;
        epilogue(c, frame)
    })
}

/// The `DJSay20E5` epilogue: restore s0-s5, ra, sp and return.
fn epilogue(c: &mut Cpu, frame: Frame) -> Flow {
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
}

/// `void SSXAudioSystem_ArcadeSay20A7(void* self, void* obj, int arg)`
/// (0x2a3b18): when arcade speech is enabled, the object's animation has not
/// finished (`+0x470 >= 0`), the object is this system's current animation
/// target and the speech system is idle, announce event `0x20a7` and hand it
/// to the speech interface.
pub fn ssx_audio_system_arcade_say20a7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x40);
        callee::IS_ARCADE_SPEECH_ON.call(c, 0x002a_3b40)?;

        'outer: {
            if r(c, V0) == 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            // f20 = 0; fcc = (f20 <= *(*obj + 0x470)), i.e. ">= 0".
            c.cycles += 6;
            c.lwc1(0, p(r(c, S0), 0x470));
            c.mtc1(0, 20);
            c.c_le_s(20, 0);
            let done = c.fcc();
            c.g[V0][0] = 1;
            if !done {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if r(c, V0) != 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            // The object must be this system's bound animation object.
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = u64::MAX;
            call_ptr(c, 0x0028_5d98, 0x002a_3b80)?;
            if c.g[S0][0] != c.g[V0][0] {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0028_8ae0, 0x002a_3b90)?;
            let busy = r(c, V0) != 0;
            frame.restore(c, 16, 0x30);
            if busy {
                break 'outer;
            }

            'context: {
                c.cycles += 11;
                c.g[A0][0] = sx(p(r(c, S1), 0x5560));
                c.mov_s(12, 20);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20a7;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0xc;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                callee::SPEECH_SET_EVENT_CONTEXT.call(c, 0x002a_3bbc)?;
                let skip = r(c, V0) == 0;
                c.g[A0][0] = 1;
                if skip {
                    break 'context;
                }
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20a7;
                call_ptr(c, 0x003d_8008, 0x002a_3bd0)?;
                c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                let target = r(c, V1);
                c.g[A1][0] = 1;
                call_ptr(c, target, 0x002a_3be4)?;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ArcadeSay20A8(void* self, void* obj, int arg)`
/// (0x2a3c00): as 0x2a3b18 with event `0x20a8`; the `+0x470` animation test is
/// skipped when `arg == 2`.
pub fn ssx_audio_system_arcade_say20a8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        callee::IS_ARCADE_SPEECH_ON.call(c, 0x002a_3c24)?;

        'outer: {
            if r(c, V0) == 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            'checks: {
                c.cycles += 2;
                let no_object = c.g[S0][0] == 0;
                c.g[V0][0] = 2;
                if no_object {
                    break 'checks;
                }

                'idle: {
                    c.cycles += 2;
                    let shortcut = c.g[S2][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if shortcut {
                        break 'idle;
                    }
                    // f0 = 0; fcc = (f0 <= *(*obj + 0x470)).
                    c.cycles += 6;
                    c.lwc1(1, p(r(c, S0), 0x470));
                    c.mtc1(0, 0);
                    c.c_le_s(0, 1);
                    let done = c.fcc();
                    c.g[V0][0] = 1;
                    if !done {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    if r(c, V0) != 0 {
                        frame.restore(c, 16, 0x30);
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                }

                c.cycles += 4;
                c.g[A1][0] = u64::MAX;
                call_ptr(c, 0x0028_5d98, 0x002a_3c6c)?;
                if c.g[S0][0] != c.g[V0][0] {
                    frame.restore(c, 16, 0x30);
                    break 'outer;
                }

                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0028_8ae0, 0x002a_3c7c)?;
                let busy = r(c, V0) != 0;
                frame.restore(c, 16, 0x30);
                if busy {
                    break 'outer;
                }

                c.cycles += 11;
                c.mtc1(0, 12);
                c.g[A0][0] = sx(p(r(c, S1), 0x5560));
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20a8;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0xc;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                callee::SPEECH_SET_EVENT_CONTEXT.call(c, 0x002a_3ca8)?;
                let skip = r(c, V0) == 0;
                c.g[A0][0] = 1;
                if !skip {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x20a8;
                    call_ptr(c, 0x003d_8008, 0x002a_3cbc)?;
                    c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S2][0];
                    let target = r(c, V1);
                    c.g[A1][0] = 1;
                    call_ptr(c, target, 0x002a_3cd0)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        }

        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `int ScriptEngine_SpawnScript(ScriptEngine* self, int key, int a, int b,
/// int c)` (0x2795d0): allocate a script node, clamp the three rider indices
/// to `[-1, rider_count)`, fill the node and start the script; a failed start
/// returns the node to the free list.
pub fn script_engine_spawn_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[T0][0];
        c.g[S4][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A1][0];
        frame.save_ra(c, 0);

        // rider_count = world->+0x84->+0xc->+0x78
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(r(c, V0)).off(0x84).lw(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0xc).lw(c);
        let first_negative = (c.g[S1][0] as i64) < 0;
        c.g[V1][0] = Addr(r(c, V0)).off(0x78).lw(c);
        if first_negative {
            c.cycles += 1;
            c.g[S1][0] = u64::MAX;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if r(c, V0) == 0 {
                c.cycles += 1;
                c.g[S1][0] = u64::MAX;
            }
        }

        // a3
        c.cycles += 2;
        let negative = (c.g[S2][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let in_range = if negative {
            false
        } else {
            c.cycles += 2;
            r(c, V0) != 0
        };
        if !in_range {
            c.cycles += 1;
            c.g[S2][0] = u64::MAX;
        }

        // t0
        c.cycles += 2;
        let negative = (c.g[S3][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let in_range = if negative {
            false
        } else {
            c.cycles += 2;
            r(c, V0) != 0
        };
        if !in_range {
            c.cycles += 1;
            c.g[S3][0] = u64::MAX;
        }

        c.cycles += 13;
        c.g[A0][0] = c.g[S4][0];
        callee::SCRIPT_POP_FREE_NODE.call(c, 0x0027_9658)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S5][0];
        c.sw(r(c, S0), c.g[S1][0]);
        c.g[A0][0] = c.g[S4][0];
        c.sw(p(r(c, S0), 0x4), c.g[S2][0]);
        c.g[A2][0] = c.g[S0][0];
        c.sw(p(r(c, S0), 0x8), c.g[S3][0]);
        callee::SCRIPT_START_SCRIPT.call(c, 0x0027_9678)?;

        c.g[S1][0] = c.g[V0][0];
        let started = (c.g[S1][0] as i64) >= 0;
        c.g[V0][0] = c.g[S1][0];
        if !started {
            c.cycles += 4;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SCRIPT_PUSH_FREE_NODE.call(c, 0x0027_9690)?;
            c.g[V0][0] = c.g[S1][0];
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

// ---------------------------------------------------------------------------
// Memory card manager.

/// `void MCManager_State09(MCManager* m)` (0x2409c8): the memory-card
/// manager's "check" state. The first sub-object query sets the next state
/// directly; otherwise the idle/defer query runs, then the two condition
/// callbacks, and a non-zero result selects state `1` or `0xd`.
pub fn mcmanager_state09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];

        // if (m->sub434->v11()) return;
        c.g[A1][0] = Addr(r(c, S0)).off(0x434).lw(c);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x58).lh(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0x5c).lw(c);
        let target = r(c, V0);
        c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
        call_ptr(c, target, 0x0024_09f0)?;
        if r(c, V0) != 0 {
            return mcmanager_return(c, frame);
        }

        c.cycles += 8;
        c.g[A1][0] = Addr(r(c, S0)).off(0x434).lw(c);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x1a8).lh(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0x1ac).lw(c);
        let target = r(c, V0);
        c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
        call_ptr(c, target, 0x0024_0a10)?;
        if r(c, V0) != 0 {
            // v53() non-zero: the sub-object's v73() decides whether to wait.
            c.g[A1][0] = Addr(r(c, S0)).off(0x434).lw(c);
            c.cycles += 7;
            c.g[V1][0] = Addr(r(c, A1)).lw(c);
            c.g[A0][0] = Addr(r(c, V1)).off(0x248).lh(c);
            c.g[V0][0] = Addr(r(c, V1)).off(0x24c).lw(c);
            let target = r(c, V0);
            c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
            call_ptr(c, target, 0x0024_0a38)?;
            let idle = r(c, V0) == 0;
            c.g[A1][0] = 0x14;
            if !idle {
                c.cycles += 2;
                c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
            c.g[A1][0] = 4;
        }

        // if (m->v70() == 0) { if (m->v6c() != 0) m->v01(1); return; }
        c.cycles += 5;
        c.g[V0][0] = Addr(r(c, S0)).off(0x70).lw(c);
        let target = r(c, V0);
        call_ptr(c, target, 0x0024_0a54)?;
        let zero = r(c, V0) == 0;
        c.g[A1][0] = 0xd;
        if zero {
            c.cycles += 5;
            c.g[V0][0] = Addr(r(c, S0)).off(0x6c).lw(c);
            let target = r(c, V0);
            call_ptr(c, target, 0x0024_0a84)?;
            let zero = r(c, V0) == 0;
            c.g[A1][0] = 1;
            if !zero {
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
                c.g[A0][0] = sx(p(r(c, S0), Addr(r(c, V0)).off(0x8).lh(c) as u32));
                let target = Addr(r(c, V0)).off(0xc).lw(c) as u32;
                call_ptr(c, target, 0x0024_0aa0)?;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
            c.cycles += 6;
            c.g[A0][0] = sx(p(r(c, S0), Addr(r(c, V0)).off(0x8).lh(c) as u32));
            let target = Addr(r(c, V0)).off(0xc).lw(c) as u32;
            call_ptr(c, target, 0x0024_0a70)?;
        }

        c.cycles += 1;
        mcmanager_return(c, frame)
    })
}

/// The `MCManager_State09` tail: `lq s0; ld ra; addiu sp; jr ra`.
fn mcmanager_return(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Modifier objects.

/// `void ModObj_v40(ModObj* self, void* a1, void* a2, int a3)` (0x355770):
/// forward the "run script event" call to the holder object at `self+0x1c`
/// (its slot 9/10 pair), then, if the object's script timer at `+0x20` has
/// expired, restart it from half the global tick delta and run the event.
pub fn mod_obj_v40(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r(c, S2)).off(0x1c).lw(c);
        let no_holder = r(c, V1) == 0;
        c.g[S5][0] = c.g[A3][0];

        'outer: {
            if !no_holder {
                c.cycles += 3;
                c.g[A3][0] = Addr(r(c, V1)).lw(c);
                if c.g[A3][0] != 0 {
                    // obj->v10(a1, a2)
                    c.cycles += 18;
                    c.g[V1][0] = Addr(r(c, A3)).lw(c);
                    c.g[A0][0] = Addr(r(c, V1)).off(0x50).lh(c);
                    c.g[V0][0] = Addr(r(c, V1)).off(0x54).lw(c);
                    let target = r(c, V0);
                    c.g[A0][0] = sx(p(r(c, A3), r(c, A0)));
                    call_ptr(c, target, 0x0035_57c8)?;

                    c.g[V1][0] = Addr(r(c, S2)).off(0x1c).lw(c);
                    c.g[A0][0] = c.g[S5][0];
                    c.g[S0][0] = Addr(r(c, V1)).lw(c);
                    c.g[S1][0] = Addr(r(c, S0)).lw(c);
                    c.g[V0][0] = Addr(r(c, S1)).off(0x48).lh(c);
                    c.g[S0][0] = sx(p(r(c, S0), r(c, V0)));
                    call_ptr(c, 0x002d_1b30, 0x0035_57e4)?;
                    c.g[V1][0] = Addr(r(c, S1)).off(0x4c).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    let target = r(c, V1);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, target, 0x0035_57f4)?;

                    let done = r(c, V0) == 0;
                    frame.restore(c, 16, 0x60);
                    if done {
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r(c, S2)).off(0x20).lw(c);
                } else {
                    c.g[V0][0] = Addr(r(c, S2)).off(0x20).lw(c);
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = Addr(r(c, S2)).off(0x20).lw(c);
            }

            c.cycles += 2;
            let expired = (c.g[V0][0] as i64) > 0;
            frame.restore(c, 16, 0x60);
            if expired {
                break 'outer;
            }
            c.cycles += 12;
            c.g[V0][0] = Addr(gp(c, 0x2a74)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[V1][0] = Addr(r(c, V0)).off(0x10).lw(c);
            c.g[A3][0] = c.g[S5][0];
            c.g[V0][0] = sx(r(c, V1) >> 31);
            c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
            c.g[V1][0] = sx((r(c, V1) as i32 >> 1) as u32);
            c.sw(Addr(r(c, S2)).off(0x20).0, c.g[V1][0]);
            callee::MOD_OBJ_RUN_SCRIPT_EVENT.call(c, 0x0035_5834)?;
            frame.restore(c, 16, 0x60);
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
    })
}

// ---------------------------------------------------------------------------
// Lobby.

/// `void LobbyMenu_CreateAccount(void* self, char* user, char* pass)`
/// (0x17e030): drain the menu's page list, store the new account name and
/// password into the lobby's strings and fill the pending account request
/// (user/pass/pass2/host, port 0x79e), then hand it to the lobby manager.
pub fn lobby_menu_create_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(p(r(c, S1), 0x8));
        c.g[S3][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r(c, S1), 0x8)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);

        'drain: {
            if empty {
                break 'drain;
            }
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e068)?;
                c.g[V0][0] = Addr(r(c, S0)).lw(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    break 'drain;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_e060);
                }
            }
        }

        c.cycles += 41;
        c.g[A0][0] = sx(p(r(c, S1), 0x1c04));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e080)?;
        c.g[A0][0] = sx(p(r(c, S1), 0x1c08));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e08c)?;

        c.g[S0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[V0][0] = 0x4;
        c.sw(p(r(c, S1), 0x4), c.g[V0][0]);
        c.g[A1][0] = c.g[S3][0];
        c.g[S1][0] = sx(p(r(c, S0), 0x260));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e0a8)?;
        c.g[A0][0] = sx(p(r(c, S0), 0x264));
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e0b4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(p(r(c, S0), 0x268));
        call_ptr(c, 0x0031_84e8, 0x0017_e0c0)?;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(p(r(c, S0), 0x26c));
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x2db8));
        call_ptr(c, 0x0031_84e8, 0x0017_e0d0)?;

        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[V1][0] = 1;
        c.g[V0][0] = 0x79e;
        c.sw(p(r(c, S0), 0x29c), 0);
        c.sw(p(r(c, S0), 0x298), 0);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(r(c, S0), 0x28c), c.g[V1][0]);
        c.sw(p(r(c, S0), 0x290), c.g[V0][0]);
        c.sw(p(r(c, S0), 0x294), c.g[V1][0]);
        c.sw(p(r(c, S0), 0x288), c.g[V1][0]);
        call_ptr(c, 0x0025_b9c8, 0x0017_e0fc)?;

        // The tail of 0x17e030 runs no extra block charge.
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `void cFEStateRewardGallery_PollShapeLoad(cFEStateRewardGallery* self)`
/// (0x1cff10): while the screen waits for the reward shape's file (bit 3 of
/// `+0x48` clear), poll the shape's load state; once loaded, register the
/// textures, release the file and mark the screen "done" (`+0x48` bit 3),
/// firing the two completion slots when bit 1 is set.
pub fn cfe_state_reward_gallery_poll_shape_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);

        c.g[V0][0] = Addr(r(c, S0)).off(0x48).lw(c);
        c.g[V0][0] = sx((r(c, V0) as i32 >> 3) as u32);
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] != 0 {
            frame.restore(c, 16, 0x20);
            return cfe_poll_tail(c, frame);
        }

        c.cycles += 9;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A1][0] = Addr(r(c, S0)).off(0x50).lw(c);
        c.g[V1][0] = Addr(r(c, V0)).off(0x7c).lw(c);
        c.g[A2][0] = Addr(r(c, S0)).off(0x8ec).lw(c);
        c.g[S1][0] = sx(p(r(c, V1), 0x1a70));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_e238, 0x001c_ff54)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x20);
            return cfe_poll_tail(c, frame);
        }

        'done: {
            c.cycles += 21;
            c.g[A2][0] = Addr(r(c, S0)).off(0x8ec).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = Addr(r(c, S0)).off(0x50).lw(c);
            call_ptr(c, 0x0019_e2b0, 0x001c_ff6c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::REWARD_GALLERY_REGISTER_TEXTURES.call(c, 0x001c_ff78)?;
            c.g[A1][0] = Addr(r(c, S0)).off(0x50).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = Addr(r(c, S0)).off(0x8ec).lw(c);
            call_ptr(c, 0x0019_dc20, 0x001c_ff88)?;

            c.g[V1][0] = Addr(r(c, S0)).off(0x48).lw(c);
            c.g[V0][0] = 0xffff_ffff_ffff_fffb;
            c.g[A0][0] = u64::MAX;
            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
            c.sw(Addr(r(c, S0)).off(0x8ec).0, c.g[A0][0]);
            c.g[V1][0] = c.g[V1][0] | 0x8;
            c.g[V0][0] = sx((r(c, V1) as i32 >> 1) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let quiet = c.g[V0][0] == 0;
            c.sw(Addr(r(c, S0)).off(0x48).0, c.g[V1][0]);
            if quiet {
                break 'done;
            }
            c.cycles += 13;
            c.g[V0][0] = Addr(r(c, S0)).off(0x924).lw(c);
            c.g[V1][0] = Addr(r(c, S0)).off(0x8).lw(c);
            c.sw(Addr(r(c, S0)).off(0x8fc).0, c.g[V0][0]);
            c.g[A0][0] = Addr(r(c, V1)).off(0x140).lh(c);
            c.g[V0][0] = Addr(r(c, V1)).off(0x144).lw(c);
            let target = r(c, V0);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x001c_ffcc)?;

            c.g[V1][0] = Addr(r(c, S0)).off(0x8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(r(c, V1)).off(0x108).lh(c);
            c.g[V0][0] = Addr(r(c, V1)).off(0x10c).lw(c);
            let target = r(c, V0);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x001c_ffe4)?;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
        cfe_poll_tail(c, frame)
    })
}

/// The `lq s1; ld ra; addiu sp; jr ra` tail.
fn cfe_poll_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int cFEStatePreviewReward_HandleInput(cFEStatePreviewReward* self,
/// cUIEvent* ev)` (0x1d1ff0): when the event is a confirm on the preview, close
/// the state's objects and play the reward's "confirm" frame; returns 1 when
/// handled, 0 otherwise.
pub fn cfe_state_preview_reward_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);

        c.g[V1][0] = Addr(r(c, A1)).off(0x8).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x30).lh(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0x34).lw(c);
        let target = r(c, V0);
        c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
        call_ptr(c, target, 0x001d_201c)?;
        let not_confirm = r(c, V0) == 0;
        c.g[V0][0] = 0;
        if not_confirm {
            return preview_reward_tail(c, frame);
        }

        'objects: {
            c.cycles += 6;
            c.g[V0][0] = Addr(r(c, S2)).off(0x10).lw(c);
            c.g[V1][0] = sx(p(r(c, GP), 0x2968));
            c.g[A2][0] = Addr(r(c, V0)).off(0x14).lw(c);
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if none {
                break 'objects;
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(r(c, A2)).lw(c);
            c.g[A0][0] = Addr(r(c, V0)).off(0x10).lh(c);
            c.g[V1][0] = Addr(r(c, V0)).off(0x14).lw(c);
            let target = r(c, V1);
            c.g[A0][0] = sx(p(r(c, A2), r(c, A0)));
            call_ptr(c, target, 0x001d_2050)?;
        }

        c.cycles += 2;
        c.g[S0][0] = sx(p(r(c, S2), 0x48));
        c.g[S1][0] = 2;
        loop {
            'slot: {
                c.cycles += 3;
                c.g[A2][0] = Addr(r(c, S0)).lw(c);
                let empty = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if empty {
                    break 'slot;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, A2)).off(0x8).lw(c);
                c.g[A0][0] = Addr(r(c, V0)).off(0x48).lh(c);
                c.g[V1][0] = Addr(r(c, V0)).off(0x4c).lw(c);
                let target = r(c, V1);
                c.g[A0][0] = sx(p(r(c, A2), r(c, A0)));
                call_ptr(c, target, 0x001d_2078)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx(r(c, S1).wrapping_sub(0x1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(p(r(c, S0), 0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_2058);
            }
        }

        c.cycles += 15;
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x23a0));
        call_ptr(c, 0x0031_7670, 0x001d_2090)?;
        c.g[A0][0] = Addr(r(c, S2)).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_c778, 0x001d_209c)?;
        c.g[A0][0] = Addr(r(c, S2)).off(0x40).lw(c);
        c.g[A2][0] = 1;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_c870, 0x001d_20ac)?;
        c.g[A0][0] = Addr(r(c, S2)).off(0x10).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx(p(r(c, A0), 0x18));
        call_ptr(c, 0x0039_f190, 0x001d_20bc)?;
        c.g[V0][0] = 1;
        preview_reward_tail(c, frame)
    })
}

/// The `lq s0-s2; ld ra; addiu sp; jr ra` tail.
fn preview_reward_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, 16, 0x30);
    frame.restore(c, 17, 0x20);
    frame.restore(c, 18, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void OVScreenInGameError_Update(OVScreenInGameError* self)` (0x1faf10):
/// run the overlay's common update, then, while the online error is armed,
/// check the network timeout / disconnect / ping state and either show the
/// timeout or disconnect message, or hide the "ping" object; finally refresh
/// the cancel-game button and clear the armed flag.
pub fn ovscreen_in_game_error_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0020_e900, 0x001f_af24)?;

        c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
        if c.g[V1][0] == 0 {
            frame.restore(c, 16, 0x10);
            return game_error_tail(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r(c, S0)).off(0x9c).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x10);
            return game_error_tail(c, frame);
        }

        // a0 = (p->+0x138 <= 0 && p->+0x13c == 0): the ping timed out.
        c.cycles += 3;
        c.g[V0][0] = Addr(r(c, V1)).off(0x138).lw(c);
        let positive = (c.g[V0][0] as i64) > 0;
        c.g[A0][0] = 0;
        if !positive {
            c.cycles += 2;
            c.g[V0][0] = Addr(r(c, V1)).off(0x13c).lw(c);
            c.g[A0][0] = (c.g[V0][0] < 1) as u64;
        }

        'outer: {
            'reply: {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(r(c, V1)).off(0x13c).lw(c);
                    break 'reply;
                }
                c.cycles += 5;
                c.sw(Addr(r(c, V1)).off(0x9c).0, 0);
                c.g[A0][0] = c.g[S0][0];
                callee::GAME_ERROR_SERVER_TIMEOUT.call(c, 0x001f_af64)?;
                c.g[A0][0] = c.g[S0][0];
                break 'outer;
            }

            c.cycles += 2;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0x10);
                return game_error_tail(c, frame);
            }

            // a0 = (p->+0x80 != 0 && p->+0xa0 == 0 && p->+0x24 == 0).
            'received: {
                c.cycles += 3;
                c.g[V0][0] = Addr(r(c, V1)).off(0x80).lw(c);
                let none = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                if none {
                    break 'received;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r(c, V1)).off(0xa0).lw(c);
                let busy = c.g[V0][0] != 0;
                if busy {
                    break 'received;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(r(c, V1)).off(0x24).lw(c);
                c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            }

            'message: {
                c.cycles += 2;
                let not_received = c.g[A0][0] == 0;
                c.g[A0][0] = 0x470000;
                if not_received {
                    break 'message;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::GAME_ERROR_OPPONENT_DISCONNECTED.call(c, 0x001f_afa4)?;
                c.g[A0][0] = c.g[S0][0];
                break 'outer;
            }

            c.cycles += 12;
            c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x4e8));
            call_ptr(c, 0x0031_7670, 0x001f_afb4)?;
            c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_afc0)?;
            c.g[V1][0] = Addr(r(c, V0)).off(0x8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(r(c, V1)).off(0x48).lh(c);
            c.g[A2][0] = Addr(r(c, V1)).off(0x4c).lw(c);
            let target = r(c, A2);
            c.g[A0][0] = sx(p(r(c, V0), r(c, A0)));
            call_ptr(c, target, 0x001f_afd8)?;
            c.g[A0][0] = c.g[S0][0];
        }

        c.cycles += 4;
        c.g[A1][0] = 1;
        callee::GAME_ERROR_SHOW_CANCEL.call(c, 0x001f_afe4)?;
        c.sw(Addr(r(c, S0)).off(0x9c).0, 0);
        frame.restore(c, 16, 0x10);
        game_error_tail(c, frame)
    })
}

/// The `ld ra; addiu sp; jr ra` tail.
fn game_error_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// BE rewards.

/// `void BERewardInt_UnlockVideo(BERewardInt* self, int profile, int c,
/// int idx, int award)` (0x158470): mark video reward `idx` as seen in the
/// profile's video bit table and, when awarded, add its point value (ten
/// times the reward's score) to the player's money.
pub fn bereward_int_unlock_video(c: &mut Cpu) {
    guest(c, |c| {
        bereward_unlock(
            c,
            0x0015_8470,
            0x24,
            0xf52,
            0x0015_84a8,
            0x0015_84b4,
            0x0015_84e8,
            0x0015_84f8,
            0x0015_8524,
            0x0015_8534,
        )
    })
}

/// `void BERewardInt_UnlockRiderItem(...)` (0x158618): as 0x158470 for rider
/// items; `idx` is offset by ten and the bit table is at `+0x28`.
pub fn bereward_int_unlock_rider_item(c: &mut Cpu) {
    guest(c, |c| {
        bereward_unlock(
            c,
            0x0015_8618,
            0x28,
            0xf57,
            0x0015_8650,
            0x0015_865c,
            0x0015_8690,
            0x0015_86a0,
            0x0015_86cc,
            0x0015_86dc,
        )
    })
}

/// The shared body of the two unlock helpers. `idx` is `a3` (`- 10` for the
/// rider-item variant), `table` the rewards-file field holding the bit table
/// and `bit_off` the byte offset of the bit inside a 0x9b50/0xf88 entry.
#[allow(clippy::too_many_arguments)]
fn bereward_unlock(
    c: &mut Cpu,
    _entry: u32,
    table: u32,
    bit_off: u32,
    ret_inst: u32,
    ret_iface: u32,
    ret_setbit: u32,
    ret_file: u32,
    ret_spend: u32,
    ret_money: u32,
) -> Flow {
    c.cycles += 32;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S1, 0x50);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[T0][0];
    frame.save(c, S3, 0x30);
    c.g[S2][0] = c.g[A1][0];
    frame.save(c, S4, 0x20);
    c.g[S3][0] = c.g[A2][0];
    frame.save(c, S5, 0x10);
    frame.save_ra(c, 0);
    if bit_off == 0xf57 {
        c.g[S4][0] = sx(r(c, A3).wrapping_sub(0xa));
    } else {
        c.g[S4][0] = c.g[A3][0];
    }

    callee::BE_LIBRARY_INSTANCE.call(c, ret_inst)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xb;
    callee::BE_LIBRARY_INTERFACE.call(c, ret_iface)?;

    // econ = interface; entry = D_004A6CA8 + profile * 0x9b50 + c * 0xf88.
    c.g[V1][0] = 0xf88;
    c.g[A2][0] = 0x9b50;
    c.mult(0, V1, S3, V1);
    c.g[S5][0] = c.g[V0][0];
    c.mult(0, A2, S2, A2);
    c.g[V0][0] = 0x4a0000;
    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x6ca8));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
    c.g[A2][0] = sx(p(r(c, A2), r(c, V1)));
    c.g[A2][0] = sx(p(r(c, A2), bit_off));
    callee::BE_REWARD_SET_BIT.call(c, ret_setbit)?;

    let no_award = c.g[S1][0] == 0;
    c.g[A0][0] = c.g[S5][0];
    if !no_award {
        c.cycles += 8;
        callee::BE_REWARD_GET_FILE.call(c, ret_file)?;
        c.g[V1][0] = Addr(r(c, V0)).off(table).lw(c);
        c.g[V0][0] = sx(r(c, S4) << 4);
        c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
        c.g[A3][0] = Addr(r(c, V1)).off(0xc).lh(c);
        let not_scored = (c.g[A3][0] as i64) <= 0;
        c.g[V0][0] = 0xa;
        if !not_scored {
            c.cycles += 1;
            c.mult(0, A3, A3, V0);
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        callee::BE_ECON_SPEND_MONEY.call(c, ret_spend)?;
        c.g[A0][0] = c.g[S5][0];
        return bereward_tail(c, frame);
    }

    c.cycles += 12;
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    callee::BE_ECON_GET_MONEY.call(c, ret_money)?;
    bereward_tail(c, frame)
}

/// The `lq s0-s5; ld ra; addiu sp; jr ra` tail.
fn bereward_tail(c: &mut Cpu, frame: Frame) -> Flow {
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Tag fields.

/// `void TagFieldSetNumber(TagField* record, char* out, ...)` (0x3ebe40):
/// format `value` in decimal into a temporary buffer and store it through the
/// tag field's append/terminate helpers. This one is a resume point list: the
/// two division/character loops can be re-entered after a host-thread unwind,
/// so every `pc` the generated dispatcher can continue at is accepted.
pub fn tag_field_set_number(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_be40 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x160));
                    c.sq(17, p(r(c, SP), 0x140));
                    c.sq(18, p(r(c, SP), 0x130));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, p(r(c, SP), 0x150));
                    c.g[S2][0] = c.g[A1][0];
                    c.sd(p(r(c, SP), 0x120), c.g[RA][0]);
                    c.g[S0][0] = c.g[A3][0];
                    c.g[A1][0] = c.g[SP][0];
                    callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_be68)?;
                    let non_negative = (c.g[S0][0] as i64) >= 0;
                    c.g[A2][0] = c.g[V0][0];
                    pc = if non_negative {
                        0x003e_be80
                    } else {
                        0x003e_be70
                    };
                }
                0x003e_be70 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x2d;
                    c.g[S0][0] = sx(0u32.wrapping_sub(r(c, S0)));
                    c.sb(r(c, A2), c.g[V0][0]);
                    c.g[A2][0] = sx(p(r(c, A2), 0x1));
                    pc = 0x003e_be80;
                }
                0x003e_be80 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(p(r(c, A2), 0x1f));
                    let no_digits = (c.g[S0][0] as i64) <= 0;
                    c.sb(p(r(c, A2), 0x1f), 0);
                    pc = if no_digits { 0x003e_bec8 } else { 0x003e_be8c };
                }
                0x003e_be8c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0xa;
                    c.g[A3][0] = 0;
                    c.div(0, 16, 5);
                    pc = 0x003e_be98;
                }
                0x003e_be98 => {
                    c.cycles += 2;
                    if c.g[A1][0] == c.g[A3][0] {
                        pc = 0x003e_bea0;
                    } else {
                        pc = 0x003e_bea0;
                    }
                }
                0x003e_bea0 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x1));
                    c.g[V0][0] = c.hi[0];
                    c.g[V1][0] = c.lo[0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x30));
                    c.g[S0][0] = c.g[V1][0];
                    c.sb(r(c, A0), c.g[V0][0]);
                    if c.g[A1][0] == c.g[A3][0] {
                        pc = 0x003e_bec0;
                    } else {
                        pc = 0x003e_bec0;
                    }
                }
                0x003e_bec0 => {
                    c.cycles += 2;
                    if (c.g[S0][0] as i64) > 0 {
                        c.div(0, 16, 5);
                        if event_due(c) && deliver_event(c, 0x003e_be98) {
                            return Ok(());
                        }
                        pc = 0x003e_be98;
                    } else {
                        pc = 0x003e_bec8;
                    }
                }
                0x003e_bec8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(r(c, A0));
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(r(c, A0));
                        pc = 0x003e_bee4;
                    } else {
                        pc = 0x003e_bed4;
                    }
                }
                0x003e_bed4 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x1));
                    c.g[V0][0] = 0x30;
                    c.sb(r(c, A0), c.g[V0][0]);
                    pc = 0x003e_bee0;
                }
                0x003e_bee0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu(r(c, A0));
                    pc = 0x003e_bee4;
                }
                0x003e_bee4 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx(p(r(c, A0), 0x1));
                    c.sb(r(c, A2), c.g[V0][0]);
                    c.g[V0][0] = c.lbu(r(c, A0));
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = sx(p(r(c, A2), 0x1));
                    if more {
                        if event_due(c) && deliver_event(c, 0x003e_bee0) {
                            return Ok(());
                        }
                        pc = 0x003e_bee0;
                    } else {
                        pc = 0x003e_befc;
                    }
                }
                0x003e_befc => {
                    c.cycles += 11;
                    c.sb(r(c, A2), 0);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[SP][0];
                    callee::TAG_FIELD_SET_RAW.call(c, 0x003e_bf10)?;
                    frame_lq_3(c, 0x150, 0x140, 0x130);
                    c.g[RA][0] = c.ld(p(r(c, SP), 0x120));
                    let ra = r(c, RA);
                    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x160));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, 0x003e_be40, 0x003e_bf28);
                    return Ok(());
                }
            }
        }
    })
}

/// `lq s0/s1/s2, off(sp)` in that order.
#[inline(always)]
fn frame_lq_3(c: &mut Cpu, a: u32, b: u32, d: u32) {
    c.lq(16, p(r(c, SP), a));
    c.lq(17, p(r(c, SP), b));
    c.lq(18, p(r(c, SP), d));
}

// ---------------------------------------------------------------------------
// Script camera.

/// `ScriptCamController* ScriptCamController_ctor(self, info, script)`
/// (0x168fe8): construct the camera controller and allocate its two script
/// object slots (`+0x1c`), each a script camera bound to the script handle.
pub fn script_cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S5][0] = 0x460000;
        call_ptr(c, 0x0015_cba0, 0x0016_9020)?;

        c.g[V0][0] = 0x460000;
        c.g[V1][0] = 3;
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x3bd8));
        c.g[A1][0] = 0x460000;
        c.sw(Addr(r(c, S2)).off(0x14).0, c.g[V0][0]);
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x4178));
        c.sw(r(c, S2), c.g[V1][0]);
        c.g[A0][0] = 8;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0016_904c)?;
        c.sw(Addr(r(c, S2)).off(0x1c).0, c.g[V0][0]);
        c.g[S1][0] = Addr(r(c, S2)).off(0x1c).lw(c);

        loop {
            c.cycles += 19;
            c.g[S0][0] = sx(r(c, S3) << 2);
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[S1][0] = sx(p(r(c, S1), r(c, S0)));
            c.g[A0][0] = 0x80;
            c.g[A1][0] = sx(r(c, S5).wrapping_sub(0x4160));
            c.g[S3][0] = sx(p(r(c, S3), 0x1));
            call_ptr(c, 0x0031_7d70, 0x0016_9078)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x0027_d5e8, 0x0016_9084)?;
            c.sw(r(c, S1), c.g[V0][0]);
            c.g[V0][0] = Addr(r(c, S2)).off(0x1c).lw(c);
            c.g[S0][0] = sx(p(r(c, S0), r(c, V0)));
            c.g[A0][0] = Addr(r(c, S0)).lw(c);
            call_ptr(c, 0x0028_3298, 0x0016_9098)?;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 2) as u64;
            if r(c, V0) == 0 {
                break;
            }
            c.g[S1][0] = Addr(r(c, S2)).off(0x1c).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0016_9058);
            }
        }

        c.cycles += 11;
        c.g[V0][0] = c.g[S2][0];
        c.sw(Addr(r(c, S2)).off(0x18).0, 0);
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
// Lobby menu pages.

/// `void LobbyMenu_InitDeletePersonaPage(LobbyMenu* self, void* info)`
/// (0x1dabd0): build the delete-persona page: a label item and a spacer, then
/// fill the four persona action items and add them all to the menu.
pub fn lobby_menu_init_delete_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        lobby_menu_init_page(
            c,
            3,
            0x46d5e0,
            0x467a98,
            4,
            0x001d_abd0,
            0x001d_ac00,
            0x001d_ac28,
            0x001d_ac34,
            0x001d_ac44,
            0x001d_ac60,
            0x001d_ac74,
            0x001d_ac88,
        )
    })
}

/// `void LobbyMenu_InitSelectRoomPage(LobbyMenu* self, void* info)`
/// (0x1dae20): as 0x1dabd0 with twelve room items.
pub fn lobby_menu_init_select_room_page(c: &mut Cpu) {
    guest(c, |c| {
        lobby_menu_init_page(
            c,
            11,
            0x46d4d8,
            0x467ab8,
            12,
            0x001d_ae20,
            0x001d_ae50,
            0x001d_ae78,
            0x001d_ae84,
            0x001d_ae94,
            0x001d_aeb0,
            0x001d_aec4,
            0x001d_aed8,
        )
    })
}

/// The shared body of the two page initialisers: `items` action items at
/// `self+0x168` and `count` insert-item calls.
#[allow(clippy::too_many_arguments)]
fn lobby_menu_init_page(
    c: &mut Cpu,
    items: u32,
    vtable: u32,
    label: u32,
    count: u32,
    _entry: u32,
    ret_ctor: u32,
    ret_label: u32,
    ret_spacer: u32,
    ret_action: u32,
    ret_insert1: u32,
    ret_insert2: u32,
    ret_insertn: u32,
) -> Flow {
    c.cycles += 26;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S1, 0x40);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x50);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S2, 0x30);
    frame.save(c, S3, 0x20);
    frame.save(c, S4, 0x10);
    c.g[S2][0] = sx(p(r(c, S1), 0x168));
    frame.save_ra(c, 0);
    c.g[S3][0] = items as u64;
    call_ptr(c, 0x002c_aa58, ret_ctor)?;

    c.g[S4][0] = u64::MAX;
    c.g[V0][0] = vtable as u64;
    c.g[A1][0] = label as u64;
    c.sw(Addr(r(c, S1)).off(0x12c).0, c.g[V0][0]);
    c.g[A0][0] = sx(p(r(c, S1), 0x134));
    c.g[V0][0] = Addr(r(c, S0)).lw(c);
    c.sw(Addr(r(c, S1)).off(0x130).0, c.g[V0][0]);
    call_ptr(c, 0x002c_d2a0, ret_label)?;
    c.g[A0][0] = sx(p(r(c, S1), 0x150));
    c.g[A1][0] = 0xe;
    call_ptr(c, 0x002c_d350, ret_spacer)?;

    'items: {
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx(r(c, S3).wrapping_sub(0x1));
            call_ptr(c, 0x002c_cf08, ret_action)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = sx(p(r(c, S2), 0x24));
            if !more {
                break 'items;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_ac38);
            }
        }
    }

    c.cycles += 11;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = sx(p(r(c, S1), 0x134));
    c.g[A2][0] = u64::MAX;
    c.g[S0][0] = sx(p(r(c, S1), 0x168));
    call_ptr(c, 0x002c_bcc8, ret_insert1)?;

    c.g[S2][0] = count as u64;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = sx(p(r(c, S1), 0x150));
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret_insert2)?;

    c.g[A1][0] = c.g[S0][0];
    loop {
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        c.g[S0][0] = sx(p(r(c, S0), 0x24));
        call_ptr(c, 0x002c_bcc8, ret_insertn)?;
        c.g[S2][0] = sx(r(c, S2).wrapping_sub(0x1));
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[A1][0] = c.g[S0][0];
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_ac78);
        }
    }

    c.cycles += 9;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `cKeyboardPopup* OVScreenReadPrivateMessage_CreateKeyboardPopup(
/// self, arg1, arg2, arg3, arg4)` (0x210fa0): allocate a 0x444-byte keyboard
/// popup, construct it, push it on the screen's state manager, set its
/// max length / key set / function keys, and remember it on the screen.
pub fn ovscreen_read_private_message_create_keyboard_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0];
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x1030));
        c.g[A0][0] = 0x444;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0021_0fe8)?;

        c.g[A1][0] = Addr(r(c, S0)).off(0x10).lw(c);
        c.g[T0][0] = 0xf;
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_b030, 0x0021_1000)?;

        c.g[V1][0] = Addr(r(c, S0)).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.sw(Addr(r(c, S0)).off(0xe0).0, c.g[V0][0]);
        c.g[V0][0] = Addr(r(c, V1)).off(0xd0).lw(c);
        c.g[A0][0] = Addr(r(c, V0)).off(0x10).lw(c);
        c.g[A0][0] = sx(p(r(c, A0), 0x18));
        call_ptr(c, 0x0039_f290, 0x0021_101c)?;

        c.g[V0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.sw(Addr(r(c, V0)).off(0x18).0, c.g[S3][0]);
        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        call_ptr(c, 0x001c_d088, 0x0021_1030)?;

        c.g[V0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A2][0] = 1;
        c.g[A3][0] = 2;
        c.g[A1][0] = 0x4b;
        c.sw(Addr(r(c, V0)).off(0x43c).0, 0);
        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        call_ptr(c, 0x001c_e3c8, 0x0021_104c)?;

        c.g[A0][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[A1][0] = 1;
        call_ptr(c, 0x001c_e468, 0x0021_1058)?;

        c.g[V1][0] = Addr(r(c, S0)).off(0xe0).lw(c);
        c.g[V0][0] = 1;
        c.lq(17, p(r(c, SP), 0x40));
        c.sw(Addr(r(c, V1)).off(0x440).0, c.g[V0][0]);
        c.sw(Addr(r(c, S0)).off(0xdc).0, c.g[S4][0]);
        c.lq(16, p(r(c, SP), 0x50));
        c.lq(18, p(r(c, SP), 0x30));
        c.lq(19, p(r(c, SP), 0x20));
        c.lq(20, p(r(c, SP), 0x10));
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input parser.

/// `int InputParser_LookupButton(const char* name)` (0x3216e8): parse
/// `"buttonNN[aA]?"` and return the button index, or -1 when the name does
/// not have the `button` prefix, has no digits, ends in a letter, or is `_`.
pub fn input_parser_lookup_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 6;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3ef0));
        call_ptr(c, 0x0041_ab78, 0x0032_170c)?;

        let mismatch = r(c, V0) != 0;
        c.g[V0][0] = u64::MAX;
        if mismatch {
            return input_parser_tail(c, frame);
        }
        c.cycles += 6;
        c.g[V1][0] = c.lb(Addr(r(c, S0)).off(0x6).0);
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x30));
        c.g[V0][0] = c.g[V1][0] & 0xff;
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        let no_digit = c.g[V0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if no_digit {
            return input_parser_tail(c, frame);
        }

        'number: {
            c.cycles += 6;
            c.g[V0][0] = c.lb(Addr(r(c, S0)).off(0x7).0);
            c.g[A2][0] = c.g[V1][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let no_more = c.g[V0][0] == 0;
            c.g[A1][0] = sx(p(r(c, S0), 0x7));
            if no_more {
                break 'number;
            }
            c.cycles += 1;
            c.g[A3][0] = 0xa;
            loop {
                c.cycles += 9;
                c.mult(0, V1, A2, A3);
                c.g[A0][0] = c.lb(r(c, A1));
                c.g[A1][0] = sx(p(r(c, A1), 0x1));
                c.g[V0][0] = c.lb(r(c, A1));
                c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x30));
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(p(r(c, V1), r(c, A0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1748);
                }
            }
        }

        'alpha: {
            'inner: {
                c.cycles += 6;
                c.g[V1][0] = c.lb(r(c, A1));
                c.g[A3][0] = 0;
                c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x61));
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let lower = c.g[V0][0] != 0;
                c.g[A0][0] = c.lbu(r(c, A1));
                if lower {
                    break 'inner;
                }
                c.cycles += 4;
                c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x41));
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let not_upper = c.g[V0][0] == 0;
                if not_upper {
                    break 'alpha;
                }
            }
            c.cycles += 1;
            c.g[A3][0] = 1;
        }

        c.cycles += 2;
        let letter = c.g[A3][0] != 0;
        c.g[V0][0] = u64::MAX;
        if letter {
            return input_parser_tail(c, frame);
        }

        'end: {
            c.cycles += 5;
            c.g[V0][0] = sx(r(c, A0) << 24);
            c.g[V1][0] = 0x5f;
            c.g[V0][0] = sx((r(c, V0) as i32 >> 24) as u32);
            let underscore = c.g[V0][0] == c.g[V1][0];
            c.g[V1][0] = ((c.g[A2][0] as i64) < 924) as u64;
            if underscore {
                break 'end;
            }
            c.cycles += 2;
            let small = c.g[V1][0] != 0;
            c.g[V0][0] = c.g[A2][0];
            if small {
                return input_parser_tail(c, frame);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = u64::MAX;
        input_parser_tail(c, frame)
    })
}

/// The `lq s0; ld ra; addiu sp; jr ra` tail.
fn input_parser_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Bragging rights screen.

/// `void cFEStateBraggingRights_OnActivate(cFEStateBraggingRights* self)`
/// (0x193568): when the screen is enabled, play the intro or the idle frame
/// animation depending on the state object's fields, then refresh both row
/// colour tables; when disabled, hand activation to the net state.
pub fn cfe_state_bragging_rights_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x1680)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let on = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];

        'outer: {
            if !on {
                c.cycles += 4;
                call_ptr(c, 0x001a_87d0, 0x0019_358c)?;
                frame.restore(c, 16, 0x10);
                break 'outer;
            }

            'frames: {
                'first: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_e4c0, 0x0019_359c)?;
                    c.g[V0][0] = Addr(r(c, S0)).off(0x6f8).lw(c);
                    let idle = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(r(c, S0)).off(0x72c).lw(c);
                    if idle {
                        break 'first;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(r(c, S0)).off(0x6f8).lw(c);
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = 0x4a1790;
                        call_ptr(c, 0x0031_7670, 0x0019_35bc)?;
                        c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0019_35c8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let missing = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if missing {
                            break 'frames;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x0019_35e4)?;
                        break 'frames;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        break 'frames;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r(c, S0)).off(0x72c).lw(c);
                }

                c.cycles += 2;
                let idle = c.g[V0][0] == 0;
                c.g[A0][0] = 0x4a0000;
                if idle {
                    break 'frames;
                }
                c.cycles += 9;
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x1798));
                call_ptr(c, 0x0031_7670, 0x0019_3608)?;
                c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x0019_3614)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let missing = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if missing {
                    break 'frames;
                }
                c.cycles += 3;
                c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0019_3630)?;
            }

            c.cycles += 5;
            c.g[A0][0] = sx(p(r(c, S0), 0x6d0));
            callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS.call(c, 0x0019_3638)?;
            c.g[A0][0] = sx(p(r(c, S0), 0x704));
            callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS.call(c, 0x0019_3640)?;
            frame.restore(c, 16, 0x10);
        }

        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Pause menu.

/// `void OVScreenPauseFreeride_PlayIconAnim(self, idx, on)` (0x1f8df8): format
/// the icon animation's label, look its frame up on the screen and either
/// start it (remembering the playing frame) or retarget the running
/// animation.
pub fn ovscreen_pause_freeride_play_icon_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        let on = c.g[A2][0] != 0;
        c.g[S0][0] = c.g[A0][0];

        'outer: {
            if !on {
                c.cycles += 18;
                c.g[V0][0] = 0x440000;
                c.g[V1][0] = sx(r(c, A1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(0x1c30));
                c.g[A1][0] = 0x4a0000;
                c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = Addr(r(c, V1)).lw(c);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(0x2490));
                call_ptr(c, 0x0041_7828, 0x001f_8e98)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x001f_8ea0)?;
                c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001f_8eac)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = 0xffff;
                if c.g[V1][0] == c.g[A0][0] {
                    frame.restore(c, 16, 0x50);
                    break 'outer;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S0)).off(0xc4).lw(c);
                c.sh(Addr(r(c, V0)).off(0xc).0, c.g[V1][0]);
                c.g[V1][0] = Addr(r(c, S0)).off(0xc4).lw(c);
                c.sh(Addr(r(c, V1)).off(0xe).0, c.g[A0][0]);
                c.sw(Addr(r(c, S0)).off(0xc4).0, 0);
                c.cycles += 1;
                frame.restore(c, 16, 0x50);
                break 'outer;
            }

            c.cycles += 18;
            c.g[V0][0] = 0x440000;
            c.g[V1][0] = sx(r(c, A1) << 2);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x1c30));
            c.g[A1][0] = 0x4a0000;
            c.g[V1][0] = sx(p(r(c, V1), r(c, V0)));
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = Addr(r(c, V1)).lw(c);
            c.g[A1][0] = sx(r(c, A1).wrapping_add(0x2488));
            call_ptr(c, 0x0041_7828, 0x001f_8e30)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_8e38)?;
            c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001f_8e44)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            if c.g[A1][0] == c.g[V0][0] {
                frame.restore(c, 16, 0x50);
                break 'outer;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r(c, S0)).off(0xc4).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x50);
                break 'outer;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_c870, 0x001f_8e6c)?;
            c.sw(Addr(r(c, S0)).off(0xc4).0, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, 16, 0x50);
        }

        c.cycles += 3;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE character queries.

/// `int BECharInt_IsTrickScoreAboveRating(char* self, void* rider, int value)`
/// (0x149778): true when `value` is at least the rating threshold of the
/// rider's overall rating level (clamped to the level table).
pub fn bechar_int_is_trick_score_above_rating(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD.call(c, 0x0014_97a0)?;

        let shared = r(c, V0) != 0;
        c.g[V0][0] = 0;
        if shared {
            return bechar_rating_tail(c, frame);
        }

        c.cycles += 30;
        c.g[V0][0] = 0x460000;
        c.g[A0][0] = c.g[S1][0];
        c.g[T0][0] = sx(r(c, V0).wrapping_sub(0x5d50));
        // Copy the 0x2c-byte rating table to the stack.
        c.ldl(3, p(r(c, T0), 0x7));
        c.ldr(3, r(c, T0));
        c.ldl(6, p(r(c, T0), 0xf));
        c.ldr(6, p(r(c, T0), 0x8));
        c.ldl(7, p(r(c, T0), 0x17));
        c.ldr(7, p(r(c, T0), 0x10));
        c.sdl(3, p(r(c, SP), 0x7));
        c.sdr(3, r(c, SP));
        c.sdl(6, p(r(c, SP), 0xf));
        c.sdr(6, p(r(c, SP), 0x8));
        c.sdl(7, p(r(c, SP), 0x17));
        c.sdr(7, p(r(c, SP), 0x10));
        c.ldl(3, p(r(c, T0), 0x1f));
        c.ldr(3, p(r(c, T0), 0x18));
        c.ldl(6, p(r(c, T0), 0x27));
        c.ldr(6, p(r(c, T0), 0x20));
        c.g[A3][0] = Addr(p(r(c, T0), 0x28)).lw(c);
        c.sdl(3, p(r(c, SP), 0x1f));
        c.sdr(3, p(r(c, SP), 0x18));
        c.sdl(6, p(r(c, SP), 0x27));
        c.sdr(6, p(r(c, SP), 0x20));
        c.sw(p(r(c, SP), 0x28), c.g[A3][0]);
        c.g[A1][0] = c.g[S0][0];
        callee::BECHAR_GET_OVERALL_RATING.call(c, 0x0014_9814)?;

        c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x1));
        let negative = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = 0xa;
        let level;
        if negative {
            level = 0;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A0][0];
            }
            level = 1;
        }
        if level == 0 {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }

        c.cycles += 5;
        c.g[V0][0] = sx(r(c, V1) << 2);
        c.g[V1][0] = sx(p(r(c, SP), r(c, V0)));
        c.g[V0][0] = Addr(r(c, V1)).lw(c);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 1;
        bechar_rating_tail(c, frame)
    })
}

/// The `lq s0-s2; ld ra; addiu sp; jr ra` tail.
fn bechar_rating_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Message center.

/// `void MessageCenter_PostPeakUnlockMessage(self, which)` (0x1e2428): while
/// the current peak is one of the first ten, do nothing; otherwise set the
/// per-slot "seen" flag (`0x39`/`0x3a`/`0x3b`) and post the corresponding
/// unlock message (`0xfc`/`0xfd`/`0xfe`).
pub fn message_center_post_peak_unlock_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(r(c, V0)).off(0x84).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x28).lw(c);
        c.g[A0][0] = Addr(r(c, A0)).lw(c);
        let no_peak = c.g[A0][0] == 0;
        c.g[A2][0] = 0;
        let busy: bool;
        if no_peak {
            busy = false;
        } else {
            c.cycles += 1;
            c.g[A2][0] = ((c.g[A0][0] as i64) < 10) as u64;
            busy = r(c, A2) != 0;
        }

        c.cycles += 2;
        'outer: {
            if busy {
                break 'outer;
            }
            'b_1e24e0: {
                'b_1e24d0: {
                    'b_1e24b0: {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        let is_one = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                        if is_one {
                            break 'b_1e24b0;
                        }
                        'b_1e248c: {
                            'b_1e247c: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = 2;
                                    break 'b_1e247c;
                                }
                                c.cycles += 2;
                                if c.g[A1][0] == 0 {
                                    break 'b_1e248c;
                                }
                                c.cycles += 2;
                                break 'outer;
                            }
                            c.cycles += 2;
                            let is_two = c.g[A1][0] == c.g[V0][0];
                            if !is_two {
                                c.cycles += 2;
                                break 'outer;
                            }
                            break 'b_1e24e0;
                        }

                        // which == 0: flag 0x39, message 0xfc.
                        c.cycles += 4;
                        c.g[A0][0] = 0x39;
                        callee::MESSAGE_IS_FLAG_SET.call(c, 0x001e_2494)?;
                        if r(c, V0) != 0 {
                            break 'outer;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = 0x39;
                        c.g[A1][0] = 1;
                        callee::MESSAGE_SET_FLAG.call(c, 0x001e_24a8)?;
                        c.g[A0][0] = 0xfc;
                        break 'b_1e24d0;
                    }

                    // which == 1: flag 0x3a, message 0xfd.
                    c.cycles += 4;
                    c.g[A0][0] = 0x3a;
                    callee::MESSAGE_IS_FLAG_SET.call(c, 0x001e_24b8)?;
                    if r(c, V0) != 0 {
                        break 'outer;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 0x39;
                    c.g[A1][0] = 1;
                    callee::MESSAGE_SET_FLAG.call(c, 0x001e_24cc)?;
                    c.g[A0][0] = 0xfd;
                    break 'b_1e24d0;
                }

                // Post the queued message (a0 holds 0xfc or 0xfd).
                c.cycles += 4;
                c.g[A1][0] = 0;
                callee::MESSAGE_POST.call(c, 0x001e_24d8)?;
                break 'outer;
            }

            // which == 2: flag 0x3b, message 0xfe.
            c.cycles += 4;
            c.g[A0][0] = 0x3b;
            callee::MESSAGE_IS_FLAG_SET.call(c, 0x001e_24e8)?;
            if r(c, V0) != 0 {
                break 'outer;
            }
            c.cycles += 7;
            c.g[A0][0] = 0x3b;
            c.g[A1][0] = 1;
            callee::MESSAGE_SET_FLAG.call(c, 0x001e_24fc)?;
            c.g[A0][0] = 0xfe;
            c.g[A1][0] = 0;
            callee::MESSAGE_POST.call(c, 0x001e_2508)?;
        }
        peak_unlock_tail(c, frame)
    })
}

/// The `ld ra; addiu sp; jr ra` tail.
fn peak_unlock_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Tex-flip modifier.

/// `void TexFlipModifier_HandleEvent(TexFlipModifier* self, int id, void* v)`
/// (0x35f598): event 300 stores the speed, 301 selects a frame from a float
/// (rounding up) and 302 advances to the next frame cyclically.
pub fn texflip_modifier_handle_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = 0x12d;
        let is301 = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);

        'outer: {
            if is301 {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_le_s(0, 12);
                if !c.fcc() {
                    break 'outer;
                }
                c.cycles += 5;
                c.g[AT][0] = 0x4110_0000;
                c.mtc1(1, 0);
                c.c_lt_s(12, 0);
                if !c.fcc() {
                    break 'outer;
                }
                'round: {
                    c.cycles += 5;
                    c.cvt_w_s(1, 12);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(1, 12);
                    if !c.fcc() {
                        break 'round;
                    }
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.add_s(1, 1, 0);
                }
                c.cycles += 8;
                c.cvt_w_s(0, 1);
                c.mfc1(2, 0);
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = c.g[V0][0];
                c.sw(Addr(r(c, A2)).off(0x14).0, c.g[V0][0]);
                callee::TEXFLIP_SET_FRAME.call(c, 0x0035_f644)?;
                break 'outer;
            }

            'switch: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 302) as u64;
                let large = c.g[V0][0] == 0;
                c.g[V0][0] = 0x12c;
                if !large {
                    c.cycles += 2;
                    let is300 = c.g[A1][0] == c.g[V0][0];
                    frame.restore_ra(c, 0);
                    if !is300 {
                        c.cycles += 2;
                        break 'outer;
                    }
                    // event 300: speed = v
                    c.cycles += 2;
                    c.swc1(12, Addr(r(c, A2)).off(0x8).0);
                    break 'switch;
                }

                c.cycles += 3;
                c.g[V0][0] = 0x12e;
                let is302 = c.g[A1][0] == c.g[V0][0];
                frame.restore_ra(c, 0);
                if !is302 {
                    c.cycles += 2;
                    break 'outer;
                }

                // event 302: next frame, cycling at count.
                'advance: {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r(c, A2)).off(0x14).lw(c);
                    c.g[V1][0] = Addr(r(c, A2)).off(0x20).lw(c);
                    c.g[V0][0] = sx(p(r(c, V0), 0x1));
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let inside = c.g[V1][0] != 0;
                    c.sw(Addr(r(c, A2)).off(0x14).0, c.g[V0][0]);
                    if !inside {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        c.sw(Addr(r(c, A2)).off(0x14).0, c.g[V0][0]);
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = Addr(r(c, A0)).off(0x14).lw(c);
                callee::TEXFLIP_SET_FRAME.call(c, 0x0035_f678)?;
                break 'switch;
            }
            c.cycles += 1;
        }

        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby quick play.

/// `void LobbyMan_OnQuickPlayReply(LobbyMan* self, void* msg)` (0x25f030):
/// a null reply moves state 0x12 to 0x14; otherwise enter state 1, allocate
/// the reply message record and link it into the message list.
pub fn lobby_man_on_quick_play_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = Addr(r(c, V1)).off(0x8).lw(c);
        let empty = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A0][0];

        if !empty {
            c.cycles += 4;
            c.g[V1][0] = Addr(r(c, S1)).lw(c);
            c.g[V0][0] = 0x12;
            let other = c.g[V1][0] != c.g[V0][0];
            frame.restore(c, 16, 0x50);
            if other {
                frame.restore(c, 17, 0x40);
                frame.restore(c, 18, 0x30);
                c.cycles += 5;
                frame.restore_ra(c, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            c.sw(r(c, S1), c.g[V0][0]);
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            c.cycles += 5;
            frame.restore_ra(c, 0x20);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 28;
        c.g[V0][0] = 1;
        c.g[A1][0] = 0x480488;
        c.sw(r(c, S1), c.g[V0][0]);
        c.g[A3][0] = 0;
        c.g[A0][0] = 0xc;
        c.g[S0][0] = Addr(r(c, V1)).off(0x8).lw(c);
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0025_f094)?;

        c.g[S2][0] = c.g[V0][0];
        c.g[A3][0] = Addr(r(c, S1)).off(0xf0).lw(c);
        c.g[V0][0] = 0x481290;
        c.g[V1][0] = 0x102;
        c.sw(r(c, S2), c.g[V1][0]);
        c.sw(p(r(c, S2), 0x4), c.g[V0][0]);
        c.sw(p(r(c, S2), 0x8), c.g[S0][0]);
        c.sw(p(r(c, SP), 0x10), c.g[A3][0]);
        c.g[A0][0] = 0xc;
        c.g[A1][0] = 0x4804a8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_f0d0)?;

        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(p(r(c, A1), 0x8));
        if c.g[V0][0] != 0 {
            c.sw(Addr(r(c, A1)).off(0x8).0, c.g[S2][0]);
        }

        c.cycles += 10;
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        c.sw(r(c, A1), c.g[V0][0]);
        c.g[V1][0] = Addr(r(c, V0)).off(0x4).lw(c);
        c.sw(Addr(r(c, A1)).off(0x4).0, c.g[V1][0]);
        c.g[A0][0] = Addr(r(c, V0)).off(0x4).lw(c);
        c.sw(r(c, A0), c.g[A1][0]);
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        c.sw(Addr(r(c, V0)).off(0x4).0, c.g[A1][0]);
        c.sw(r(c, SP), c.g[A1][0]);
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE medals.

/// `int BERewardInt_GetMedalRewardId(int self, int a1, int kind, int tier)`
/// (0x159080): the reward id for a medal: `5 + tier*3 + kind` for tiers 0-3
/// and kinds 0-2, -1 otherwise.
pub fn bereward_int_get_medal_reward_id(c: &mut Cpu) {
    // A leaf with no frame; tier = t0, kind = a3.
    'outer: {
        'b_159160: {
            'b_15915c: {
                'b_159138: {
                    'b_159110: {
                        'b_1590e8: {
                            c.cycles += 3;
                            c.g[V1][0] = 1;
                            let tier_one = c.g[T0][0] == c.g[V1][0];
                            c.g[A0][0] = u64::MAX;
                            if tier_one {
                                break 'b_1590e8;
                            }
                            'b_1590c0: {
                                'b_1590a8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[T0][0] as i64) < 2) as u64;
                                    let ge_two = c.g[V0][0] == 0;
                                    c.g[V0][0] = 2;
                                    if ge_two {
                                        break 'b_1590a8;
                                    }
                                    c.cycles += 2;
                                    let tier_zero = c.g[T0][0] == 0;
                                    c.g[V0][0] = c.g[A0][0];
                                    if tier_zero {
                                        break 'b_1590c0;
                                    }
                                    c.cycles += 2;
                                    break 'outer;
                                }
                                c.cycles += 2;
                                let tier_two = c.g[T0][0] == c.g[V0][0];
                                c.g[V0][0] = 3;
                                if tier_two {
                                    break 'b_159110;
                                }
                                c.cycles += 2;
                                let tier_three = c.g[T0][0] == c.g[V0][0];
                                c.g[V0][0] = c.g[A0][0];
                                if tier_three {
                                    break 'b_159138;
                                }
                                c.cycles += 2;
                                break 'outer;
                            }

                            // tier 0
                            c.cycles += 2;
                            if c.g[A3][0] != 0 {
                                c.cycles += 2;
                                let not_one = c.g[A3][0] != c.g[V1][0];
                                c.g[V1][0] = c.g[A3][0] ^ 2;
                                if not_one {
                                    c.cycles += 2;
                                    c.g[V0][0] = 7;
                                    break 'b_15915c;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 6;
                                break 'b_159160;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 5;
                            break 'b_159160;
                        }

                        // tier 1
                        c.cycles += 2;
                        if c.g[A3][0] != 0 {
                            c.cycles += 2;
                            let not_one = c.g[A3][0] != c.g[V1][0];
                            c.g[V1][0] = c.g[A3][0] ^ 2;
                            if not_one {
                                c.cycles += 2;
                                c.g[V0][0] = 0xa;
                                break 'b_15915c;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 9;
                            break 'b_159160;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 8;
                        break 'b_159160;
                    }

                    // tier 2
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        c.cycles += 2;
                        let not_one = c.g[A3][0] != c.g[V1][0];
                        c.g[V1][0] = c.g[A3][0] ^ 2;
                        if not_one {
                            c.cycles += 2;
                            c.g[V0][0] = 0xd;
                            break 'b_15915c;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0xc;
                        break 'b_159160;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0xb;
                    break 'b_159160;
                }

                // tier 3
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    c.cycles += 2;
                    let not_one = c.g[A3][0] != c.g[V1][0];
                    c.g[V1][0] = c.g[A3][0] ^ 2;
                    if not_one {
                        c.cycles += 1;
                        c.g[V0][0] = 0x10;
                        break 'b_15915c;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0xf;
                    break 'b_159160;
                }
                c.cycles += 2;
                c.g[A0][0] = 0xe;
                break 'b_159160;
            }
            c.cycles += 1;
            if c.g[V1][0] == 0 {
                c.g[A0][0] = c.g[V0][0];
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
    }
    c.cycles += 2;
    let ra = r(c, RA);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// BE new-player flags.

/// `void BENewPlayerInt_SetMessageFlag(char* self, void* rider, int idx,
/// int set)` (0x147a30): set (or clear) the "new player message" bit for the
/// rider's slot in the profile's flags table.
pub fn benewplayer_int_set_message_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);

        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_7a5c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::BENEWPLAYER_GET_SLOT.call(c, 0x0014_7a68)?;

        // entry = D_004A6CA8 + profile * 0x9b50 + slot * 0xf88
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = 0x530000;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4fe0));
        c.g[A0][0] = 0x9b50;
        c.g[A2][0] = 0xf88;
        c.mult(0, S0, S0, A0);
        c.g[V1][0] = u64::MAX;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.g[A0][0] = sx(p(r(c, S2), 0x1f));
        c.g[V0][0] = sx(p(r(c, V0), r(c, A1)));
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.g[S2][0];
        }
        c.g[A1][0] = c.lb(p(r(c, V0), 0x11));
        c.g[A0][0] = sx((r(c, A0) as i32 >> 5) as u32);
        c.g[V0][0] = 0x4a0000;
        c.g[V1][0] = sx(r(c, A0) << 5);
        c.mult(0, A1, A1, A2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(0x6ca8));
        c.g[S2][0] = sx(r(c, S2).wrapping_sub(r(c, V1)));
        c.g[S0][0] = sx(p(r(c, S0), r(c, A1)));
        c.g[S0][0] = sx(p(r(c, S0), r(c, V0)));
        let clear = c.g[S3][0] == 0;
        c.g[S0][0] = sx(p(r(c, S0), 0xf08));
        c.cycles += 7;
        c.g[A0][0] = sx(r(c, A0) << 2);
        c.g[V1][0] = 1;
        c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
        c.g[V1][0] = sx(r(c, V1) << (r(c, S2) & 31));
        c.g[V0][0] = Addr(r(c, A0)).lw(c);
        if clear {
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        } else {
            c.g[V1][0] = !(0u64 | c.g[V1][0]);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        }
        c.cycles += 8;
        c.sw(r(c, A0), c.g[V0][0]);
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
// Plant sway matrices.

/// `void PS2GraphicsMan_v94(PS2GraphicsMan* self, float* mtx)` (0x36a2c0):
/// multiply the object's four sway matrices by the caller's matrix and write
/// them back (VU0, in place).
pub fn ps2_graphics_man_v94(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[V0][0] = Addr(r(c, A0)).off(0xa4).lw(c);
        let none: bool = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if none {
            c.cycles += 4;
            frame.restore(c, 16, 0xa0);
            frame.restore_ra(c, 0x90);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 49;
        c.lq(2, p(r(c, S0), 0x30));
        c.g[A0][0] = sx(p(r(c, SP), 0x40));
        c.sq(2, p(r(c, SP), 0x40));
        callee::VFX_PLANTS_GET_SWAY_MATRIX.call(c, 0x0036_a2e8)?;

        c.g[V1][0] = sx(p(r(c, SP), 0x50));
        c.lqc2(4, r(c, S0));
        c.lqc2(5, p(r(c, S0), 0x10));
        c.lqc2(6, p(r(c, S0), 0x20));
        c.lqc2(7, p(r(c, S0), 0x30));
        c.lqc2(8, r(c, V0));
        c.lqc2(9, p(r(c, V0), 0x10));
        c.lqc2(10, p(r(c, V0), 0x20));
        c.lqc2(11, p(r(c, V0), 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, r(c, V1));
        c.sqc2(13, p(r(c, V1), 0x10));
        c.sqc2(14, p(r(c, V1), 0x20));
        c.sqc2(15, p(r(c, V1), 0x30));
        c.lqc2(1, r(c, V1));
        c.lqc2(2, p(r(c, V1), 0x10));
        c.lqc2(3, p(r(c, V1), 0x20));
        c.lqc2(4, p(r(c, V1), 0x30));
        c.sqc2(1, r(c, SP));
        c.sqc2(2, p(r(c, SP), 0x10));
        c.sqc2(3, p(r(c, SP), 0x20));
        c.sqc2(4, p(r(c, SP), 0x30));
        c.lqc2(1, r(c, SP));
        c.lqc2(2, p(r(c, SP), 0x10));
        c.lqc2(3, p(r(c, SP), 0x20));
        c.lqc2(4, p(r(c, SP), 0x30));
        c.sqc2(1, r(c, S0));
        c.sqc2(2, p(r(c, S0), 0x10));
        c.sqc2(3, p(r(c, S0), 0x20));
        c.sqc2(4, p(r(c, S0), 0x30));

        c.cycles += 4;
        frame.restore(c, 16, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{seed_count, Rng, DATA, GAME_GP};

    /// The zeroed BELibrary dummy and the world singleton.
    const BE: u32 = DATA + 0x1_0000;
    const W: u32 = DATA + 0x1_4000;
    /// `{ target, v0, f0 }` overrides applied after a fake call; `f0` ==
    /// `KEEP_F0` leaves the fake's value alone.
    const HOOKS: u32 = DATA + 0x7_0000;
    const KEEP_F0: u32 = 0xdead_beef;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The `miss` hook: the shared fake, then the queued per-target results.
    fn hook(c: &mut Cpu, target: u32) {
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, target, ret);
        if c.pc != ret {
            return;
        }
        let mut p = HOOKS;
        loop {
            let t = c.bus.read32(p);
            if t == 0 {
                return;
            }
            if t == target {
                c.g[V0][0] = c.bus.read32(p + 4) as u64;
                let f0 = c.bus.read32(p + 8);
                if f0 != KEEP_F0 {
                    c.f[0] = f0;
                }
                return;
            }
            p += 0xc;
        }
    }

    /// Queue the results an indirect call to `target` returns.
    fn force(c: &mut Cpu, slot: u32, target: u32, v0: u32, f0: u32) {
        let p = HOOKS + 0xc * slot;
        w(c, p, target);
        w(c, p + 4, v0);
        w(c, p + 8, f0);
        w(c, p + 0xc, 0);
    }

    /// Call a setup helper (a generated function) with up to six arguments.
    fn call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        c.g[V0][0]
    }

    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
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
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// The dlmalloc arena the named allocators fall back to.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn allocator(c: &mut Cpu) {
        pool_allocator(c);
        call(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// Strict comparison: hard, soft and LO/HI must all match.
    fn strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let report = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(
                report.hard.is_empty(),
                "seed {seed}: hard {:#?}",
                report.hard
            );
            assert!(
                report.soft.is_empty(),
                "seed {seed}: soft {:#?}",
                report.soft
            );
            assert!(
                report.lohi.is_empty(),
                "seed {seed}: LO/HI {:#?}",
                report.lohi
            );
        }
    }

    /// Scrambled registers, a game-like `gp`, the BE singleton, a world with
    /// no riders and the result-queueing miss hook.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = hook;
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, GAME_GP.wrapping_sub(0x848), W);
        let game = W + 0x1000;
        w(c, W + 0x84, game);
        let race = W + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x78, 0);
        r
    }

    // ------------------------------------------------------------ DJ speech

    #[test]
    fn ssx_audio_system_dj_say20e5_matches() {
        strict(
            ee::s08::f_002a26f0__generated,
            ssx_audio_system_dj_say20e5,
            120,
            &|c, seed| {
                let mut r = base(c, seed, 0x2a26, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = ((seed / 2) % 2) as u64;
                // IsDJSpeechOn is "speech enabled" when the 0x2 type's float at
                // self+0x62dc is non-zero (and the replay query is quiet).
                w(
                    c,
                    DATA + 0x62dc,
                    if seed % 3 == 0 { 0 } else { r.edge_f32() | 1 },
                );
                w(c, W + 0x1000 + 0x28, 0);
                // The pending word clear.
                w(c, DATA + 0x5794, if seed % 2 == 0 { 0 } else { 7 });
                // The speech object at self+0x5560: an entry matching the
                // 0x20e5 context makes SetEventContext return 0.
                let speech = DATA + 0x5560;
                w(c, speech + 0x18, (seed % 2) as u32);
                w(c, speech + 0x20, 0);
                w(
                    c,
                    speech + 0x24,
                    if seed % 3 == 0 { r.next_u32() } else { 0x20e5 },
                );
                w(c, GAME_GP + 0x173c, FAKE);
            },
        );
    }

    // --------------------------------------------------------- arcade speech

    #[test]
    fn ssx_audio_system_arcade_say20a7_matches() {
        strict(
            ee::s08::f_002a3b18__generated,
            ssx_audio_system_arcade_say20a7,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x2a3b, 6);
                let obj = DATA + 0x800;
                let v = seed % 6;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (if v == 1 { 0 } else { obj }) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // The animation object: +0x470 is the animation end time
                // (non-negative means the animation has finished).
                w(
                    c,
                    obj + 0x470,
                    if v == 2 { 0x3f80_0000 } else { 0xbf80_0000 },
                );
                // IsArcadeSpeechOn: the 0x20a7 type's float at +0x635c.
                w(c, DATA + 0x635c, if v == 0 { 0 } else { 0x3f80_0000 });
                // IsReplayPlaying: non-zero when world->game->+0x28->+0 is
                // a small index.
                let busy = DATA + 0x5000;
                w(c, W + 0x1000 + 0x28, if v == 5 { busy } else { 0 });
                w(c, busy, 5);
                // 0x285d98 returns the bound object: either race->+0x28 or
                // race->+0x40->+0x18.
                let q = DATA + 0x5100;
                w(c, W + 0x2000 + 0x28, if v == 3 { 0 } else { obj });
                w(c, W + 0x2000 + 0x40, if v == 5 { q } else { 0 });
                w(c, q + 0x18, obj);
                w(c, GAME_GP + 0x173c, FAKE);
                // A matching speech entry makes SetEventContext return 0.
                if seed % 4 == 1 {
                    w(c, DATA + 0x5560 + 0x18, 1);
                    w(c, DATA + 0x5560 + 0x20, 0);
                    w(c, DATA + 0x5560 + 0x24, 0x20a7);
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_arcade_say20a8_matches() {
        strict(
            ee::s08::f_002a3c00__generated,
            ssx_audio_system_arcade_say20a8,
            150,
            &|c, seed| {
                let _ = base(c, seed, 0x2a3c, 6);
                let obj = DATA + 0x800;
                let v = seed % 6;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (if v == 1 { 0 } else { obj }) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                w(
                    c,
                    obj + 0x470,
                    if v == 2 { 0x3f80_0000 } else { 0xbf80_0000 },
                );
                w(c, DATA + 0x635c, if v == 0 { 0 } else { 0x3f80_0000 });
                let busy = DATA + 0x5000;
                w(c, W + 0x1000 + 0x28, if v == 5 { busy } else { 0 });
                w(c, busy, 5);
                let q = DATA + 0x5100;
                w(c, W + 0x2000 + 0x28, if v == 3 { 0 } else { obj });
                w(c, W + 0x2000 + 0x40, if v == 5 { q } else { 0 });
                w(c, q + 0x18, obj);
                w(c, GAME_GP + 0x173c, FAKE);
                if seed % 4 == 1 {
                    w(c, DATA + 0x5560 + 0x18, 1);
                    w(c, DATA + 0x5560 + 0x20, 0);
                    w(c, DATA + 0x5560 + 0x24, 0x20a8);
                }
            },
        );
    }

    // --------------------------------------------------------- script engine

    #[test]
    fn script_engine_spawn_script_matches() {
        strict(
            ee::s07::f_002795d0__generated,
            script_engine_spawn_script,
            90,
            &|c, seed| {
                let mut r = base(c, seed, 0x2795, 0);
                w(c, W + 0x2000 + 0x78, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 1;
                c.g[A2][0] = ((seed * 7) % 7) as u64;
                c.g[A3][0] = ((seed * 11) % 7) as u64;
                c.g[T0][0] = ((seed * 13) % 7) as u64;
                // Script process table: a non-empty entry for the given key
                // makes StartScript return 2 (already running).
                let table = DATA + 0x3000;
                w(c, DATA + 0x2b0, table);
                w(c, table, if seed % 2 == 0 { 1 } else { 0 });
                // AllocProcess: a free slot on some seeds, full otherwise.
                for i in 0..16u32 {
                    w(
                        c,
                        DATA + 0x2e0 + 0x24 * i,
                        if seed % 3 == 0 { 0 } else { (i + 1) as u32 },
                    );
                }
                // The free-list node.
                w(c, DATA + 0x548, DATA + 0x4000);
                w(c, DATA + 0x4000 + 0x10, 0);
                let _ = r.next_u32();
            },
        );
    }

    // ----------------------------------------------------------- tag fields

    #[test]
    fn tag_field_set_number_matches() {
        strict(ee::s14::f_003ebe40__generated, tag_field_set_number, 90, &|c, seed| {
            let _ = base(c, seed, 0x3ebe, 6);
            let value = match seed % 5 {
                0 => 0,
                1 => 7,
                2 => 12345,
                3 => 0x7fff_ffff,
                _ => (seed % 1000) + 1,
            };
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 8;
            c.g[A2][0] = (DATA + 0x900) as u64;
            c.g[A3][0] = if seed % 2 == 0 {
                value as u64
            } else {
                (-(value as i64)) as u64
            };
            for (i, b) in b"name\0".iter().enumerate() {
                c.bus.write8(DATA + 0x900 + i as u32, *b);
            }
            c.pc = 0x003e_be40;
        });
    }

    // ------------------------------------------------------- script camera

    #[test]
    fn script_cam_controller_ctor_matches() {
        strict(
            ee::s02::f_00168fe8__generated,
            script_cam_controller_ctor,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x168f, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed % 4) + 1) as u64;
            },
        );
    }

    // -------------------------------------------------------- lobby pages

            #[test]
    fn ovscreen_read_private_message_create_keyboard_popup_matches() {
        strict(
            ee::s05::f_00210fa0__generated,
            ovscreen_read_private_message_create_keyboard_popup,
            40,
            &|c, seed| {
                let _ = base(c, seed, 0x210f, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed % 3) + 1) as u64;
                c.g[A3][0] = (seed * 3) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                w(c, DATA + 0x10, DATA + 0x900);
            },
        );
    }

    // --------------------------------------------------------- input parser

    #[test]
    fn input_parser_lookup_button_matches() {
        strict(
            ee::s10::f_003216e8__generated,
            input_parser_lookup_button,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x3216, 6);
                // The "button" prefix the parser compares against.
                for (i, b) in b"button\0".iter().enumerate() {
                    c.bus.write8(0x004a_3ef0 + i as u32, *b);
                }
                let name: &[u8] = match seed % 8 {
                    0 => b"button0\0",
                    1 => b"button12\0",
                    2 => b"button12_\0",
                    3 => b"buttonA\0",
                    4 => b"button\0",
                    5 => b"buttons\0",
                    6 => b"button9999\0",
                    _ => b"butto\0",
                };
                for (i, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x900 + i as u32, *b);
                }
                c.g[A1][0] = (DATA + 0x900) as u64;
            },
        );
    }

    // ---------------------------------------------------- bragging rights

    #[test]
    fn cfe_state_bragging_rights_on_activate_matches() {
        strict(
            ee::s02::f_00193568__generated,
            cfe_state_bragging_rights_on_activate,
            90,
            &|c, seed| {
                let _ = base(c, seed, 0x1935, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                w(c, GAME_GP.wrapping_sub(0x1680), (seed % 2) as u32);
                w(c, DATA + 0x40, DATA + 0x800);
                w(c, DATA + 0x6f8, if seed % 3 == 0 { 0 } else { 1 });
                w(c, DATA + 0x72c, if seed % 4 < 2 { 0 } else { 1 });
                // A little screen data so the frame lookups differ.
                w(c, DATA + 0x800 + 0x38, DATA + 0x900);
                w(c, DATA + 0x900, 2);
                w(c, DATA + 0x804, (seed % 5) as u32);
            },
        );
    }

    // ----------------------------------------------------------- pause menu

    #[test]
    fn ovscreen_pause_freeride_play_icon_anim_matches() {
        strict(
            ee::s04::f_001f8df8__generated,
            ovscreen_pause_freeride_play_icon_anim,
            90,
            &|c, seed| {
                let _ = base(c, seed, 0x1f8d, 6);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 2) as u64;
                w(c, DATA + 0xc0, DATA + 0x800);
                w(c, DATA + 0xc4, if seed % 3 == 0 { 0 } else { DATA + 0x900 });
                w(c, DATA + 0x800 + 0x38, DATA + 0xa00);
                w(c, DATA + 0xa00, 3);
                w(c, DATA + 0x900 + 0x8, 0);
            },
        );
    }

    // ------------------------------------------------------------ BE char

    #[test]
    fn bechar_int_is_trick_score_above_rating_matches() {
        strict(
            ee::s01::f_00149778__generated,
            bechar_int_is_trick_score_above_rating,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x1497, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = ((seed * 37) % 2000) as u64;
                // rider object for the BE queries
                w(c, DATA + 0x800, (seed % 4) as u32);
                w(c, DATA + 0x804, ((seed / 4) % 5) as u32);
                w(c, DATA + 0x808, 1);
                w(c, DATA + 0x80c, 0);
                w(c, DATA + 0x810, (seed % 7) as u32);
            },
        );
    }

    // ------------------------------------------------------ message center

    #[test]
    fn message_center_post_peak_unlock_message_matches() {
        strict(
            ee::s04::f_001e2428__generated,
            message_center_post_peak_unlock_message,
            90,
            &|c, seed| {
                let _ = base(c, seed, 0x1e24, 6);
                let peak = DATA + 0x900;
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                // world->+0x84->+0x28 = peak, *peak = index
                w(c, W + 0x1000 + 0x28, peak);
                w(c, peak, if seed % 3 == 0 { 0 } else { (seed % 12) as u32 });
            },
        );
    }

    // -------------------------------------------------------- tex flip

    #[test]
    fn texflip_modifier_handle_event_matches() {
        strict(
            ee::s12::f_0035f598__generated,
            texflip_modifier_handle_event,
            90,
            &|c, seed| {
                let mut r = base(c, seed, 0x35f5, 6);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => 300,
                    1 => 301,
                    2 => 302,
                    3 => 299,
                    _ => 303,
                };
                c.f[12] = r.edge_f32();
                w(c, DATA + 0x8, 0);
                w(c, DATA + 0x14, (seed % 4) as u32);
                w(c, DATA + 0x20, 1 + (seed % 4) as u32);
            },
        );
    }

    // -------------------------------------------------------- lobby reply

    #[test]
    fn lobby_man_on_quick_play_reply_matches() {
        strict(
            ee::s06::f_0025f030__generated,
            lobby_man_on_quick_play_reply,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x25f0, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                w(c, DATA, if seed % 3 == 0 { 0x12 } else { 0x13 });
                w(c, DATA + 0xf0, DATA + 0x900);
                w(
                    c,
                    DATA + 0x800 + 0x8,
                    if seed % 2 == 0 { 0 } else { (seed % 5) + 1 },
                );
                w(c, DATA + 0x900, DATA + 0x980);
                w(c, DATA + 0x980 + 0x4, DATA + 0x9c0);
            },
        );
    }

    // --------------------------------------------------------- memory card

    #[test]
    fn bereward_int_get_medal_reward_id_matches() {
        strict(
            ee::s01::f_00159080__generated,
            bereward_int_get_medal_reward_id,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x1590, 0);
                c.g[A1][0] = seed as u64;
                c.g[T0][0] = (seed % 6) as u64;
                c.g[A3][0] = ((seed / 6) % 4) as u64;
            },
        );
    }

    // --------------------------------------------------------- memory card

        #[test]
    fn ps2_graphics_man_v94_matches() {
        strict(ee::s12::f_0036a2c0__generated, ps2_graphics_man_v94, 60, &|c, seed| {
            let mut r = base(c, seed, 0x36a2, 0);
            let mtx = DATA + 0x800;
            let obj = DATA + 0xa00;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = mtx as u64;
            w(c, DATA + 0xa4, if seed % 4 == 0 { 0 } else { obj });
            for off in (0..0x40u32).step_by(4) {
                w(c, mtx + off, r.edge_f32());
                w(c, obj + off, r.edge_f32());
            }
        });
    }

    // --------------------------------------------------------- memory card

        // --------------------------------------------------------- modifier obj

    #[test]
    fn mod_obj_v40_matches() {
        strict(ee::s11::f_00355770__generated, mod_obj_v40, 90, &|c, seed| {
            let mut r = base(c, seed, 0x3557, 6);
            allocator(c);
            let holder = DATA + 0x400;
            let obj = DATA + 0x500;
            let vt = DATA + 0x800;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            w(c, DATA + 0x1c, if seed % 6 == 0 { 0 } else { holder });
            w(c, holder, if seed % 6 == 1 { 0 } else { obj });
            w(c, obj, vt);
            w(c, vt + 0x50, 0);
            w(c, vt + 0x54, FAKE);
            w(c, vt + 0x38, 0);
            w(c, vt + 0x3c, FAKE + 0x20);
            w(c, vt + 0x48, 0);
            w(c, vt + 0x4c, FAKE + 0x10);
            // The holder's slot-9 result selects the "done" path.
            force(
                c,
                0,
                FAKE + 0x10,
                if seed % 4 == 0 { 0 } else { 1 },
                KEEP_F0,
            );
            // The script timer at +0x20 and the global tick object.
            w(c, DATA + 0x20, if seed % 3 == 0 { 0 } else { 1 });
            let tick = DATA + 0x1000;
            w(c, GAME_GP + 0x2a74, tick);
            w(c, tick + 0x10, 0x20);
        });
    }

    // --------------------------------------------------------------- lobby

    #[test]
    fn lobby_menu_create_account_matches() {
        strict(
            ee::s02::f_0017e030__generated,
            lobby_menu_create_account,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x17e0, 4);
                allocator(c);
                let menu = DATA;
                let g = DATA + 0x3000;
                let buf = DATA + 0x6000;
                c.g[A0][0] = menu as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.g[A2][0] = (DATA + 0x940) as u64;
                for (i, b) in b"user5\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x900 + i as u32, *b);
                }
                for (i, b) in b"passw0rd\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x940 + i as u32, *b);
                }
                // Pre-sized BXString buffers (capacity and length large
                // enough) so the assigns take the no-allocation path.
                w(c, buf.wrapping_sub(0xc), 0x1000);
                w(c, buf.wrapping_sub(4), 0x1000);
                w(c, menu + 0x1c04, buf);
                w(c, menu + 0x1c08, buf);
                w(c, GAME_GP.wrapping_sub(0xc8), g);
                for off in [0x260u32, 0x264, 0x268, 0x26c] {
                    w(c, g + off, buf);
                }
                // The page list: empty, or one page that MenuSys_PopPage drains.
                if seed % 3 == 0 {
                    let page = DATA + 0x400;
                    let vt = DATA + 0x1000;
                    w(c, menu + 8, 1);
                    w(c, menu + 0xc, page);
                    w(c, page + 0x12c, vt);
                    w(c, vt + 0x38, 0);
                    w(c, vt + 0x3c, FAKE);
                }
            },
        );
    }

    // -------------------------------------------------------- front-end FE

    #[test]
    fn cfe_state_reward_gallery_poll_shape_load_matches() {
        strict(
            ee::s04::f_001cff10__generated,
            cfe_state_reward_gallery_poll_shape_load,
            90,
            &|c, seed| {
                let _ = base(c, seed, 0x1cff, 6);
                let flags = [0u32, 2, 8, 0xa][(seed % 4) as usize];
                let index = (seed % 2) as u32;
                let id = (seed % 3) as u32;
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x48, flags);
                w(c, DATA + 0x50, index);
                w(c, DATA + 0x8ec, seed * 7 + 1);
                // world->+0x7c is the game; the shape manager sits at
                // game+0x1a70.
                w(c, W + 0x7c, W + 0x1000);
                let mgr = W + 0x1000 + 0x1a70;
                let slot = mgr + id * 0x120 + index * 0x12000;
                w(c, slot, if seed % 2 == 0 { 1 } else { 0 });
                // The loaded file's data: a zeroed shape (no textures).
                w(c, slot + 0x104, DATA + 0x7000);
                // self->+8 is the state's vtable, used for the two optional
                // completion callbacks.
                let vt = DATA + 0x2000;
                w(c, DATA + 8, vt);
                w(c, vt + 0x140, 0);
                w(c, vt + 0x144, FAKE + 0x70);
                w(c, vt + 0x108, 0);
                w(c, vt + 0x10c, FAKE + 0x80);
                w(c, DATA + 0x924, 0x1234);
            },
        );
    }

    #[test]
    fn cfe_state_preview_reward_handle_input_matches() {
        strict(
            ee::s04::f_001d1ff0__generated,
            cfe_state_preview_reward_handle_input,
            90,
            &|c, seed| {
                let _ = base(c, seed, 0x1d1f, 6);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                let evt_vt = DATA + 0x1000;
                w(c, DATA + 0x808, evt_vt);
                w(c, evt_vt + 0x30, 0);
                w(c, evt_vt + 0x34, FAKE);
                force(c, 0, FAKE, if seed % 4 == 0 { 0 } else { 1 }, KEEP_F0);
                // self->+0x10 points at the screen; its +0x14 at the default
                // object (or zero, which selects the static default).
                let holder = DATA + 0x900;
                w(c, DATA + 0x10, holder);
                w(
                    c,
                    holder + 0x14,
                    if seed % 3 == 0 { 0 } else { DATA + 0xa00 },
                );
                let p = DATA + 0xa00;
                let pvt = DATA + 0x1100;
                w(c, p, pvt);
                w(c, pvt + 0x10, 0);
                w(c, pvt + 0x14, FAKE + 0x10);
                // The three child objects at +0x48.
                for i in 0..3u32 {
                    let o = DATA + 0xb00 + 0x40 * i;
                    let vt = DATA + 0x1200 + 0x20 * i;
                    let present = (seed >> i) & 1 == 0;
                    w(c, DATA + 0x48 + 4 * i, if present { o } else { 0 });
                    w(c, o + 8, vt);
                    w(c, vt + 0x48, 0);
                    w(c, vt + 0x4c, FAKE + 0x20);
                }
                w(c, DATA + 0x40, DATA + 0xc00);
            },
        );
    }

    #[test]
    fn ovscreen_in_game_error_update_matches() {
        strict(
            ee::s04::f_001faf10__generated,
            ovscreen_in_game_error_update,
            120,
            &|c, seed| {
                let _ = base(c, seed, 0x1faf, 6);
                c.g[A0][0] = DATA as u64;
                let err = DATA + 0x300;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 8 == 0 { 0 } else { err },
                );
                w(c, DATA + 0x9c, if seed % 7 == 0 { 0 } else { 1 });
                w(
                    c,
                    err + 0x138,
                    match seed % 3 {
                        0 => 0,
                        1 => 5,
                        _ => 0xffff_ffff,
                    },
                );
                w(c, err + 0x13c, if seed % 4 == 0 { 0 } else { 1 });
                w(c, err + 0x80, if seed % 5 == 0 { 0 } else { 1 });
                w(c, err + 0xa0, if seed % 6 == 0 { 0 } else { 1 });
                w(c, err + 0x24, if seed % 9 == 0 { 0 } else { 1 });
                // The screen and the "ping" object found by hash.
                w(c, DATA + 0x40, DATA + 0x400);
                let obj = DATA + 0x500;
                let vt = DATA + 0x1000;
                w(c, obj + 8, vt);
                w(c, vt + 0x48, 0);
                w(c, vt + 0x4c, FAKE);
            },
        );
    }

    // ----------------------------------------------------------- BE rewards

        }
