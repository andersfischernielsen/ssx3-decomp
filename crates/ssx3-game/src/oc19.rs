//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements from the avalanche/transition/effects banks (s09)
//! and the script/container/memory banks (s10). Each copies the generated
//! body's logic, cycle charging and observable register (and FPU condition)
//! results exactly, with a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_55c0, avalanche_pack_clamped255x3, "Avalanche_PackClamped255x3"),
    (0x002d_5658, avalanche_pack_snorm3_alpha, "Avalanche_PackSnorm3Alpha"),
    (0x002d_7c00, avalanche_compute_instance_fade, "Avalanche_ComputeInstanceFade"),
    (0x002d_b9c8, c_crowd_anim2d_ctor, "cCrowdAnim2D_ctor"),
    (0x002d_bf80, c_ps2_fog_particle_man_base_ctor, "cPS2FogParticleMan_BaseCtor"),
    (0x002e_4638, c_transition_man_set_paused, "cTransitionMan_SetPaused"),
    (0x002e_4678, c_transition_man_get_time_remaining, "cTransitionMan_GetTimeRemaining"),
    (0x002e_4d70, transition_effect_ctor, "TransitionEffect_ctor"),
    (0x002e_4f10, snow_box_reset, "SnowBox_Reset"),
    (0x002e_6640, c_speed_boost_streamer_fx_reset, "cSpeedBoostStreamerFX_Reset"),
    (0x002e_a780, c_cut_scene_transition_man_start_out, "cCutSceneTransitionMan_StartOut"),
    (0x002e_a8b8, c_cut_scene_transition_man_set_paused, "cCutSceneTransitionMan_SetPaused"),
    (0x002e_adc0, rider_base_reset_power_up_trail, "RiderBase_ResetPowerUpTrail"),
    (0x002e_f6a0, c_air_streamer_fx_reset, "cAirStreamerFX_Reset"),
    (0x002f_0368, ai_visual_effects_man_ctor, "AIVisualEffectsMan_ctor"),
    (0x002f_19f0, attached_emitter_fx_set_model, "AttachedEmitterFX_SetModel"),
    (0x002f_39c8, c_splash_manager_reset, "cSplashManager_Reset"),
    (0x002f_4330, c_splash_manager_set_snow_amount, "cSplashManager_SetSnowAmount"),
    (0x002f_6b28, rider_base_clear_cheat_code_events, "RiderBase_ClearCheatCodeEvents"),
    (0x002f_6b38, rider_base_get_cheat_code_events, "RiderBase_GetCheatCodeEvents"),
    (0x0030_79d8, wscript_process_get_current_task_id, "WScriptProcess_GetCurrentTaskId"),
    (0x0030_97e0, wscript_man_post_trigger_event, "WScriptMan_PostTriggerEvent"),
    (0x0030_9848, wscript_man_post_jump_event, "WScriptMan_PostJumpEvent"),
    (0x0030_9918, wscript_man_post_trick_event, "WScriptMan_PostTrickEvent"),
    (0x0030_9990, wscript_man_post_wipeout_event, "WScriptMan_PostWipeoutEvent"),
    (0x0030_99f8, wscript_man_post_reset_event, "WScriptMan_PostResetEvent"),
    (0x0030_b058, wscript_process_list_ctor, "WScriptProcessList_ctor"),
    (0x0030_b2c0, wscript_process_list_push_front_2, "WScriptProcessList_PushFront_2"),
    (0x0030_c7c0, wscript_man_event_queue_push, "WScriptMan_EventQueuePush"),
    (0x0030_dba0, rider_model_set_color_scale, "RiderModel_SetColorScale"),
    (0x0030_ebc0, rider_model_is_part_visible, "RiderModel_IsPartVisible"),
    (0x0031_1290, adf_data_source_set_data, "AdfDataSource_SetData"),
    (0x0031_42e8, anim_seq_list_init, "AnimSeqList_Init"),
    (0x0031_4978, mdf_file_ctor, "MdfFile_ctor"),
    (0x0031_4aa0, mdf_file_parse_header, "MdfFile_ParseHeader"),
    (0x0031_4d98, game_anim_map_ctor, "GameAnimMap_ctor"),
    (0x0031_6898, big_file_ctor, "BigFile_ctor"),
    (0x0031_6a00, block_man_ctor, "BlockMan_ctor"),
    (0x0031_6a20, block_man_align_size, "BlockMan_AlignSize"),
    (0x0031_6cd0, app_ctor, "App_ctor"),
    (0x0031_7520, app_get_exec_man, "App_GetExecMan"),
    (0x0031_7550, app_load_progress_set_target, "App_LoadProgressSetTarget"),
    (0x0031_a088, mem_is_pool_pointer, "MEM_IsPoolPointer"),
    (0x0031_a130, mem_get_pool_block_size, "MEM_GetPoolBlockSize"),
    (0x0031_d368, md5_init, "MD5_Init"),
    (0x0031_d5e8, c_csi_cubic_spline_ctor, "cCSICubicSpline_ctor"),
    (0x0031_e1d8, dlheap_large_bin_index, "DLHeap_LargeBinIndex"),
    (0x0031_fbb8, dlheap_usable_size, "DLHeap_UsableSize"),
    (0x0032_0488, mem_pool8_free, "MEM_Pool8Free"),
    (0x0032_0498, mem_pool16_free, "MEM_Pool16Free"),
];

// ---------------------------------------------------------------------------
// Avalanche packing / fade.

/// `void Avalanche_PackClamped255x3(u8 *out, float x, float y, float z)`
/// (0x2d55c0): scale the three inputs by the global at `gp+0x948` (255.0),
/// clamp each to `[0, 1]` and convert to bytes at `out+3..6`.
pub fn avalanche_pack_clamped255x3(c: &mut Cpu) {
    c.cycles += 12;
    let out = abi::arg(c, 0);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x948));
    c.g[A1][0] = c.g[A0][0];
    c.mtc1(ZERO, 1); // f1 = 0.0
    c.mul_s(12, 12, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3); // f3 = 1.0
    c.mul_s(14, 14, 0);
    c.mov_s(2, 1);
    c.c_le_s(1, 12);
    let neg_x = !c.fcc();
    c.mul_s(13, 13, 0);
    if !neg_x {
        c.cycles += 1;
        c.min_s(2, 12, 3);
    }
    c.cycles += 4;
    c.c_le_s(1, 13);
    let neg_y = !c.fcc();
    c.mov_s(4, 1);
    if !neg_y {
        c.cycles += 1;
        c.min_s(4, 13, 3);
    }
    c.cycles += 4;
    c.c_le_s(1, 14);
    if c.fcc() {
        c.min_s(1, 14, 3);
    }
    c.cycles += 15;
    c.g[AT][0] = 0x437f_0000; // 255.0
    c.mtc1(AT, 0);
    c.mul_s(1, 1, 0);
    c.mul_s(2, 2, 0);
    c.mul_s(0, 4, 0);
    c.cvt_w_s(3, 1);
    c.mfc1(V0, 3);
    c.cvt_w_s(1, 2);
    c.mfc1(V1, 1);
    c.cvt_w_s(1, 0);
    c.mfc1(A0, 1);
    Addr(out).off(0x5).sb(c, c.g[V0][0]);
    Addr(out).off(0x3).sb(c, c.g[V1][0]);
    Addr(out).off(0x4).sb(c, c.g[A0][0]);
    abi::ret(c);
}

