//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 small generated functions, each with a differential
//! test against the generated body.
//!
//! Mostly front-end state initialisers, string/allocator plumbing, small
//! list/queue resets and assorted destructors. Calls follow [`crate::call`]
//! (same frame, same registers at every call, unwind check after each call);
//! loops that check events use [`crate::events`] at the original's back edge.
//!
//! Test fixtures: `machine` (scrambled registers, fake callees), the BELibrary
//! dummy at `gp-0x1f04`, the static empty `cBXString` at `gp+0xda0` and — where
//! a real allocation happens — the game's own `DLHeap`/`VramHeap` set-up.
//! The destructors are driven with flag 0 (the `flags & 1` delete branch would
//! free an arbitrary test pointer) and the screen initialisers with a null UI
//! engine, the same fixtures the earlier waves use for these shapes. The
//! pause-menu modes get a dummy engine whose state lists are properly empty,
//! because their callee `OVScreenPauseFreeride_Init` asks
//! `UIStateMgr_FindState` for one (an empty list makes the walk return at
//! once; with a null engine it walks the screen built at address 0).

use crate::abi;
use crate::call::{call_ptr, guest, sp, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_8950, c_challenger_dtor, "cChallenger_dtor"),
    (
        0x0026_a8b8,
        path_get_dir_at_distance,
        "Path_GetDirAtDistance",
    ),
    (
        0x0028_f2c0,
        ssx_audio_system_on_replay_end,
        "SSXAudioSystem_OnReplayEnd",
    ),
    (
        0x0018_d280,
        c_fe_state_options_save_load_init,
        "cFEStateOptionsSaveLoad_Init",
    ),
    (
        0x0028_2a18,
        script_engine_seq_listener_on_stop,
        "ScriptEngine_SeqListenerOnStop",
    ),
    (
        0x0014_7cb8,
        be_new_player_int_get_difficulty,
        "BENewPlayerInt_GetDifficulty",
    ),
    (
        0x0033_0710,
        coll_sphere_tree_transform,
        "CollSphereTree_Transform",
    ),
    (0x0034_2ba0, boost_modifier_v06, "BoostModifier_v06"),
    (
        0x0014_aea8,
        be_bolt_int_save_equip_flags,
        "BEBoltInt_SaveEquipFlags",
    ),
    (
        0x002b_6868,
        world_trigger_manager_mark_played,
        "WorldTriggerManager_MarkPlayed",
    ),
    (
        0x002b_6c90,
        c_world_sound_insert_emitters,
        "cWorldSound_InsertEmitters",
    ),
    (0x0031_80b8, c_bx_string_release, "cBXString_Release"),
    (
        0x0031_8120,
        c_bx_string_copy_before_write,
        "cBXString_CopyBeforeWrite",
    ),
    (
        0x0031_85c8,
        c_bx_string_op_plus_str_str,
        "cBXString_OpPlusStrStr",
    ),
    (
        0x002e_cbc0,
        c_bolt_on_fx_interface_find_or_create_emitter,
        "cBoltOnFXInterface_FindOrCreateEmitter",
    ),
    (
        0x003a_82c8,
        c_world_cache_init_hull_slots,
        "cWorldCache_InitHullSlots",
    ),
    (
        0x0034_42e0,
        crowd_man3d_group_reset,
        "CrowdMan3d_GroupReset",
    ),
    (0x0037_0af8, emitter_load, "Emitter_Load"),
    (
        0x001d_4368,
        c_fe_state_trophy_room_init,
        "cFEStateTrophyRoom_Init",
    ),
    (0x001d_5078, c_fe_state_relogin_init, "cFEStateRelogin_Init"),
    (0x0022_9278, crowd_man2d_dtor, "CrowdMan2d_dtor"),
    (
        0x0023_3260,
        c_game_load_screen_enter,
        "cGameLoadScreen_Enter",
    ),
    (0x0023_38c8, c_fe_load_screen_enter, "cFELoadScreen_Enter"),
    (
        0x0024_5d50,
        c_fe_load_state_in_lodge_init,
        "cFELoadStateInLodge_Init",
    ),
    (
        0x0036_71c8,
        tex_man_free_nb_tex_info,
        "TexMan_FreeNBTexInfo",
    ),
    (
        0x0036_73d8,
        tex_man_alloc_frame_buffer,
        "TexMan_AllocFrameBuffer",
    ),
    (
        0x001f_74e0,
        ov_screen_bc_start_on_widget_created,
        "OVScreenBCStart_OnWidgetCreated",
    ),
    (
        0x0021_1aa8,
        ov_screen_profile_save_pda_v33,
        "OVScreenProfileSavePDA_v33",
    ),
    (
        0x0018_ead8,
        fe_screen_select_profile_init,
        "FEScreenSelectProfile_Init",
    ),
    (
        0x0019_0858,
        c_fe_state_rider_bio_ctor,
        "cFEStateRiderBio_ctor",
    ),
    (
        0x001d_3890,
        c_fe_state_peak_room_init,
        "cFEStatePeakRoom_Init",
    ),
    (
        0x001d_9db8,
        c_fe_state_net_help_popup_init,
        "cFEStateNetHelpPopup_Init",
    ),
    (
        0x001d_a4a8,
        c_fe_state_net_help_popup_clear_fields,
        "cFEStateNetHelpPopup_ClearFields",
    ),
    (0x0020_e938, ov_screen_trophy_init, "OVScreenTrophy_Init"),
    (0x0026_b5e0, path_data_find_marker, "PathData_FindMarker"),
    (
        0x0018_2420,
        c_fe_state_cheat_char_select_init,
        "cFEStateCheatCharSelect_Init",
    ),
    (
        0x0019_49e0,
        c_fe_state_title_on_widget_action,
        "cFEStateTitle_OnWidgetAction",
    ),
    (
        0x001f_8ee0,
        ov_screen_pause_freeride_init_mode1,
        "OVScreenPauseFreeride_InitMode1",
    ),
    (
        0x001f_8f48,
        ov_screen_pause_freeride_init_mode2,
        "OVScreenPauseFreeride_InitMode2",
    ),
    (
        0x001f_8fb0,
        ov_screen_pause_freeride_init_mode3,
        "OVScreenPauseFreeride_InitMode3",
    ),
    (
        0x001f_9018,
        ov_screen_pause_freeride_init_mode4,
        "OVScreenPauseFreeride_InitMode4",
    ),
    (
        0x001f_9080,
        ov_screen_pause_freeride_init_mode5,
        "OVScreenPauseFreeride_InitMode5",
    ),
    (
        0x0015_5208,
        be_score_int_get_high_score_name,
        "BEScoreInt_GetHighScoreName",
    ),
    (
        0x0034_fe28,
        world_obj_replace_with_dead_node,
        "WorldObj_ReplaceWithDeadNode",
    ),
    (
        0x0028_3518,
        script_sound_bank_man_dtor,
        "ScriptSoundBankMan_dtor",
    ),
    (
        0x0014_f898,
        be_option_int_swap_player_settings,
        "BEOptionInt_SwapPlayerSettings",
    ),
    (
        0x0027_5498,
        script_anim_bank_man_dtor,
        "ScriptAnimBankMan_dtor",
    ),
    (0x0031_4e88, game_anim_map_find_hash, "GameAnimMap_FindHash"),
    (
        0x002d_56b0,
        visual_effects_man_emitter_holder_dtor,
        "VisualEffectsMan_EmitterHolderDtor",
    ),
];

// ---------------------------------------------------------------------------
// Memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

#[inline(always)]
fn lh(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

#[inline(always)]
fn lhu(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lhu(c)
}

#[inline(always)]
fn lb(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lb(c)
}

#[inline(always)]
fn ld(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).ld(c)
}

#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sw(c, v);
}

#[inline(always)]
fn sh(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sh(c, v);
}

#[inline(always)]
fn sb(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sb(c, v);
}

// ---------------------------------------------------------------------------
// cBXString, cChallenger.

/// `cChallenger_dtor(cChallenger* self, int flags)` (0x268950): install the
/// challenger vtable, destroy the two embedded strings, then delete the
/// object when the low flag bit is set.
pub fn c_challenger_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        let this = c.g[A0][0];
        let flags = c.g[A1][0];
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0x0048_10e0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = flags;
        frame.save_ra(c, 0);
        c.g[S1][0] = this;
        sw(c, p(this as u32, 0x3c), 0x0048_10e0);
        c.g[A0][0] = sx((this as u32).wrapping_add(4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8980)?;
        c.g[A0][0] = this;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_898c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0026_89a0)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `sVec4_12F398 Path_GetDirAtDistance(Path* self, float t)` (0x26a8b8):
