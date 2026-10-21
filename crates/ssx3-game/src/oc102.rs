//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 27 cold-path functions — destructors, constructors,
//! small list/UI/FE helpers and two data-only scans — each with a
//! differential test against the generated body.
//!
//! Most of these are thin wrappers over already-ported callees; the tests set
//! up the objects (and, where a vtable or the BE interface is involved, the
//! fakes from [`crate::difftest::fakes`]) so the callees take their short
//! paths.

use crate::abi;
use crate::call::{call_ptr, guest};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_6008, be_new_player_int_get_rider_state_flag, "BENewPlayerInt_GetRiderStateFlag"),
    (0x0015_3390, be_mission_int_get_peak_collectible_total, "BEMissionInt_GetPeakCollectibleTotal"),
    (0x0015_57e0, be_score_int_can_submit_score, "BEScoreInt_CanSubmitScore"),
    (0x0015_7b08, be_reward_int_is_rival_reward_pending, "BERewardInt_IsRivalRewardPending"),
    (0x0016_92d8, script_cam_controller_stop_script_camera, "ScriptCamController_StopScriptCamera"),
    (0x0018_2808, cfe_state_cheat_char_select_on_widget_notify, "cFEStateCheatCharSelect_OnWidgetNotify"),
    (0x0018_28c0, cfe_state_char_setup_ctor, "cFEStateCharSetup_ctor"),
    (0x0018_31d0, cfe_state_char_setup_set_rider_preview_active, "cFEStateCharSetup_SetRiderPreviewActive"),
    (0x0018_64b0, cfe_options_credits_on_widget_action, "cFEOptionsCredits_OnWidgetAction"),
    (0x0018_65a8, cfe_state_on_deactivate, "cFEState_OnDeactivate"),
    (0x001a_07c8, feuicustom_v03, "FEUICustom_v03"),
    (0x001a_2128, femodule_try_exit, "FEModule_TryExit"),
    (0x0020_eaa0, hud_meter_ctor, "HUDMeter_ctor"),
    (0x0023_4120, game_state_finish_load_state, "GameStateFinish_LoadState"),
    (0x0024_ff28, flags_menu_dtor, "FlagsMenu_dtor"),
    (0x0025_2248, mem_is_valid_block_link, "MEM_IsValidBlockLink"),
    (0x0026_ca90, playback_man_reset, "PlaybackMan_Reset"),
    (0x0031_8d28, cbx_string_make_upper, "cBXString_MakeUpper"),
    (0x0032_0ca0, input_map_init, "InputMap_Init"),
    (0x0034_5828, particle_modifier_base_dtor, "ParticleModifier_BaseDtor"),
    (0x0034_61c0, halo_modifier_ctor_2, "HaloModifier_ctor_2"),
    (0x0034_cb80, fence_dtor, "Fence_dtor"),
    (0x0035_a3f0, multi_spline_modifier_alloc_node_instances, "MultiSplineModifier_AllocNodeInstances"),
    (0x0039_7b08, ui_engine_ctor, "UIEngine_ctor"),
    (0x0039_be70, ui_pair_free_children, "UIPair_FreeChildren"),
    (0x003a_9558, cworld_cache_streamer_release_node_hulls, "cWorldCache_StreamerReleaseNodeHulls"),
];

/// `a + off`.
#[inline(always)]
fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `addiu sp, sp, -n`.
#[inline(always)]
fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}

/// `addiu sp, sp, n; jr ra` (with `ra` already reloaded by the caller).
#[inline(always)]
fn close(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(n));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Destructors.

/// `void FlagsMenu_dtor(FlagsMenu* self, int flags)` (0x24ff28): destroy the
/// four sub-menus (each as a `MenuSysItem`) and then the page base (with
/// `flags`). `flags & 1` asks the page base to free `self`.
pub fn flags_menu_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[S1][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        // The four sub-menus, each destroyed as its base `MenuSysItem`.
        for (off, ret) in [
            (0x194u32, 0x0024_ff4cu32),
            (0x164, 0x0024_ff58),
            (0x14c, 0x0024_ff64),
            (0x130, 0x0024_ff70),
        ] {
            c.g[A0][0] = sx(at(this, off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_ff7c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void ParticleModifier_BaseDtor(ParticleModifier* self, int flags)`
/// (0x345828): retag the vtable twice (this class, then its base), unlink the
/// object from the modifier list and, for `flags & 1`, delete it.
pub fn particle_modifier_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        open(c, 0x30);
        c.g[V0][0] = 0x490000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12f8));
        c.g[V1][0] = c.lw(gp(c, 0xf04));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = 0x490000;
        c.sw(gp(c, 0xf04), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1340));
        c.g[RA][0] = sx(0x0034_5868);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0034_5760, 0x0034_5868)?;
        c.g[S0][0] = c.g[S0][0] & 0x1;
        let free = c.g[S0][0] == 0;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if !free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0034_587c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void Fence_dtor(Fence* self, int flags)` (0x34cb80): install the deleting
/// vtable, release the handler object through the world's interface (slot
/// 77), then run the `WorldObj` destructor.
pub fn fence_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[V0][0] = 0x490000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x600));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        let none = c.g[V1][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 7;
            c.g[A2][0] = c.lw(gp(c, 0x2a90));
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x268));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x26c));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0034_cbc8);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0034_cbc8)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_fbf0, 0x0034_cbd4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Constructors.