/// `void Avalanche_PackSnorm3Alpha(u8 *out, float x, float y, float z,
/// float a)` (0x2d5658): scale the x/y/z by 127 and the alpha by 255, convert
/// to integers and store at `out+6..10`.
pub fn avalanche_pack_snorm3_alpha(c: &mut Cpu) {
    c.cycles += 21;
    let out = abi::arg(c, 0);
    c.g[AT][0] = 0x42fe_0000; // 127.0
    c.mtc1(AT, 0);
    c.g[AT][0] = 0x437f_0000; // 255.0
    c.mtc1(AT, 1);
    c.mul_s(14, 14, 0);
    c.mul_s(12, 12, 0);
    c.mul_s(13, 13, 0);
    c.cvt_w_s(0, 14);
    c.mfc1(V0, 0);
    c.mul_s(15, 15, 1);
    c.cvt_w_s(0, 12);
    c.mfc1(V1, 0);
    c.cvt_w_s(0, 13);
    c.mfc1(A1, 0);
    Addr(out).off(0x8).sb(c, c.g[V0][0]);
    c.cvt_w_s(0, 15);
    c.mfc1(A2, 0);
    Addr(out).off(0x6).sb(c, c.g[V1][0]);
    Addr(out).off(0x7).sb(c, c.g[A1][0]);
    Addr(out).off(0x9).sb(c, c.g[A2][0]);
    abi::ret(c);
}

/// `void Avalanche_ComputeInstanceFade(void *self, float dt)` (0x2d7c00):
/// advance the 30Hz fade of the particle instance at `self+0x2e0`, writing the
/// fade ramp at `self+0xb0` and the fade fraction at `self+0xb4`.
pub fn avalanche_compute_instance_fade(c: &mut Cpu) {
    c.cycles += 9;
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 1);
    let this = abi::arg(c, 0);
    let inst = Addr(Addr(this).off(0x2e0).lw(c) as u32);
    c.g[V0][0] = inst.0 as u64;
    c.mul_s(12, 12, 1);
    c.lwc1(0, inst.off(0x104).0);
    c.c_lt_s(12, 0);
    if !c.fcc() {
        // dt >= lifetime: either ramp down from the range, or snap to 1.
        c.lwc1(1, inst.off(0x100).0);
        c.cycles += 6;
        c.lwc1(2, inst.off(0x108).0);
        c.sub_s(0, 1, 2);
        c.c_lt_s(12, 0);
        if !c.fcc() {
            c.g[V1][0] = inst.off(0xf0).lhu(c);
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 7;
                c.sub_s(0, 1, 12);
                c.mtc1(ZERO, 1);
                c.div_s(0, 0, 2);
                c.c_lt_s(1, 0);
                c.swc1(0, this.wrapping_add(0xb4));
                let nonpos = !c.fcc();
                c.swc1(0, this.wrapping_add(0xb0));
                if !nonpos {
                    c.cycles += 2;
                    c.sqrt_s(0, 0);
                    c.swc1(0, this.wrapping_add(0xb0));
                }
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.swc1(0, this.wrapping_add(0xb0));
            c.swc1(0, this.wrapping_add(0xb4));
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.swc1(1, this.wrapping_add(0xb0));
        c.lwc1(0, inst.off(0x104).0);
        c.div_s(0, 12, 0);
        c.swc1(0, this.wrapping_add(0xb4));
        abi::ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// 2D crowd / fog particles.

/// `cCrowdAnim2D *cCrowdAnim2D_ctor(cCrowdAnim2D *this, cCrowdAnim2DData *data)`
/// (0x2db9c8): initialise the animation record, storing the data pointer at
/// +0x1c and resolving the shared tables.
pub fn c_crowd_anim2d_ctor(c: &mut Cpu) {
    c.cycles += 16;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x4;
    Addr(this).off(0x1c).sw(c, c.g[A1][0]);
    Addr(this).off(0x8).sw(c, 0x4);
    c.g[A1][0] = 0x44_0000;
    Addr(this).off(0x4).sw(c, 0);
    c.g[A3][0] = 0x3;
    Addr(this).off(0xc).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x5dd0)).lw(c);
    Addr(this).sw(c, 0);
    Addr(this).off(0x10).sw(c, c.g[V1][0]);
    c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x1c)).lw(c);
    Addr(this).off(0x14).sw(c, c.g[A3][0]);
    Addr(this).off(0x18).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void cPS2FogParticleMan_BaseCtor(cPS2FogParticleMan *this)` (0x2dbf80):
/// clear the first word and install the vtable at +4.
pub fn c_ps2_fog_particle_man_base_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7cf8));
    Addr(this).sw(c, 0);
    Addr(this).off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Transition managers.

/// `void cTransitionMan_SetPaused(cTransitionMan *this, int paused)`
/// (0x2e4638): while active, bump or drop the pause hold counter at +0x40.
pub fn c_transition_man_set_paused(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x44).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        let paused = c.g[A1][0];
        c.g[V0][0] = Addr(this).off(0x40).lw(c);
        if paused == 0 {
            c.cycles += 2;
            let was_zero = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if !was_zero {
                c.cycles += 1;
                Addr(this).off(0x40).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(this).off(0x40).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 2;
        Addr(this).off(0x40).sw(c, 0);
        abi::ret(c);
        return;
    }
}

/// `float cTransitionMan_GetTimeRemaining(cTransitionMan *this)` (0x2e4678):
/// remaining transition time, clamped at zero.
pub fn c_transition_man_get_time_remaining(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x44).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.mtc1(ZERO, 0); // f0 = 0
    } else {
        c.cycles += 12;
        c.lwc1(0, this.wrapping_add(0x58));
        c.lwc1(1, this.wrapping_add(0x54));
        c.lwc1(3, this.wrapping_add(0x5c));
        c.add_s(1, 1, 0);
        c.lwc1(4, this.wrapping_add(0x48));
        c.mtc1(ZERO, 2); // f2 = 0
        c.add_s(1, 1, 3);
        c.sub_s(1, 1, 4);
        c.c_lt_s(1, 2);
        let below = c.fcc();
        c.mov_s(0, 2);
        if !below {
            c.cycles += 2;
            c.mov_s(0, 1);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void TransitionEffect_ctor(TransitionEffect *this)` (0x2e4d70): install