/// walk the path segments (16 bytes each, `fC` at +0xc) subtracting each
/// segment's length until `t` falls inside one; return that segment's
/// direction (`f0`, `f4`, `f8`) with `fC` zeroed.
pub fn path_get_dir_at_distance(c: &mut Cpu) {
    c.cycles += 2;
    let out = c.g[A0][0];
    let path = c.g[A1][0];
    c.g[A2][0] = out;
    c.g[A3][0] = 0;
    let mut i: u32 = 0;
    loop {
        c.cycles += 6;
        let segs = lw(c, p(path as u32, 0x18)) as u32;
        c.g[V1][0] = sx(i << 4);
        c.g[V0][0] = sx((lw(c, p(path as u32, 8)) as u32).wrapping_sub(1));
        let last = c.g[A3][0] == c.g[V0][0];
        c.g[A0][0] = sx(segs.wrapping_add(i << 4));
        let e = c.g[A0][0] as u32;
        let mut selected = last;
        if !last {
            c.cycles += 5;
            c.lwc1(0, p(e, 0xc));
            c.c_le_s(12, 0);
            if !c.fcc() {
                c.sub_s(12, 12, 0);
            } else {
                selected = true;
            }
        }
        if selected {
            c.cycles += 8;
            c.lwc1(0, e);
            c.swc1(0, out as u32);
            c.lwc1(1, p(e, 4));
            c.swc1(1, p(out as u32, 4));
            c.lwc1(0, p(e, 8));
            sw(c, p(out as u32, 0xc), 0);
            c.swc1(0, p(out as u32, 8));
            break;
        }
        c.cycles += 2;
        i = i.wrapping_add(1);
        c.g[A3][0] = sx(i);
        if event_due(c) {
            deliver_event_here(c, 0x0026_a8c0);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `SSXAudioSystem_OnReplayEnd(SSXAudioSystem* self)` (0x28f2c0): stop the
/// music player backing the replay (`+0x6294`) or send the stop command
/// (`0x24`) through the bank manager vtable (`+0x5558`), then clear both
/// replay flags.
pub fn ssx_audio_system_on_replay_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let base = this as u32;
        c.g[V0][0] = lw(c, p(base, 0x6294));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(p(base, 0x118));
            call_ptr(c, 0x002b_3ac0, 0x0028_f2e4)?;
            sw(c, p(base, 0x6298), 0);
            c.cycles += 5;
            sw(c, p(base, 0x6294), 0);
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[V0][0] = lw(c, p(base, 0x6298));
        c.cycles += 2;
        let idle = c.g[V0][0] == 0;
        c.g[A2][0] = sx(p(base, 0x118));
        if !idle {
            c.cycles += 6;
            c.g[V1][0] = lw(c, p(base, 0x5558));
            c.g[A1][0] = 0x24;
            c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x20));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_f30c)?;
        }
        c.cycles += 1;
        sw(c, p(base, 0x6298), 0);
        c.cycles += 5;
        sw(c, p(base, 0x6294), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end state initialisers: `StrHash` of a screen name, create the
// screen through the UI engine, store it and start its thread.

fn fe_state_screen_init(c: &mut Cpu, hash: u32, rets: [u32; 3]) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    let this = c.g[A0][0];
    c.g[S0][0] = this;
    frame.save_ra(c, 0);
    frame.save(c, S1, 0x10);
    c.g[A0][0] = sx(hash);
    c.g[S1][0] = lw(c, p(this as u32, 0x10));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = this;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0039_7cc0, rets[1])?;
    let no_screen = c.g[V0][0] == 0;
    sw(c, p(this as u32, 0x40), c.g[V0][0]);
    if !no_screen {
        c.cycles += 4;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0039_c870, rets[2])?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFEStateOptionsSaveLoad_Init(cFEStateOptionsSaveLoad* self)` (0x18d280).
pub fn c_fe_state_options_save_load_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0045_e0c0, [0x0018_d2a4, 0x0018_d2b8, 0x0018_d2d0])
    })
}

/// `cFEStateTrophyRoom_Init(cFEStateTrophyRoom* self)` (0x1d4368).
pub fn c_fe_state_trophy_room_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_76c8, [0x001d_438c, 0x001d_43a0, 0x001d_43b8])
    })
}

/// `cFEStateRelogin_Init(cFEStateRelogin* self)` (0x1d5078).
pub fn c_fe_state_relogin_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_7778, [0x001d_509c, 0x001d_50b0, 0x001d_50c8])
    })
}

/// `cFELoadStateInLodge_Init(cFELoadStateInLodge* self)` (0x245d50).
pub fn c_fe_load_state_in_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0047_bfa0, [0x0024_5d74, 0x0024_5d88, 0x0024_5da0])
    })
}

/// `FEScreenSelectProfile_Init(FEScreenSelectProfile* self)` (0x18ead8).
pub fn fe_screen_select_profile_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0045_e240, [0x0018_eafc, 0x0018_eb10, 0x0018_eb28])
    })
}

/// `cFEStatePeakRoom_Init(cFEStatePeakRoom* self)` (0x1d3890).
pub fn c_fe_state_peak_room_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_7648, [0x001d_38b4, 0x001d_38c8, 0x001d_38e0])
    })
}

/// `cFEStateNetHelpPopup_Init(cFEStateNetHelpPopup* self)` (0x1d9db8).
pub fn c_fe_state_net_help_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_79a0, [0x001d_9ddc, 0x001d_9df0, 0x001d_9e08])
    })
}

/// `cFEStateCheatCharSelect_Init(cFEStateCheatCharSelect* self)` (0x182420).
pub fn c_fe_state_cheat_char_select_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0045_d780, [0x0018_2444, 0x0018_2458, 0x0018_2470])
    })
}

/// `OVScreenTrophy_Init(OVScreenTrophy* self)` (0x20e938).
pub fn ov_screen_trophy_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0047_1cd8, [0x0020_e95c, 0x0020_e970, 0x0020_e988])
    })
}

// ---------------------------------------------------------------------------
// cBXString refcounting.

/// `void cBXString_Release(cBXString* self)` (0x3180b8): drop the buffer's
/// reference count and free it at zero; a released string becomes the static
/// empty string (`D_004A3E90`).
pub fn c_bx_string_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x0048_dcd4;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let s = lw(c, this as u32);
        c.g[V1][0] = s;
        if s != c.g[V0][0] {
            c.cycles += 4;
            c.g[V0][0] = lw(c, (s as u32).wrapping_sub(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let live = c.g[V0][0] != 0;
            sw(c, (s as u32).wrapping_sub(0xc), c.g[V0][0]);
            let mut skip_reload = false;
            if !live {
                c.cycles += 4;
                c.g[V0][0] = lw(c, this as u32);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                let empty = c.g[A0][0] == 0;
                c.g[V0][0] = lw(c, gp(c, 0xda0));
                if empty {
                    skip_reload = true;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0031_7e98, 0x0031_8104)?;
                }
            }
            if !skip_reload {
                c.cycles += 1;
                c.g[V0][0] = lw(c, gp(c, 0xda0));
            }
            c.cycles += 2;
            sw(c, this as u32, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_CopyBeforeWrite(cBXString* self)` (0x318120): when the
