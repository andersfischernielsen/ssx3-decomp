//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path script / menu / sound / UI helpers, each with a
//! differential test against the generated body.
//!
//! Most of these are small: a constructor that publishes a vtable and
//! initialises a few fields, a destructor that unlinks its list nodes and
//! forwards to the base class, an "is this character already selected" test,
//! or a table walk. Calls go through [`crate::call`] (direct callees through
//! [`Callee`] and `jalr`/cross-shard targets through [`call_ptr`]), so a test
//! compares the whole register dance; loops that check events use
//! [`crate::events`].

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0026_8308, net_event102_lock_clone, "NetEvent102Lock_Clone"),(0x0027_8f68, script_engine_list_start, "ScriptEngine_ListStart"),(0x0027_f968, rider_script_obj_ctor, "RiderScriptObj_ctor"),
    (0x0028_12e0, script_view_obj_set_transition, "ScriptViewObj_SetTransition"),
    (0x002a_e298, path_plat_add_bank, "PATH_PlatAddBank"),
    (0x002c_cc38, menu_sys_enum_item_draw, "MenuSysEnumItem_Draw"),
    (0x002c_dce8, menu_sys_float_item_ctor_2, "MenuSysFloatItem_ctor_2"),
    (0x0027_25f0, in_seq_curve_track_start, "InSeq_CurveTrackStart"),
    (
        0x0016_0fd0,
        relative_chase_camera_algorithm_reset_for_rider,
        "RelativeChaseCameraAlgorithm_ResetForRider",
    ),
    (0x003a_5068, c_ui_group_dtor, "cUIGroup_dtor"),
    (0x003a_52a0, ui_menu_dtor, "UIMenu_dtor"),(0x001c_c848, c_keyboard_popup_refresh_entry_text, "cKeyboardPopup_RefreshEntryText"),
    (0x0018_06d8, keyboard_screen_cursor_up, "KeyboardScreen_CursorUp"),(0x0015_7a78, be_reward_int_is_peak_complete, "BERewardInt_IsPeakComplete"),
    (0x0011_7a58, rider_metrix_find_hud_slot, "RiderMetrix_FindHUDSlot"),(0x0032_fa30, coll_hull_collide_sphere_tree, "CollHull_CollideSphereTree"),
    (0x0033_0360, coll_sphere_plane_distance, "CollSphere_PlaneDistance"),
    (0x0027_7310, script_list_on_seq_advance, "ScriptList_OnSeqAdvance"),
    (0x0032_1a40, input_parser_find_identifier, "InputParser_FindIdentifier"),
    (0x0031_86d0, c_bx_string_op_plus_c_str_str, "cBXString_OpPlusCStrStr"),
    (0x0038_f930, tex_man_swizzle_t8_block, "TexMan_SwizzleT8Block"),
    (0x002e_cc28, c_bolt_on_fx_interface_create_emitter, "cBoltOnFXInterface_CreateEmitter"),(0x0022_9408, crowd_man2d_remove_by_type, "CrowdMan2d_RemoveByType"),(0x0026_edd8, replay_apply_camera, "Replay_ApplyCamera"),
    (0x0016_9418, script_camera_algo_ctor, "ScriptCameraAlgo_ctor"),(0x003a_0c70, ui_text_set_ascii_string, "UIText_SetAsciiString"),
    (0x0014_ffb8, bebagt_int_test_uber_trick_bit_a, "BEBAGTInt_TestUberTrickBitA"),
    (0x0015_0048, bebagt_int_test_uber_trick_bit_b, "BEBAGTInt_TestUberTrickBitB"),
    (0x0034_c428, c_flag_manager_ctor, "cFlagManager_ctor"),(0x0023_00f0, game_module_reset_scripts_and_world, "GameModule_ResetScriptsAndWorld"),(0x0019_4498, ui_object_show_clamp_alpha_half, "UIObject_ShowClampAlphaHalf"),
    (0x0019_5d50, fe_locale_unload_fe_tables, "FELocale_UnloadFETables"),
    (0x0015_5390, bescore_int_update_record_level, "BEScoreInt_UpdateRecordLevel"),
    (0x0018_6518, c_fe_state_on_activate, "cFEState_OnActivate"),
    (0x0014_dd98, be_library_shutdown, "BELibrary_Shutdown"),
    (0x0027_5760, script_anim_bank_man_free_entry, "ScriptAnimBankMan_FreeEntry"),];

/// Direct and cross-shard callees, by guest address.
mod callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const SETUP_MAN_SET_LOCAL_CHARACTER: Callee =
        Callee { addr: 0x0026_4cf0, f: ee::s07::f_00264cf0 };
    pub const SETUP_MAN_SEND_CHARACTER_MSG: Callee =
        Callee { addr: 0x0026_4e50, f: ee::s07::f_00264e50 };

    pub const CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES: Callee =
        Callee { addr: 0x002a_6430, f: ee::s08::f_002a6430 };

    pub const SCRIPT_LIST_START: Callee = Callee { addr: 0x0027_6388, f: ee::s07::f_00276388 };
    pub const SCRIPT_LIST_CAN_SKIP: Callee =
        Callee { addr: 0x0027_7060, f: ee::s07::f_00277060 };
    pub const SCRIPT_LIST_GET_PLAY_STATUS: Callee =
        Callee { addr: 0x0027_6bd8, f: ee::s07::f_00276bd8 };

    pub const SCRIPT_BINDING_CTOR: Callee = Callee { addr: 0x0028_2f60, f: ee::s07::f_00282f60 };
    pub const SCRIPT_OBJ_CTOR: Callee = Callee { addr: 0x0028_3200, f: ee::s07::f_00283200 };

    pub const TRANSITION_MAN_STOP_TRANSITION: Callee =
        Callee { addr: 0x0028_2db0, f: ee::s07::f_00282db0 };
    pub const TRANSITION_MAN_SCRIPT_SET_OWNER: Callee =
        Callee { addr: 0x0028_2da8, f: ee::s07::f_00282da8 };

    pub const PATH_PLAT_FIND_BANK_SLOT: Callee =
        Callee { addr: 0x002a_e230, f: ee::s08::f_002ae230 };

    pub const MENU_SYS_ITEM_DRAW_ROW: Callee = Callee { addr: 0x002c_a988, f: ee::s09::f_002ca988 };
    pub const MENU_SYS_ITEM_CTOR: Callee = Callee { addr: 0x002c_a258, f: ee::s09::f_002ca258 };

    pub const MATH_EVAL_CUBIC: Callee = Callee { addr: 0x0027_2570, f: ee::s07::f_00272570 };

    pub const UI_LIST_DELETE_ALL: Callee = Callee { addr: 0x0039_7948, f: ee::s13::f_00397948 };
    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee { addr: 0x003a_4868, f: ee::s13::f_003a4868 };
    pub const UI_OBJECT_DTOR: Callee = Callee { addr: 0x0039_fc48, f: ee::s13::f_0039fc48 };

    pub const LINE_STRIP_CLOSE: Callee = Callee { addr: 0x0038_4dc0, f: ee::s12::f_00384dc0 };
    pub const LINE_STRIP_ADD_VERTEX: Callee = Callee { addr: 0x0038_4fd0, f: ee::s12::f_00384fd0 };

    pub const KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW: Callee =
        Callee { addr: 0x001c_c8d8, f: ee::s04::f_001cc8d8 };
    pub const KEYBOARD_POPUP_MASK_TEXT: Callee =
        Callee { addr: 0x001c_ce98, f: ee::s04::f_001cce98 };
    pub const KEYBOARD_POPUP_UPDATE_CURSOR: Callee =
        Callee { addr: 0x001c_cc30, f: ee::s04::f_001ccc30 };

    pub const INPUT_SYNC_PACK_SAMPLE: Callee = Callee { addr: 0x0023_7b28, f: ee::s06::f_00237b28 };
    pub const INPUT_SYNC_UNPACK_SAMPLE: Callee = Callee { addr: 0x0023_7be8, f: ee::s06::f_00237be8 };

    pub const SND_BANK_IS_LOADED_WITH_NAME: Callee =
        Callee { addr: 0x0028_b830, f: ee::s07::f_0028b830 };
    pub const SND_LOAD_QUEUE_CANCEL_BANK: Callee =
        Callee { addr: 0x0028_a728, f: ee::s07::f_0028a728 };
    pub const SND_BANK_LOAD: Callee = Callee { addr: 0x0028_b320, f: ee::s07::f_0028b320 };

    pub const BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE: Callee =
        Callee { addr: 0x0015_7bf0, f: ee::s01::f_00157bf0 };

    pub const VEC3_NEG_DOT: Callee = Callee { addr: 0x0033_00f8, f: ee::s11::f_003300f8 };

    pub const SCRIPT_LIST_IS_PLAYING_SCRIPT: Callee =
        Callee { addr: 0x0027_71c8, f: ee::s07::f_002771c8 };
    pub const SCRIPT_DATA_GET_INFO: Callee = Callee { addr: 0x0027_c070, f: ee::s07::f_0027c070 };
    pub const IN_SEQ_INSTANCE_GET_REMAINING_STEPS: Callee =
        Callee { addr: 0x0027_4d08, f: ee::s07::f_00274d08 };
    pub const SCRIPT_LIST_START_TRANSITION: Callee =
        Callee { addr: 0x0027_7980, f: ee::s07::f_00277980 };

    pub const CBX_STRING_CONCAT_COPY: Callee = Callee { addr: 0x0031_8540, f: ee::s10::f_00318540 };
    pub const CBX_STRING_CTOR_COPY: Callee = Callee { addr: 0x0031_7fe8, f: ee::s10::f_00317fe8 };
    pub const CBX_STRING_DTOR: Callee = Callee { addr: 0x0031_8260, f: ee::s10::f_00318260 };

    pub const ATTACHED_DYNAMIC_EMITTER_CREATE: Callee =
        Callee { addr: 0x002f_2130, f: ee::s09::f_002f2130 };
    pub const CROWD_MAN2D_CLEAR_SLOT: Callee = Callee { addr: 0x0022_9b90, f: ee::s05::f_00229b90 };
    pub const REPLAY_STREAM_REWIND_READ: Callee =
        Callee { addr: 0x0026_e9b0, f: ee::s07::f_0026e9b0 };
    pub const CAMERA_ALGORITHM_DEFAULT_FOV: Callee =
        Callee { addr: 0x0016_7e30, f: ee::s02::f_00167e30 };

    pub const BE_GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ee::s01::f_0014a0e0 };
    pub const BE_GET_RIDER_CHARACTER: Callee = Callee { addr: 0x0014_a080, f: ee::s01::f_0014a080 };

    pub const GAME_PROCESS_CTOR: Callee = Callee { addr: 0x0035_4648, f: ee::s11::f_00354648 };
    pub const C_FLAG_MANAGER_GROUP_CTOR: Callee =
        Callee { addr: 0x0034_af38, f: ee::s11::f_0034af38 };

    pub const MC_MANAGER_GET: Callee = Callee { addr: 0x0022_7f80, f: ee::s05::f_00227f80 };

    pub const WORLD_OBJ_REPLACE_WITH_DEAD_NODE: Callee =
        Callee { addr: 0x0034_fe28, f: ee::s11::f_0034fe28 };

    pub const FE_LOCALE_UNLOAD_TABLE: Callee =
        Callee { addr: 0x0019_5a50, f: ee::s02::f_00195a50 };

    pub const UI_LIST_FIRST: Callee = Callee { addr: 0x0039_7840, f: ee::s13::f_00397840 };
    pub const UI_LIST_NODE_IS_SENTINEL: Callee =
        Callee { addr: 0x0039_7718, f: ee::s13::f_00397718 };
    pub const UI_ASCII_TO_WIDE: Callee = Callee { addr: 0x0039_7ad8, f: ee::s13::f_00397ad8 };
    pub const UI_TEXT_SET_WIDE_STRING_INTERNAL: Callee =
        Callee { addr: 0x003a_0eb0, f: ee::s13::f_003a0eb0 };

    pub const BELIBRARY_DTOR: Callee = Callee { addr: 0x0014_99a8, f: ee::s01::f_001499a8 };

    pub const MEM_MAN_FREE_ENTRY: Callee = Callee { addr: 0x0031_7e98, f: ee::s10::f_00317e98 };

    pub const UI_SCREEN_FIND_OBJECT: Callee =
        Callee { addr: 0x0039_d860, f: ee::s13::f_0039d860 };
}