/// the vtable at +0.
pub fn transition_effect_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7f20));
    Addr(this).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void SnowBox_Reset(SnowBox *this)` (0x2e4f10): clear the particle ranges
/// and arm the box.
pub fn snow_box_reset(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x1;
    let this = abi::arg(c, 0);
    Addr(this).off(0x90).sw(c, 0);
    Addr(this).off(0xb0).sw(c, c.g[V0][0]);
    Addr(this).off(0x94).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Speed-boost streamer.

/// `void cSpeedBoostStreamerFX_Reset(cSpeedBoostStreamerFX *this)` (0x2e6640):
/// build the default emit descriptor in the local frame, splat it into the
/// object and set the streamer id from the owning rider's id.
pub fn c_speed_boost_streamer_fx_reset(c: &mut Cpu) {
    c.cycles += 29;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0); // f0 = 1.0
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let sp = c.g[SP][0] as u32;
    let this = abi::arg(c, 0);
    Addr(sp).off(0x4).sw(c, 0);
    c.g[V1][0] = 0x39;
    c.swc1(0, sp.wrapping_add(0xc));
    c.g[A2][0] = 0x14;
    c.swc1(0, sp);
    c.g[A1][0] = 0x12;
    Addr(sp).off(0x8).sw(c, 0);
    Addr(this).off(0x398).sw(c, c.g[V1][0]);
    c.g[V1][0] = Addr(this).lw(c);
    c.lq(2, sp);
    Addr(this).off(0x8).sw(c, 0);
    c.sq(2, this.wrapping_add(0x40));
    Addr(this).off(0xc).sw(c, 0);
    Addr(this).off(0x10).sw(c, 0);
    Addr(this).off(0x18).sw(c, 0);
    Addr(this).off(0x14).sw(c, 0);
    Addr(this).off(0x30).sw(c, 0);
    Addr(this).off(0x3a0).sw(c, 0);
    Addr(this).off(0x39c).sw(c, 0);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x870)).lw(c);
    Addr(this).off(0x3a8).sw(c, 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 0i64) as u64;
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[A2][0];
    }
    Addr(this).off(0x3a4).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Cut-scene transitions.

/// `void cCutSceneTransitionMan_StartOut(cCutSceneTransitionMan *this,
/// float speed)` (0x2ea780): kick off a transition depending on the state at
/// +0x10 (0 idle, 1 in, 2 out, 3 pending), writing the phase and rates.
pub fn c_cut_scene_transition_man_start_out(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    'b_2ea814: {
        'b_2ea80c: {
            'b_2ea7e8: {
                'b_2ea7cc: {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(this).off(0x10).lw(c);
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        c.cycles += 3;
                        c.swc1(12, this.wrapping_add(0x18));
                        c.sw(this.wrapping_add(0x14), 0);
                        break 'b_2ea80c;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x2;
                        if t {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_2ea7cc;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_2ea7e8;
                                } else {
                                    c.cycles += 2;
                                    Addr(this).off(0x10).sw(c, c.g[V0][0]);
                                    abi::ret(c);
                                    return;
                                }
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0u64;
                            if t {
                                break 'b_2ea814;
                            } else {
                                c.cycles += 2;
                                Addr(this).off(0x10).sw(c, c.g[V0][0]);
                                abi::ret(c);
                                return;
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.lwc1(0, this.wrapping_add(0x14));
                c.lwc1(1, this.wrapping_add(0x18));
                c.swc1(12, this.wrapping_add(0x18));
                c.div_s(0, 0, 1);
                c.mul_s(0, 0, 12);
                c.swc1(0, this.wrapping_add(0x14));
                break 'b_2ea80c;
            }
            c.cycles += 9;
            c.lwc1(0, this.wrapping_add(0x14));
            c.lwc1(2, this.wrapping_add(0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1); // f1 = 1.0
            c.div_s(0, 0, 2);
            c.swc1(12, this.wrapping_add(0x18));
            c.sub_s(1, 1, 0);
            c.mul_s(1, 1, 12);
            c.swc1(1, this.wrapping_add(0x14));
            break 'b_2ea80c;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        Addr(this).off(0x10).sw(c, c.g[V0][0]);
        break 'b_2ea814;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cCutSceneTransitionMan_SetPaused(cCutSceneTransitionMan *this,
/// int paused)` (0x2ea8b8): while fading (state 2/3), bump or drop the hold
/// counter at +0x1c.
pub fn c_cut_scene_transition_man_set_paused(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        let paused = c.g[A1][0];
        c.g[V0][0] = Addr(this).off(0x1c).lw(c);
        if paused == 0 {
            c.cycles += 2;
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if positive {
                c.cycles += 1;
                Addr(this).off(0x1c).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(this).off(0x1c).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 2;
        Addr(this).off(0x1c).sw(c, 0);
        abi::ret(c);
        return;
    }
}

/// `void RiderBase_ResetPowerUpTrail(Rider *rider)` (0x2eadc0): clear the
/// power-up trail state.
pub fn rider_base_reset_power_up_trail(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    Addr(this).off(0x10).sw(c, 0);
    Addr(this).off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `void cAirStreamerFX_Reset(cAirStreamerFX *this)` (0x2ef6a0): reset the
/// streamer and set its id from the owning rider's id.
pub fn c_air_streamer_fx_reset(c: &mut Cpu) {
    c.cycles += 11;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).lw(c);
    c.g[A1][0] = 0x12;
    c.g[A2][0] = 0x19;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x870)).lw(c);
    Addr(this).off(0x10).sw(c, 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 0i64) as u64;
    Addr(this).off(0x8).sw(c, 0);
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[A2][0];
    }
    Addr(this).off(0xc).sw(c, 0);
    Addr(this).off(0x14).sw(c, c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Visual effects.

/// `void AIVisualEffectsMan_ctor(AIVisualEffectsMan *this)` (0x2f0368):
/// install the vtable and publish the singleton at `gp+0xa90`.
pub fn ai_visual_effects_man_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x48_0000;
    let this = abi::arg(c, 0);
    Addr(this).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d78));
    Addr(this).off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0xa90)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

/// `void AttachedEmitterFX_SetModel(AttachedEmitterFX *this, void *model)`
/// (0x2f19f0): store the model at +0x94 and clear the active flag.
pub fn attached_emitter_fx_set_model(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    Addr(this).off(0x94).sw(c, c.g[A1][0]);
    Addr(this).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Splash manager.

/// `void cSplashManager_Reset(cSplashManager *this)` (0x2f39c8): clear the
/// counters and timers.
pub fn c_splash_manager_reset(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    Addr(this).off(0x1028).sw(c, 0);
    Addr(this).off(0x100c).sw(c, 0);
    Addr(this).off(0x1024).sw(c, 0);
    Addr(this).off(0x14).sw(c, 0);
    Addr(this).off(0x18).sw(c, 0);
    abi::ret(c);
}

/// `void cSplashManager_SetSnowAmount(cSplashManager *this, float amount)`
/// (0x2f4330): store the amount and, when it crosses the global threshold,
/// integrate it into the splash accumulation at +0x1024.
pub fn c_splash_manager_set_snow_amount(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0);
    c.swc1(12, this.wrapping_add(0x1028));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x1190));
    c.c_lt_s(0, 12);
    let below = !c.fcc();
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x1194));
    if below {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[AT][0] = 0x4120_0000; // 10.0
    c.mtc1(AT, 1);
    c.lwc1(0, this.wrapping_add(0x1020));
    c.add_s(0, 0, 1);
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x38f4));
    c.lwc1(1, this.wrapping_add(0x1024));
    c.mul_s(2, 2, 3);
    c.mul_s(0, 12, 0);
    c.mul_s(0, 0, 2);
    c.add_s(1, 1, 0);
    c.swc1(1, this.wrapping_add(0x1024));
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider cheat events.