/// buffer is shared, release it, allocate a private copy and memcpy the bytes
/// (length + 1) over.
pub fn c_bx_string_copy_before_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save(c, S0, 0x20);
        let s = lw(c, this as u32);
        c.g[S0][0] = s;
        // The original leaves the `refcount < 2` test in v0; the copy path
        // overwrites it with the callees' results.
        let private = (lw(c, (s as u32).wrapping_sub(0xc)) as i64) < 2;
        c.g[V0][0] = private as u64;
        if !private {
            c.cycles += 11;
            call_ptr(c, 0x0031_80b8, 0x0031_8150)?;
            c.g[A1][0] = lw(c, (s as u32).wrapping_sub(8));
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_8040, 0x0031_815c)?;
            c.g[A2][0] = lw(c, (s as u32).wrapping_sub(8));
            c.g[A1][0] = s;
            c.g[A0][0] = lw(c, this as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0041_605c, 0x0031_8170)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cBXString* cBXString_OpPlusStrStr(cBXString* self, cBXString* a,
/// cBXString* b)` (0x3185c8): concatenate the two strings' buffers into a
/// temporary, copy-construct `self` from it and destroy the temporary.
pub fn c_bx_string_op_plus_str_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[V1][0] = lw(c, gp(c, 0xda0));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        frame_save_16(c);
        c.g[S0][0] = c.g[A0][0];
        sw(c, c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[V1][0] = lw(c, c.g[A1][0] as u32);
        c.g[V0][0] = lw(c, c.g[A2][0] as u32);
        c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_sub(8));
        c.g[A2][0] = c.g[V1][0];
        c.g[A3][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(8));
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0031_8540, 0x0031_8600)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7fe8, 0x0031_860c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0031_8618)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = ld(c, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `sd ra, 0x10(sp); sq s0, 0x20(sp)` of [`c_bx_string_op_plus_str_str`].
#[inline(always)]
fn frame_save_16(c: &mut Cpu) {
    let a = c.g[SP][0] as u32;
    c.sq(16, a.wrapping_add(0x20));
    let ra = c.g[RA][0];
    c.sd(a.wrapping_add(0x10), ra);
}

// ---------------------------------------------------------------------------
// Script engine listeners.

/// `ScriptEngine_SeqListenerOnStop(ScriptEngine* self, ScriptSeqMsg* msg)`
/// (0x282a18): tell the engine's sequence callback (vtable slot 11) that the
/// sequence stopped, mark the sequence object's state 2 and kill the process
/// when it is still running.
pub fn script_engine_seq_listener_on_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        let msg = c.g[A1][0];
        c.g[V0][0] = lw(c, p(this as u32, 0x2a8));
        c.g[S0][0] = lw(c, p(msg as u32, 0xc));
        let seq = c.g[S0][0] as u32;
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x58));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x5c));
        c.g[A2][0] = lw(c, p(seq, 8));
        c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V1][0] as u32;
        c.g[A3][0] = lw(c, p(seq, 0xc));
        call_ptr(c, target, 0x0028_2a4c)?;
        c.g[V0][0] = 2;
        c.g[V1][0] = lh(c, p(seq, 0x10));
        let done = c.g[V1][0] == 0;
        sw(c, p(seq, 4), c.g[V0][0]);
        if !done {
            c.cycles += 3;
            c.g[A1][0] = lw(c, p(seq, 0x1c));
            c.g[A0][0] = this;
            call_ptr(c, 0x0028_2540, 0x0028_2a68)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BENewPlayerInt_GetDifficulty(int which, void* player)` (0x147cb8):
/// the rider's character row (`D_004A6CA8 + id*0xF88`) holds two shorts; take
/// `+0x284` when `which` is zero and `+0x280` otherwise.
pub fn be_new_player_int_get_difficulty(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let which = c.g[A1][0];
        c.g[S0][0] = which;
        call_ptr(c, 0x0014_a080, 0x0014_7cd0)?;
        c.g[A0][0] = c.g[V0][0];
        if which != 0 {
            c.cycles += 6;
            c.g[V1][0] = 0xf88;
            c.g[V0][0] = 0x004a_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = 0x004a_6ca8;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = lh(c, p(c.g[V1][0] as u32, 0x280));
        } else {
            c.cycles += 7;
            c.g[V1][0] = 0xf88;
            c.g[V0][0] = 0x004a_0000;
            c.mult(0, V1, A0, V1);
            c.g[V0][0] = 0x004a_6ca8;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = lh(c, p(c.g[V1][0] as u32, 0x284));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CollSphereTree_Transform(CollSphereTree* self, SphereTree* tree)`
/// (0x330710): copy the tree's matrix block into the collision tree, apply
/// the transform and flag the tree dirty.
pub fn coll_sphere_tree_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let tree = c.g[A1][0];
        c.g[S1][0] = tree;
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[A0][0] = lw(c, p(this as u32, 0x68));
        c.g[A1][0] = lw(c, p(this as u32, 0x60));
        call_ptr(c, 0x0032_7cc8, 0x0033_073c)?;
        c.g[V1][0] = lw(c, p(this as u32, 0x68));
        c.g[A1][0] = tree;
        c.mov_s(12, 20);
        sw(c, p(this as u32, 0x60), c.g[V1][0]);
        c.g[A0][0] = c.g[V1][0];
        call_ptr(c, 0x0032_c770, 0x0033_0754)?;
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        sw(c, p(this as u32, 4), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `BoostModifier_v06(BoostModifier* self, void* stream)` (0x342ba0): save
/// the base modifier, then write `self+0x10` (0x30 bytes) and `self+0x40`
/// through two stream vtable slots.
pub fn boost_modifier_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let stream = c.g[A1][0];
        c.g[S0][0] = stream;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        call_ptr(c, 0x0035_84b8, 0x0034_2bbc)?;
        c.g[V0][0] = lw(c, stream as u32);
        c.g[A1][0] = sx((this as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x30;
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 8));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((stream as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_2bd8)?;
        c.g[V0][0] = lw(c, stream as u32);
        c.g[A1][0] = lw(c, p(this as u32, 0x40));
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x28));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x2c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((stream as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_2bf0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEBoltInt_SaveEquipFlags(int a1, int a2, int self)` (0x14aea8): for each
/// bolt entry (`{u16 pad; u16 flags; u16 kind; u16 pad}` at +0x290, `a1*0x9B50
/// + a2*0xF88` into `D_004A6CA8`), set bit 2 when bit 4 is set and clear it
/// otherwise.
pub fn be_bolt_int_save_equip_flags(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0xf88;
    c.g[V1][0] = 0x9b50;
    c.mult(0, A2, A2, V0);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = 0x004a_0000;
    c.g[V0][0] = 0x004a_6ca8;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x28c));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = sx(p(c.g[A1][0] as u32, 0x290));
    if !none {
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0xffff_ffff_ffff_fffb;
        loop {
            c.cycles += 4;
            c.g[V1][0] = lhu(c, p(c.g[A1][0] as u32, 2));
            c.g[V0][0] = c.g[V1][0] & 0x10;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[V1][0] | 4;
            if !set {
                c.cycles += 1;
                c.g[V0][0] = c.g[V1][0] & c.g[A2][0];
            }
            c.cycles += 4;
            sh(c, p(c.g[A1][0] as u32, 2), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = c.g[A0][0] != 0;
            c.g[A1][0] = sx(p(c.g[A1][0] as u32, 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_aee0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `WorldTriggerManager_MarkPlayed(WorldTriggerManager* self, int id)`
/// (0x2b6868): remember the id as played (append to the 200-entry ring,
/// invalidate the cached last id) unless it already was.
pub fn world_trigger_manager_mark_played(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let id = c.g[A1][0];
        c.g[S1][0] = id;
        call_ptr(c, 0x002b_6808, 0x002b_6884)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[V1][0] = lw(c, p(this as u32, 0x324));
            c.g[V0][0] = ((c.g[V1][0] as i64) < 200) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            if !full {
                c.cycles += 5;
                c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
                sw(c, p(c.g[V0][0] as u32, 4), id);
                c.g[V1][0] = lw(c, p(this as u32, 0x324));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                sw(c, p(this as u32, 0x324), c.g[V1][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            sw(c, p(this as u32, 0x32c), c.g[V0][0]);
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cWorldSound_InsertEmitters(cWorldSound* self, EmitterEntry* entries,
/// int unused, int n)` (0x2b6c90): insert each active emitter entry, 0x18
/// bytes apiece; always true.
pub fn c_world_sound_insert_emitters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        let self_ = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = self_;
        frame.save(c, S1, 0x20);
        let n = c.g[A3][0];
        let none = (n as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            let mut entries = c.g[A1][0];
            c.g[S0][0] = entries;
            let mut count = n;
            c.g[S1][0] = count;
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(entries as u32, 0xc));
                c.g[A0][0] = self_;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = entries;
                    call_ptr(c, 0x002b_7318, 0x002b_6ccc)?;
                }
                c.cycles += 3;
                count = sx((count as u32).wrapping_sub(1));
                c.g[S1][0] = count;
                let more = count != 0;
                entries = sx((entries as u32).wrapping_add(0x18));
                c.g[S0][0] = entries;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_6cb8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PadQueue_StopAllActuators(PadQueue* self)` (0x326ae0): stop the actuators
/// of every queued pad device.
pub fn pad_queue_stop_all_actuators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(this as u32, 0x2ee8));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(p(this as u32, 0x2eec));
            c.g[A0][0] = lw(c, p(this as u32, 0x2eec));
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                call_ptr(c, 0x0032_6cf0, 0x0032_6b1c)?;
                c.g[V0][0] = lw(c, p(this as u32, 0x2ee8));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0));
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6b10);
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
    })
}

/// `int cBoltOnFXInterface_FindOrCreateEmitter(cBoltOnFXInterface* self,
/// int a1, int a2)` (0x2ecbc0): find the first of the 15 `{id, a, b}` slots
/// matching `(a2, a1)` (id != -1) and return its index, else create one.
pub fn c_bolt_on_fx_interface_find_or_create_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        let mut i = c.g[A3][0];
        let mut slot = c.g[V1][0];
        loop {
            let mut found = false;
            c.cycles += 3;
            c.g[V0][0] = lw(c, (slot as u32).wrapping_sub(8));
            if c.g[V0][0] == c.g[T0][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, (slot as u32).wrapping_sub(4));
                if c.g[V0][0] != c.g[A2][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, slot as u32);
                    if c.g[V0][0] != c.g[A1][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        found = true;
                    }
                }
            }
            i = c.g[A3][0];
            slot = c.g[V1][0];
            if found {
                c.g[V0][0] = i;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((i as i64) < 15) as u64;
            let more = c.g[V0][0] != 0;
            slot = sx((slot as u32).wrapping_add(0xc));
            c.g[V1][0] = slot;
            if !more {
                c.cycles += 2;
                call_ptr(c, 0x002e_cc28, 0x002e_cc18)?;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_cbd8);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cWorldCache_InitHullSlots(cWorldCache* self)` (0x3a82c8): ask the streamer
/// how many hulls there are and zero each slot (six words, 0x18 bytes).
pub fn c_world_cache_init_hull_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x10));
        call_ptr(c, 0x003a_9ab0, 0x003a_82e0)?;
        c.g[A0][0] = 0;
        let none = c.g[V0][0] == 0;
        sw(c, p(this as u32, 0x3ec), c.g[V0][0]);
        if !none {
            c.cycles += 1;
            c.g[V1][0] = sx(p(this as u32, 0x3fc));
            loop {
                c.cycles += 11;
                sw(c, (c.g[V1][0] as u32).wrapping_sub(0xc), 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                sw(c, (c.g[V1][0] as u32).wrapping_sub(8), 0);
                sw(c, (c.g[V1][0] as u32).wrapping_sub(4), 0);
                sw(c, (c.g[V1][0] as u32).wrapping_add(4), 0);
                sw(c, (c.g[V1][0] as u32).wrapping_add(8), 0);
                sw(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = lw(c, p(this as u32, 0x3ec));
                c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_82f0);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdMan3d_GroupReset(CrowdMan3d* self)` (0x3442e0): clear the group's
/// counters and reset the 32 slots (`{id = -1; value = 0}`, 8 bytes each).
pub fn crowd_man3d_group_reset(c: &mut Cpu) {
    c.cycles += 16;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A2][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    sw(c, this, 0);
    sw(c, p(this, 4), c.g[V0][0]);
    c.g[V1][0] = sx(this.wrapping_add(0x10));
    sw(c, p(this, 0x1dc), 0);
    c.g[A2][0] = c.g[A2][0] | 0xffff;
    sw(c, p(this, 8), c.g[V0][0]);
    c.g[A1][0] = 0x1f;
    sw(c, p(this, 0x1e8), 0);
    sw(c, p(this, 0x1ec), 0);
    sw(c, p(this, 0x1f0), 0);
    sw(c, p(this, 0x114), 0);
    sw(c, p(this, 0x110), 0);
    loop {
        c.cycles += 7;
        sw(c, (c.g[V1][0] as u32).wrapping_sub(4), c.g[A2][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        sw(c, c.g[V1][0] as u32, 0);
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_4320);
        }
    }
}

/// `Emitter_Load(Emitter* self, void* stream)` (0x370af8): read the 12-byte
/// record (`f0`, `f18`, `c`) through the stream vtable and compute the
/// descriptor bounds.
pub fn emitter_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        let stream = c.g[A1][0];
        c.g[A3][0] = stream;
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sp(c) as u64;
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A2][0] = 0xc;
        c.g[V1][0] = lw(c, stream as u32);
        c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x10));
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((stream as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0037_0b28)?;
        c.lwc1(1, sp(c));
        c.g[A0][0] = sx((this as u32).wrapping_add(0x10));
        c.lwc1(0, p(sp(c), 4));
        c.g[A1][0] = sx((this as u32).wrapping_add(0x160));
        c.swc1(1, this as u32);
        c.swc1(0, p(this as u32, 0x18));
        call_ptr(c, 0x0036_d500, 0x0037_0b44)?;
        c.g[V0][0] = lw(c, p(sp(c), 8));
        frame.restore_ra(c, 0x10);
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdMan2d_dtor(CrowdMan2d* self, int flags)` (0x229278): empty the crowd,
/// free the render object, then delete the object when the low flag bit is
/// set.
pub fn crowd_man2d_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0022_9398, 0x0022_9294)?;
        c.g[A0][0] = lw(c, p(this as u32, 4));
        sw(c, gp(c, -0x6f0), 0);
        call_ptr(c, 0x0031_7e50, 0x0022_92a0)?;
        c.g[A0][0] = lw(c, this as u32);
        let no_render = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !no_render {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x002d_b700, 0x0022_92b4)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0022_92c8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cGameLoadScreen_Enter()` (0x233260): snapshot the texture table, start
/// the game module, clear the flag at `gp+0x1234`, push an empty vector to the
/// render manager and start the loading-screen audio loop.
pub fn c_game_load_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = lw(c, gp(c, -0x854));
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0023_3274)?;
        c.g[A0][0] = lw(c, gp(c, -0x848));
        call_ptr(c, 0x0022_81d8, 0x0023_327c)?;
        sw(c, gp(c, 0x1234), 0);
        let g = lw(c, gp(c, -0x854));
        c.g[V0][0] = g;
        c.g[V1][0] = lw(c, p(g as u32, 0x10d8));
        c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x78));
        sw(c, sp(c), 0);
        sw(c, p(sp(c), 4), 0);
        c.g[A0][0] = sx((g as u32).wrapping_add(c.g[A0][0] as u32));
        sw(c, p(sp(c), 8), 0);
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x7c));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sp(c) as u64;
        call_ptr(c, target, 0x0023_32a8)?;
        call_ptr(c, 0x0028_b180, 0x0023_32b0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_f768, 0x0023_32b8)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFELoadScreen_Enter()` (0x2338c8): the front-end twin of
/// [`c_game_load_screen_enter`].
pub fn c_fe_load_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = lw(c, gp(c, -0x854));
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0023_38dc)?;
        c.g[A0][0] = lw(c, gp(c, -0x848));
        call_ptr(c, 0x0022_8238, 0x0023_38e4)?;
        sw(c, gp(c, 0x1234), 0);
        let g = lw(c, gp(c, -0x854));
        c.g[V0][0] = g;
        c.g[V1][0] = lw(c, p(g as u32, 0x10d8));
        c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x78));
        sw(c, sp(c), 0);
        sw(c, p(sp(c), 4), 0);
        c.g[A0][0] = sx((g as u32).wrapping_add(c.g[A0][0] as u32));
        sw(c, p(sp(c), 8), 0);
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x7c));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sp(c) as u64;
        call_ptr(c, target, 0x0023_3910)?;
        call_ptr(c, 0x0028_b180, 0x0023_3918)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_f768, 0x0023_3920)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `TexMan_FreeNBTexInfo(TexMan* self, int id)` (0x3671c8): free texture
/// slot `id` and push it onto the free list, then drop the live count.
pub fn tex_man_free_nb_tex_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let id = c.g[A1][0];
        c.g[S0][0] = sx((id as u32) << 2);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((this as u32).wrapping_add(8));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A0][0] = lw(c, c.g[S1][0] as u32);
        call_ptr(c, 0x0031_7e50, 0x0036_71f4)?;
        c.g[V0][0] = lw(c, p(this as u32, 0x1f48));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        c.g[S0][0] = sx((this as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore_ra(c, 0);
        sw(c, c.g[S1][0] as u32, c.g[V0][0]);
        sw(c, p(this as u32, 0x1f48), c.g[S0][0]);
        c.g[V0][0] = lw(c, p(this as u32, 4));
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        frame.restore(c, S1, 0x20);
        sw(c, p(this as u32, 4), c.g[V0][0]);
        frame.restore(c, S2, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int TexMan_AllocFrameBuffer(TexMan* self, int size)` (0x3673d8): allocate
/// a frame buffer from the VRAM heap, tag its 0x1c-byte entry and return the
/// entry index.
pub fn tex_man_alloc_frame_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x1f60));
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = 1;
        call_ptr(c, 0x0036_6ce0, 0x0036_7400)?;
        c.g[V1][0] = 0x1c;
        c.g[A1][0] = lw(c, p(this as u32, 0x3f50));
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x7fff_ffff;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = lw(c, c.g[A1][0] as u32);
        sw(c, p(c.g[A1][0] as u32, 8), c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] | 0x22;
        c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 13) as u32);
        sw(c, c.g[A1][0] as u32, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenBCStart_OnWidgetCreated(OVScreenBCStart* self, Widget* item)`
/// (0x1f74e0): compare the widget's id with the two screen-name hashes and set
/// the widget's `+0x18` to 0 or 1 when one matches.
pub fn ov_screen_bc_start_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x004a_23e0;
        frame.save_ra(c, 0);
        let item = c.g[A1][0];
        c.g[S1][0] = item;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, p(item as u32, 0x38));
        call_ptr(c, 0x0031_7670, 0x001f_7504)?;
        let first = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        if !first {
            c.cycles += 2;
            sw(c, p(item as u32, 0x18), 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw(c, p(item as u32, 0x38));
            c.g[A0][0] = 0x004a_23e8;
            call_ptr(c, 0x0031_7670, 0x001f_7520)?;
            let second = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x20);
            if !second {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw(c, p(item as u32, 0x18), c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenProfileSavePDA_v33(OVScreenProfileSavePDA* self)` (0x211aa8):
/// leave the top profile state (or quit the game module when the flow is at
/// state 0x18), then flag the screen as done.
pub fn ov_screen_profile_save_pda_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = lw(c, p(this as u32, 0x1ac));
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 4;
            c.g[V1][0] = lw(c, p(this as u32, 0x278));
            c.g[V0][0] = 0x18;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = lw(c, p(this as u32, 0x10));
            } else {
                c.cycles += 6;
                c.g[V0][0] = lw(c, gp(c, -0x848));
                c.g[A1][0] = 0;
                c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
                call_ptr(c, 0x0023_0488, 0x0021_1ae4)?;
                c.g[V0][0] = 1;
                c.cycles += 5;
                frame.restore_ra(c, 0);
                sw(c, p(this as u32, 0x1ac), c.g[V0][0]);
                frame.restore(c, S0, 0x10);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f718, 0x0021_1af4)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        sw(c, p(this as u32, 0x1ac), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRiderBio_ctor(cFEStateRiderBio* self, int, int rider)`
/// (0x190858): construct the UI state, install the vtable, remember the rider
/// index and its pad mask.
pub fn c_fe_state_rider_bio_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x0019_0878)?;
        c.g[V0][0] = 0xf;
        c.g[V1][0] = 0x0047_0000;
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        c.g[V1][0] = 0x0046_b160;
        sw(c, p(this as u32, 8), c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        sb(c, p(this as u32, 0x44), c.g[S1][0]);
        c.g[V0][0] = lw(c, p(this as u32, 0x10));
        c.g[A0][0] = lw(c, c.g[V0][0] as u32);
        call_ptr(c, 0x001a_1cd0, 0x0019_08a0)?;
        sb(c, p(this as u32, 0x15), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup_ClearFields(cFEStateNetHelpPopup* self)` (0x1da4a8):
/// clear the popup's state words, its three counters, and its seven
/// `{a = 0; b = -1}` pairs (0x68 bytes apart).
pub fn c_fe_state_net_help_popup_clear_fields(c: &mut Cpu) {
    c.cycles += 4;
    let this = c.g[A0][0] as u32;
    sw(c, p(this, 0x48), 0);
    c.g[V1][0] = 2;
    sw(c, p(this, 0x54), 0);
    c.g[V0][0] = sx(p(this, 0x60));
    loop {
        c.cycles += 7;
        sw(c, c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_a4b8);
        }
    }
    c.cycles += 5;
    sw(c, p(this, 0x64), 0);
    c.g[V0][0] = sx(p(this, 0x68));
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(this, 0x84));
    c.g[V1][0] = 6;
    loop {
        c.cycles += 7;
        sw(c, c.g[V0][0] as u32, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        sw(c, c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_a4e8);
        }
    }
}