/// `a + off`, wrapping like the original's `addiu`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Character selection (cSetupMan).

/// `int cSetupMan_SelectCharacter(cSetupMan* self, int character)`
/// (0x264f88): called when the player picks a character in the front end.
///
/// When the setup manager is already switching to `character` (its state is
/// 12) there is nothing to do. Otherwise, if a character change is pending
/// (`self+0x124`), the chosen character becomes the local character and the
/// script is told about it with the current state; if none is pending the
/// script only gets the "player selected" message.
pub fn c_setup_man_select_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xc;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[S1][0] = c.g[A1][0];

        let this = r32(c, S0);
        let state = c.g[V1][0];
        if state == c.g[V0][0] {
            // Already on the way to this character.
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            let pending = Addr(p(this, 0x124)).lw(c);
            c.g[V0][0] = pending;
            if pending == 0 {
                c.g[V0][0] = 1;
                c.cycles += 5;
                // SendCharacterMsg(self, 1, character, 0).
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = 1;
                c.g[A3][0] = 0;
                callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_4fe4)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                if state == c.g[V0][0] {
                    // The pending character is the one asked for: mark the
                    // setup manager as "switching".
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    Addr(this).sw(c, c.g[V0][0]);
                } else {
                    c.g[A0][0] = c.g[S0][0];
                }
                c.cycles += 9;
                // SetLocalCharacter(self, character).
                c.g[A1][0] = c.g[S1][0];
                callee::SETUP_MAN_SET_LOCAL_CHARACTER.call(c, 0x0026_4fd0)?;
                // SendCharacterMsg(self, 0, character, self->state).
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = Addr(r32(c, A0)).lw(c);
                c.g[A1][0] = 0;
                callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_4fe4)?;
                frame.restore(c, S0, 0x20);
            }
        }
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.cycles += 4;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Network lock message.

/// `void* NetEvent102Lock_Clone(NetEvent102Lock* self)` (0x268308): allocates
/// a 0x14-byte copy of the lock message with its two `cBXString`s and returns
/// it.
pub fn net_event102_lock_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S2, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x488));
        c.g[A0][0] = 0x14;
        call_ptr(c, 0x0031_7d70, 0x0026_8338)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x6c6f_0000;
        c.g[A2][0] = 0x0048_0000;
        c.g[S1][0] = Addr(p(r32(c, S2), 0x10)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S2), 0xc)).lw(c);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x11d0));
        c.g[V1][0] = c.g[V1][0] | 0x636b;
        c.g[V0][0] = 0x102;
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        Addr(p(r32(c, S0), 4)).sw(c, c.g[A2][0]);
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_836c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(p(r32(c, S0), 0x10));
        call_ptr(c, 0x0031_82d8, 0x0026_8378)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd audio.

/// `void CrowdAudio_StopAll(CrowdAudio* self)` (0x2a7718): remove every source
/// from the per-instance source lists, then stop all of the crowd's sound
/// through its interface.
pub fn crowd_audio_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x14)).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.mtc1(0, 12);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
                callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES.call(c, 0x002a_7754)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x14)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x190));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7740);
                }
            }
        }
        c.cycles += 16;
        c.g[A0][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = 0x004a_0000;
        c.ldl(5, p(0x004a_0000, 0x3707));
        c.ldr(5, p(0x004a_0000, 0x3700));
        c.g[V1][0] = Addr(p(r32(c, A0), 0xab0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x1d8));
        c.g[V0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
        c.g[A2][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
        let target = r32(c, A2);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, V0)));
        call_ptr(c, target, 0x002a_778c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `int ScriptEngine_ListStart(ScriptEngine* self, int offset, void* a,
/// void* b)` (0x278f68): start the script list at `self+0x54c+offset`, publish
/// the new list state and return the list's start result.
pub fn script_engine_list_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        Addr(p(r32(c, S3), 0x550)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0x1;
        call_ptr(c, 0x002e_f368, 0x0027_8f9c)?;
        c.g[V0][0] = 0xcc;
        c.g[A0][0] = Addr(p(r32(c, S3), 0x54c)).lw(c);
        c.mult(0, S0, S0, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
        callee::SCRIPT_LIST_START.call(c, 0x0027_8fb8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = 0x3;
        if c.g[S0][0] == c.g[V0][0] {
            // Finished immediately.
        } else {
            c.cycles += 3;
            c.g[A0][0] = 0;
            c.g[V0][0] = 0x2;
            Addr(p(r32(c, S3), 0x550)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x002e_f368, 0x0027_8fd4)?;
        }
        c.cycles += 8;
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


// ---------------------------------------------------------------------------
// HUD / script list state.

/// `int ScriptEngine_IsHUDAllowed(ScriptEngine* self)` (0x2792e0): the HUD may
/// draw unless the engine is in state 2 or in the middle of a skippable
/// script; the script's play status must also not be "done" (3).
pub fn script_engine_is_hud_allowed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 2;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, S0), 0x550)).lw(c);
        if c.g[A0][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 1;
        } else {
            c.g[V0][0] = Addr(p(r32(c, S0), 0x554)).lw(c);
            c.cycles += 2;
            c.g[S2][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 7;
                c.g[S1][0] = 0xcc;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x54c)).lw(c);
                c.mult(0, A0, A0, S1);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                callee::SCRIPT_LIST_CAN_SKIP.call(c, 0x0027_932c)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.g[S2][0];
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(p(r32(c, S0), 0x550)).lw(c);
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x54c)).lw(c);
                    c.mult(0, A0, A0, S1);
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    callee::SCRIPT_LIST_GET_PLAY_STATUS.call(c, 0x0027_9348)?;
                    c.g[V0][0] = c.g[V0][0] ^ 0x3;
                    c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
            }
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
// Script objects.