/// `UIEngine* UIEngine_ctor(UIEngine* self)` (0x397b08): clear the engine
/// header, construct both UI lists and clear the tail pointers.
pub fn ui_engine_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32), 0u64);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.g[RA][0] = sx(0x0039_7b38);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        call_ptr(c, 0x0039_77e8, 0x0039_7b38)?;
        c.g[RA][0] = sx(0x0039_7b40);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
        call_ptr(c, 0x0039_77e8, 0x0039_7b40)?;
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `HaloModifier* HaloModifier_ctor_2(HaloModifier* self, sMbStream* stream)`
/// (0x3461c0): base particle-modifier constructor, install this class's
/// vtable, then let the stream's slot 2 (`+0x10`) initialise `self + 0x10`.
pub fn halo_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0034_61dc);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0034_57c8, 0x0034_61dc)?;
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1220));
        c.g[A2][0] = 0x30;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let target = c.g[A3][0] as u32;
        c.g[RA][0] = sx(0x0034_6204);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0034_6204)?;
        c.g[V1][0] = c.lw(gp(c, 0xf10));
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw(gp(c, 0xf10), c.g[V1][0]);
        close(c, 0x30);
        Ok(())
    })
}

/// `cFEStateCharSetup* cFEStateCharSetup_ctor(self, engine, signed char idx)`
/// (0x1828c0): `UIState` constructor, this class's vtable, the player index
/// byte and the pad mask for that player.
pub fn cfe_state_char_setup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0018_28e0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x0018_28e0)?;
        c.g[V0][0] = 0xc;
        c.g[V1][0] = 0x470000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x31a0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sb((c.g[S0][0] as u32).wrapping_add(0x44), c.g[S1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x0018_2908);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        call_ptr(c, 0x001a_1cd0, 0x0018_2908)?;
        c.sb((c.g[S0][0] as u32).wrapping_add(0x15), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void HUDMeter_ctor(HUDMeter* self)` (0x20eaa0): install the vtable, set a
/// full-screen 640x480 rectangle and hand it to `HUDMeter_SetRect` as both
/// the source and the destination.
pub fn hud_meter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 0);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x470000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4230));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32), 0u64);
        c.g[RA][0] = sx(0x0020_eae8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        call_ptr(c, 0x0020_ec18, 0x0020_eae8)?;
        c.g[V1][0] = c.lw(gp(c, -0x984));
        c.g[V0][0] = c.g[S0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw(gp(c, -0x984), c.g[V1][0]);
        close(c, 0x30);
        Ok(())
    })
}

/// `void InputMap_Init(InputMap* self, int count)` (0x320ca0): size the map
/// array, allocate `count * 4` bytes for it and fill the buffer with -1.
pub fn input_map_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x490000;
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2300));
        c.g[RA][0] = sx(0x0032_0cd0);
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0032_0cd0)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[RA][0] = sx(0x0032_0cec);
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        call_ptr(c, 0x003e_6448, 0x0032_0cec)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `void InputMap_CreateParser(InputMap* self, name, arg)` (0x320db0):
/// allocate a 0x109ec-byte parser from the game heap and construct it.
pub fn input_map_create_parser(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x40);
        c.g[A3][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x10000;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x22f0));
        c.g[A0][0] = c.g[A0][0] | 0x9ec;
        c.g[RA][0] = sx(0x0032_0dec);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_7d70, 0x0032_0dec)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0032_0dfc);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0032_1428, 0x0032_0dfc)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x40);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small list / object helpers.

/// `void MEM_IsValidBlockLink(void* p)` (0x252248): true if `p` is a valid
/// allocator pointer whose block magic is `"BF"` or `"BS"`; otherwise record
/// the failure in `D_004A2E74` and return false.
pub fn mem_is_valid_block_link(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[A1][0] = 0x18;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0025_226c);
        c.g[S1][0] = 0u64;
        call_ptr(c, 0x003e_6240, 0x0025_226c)?;
        let mut ok = 0u64;
        if c.g[V0][0] == 0 {
            // Not a pointer EA considers valid: flag the error.
            c.cycles += 1;
            c.cycles += 1;
            c.sw(gp(c, -0x27c), 1);
        } else {
            c.cycles += 3;
            let magic = Addr(c.g[S0][0] as u32).lhu(c);
            c.g[A0][0] = magic;
            c.g[V0][0] = 0x4253;
            if magic == 0x4246 {
                c.cycles += 2;
                ok = 1;
            } else {
                c.cycles += 2;
                if magic == 0x4253 {
                    c.cycles += 2;
                    ok = 1;
                } else {
                    c.cycles += 1;
                    c.sw(gp(c, -0x27c), 1);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = ok;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void UIPair_FreeChildren(UIPair* self)` (0x39be70): destroy the two child
/// objects (vtable slot 1 with argument 3) and clear the pointers.
pub fn ui_pair_free_children(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        let a_none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !a_none {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_bea0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0039_bea0)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x78), 0u64);
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        let b_none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !b_none {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039_bec4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0039_bec4)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `int FEUICustom_v03(FEUIObject* self, int unused, int hash)` (0x1a07c8):
/// true (and the screen object's slot 24 invoked with `(1, 0)`) when `hash`
/// is the hash of the credits object name `D_0045FFF8`.
pub fn feuicustom_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = 0x460000;
        c.g[S0][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x001a_07ec);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x8));
        call_ptr(c, 0x0031_7670, 0x001a_07ec)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            let no_obj = c.g[A3][0] == 0;
            c.g[A1][0] = 0x1;
            if !no_obj {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001a_0818);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x001a_0818)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        } else {
            c.g[V0][0] = 0u64;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `bool FEModule_TryExit(FEModule* self)` (0x1a2128): exit the current flow
/// state, stop the module's UI screens and free the module's flow state.
pub fn femodule_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x001a_213c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0023_1cf0, 0x001a_213c)?;
        let can_exit = c.g[V0][0] != 0;
        c.g[V0][0] = 0xb0000;
        if can_exit {
            c.cycles += 4;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5ad4));
            if c.g[A0][0] == 0 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad8), 0u64);
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a_2164);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x0025_3418, 0x001a_2164)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad4), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5ad8), 0u64);
            }
            c.cycles += 5;
            c.g[A0][0] = sx(gp(c, 0x1d90));
            c.sw(gp(c, -0x1724), 0u64);
            c.g[RA][0] = sx(0x001a_217c);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5adc), 0u64);
            call_ptr(c, 0x001a_06b0, 0x001a_217c)?;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `void MultiSplineModifier_AllocNodeInstances(MultiSplineModifier* self)`