/// `PathMarker* PathData_FindMarker(PathData* self, int type, int id)`
/// (0x26b5e0): linear search of the 0x28-byte marker array; then the first
/// marker when there is one, else null.
pub fn path_data_find_marker(c: &mut Cpu) {
    c.cycles += 3;
    let list = c.g[A0][0];
    let ty = c.g[A1][0];
    let id = c.g[A2][0];
    c.g[V0][0] = lw(c, list as u32);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T0][0] = 0;
    let mut found: Option<u64> = None;
    if !empty {
        c.cycles += 3;
        let mut entry = lw(c, p(list as u32, 4));
        c.g[A3][0] = entry;
        let count = c.g[V0][0];
        c.g[T1][0] = count;
        loop {
            c.cycles += 4;
            c.g[V1][0] = entry;
            c.g[V0][0] = lw(c, p(entry as u32, 4));
            let next = c.g[V0][0] != ty;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
            if !next {
                c.cycles += 3;
                c.g[V0][0] = lw(c, entry as u32);
                let hit = c.g[V0][0] == id;
                if hit {
                    c.cycles += 2;
                    found = Some(entry);
                }
            }
            if found.is_some() {
                c.g[V0][0] = found.unwrap();
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            entry = sx((entry as u32).wrapping_add(0x28));
            c.g[A3][0] = entry;
            if !more {
                c.cycles += 1;
                c.g[V0][0] = lw(c, list as u32);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0026_b5f8);
            }
        }
    }
    c.cycles += 2;
    let nonempty = (c.g[V0][0] as i64) > 0;
    c.g[V0][0] = 0;
    c.cycles += 2;
    if nonempty {
        c.g[V0][0] = lw(c, p(list as u32, 4));
    }
    abi::ret(c);
}