/// `RiderScriptObj* RiderScriptObj_ctor(RiderScriptObj* self)` (0x27f968):
/// build a rider script binding (a script binding plus a script object of
/// kind 3), publish its vtable, copy the identity matrix into the object's
/// transform and zero its counters.
pub fn rider_script_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR.call(c, 0x0027_f97c)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[A1][0] = 3;
        callee::SCRIPT_OBJ_CTOR.call(c, 0x0027_f988)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = 0x0048_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1fd8));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x2018));
        c.g[A0][0] = 0x0050_0000;
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0xe60));
        Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
        c.g[V0][0] = sx(p(r32(c, S0), 0x30));
        Addr(p(r32(c, S0), 0x20)).sw(c, 0);
        c.lqc2(1, r32(c, A0));
        c.lqc2(2, p(r32(c, A0), 0x10));
        c.lqc2(3, p(r32(c, A0), 0x20));
        c.lqc2(4, p(r32(c, A0), 0x30));
        c.sqc2(1, r32(c, V0));
        c.sqc2(2, p(r32(c, V0), 0x10));
        c.sqc2(3, p(r32(c, V0), 0x20));
        c.sqc2(4, p(r32(c, V0), 0x30));
        c.g[V0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0x70)).sw(c, 0);
        Addr(p(r32(c, S0), 0x74)).sw(c, 0);
        Addr(p(r32(c, S0), 0x78)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptViewObj_SetTransition(ScriptViewObj* self, void* obj)`
/// (0x2812e0): set (or, with a null `obj`, stop) the view's transition and
/// report whether it changed.
pub fn script_view_obj_set_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(p(r32(c, S0), 0xd8)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, A0), 8)).lw(c);
            c.g[A1][0] = u64::MAX;
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[A2][0] = 9;
                c.mov_s(13, 12);
                c.mov_s(14, 12);
                c.mov_s(15, 12);
                callee::TRANSITION_MAN_STOP_TRANSITION.call(c, 0x0028_1320)?;
                Addr(p(r32(c, S0), 0xe8)).sw(c, 0);
                c.g[A0][0] = Addr(p(r32(c, S0), 0xd8)).lw(c);
            }
            c.cycles += 5;
            c.g[A1][0] = 0;
            callee::TRANSITION_MAN_SCRIPT_SET_OWNER.call(c, 0x0028_1330)?;
            Addr(p(r32(c, S0), 0xd8)).sw(c, 0);
            c.g[V0][0] = 1;
        } else {
            c.cycles += 3;
            let running = Addr(p(r32(c, S0), 0xd8)).lw(c) != 0;
            c.g[V0][0] = 0;
            if running {
                // A transition is already running: refuse.
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[V1][0];
                c.g[A1][0] = c.g[S0][0];
                Addr(p(r32(c, S0), 0xd8)).sw(c, c.g[V1][0]);
                callee::TRANSITION_MAN_SCRIPT_SET_OWNER.call(c, 0x0028_1358)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bank monitor.

/// `int PATH_PlatAddBank(int a, void* unused, int b, int c)` (0x2ae298): take a
/// free bank-monitor slot for bank `b` (trying `b` then any bank) and store
/// `{b, c, a}`, or return -13 when the table is full.
pub fn path_plat_add_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S2][0];
        callee::PATH_PLAT_FIND_BANK_SLOT.call(c, 0x002a_e2c0)?;
        c.g[V1][0] = 0x28;
        let mut found = (c.g[V0][0] as i64) >= 0;
        if !found {
            c.cycles += 5;
            c.g[A0][0] = u64::MAX;
            c.g[A1][0] = u64::MAX;
            callee::PATH_PLAT_FIND_BANK_SLOT.call(c, 0x002a_e2d4)?;
            c.g[V1][0] = 0x28;
            found = (c.g[V0][0] as i64) >= 0;
        }
        if found {
            c.cycles += 11;
            c.g[A0][0] = 0x004d_0000;
            c.mult(0, V1, V0, V1);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x3ef8));
            c.g[A1][0] = 1;
            c.g[V0][0] = c.g[S1][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            Addr(p(r32(c, V1), 0x1c)).sw(c, c.g[A1][0]);
            Addr(p(r32(c, V1), 8)).sw(c, c.g[S0][0]);
            Addr(r32(c, V1)).sw(c, c.g[S2][0]);
            Addr(p(r32(c, V1), 4)).sw(c, c.g[S1][0]);
        } else {
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_fff3);
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
// Menu items.

/// The entry point and the loop head (where an event resumes) of
/// [`menu_sys_enum_item_draw`].
const ENUM_DRAW_ENTRY: u32 = 0x002c_cc38;
const ENUM_DRAW_HEAD: u32 = 0x002c_cc58;

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
                    callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_cc8c)?;
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
                    callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002c_ccb8)?;
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
        callee::MENU_SYS_ITEM_CTOR.call(c, 0x002c_dd1c)?;
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
// Replay curve tracks.

/// `int InSeq_CurveTrackStart(void* self, CurveTrack* data, void* owner)`
/// (0x2725f0): attach a curve track (its key/value table and first segment)
/// to the in-sequence object and evaluate the first value.
pub fn in_seq_curve_track_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = sx(r32(c, A1).wrapping_add(4));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[A2][0]);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[A1][0]);
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[A1][0]);
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        Addr(p(r32(c, S0), 0x18)).sw(c, 0);
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        if c.g[V0][0] != 0 {
            Addr(p(r32(c, S0), 0x24)).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, A1), 4)).lhu(c);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            Addr(p(r32(c, S0), 0x24)).sw(c, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        c.mtc1(0, 12);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        callee::MATH_EVAL_CUBIC.call(c, 0x0027_2644)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
        c.swc1(0, p(r32(c, S0), 0x1c));
        c.g[V0][0] = Addr(r32(c, V1)).lhu(c);
        c.g[V0][0] = (c.g[V0][0] < 0x15u64) as u64;
        if c.g[V0][0] != 0 {
            Addr(p(r32(c, S0), 0x20)).sw(c, 0);
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x14));
            Addr(p(r32(c, S0), 0x20)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x24)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chase camera.

/// `void RelativeChaseCamerAlgorithm_ResetForRider(Cam* self, int id)`
/// (0x160fd0): if the camera's rider is `id`, reset the relative chase camera
/// (drop its blend state and refresh the camera behind the rider).
pub fn relative_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x30)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 4)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, A0), 0x6c0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x6c0));
        c.g[A1][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, A1)));
        call_ptr(c, target, 0x0016_1008)?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 15;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0016_6c60, 0x0016_101c)?;
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(13, 12);
            call_ptr(c, 0x0016_6550, 0x0016_1030)?;
            Addr(p(r32(c, S0), 0x394)).sw(c, 0);
            c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, 0x0016_1048)?;
            frame.restore(c, S0, 0x20);
        }
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.cycles += 4;
        frame.close(c);
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
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_50c0)?;
        Addr(p(r32(c, S1), 0x7c)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_50cc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        callee::UI_OBJECT_DTOR.call(c, 0x003a_50d8)?;
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
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_52f8)?;
        Addr(p(r32(c, S1), 0x7c)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::UI_LIST_NODE_DTOR_BODY.call(c, 0x003a_5304)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        callee::UI_OBJECT_DTOR.call(c, 0x003a_5310)?;
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
// PS2 graphics manager.

/// `void PS2GraphicsMan_Shutdown(PS2GraphicsMan* self)` (0x376268): tear down
/// the GS/DMA channels, the texture manager and the interrupt table.
pub fn ps2_graphics_man_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 2;
        call_ptr(c, 0x0042_4880, 0x0037_6280)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x5ac0)).lw(c);
        c.g[A0][0] = 2;
        call_ptr(c, 0x0042_3aa0, 0x0037_628c)?;
        c.g[A0][0] = 1;
        call_ptr(c, 0x0042_4950, 0x0037_6294)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x5ac4)).lw(c);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_3ad0, 0x0037_62a0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5ad0)).lw(c);
        call_ptr(c, 0x0042_3bf0, 0x0037_62a8)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5ad0)).lw(c);
        call_ptr(c, 0x0042_3bb0, 0x0037_62b0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5ac8)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0037_62b8)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x5acc)).lw(c);
        call_ptr(c, 0x0042_3db0, 0x0037_62c0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x18f0)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0037_62c8)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x18f4)).lw(c);
        call_ptr(c, 0x0036_7360, 0x0037_62d0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x18f4)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0037_62d8)?;
        c.g[A0][0] = 0x0050_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x59d8));
        call_ptr(c, 0x0036_1f40, 0x0037_62e4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Line strips.

/// `void LineStrip_Begin(LineStrip* ctx, void* v, void* pos)` (0x3850a8):
/// close the current strip, then start a new batch of vertices at `ctx->p`.
pub fn line_strip_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[S2][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x20));
            Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
            Addr(p(r32(c, S0), 4)).sw(c, c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                callee::LINE_STRIP_CLOSE.call(c, 0x0038_50f8)?;
                c.cycles += 1;
                c.g[V1][0] = Addr(p(r32(c, S0), 4)).lw(c);
            }
        }
        c.cycles += 14;
        c.g[A0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x30));
        c.g[A2][0] = c.g[S2][0];
        Addr(p(r32(c, S0), 4)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V1][0]);
        callee::LINE_STRIP_ADD_VERTEX.call(c, 0x0038_511c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end keyboard.

/// `void cKeyboardPopup_RefreshEntryText(cKeyboardPopup* self)`
/// (0x1cc848): refresh the popup's scroll window, look its text object up by
/// the entry's hash name and push the (masked) entry text into it.
pub fn c_keyboard_popup_refresh_entry_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW.call(c, 0x001c_c860)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6530));
        call_ptr(c, 0x0031_7670, 0x001c_c86c)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001c_c878)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x70)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(p(r32(c, S0), 0xb4));
                callee::KEYBOARD_POPUP_MASK_TEXT.call(c, 0x001c_c89c)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx(p(r32(c, S0), 0xf4));
                call_ptr(c, 0x003a_0c50, 0x001c_c8a8)?;
            } else {
                c.cycles += 2;
                c.g[A1][0] = sx(p(r32(c, S0), 0xb4));
                call_ptr(c, 0x003a_0c50, 0x001c_c8b8)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        callee::KEYBOARD_POPUP_UPDATE_CURSOR.call(c, 0x001c_c8c0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_CursorUp(KeyboardScreen* self)` (0x1806d8): move the
/// keyboard cursor up one row (skipping the page-jump cases), wrapping to the
/// last row.
pub fn keyboard_screen_cursor_up(c: &mut Cpu) {
    c.cycles += 4;
    let self_ = abi::arg(c, 0);
    c.g[V1][0] = Addr(p(self_, 0x170)).lw(c);
    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
    let in_first_row_band = c.g[V0][0] == 0;
    c.g[V0][0] = 8;
    if in_first_row_band {
        // Row 10..19.
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0xa));
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xa));
            Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(p(self_, 0xd8)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xa));
            Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = 0x12;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
            let wrap = c.g[V0][0] == 0;
            c.g[V0][0] = 0x27;
            if wrap {
                // Straight to the common store: row 39.
                c.cycles += 2;
                Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
        Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
        abi::ret(c);
    } else {
        // Row 0..9.
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0x27;
                c.cycles += 2;
                Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
                abi::ret(c);
                return;
            }
            c.g[V0][0] = Addr(p(self_, 0x170)).lw(c);
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1e));
        Addr(p(self_, 0x170)).sw(c, c.g[V0][0]);
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Input sync.

/// `void InputSync_SendSample(InputSync* self, void* src)` (0x237a98): pack the
/// sample into the frame buffer, unpack it back and, when the game comms
/// queue is open, send it.
pub fn input_sync_send_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        callee::INPUT_SYNC_PACK_SAMPLE.call(c, 0x0023_7abc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::INPUT_SYNC_UNPACK_SAMPLE.call(c, 0x0023_7ad0)?;
        c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x40)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, V1), 4)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
            }
        }
        let send = !(c.g[A0][0] == 0);
        c.cycles += 2;
        c.g[A0][0] = Addr(gp(c, -0x204)).lw(c);
        if send {
            c.cycles += 4;
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0025_7e98, 0x0023_7b0c)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound banks.

/// `void SndBankMan_LoadBank(SndBankMan* self, int id, void* a2, void* a3)`
/// (0x28bc58): if the bank is not loaded yet, cancel its pending load and
/// load it into the bank table entry.
pub fn snd_bank_man_load_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x60;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.mult(0, S3, S2, V0);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0xacc)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
        callee::SND_BANK_IS_LOADED_WITH_NAME.call(c, 0x0028_bc9c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            callee::SND_LOAD_QUEUE_CANCEL_BANK.call(c, 0x0028_bcb0)?;
            c.g[A0][0] = Addr(p(r32(c, S0), 0xacc)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
            callee::SND_BANK_LOAD.call(c, 0x0028_bcc4)?;
        }
        c.cycles += 7;
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
// BE reward goals.

/// `int BERewardInt_IsPeakComplete(BERewardInt* self, int a, int b, int c)`
/// (0x157a78): the peak is complete when its four goals are.
pub fn be_reward_int_is_peak_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S4][0];
        loop {
            c.cycles += 7;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S0][0];
            callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE.call(c, 0x0015_7ac4)?;
            let done = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            if !done {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 4i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S4][0];
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_7ab0);
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
// Rider metrics HUD slots.