/// (0x35a3f0): fill `self->instances[0 .. count)` with fresh instance-man
/// allocations.
pub fn multi_spline_modifier_alloc_node_instances(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0u64;
        if !empty {
            loop {
                c.cycles += 13;
                c.g[RA][0] = sx(0x0035_a418);
                call_ptr(c, 0x002d_1ce0, 0x0035_a418)?;
                c.g[RA][0] = sx(0x0035_a420);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0035_1170, 0x0035_a420)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_a410);
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Data scans.

/// `int BEMissionInt_GetPeakCollectibleTotal(iface, int owner)` (0x153390):
/// for each of the 22 mission records whose `owner` field (at `+0x54`) is
/// `owner`, add the matching byte of the per-mission table.
pub fn be_mission_int_get_peak_collectible_total(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = 0x440000;
    // 0x43d950: 22 mission records (0x64 bytes each), owner at +0x54.
    c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b0));
    // 0x43fa70: per-record contribution bytes.
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x590));
    c.g[T0][0] = 0u64;
    c.g[V0][0] = 0u64;
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    loop {
        c.cycles += 4;
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x54));
        c.g[A3][0] = 0x15;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
            if c.g[V0][0] != c.g[A1][0] {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[A2][0] as u32));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 3;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            let more = (c.g[A3][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_33c0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[T1][0];
        let done = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        if !done {
            c.cycles += 2;
            c.g[V0][0] = c.g[T0][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_33b0);
        }
    }
}

// ---------------------------------------------------------------------------
// BE reward / score wrappers (rider index -> profile + character).

/// `int BERewardInt_IsRivalRewardPending(BERewardInt* self, int rider,
/// int peak)` (0x157b08): resolve the rider's profile and character, then
/// test the peak's rival-reward bit.
pub fn be_reward_int_is_rival_reward_pending(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x50);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0015_7b34);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0015_7b34)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0015_7b40);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a080, 0x0015_7b40)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0015_7b54);
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0015_7b70, 0x0015_7b54)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x50);
        Ok(())
    })
}

/// `int BENewPlayerInt_GetRiderStateFlag(BENewPlayerInt* self, int rider,
/// int flag)` (0x146008): resolve profile and character, then read the
/// character-state bit.
pub fn be_new_player_int_get_rider_state_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x50);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0014_6034);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a0e0, 0x0014_6034)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014_6040);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_a080, 0x0014_6040)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0014_6054);
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_5f90, 0x0014_6054)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x50);
        Ok(())
    })
}

/// `bool BEScoreInt_CanSubmitScore(BEScoreInt* self, int riders)` (0x1557e0):
/// no handicap is set, the BE interface is up, and fewer than two profiles
/// are in career mode.
pub fn be_score_int_can_submit_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        open(c, 0x10);
        c.g[A1][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
        let few = c.g[A1][0] == 0;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if !few {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0015_57f8);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014_f810, 0x0015_57f8)?;
            let no_handicap = c.g[V0][0] == 0;
            if !no_handicap {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                c.cycles += 3;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                close(c, 0x10);
                return Ok(());
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.cycles += 3;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            close(c, 0x10);
            return Ok(());
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x0015_5810);
        call_ptr(c, 0x0014_dc80, 0x0015_5810)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0015_581c);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014_dd58, 0x0015_581c)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5bfc));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
        let career = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1;
        if !career {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0015_5838);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014_f810, 0x0015_5838)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x10);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state helpers.