/// `cFEStateTitle_OnWidgetAction(cFEStateTitle* self, Widget* w, int type)`
/// (0x1949e0): on a widget action of type 5, let the first UI object handle
/// the message and push the resulting state.
pub fn c_fe_state_title_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 5;
        frame.save_ra(c, 0);
        let action = c.g[A2][0] != c.g[V0][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        if !action {
            c.cycles += 11;
            c.g[V0][0] = lw(c, p(this as u32, 0x10));
            c.g[A1][0] = this;
            c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, 0x18));
            c.g[A3][0] = lw(c, c.g[V0][0] as u32);
            c.g[V1][0] = lw(c, p(c.g[A3][0] as u32, 4));
            c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x20));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_4a20)?;
            let handled = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !handled {
                c.cycles += 3;
                c.g[A0][0] = lw(c, p(this as u32, 0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x0019_4a34)?;
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
// Pause-menu template initialisers (five modes, same shape).

fn pause_freeride_init(
    c: &mut Cpu,
    slots: u64,
    table: u32,
    mode: u64,
    head: u32,
    ret: u32,
) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    let this = c.g[A0][0];
    c.g[A1][0] = this;
    frame.save_ra(c, 0);
    c.g[V0][0] = slots;
    c.g[V1][0] = 0x0048_0000;
    sw(c, p(this as u32, 0xbc), c.g[V0][0]);
    c.g[V1][0] = sx(table);
    c.g[A2][0] = 0;
    c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
    loop {
        c.cycles += 8;
        c.g[V0][0] = lw(c, c.g[V1][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        sw(c, c.g[A0][0] as u32, c.g[V0][0]);
        c.g[V0][0] = lw(c, p(this as u32, 0xbc));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 7;
            c.g[V0][0] = mode;
            c.g[A0][0] = this;
            sw(c, p(this as u32, 0x110), c.g[V0][0]);
            call_ptr(c, 0x001f_8168, ret)?;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `OVScreenPauseFreeride_InitMode1(OVScreenPauseFreeride* self)` (0x1f8ee0).
pub fn ov_screen_pause_freeride_init_mode1(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 6, 0x0047_8160, 1, 0x001f_8f08, 0x001f_8f38)
    })
}

/// `OVScreenPauseFreeride_InitMode2(OVScreenPauseFreeride* self)` (0x1f8f48).
pub fn ov_screen_pause_freeride_init_mode2(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 6, 0x0047_8178, 2, 0x001f_8f70, 0x001f_8fa0)
    })
}

/// `OVScreenPauseFreeride_InitMode3(OVScreenPauseFreeride* self)` (0x1f8fb0).
pub fn ov_screen_pause_freeride_init_mode3(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 7, 0x0047_8140, 3, 0x001f_8fd8, 0x001f_9008)
    })
}

/// `OVScreenPauseFreeride_InitMode4(OVScreenPauseFreeride* self)` (0x1f9018).
pub fn ov_screen_pause_freeride_init_mode4(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 5, 0x0047_8190, 4, 0x001f_9040, 0x001f_9070)
    })
}

/// `OVScreenPauseFreeride_InitMode5(OVScreenPauseFreeride* self)` (0x1f9080).
pub fn ov_screen_pause_freeride_init_mode5(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 5, 0x0047_81a8, 5, 0x001f_90a8, 0x001f_90d8)
    })
}

/// `char* BEScoreInt_GetHighScoreName(int idx)` (0x155208): resolve the BE
/// interface and the score table, then return entry `idx`'s name (20-byte
/// records, name at +8) or the default name at `gp-0x1eac`.
pub fn be_score_int_get_high_score_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let idx = c.g[A1][0];
        c.g[S0][0] = idx;
        call_ptr(c, 0x0014_dc80, 0x0015_521c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0015_5228)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = 0x0053_0000;
        c.g[A1][0] = lb(c, 0x0053_5c12);
        c.g[A0][0] = lw(c, 0x0053_05f0);
        call_ptr(c, 0x0014_ab20, 0x0015_523c)?;
        c.g[A1][0] = c.g[V0][0];
        let have = c.g[A1][0] != 0;
        c.g[V0][0] = 0x14;
        if have {
            c.cycles += 3;
            c.mult(0, V1, S0, V0);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, gp(c, -0x1eac));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WorldObj_ReplaceWithDeadNode(WorldObj* self)` (0x34fe28): tell the node
/// dtor (vtable slot 1) to release, allocate a 0x1c-byte dead node named
/// `D_0048E738` and construct it with the object's node index.
pub fn world_obj_replace_with_dead_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let this = c.g[A0][0];
        c.g[A2][0] = this;
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        let null = c.g[A2][0] == 0;
        c.g[S0][0] = lw(c, p(this as u32, 0x18));
        if !null {
            c.cycles += 6;
            c.g[V0][0] = lw(c, p(this as u32, 0xc));
            c.g[A1][0] = 3;
            c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 8));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0034_fe58)?;
        }
        c.cycles += 13;
        c.g[A1][0] = 0x0049_0000;
        c.g[A0][0] = 0x1c;
        c.g[A1][0] = 0x0048_e738;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0034_fe70)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0035_06d8, 0x0034_fe7c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptSoundBankMan_dtor(ScriptSoundBankMan* self, int flags)` (0x283518):