/// `int RiderMetrix_FindHUDSlot(RiderMetrix* self, int type)` (0x117a58): the
/// index type itself below 34; otherwise the first free slot at or after 35,
/// else the slot with the smallest value/total ratio (-1 when none).
pub fn rider_metrix_find_hud_slot(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = ((c.g[A1][0] as i64) < 34i64) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        abi::ret(c);
        return;
    }
    c.g[A1][0] = Addr(p(this, 0x1b4)).lw(c);
    c.cycles += 6;
    c.g[V1][0] = 0x23;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 2); // f2 = 1.0
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[A2][0] = u64::MAX;
    if !empty {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(this, 0x1b0)).lw(c);
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x1554));
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] ^ 0x34;
            if c.g[V0][0] == 0 {
                // Free slot: take it.
                c.cycles += 2;
                c.g[V0][0] = c.g[V1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 7;
            c.lwc1(1, p(r32(c, A0), 8));
            c.lwc1(0, p(r32(c, A0), 4));
            c.div_s(1, 1, 0);
            c.c_lt_s(1, 2);
            if !c.fcc() {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.mov_s(2, 1);
                c.g[A2][0] = c.g[V1][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(p(r32(c, A0), 0x9c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_7a98);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bolt-on scene graph.

const BOLTS_COUNT_ENTRY: u32 = 0x0014_d4c8;
/// The two back edges of the child walk (event resume points).
const BOLTS_COUNT_MATCH: u32 = 0x0014_d510;
const BOLTS_COUNT_NEXT: u32 = 0x0014_d514;

/// `int Bolts_CountChildren(Bolts* self, int group, short key)` (0x14d4c8):
/// with a lookup table, the cached count; otherwise walk the group's entries
/// and count the ones whose key matches.
///
/// The original is a `match pc` dispatcher re-enterable at both loop heads.
pub fn bolts_count_children(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
        match pc {
            BOLTS_COUNT_ENTRY => {
                c.cycles += 6;
                c.g[T0][0] = c.g[A0][0];
                c.g[A1][0] = sx(r32(c, A1) << 2);
                c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, A1)));
                c.g[V0][0] = Addr(p(r32(c, V0), 0x11c)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = sx(r32(c, A2) << 1);
                    pc = 0x0014_d548;
                } else {
                    pc = 0x0014_d4e0;
                }
            }
            0x0014_d4e0 => {
                c.cycles += 11;
                c.g[V1][0] = sx(r32(c, T0).wrapping_add(r32(c, A1)));
                c.g[A0][0] = 0x38;
                c.g[A3][0] = Addr(p(r32(c, V1), 0xa4)).lw(c);
                c.g[V0][0] = c.g[V1][0];
                c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
                c.g[T1][0] = 0;
                c.mult(0, A0, A3, A0);
                c.g[V0][0] = Addr(p(r32(c, T0), 4)).lw(c);
                c.g[A1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                pc = 0x0014_d518;
            }
            0x0014_d50c => {
                c.cycles += 1;
                pc = BOLTS_COUNT_MATCH;
            }
            BOLTS_COUNT_MATCH => {
                c.cycles += 1;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x38));
                pc = BOLTS_COUNT_NEXT;
            }
            BOLTS_COUNT_NEXT => {
                c.cycles += 1;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                pc = 0x0014_d518;
            }
            0x0014_d518 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                pc = if c.g[V0][0] == 0 { 0x0014_d540 } else { 0x0014_d524 };
            }
            0x0014_d524 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, V1), 6)).lh(c);
                let different = c.g[V0][0] != c.g[A2][0];
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                pc = if different { 0x0014_d538 } else { 0x0014_d530 };
            }
            0x0014_d530 => {
                c.cycles += 2;
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                if crate::events::back_edge_unwinds(c, BOLTS_COUNT_MATCH) {
                    return Ok(());
                }
                pc = BOLTS_COUNT_MATCH;
            }
            0x0014_d538 => {
                c.cycles += 2;
                let advance = c.g[V0][0] == 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x38));
                if !advance {
                    pc = 0x0014_d540;
                } else {
                    if crate::events::back_edge_unwinds(c, BOLTS_COUNT_NEXT) {
                        return Ok(());
                    }
                    pc = BOLTS_COUNT_NEXT;
                }
            }
            0x0014_d540 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                abi::ret(c);
                return Ok(());
            }
            0x0014_d548 => {
                c.cycles += 3;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[V0][0] = Addr(r32(c, V1)).lh(c);
                abi::ret(c);
                return Ok(());
            }
            other => {
                c.jump_out(other, BOLTS_COUNT_ENTRY, 0x0014_d554);
                return Ok(());
            }
        }
    }
    })
}

// ---------------------------------------------------------------------------
// Collision.

/// `int CollHull_CollideSphereTree(CollHull* self, CollSphere* sphere,
/// Hit* hit)` (0x32fa30): collide the sphere with the hull's sphere tree and,
/// on a hit, store the hit direction length and normalise the direction.
pub fn coll_hull_collide_sphere_tree(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A2][0] = sx(p(r32(c, S0), 0x10));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = Addr(p(r32(c, V0), 0x60)).lw(c);
        call_ptr(c, 0x0032_7f18, 0x0032_fa58)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 19;
            c.lqc2(3, p(r32(c, S0), 0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.swc1(0, p(r32(c, S0), 0x40));
            c.g[V0][0] = 1;
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, p(r32(c, S0), 0x10));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, p(r32(c, S0), 0x10));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CollSphere_PlaneDistance(CollSphere* self, Vec4* plane, Vec4* p,
/// float* out)` (0x330360): the signed distance of the sphere to the plane,
/// written to `out`, and whether its magnitude is inside the radius.
pub fn coll_sphere_plane_distance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S0][0];
        callee::VEC3_NEG_DOT.call(c, 0x0033_038c)?;
        c.lqc2(3, r32(c, S0));
        c.lqc2(5, p(r32(c, S1), 0x60));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.add_s(1, 1, 0);
        c.g[V0][0] = 1;
        c.abs_s(2, 1);
        c.swc1(1, r32(c, S2));
        c.lwc1(0, p(r32(c, S1), 0x70));
        c.c_lt_s(2, 0);
        if !c.fcc() {
            c.g[V0][0] = 0;
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
// Script lists.

/// `void ScriptList_OnSeqAdvance(ScriptList* self, ScriptObj* obj)`
/// (0x277310): when the object is the list's playing script with no pending
/// transition and its remaining steps have run out, start the transition.
pub fn script_list_on_seq_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S1][0];
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT.call(c, 0x0027_7334)?;
        frame.restore(c, S0, 0x30);
        // Two of the four paths jump straight to the epilogue (their `lq16`
        // and `lq17` happen before the +1 cycle charge).
        let mut short_epilogue = false;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S2), 4)).lw(c);
            if c.g[V0][0] == 0 {
                short_epilogue = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                if c.g[V0][0] != 0 {
                    short_epilogue = true;
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    callee::SCRIPT_DATA_GET_INFO.call(c, 0x0027_735c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS.call(c, 0x0027_7368)?;
                    c.g[V1][0] = Addr(p(r32(c, S0), 0x12)).lh(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let exhausted = c.g[V1][0] != 0;
                    frame.restore(c, S0, 0x30);
                    if !exhausted {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S2][0];
                        callee::SCRIPT_LIST_START_TRANSITION.call(c, 0x0027_7384)?;
                    }
                }
            }
        }
        if !short_epilogue {
            c.cycles += 1;
        }
        frame.restore(c, S1, 0x20);
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input parser.

/// `InputEntry* InputParser_FindIdentifier(InputParser* self, char* name)`
/// (0x321a40): look `name` up in the parser's 0x44-byte entry table.
pub fn input_parser_find_identifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x8000;
        frame.save(c, S1, 0x30);
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xe4)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[A1][0];
        if !empty {
            c.cycles += 4;
            c.g[V0][0] = 0x80e8;
            c.g[S2][0] = c.g[V1][0];
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_aa88, 0x0032_1a8c)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    c.cycles += 7;
                    frame.restore(c, S0, 0x40);
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.g[V0][0] = Addr(p(r32(c, S2), 0xe4)).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(p(r32(c, S0), 0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1a80);
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0;
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
// cBXString.

/// `cBXString* cBXString_OpPlusCStrStr(cBXString* self, char* str,
/// cBXString* b)` (0x3186d0): `*self = str + *b` through a temporary string.
pub fn c_bx_string_op_plus_c_str_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A0][0];
        Addr(r32(c, SP)).sw(c, c.g[V0][0]);
        let mut len = 0u64;
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6810, 0x0031_8704)?;
            len = c.g[V0][0];
        } else {
            c.cycles += 1;
        }
        c.g[A1][0] = len;
        c.cycles += 19;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = Addr(r32(c, V0).wrapping_sub(8)).lw(c);
        c.g[T0][0] = c.g[V0][0];
        callee::CBX_STRING_CONCAT_COPY.call(c, 0x0031_8728)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S2][0];
        callee::CBX_STRING_CTOR_COPY.call(c, 0x0031_8734)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        callee::CBX_STRING_DTOR.call(c, 0x0031_8740)?;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Texture swizzling.

/// `int TexMan_SwizzleT8Block(void* unused, u8* src, u8* dst)` (0x38f930):
/// reorder a 16x16 T8 block through the swizzle table at `D_0044BDE0`.
pub fn tex_man_swizzle_t8_block(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0045_0000;
    c.g[T3][0] = 0;
    c.g[T6][0] = sx(r32(c, V0).wrapping_sub(0x4220));
    c.g[V1][0] = 0;
    'outer: loop {
        c.cycles += 6;
        c.g[V0][0] = c.g[V1][0] & 1;
        c.g[T5][0] = sx(r32(c, V1).wrapping_add(1));
        c.g[T1][0] = sx(r32(c, V0) << 6);
        c.g[V1][0] = 0;
        c.g[T4][0] = sx(r32(c, A1).wrapping_add(0x40));
        c.g[V0][0] = sx(r32(c, T1) << 2);
        'middle: loop {
            c.cycles += 4;
            c.g[T2][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, T6)));
            c.g[T0][0] = 0;
            'inner: loop {
                c.cycles += 11;
                c.g[V0][0] = Addr(r32(c, A3)).lw(c);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, T3)));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(1));
                c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
                c.g[V0][0] = (c.g[T0][0] < 4u64) as u64;
                let more = c.g[V0][0] != 0;
                Addr(r32(c, A0)).sb(c, c.g[V1][0]);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_f968);
                    }
                    continue 'inner;
                }
                c.cycles += 4;
                c.g[V1][0] = c.g[T2][0];
                c.g[V0][0] = (c.g[V1][0] < 0x10u64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, T1) << 2);
                if !more {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[T5][0];
                    c.g[V0][0] = (c.g[V1][0] < 4u64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[T4][0];
                    if !more {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        abi::ret(c);
                        return;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_f940);
                    }
                    continue 'outer;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_f958);
                }
                continue 'middle;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Lens FX emitters.