/// `int cFEStateCheatCharSelect_OnWidgetNotify(self, item, int msg)`
/// (0x182808): route widget notifications to `ApplyOutfit`; return 0x100 for
/// the range messages 7..9, 0x101 otherwise, and for message 1 whether the
/// value is below the state's limit.
pub fn cfe_state_cheat_char_select_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        open(c, 0x10);
        c.g[V0][0] = 0x5;
        let msg = c.g[A2][0];
        let is5 = msg == 5;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if !is5 {
            c.cycles += 3;
            c.g[V0][0] = (msg < 6) as u64;
            if msg < 6 {
                c.cycles += 2;
                if msg == 1 {
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
                    c.cycles += 2;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x101;
                }
            } else {
                // The second compare block's own +3 (the one above is the
                // first compare's), charged on every msg >= 6 path.
                c.cycles += 3;
                c.g[V0][0] = (msg < 0xa) as u64;
                if msg < 0xa {
                    c.cycles += 2;
                    c.g[V0][0] = 0x101;
                    if msg >= 7 {
                        c.cycles += 2;
                        c.g[V0][0] = 0x100;
                    }
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0x101;
                }
            }
        } else {
            c.cycles += 2;
            call_ptr(c, 0x0018_2870, 0x0018_2860)?;
            c.cycles += 1;
            c.g[V0][0] = 0x101;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x10);
        Ok(())
    })
}