/// `void RiderBase_ClearCheatCodeEvents(Rider *rider)` (0x2f6b28): clear the
/// low cheat-event bits of the byte at +7.
pub fn rider_base_clear_cheat_code_events(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x7).lbu(c);
    c.g[V0][0] = c.g[V0][0] & 0x8f;
    Addr(this).off(0x7).sb(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int RiderBase_GetCheatCodeEvents(Rider *rider)` (0x2f6b38): the high
/// cheat-event bits of the byte at +7.
pub fn rider_base_get_cheat_code_events(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x7).lbu(c);
    c.g[V0][0] = c.g[V0][0] & 0x70;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WScript processes.

/// `int WScriptProcess_GetCurrentTaskId(WScriptProcess *this)` (0x3079d8):
/// the task id of the current instruction, or -1 when exhausted.
pub fn wscript_process_get_current_task_id(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x8).lw(c);
    c.g[V0][0] = Addr(this).off(0x4).lw(c);
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let in_range = c.g[V0][0] != 0;
    c.g[V1][0] = 0x44;
    if in_range {
        c.cycles += 5;
        c.g[V0][0] = Addr(this).off(0x14).lw(c);
        c.mult(0, V1, A1, V1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    abi::ret(c);
}

/// `void WScriptMan_PostTriggerEvent(WScriptMan *this, int value)` (0x3097e0):
/// append a `0x70000` trigger event to the queue at `this+0x40`.
pub fn wscript_man_post_trigger_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).off(0x40).lw(c);
    c.g[V0][0] = 0xc;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.g[A2][0] = 0x7_0000;
    c.mult(0, V0, V1, V0);
    Addr(sp).off(0x4).sw(c, c.g[A1][0]);
    Addr(sp).sw(c, c.g[A2][0]);
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(A2, sp.wrapping_add(0x7));
    c.ldr(A2, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(A2, sp.wrapping_add(0x17));
    c.sdr(A2, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.ldl(A1, sp.wrapping_add(0x17));
    c.ldr(A1, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(A1, (c.g[V0][0] as u32).wrapping_add(0xb));
    c.sdr(A1, (c.g[V0][0] as u32).wrapping_add(0x4));
    Addr((c.g[V0][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void WScriptMan_PostJumpEvent(WScriptMan *this)` (0x309848): append a
/// `0x70001` jump event.
pub fn wscript_man_post_jump_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = c.g[V0][0] | 0x1;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.ldl(V0, sp.wrapping_add(0x17));
    c.ldr(V0, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(V0, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void WScriptMan_PostTrickEvent(WScriptMan *this, const void *trick)`
/// (0x309918): append a `0x70003` trick event carrying the first quad of the
/// trick record.
pub fn wscript_man_post_trick_event(c: &mut Cpu) {
    c.cycles += 29;
    let this = abi::arg(c, 0);
    let trick = c.g[A1][0] as u32;
    c.g[A2][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A2, V1);
    c.g[V0][0] = c.g[V0][0] | 0x3;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, trick.wrapping_add(0x7));
    c.ldr(V0, trick);
    c.sdl(V0, sp.wrapping_add(0xb));
    c.sdr(V0, sp.wrapping_add(0x4));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A1][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.ldl(T2, sp.wrapping_add(0x17));
    c.ldr(T2, sp.wrapping_add(0x10));
    c.g[V0][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(T2, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(T2, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
    Addr(this).off(0x40).sw(c, c.g[A2][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void WScriptMan_PostWipeoutEvent(WScriptMan *this)` (0x309990): append a
/// `0x70004` wipeout event.
pub fn wscript_man_post_wipeout_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = c.g[V0][0] | 0x4;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.ldl(V0, sp.wrapping_add(0x17));
    c.ldr(V0, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(V0, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void WScriptMan_PostResetEvent(WScriptMan *this)` (0x3099f8): append a
/// `0x70005` reset event.
pub fn wscript_man_post_reset_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = c.g[V0][0] | 0x5;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.ldl(V0, sp.wrapping_add(0x17));
    c.ldr(V0, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(V0, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void WScriptProcessList_ctor(WScriptProcessList *this)` (0x30b058):
/// clear the list head.
pub fn wscript_process_list_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}

/// `void WScriptProcessList_PushFront_2(WScriptProcessList *this,
/// WScriptProcess *item)` (0x30b2c0): link `item` at the front, chaining the
/// previous head at `item+0x18`.
pub fn wscript_process_list_push_front_2(c: &mut Cpu) {
    c.cycles += 4;
    let list = abi::arg(c, 0);
    c.g[V0][0] = Addr(list).lw(c);
    Addr(c.g[A1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
    Addr(list).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void WScriptMan_EventQueuePush(WScriptMan *this, int a, int b, int c)`
/// (0x30c7c0): store three words at the ring slot `this+4` (12-byte records,
/// 16 per page) and advance the wrapped index.
pub fn wscript_man_event_queue_push(c: &mut Cpu) {
    c.cycles += 24;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).off(0x4).lw(c);
    c.g[T0][0] = 0xc;
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V0, V1, T0);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(this).off(0x4).lw(c);
    c.mult(0, V1, V0, T0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr((c.g[V0][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    c.g[V1][0] = Addr(this).off(0x4).lw(c);
    c.mult(0, V0, V1, T0);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sw(c, c.g[A3][0]);
    c.g[V0][0] = Addr(this).off(0x4).lw(c);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[T1][0] = ((c.g[T1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    if c.g[T1][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    Addr(this).off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider models.

/// `void RiderModel_SetColorScale(RiderModel *model, float r, float g)`
/// (0x30dba0): build the `(r, g, g, 1)` quad and store it at `model+0x140`.
pub fn rider_model_set_color_scale(c: &mut Cpu) {
    c.cycles += 11;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0); // f0 = 1.0
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let sp = c.g[SP][0] as u32;
    let this = abi::arg(c, 0);
    c.swc1(12, sp);
    c.swc1(13, sp.wrapping_add(0x8));
    c.swc1(0, sp.wrapping_add(0xc));
    c.swc1(13, sp.wrapping_add(0x4));
    c.lq(V0, sp);
    c.sq(V0, this.wrapping_add(0x140));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `int RiderModel_IsPartVisible(RiderModel *model, int part, int sub)`
/// (0x30ebc0): resolve `model->parts[part]` and return the visible entry for
/// the sub-part at `entry*0x58 + model->offset + sub*8` + 0x18, or 0 when the
/// part is out of range / unloaded / not visible.
pub fn rider_model_is_part_visible(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x14).lw(c);
    c.g[V1][0] = c.g[A1][0];
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(this).off(0x1c).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    let visible = (c.g[V0][0] as i64) >= 0;
    c.g[V1][0] = 0x58;
    if !visible {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(this).off(0xc).lw(c);
    c.mult(0, V1, V0, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Container / animation helpers.

/// `void AdfDataSource_SetData(AdfDataSource *this, const u8 *data)`
/// (0x311290): publish the four variable-length sections of the data block.
pub fn adf_data_source_set_data(c: &mut Cpu) {
    c.cycles += 13;
    let this = abi::arg(c, 0);
    let src = c.g[A1][0] as u32;
    c.g[V1][0] = sx(src.wrapping_add(0x10));
    Addr(this).off(0x4).sw(c, c.g[A1][0]);
    Addr(this).off(0x8).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(src).off(0x4).lw(c);
    c.g[V0][0] = sx(src.wrapping_add(c.g[V0][0] as u32));
    Addr(this).off(0xc).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(src).off(0x8).lw(c);
    c.g[V1][0] = sx(src.wrapping_add(c.g[V1][0] as u32));
    Addr(this).off(0x10).sw(c, c.g[V1][0]);
    c.g[V0][0] = Addr(src).off(0xc).lw(c);
    c.g[A1][0] = sx(src.wrapping_add(c.g[V0][0] as u32));
    Addr(this).off(0x14).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AnimSeqList_Init(AnimSeqList *this)` (0x3142e8): clear the head and
/// count.
pub fn anim_seq_list_init(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, 0);
    abi::ret(c);
}

/// `MdfFile *MdfFile_ctor(MdfFile *this)` (0x314978): clear the header word.
pub fn mdf_file_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}

/// `void MdfFile_ParseHeader(MdfFile *this, const u8 *data)` (0x314aa0):
/// store the data pointer and resolve the geometry/vertex sections.
pub fn mdf_file_parse_header(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    let data = c.g[A1][0] as u32;
    Addr(this).off(0x4).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(data).off(0x6).lh(c);
    c.g[V0][0] = sx(data.wrapping_add(c.g[V0][0] as u32));
    Addr(this).off(0x8).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(data).off(0x8).lw(c);
    c.g[A1][0] = sx(data.wrapping_add(c.g[V1][0] as u32));
    Addr(this).off(0xc).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void GameAnimMap_ctor(GameAnimMap *this)` (0x314d98): publish the map at
/// `gp+0xd8c`.
pub fn game_anim_map_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x205;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0xd8c)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

/// `BigFile *BigFile_ctor(BigFile *this)` (0x316898): mark empty with size -1.
pub fn big_file_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `BlockMan *BlockMan_ctor(BlockMan *this)` (0x316a00): clear the free lists
/// and set the block size to 16.
pub fn block_man_ctor(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x10;
    Addr(this).sw(c, 0);
    Addr(this).off(0x10).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).off(0x8).sw(c, 0);
    Addr(this).off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `int BlockMan_AlignSize(BlockMan *this, int size)` (0x316a20): round `size`
/// up to the allocator's block granularity at +0x10.
pub fn block_man_align_size(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x10).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
    c.div(0, A1, V0);
    c.g[A1][0] = c.lo[0];
    c.mult(0, V0, V0, A1);
    abi::ret(c);
}

/// `App *App_ctor(App *this)` (0x316cd0): initialise the app and publish it
/// at `gp+0x2a74`.
pub fn app_ctor(c: &mut Cpu) {
    c.cycles += 21;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x49_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2efc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2418));
    c.g[V1][0] = 0xc;
    Addr(this).off(0x5c).sw(c, c.g[V0][0]);
    Addr(this).off(0x20).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[A0][0];
    c.swc1(0, this.wrapping_add(0x24));
    c.swc1(1, this.wrapping_add(0x30));
    Addr(this).off(0x34).sw(c, 0);
    Addr(this).sw(c, 0);
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).off(0x1c).sw(c, 0);
    Addr(this).off(0x18).sw(c, 0);
    Addr(this).off(0x28).sw(c, 0);
    Addr(this).off(0x2c).sw(c, 0);
    Addr(this).off(0xc).sw(c, 0);
    Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

/// `ExecMan *App_GetExecMan(void)` (0x317520): `app->execMan` at `app+8`.
pub fn app_get_exec_man(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x8).lw(c);
    abi::ret(c);
}

/// `void App_LoadProgressSetTarget(App *this, float target)` (0x317550):
/// set up the loading progress interpolation from the current values and the
/// target at `f12`.
pub fn app_load_progress_set_target(c: &mut Cpu) {
    c.cycles += 20;
    let this = abi::arg(c, 0);
    c.lwc1(5, this.wrapping_add(0x8));
    c.lwc1(1, this.wrapping_add(0xc));
    c.sub_s(3, 12, 5);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x2ef4));
    c.g[AT][0] = 0xffff_ffff_bfc0_0000;
    c.mtc1(AT, 2); // f2 = -1.5
    c.lwc1(0, this);
    c.mul_s(1, 1, 3);
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.sub_s(0, 0, 12);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
    c.mul_s(1, 1, 4);
    c.swc1(5, this.wrapping_add(0x4));
    c.swc1(0, this.wrapping_add(0x18));
    Addr(this).off(0x10).sw(c, c.g[V1][0]);
    c.div_s(2, 2, 1);
    c.swc1(12, this.wrapping_add(0x8));
    c.swc1(12, this.wrapping_add(0x14));
    c.swc1(2, this.wrapping_add(0x1c));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory manager.

/// `int MEM_IsPoolPointer(void *ptr)` (0x31a088): true when `ptr` falls in
/// one of the fixed-size pool ranges described by the globals at gp+0x2a60..
pub fn mem_is_pool_pointer(c: &mut Cpu) {
    let a0 = abi::arg(c, 0);
    let gp = c.g[GP][0] as u32;
    'b_31a0f4: {
        c.cycles += 5;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a70)).lw(c);
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x800u64) as u64;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a60)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x2000u64) as u64;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a6c)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x6000u64) as u64;
        let t = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(gp.wrapping_add(0x2a7c)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 7;
        c.g[V0][0] = 0x3_0000;
        c.g[V0][0] = c.g[V0][0] | 0x1fff;
        c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] != 0;
        c.g[V1][0] = Addr(gp.wrapping_add(0x2a84)).lw(c);
        if t {
            break 'b_31a0f4;
        }
        c.cycles += 7;
        c.g[V0][0] = 0x2_0000;
        c.g[V0][0] = c.g[V0][0] | 0x57ff;
        c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a78)).lw(c);
        if t {
            c.cycles += 7;
            c.g[A1][0] = 0x1_0000;
            c.g[A1][0] = c.g[A1][0] | 0xffff;
            c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            let t = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp.wrapping_add(0x2a68)).lw(c);
            if t {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                abi::ret(c);
                return;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            abi::ret(c);
            return;
        }
        break 'b_31a0f4;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    abi::ret(c);
}

/// `int MEM_GetPoolBlockSize(void *ptr)` (0x31a130): the block size (8, 16,
/// 32, 64, 128, 256 or 512) of the pool containing `ptr`, or 0.
pub fn mem_get_pool_block_size(c: &mut Cpu) {
    c.cycles += 5;
    let a0 = abi::arg(c, 0);
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x2a70)).lw(c);
    c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
    c.g[V0][0] = (c.g[V0][0] < 0x800u64) as u64;
    let t = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(gp.wrapping_add(0x2a60)).lw(c);
    if t {
        c.cycles += 4;
        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[V0][0] < 0x2000u64) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2a6c)).lw(c);
        if t {
            c.cycles += 4;
            c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[V0][0] < 0x6000u64) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V1][0] = Addr(gp.wrapping_add(0x2a7c)).lw(c);
            if t {
                c.cycles += 7;
                c.g[V0][0] = 0x3_0000;
                c.g[V0][0] = c.g[V0][0] | 0x1fff;
                c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                let t = c.g[V0][0] == 0;
                c.g[V1][0] = Addr(gp.wrapping_add(0x2a84)).lw(c);
                if t {
                    c.cycles += 7;
                    c.g[V0][0] = 0x2_0000;
                    c.g[V0][0] = c.g[V0][0] | 0x57ff;
                    c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(gp.wrapping_add(0x2a78)).lw(c);
                    if t {
                        c.cycles += 7;
                        c.g[A1][0] = 0x1_0000;
                        c.g[A1][0] = c.g[A1][0] | 0xffff;
                        c.g[V0][0] = sx(a0.wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        let t = c.g[V0][0] != 0;
                        c.g[V1][0] = Addr(gp.wrapping_add(0x2a68)).lw(c);
                        if t {
                            c.cycles += 2;
                            c.g[V0][0] = 0x100;
                            abi::ret(c);
                            return;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = 0x200;
                        c.g[V1][0] = sx(a0.wrapping_sub(c.g[V1][0] as u32));
                        c.g[V1][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = 0;
                        }
                        abi::ret(c);
                        return;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x80;
                    abi::ret(c);
                    return;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x40;
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x20;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x10;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x8;
    abi::ret(c);
}

/// `void MD5_Init(MD5_CTX *ctx)` (0x31d368): load the standard MD5 constants.
pub fn md5_init(c: &mut Cpu) {
    c.cycles += 15;
    let this = abi::arg(c, 0);
    c.g[V1][0] = 0x6745_0000;
    c.g[A1][0] = 0xffff_ffff_efcd_0000;
    c.g[A2][0] = 0xffff_ffff_98ba_0000;
    c.g[V0][0] = 0x1032_0000;
    c.g[V1][0] = c.g[V1][0] | 0x2301;
    c.g[A1][0] = c.g[A1][0] | 0xab89;
    c.g[A2][0] = c.g[A2][0] | 0xdcfe;
    c.g[V0][0] = c.g[V0][0] | 0x5476;
    Addr(this).off(0x8).sw(c, c.g[V1][0]);
    Addr(this).off(0x14).sw(c, c.g[V0][0]);
    Addr(this).off(0xc).sw(c, c.g[A1][0]);
    Addr(this).off(0x10).sw(c, c.g[A2][0]);
    Addr(this).off(0x4).sw(c, 0);
    Addr(this).sw(c, 0);
    abi::ret(c);
}

/// `cCSICubicSpline *cCSICubicSpline_ctor(cCSICubicSpline *this)`
/// (0x31d5e8): mask the flags word at +0 and clear the count at +8.
pub fn c_csi_cubic_spline_ctor(c: &mut Cpu) {
    c.cycles += 12;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).lw(c);
    c.g[A2][0] = 0xffff_ffff_ffff_fffd;
    c.g[A3][0] = 0xffff_ffff_ffff_fffb;
    c.g[A1][0] = 0xffff_ffff_fff8_0000;
    c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
    c.g[A1][0] = c.g[A1][0] | 0x7ff;
    c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
    Addr(this).off(0x8).sw(c, 0);
    Addr(this).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `int DLHeap_LargeBinIndex(uint size)` (0x31e1d8): the free-list bin for a
/// large allocation of `size` bytes (0x5f when the size index is too big).
pub fn dlheap_large_bin_index(c: &mut Cpu) {
    c.cycles += 6;
    let size = abi::arg(c, 0);
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = 0xffff;
    c.g[A0][0] = sx(size >> 8);
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    let too_big = c.g[V0][0] != 0;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x100));
    if too_big {
        c.cycles += 2;
        c.g[V0][0] = 0x5f;
        abi::ret(c);
        return;
    }
    c.cycles += 26;
    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 16);
    c.g[V1][0] = c.g[V1][0] & 0x8;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1000));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
    c.g[A1][0] = c.g[V0][0] & 0x4;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4000));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
    c.g[A1][0] = c.g[V0][0] & 0x2;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 15);
    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 14);
    c.g[V0][0] = !(0u64 | c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd));
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6));
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
    c.g[V0][0] = c.g[V0][0] & 0x3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `uint DLHeap_UsableSize(void *block)` (0x31fbb8): usable payload size of a
/// heap block, from its header word at `block-4`.
pub fn dlheap_usable_size(c: &mut Cpu) {
    let a1 = abi::arg(c, 1);
    'b_31fc18: {
        c.cycles += 2;
        if a1 == 0 {
            break 'b_31fc18;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr(a1.wrapping_sub(4)).lw(c);
        c.g[V0][0] = c.g[A0][0] & 0x2;
        let small = c.g[V0][0] == 0;
        c.g[A1][0] = sx(a1.wrapping_sub(8));
        if small {
            c.cycles += 8;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xfffe;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x1;
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] = c.g[V0][0] | 0xfffc;
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                abi::ret(c);
                return;
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xfffc;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void MEM_Pool8Free(MEM_Pool8 *pool, void *ptr)` (0x320488): push `ptr` on
/// the pool's free list at +0x800.
pub fn mem_pool8_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = abi::arg(c, 0);
    c.g[V0][0] = Addr(pool).off(0x800).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    Addr(pool).off(0x800).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void MEM_Pool16Free(MEM_Pool16 *pool, void *ptr)` (0x320498): push `ptr`
/// on the pool's free list at +0x2000.
pub fn mem_pool16_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = abi::arg(c, 0);
    c.g[V0][0] = Addr(pool).off(0x2000).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    Addr(pool).off(0x2000).sw(c, c.g[A1][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn avalanche_pack_clamped255x3_matches() {
        check(ssx3_ee::s09::f_002d55c0, avalanche_pack_clamped255x3, 250, &|c, seed| {
            let mut r = Rng(0x1001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP + 0x948, r.edge_f32());
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.f[14] = r.edge_f32();
            if seed % 3 == 0 {
                c.f[12] |= 0x8000_0000;
            }
            if seed % 5 == 0 {
                c.f[13] |= 0x8000_0000;
            }
            if seed % 7 == 0 {
                c.f[14] |= 0x8000_0000;
            }
        });
    }

    #[test]
    fn avalanche_pack_snorm3_alpha_matches() {
        check(ssx3_ee::s09::f_002d5658, avalanche_pack_snorm3_alpha, 200, &|c, seed| {
            let mut r = Rng(0x1002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..4 {
                c.f[12 + i] = r.edge_f32();
            }
        });
    }

    #[test]
    fn avalanche_compute_instance_fade_matches() {
        check(ssx3_ee::s09::f_002d7c00, avalanche_compute_instance_fade, 300, &|c, seed| {
            let mut r = Rng(0x1003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let inst = DATA + 0x400;
            c.bus.write32(DATA + 0x2e0, inst);
            c.bus.write32(inst + 0x100, r.edge_f32());
            c.bus.write32(inst + 0x104, r.edge_f32());
            c.bus.write32(inst + 0x108, r.edge_f32());
            c.bus.write16(
                inst + 0xf0,
                if seed % 3 == 0 { 2 } else { (r.next_u32() & 0xffff) as u16 },
            );
            c.f[12] = match seed % 5 {
                0 => 0,
                1 => 0x8000_0000,
                2 => 0x7f7f_ffff,
                3 => r.edge_f32(),
                _ => r.f32(1e4),
            };
        });
    }

    #[test]
    fn c_crowd_anim2d_ctor_matches() {
        check(ssx3_ee::s09::f_002db9c8, c_crowd_anim2d_ctor, 200, &|c, seed| {
            let mut r = Rng(0x1004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let table = DATA + 0x500;
            c.bus.write32(GAME_GP + 0x2a74, table);
            c.bus.write32(table + 0x1c, r.next_u32());
            c.bus.write32(0x44_5dd0, r.next_u32());
        });
    }

    #[test]
    fn c_ps2_fog_particle_man_base_ctor_matches() {
        check(ssx3_ee::s09::f_002dbf80, c_ps2_fog_particle_man_base_ctor, 100, &|c, seed| {
            let mut r = Rng(0x1005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn c_transition_man_set_paused_matches() {
        check(ssx3_ee::s09::f_002e4638, c_transition_man_set_paused, 300, &|c, seed| {
            let mut r = Rng(0x1006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { ((r.next_u32() as u64) << 32) | 1 };
            c.bus.write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(DATA + 0x40, if seed % 5 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn c_transition_man_get_time_remaining_matches() {
        check(ssx3_ee::s09::f_002e4678, c_transition_man_get_time_remaining, 200, &|c, seed| {
            let mut r = Rng(0x1007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
            for off in [0x48u32, 0x54, 0x58, 0x5c] {
                c.bus.write32(DATA + off, r.edge_f32());
            }
        });
    }

    #[test]
    fn transition_effect_ctor_matches() {
        check(ssx3_ee::s09::f_002e4d70, transition_effect_ctor, 100, &|c, seed| {
            let mut r = Rng(0x1008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn snow_box_reset_matches() {
        check(ssx3_ee::s09::f_002e4f10, snow_box_reset, 100, &|c, seed| {
            let mut r = Rng(0x1009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x90, r.next_u32());
            c.bus.write32(DATA + 0xb0, r.next_u32());
        });
    }

    #[test]
    fn c_speed_boost_streamer_fx_reset_matches() {
        check(ssx3_ee::s09::f_002e6640, c_speed_boost_streamer_fx_reset, 150, &|c, seed| {
            let mut r = Rng(0x100a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(
                0x870,
                if seed % 2 == 0 { r.next_u32() } else { r.next_u32() | 0x8000_0000 },
            );
        });
    }

    #[test]
    fn c_cut_scene_transition_man_start_out_matches() {
        check(ssx3_ee::s09::f_002ea780, c_cut_scene_transition_man_start_out, 300, &|c, seed| {
            let mut r = Rng(0x100b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let state = match seed % 8 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 4,
                5 => 0x8000_0000, // sign-extends to a negative i64
                6 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0x10, state);
            c.bus.write32(DATA + 0x14, r.edge_f32());
            c.bus.write32(DATA + 0x18, r.edge_f32());
            c.f[12] = r.edge_f32();
        });
    }

    #[test]
    fn c_cut_scene_transition_man_set_paused_matches() {
        check(ssx3_ee::s09::f_002ea8b8, c_cut_scene_transition_man_set_paused, 300, &|c, seed| {
            let mut r = Rng(0x100c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { ((r.next_u32() as u64) << 32) | 1 };
            c.bus.write32(
                DATA + 0x10,
                match seed % 3 {
                    0 => 2,
                    1 => 3,
                    _ => r.next_u32(),
                },
            );
            c.bus.write32(
                DATA + 0x1c,
                match seed % 4 {
                    0 => 0,
                    1 => r.next_u32() & 0x7fff_ffff,
                    2 => 0x8000_0000,
                    _ => r.next_u32(),
                },
            );
        });
    }

    #[test]
    fn rider_base_reset_power_up_trail_matches() {
        check(ssx3_ee::s09::f_002eadc0, rider_base_reset_power_up_trail, 100, &|c, seed| {
            let mut r = Rng(0x100d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0xc, r.next_u32());
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn c_air_streamer_fx_reset_matches() {
        check(ssx3_ee::s09::f_002ef6a0, c_air_streamer_fx_reset, 200, &|c, seed| {
            let mut r = Rng(0x100e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write32(
                DATA + 0x100 + 0x870,
                if seed % 2 == 0 { r.next_u32() } else { r.next_u32() | 0x8000_0000 },
            );
        });
    }

    #[test]
    fn ai_visual_effects_man_ctor_matches() {
        check(ssx3_ee::s09::f_002f0368, ai_visual_effects_man_ctor, 100, &|c, seed| {
            let mut r = Rng(0x100f ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn attached_emitter_fx_set_model_matches() {
        check(ssx3_ee::s09::f_002f19f0, attached_emitter_fx_set_model, 100, &|c, seed| {
            let mut r = Rng(0x1010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn c_splash_manager_reset_matches() {
        check(ssx3_ee::s09::f_002f39c8, c_splash_manager_reset, 100, &|c, seed| {
            let mut r = Rng(0x1011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x1028, r.next_u32());
        });
    }

    #[test]
    fn c_splash_manager_set_snow_amount_matches() {
        check(ssx3_ee::s09::f_002f4330, c_splash_manager_set_snow_amount, 300, &|c, seed| {
            let mut r = Rng(0x1012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.f[12] = match seed % 5 {
                0 => 0,
                1 => 0x7f7f_ffff,
                2 => 0x8000_0000,
                3 => r.edge_f32(),
                _ => r.f32(1000.0),
            };
            c.bus.write32(GAME_GP + 0x1190, r.edge_f32());
            c.bus.write32(GAME_GP + 0x1194, r.edge_f32());
            c.bus.write32(GAME_GP - 0x38f4, r.edge_f32());
            c.bus.write32(DATA + 0x1020, r.edge_f32());
            c.bus.write32(DATA + 0x1024, r.edge_f32());
        });
    }

    #[test]
    fn rider_base_clear_cheat_code_events_matches() {
        check(ssx3_ee::s09::f_002f6b28, rider_base_clear_cheat_code_events, 200, &|c, seed| {
            let mut r = Rng(0x1013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(DATA + 7, (r.next_u32() & 0xff) as u8);
        });
    }

    #[test]
    fn rider_base_get_cheat_code_events_matches() {
        check(ssx3_ee::s09::f_002f6b38, rider_base_get_cheat_code_events, 200, &|c, seed| {
            let mut r = Rng(0x1014 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(DATA + 7, (r.next_u32() & 0xff) as u8);
        });
    }

    // ----- s10 ------------------------------------------------------------

    #[test]
    fn wscript_process_get_current_task_id_matches() {
        check_lohi(ssx3_ee::s10::f_003079d8, wscript_process_get_current_task_id, 250, &|c, seed| {
            let mut r = Rng(0x2001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let (v, a) = if seed % 2 == 0 { (0x100u32, 0x40u32) } else { (0x40u32, 0x100u32) };
            c.bus.write32(DATA + 4, v);
            c.bus.write32(DATA + 8, a);
            c.bus.write32(DATA + 0x14, 0);
            c.bus.write32(0x1104, r.next_u32());
        });
    }

    #[test]
    fn wscript_man_post_trigger_event_matches() {
        check_lohi(ssx3_ee::s10::f_003097e0, wscript_man_post_trigger_event, 200, &|c, seed| {
            let mut r = Rng(0x2002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }

    #[test]
    fn wscript_man_post_jump_event_matches() {
        check_lohi(ssx3_ee::s10::f_00309848, wscript_man_post_jump_event, 200, &|c, seed| {
            let mut r = Rng(0x2003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }

    #[test]
    fn wscript_man_post_trick_event_matches() {
        check_lohi(ssx3_ee::s10::f_00309918, wscript_man_post_trick_event, 200, &|c, seed| {
            let mut r = Rng(0x2004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
            for i in 0..2 {
                c.bus.write32(DATA + 0x200 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn wscript_man_post_wipeout_event_matches() {
        check_lohi(ssx3_ee::s10::f_00309990, wscript_man_post_wipeout_event, 200, &|c, seed| {
            let mut r = Rng(0x2005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }

    #[test]
    fn wscript_man_post_reset_event_matches() {
        check_lohi(ssx3_ee::s10::f_003099f8, wscript_man_post_reset_event, 200, &|c, seed| {
            let mut r = Rng(0x2006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }

    #[test]
    fn wscript_process_list_ctor_matches() {
        check(ssx3_ee::s10::f_0030b058, wscript_process_list_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn wscript_process_list_push_front_2_matches() {
        check(ssx3_ee::s10::f_0030b2c0, wscript_process_list_push_front_2, 150, &|c, seed| {
            let mut r = Rng(0x2008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 0x100 + 0x18, r.next_u32());
        });
    }

    #[test]
    fn wscript_man_event_queue_push_matches() {
        check_lohi(ssx3_ee::s10::f_0030c7c0, wscript_man_event_queue_push, 250, &|c, seed| {
            let mut r = Rng(0x2009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 4, (seed % 20) as u32);
        });
    }

    #[test]
    fn rider_model_set_color_scale_matches() {
        check(ssx3_ee::s10::f_0030dba0, rider_model_set_color_scale, 200, &|c, seed| {
            let mut r = Rng(0x200a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.bus.write32(DATA + 0x140, r.next_u32());
        });
    }

    #[test]
    fn rider_model_is_part_visible_matches() {
        check_lohi(ssx3_ee::s10::f_0030ebc0, rider_model_is_part_visible, 300, &|c, seed| {
            let mut r = Rng(0x200b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let idx = seed % 4;
            c.g[A1][0] = idx as u64;
            c.g[A2][0] = (seed % 8) as u64;
            c.bus.write32(DATA + 0x14, if seed % 3 == 0 { idx } else { 0x10 });
            let arr = if seed % 5 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(DATA + 0x1c, arr);
            c.bus.write32(DATA + 0xc, DATA + 0x300);
            let entry = match seed % 6 {
                0 => 0u32,
                1 => 0x8000_0000,
                _ => (seed % 64) as u32,
            };
            c.bus.write32(arr.wrapping_add(idx * 4), entry);
            for i in 0..8 {
                c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn adf_data_source_set_data_matches() {
        check(ssx3_ee::s10::f_00311290, adf_data_source_set_data, 200, &|c, seed| {
            let mut r = Rng(0x200c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..4 {
                c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn anim_seq_list_init_matches() {
        check(ssx3_ee::s10::f_003142e8, anim_seq_list_init, 100, &|c, seed| {
            let mut r = Rng(0x200d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn mdf_file_ctor_matches() {
        check(ssx3_ee::s10::f_00314978, mdf_file_ctor, 100, &|c, seed| {
            let mut r = Rng(0x200e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn mdf_file_parse_header_matches() {
        check(ssx3_ee::s10::f_00314aa0, mdf_file_parse_header, 200, &|c, seed| {
            let mut r = Rng(0x200f ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write16(DATA + 0x106, (r.next_u32() & 0xffff) as u16);
            c.bus.write32(DATA + 0x108, r.next_u32());
        });
    }

    #[test]
    fn game_anim_map_ctor_matches() {
        check(ssx3_ee::s10::f_00314d98, game_anim_map_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP + 0xd8c, r.next_u32());
        });
    }

    #[test]
    fn big_file_ctor_matches() {
        check(ssx3_ee::s10::f_00316898, big_file_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn block_man_ctor_matches() {
        check(ssx3_ee::s10::f_00316a00, block_man_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn block_man_align_size_matches() {
        check_lohi(ssx3_ee::s10::f_00316a20, block_man_align_size, 300, &|c, seed| {
            let mut r = Rng(0x2013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(
                DATA + 0x10,
                match seed % 6 {
                    0 => 0,
                    1 => 8,
                    2 => 16,
                    3 => 0x1000,
                    4 => 0x8000_0000,
                    _ => r.next_u32() & 0xffff,
                },
            );
        });
    }

    #[test]
    fn app_ctor_matches() {
        check(ssx3_ee::s10::f_00316cd0, app_ctor, 150, &|c, seed| {
            let mut r = Rng(0x2014 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP - 0x2efc, r.edge_f32());
            c.bus.write32(GAME_GP + 0x2a74, r.next_u32());
        });
    }

    #[test]
    fn app_get_exec_man_matches() {
        check(ssx3_ee::s10::f_00317520, app_get_exec_man, 100, &|c, seed| {
            let mut r = Rng(0x2015 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let app = DATA + 0x100;
            c.bus.write32(GAME_GP + 0x2a74, app);
            c.bus.write32(app + 8, r.next_u32());
        });
    }

    #[test]
    fn app_load_progress_set_target_matches() {
        check(ssx3_ee::s10::f_00317550, app_load_progress_set_target, 300, &|c, seed| {
            let mut r = Rng(0x2016 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.f[12] = match seed % 4 {
                0 => 0,
                1 => 0x3f80_0000,
                2 => 0x8000_0000,
                _ => r.edge_f32(),
            };
            c.bus.write32(GAME_GP - 0x2ef4, r.edge_f32());
            let app = DATA + 0x200;
            c.bus.write32(GAME_GP + 0x2a74, app);
            c.bus.write32(app + 0x1c, r.next_u32());
            for off in [0u32, 4, 8, 0xc] {
                c.bus.write32(DATA + off, r.edge_f32());
            }
        });
    }

    /// The seven pool bases/boundaries `MEM_*` read through `gp`.
    fn write_pool_globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0x2a70, 0x0040_0000);
        c.bus.write32(GAME_GP + 0x2a60, 0x0050_0000);
        c.bus.write32(GAME_GP + 0x2a6c, 0x0060_0000);
        c.bus.write32(GAME_GP + 0x2a7c, 0x0080_0000);
        c.bus.write32(GAME_GP + 0x2a84, 0x00c0_0000);
        c.bus.write32(GAME_GP + 0x2a78, 0x0100_0000);
        c.bus.write32(GAME_GP + 0x2a68, 0x0180_0000);
    }

    fn pool_candidates() -> [u32; 12] {
        [
            0x0040_0004,
            0x0050_0004,
            0x0060_0004,
            0x0080_0004,
            0x0090_0000,
            0x00c0_0004,
            0x00d0_0000,
            0x0100_0004,
            0x0101_0000,
            0x0180_0004,
            0x0181_0000,
            0x0000_0200,
        ]
    }

    #[test]
    fn mem_is_pool_pointer_matches() {
        check(ssx3_ee::s10::f_0031a088, mem_is_pool_pointer, 300, &|c, seed| {
            let mut r = Rng(0x2017 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            write_pool_globals(c);
            let cands = pool_candidates();
            c.g[A0][0] = cands[(seed as usize) % cands.len()] as u64;
        });
    }

    #[test]
    fn mem_get_pool_block_size_matches() {
        check(ssx3_ee::s10::f_0031a130, mem_get_pool_block_size, 300, &|c, seed| {
            let mut r = Rng(0x2018 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            write_pool_globals(c);
            let cands = pool_candidates();
            c.g[A0][0] = cands[(seed as usize) % cands.len()] as u64;
        });
    }

    #[test]
    fn md5_init_matches() {
        check(ssx3_ee::s10::f_0031d368, md5_init, 100, &|c, seed| {
            let mut r = Rng(0x2019 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn c_csi_cubic_spline_ctor_matches() {
        check(ssx3_ee::s10::f_0031d5e8, c_csi_cubic_spline_ctor, 200, &|c, seed| {
            let mut r = Rng(0x201a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 8, r.next_u32());
        });
    }

    #[test]
    fn dlheap_large_bin_index_matches() {
        check(ssx3_ee::s10::f_0031e1d8, dlheap_large_bin_index, 300, &|c, seed| {
            let mut r = Rng(0x201b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = match seed % 6 {
                0 => 0,
                1 => 0xffff_ffff,
                2 => 0x0100_0000,
                3 => 0x00ff_ffff,
                _ => r.next_u32(),
            } as u64;
        });
    }

    #[test]
    fn dlheap_usable_size_matches() {
        check(ssx3_ee::s10::f_0031fbb8, dlheap_usable_size, 300, &|c, seed| {
            let mut r = Rng(0x201c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = if seed % 8 == 0 { 0 } else { (DATA + 0x100) as u64 };
            c.bus.write32(DATA + 0xfc, r.next_u32());
            for i in 0..16 {
                c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn mem_pool8_free_matches() {
        check(ssx3_ee::s10::f_00320488, mem_pool8_free, 100, &|c, seed| {
            let mut r = Rng(0x201d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x800, r.next_u32());
        });
    }

    #[test]
    fn mem_pool16_free_matches() {
        check(ssx3_ee::s10::f_00320498, mem_pool16_free, 100, &|c, seed| {
            let mut r = Rng(0x201e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x2000, r.next_u32());
        });
    }
}