/// install the vtable, remove all banks, free the name block and delete the
/// object when the low flag bit is set.
pub fn script_sound_bank_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0x0048_2240;
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        call_ptr(c, 0x0028_3658, 0x0028_3540)?;
        c.g[A0][0] = lw(c, p(this as u32, 8));
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !none {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0028_3554)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0028_3568)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BEOptionInt_SwapPlayerSettings()` (0x14f898): swap the two 12-byte
/// player option triples at `D_005308B8` through a stack temporary.
pub fn be_option_int_swap_player_settings(c: &mut Cpu) {
    c.cycles += 25;
    c.g[V0][0] = 0x0053_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = lw(c, (c.g[A3][0] as u32).wrapping_add(8));
    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(7));
    c.sdr(4, c.g[SP][0] as u32);
    sw(c, (c.g[SP][0] as u32).wrapping_add(8), c.g[A1][0]);
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x13));
    c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x14));
    c.sdl(4, (c.g[A3][0] as u32).wrapping_add(7));
    c.sdr(4, c.g[A3][0] as u32);
    sw(c, (c.g[A3][0] as u32).wrapping_add(8), c.g[A1][0]);
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(7));
    c.ldr(2, c.g[SP][0] as u32);
    c.g[A0][0] = lw(c, (c.g[SP][0] as u32).wrapping_add(8));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x13));
    c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0xc));
    sw(c, (c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `ScriptAnimBankMan_dtor(ScriptAnimBankMan* self, int flags)` (0x275498):
/// install the vtable, remove all banks, free the name block and delete the
/// object when the low flag bit is set.
pub fn script_anim_bank_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0x0048_25d8;
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p(this as u32, 0xc), c.g[V0][0]);
        call_ptr(c, 0x0027_5698, 0x0027_54c0)?;
        c.g[A0][0] = lw(c, p(this as u32, 8));
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !none {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0027_54d4)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0027_54e8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int GameAnimMap_FindHash(GameAnimMap* self, int hash)` (0x314e88):
/// binary search of the 0x207-entry name order (`u32` indices at +0x818)
/// against the hashes at the object base; -1 when absent.
pub fn game_anim_map_find_hash(c: &mut Cpu) {
    c.cycles += 4;
    let this = c.g[A0][0] as u32;
    c.g[T1][0] = sx(this.wrapping_add(0x818));
    c.g[T0][0] = 0;
    c.g[A3][0] = 0x206;
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    loop {
        c.cycles += 11;
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 31);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = lw(c, c.g[V0][0] as u32);
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = lw(c, c.g[V1][0] as u32);
        if c.g[A1][0] == c.g[V1][0] {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
        if c.g[V0][0] == 0 {
            c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 1;
            c.g[A3][0] = c.g[A2][0];
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        if !more {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0031_4e98);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `VisualEffectsMan_EmitterHolderDtor(void* self, int flags)` (0x2d56b0):
/// free the holder's data and destroy the emitter, then delete the holder
/// when the low flag bit is set.
pub fn visual_effects_man_emitter_holder_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        frame.save(c, S1, 0x20);
        let flags = c.g[A1][0];
        c.g[S0][0] = flags;
        c.g[S1][0] = sx((this as u32).wrapping_add(0xd0));
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0037_15b0, 0x002d_56dc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0037_14f8, 0x002d_56e8)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x002d_56fc)?;
            frame.restore(c, S0, 0x30);
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
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// `D_0048DCD4`: the immutable empty `cBXString` buffer.
    const EMPTY: u32 = 0x0048_dcd4;
    /// `D_004A3E90`: the global empty-`cBXString` pointer (`gp+0xda0`).
    const EMPTY_PTR: u32 = 0x004a_3e90;

    /// Scrambled registers, fake hooks, a game-like `gp`, the BELibrary dummy
    /// and the static empty string the string code compares against.
    fn setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x1000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        c.bus.write32(EMPTY_PTR, EMPTY);
        c.bus.write32(EMPTY - 0xc, 0xffff_ffff);
        c.bus.write32(EMPTY - 8, 0);
        c.bus.write32(EMPTY, 0);
        r
    }

    /// Runs a generated function to completion as a setup step.
    fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    // ------------------------------------------------------- challenger/string

    #[test]
    fn c_challenger_dtor_matches() {
        check(
            ssx3_ee::s07::f_00268950__generated,
            c_challenger_dtor,
            90,
            &|c, seed| {
                let _r = setup(c, seed, 0x268950);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA, EMPTY);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 4, EMPTY);
                } else {
                    // A shared (refcount 2) buffer: the dtor only drops the count.
                    c.bus.write32(DATA + 4, DATA + 0x40);
                    c.bus.write32(DATA + 0x34, 2);
                    c.bus.write32(DATA + 0x38, 0);
                }
            },
        );
    }

    #[test]
    fn c_bx_string_release_matches() {
        check(
            ssx3_ee::s10::f_003180b8__generated,
            c_bx_string_release,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x3180b8);
                c.g[A0][0] = DATA as u64;
                match seed % 3 {
                    0 => c.bus.write32(DATA, EMPTY),
                    1 => {
                        c.bus.write32(DATA, DATA + 0x40);
                        c.bus.write32(DATA + 0x34, 2 + seed % 3);
                        c.bus.write32(DATA + 0x38, 0);
                    }
                    _ => {
                        // A buffer at 0xc: its header is at zero, so the release
                        // drops the count without calling the allocator.
                        c.bus.write32(DATA, 0xc);
                        c.bus.write32(0, 1);
                    }
                }
            },
        );
    }

    #[test]
    fn c_bx_string_copy_before_write_matches() {
        check(
            ssx3_ee::s10::f_00318120__generated,
            c_bx_string_copy_before_write,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x318120);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    // Shared buffer (refcount 2): release, reallocate zero bytes
                    // (the static empty string) and copy one byte.
                    c.bus.write32(DATA, DATA + 0x40);
                    c.bus.write32(DATA + 0x34, 2);
                    c.bus.write32(DATA + 0x38, 0);
                } else {
                    // Private buffer (refcount 1, header at zero): the early out,
                    // which leaves the `refcount < 2` result in v0.
                    c.bus.write32(DATA, 0xc);
                    c.bus.write32(0, 1);
                    c.bus.write32(4, 0);
                }
            },
        );
    }

    #[test]
    fn c_bx_string_op_plus_str_str_matches() {
        check(
            ssx3_ee::s10::f_003185c8__generated,
            c_bx_string_op_plus_str_str,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x3185c8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                // Two empty strings: the concatenation stays the static empty one.
                c.bus.write32(DATA + 0x100, DATA + 0x300);
                c.bus.write32(DATA + 0x2f4, 0xffff_ffff);
                c.bus.write32(DATA + 0x2f8, 0);
                c.bus.write32(DATA + 0x300, 0);
                c.bus.write32(DATA + 0x200, DATA + 0x340);
                c.bus.write32(DATA + 0x334, 0xffff_ffff);
                c.bus.write32(DATA + 0x338, 0);
                c.bus.write32(DATA + 0x340, 0);
            },
        );
    }

    // ------------------------------------------------------------ path/search

    #[test]
    fn path_get_dir_at_distance_matches() {
        check(
            ssx3_ee::s07::f_0026a8b8__generated,
            path_get_dir_at_distance,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x26a8b8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let n = 1 + seed % 3;
                c.bus.write32(DATA + 0x108, n);
                c.bus.write32(DATA + 0x118, DATA + 0x400);
                let lens = [1.0f32, 2.5, 6.0];
                for i in 0..n {
                    let e = DATA + 0x400 + i * 16;
                    c.bus.write32(e, r.next_u32());
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, lens[(i % 3) as usize].to_bits());
                }
                c.f[12] = if seed % 4 == 0 {
                    20.0f32.to_bits()
                } else {
                    r.f32(6.0)
                };
            },
        );
    }

    #[test]
    fn path_data_find_marker_matches() {
        check(
            ssx3_ee::s07::f_0026b5e0__generated,
            path_data_find_marker,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x26b5e0);
                c.g[A0][0] = DATA as u64;
                let n = seed % 4;
                c.bus.write32(DATA, n);
                c.bus.write32(DATA + 4, DATA + 0x200);
                let ty: u32 = if seed % 2 == 0 { 7 } else { 9 };
                let id = r.next_u32();
                c.g[A1][0] = ty as u64;
                c.g[A2][0] = id as u64;
                for i in 0..n {
                    let e = DATA + 0x200 + i * 0x28;
                    let hit = seed % 3 == 0 && i == n - 1;
                    c.bus.write32(e, if hit { id } else { r.next_u32() });
                    c.bus.write32(e + 4, if hit { ty } else { r.next_u32() });
                    c.bus.write32(e + 8, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_anim_map_find_hash_matches() {
        check(
            ssx3_ee::s10::f_00314e88__generated,
            game_anim_map_find_hash,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x314e88);
                c.g[A0][0] = DATA as u64;
                // A sorted table of 0x207 hashes with the identity order.
                for i in 0..0x207u32 {
                    c.bus.write32(DATA + 4 * i, 100 + i * 10);
                    c.bus.write32(DATA + 0x818 + 4 * i, i);
                }
                let hash = if seed % 2 == 0 {
                    100 + (seed % 0x207) * 10
                } else {
                    r.next_u32()
                };
                c.g[A1][0] = hash as u64;
            },
        );
    }

    // ------------------------------------------------------------- audio/triggers

    #[test]
    fn ssx_audio_system_on_replay_end_matches() {
        check(
            ssx3_ee::s07::f_0028f2c0__generated,
            ssx_audio_system_on_replay_end,
            90,
            &|c, seed| {
                let mut r = setup(c, seed, 0x28f2c0);
                c.g[A0][0] = DATA as u64;
                let mode = seed % 3;
                c.bus.write32(DATA + 0x6294, u32::from(mode == 1));
                c.bus.write32(DATA + 0x6298, u32::from(mode == 2));
                if mode == 1 {
                    // A song player with no song: the timer walks are empty.
                    c.bus.write32(DATA + 0x118, DATA + 0x500);
                    c.bus.write32(DATA + 0x500 + 0x408, 0);
                }
                if mode == 2 {
                    let vt = vtable(c, &mut r, &[0x20]);
                    c.bus.write32(DATA + 0x5558, vt);
                }
            },
        );
    }

    #[test]
    fn world_trigger_manager_mark_played_matches() {
        check(
            ssx3_ee::s08::f_002b6868__generated,
            world_trigger_manager_mark_played,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2b6868);
                c.g[A0][0] = DATA as u64;
                let id = if seed % 3 == 0 {
                    0x1111_2222
                } else {
                    r.next_u32()
                };
                c.g[A1][0] = id as u64;
                let count = seed % 4;
                c.bus.write32(DATA + 0x324, count);
                c.bus
                    .write32(DATA + 0x32c, if seed % 2 == 0 { id } else { 0x3333_4444 });
                c.bus.write32(DATA + 0x330, 0);
                for i in 0..count {
                    c.bus.write32(DATA + 4 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_world_sound_insert_emitters_matches() {
        check(
            ssx3_ee::s08::f_002b6c90__generated,
            c_world_sound_insert_emitters,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2b6c90);
                c.g[A0][0] = DATA as u64;
                // The entries pointer arrives in a2 and is moved into a1.
                c.g[A2][0] = (DATA + 0x200) as u64;
                let n = seed % 4;
                c.g[A3][0] = n as u64;
                for i in 0..n {
                    let e = DATA + 0x200 + i * 0x18;
                    // Only the active flag matters; `+0x10` stays null so the
                    // insert takes its early-out.
                    c.bus
                        .write32(e + 0xc, u32::from(seed % 2 == 0 || i % 2 == 0));
                    c.bus.write32(e + 0x10, 0);
                    c.bus.write32(e + 4, r.next_u32());
                }
            },
        );
    }

    

    // ------------------------------------------------------------- FX / emitters

    #[test]
    fn c_bolt_on_fx_interface_find_or_create_emitter_matches() {
        check(
            ssx3_ee::s09::f_002ecbc0__generated,
            c_bolt_on_fx_interface_find_or_create_emitter,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2ecbc0);
                c.g[A0][0] = DATA as u64;
                let a1 = r.next_u32();
                let a2 = r.next_u32();
                c.g[A1][0] = (if seed % 2 == 0 { a1 } else { r.next_u32() }) as u64;
                c.g[A2][0] = a2 as u64;
                // Slot 0 always matches (id = a2, kind = a1); the others do not,
                // so both the found and the create paths are visited.
                c.bus
                    .write32(DATA + 0x10, if seed % 3 == 0 { 0xffff_ffff } else { 5 });
                c.bus.write32(DATA + 0x14, a2);
                c.bus.write32(DATA + 0x18, a1);
                for i in 1..15u32 {
                    c.bus.write32(DATA + 0x18 + 0xc * i - 8, 0xffff_ffff);
                }
            },
        );
    }

    #[test]
    fn emitter_load_matches() {
        check(ssx3_ee::s12::f_00370af8__generated, emitter_load, 80, &|c, seed| {
            let mut r = setup(c, seed, 0x370af8);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            // stream->v02: vtable slot 0x10/0x14 (a fake method that just logs).
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(DATA + 0x800, vt);
        });
    }

    #[test]
    fn c_world_cache_init_hull_slots_matches() {
        check(
            ssx3_ee::s13::f_003a82c8__generated,
            c_world_cache_init_hull_slots,
            100,
            &|c, seed| {
                let _r = setup(c, seed, 0x3a82c8);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                let n = seed % 3;
                c.bus.write32(DATA + 0x1000 + 0x14, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0x10, n);
            },
        );
    }

    #[test]
    fn visual_effects_man_emitter_holder_dtor_matches() {
        check(
            ssx3_ee::s09::f_002d56b0__generated,
            visual_effects_man_emitter_holder_dtor,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2d56b0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                // The emitter holder's data pointer: null keeps the free path out.
                c.bus.write32(DATA + 0xd0, 0);
                let _ = r.next_u32();
            },
        );
    }

    // --------------------------------------------------------------- crowd/pads

    #[test]
    fn crowd_man3d_group_reset_matches() {
        check(
            ssx3_ee::s11::f_003442e0__generated,
            crowd_man3d_group_reset,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x3442e0);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x200u32 / 4 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn crowd_man2d_dtor_matches() {
        check(
            ssx3_ee::s05::f_00229278__generated,
            crowd_man2d_dtor,
            60,
            &|c, seed| {
                let mut r = setup(c, seed, 0x229278);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x400 + 4, 0);
                c.bus.write32(DATA + 0x400, 0);
                let _ = r.next_u32();
            },
        );
    }

    // ---------------------------------------------------------- FE state screens

    #[test]
    fn c_fe_state_options_save_load_init_matches() {
        check(
            ssx3_ee::s02::f_0018d280__generated,
            c_fe_state_options_save_load_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x18d280);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_trophy_room_init_matches() {
        check(
            ssx3_ee::s04::f_001d4368__generated,
            c_fe_state_trophy_room_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d4368);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_relogin_init_matches() {
        check(
            ssx3_ee::s04::f_001d5078__generated,
            c_fe_state_relogin_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d5078);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_load_state_in_lodge_init_matches() {
        check(
            ssx3_ee::s06::f_00245d50__generated,
            c_fe_load_state_in_lodge_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x245d50);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn fe_screen_select_profile_init_matches() {
        check(
            ssx3_ee::s02::f_0018ead8__generated,
            fe_screen_select_profile_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x18ead8);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_peak_room_init_matches() {
        check(
            ssx3_ee::s04::f_001d3890__generated,
            c_fe_state_peak_room_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d3890);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_init_matches() {
        check(
            ssx3_ee::s04::f_001d9db8__generated,
            c_fe_state_net_help_popup_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d9db8);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_cheat_char_select_init_matches() {
        check(
            ssx3_ee::s02::f_00182420__generated,
            c_fe_state_cheat_char_select_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x182420);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn ov_screen_trophy_init_matches() {
        check(
            ssx3_ee::s05::f_0020e938__generated,
            ov_screen_trophy_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x20e938);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_clear_fields_matches() {
        check(
            ssx3_ee::s04::f_001da4a8__generated,
            c_fe_state_net_help_popup_clear_fields,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1da4a8);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x60u32 / 4 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_fe_state_rider_bio_ctor_matches() {
        check(
            ssx3_ee::s02::f_00190858__generated,
            c_fe_state_rider_bio_ctor,
            60,
            &|c, seed| {
                let _r = setup(c, seed, 0x190858);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = (seed % 4) as u64;
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn script_engine_seq_listener_on_stop_matches() {
        check(
            ssx3_ee::s07::f_00282a18__generated,
            script_engine_seq_listener_on_stop,
            90,
            &|c, seed| {
                let mut r = setup(c, seed, 0x282a18);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let vt = vtable(c, &mut r, &[0x58]);
                c.bus.write32(DATA + 0x2a8, vt);
                c.bus.write32(DATA + 0x10c, DATA + 0x200);
                c.bus.write32(DATA + 0x208, r.next_u32());
                c.bus.write32(DATA + 0x20c, r.next_u32());
                c.bus
                    .write16(DATA + 0x210, if seed % 2 == 0 { 0 } else { 3 });
                c.bus.write32(DATA + 0x21c, r.next_u32());
            },
        );
    }

    // ---------------------------------------------------------- misc FE widgets

    #[test]
    fn ov_screen_bc_start_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001f74e0__generated,
            ov_screen_bc_start_on_widget_created,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1f74e0);
                let h0 = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_23e0]);
                let h1 = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_23e8]);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let id = match seed % 3 {
                    0 => h0,
                    1 => h1,
                    _ => r.next_u32() as u64,
                };
                c.bus.write32(DATA + 0x138, id as u32);
                c.bus.write32(DATA + 0x118, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_v33_matches() {
        check(
            ssx3_ee::s05::f_00211aa8__generated,
            ov_screen_profile_save_pda_v33,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x211aa8);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x1ac, u32::from(seed % 2 == 0));
                c.bus
                    .write32(DATA + 0x278, if seed % 4 == 0 { 0x18 } else { 0 });
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
            },
        );
    }

    #[test]
    fn c_fe_state_title_on_widget_action_matches() {
        check(
            ssx3_ee::s02::f_001949e0__generated,
            c_fe_state_title_on_widget_action,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1949e0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    5
                } else {
                    1 + seed as u64 % 7
                };
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x118, r.next_u32());
            },
        );
    }

    #[test]
    fn be_new_player_int_get_difficulty_matches() {
        check(
            ssx3_ee::s01::f_00147cb8__generated,
            be_new_player_int_get_difficulty,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x147cb8);
                c.g[A0][0] = 0;
                c.g[A1][0] = u64::from(seed % 2 != 0);
                // A rider character row with both difficulty shorts.
                c.bus.write16(0x004a_6ca8 + 0x280, 1 + (seed % 5) as u16);
                c.bus.write16(0x004a_6ca8 + 0x284, 1 + (seed % 3) as u16);
            },
        );
    }

    // -------------------------------------------------------- pause-menu modes

    /// The pause screen at `DATA` with a dummy UI engine and a null race
    /// manager. `cOVStateOverlay_Init` looks the engine up at `self+0x10` and
    /// hands `UIStateMgr_FindState(engine+0x18)` a *properly empty* `UIList`
    /// (sentinel at `+4`, `+8`), so the state walk returns at once instead of
    /// chasing the screen objects the null engine builds at address 0.
    fn pause_screen(c: &mut Cpu, seed: u32) {
        let engine = DATA + 0x8000;
        let world = DATA + 0x9000;
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0x10, engine);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, 0);
        for off in [0x18u32, 0x34] {
            let list = engine + off;
            c.bus.write32(list + 4, list);
            c.bus.write32(list + 8, list);
        }
        c.bus.write32(engine + 0x10, 0);
        c.bus.write32(DATA + 0xbc, seed % 3);
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode1_matches() {
        check(
            ssx3_ee::s04::f_001f8ee0__generated,
            ov_screen_pause_freeride_init_mode1, 8 ,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f8ee0);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode2_matches() {
        check(
            ssx3_ee::s04::f_001f8f48__generated,
            ov_screen_pause_freeride_init_mode2, 8 ,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f8f48);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode3_matches() {
        check(
            ssx3_ee::s04::f_001f8fb0__generated,
            ov_screen_pause_freeride_init_mode3, 8 ,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f8fb0);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode4_matches() {
        check(
            ssx3_ee::s04::f_001f9018__generated,
            ov_screen_pause_freeride_init_mode4, 8 ,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f9018);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode5_matches() {
        check(
            ssx3_ee::s04::f_001f9080__generated,
            ov_screen_pause_freeride_init_mode5, 8 ,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f9080);
                pause_screen(c, seed);
            },
        );
    }

    // ------------------------------------------------------ transform/boost/sphere

    #[test]
    fn coll_sphere_tree_transform_matches() {
        check(
            ssx3_ee::s11::f_00330710__generated,
            coll_sphere_tree_transform,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x330710);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x60, DATA + 0x300);
                c.bus.write32(DATA + 0x68, DATA + 0x400);
                for i in 0..0x200u32 / 4 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                    c.bus.write32(DATA + 0x400 + 4 * i, r.next_u32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn boost_modifier_v06_matches() {
        check(
            ssx3_ee::s11::f_00342ba0__generated,
            boost_modifier_v06,
            80,
            &|c, seed| {
                let mut r = setup(c, seed, 0x342ba0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                let vt = vtable(c, &mut r, &[8, 0x28]);
                c.bus.write32(DATA + 0x800, vt);
                c.bus.write32(DATA + 0x40, r.next_u32());
            },
        );
    }

    #[test]
    fn be_bolt_int_save_equip_flags_matches() {
        check(
            ssx3_ee::s01::f_0014aea8__generated,
            be_bolt_int_save_equip_flags,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x14aea8);
                c.g[A0][0] = 0;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                let n = seed % 5;
                c.bus.write32(0x004a_6ca8 + 0x28c, n);
                for i in 0..n {
                    let e = 0x004a_6ca8 + 0x290 + 4 * i;
                    c.bus.write16(e, r.next_u32() as u16);
                    c.bus.write16(
                        e + 2,
                        if seed % 2 == 0 {
                            (r.next_u32() | 0x10) as u16
                        } else {
                            (r.next_u32() & !0x10) as u16
                        },
                    );
                }
            },
        );
    }

    // -------------------------------------------------------------- allocators

    #[test]
    fn tex_man_free_nb_tex_info_matches() {
        check(
            ssx3_ee::s12::f_003671c8__generated,
            tex_man_free_nb_tex_info,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x3671c8);
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                // Slot `id` is null (a plain delete of null) and is pushed onto the
                // free list headed at +0x1f48.
                c.bus.write32(DATA + 0x808 + 4 * (seed % 4), 0);
                c.bus
                    .write32(DATA + 0x800 + 0x1f48, DATA + 0x500 + 8 * (seed % 4));
                c.bus.write32(DATA + 0x800 + 4, r.next_u32());
            },
        );
    }

    /// The TexMan at `DATA+0x10000` embeds its VRAM heap at `+0x1f60`; the
    /// node array pointer is the heap's `+0x1ff0` field (TexMan `+0x3f50`).
    const TEXMAN: u32 = DATA + 0x1_0000;
    const VHEAP: u32 = TEXMAN + 0x1f60;
    const VNODES: u32 = DATA + 0x2_0000;
    const VOWNER: u32 = DATA + 0x2_4000;

    /// A VRAM heap built by the game's own `VramHeap_Init`.
    fn vram_heap(c: &mut Cpu) {
        c.bus.write32(VHEAP + 0x1ff0, VNODES);
        for i in 0..16u32 {
            c.bus.write32(VOWNER + 4 * i, VOWNER + 0x100 + 0x40 * i);
        }
        run(
            c,
            0x0036_5e40,
            ssx3_ee::s12::VramHeap_Init,
            &[VHEAP as u64, 0, 0x10_0000, VOWNER as u64],
        );
    }

    #[test]
    fn tex_man_alloc_frame_buffer_matches() {
        check(
            ssx3_ee::s12::f_003673d8__generated,
            tex_man_alloc_frame_buffer,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x3673d8);
                vram_heap(c);
                c.g[A0][0] = TEXMAN as u64;
                c.g[A1][0] = (0x100 * (1 + seed % 8)) as u64;
            },
        );
    }

    #[test]
    fn world_obj_replace_with_dead_node_matches() {
        check(
            ssx3_ee::s11::f_0034fe28__generated,
            world_obj_replace_with_dead_node,
            80,
            &|c, seed| {
                let mut r = setup(c, seed, 0x34fe28);
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.bus.write32(DATA + 0x800 + 0x18, r.next_u32());
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x800 + 0xc, 0);
                } else {
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(DATA + 0x800 + 0xc, vt);
                }
            },
        );
    }

    // ------------------------------------------------------------- option swap

    #[test]
    fn be_option_int_swap_player_settings_matches() {
        check(
            ssx3_ee::s01::f_0014f898__generated,
            be_option_int_swap_player_settings,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x14f898);
                for i in 0..8u32 {
                    c.bus.write32(0x0053_08b8 + 4 * i, r.next_u32());
                }
                let _ = seed;
            },
        );
    }

    // ------------------------------------------------------------- score names

    #[test]
    fn be_score_int_get_high_score_name_matches() {
        check(
            ssx3_ee::s01::f_00155208__generated,
            be_score_int_get_high_score_name,
            60,
            &|c, seed| {
                let _r = setup(c, seed, 0x155208);
                c.g[A0][0] = 0;
                c.g[A1][0] = (seed % 8) as u64;
                c.bus.write8(0x0053_5c12, (seed % 3) as u8);
                c.bus.write32(0x0053_05f0, DATA + 0x600);
                c.bus.write32(0x004a_1244, DATA + 0x700);
            },
        );
    }

    // ------------------------------------------------------- bank-manager dtors

    #[test]
    fn script_sound_bank_man_dtor_matches() {
        check(
            ssx3_ee::s07::f_00283518__generated,
            script_sound_bank_man_dtor,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x283518);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x408, 0);
            },
        );
    }

    #[test]
    fn script_anim_bank_man_dtor_matches() {
        check(
            ssx3_ee::s07::f_00275498__generated,
            script_anim_bank_man_dtor,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x275498);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x408, 0);
            },
        );
    }

    // ------------------------------------------------------------ load screens

    #[test]
    fn c_game_load_screen_enter_matches() {
        check(
            ssx3_ee::s06::f_00233260__generated,
            c_game_load_screen_enter,
            30,
            &|c, seed| {
                let mut r = setup(c, seed, 0x233260);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), DATA + 0x200);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x300);
                let rm = DATA + 0x400;
                c.bus.write32(DATA + 0x200 + 0x10d8, rm);
                c.bus.write32(rm + 0x78, 0);
                c.bus.write32(rm + 0x7c, 0);
                c.bus.write32(DATA + 0x300 + 0x84, 0);
                c.bus.write32(DATA + 0x300 + 0x7c, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn c_fe_load_screen_enter_matches() {
        check(
            ssx3_ee::s06::f_002338c8__generated,
            c_fe_load_screen_enter,
            30,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2338c8);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), DATA + 0x200);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x300);
                let rm = DATA + 0x400;
                c.bus.write32(DATA + 0x200 + 0x10d8, rm);
                c.bus.write32(rm + 0x78, 0);
                c.bus.write32(rm + 0x7c, 0);
                c.bus.write32(DATA + 0x300 + 0x7c, 0);
                let _ = r.next_u32();
            },
        );
    }
}