/// `void cFEOptionsCredits_OnWidgetAction(self, item, int msg)` (0x1864b0):
/// for message 6, ask the engine object's slot 5 for the credits screen
/// (hash name 0x24) and push it if found.
pub fn cfe_options_credits_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let no_item = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if no_item {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x6;
            if c.g[A2][0] != c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x24;
                c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0018_64f4);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x0018_64f4)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if none {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0018_6508);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0018_6508)?;
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                }
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `void cFEState_OnDeactivate(cFEState* self)` (0x1865a8): find the current
/// screen's credits object and tell the FE module it was left.
pub fn cfe_state_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0018_65c4);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
        call_ptr(c, 0x0039_7840, 0x0018_65c4)?;
        c.g[S0][0] = c.g[V0][0];
        let no_screen = c.g[S0][0] == 0;
        c.g[A0][0] = 0x4a0000;
        if no_screen {
            // The empty-screen path runs the shared tail (its +1 and lq16).
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        } else {
            c.cycles += 7;
            c.g[RA][0] = sx(0x0018_65d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
            call_ptr(c, 0x0031_7670, 0x0018_65d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0018_65e4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_65e4)?;
            let not_found = c.g[V0][0] == 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if !not_found {
                c.cycles += 4;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = sx(gp(c, 0x1d90));
                c.g[RA][0] = sx(0x0018_65fc);
                c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x95));
                call_ptr(c, 0x001a_0708, 0x0018_65fc)?;
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void cFEStateCharSetup_SetRiderPreviewActive(self, int on)` (0x1831d0):
/// look the rider up in the game's rider manager and toggle the rider
/// preview.
pub fn cfe_state_char_setup_set_rider_preview_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x30);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        let no_mgr = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !no_mgr {
            c.cycles += 5;
            c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[RA][0] = sx(0x0018_3200);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a_0548, 0x0018_3200)?;
            if c.g[S0][0] != 0 {
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0018_3214);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0019_e538, 0x0018_3214)?;
                let still = c.g[S0][0] != 0;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                if !still {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// String, world-cache, replay and camera helpers.

/// `void cBXString_MakeUpper(cBXString* self)` (0x318d28): copy the buffer if
/// shared, then uppercase every character (`tolower`-table bit 1).
pub fn cbx_string_make_upper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0031_8d3c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0031_8120, 0x0031_8d3c)?;
        c.g[S0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x8));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
        let empty = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = 0x4a0000;
        if !empty {
            c.cycles += 2;
            // 0x499c81: the character-class table (bit 1 = lowercase).
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x637f));
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.lb((c.g[S0][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20));
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x2;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[A0][0];
                }
                c.sb((c.g[S0][0] as u32), c.g[V1][0]);
                let more = (c.g[A2][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_8d58);
                }
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}

/// `void cWorldCache_StreamerReleaseNodeHulls(cWorldCache* self, Node* node)`
/// (0x3a9558): recursively expire the node's section hull and both children.
pub fn cworld_cache_streamer_release_node_hulls(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
        let no_section = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        if !no_section {
            c.cycles += 3;
            c.g[A1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
            c.g[RA][0] = sx(0x003a_9584);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
            call_ptr(c, 0x003a_84c8, 0x003a_9584)?;
        }
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_9598);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_9558, 0x003a_9598)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        } else {
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        }
        c.cycles += 2;
        let no_right = c.g[A1][0] == 0;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if !no_right {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_95ac);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_9558, 0x003a_95ac)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void PlaybackMan_Reset(PlaybackMan* self)` (0x26ca90): stop playback,
/// clear the state words and re-arm the replay buffer header.
pub fn playback_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0026_caa8);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0026_cdf8, 0x0026_caa8)?;
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x70), 0u64);
        let no_buf = c.g[S1][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        if !no_buf {
            c.cycles += 11;
            c.g[A2][0] = 0x10000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x0026_cacc);
            c.g[A2][0] = c.g[A2][0] | 0xc22c;
            call_ptr(c, 0x003e_6448, 0x0026_cacc)?;
            c.g[V1][0] = 0xffff_ffff_dbac_0000;
            c.g[V0][0] = 0x10000;
            c.g[V1][0] = c.g[V1][0] | 0x101;
            c.g[V0][0] = c.g[V0][0] | 0xc200;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[S1][0] as u32), c.g[V1][0]);
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void ScriptCamController_StopScriptCamera(ctrl, algo)` (0x1692d8): walk
/// the algorithm list for the entry with `algo->key` and remove it; clear the
/// active flag when the list empties.
pub fn script_cam_controller_stop_script_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        let inactive = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[A1][0];
        if !inactive {
            c.cycles += 2;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
            loop {
                c.cycles += 2;
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    break;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_9300);
                    }
                    continue;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x0016_9320);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0015_ca50, 0x0016_9320)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                break;
            }
            c.cycles += 2;
            if (c.g[V0][0] as i64) <= 0 {
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void GameStateFinish_LoadState(self, stream)` (0x234120): run the base
/// load, then read the two 4-byte fields through stream slot 2.
pub fn game_state_finish_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0023_413c);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0023_3b28, 0x0023_413c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_4158);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0023_4158)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_4174);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0023_4174)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP, RA as RET};
    use ssx3_ee as ee;

    /// Scrambled registers with a game-like `gp` (no fake call hooks: these
    /// tests only use callees that are already ported or reach no unknown
    /// code).
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x31_0102 ^ ((seed as u64) << 9) ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// The lazily built BE singleton (`gp-0x1f04`) pointed at a zeroed dummy,
    /// and the current interface index (`gp-0x1efc`) at zero.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
    }

    /// Call a guest setup function and return `v0`.
    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena: the game
    /// heap, for the paths that allocate more than the pools hold.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// `str_hash(s)`: the ELF string hash, used to build the hash-keyed tests.
    fn str_hash(c: &mut Cpu, s: u32) -> u64 {
        c.g[A0][0] = sx(s);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }

    /// The engine's size-class pool allocator (see `oc57`): short free lists
    /// behind `gp + 0x2a60..0x2a84`, enabled by the "pools ready" flag at
    /// `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    /// A two-word object at `obj` whose `+off` vtable slot calls the fake.
    fn fake_slot(c: &mut Cpu, r: &mut Rng, obj: u32, off: u32, slot: u32) {
        let vt = vtable(c, r, &[slot]);
        c.bus.write32(obj + off, vt);
    }

    // -----------------------------------------------------------------------

    #[test]
    fn flags_menu_dtor_matches() {
        check(ee::s06::f_0024ff28__generated, flags_menu_dtor, 40, &|c, seed| {
            base(c, seed, 0x24_ff28);
            c.g[A0][0] = sx(DATA);
            // Even flags: the base destructor must not free the test object
            // (the sub-objects are test data, not heap blocks).
            c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
            for off in [0x130u32, 0x14c, 0x164, 0x194] {
                c.bus.write32(DATA + off + 0x10, 0x1111);
                c.bus.write32(DATA + off + 0x12c, 0x2222);
            }
        });
    }

    #[test]
    fn mem_is_valid_block_link_matches() {
        check(ee::s06::f_00252248__generated, mem_is_valid_block_link, 60, &|c, seed| {
            let mut r = base(c, seed, 0x25_2248);
            // `EA_IsValidPointer` accepts the low EE window (< 0x100000) and
            // the high kernel windows; main RAM (0x800000) is rejected.
            let valid_ptr = seed % 4 != 0;
            c.g[A0][0] = if valid_ptr { 0x1000 } else { sx(DATA + 0x400) };
            let magic = match seed % 3 {
                0 => 0x4246u32,
                1 => 0x4253,
                _ => r.next_u32() & 0xffff,
            };
            c.bus.write16(0x1000, magic as u16);
            c.bus.write32(GAME_GP.wrapping_sub(0x27c), 0xbeef);
        });
    }

    #[test]
    fn be_reward_int_is_rival_reward_pending_matches() {
        check(ee::s01::f_00157b08__generated, be_reward_int_is_rival_reward_pending, 40, &|c, seed| {
            let mut r = base(c, seed, 0x15_7b08);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            c.g[A2][0] = (seed % 4) as u64;
            // The profile table stores offsets from 0x5305b20, not pointers.
            for i in 0..3u32 {
                let rec = DATA + 0x100 + 0x40 * i;
                c.bus.write32(0x0053_05b0 + 4 * i, rec.wrapping_sub(0x0530_5b20));
                c.bus.write32(rec + 0xc, if (seed + i) % 2 == 0 { 0xffff_ffff } else { 0 });
                c.bus.write32(rec + 0x10, if (seed + i) % 3 == 0 { 1 } else { 0 });
                c.bus.write8(rec + 0x11, (r.next_u32() & 7) as u8);
            }
        });
    }

    #[test]
    fn be_new_player_int_get_rider_state_flag_matches() {
        check(ee::s01::f_00146008__generated, be_new_player_int_get_rider_state_flag, 40, &|c, seed| {
            let mut r = base(c, seed, 0x14_6008);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            c.g[A2][0] = (seed % 4) as u64;
            // The profile table stores offsets from 0x5305b20, not pointers.
            for i in 0..3u32 {
                let rec = DATA + 0x100 + 0x40 * i;
                c.bus.write32(0x0053_05b0 + 4 * i, rec.wrapping_sub(0x0530_5b20));
                c.bus.write32(rec + 0xc, if (seed + i) % 2 == 0 { 0xffff_ffff } else { 0 });
                c.bus.write32(rec + 0x10, if (seed + i) % 3 == 0 { 1 } else { 0 });
                c.bus.write8(rec + 0x11, (r.next_u32() & 7) as u8);
            }
        });
    }

    #[test]
    fn cbx_string_make_upper_matches() {
        check(ee::s10::f_00318d28__generated, cbx_string_make_upper, 60, &|c, seed| {
            base(c, seed, 0x31_8d28);
            let buf = DATA + 0x400;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, buf);
            // Private buffer: `CopyBeforeWrite` takes its early out, so the
            // characters are uppercased in place.
            c.bus.write32(buf - 0xc, 1);
            let len = seed % 6;
            c.bus.write32(buf - 8, len);
            for i in 0..len {
                c.bus.write8(buf + i, b"aZ0_~"[((seed + i) % 5) as usize]);
            }
            // The class table at 0x499c81: bit 1 marks lowercase letters.
            for ch in [b'a', b'z'] {
                c.bus.write8(0x0049_9c81 + ch as u32, if seed % 2 == 0 { 2 } else { 0 });
            }
            c.bus.write8(0x0049_9c81 + b'A' as u32, 0);
        });
    }

    #[test]
    fn ui_engine_ctor_matches() {
        check(ee::s13::f_00397b08__generated, ui_engine_ctor, 30, &|c, seed| {
            base(c, seed, 0x39_7b08);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn particle_modifier_base_dtor_matches() {
        check(ee::s11::f_00345828__generated, particle_modifier_base_dtor, 40, &|c, seed| {
            base(c, seed, 0x34_5828);
            c.g[A0][0] = sx(DATA);
            // Even flags: no `operator delete` on test data.
            c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
            c.bus.write32(GAME_GP + 0xf04, seed % 5);
        });
    }

    #[test]
    fn halo_modifier_ctor_2_matches() {
        check(ee::s11::f_003461c0__generated, halo_modifier_ctor_2, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x34_61c0, 2);
            let stream = vtable(c, &mut r, &[0x10, 0x18]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(stream);
            c.bus.write32(GAME_GP + 0xf04, 3);
            c.bus.write32(GAME_GP + 0xf10, 3);
        });
    }

    #[test]
    fn game_state_finish_load_state_matches() {
        check(ee::s06::f_00234120__generated, game_state_finish_load_state, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x23_4120, 3);
            // The stream's vtable slot 2 (`+0x10`) is the fake field reader.
            let stream = vtable(c, &mut r, &[0x10]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(stream);
        });
    }

    #[test]
    fn multi_spline_modifier_alloc_node_instances_matches() {
        check(ee::s11::f_0035a3f0__generated, multi_spline_modifier_alloc_node_instances, 40, &|c, seed| {
            base(c, seed, 0x35_a3f0);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x4, (seed % 4) as u32);
            c.bus.write32(DATA + 0x44, DATA + 0x200);
            // gp-0x848 -> world, +0x84 -> module, +0x88 -> instance manager.
            c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
            c.bus.write32(DATA + 0x1100 + 0x88, DATA + 0x1200);
            // Empty instance-manager free list.
            c.bus.write32(DATA + 0x1200, 0);
            c.bus.write32(DATA + 0x1208, 0);
            c.bus.write32(DATA + 0x120c, 0);
        });
    }

    #[test]
    fn ui_pair_free_children_matches() {
        check(ee::s13::f_0039be70__generated, ui_pair_free_children, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x39_be70, 2);
            c.g[A0][0] = sx(DATA);
            let a = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
            let b = if seed % 4 == 0 { 0 } else { DATA + 0x300 };
            if a != 0 {
                fake_slot(c, &mut r, a, 0x8, 0x8);
            }
            if b != 0 {
                fake_slot(c, &mut r, b, 0x8, 0x8);
            }
            c.bus.write32(DATA + 0x78, a);
            c.bus.write32(DATA + 0x7c, b);
        });
    }

    #[test]
    fn fence_dtor_matches() {
        check(ee::s11::f_0034cb80__generated, fence_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x34_cb80, 1);
            c.g[A0][0] = sx(DATA);
            // Even flags: `WorldObj_dtor` passes them on to a process
            // destructor that would otherwise free the test object.
            c.g[A1][0] = [0u64, 2, 4][seed as usize % 3];
            c.bus.write32(DATA + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            // The world interface (gp+0x2a90), its table at +0x10d8 and slot 77.
            let vt = DATA + 0x300;
            c.bus.write16(vt + 0x268, 0);
            c.bus.write32(vt + 0x26c, FAKE);
            c.bus.write32(DATA + 0x400 + 0x10d8, vt);
            c.bus.write32(GAME_GP + 0x2a90, DATA + 0x400);
            c.bus.write32(DATA + 0x18, DATA + 0x500);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn feuicustom_v03_matches() {
        check(ee::s03::f_001a07c8__generated, feuicustom_v03, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1a_07c8, 1);
            let hash = str_hash(c, 0x0045_fff8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 0;
            c.g[A2][0] = if seed % 2 == 0 { hash } else { hash ^ 1 };
            let obj = DATA + 0x200;
            fake_slot(c, &mut r, obj, 0x8, 0xc0);
            c.bus.write32(DATA + 0x10, obj);
        });
    }

    #[test]
    fn femodule_try_exit_matches() {
        check(ee::s03::f_001a2128__generated, femodule_try_exit, 40, &|c, seed| {
            let mut r = base(c, seed, 0x1a_2128);
            c.g[A0][0] = sx(DATA);
            // `AppModule_TryExit` compares a float at self+4 with the global
            // at gp-0x51b4, so both outcomes are reachable.
            let t = if seed % 2 == 0 { 0.5f32 } else { 1.5f32 };
            c.bus.write32(DATA + 0x4, t.to_bits());
            c.bus.write32(GAME_GP.wrapping_sub(0x51b4), 1.0f32.to_bits());
            c.bus.write32(DATA + 0xb0000 + 0x5ad4, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
            c.bus.write32(DATA + 0xb0000 + 0x5ad8, 0xdead);
            c.bus.write32(DATA + 0xb0000 + 0x5adc, 0xdead);
            c.bus.write32(GAME_GP.wrapping_sub(0x1724), 0xdead);
            c.bus.write32(GAME_GP + 0x1d90, 0xdead);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn hud_meter_ctor_matches() {
        check(ee::s05::f_0020eaa0__generated, hud_meter_ctor, 40, &|c, seed| {
            base(c, seed, 0x20_eaa0);
            c.g[A0][0] = sx(DATA);
            if seed % 2 == 0 {
                c.bus.write32(GAME_GP.wrapping_sub(0x988), 0);
            } else {
                c.bus.write32(GAME_GP.wrapping_sub(0x988), 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x98c), DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x14, 320.0f32.to_bits());
                c.bus.write32(DATA + 0x200 + 0x34, 240.0f32.to_bits());
            }
            c.bus.write32(GAME_GP.wrapping_sub(0x54f4), 1.0f32.to_bits());
            c.bus.write32(GAME_GP.wrapping_sub(0x54f0), 1.0f32.to_bits());
        });
    }

    #[test]
    fn input_map_init_matches() {
        check(ee::s10::f_00320ca0__generated, input_map_init, 40, &|c, seed| {
            base(c, seed, 0x32_0ca0);
            pool_allocator(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (1 + seed % 8) as u64;
        });
    }

    

    #[test]
    fn be_mission_int_get_peak_collectible_total_matches() {
        check(ee::s01::f_00153390__generated, be_mission_int_get_peak_collectible_total, 40, &|c, seed| {
            let mut r = base(c, seed, 0x15_3390);
            let owner = if seed % 3 == 0 { 0x1234 } else { seed % 7 };
            c.g[A1][0] = owner as u64;
            for j in 0..22u32 {
                let rec = 0x0043_d950 + 0x64 * j;
                let this_owner = if (seed + j) % 4 == 0 { owner } else { r.next_u32() & 0x7fff_ffff };
                c.bus.write32(rec + 0x54, this_owner);
                c.bus.write8(0x0043_fa70 + j, (r.next_u32() % 200) as u8);
            }
        });
    }

    #[test]
    fn cfe_state_cheat_char_select_on_widget_notify_matches() {
        check(ee::s02::f_00182808__generated, cfe_state_cheat_char_select_on_widget_notify, 60, &|c, seed| {
            let mut r = base(c, seed, 0x18_2808);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = (seed % 13) as u64;
            c.g[A3][0] = (r.next_u32() % 8) as u64;
            c.bus.write32(DATA + 0x78, (r.next_u32() % 8) as u32);
            // `ApplyOutfit` reads a class table through self+0x6c and walks six
            // entries at self+0x54; all zeroed.
            c.bus.write32(DATA + 0x6c, DATA + 0x300);
            c.bus.write8(DATA + 0x300 + 0x98, (seed % 4) as u8);
        });
    }

    #[test]
    fn cfe_state_char_setup_ctor_matches() {
        check(ee::s02::f_001828c0__generated, cfe_state_char_setup_ctor, 30, &|c, seed| {
            let mut r = base(c, seed, 0x18_28c0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
            let idx = seed % 4;
            c.g[A2][0] = idx as u64;
            c.bus.write32(DATA + 0x200, DATA + 0x300);
            c.bus.write8(DATA + 0x300 + idx + 8, (r.next_u32() & 0x77) as u8);
        });
    }

    #[test]
    fn be_score_int_can_submit_score_matches() {
        check(ee::s01::f_001557e0__generated, be_score_int_can_submit_score, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x15_57e0, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(0x0053_5bfc, (seed % 3) as u32);
            c.bus.write32(0x0053_08b8, if seed % 3 == 0 { 0 } else { 1 });
            let _ = r.next_u32();
        });
    }

    #[test]
    fn cfe_options_credits_on_widget_action_matches() {
        check(ee::s02::f_001864b0__generated, cfe_options_credits_on_widget_action, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x18_64b0, 1);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x100) };
            c.g[A2][0] = if seed % 2 == 0 { 6 } else { (seed % 9) as u64 };
            // self+0x10 -> engine; engine+0 -> object; object+4 -> vtable.
            c.bus.write32(DATA + 0x10, DATA + 0x200);
            c.bus.write32(DATA + 0x200, DATA + 0x210);
            fake_slot(c, &mut r, DATA + 0x210, 0x4, 0x28);
        });
    }

    #[test]
    fn cfe_state_on_deactivate_matches() {
        check(ee::s02::f_001865a8__generated, cfe_state_on_deactivate, 40, &|c, seed| {
            base(c, seed, 0x18_65a8);
            let hash = str_hash(c, 0x004a_1398);
            c.g[A0][0] = sx(DATA);
            // The list at self+0x24: head at +0x28, sentinel check on +4.
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x28, 0);
            } else {
                let screen = DATA + 0x300;
                c.bus.write32(DATA + 0x28, screen);
                c.bus.write32(screen + 0x4, DATA + 0x310);
                if seed % 3 == 1 {
                    // FindObject walks a null array: returns null.
                    c.bus.write32(screen + 0x3c, 0);
                } else {
                    let count = DATA + 0x600;
                    let array = DATA + 0x610;
                    let obj = DATA + 0x700;
                    c.bus.write32(screen + 0x38, count);
                    c.bus.write32(count, 4);
                    c.bus.write32(screen + 0x3c, array);
                    c.bus.write32(array, obj);
                    c.bus.write32(obj + 0x38, hash as u32);
                    c.bus.write8(obj + 0x95, 0x2a);
                }
            }
            c.bus.write8(DATA + 0xc, 0x11);
        });
    }

    #[test]
    fn cfe_state_char_setup_set_rider_preview_active_matches() {
        check(ee::s02::f_001831d0__generated, cfe_state_char_setup_set_rider_preview_active, 40, &|c, seed| {
            base(c, seed, 0x18_31d0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x100) };
            // The rider lookup leaves v0 = a2 + module; keep a2 small so the
            // preview object stays inside the test data.
            c.g[A2][0] = 0;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x7c, if seed % 4 == 0 { 0 } else { DATA + 0x1100 });
            c.bus.write8(DATA + 0x44, (seed % 4) as u8);
            c.bus.write32(DATA + 0x48, 0xdead);
            c.bus.write32(DATA + 0x4c, 0xdead);
        });
    }

    #[test]
    fn cworld_cache_streamer_release_node_hulls_matches() {
        check(ee::s13::f_003a9558__generated, cworld_cache_streamer_release_node_hulls, 40, &|c, seed| {
            base(c, seed, 0x3a_9558);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA, DATA + 0x2000);
            c.bus.write32(GAME_GP + 0x2a74, 0);
            let n0 = DATA + 0x1000;
            let section = DATA + 0x1800;
            c.bus.write32(n0 + 0x50, 0);
            c.bus.write32(n0 + 0x54, 0);
            c.bus.write32(n0 + 0x58, if seed % 4 == 0 { 0 } else { section });
            c.bus.write16(section + 0x2, (seed % 3) as u16);
            if seed % 3 == 1 {
                // left child only
                let l = DATA + 0x1200;
                c.bus.write32(n0 + 0x50, l);
                c.bus.write32(l + 0x50, 0);
                c.bus.write32(l + 0x54, 0);
                c.bus.write32(l + 0x58, 0);
            } else if seed % 3 == 2 {
                // both children
                for child in [DATA + 0x1200u32, DATA + 0x1400] {
                    c.bus.write32(n0 + if child == DATA + 0x1200 { 0x50 } else { 0x54 }, child);
                    c.bus.write32(child + 0x50, 0);
                    c.bus.write32(child + 0x54, 0);
                    c.bus.write32(child + 0x58, 0);
                }
            }
            c.g[A1][0] = sx(n0);
        });
    }

    #[test]
    fn playback_man_reset_matches() {
        check(ee::s07::f_0026ca90__generated, playback_man_reset, 40, &|c, seed| {
            base(c, seed, 0x26_ca90);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x4, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
            // 2 = recording (the cheap stop path); 3 = already idle.
            c.bus.write32(DATA + 0x70, if seed % 2 == 0 { 2 } else { 3 });
            c.bus.write32(DATA + 0x10, 0);
            c.bus.write32(DATA + 0x1000, 0xdead);
            c.bus.write32(DATA + 0x1004, 0xdead);
        });
    }

    #[test]
    fn script_cam_controller_stop_script_camera_matches() {
        check(ee::s02::f_001692d8__generated, script_cam_controller_stop_script_camera, 40, &|c, seed| {
            base(c, seed, 0x16_92d8);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x1c, 0);
            c.bus.write32(DATA + 0x18, if seed % 5 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x8, DATA + 0x300);
            c.bus.write32(DATA + 0x304, if seed % 3 == 2 { 1 } else { 4 });
            match seed % 3 {
                0 => {
                    c.bus.write32(DATA + 0x300, 0);
                }
                1 => {
                    // No key match: the node at the head has key 0x77.
                    let node = DATA + 0x500;
                    c.bus.write32(node, 0x77);
                    c.bus.write32(node + 0x10, 0);
                    c.bus.write32(node + 0x14, 0);
                    c.bus.write32(DATA + 0x300, node);
                }
                _ => {
                    // A real heap block, so `AlgorithmList_Remove` can free it.
                    heap_init(c);
                    let node = run(
                        c,
                        0x0032_0058,
                        ee::s10::MEM_HeapMemalign,
                        &[0x004f_f250, 16, 0x40],
                    ) as u32;
                    for off in [0u32, 0x10, 0x14, 0x18] {
                        c.bus.write32(node + off, 0);
                    }
                    c.bus.write32(DATA + 0x300, node);
                }
            }
        });
    }
}