/// `int cBoltOnFXInterface_CreateEmitter(LensFX* self, void* x, void* y)`
/// (0x2ecc28): take the next free emitter slot (of 15) and start an attached
/// dynamic emitter for it.
pub fn c_bolt_on_fx_interface_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(p(r32(c, A0), 0x10));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != c.g[V1][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 15i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_cc58);
                }
                continue;
            }
            c.cycles += 9;
            c.g[A0][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            callee::ATTACHED_DYNAMIC_EMITTER_CREATE.call(c, 0x002e_cc74)?;
            Addr(r32(c, S0)).sw(c, c.g[V0][0]);
            Addr(p(r32(c, S0), 8)).sw(c, c.g[S2][0]);
            c.g[V0][0] = c.g[S1][0];
            Addr(p(r32(c, S0), 4)).sw(c, c.g[S3][0]);
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
// Replay headers.

/// `int ReplayCache_Deserialize(ReplayCache* self, u8** pp)` (0x26d558): read
/// the entry header, copy the body through the replay codec, advance the
/// stream and return 1.
pub fn replay_cache_deserialize(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        call_ptr(c, 0x003e_6574, 0x0026_d580)?;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, SP), 0x10));
        c.g[V1][0] = Addr(p(r32(c, SP), 4)).lw(c);
        c.g[A2][0] = 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        call_ptr(c, 0x003e_6574, 0x0026_d5a4)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        Addr(r32(c, S0)).sw(c, c.g[A0][0]);
        c.g[A2][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
        call_ptr(c, 0x002c_85a0, 0x0026_d5bc)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = 1;
        c.g[A0][0] = Addr(p(r32(c, SP), 0x10)).lw(c);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        frame.restore_ra(c, 0x20);
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 2D crowd.

/// `void CrowdMan2d_RemoveByType(CrowdMan2d* self, int id)` (0x229408):
/// clear the 128 slots whose type byte is `id`.
pub fn crowd_man2d_remove_by_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[S2][0] | 0xffff;
        frame.save(c, S4, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        c.g[S0][0] = sx(p(r32(c, S3), 8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] == c.g[S2][0] {
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                if c.g[V0][0] != c.g[S4][0] {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S1][0];
                    callee::CROWD_MAN2D_CLEAR_SLOT.call(c, 0x0022_9464)?;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_9440);
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

/// `int ReplayStream_Decompress(ReplayStream* self, u8** pp, u8* src)`
/// (0x26eab8): decode one compressed frame: header, body, then rewind the
/// read pointer.
pub fn replay_stream_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eae0)?;
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = c.g[SP][0] | 4;
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eaf0)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        Addr(r32(c, S0)).sw(c, c.g[A0][0]);
        c.g[A2][0] = Addr(p(r32(c, S1), 4)).lw(c);
        call_ptr(c, 0x002c_85a0, 0x0026_eb08)?;
        c.g[V1][0] = Addr(p(r32(c, SP), 4)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
        Addr(r32(c, S0)).sw(c, c.g[A1][0]);
        c.g[V1][0] = Addr(p(r32(c, S1), 4)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        Addr(p(r32(c, S1), 8)).sw(c, c.g[V1][0]);
        callee::REPLAY_STREAM_REWIND_READ.call(c, 0x0026_eb2c)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_ApplyCamera(void* self)` (0x26edd8): apply the recorded
/// camera index to every rider in the world's rider list.
pub fn replay_apply_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x84)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[V0][0] = 0x0044_0000;
        if !empty {
            c.cycles += 5;
            c.g[S1][0] = sx(p(r32(c, V1), 4));
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x5438));
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = Addr(p(r32(c, S2), 0x630)).lw(c);
            loop {
                c.cycles += 10;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                c.g[A0][0] = Addr(p(r32(c, V1), 0xa8)).lw(c);
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0016_1fa0, 0x0026_ee40)?;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p(r32(c, S2), 0x630)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_ee20);
                }
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
// Script camera algorithm.

/// `ScriptCameraAlgo* ScriptCameraAlgo_ctor(ScriptCameraAlgo* self)`
/// (0x169418): default FOV, camera constants and the rider's camera control
/// index.
pub fn script_camera_algo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4220));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[V0][0]);
        callee::CAMERA_ALGORITHM_DEFAULT_FOV.call(c, 0x0016_9438)?;
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 1);
        c.g[V1][0] = 0x55;
        c.lwc1(2, gp(c, -0x6548));
        c.g[V0][0] = 0x0046_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x3748));
        Addr(p(r32(c, S0), 0x10)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0x0050_0000;
        c.swc1(0, r32(c, S0));
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0xed0));
        c.swc1(1, p(r32(c, S0), 4));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(2, p(r32(c, S0), 8));
        Addr(p(r32(c, S0), 0x14)).sw(c, 0);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[RA][0] = Addr(r32(c, SP)).ld(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x84)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, A0), 0x84)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0xac)).lw(c);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[A0][0]);
        c.lq(3, r32(c, A2));
        c.sq(3, p(r32(c, S0), 0x20));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end rider preview.

/// `int FEModule_RiderPreviewFindTexture(RiderPreview* self, char* name)`
/// (0x19f878): the id of the first preview entry whose name matches, else -1.
pub fn fe_module_rider_preview_find_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S3), 0x18)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        let mut found = false;
        if !empty {
            c.cycles += 2;
            c.g[S1][0] = sx(p(r32(c, S3), 0x1c));
            c.g[S0][0] = sx(p(r32(c, S3), 0x20));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_65a8, 0x0019_f8bc)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    found = true;
                    break;
                }
                c.g[V0][0] = Addr(p(r32(c, S3), 0x18)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_f8b0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x10);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// UI text.

/// `void cUIText_setAsciiString(cUIText* self, const char* str)` (0x3a0c70):
/// clear the text's animations, then convert up to 511 ASCII characters into
/// the stack buffer and set them.
pub fn ui_text_set_ascii_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x430);
        frame.save(c, S0, 0x420);
        frame.save(c, S1, 0x410);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x400);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x003a_0ca0)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p(r32(c, V0), 0xc0)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0xc4)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x003a_0cb8)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x003a_0cc0)?;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 512i64) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[SP][0];
            callee::UI_ASCII_TO_WIDE.call(c, 0x003a_0cdc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            callee::UI_TEXT_SET_WIDE_STRING_INTERNAL.call(c, 0x003a_0ce8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x420);
        frame.restore(c, S1, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE bag tricks.

/// `int BEBAGTInt_TestUberTrickBitA(void*, int rider, int idx, int bit)`
/// (0x14ffb8): bit `bit` of `flags` in the rider's character bag-trick table.
pub fn bebagt_int_test_uber_trick_bit_a(c: &mut Cpu) {
    bebagt_test_bit(c, 2);
}

/// `int BEBAGTInt_TestUberTrickBitB(void*, int rider, int idx, int bit)`
/// (0x150048): the same for the second flag word.
pub fn bebagt_int_test_uber_trick_bit_b(c: &mut Cpu) {
    bebagt_test_bit(c, 4);
}

/// The shared body of the two "test a bag-trick bit" wrappers.
fn bebagt_test_bit(c: &mut Cpu, flag_offset: u32) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_ffe4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0014_fff0)?;
        c.g[A1][0] = 6;
        c.g[V1][0] = 0x1fe;
        c.mult(0, V0, V0, V1); // character * 510
        c.mult(0, S0, S0, A1); // index * 6
        c.g[A0][0] = 0x13ec;
        c.g[V1][0] = 0x0053_0000;
        c.mult(0, S1, S1, A0); // profile * 0x13ec
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xec0));
        frame.restore(c, S2, 0x20);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, S1)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = Addr(p(r32(c, V1), flag_offset)).lhu(c);
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> (r32(c, S3) & 31)) as u32);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = c.g[V0][0] & 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Flag manager.

/// `cFlagManager* cFlagManager_ctor(cFlagManager* self)` (0x34c428): process
/// base, then fourteen flag groups and the blend constants.
pub fn c_flag_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(p(r32(c, S0), 0x20));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xe;
        callee::GAME_PROCESS_CTOR.call(c, 0x0034_c450)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x0049_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x480));
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
            callee::C_FLAG_MANAGER_GROUP_CTOR.call(c, 0x0034_c46c)?;
            let more = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x188));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c460);
            }
        }
        c.cycles += 16;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(1, 0);
        c.swc1(1, p(r32(c, S0), 0x14));
        c.swc1(0, p(r32(c, S0), 0x18));
        Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
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
// Profile / PDA overlay.

/// `int OVScreenProfileSavePDA_OnWidgetNotify(OVScreen*, int, int key)`
/// (0x211b38): the save-PDA overlay answers widget 6/8 with the MC manager's
/// state and widget 9 with "handled".
pub fn ov_screen_profile_save_pda_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::MC_MANAGER_GET.call(c, 0x0021_1b50)?;
        let mgr = r32(c, V0);
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 8;
        let key = r32(c, S0);
        let is8 = key == 8;
        let below9 = key < 9;
        c.g[V0][0] = below9 as u64;
        let mut result = 0x101u64;
        let mut vtable = is8;
        if !is8 {
            c.cycles += 2;
            c.g[V0][0] = 6;
            if !below9 {
                c.cycles += 3;
                c.g[V0][0] = 9;
                let is9 = key == 9;
                c.g[V0][0] = 0x101;
                if is9 {
                    c.cycles += 1;
                    result = 0x100;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                let is6 = key == 6;
                c.g[V0][0] = 0x101;
                if is6 {
                    vtable = true;
                } else {
                    c.cycles += 2;
                }
            }
        }
        if vtable {
            c.cycles += 9;
            c.g[A2][0] = Addr(p(mgr, 0x434)).lw(c);
            c.g[A1][0] = Addr(p(mgr, 0x428)).lw(c);
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x1b0)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x1b4)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0021_1ba8)?;
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = 0x101;
            c.cycles += 1;
            if !nonzero {
                result = 0x100;
                c.cycles += 1;
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = result;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game module reset.

/// `void GameModule_ResetScriptsAndWorld(GameModule* self)` (0x2300f0): tear
/// down the script/animation banks and reset the world map.
pub fn game_module_reset_scripts_and_world(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(gp(c, 0x2898));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(r32(c, S1), 0x34)).lw(c);
        call_ptr(c, 0x0026_ca90, 0x0023_0110)?;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x30)).lw(c);
        call_ptr(c, 0x0030_9030, 0x0023_0118)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0023_0124)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_5118, 0x0023_0130)?;
        c.g[A1][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_5118, 0x0023_013c)?;
        c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0xa4)).lw(c);
        call_ptr(c, 0x0010_3358, 0x0023_0148)?;
        call_ptr(c, 0x002d_9b40, 0x0023_0150)?;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x003a_6800, 0x0023_0158)?;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        call_ptr(c, 0x0030_9f18, 0x0023_0160)?;
        c.g[A0][0] = Addr(p(r32(c, S1), 0x30)).lw(c);
        call_ptr(c, 0x0030_8c60, 0x0023_0168)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Instance manager.

/// `void MeshAnim_Kill(void* self)` (0x352780): mark the instance's animation
/// dead (or replace the world object when it has none).
pub fn mesh_anim_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, A2), 0x6b0)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 16;
            c.g[A1][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] = c.g[V1][0] | 0x300;
            c.g[V0][0] = Addr(p(r32(c, A1), 8)).lw(c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            Addr(p(r32(c, A1), 8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, A0), 8)).lw(c);
            c.g[V1][0] = sx(((r32(c, V0) as i32) >> 16) as u32);
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            Addr(p(r32(c, A0), 8)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 8)).lw(c);
            c.g[V0][0] = c.g[V0][0] | 2;
            let no_self = c.g[A2][0] == 0;
            Addr(p(r32(c, V1), 8)).sw(c, c.g[V0][0]);
            if !no_self {
                c.cycles += 8;
                c.g[V0][0] = Addr(p(r32(c, A2), 0xc)).lw(c);
                c.g[A1][0] = 3;
                c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, target, 0x0035_27f0)?;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[A2][0];
            callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE.call(c, 0x0035_2800)?;
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI object alpha.

/// `void UIObject_ShowClampAlphaHalf(UIObject* self)` (0x194498): show the
/// object and clamp its colour alpha to 0.5.
pub fn ui_object_show_clamp_alpha_half(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x40)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x44)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x0019_44c0)?;
        c.ldl(2, p(r32(c, S0), 0x23));
        c.ldr(2, p(r32(c, S0), 0x1c));
        c.ldl(3, p(r32(c, S0), 0x2b));
        c.ldr(3, p(r32(c, S0), 0x24));
        c.sdl(2, p(r32(c, SP), 0x7));
        c.sdr(2, r32(c, SP));
        c.sdl(3, p(r32(c, SP), 0xf));
        c.sdr(3, p(r32(c, SP), 8));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.lwc1(0, r32(c, SP));
        c.c_lt_s(1, 0);
        let clamped = c.fcc();
        c.g[A1][0] = c.g[SP][0];
        if clamped {
            c.cycles += 6;
            c.swc1(1, r32(c, SP));
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x58)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x5c)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, 0x0019_4514)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Locale.

/// `void FELocale_UnloadFETables(FELocale* self)` (0x195d50): drop the eight
/// front-end tables unless the locale is already detached.
pub fn fe_locale_unload_fe_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 3)).lb(c);
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] == 0 {
            c.cycles += 24;
            for (i, id) in [4u32, 5, 6, 7, 0x10, 0x11, 0x12, 0x13].into_iter().enumerate() {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(id);
                callee::FE_LOCALE_UNLOAD_TABLE.call(c, 0x0019_5d78 + 0xc * i as u32)?;
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE scores.

/// `void BEScoreInt_UpdateRecordLevel(BEScoreInt* self, int idx, int score)`
/// (0x155390): raise the stored record level for `idx` when `score` beats it.
pub fn bescore_int_update_record_level(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = 6;
    c.g[V0][0] = 0x0044_0000;
    c.mult(0, V1, A1, V1); // idx * 6
    c.g[T3][0] = c.g[V0][0];
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xed0));
    c.g[T1][0] = c.g[A0][0];
    c.g[A0][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
    c.g[T0][0] = 0;
    c.g[T2][0] = Addr(p(r32(c, A0), 0xbb8)).lb(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[V0][0] = Addr(p(r32(c, V1), 4)).lh(c);
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let below = c.g[V0][0] != 0;
    c.g[A3][0] = 2;
    if !below {
        c.cycles += 2;
        c.g[T0][0] = 3;
    } else {
        c.cycles += 1;
        c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
        loop {
            c.cycles += 2;
            let negative = (c.g[A3][0] as i64) < 0;
            c.g[V0][0] = 6;
            if negative {
                break;
            }
            c.cycles += 9;
            c.g[V1][0] = sx(r32(c, A3) << 1);
            c.mult(0, V0, A1, V0); // idx * 6
            c.g[A0][0] = sx(r32(c, T3).wrapping_add(0xed0));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, V1)).lh(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[T0][0] = sx(r32(c, A3).wrapping_add(1));
                break;
            }
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            if event_due(c) {
                deliver_event_here(c, 0x0015_53d4);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[T0][0] as i64)) as u64;
    let raise = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, A1)));
    if raise {
        c.cycles += 1;
        Addr(p(r32(c, V0), 0xbb8)).sb(c, c.g[T0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `void cFEState_OnActivate(cFEState* self, int a1)` (0x186518): select the
/// credits menu entry, then run the base activation.
pub fn c_fe_state_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S1), 0x24));
        callee::UI_LIST_FIRST.call(c, 0x0018_653c)?;
        c.g[S0][0] = c.g[V0][0];
        let empty = c.g[S0][0] == 0;
        c.g[A0][0] = 0x004a_0000;
        if empty {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 8;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1398));
            call_ptr(c, 0x0031_7670, 0x0018_6550)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_655c)?;
            c.g[S0][0] = c.g[V0][0];
            let found = c.g[S0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if found {
                c.cycles += 6;
                c.g[A0][0] = sx(gp(c, 0x1d90));
                c.g[A1][0] = Addr(p(r32(c, S1), 0xc)).lb(c);
                call_ptr(c, 0x001a_06f0, 0x0018_6574)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0] & 0xff;
                call_ptr(c, 0x0039_b960, 0x0018_6580)?;
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
            }
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e4c0, 0x0018_658c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE library.

/// `void BELibrary_Shutdown()` (0x14dd98): destroy the BE library, shut the
/// three BE states down and free the singleton.
pub fn be_library_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x1ef8)).lw(c);
        frame.save(c, S0, 0x10);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            c.g[A1][0] = 3;
            callee::BELIBRARY_DTOR.call(c, 0x0014_ddb4)?;
        }
        c.cycles += 28;
        c.g[S0][0] = 0x0053_0000;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x8a8)).lw(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x8a8));
        c.g[V0][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, target, 0x0014_ddd4)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, target, 0x0014_ddec)?;
        c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, A1), 4)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, target, 0x0014_de04)?;
        c.g[A0][0] = Addr(gp(c, -0x1f04)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0014_de0c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp(c, -0x1f04)).sw(c, 0);
        Addr(gp(c, -0x1f00)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script animation banks.

/// `void ScriptAnimBankMan_FreeEntry(ScriptAnimBankMan* self, Entry* e)`
/// (0x275760): free the bank entry's animation and data.
pub fn script_anim_bank_man_free_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(gp(c, 0xd08)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[V1][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 4)).lbu(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[S1][0] = Addr(r32(c, V1)).lw(c);
        Addr(r32(c, V1)).sw(c, 0);
        call_ptr(c, 0x0031_1110, 0x0027_5794)?;
        let no_anim = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !no_anim {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0031_12c8, 0x0027_57a4)?;
        }
        let mut one_cycle = true;
        c.cycles += 3;
        c.g[V0][0] = Addr(p(r32(c, S0), 0xc)).lw(c);
        let no_data = c.g[V0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if !no_data {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
            if c.g[A0][0] == 0 {
                Addr(p(r32(c, S0), 0x10)).sw(c, 0);
                one_cycle = false;
            } else {
                c.cycles += 3;
                callee::MEM_MAN_FREE_ENTRY.call(c, 0x0027_57c4)?;
                c.g[V0][0] = u64::MAX;
            }
        }
        if one_cycle {
            c.cycles += 1;
            Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        }
        c.cycles += 8;
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, 0);
        Addr(p(r32(c, S0), 8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI states.

/// `void cUIState_hideObjSafe(cUIState* self, char* name)` (0x39e8b8): hide
/// the first screen object whose hash name matches.
pub fn ui_state_hide_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = sx(p(r32(c, A0), 0x24));
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_LIST_FIRST.call(c, 0x0039_e8d4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0031_7670, 0x0039_e8e8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                callee::UI_SCREEN_FIND_OBJECT.call(c, 0x0039_e8f4)?;
                c.g[A2][0] = c.g[V0][0];
                let missing = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !missing {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                    call_ptr(c, target, 0x0039_e914)?;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 5;
                c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_e928)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_e8e0);
                }
            }
        }
        c.cycles += 4;
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
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{machine, ptr, vtable};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// A scratch object away from `DATA` (where pointers land).
    const OBJ: u32 = DATA + 0x1000;
    /// The zeroed BELibrary dummy.
    const BE: u32 = DATA + 0x3_0000;
    /// The dummy world.
    const WORLD: u32 = DATA + 0x8_0000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }


    /// Scrambled registers, a game-like `gp` and the fake call hooks.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        machine(c, seed, salt, 0)
    }

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u32 = 0xdead_0100;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET, "DLHeap_InitArena did not return");
    }

    /// `StrHash(addr)` (0x317670), for fixtures that must match a hash name.
    fn str_hash(c: &mut Cpu, addr: u32) -> u64 {
        const RET: u32 = 0xdead_0200;
        c.g[A0][0] = sx(addr);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
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
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP.wrapping_add(slot), pool);
        }
    }

        #[test]
    fn net_event102_lock_clone_matches() {
        check(ee::s07::f_00268308__generated, net_event102_lock_clone, 200, &|c, seed| {
            let mut r = base(c, seed, 0x26_8308);
            globals(c);
            pool(c);
            w(c, DATA, if seed % 2 == 0 { 0x102 } else { r.next_u32() });
            w(c, DATA + 0x4, [0, 0x4811d0][seed as usize % 2]);
            w(c, DATA + 0x8, 0x6c6f636b);
            w(c, DATA + 0xc, DATA + 0x2000);
            w(c, DATA + 0x10, DATA + 0x2100);
            w(c, DATA + 0x2000, 0x400);
            w(c, DATA + 0x2100, 0x200);
            c.g[A0][0] = sx(DATA);
        });
    }

            #[test]
    fn rider_script_obj_ctor_matches() {
        check(ee::s07::f_0027f968__generated, rider_script_obj_ctor, 100, &|c, seed| {
            let _ = base(c, seed, 0x27_f968);
            globals(c);
            // The identity matrix at the global the original copies.
            for i in 0..4u32 {
                w(c, 0x004f_f1a0 + 0x10 * i, 0x3f80_0000);
            }
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn script_view_obj_set_transition_matches() {
        check(ee::s07::f_002812e0__generated, script_view_obj_set_transition, 150, &|c, seed| {
            let _ = base(c, seed, 0x28_12e0);
            globals(c);
            const TRANS: u32 = DATA + 0x4000;
            w(c, TRANS + 8, [0, 1, 2][seed as usize % 3]);
            w(c, TRANS + 0x10, 0x1111);
            match seed % 4 {
                0 => {
                    // No new object: the running transition is stopped.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = 0;
                    w(c, DATA + 0xd8, TRANS);
                }
                1 => {
                    // No new object and no transition.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = 0;
                    w(c, DATA + 0xd8, TRANS);
                }
                2 => {
                    // Install a new transition.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = sx(DATA + 0x5000);
                    w(c, DATA + 0xd8, 0);
                }
                _ => {
                    // A transition is already running: refuse.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = sx(DATA + 0x5000);
                    w(c, DATA + 0xd8, TRANS);
                }
            }
        });
    }

    #[test]
    fn path_plat_add_bank_matches() {
        const TABLE: u32 = 0x004d_3ef8;
        check(ee::s08::f_002ae298__generated, path_plat_add_bank, 120, &|c, seed| {
            let _ = base(c, seed, 0x2a_e298);
            globals(c);
            // Slot 0 is free (or holds bank 0), slot 1 free; the rest either
            // free (walk ends at the first free slot) or all taken.
            for i in 0..24u32 {
                w(c, TABLE + 0x28 * i + 8, if seed % 2 == 0 { 0 } else { 0x7fff_ffff });
            }
            if seed % 2 == 1 {
                // Full table: the -13 path.
                c.g[A0][0] = 5;
                c.g[A2][0] = 7;
            } else {
                c.g[A0][0] = 0;
                c.g[A2][0] = 0;
            }
            c.g[A3][0] = sx(seed ^ 0x55);
        });
    }


    driven!(enum_draw_generated, ee::s09::f_002ccc38__generated);
    driven!(enum_draw_hand, menu_sys_enum_item_draw);

    #[test]
    fn menu_sys_enum_item_draw_matches() {
        check_events(enum_draw_generated, enum_draw_hand, 90, &|c, seed| {
            let mut r = base(c, seed, 0x2c_cc38);
            globals(c);
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
            w(c, DATA + 0x2100, if seed % 4 == 0 { 0x999 } else { 0x100 + seed % 3 });
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            schedule(c, &mut r, seed, ENUM_DRAW_ENTRY)
        });
    }

    #[test]
    fn menu_sys_float_item_ctor_2_matches() {
        check(ee::s09::f_002cdce8__generated, menu_sys_float_item_ctor_2, 120, &|c, seed| {
            let mut r = base(c, seed, 0x2c_dce8);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
        });
    }

    #[test]
    fn in_seq_curve_track_start_matches() {
        check(ee::s07::f_002725f0__generated, in_seq_curve_track_start, 150, &|c, seed| {
            let mut r = base(c, seed, 0x27_25f0);
            globals(c);
            const DATA_PTR: u32 = DATA + 0x2000;
            w(c, DATA_PTR, [0, 1, 2][seed as usize % 3]);
            w(c, DATA_PTR + 4, [0, 0x10, 0x20][seed as usize % 3]);
            // A couple of curve keys for Math_EvalCubic.
            for i in 0..6u32 {
                w(c, DATA_PTR + 8 + 4 * i, r.next_u32() & 0x3f80_3f80);
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA_PTR);
            c.g[A2][0] = sx(DATA + 0x3000);
        });
    }

    #[test]
    fn relative_chase_camera_algorithm_reset_for_rider_matches() {
        // The camera's rider vtable: the original compares the id returned by
        // the first virtual call; force a match on half the seeds.
        check(ee::s01::f_00160fd0__generated, relative_chase_camera_algorithm_reset_for_rider, 120, &|c, seed| {
            let mut r = base(c, seed, 0x16_0fd0);
            globals(c);
            const HOLDER: u32 = DATA + 0x1000;
            const BASE: u32 = DATA + 0x2000;
            const VT: u32 = DATA + 0x3000;
            const VT2: u32 = DATA + 0x4000;
            w(c, DATA + 0x30, HOLDER);
            w(c, HOLDER + 4, BASE);
            w(c, BASE + 0x6c0, VT);
            w(c, VT + 0x38, 0); // delta
            w(c, VT + 0x3c, 0x0000_4000); // fake method
            w(c, DATA + 0x10, VT2);
            w(c, VT2 + 0x28, 0);
            w(c, VT2 + 0x2c, 0x0000_4000);
            let id = (r.next_u32() & 0xffff) as u32;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(id);
            if seed % 2 == 0 {
                // Make the virtual "current rider" call return this id.
                w(c, crate::difftest::fakes::LOG_RESULTS, id);
            } else {
                w(c, crate::difftest::fakes::LOG_RESULTS, id ^ 0x5a5a);
            }
        });
    }


    #[test]
    fn c_ui_group_dtor_matches() {
        check(ee::s13::f_003a5068__generated, c_ui_group_dtor, 150, &|c, seed| {
            let mut r = base(c, seed, 0x3a_5068);
            globals(c);
            pool(c);
            // Empty child list and zeroed embedded list nodes.
            w(c, DATA + 0x74, 0);
            w(c, DATA + 0x7c, 0);
            w(c, DATA + 0x80, 0);
            w(c, DATA + 0x88, 0);
            w(c, DATA + 0x8c, 0);
            c.g[A0][0] = sx(DATA);
            // Even flags: the base destructor must not free test data.
            c.g[A1][0] = [0u64, 2][seed as usize % 2];
            let _ = r.next_u32();
        });
    }

    #[test]
    fn ui_menu_dtor_matches() {
        check(ee::s13::f_003a52a0__generated, ui_menu_dtor, 150, &|c, seed| {
            let mut r = base(c, seed, 0x3a_52a0);
            globals(c);
            pool(c);
            w(c, DATA + 0x74, 0);
            w(c, DATA + 0x7c, 0);
            w(c, DATA + 0x80, 0);
            w(c, DATA + 0x88, 0);
            w(c, DATA + 0x8c, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 2][seed as usize % 2];
            let _ = r.next_u32();
        });
    }

            #[test]
    fn c_keyboard_popup_refresh_entry_text_matches() {
        check(ee::s04::f_001cc848__generated, c_keyboard_popup_refresh_entry_text, 120, &|c, seed| {
            let _ = base(c, seed, 0x1c_c848);
            globals(c);
            const SCREEN: u32 = DATA + 0x1000;
            const NODE: u32 = DATA + 0x2000;
            if seed % 2 == 0 {
                // No screen at all: the text lookup fails.
                w(c, DATA + 0x40, 0);
            } else {
                // A one-object screen whose entry matches the hash name.
                let hash = str_hash(c, 0x0046_6530);
                w(c, DATA + 0x40, SCREEN);
                w(c, SCREEN + 0x3c, 1);
                w(c, SCREEN + 0x38, DATA + 0x2100);
                w(c, DATA + 0x2100, NODE);
                w(c, NODE + 0x38, hash as u32);
                // The list at +0x74 stays empty.
                w(c, DATA + 0x74, 0);
            }
            w(c, DATA + 0x70, [0, 1][seed as usize % 2]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 0;
        });
    }

    #[test]
    fn keyboard_screen_cursor_up_matches() {
        check(ee::s02::f_001806d8__generated, keyboard_screen_cursor_up, 200, &|c, seed| {
            let _ = base(c, seed, 0x18_06d8);
            globals(c);
            c.bus.write32(DATA, 0);
            let row = match seed % 8 {
                0 => 0,
                1 => 9,
                2 => 10,
                3 => 19,
                4 => 18,
                5 => 8,
                6 => 39,
                _ => (seed * 7) % 45,
            };
            w(c, DATA + 0x170, row);
            w(c, DATA + 0xd8, if seed % 2 == 0 { 0 } else { 0x2222 });
            w(c, DATA + 0xe0, if seed % 3 == 0 { 0 } else { 0x3333 });
            c.g[A0][0] = sx(DATA);
        });
    }

            #[test]
    fn be_reward_int_is_peak_complete_matches() {
        check(ee::s01::f_00157a78__generated, be_reward_int_is_peak_complete, 120, &|c, seed| {
            let _ = base(c, seed, 0x15_7a78);
            globals(c);
            // A zeroed BE reward: the goal check only reads it.
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(seed ^ 1);
            c.g[A2][0] = sx(seed ^ 2);
            c.g[A3][0] = sx(seed ^ 3);
        });
    }

    #[test]
    fn rider_metrix_find_hud_slot_matches() {
        check(ee::s00::f_00117a58__generated, rider_metrix_find_hud_slot, 200, &|c, seed| {
            let mut r = base(c, seed, 0x11_7a58);
            globals(c);
            const SLOTS: u32 = DATA + 0x1554;
            let count = 34 + (seed % 5);
            w(c, DATA + 0x1b4, count);
            w(c, DATA + 0x1b0, SLOTS - 0x1554);
            for i in 0..39u32 {
                if i < 35 {
                    w(c, SLOTS + 0x9c * i, 0);
                    continue;
                }
                // 0x34 marks a free slot; otherwise value/total drives the
                // "smallest ratio" search.
                let free = (seed + i) % 5 == 0;
                w(c, SLOTS + 0x9c * i, if free { 0x34 } else { 0 });
                c.bus.write32(SLOTS + 0x9c * i + 4, 0x3f80_0000u32 | (r.next_u32() & 0x003f_ffff));
                c.bus.write32(SLOTS + 0x9c * i + 8, 0x3f00_0000u32 | (r.next_u32() & 0x007f_ffff));
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 17, 33, 34, 36][seed as usize % 5];
        });
    }

    driven!(bolts_count_generated, ee::s01::f_0014d4c8);
    driven!(bolts_count_hand, bolts_count_children);

        #[test]
    fn coll_hull_collide_sphere_tree_matches() {
        check(ee::s11::f_0032fa30__generated, coll_hull_collide_sphere_tree, 80, &|c, seed| {
            let _ = base(c, seed, 0x32_fa30);
            globals(c);
            // A zeroed tree node: the collide call finds nothing.
            const NODE: u32 = DATA + 0x2000;
            w(c, NODE + 0x98, NODE + 0x100);
            w(c, NODE + 0x100 + 0x20, 0);
            w(c, DATA + 0x60, NODE);
            w(c, DATA + 0x80, NODE + 0x100);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = sx(DATA + 0x4000);
        });
    }

    #[test]
    fn coll_sphere_plane_distance_matches() {
        check(ee::s11::f_00330360__generated, coll_sphere_plane_distance, 150, &|c, seed| {
            let mut r = base(c, seed, 0x33_0360);
            globals(c);
            const PLANE: u32 = DATA + 0x100;
            const PN: u32 = DATA + 0x200;
            const OUT: u32 = DATA + 0x300;
            for i in 0..4u32 {
                c.bus.write32(PLANE + 4 * i, r.edge_f32());
                c.bus.write32(PN + 4 * i, r.edge_f32());
                c.bus.write32(DATA + 0x60 + 4 * i, r.edge_f32());
            }
            c.bus.write32(DATA + 0x70, r.f32(0.01));
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(PLANE);
            c.g[A2][0] = sx(PN);
            c.g[A3][0] = sx(OUT);
        });
    }

    #[test]
    fn script_list_on_seq_advance_matches() {
        check(ee::s07::f_00277310__generated, script_list_on_seq_advance, 120, &|c, seed| {
            let _ = base(c, seed, 0x27_7310);
            globals(c);
            // self+0x4 gates the whole thing; obj+0x1c is the pending
            // transition.
            w(c, DATA + 0x4, if seed % 2 == 0 { 0 } else { 1 });
            w(c, DATA + 0xb0, 0);
            w(c, DATA + 0xac, 0);
            w(c, DATA + 0x2000 + 0x1c, [0, 0, 1][seed as usize % 3]);
            w(c, DATA + 0x2000 + 0x12, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.g[A2][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn input_parser_find_identifier_matches() {
        check(ee::s10::f_00321a40__generated, input_parser_find_identifier, 150, &|c, seed| {
            let _ = base(c, seed, 0x32_1a40);
            globals(c);
            const TABLE: u32 = DATA + 0x80e8;
            w(c, DATA + 0x8000 + 0xe4, if seed % 3 == 0 { 0 } else { 3 });
            for i in 0..3u32 {
                let e = TABLE + 0x44 * i;
                for (j, b) in [b"alpha\0".as_slice(), b"beta\0".as_slice(), b"gamma\0".as_slice()][i as usize]
                    .iter()
                    .enumerate()
                {
                    c.bus.write8(e + j as u32, *b);
                }
            }
            // The name looked up.
            let name = b"beta\0";
            for (j, b) in name.iter().enumerate() {
                c.bus.write8(DATA + 0x200 + j as u32, *b);
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
            if seed % 3 == 1 {
                c.bus.write8(DATA + 0x200, b'B');
            }
        });
    }

    #[test]
    fn c_bx_string_op_plus_c_str_str_matches() {
        check(ee::s10::f_003186d0__generated, c_bx_string_op_plus_c_str_str, 100, &|c, seed| {
            let _ = base(c, seed, 0x31_86d0);
            globals(c);
            pool(c);
            heap_init(c);
            // A cBXString at DATA+0x2000 whose buffer has a length word at -8.
            const BUF: u32 = DATA + 0x2100;
            w(c, DATA + 0x2000, BUF);
            w(c, BUF - 8, 4);
            for i in 0..8u32 {
                c.bus.write8(BUF + i, b'x' + i as u8);
            }
            // The global stack temporary (a cBXString) starts out empty.
            w(c, 0x004a_3e90, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { sx(DATA + 0x3000) } else { 0 };
            for i in 0..8u32 {
                c.bus.write8(DATA + 0x3000 + i, b'a' + i as u8);
            }
            c.g[A2][0] = sx(DATA + 0x2000);
        });
    }


    #[test]
    fn tex_man_swizzle_t8_block_matches() {
        check(ee::s12::f_0038f930__generated, tex_man_swizzle_t8_block, 60, &|c, seed| {
            let _ = base(c, seed, 0x38_f930);
            globals(c);
            for i in 0..0x100u32 {
                let v = if seed % 2 == 0 { i } else { (seed.wrapping_mul(7) + i) & 0x1ff };
                w(c, 0x0044_bde0 + 4 * i, v);
                w(c, DATA + 0x1000 + 4 * i, i.wrapping_mul(0x0101_0101));
            }
            c.g[A0][0] = 0;
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn c_bolt_on_fx_interface_create_emitter_matches() {
        check(ee::s09::f_002ecc28__generated, c_bolt_on_fx_interface_create_emitter, 90, &|c, seed| {
            let _ = base(c, seed, 0x2e_cc28);
            globals(c);
            pool(c);
            heap_init(c);
            let mut free_first = seed % 3 == 0;
            for i in 0..15u32 {
                let id = if free_first && i == 0 { u64::MAX } else { 0 };
                if i == 3 {
                    free_first = seed % 5 == 0;
                }
                w(c, DATA + 0x10 + 0xc * i, id as u32);
            }
            w(c, DATA + 0xc, DATA + 0x8000);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.g[A2][0] = sx(DATA + 0x3000);
        });
    }

        #[test]
    fn crowd_man2d_remove_by_type_matches() {
        check(ee::s05::f_00229408__generated, crowd_man2d_remove_by_type, 100, &|c, seed| {
            let _ = base(c, seed, 0x22_9408);
            globals(c);
            for i in 0..128u32 {
                let slot = DATA + 8 + 4 * i;
                match (seed.wrapping_add(i)) % 4 {
                    0 => w(c, slot, 0xffff_ffff),
                    1 => w(c, slot, seed & 0xff),
                    _ => w(c, slot, 0x0100_0000u32 | (i & 0xff)),
                }
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(seed & 0xff);
        });
    }

        #[test]
    fn replay_apply_camera_matches() {
        check(ee::s07::f_0026edd8__generated, replay_apply_camera, 100, &|c, seed| {
            let _ = base(c, seed, 0x26_edd8);
            globals(c);
            const LIST: u32 = DATA + 0x4000;
            let n = (seed % 3) as u32;
            // world -> +0x84 (game) -> +0x84 (rider list)
            let game = WORLD + 0x1000;
            w(c, WORLD + 0x84, game);
            w(c, game + 0x84, LIST);
            w(c, LIST, n);
            for i in 0..3u32 {
                let rider = DATA + 0x5000 + 0x100 * i;
                w(c, LIST + 4 + 4 * i, rider);
                w(c, rider + 0xa8, DATA + 0x6000 + 0x100 * i);
            }
            w(c, 0x0044_5438, 0);
            w(c, DATA + 0x630, [0, 1, 2][seed as usize % 3]);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn script_camera_algo_ctor_matches() {
        check(ee::s02::f_00169418__generated, script_camera_algo_ctor, 100, &|c, seed| {
            let _ = base(c, seed, 0x16_9418);
            globals(c);
            let game = WORLD + 0x1000;
            let list = WORLD + 0x2000;
            let ctrl = WORLD + 0x3000;
            w(c, WORLD + 0x84, game);
            w(c, game + 0x84, list);
            w(c, list + 4, ctrl);
            w(c, ctrl + 0xac, seed ^ 0x77);
            w(c, GAME_GP.wrapping_sub(0x6548), 0x42c8_0000);
            c.g[A0][0] = sx(DATA);
        });
    }

        #[test]
    fn ui_text_set_ascii_string_matches() {
        check(ee::s13::f_003a0c70__generated, ui_text_set_ascii_string, 60, &|c, seed| {
            let mut r = base(c, seed, 0x3a_0c70);
            globals(c);
            let vt = vtable(c, &mut r, &[0x28, 0xc0]);
            w(c, DATA + 8, vt);
            // A short ASCII string.
            let n = (seed % 20) as u32;
            for i in 0..n {
                c.bus.write8(DATA + 0x200 + i, b'a' + (i % 26) as u8);
            }
            c.bus.write8(DATA + 0x200 + n, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
        });
    }

    #[test]
    fn bebagt_int_test_uber_trick_bit_a_matches() {
        check(ee::s01::f_0014ffb8__generated, bebagt_int_test_uber_trick_bit_a, 100, &|c, seed| {
            let _ = base(c, seed, 0x14_ffb8);
            globals(c);
            w(c, DATA, if seed % 2 == 0 { 0xffff } else { 0xaaaa });
            c.g[A0][0] = 0;
            c.g[A1][0] = sx(seed % 8);
            c.g[A2][0] = sx(seed % 4);
            c.g[A3][0] = sx(seed % 16);
        });
    }

    #[test]
    fn bebagt_int_test_uber_trick_bit_b_matches() {
        check(ee::s01::f_00150048__generated, bebagt_int_test_uber_trick_bit_b, 100, &|c, seed| {
            let _ = base(c, seed, 0x15_0048);
            globals(c);
            w(c, DATA, if seed % 2 == 0 { 0xffff } else { 0x5555 });
            c.g[A0][0] = 0;
            c.g[A1][0] = sx(seed % 8);
            c.g[A2][0] = sx(seed % 4);
            c.g[A3][0] = sx(seed % 16);
        });
    }

    #[test]
    fn c_flag_manager_ctor_matches() {
        check(ee::s11::f_0034c428__generated, c_flag_manager_ctor, 40, &|c, seed| {
            let _ = base(c, seed, 0x34_c428);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(seed);
        });
    }

        #[test]
    fn game_module_reset_scripts_and_world_matches() {
        check(ee::s06::f_002300f0__generated, game_module_reset_scripts_and_world, 40, &|c, seed| {
            let _ = base(c, seed, 0x23_00f0);
            globals(c);
            pool(c);
            heap_init(c);
            w(c, DATA + 0x30, DATA + 0x1000);
            w(c, DATA + 0x34, DATA + 0x2000);
            w(c, DATA + 0xc, DATA + 0x3000);
            w(c, DATA + 0x10, DATA + 0x4000);
            w(c, GAME_GP.wrapping_add(0xce8), 0);
            let _ = seed;
            c.g[A0][0] = sx(DATA);
        });
    }

        #[test]
    fn ui_object_show_clamp_alpha_half_matches() {
        check(ee::s02::f_00194498__generated, ui_object_show_clamp_alpha_half, 100, &|c, seed| {
            let mut r = base(c, seed, 0x19_4498);
            globals(c);
            let vt = vtable(c, &mut r, &[0x40, 0x58]);
            w(c, DATA + 8, vt);
            // The colour at +0x1c: x drives the clamp.
            c.bus.write32(DATA + 0x1c, if seed % 2 == 0 { 0x3f80_0000 } else { 0x3e80_0000 });
            c.bus.write32(DATA + 0x20, 0x3f00_0000);
            c.bus.write32(DATA + 0x24, 0x3e80_0000);
            c.bus.write32(DATA + 0x28, 0x3f40_0000);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn fe_locale_unload_fe_tables_matches() {
        check(ee::s02::f_00195d50__generated, fe_locale_unload_fe_tables, 60, &|c, seed| {
            let _ = base(c, seed, 0x19_5d50);
            globals(c);
            c.bus.write8(DATA + 3, if seed % 2 == 0 { 0 } else { 1 });
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn bescore_int_update_record_level_matches() {
        check(ee::s01::f_00155390__generated, bescore_int_update_record_level, 150, &|c, seed| {
            let _ = base(c, seed, 0x15_5390);
            globals(c);
            // Three short thresholds per record.
            for i in 0..3u32 {
                c.bus.write16(0x0044_0ed0 + 6 * i, 100u16 + i as u16 * 100);
                c.bus.write16(0x0044_0ed0 + 6 * i + 2, 300u16 + i as u16 * 100);
                c.bus.write16(0x0044_0ed0 + 6 * i + 4, 600u16 + i as u16 * 100);
            }
            let idx = (seed % 3) as u32;
            c.bus.write8(DATA + idx + 0xbb8, (seed % 4) as u8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(idx);
            c.g[A2][0] = [0u64, 150, 350, 650, 80][seed as usize % 5];
        });
    }

    #[test]
    fn c_fe_state_on_activate_matches() {
        check(ee::s02::f_00186518__generated, c_fe_state_on_activate, 100, &|c, seed| {
            let _ = base(c, seed, 0x18_6518);
            globals(c);
            const LIST: u32 = DATA + 0x2000;
            // List: head at +4; a sentinel-only list on some seeds.
            if seed % 3 == 0 {
                w(c, LIST + 4, LIST);
            } else {
                w(c, DATA + 0x1000 + 0x3c, 1);
                w(c, DATA + 0x1000 + 0x38, DATA + 0x1100);
                w(c, DATA + 0x1100, DATA + 0x1200);
                w(c, DATA + 0x1200 + 0x38, 0x1234);
                w(c, LIST + 4, DATA + 0x1000);
            }
            w(c, DATA + 0x24, LIST);
            w(c, DATA + 0xc, seed & 0x7f);
            w(c, GAME_GP.wrapping_add(0x1d90), 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(seed);
        });
    }

    #[test]
    fn be_library_shutdown_matches() {
        check(ee::s01::f_0014dd98__generated, be_library_shutdown, 60, &|c, seed| {
            let _ = base(c, seed, 0x14_dd98);
            globals(c);
            // No BE library and no BE states: every call is a no-op or fake.
            w(c, GAME_GP.wrapping_sub(0x1ef8), 0);
            w(c, GAME_GP.wrapping_sub(0x1f04), 0);
            w(c, GAME_GP.wrapping_sub(0x1f00), 0);
            for i in 0..3u32 {
                w(c, 0x0053_08a8 + 4 * i, 0);
            }
        });
    }

    #[test]
    fn script_anim_bank_man_free_entry_matches() {
        check(ee::s07::f_00275760__generated, script_anim_bank_man_free_entry, 100, &|c, seed| {
            let mut r = base(c, seed, 0x27_5760);
            globals(c);
            const TBL: u32 = DATA + 0x1000;
            w(c, GAME_GP.wrapping_add(0xd08), TBL);
            for i in 0..4u32 {
                w(c, TBL + 4 * i, if i == 2 { DATA + 0x3000 } else { 0 });
            }
            w(c, DATA + 0x2000 + 4, (seed % 4) as u32);
            w(c, DATA + 0x2000 + 0xc, [0, 1, 1][seed as usize % 3]);
            w(c, DATA + 0x2000 + 0x10, [0, DATA + 0x4000, 0][seed as usize % 3]);
            w(c, DATA + 0x2000, 0x1111);
            w(c, DATA + 0x2000 + 8, 0x2222);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            let _ = r.next_u32();
        });
    }

        #[test]
    fn script_engine_list_start_matches() {
        check(ee::s07::f_00278f68__generated, script_engine_list_start, 200, &|c, seed| {
            let mut r = base(c, seed, 0x27_8f68);
            globals(c);
            // A list table at +0x54c; the list state at +0x550.
            w(c, DATA + 0x54c, DATA + 0x3000);
            for i in 0..4u32 {
                w(c, DATA + 0x3000 + 0xcc * i, 0);
            }
            w(c, DATA + 0x550, [0, 1, 2, 3][seed as usize % 4]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 1, 2][seed as usize % 3];
            c.g[A2][0] = [0u64, 1][seed as usize % 2];
            c.g[A3][0] = r.next_u32() as u64;
        });
    }
}
