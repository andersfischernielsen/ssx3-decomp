//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **Deleting destructors** (`cSongConfiguration_dtor`,
//!   `VisualEffectsMan_dtor`, the vfx toggle destructors, ...): republish the
//!   class's vtable at its offset in the object and, when bit 0 of the
//!   destructor flags (`a1`) is set, `operator delete(this)`.  One shared
//!   body.
//! - **TransitionEffect destructors**: the same shape, but bit 0 calls
//!   `TransitionEffect_FreeData(this)` instead (the effect owns a data block
//!   that must be unregistered from the effect table first).
//! - **List walks** (`WorldTriggerManager_FindFreeInstance`,
//!   `WScriptProcessList_FindById` / `_Contains`,
//!   `ModifierBlock_HasRailModifier`): follow a `next` field until a match or
//!   the end, with the generated back-edge event checks.
//! - **Menu / script one-liners** (`MenuSysPage_GetSelectedItem`,
//!   `MenuSysGroupItem_v08` / `_v10`, `cBXString_AppendStr`,
//!   `WScriptMan_ObjTriggerHook`, ...).
//!
//! Calls go through [`crate::call`] with the same registers, cycles and
//! return-address checks as the generated code.

use crate::abi;
use crate::call::{call_ptr, guest, Frame};
use crate::events;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002a_9288, ps2_sound_manager_v07, "PS2SoundManager_v07"),
    (0x002a_e188, path_plat_remove_track_slot, "PATH_PlatRemoveTrackSlot"),
    (0x002b_1b68, c_song_configuration_dtor, "cSongConfiguration_dtor"),
    (
        0x002b_5b90,
        world_trigger_manager_find_free_instance,
        "WorldTriggerManager_FindFreeInstance",
    ),
    (
        0x002b_67d8,
        world_trigger_manager_is_trigger_flag4,
        "WorldTriggerManager_IsTriggerFlag4",
    ),
    (
        0x002b_6988,
        world_trigger_manager_is_trigger_flag8,
        "WorldTriggerManager_IsTriggerFlag8",
    ),
    (0x002c_1460, world_painter_query_base_dtor, "WorldPainterQuery_BaseDtor"),
    (0x002c_a280, menu_sys_item_dtor, "MenuSysItem_dtor"),
    (0x002c_aa80, menu_sys_page_dtor, "MenuSysPage_dtor"),
    (0x002c_bf30, menu_sys_page_get_selected_item, "MenuSysPage_GetSelectedItem"),
    (0x002c_f8d8, menu_sys_group_item_v08, "MenuSysGroupItem_v08"),
    (0x002c_fb28, menu_sys_group_item_v10, "MenuSysGroupItem_v10"),
    (0x002d_19b8, wscript_man_obj_trigger_hook, "WScriptMan_ObjTriggerHook"),
    (0x002e_4c60, c_transition_man_script_deactivate, "cTransitionMan_ScriptDeactivate"),
    (0x002f_7bf0, vfx_component_dtor, "VfxComponent_dtor"),
    (0x002f_7f00, vfx_power_up_fx_toggles_dtor, "VfxPowerUpFXToggles_dtor"),
    (0x002f_7f30, c_ps2_fog_particle_man_base_dtor, "cPS2FogParticleMan_BaseDtor"),
    (0x002f_7f60, c_ps2_fog_particle_man_dtor, "cPS2FogParticleMan_dtor"),
    (0x002f_7f90, vfx_splash_toggles_dtor, "VfxSplashToggles_dtor"),
    (0x002f_8000, transition_effect_dtor, "TransitionEffect_dtor"),
    (0x002f_8030, transition_effect_fade_dtor, "TransitionEffect_FadeDtor"),
    (0x002f_80d8, transition_wipe_dtor, "TransitionWipe_dtor"),
    (0x002f_8108, transition_effect_random_dtor, "TransitionEffect_RandomDtor"),
    (0x002f_8138, vfx_snow_fall_toggles_dtor, "VfxSnowfallToggles_dtor"),
    (0x002f_8188, vfx_board_trail_dtor, "VfxBoardTrail_dtor"),
    (0x002f_81f0, vfx_light_glow_dtor, "VfxLightGlow_dtor"),
    (0x002f_8220, vfx_depth_fog_dtor, "VfxDepthFog_dtor"),
    (0x002f_8250, vfx_height_fog_dtor, "VfxHeightFog_dtor"),
    (0x002f_8280, vfx_shadow_dtor, "VfxShadow_dtor"),
    (0x002f_82b0, vfx_snow_surface_dtor, "VfxSnowSurface_dtor"),
    (0x002f_82e0, vfx_flags_dtor, "VfxFlags_dtor"),
    (0x002f_8310, visual_effects_man_dtor, "VisualEffectsMan_dtor"),
    (0x0030_b320, wscript_process_list_find_by_id, "WScriptProcessList_FindById"),
    (0x0030_b428, wscript_process_list_contains, "WScriptProcessList_Contains"),
    (0x0030_c760, wscript_man_reset_command_queue, "WScriptMan_ResetCommandQueue"),
    (0x0030_c790, wscript_man_event_queue_reset, "WScriptMan_EventQueueReset"),
    (0x0030_d420, wscript_man_sub_obj_dtor_d420, "WScriptMan_SubObjDtor_d420"),
    (0x0031_89a0, cbxstring_append_str, "cBXString_AppendStr"),
    (0x0032_0ad8, ps2_exec_man_v01, "PS2ExecMan_v01"),
    (0x0032_77d0, pad_device_dtor, "PadDevice_dtor"),
    (0x0034_6228, halo_modifier_dtor, "HaloModifier_dtor"),
    (0x0034_75a8, flex_bridge_update, "FlexBridge_Update"),
    (0x0034_afb8, c_flag_manager_group_dtor, "cFlagManager_GroupDtor"),
    (0x0035_3418, modifier_block_has_rail_modifier, "ModifierBlock_HasRailModifier"),
    (0x0035_f378, tex_flip_modifier_set_frame, "TexFlipModifier_SetFrame"),
    (0x0036_0b00, roller_modifier_dtor, "RollerModifier_dtor"),
    (0x0036_0c28, boost_modifier_base_dtor, "BoostModifier_BaseDtor"),
    (0x0036_11d8, intersect_info_dtor, "IntersectInfo_dtor"),
    (0x0036_1340, intersect_info_dtor_3, "IntersectInfo_dtor_3"),
    (0x0036_1488, boost_modifier_v01, "BoostModifier_v01"),
];

/// `operator delete` (`__builtin_delete`, 0x317e50).
const DELETE: u32 = 0x0031_7e50;
/// `TransitionEffect_FreeData` (0x2e4d30).
const FREE_DATA: u32 = 0x002e_4d30;

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the tiny deleting destructors: republish the class's
/// vtable pointer at `off` in `this`, then `operator delete(this)` when bit 0
/// of the destructor flags (`a1`) is set. `ret` is the address the delete
/// returns to.
fn deleting_dtor(c: &mut Cpu, off: u32, vtable: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = sx(vtable);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let deleting = c.g[A1][0] != 0;
        let this = c.g[A0][0] as u32;
        let v = c.g[V0][0];
        Addr(this).off(off).sw(c, v);
        if deleting {
            c.cycles += 2;
            call_ptr(c, DELETE, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The same, for the `TransitionEffect` classes: bit 0 calls
/// `TransitionEffect_FreeData(this)` (which unregisters the effect's data
/// block) instead of freeing the object.
fn free_effect_data_dtor(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = sx(0x0048_80e0);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let free_data = c.g[A1][0] != 0;
        let this = c.g[A0][0] as u32;
        let v = c.g[V0][0];
        Addr(this).sw(c, v);
        if free_data {
            c.cycles += 2;
            call_ptr(c, FREE_DATA, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSongConfiguration_dtor` (0x2b1b68): vtable at `+0xa8`.
pub fn c_song_configuration_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0xa8, 0x0048_3980, 0x002b_1b8c);
}

/// `WorldPainterQuery_BaseDtor` (0x2c1460): vtable at `+0x4`.
pub fn world_painter_query_base_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0048_5d30, 0x002c_1484);
}

/// `MenuSysItem_dtor` (0x2ca280): vtable at `+0x10`.
pub fn menu_sys_item_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x10, 0x0048_6f28, 0x002c_a2a4);
}

/// `MenuSysPage_dtor` (0x2caa80): vtable at `+0x12c`.
pub fn menu_sys_page_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x12c, 0x0048_6ed0, 0x002c_aaa4);
}

/// `VfxComponent_dtor` (0x2f7bf0): vtable at `+0`.
pub fn vfx_component_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_7c14);
}

/// `VfxPowerUpFXToggles_dtor` (0x2f7f00): vtable at `+0`.
pub fn vfx_power_up_fx_toggles_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_7f24);
}

/// `cPS2FogParticleMan_BaseDtor` (0x2f7f30): vtable at `+0x4`.
pub fn c_ps2_fog_particle_man_base_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0048_8308, 0x002f_7f54);
}

/// `cPS2FogParticleMan_dtor` (0x2f7f60): vtable at `+0x4`.
pub fn c_ps2_fog_particle_man_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0048_8308, 0x002f_7f84);
}

/// `VfxSplashToggles_dtor` (0x2f7f90): vtable at `+0`.
pub fn vfx_splash_toggles_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_7fb4);
}

/// `TransitionEffect_dtor` (0x2f8000): vtable at `+0`, frees the effect data.
pub fn transition_effect_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_8024);
}

/// `TransitionEffect_FadeDtor` (0x2f8030): vtable at `+0`, frees the effect
/// data.
pub fn transition_effect_fade_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_8054);
}

/// `TransitionWipe_dtor` (0x2f80d8): vtable at `+0`, frees the effect data.
pub fn transition_wipe_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_80fc);
}

/// `TransitionEffect_RandomDtor` (0x2f8108): vtable at `+0`, frees the effect
/// data.
pub fn transition_effect_random_dtor(c: &mut Cpu) {
    free_effect_data_dtor(c, 0x002f_812c);
}

/// `VfxSnowfallToggles_dtor` (0x2f8138): vtable at `+0`.
pub fn vfx_snow_fall_toggles_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_815c);
}

/// `VfxBoardTrail_dtor` (0x2f8188): vtable at `+0`.
pub fn vfx_board_trail_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_81ac);
}

/// `VfxLightGlow_dtor` (0x2f81f0): vtable at `+0`.
pub fn vfx_light_glow_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8214);
}

/// `VfxDepthFog_dtor` (0x2f8220): vtable at `+0`.
pub fn vfx_depth_fog_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8244);
}

/// `VfxHeightFog_dtor` (0x2f8250): vtable at `+0`.
pub fn vfx_height_fog_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8274);
}

/// `VfxShadow_dtor` (0x2f8280): vtable at `+0`.
pub fn vfx_shadow_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_82a4);
}

/// `VfxSnowSurface_dtor` (0x2f82b0): vtable at `+0`.
pub fn vfx_snow_surface_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_82d4);
}

/// `VfxFlags_dtor` (0x2f82e0): vtable at `+0`.
pub fn vfx_flags_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_8680, 0x002f_8304);
}

/// `VisualEffectsMan_dtor` (0x2f8310): vtable at `+0x44`.
pub fn visual_effects_man_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x44, 0x0048_7da0, 0x002f_8334);
}

/// `WScriptMan_SubObjDtor_d420` (0x30d420): vtable at `+0x40`.
pub fn wscript_man_sub_obj_dtor_d420(c: &mut Cpu) {
    deleting_dtor(c, 0x40, 0x0048_9ae0, 0x0030_d444);
}

/// `PS2ExecMan_v01` (0x320ad8): vtable at `+0x8`.
pub fn ps2_exec_man_v01(c: &mut Cpu) {
    deleting_dtor(c, 0x8, 0x0048_dc30, 0x0032_0afc);
}

/// `PadDevice_dtor` (0x3277d0): vtable at `+0`.
pub fn pad_device_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0048_e4b8, 0x0032_77f4);
}

/// `cFlagManager_GroupDtor` (0x34afb8): vtable at `+0x184`.
pub fn c_flag_manager_group_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x184, 0x0048_fbf8, 0x0034_afdc);
}

/// `RollerModifier_dtor` (0x360b00): vtable at `+0`.
pub fn roller_modifier_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0, 0x0049_13f8, 0x0036_0b24);
}

/// `BoostModifier_BaseDtor` (0x360c28): vtable at `+0x4`.
pub fn boost_modifier_base_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0049_13b8, 0x0036_0c4c);
}

/// `IntersectInfo_dtor` (0x3611d8): vtable at `+0x84`.
pub fn intersect_info_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0x84, 0x0049_0af0, 0x0036_11fc);
}

/// `IntersectInfo_dtor_3` (0x361340): vtable at `+0x84`.
pub fn intersect_info_dtor_3(c: &mut Cpu) {
    deleting_dtor(c, 0x84, 0x0049_0af0, 0x0036_1364);
}

/// `BoostModifier_v01` (0x361488): vtable at `+0x4`.
pub fn boost_modifier_v01(c: &mut Cpu) {
    deleting_dtor(c, 0x4, 0x0049_13b8, 0x0036_14ac);
}

// ---------------------------------------------------------------------------
// Sound.

/// `PS2SoundManager_v07` (0x2a9288): set the sound-system SPU memory base to
/// the manager's cached value (at `+0x30c`) with `-1` ("use the default"),
/// then clear the pending flag at `+0x31c`.
pub fn ps2_sound_manager_v07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = u64::MAX;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x30c).lw(c);
        call_ptr(c, 0x003b_5b20, 0x002a_92a4)?;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x31c).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Track slots.

/// `PATH_PlatRemoveTrackSlot(int slot)` (0x2ae188): clear every one of the 24
/// entries in the track-slot table at 0x4d3e98 that equals `slot`.
pub fn path_plat_remove_track_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x004d_0000;
    c.g[A1][0] = 0x17;
    c.g[V1][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
    let slot = c.g[A0][0];
    loop {
        c.cycles += 3;
        let entry = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(entry).lw(c);
        let same = c.g[V0][0] == slot;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        if same {
            c.cycles += 1;
            Addr(entry).sw(c, 0);
        }
        c.cycles += 2;
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(entry.wrapping_add(4));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if events::event_due(c) {
            events::deliver_event_here(c, 0x002a_e198);
        }
    }
}

// ---------------------------------------------------------------------------
// World triggers.

/// `WorldTriggerManager_FindFreeInstance(WorldTriggerManager* m)` (0x2b5b90):
/// the first instance (stride 0x70) whose `active` word is 0, or null after
/// 40 active instances.
pub fn world_trigger_manager_find_free_instance(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if active {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 40) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x70));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            if events::event_due(c) {
                events::deliver_event_here(c, 0x002b_5b98);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
    }
}

/// `WorldTriggerManager_IsTriggerFlag4(m, i)` (0x2b67d8): for an instance
/// index >= 2, whether the trigger object's word at `+0x4` is 1.
pub fn world_trigger_manager_is_trigger_flag4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0x2) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    if small {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x33c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
    c.g[V0][0] ^= 1;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `WorldTriggerManager_IsTriggerFlag8(m, i)` (0x2b6988): the same for the
/// trigger object's word at `+0x8`.
pub fn world_trigger_manager_is_trigger_flag8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0x2) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    if small {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x33c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
    c.g[V0][0] ^= 1;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Menus.

/// `MenuSysPage_GetSelectedItem(MenuSysPage* page)` (0x2cbf30): select the
/// page's current item first, then return `page->items[selected]` (the word
/// at `page + index*4 + 0xc`).
pub fn menu_sys_page_get_selected_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002c_bf08, 0x002c_bf44)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_v08(MenuSysGroupItem* item)` (0x2cf8d8): notify the item
/// that its selected page changed (virtual slot `+0x40`/`+0x44` of the
/// selected item's object).
pub fn menu_sys_group_item_v08(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x002c_bf30, 0x002c_f8e8)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x40).lh(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x44).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_f8fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem_v10(MenuSysGroupItem* item)` (0x2cfb28): the same for the
/// object at `(*item)->page` (slot `+0x20`/`+0x24` of the vtable at `+0x12c`).
pub fn menu_sys_group_item_v10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x12c).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_fb4c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script system.

/// `WScriptMan_ObjTriggerHook(this, a, b, c)` (0x2d19b8): forward an
/// object-trigger event to the script manager singleton at `gp+0xce8`.
pub fn wscript_man_obj_trigger_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
        c.g[A1][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V1][0];
        call_ptr(c, 0x0030_a060, 0x002d_19dc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cTransitionMan_ScriptDeactivate(cTransitionMan* m)` (0x2e4c60):
/// deactivate the script object at `m+0x10`, then stop the transition.
pub fn c_transition_man_script_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0028_3440, 0x002e_4c78)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002e_4578, 0x002e_4c80)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcessList_FindById(list, id)` (0x30b320): the node whose id
/// (`+0x20`) matches, following `+0x18`.
pub fn wscript_process_list_find_by_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x18).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0030_b330);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `WScriptProcessList_Contains(list, node)` (0x30b428): whether `node` is in
/// the process list.
pub fn wscript_process_list_contains(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 2;
        if c.g[A0][0] != c.g[A1][0] {
            c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x18).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0030_b438);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// The command/event queue reset shared by both WScriptMan queues: clear the
/// two head words and zero the 0xc0-byte array at `+0x8`.
fn reset_queue(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        let this = c.g[V0][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(8));
        Addr(this).sw(c, 0);
        Addr(this).off(4).sw(c, 0);
        c.g[A2][0] = 0xc0;
        call_ptr(c, 0x003e_6448, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMan_ResetCommandQueue(WScriptMan* m)` (0x30c760).
pub fn wscript_man_reset_command_queue(c: &mut Cpu) {
    reset_queue(c, 0x0030_c784);
}

/// `WScriptMan_EventQueueReset(WScriptMan* m)` (0x30c790).
pub fn wscript_man_event_queue_reset(c: &mut Cpu) {
    reset_queue(c, 0x0030_c7b4);
}

// ---------------------------------------------------------------------------
// Strings.

/// `cBXString_AppendStr(cBXString* self, char** str)` (0x3189a0): append the
/// C string `*str` (its length comes from the cBXString header at `*str-8`),
/// returning `self`.
pub fn cbxstring_append_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_sub(8)).lw(c);
        call_ptr(c, 0x0031_8830, 0x0031_89bc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Effects / modifiers.

/// `HaloModifier_dtor(this, flags)` (0x346228): republish the modifier vtable
/// at `+0x8`, drop the particle-modifier live count at `gp+0xf10`, then run
/// the base destructor (which unlinks the modifier and may delete it).
pub fn halo_modifier_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xf10).lw(c);
        c.g[V1][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x1220));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let this = c.g[A0][0] as u32;
        let v = c.g[V1][0];
        Addr(this).off(8).sw(c, v);
        let gp = c.g[GP][0] as u32;
        let count = c.g[V0][0];
        Addr(gp).off(0xf10).sw(c, count);
        call_ptr(c, 0x0034_5828, 0x0034_624c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FlexBridge_Update(FlexBridge* bridge)` (0x3475a8): one physics step of the
/// bridge springs, then rebuild the rendered vertices.
pub fn flex_bridge_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
        call_ptr(c, 0x0035_3fc0, 0x0034_75c0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0034_75d8, 0x0034_75c8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ModifierBlock_HasRailModifier(block, id)` (0x353418): walk the modifier
/// list at `+0x1c` for a node whose id (`+0x30`) matches.
pub fn modifier_block_has_rail_modifier(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0035_3428);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `TexFlipModifier_SetFrame(TexFlipModifier* m, int frame)` (0x35f378):
/// clamp `frame` to `< count` (else `count-1`), store it at `+0x14` and apply
/// it to the flip textures.
pub fn tex_flip_modifier_set_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = Addr(c.g[A2][0] as u32).off(0x20).lw(c);
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A1][0];
        }
        let this = c.g[A2][0] as u32;
        let v = c.g[V0][0];
        Addr(this).off(0x14).sw(c, v);
        call_ptr(c, 0x0035_f3a8, 0x0035_f39c)?;
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
    use crate::difftest::fakes::{machine, ptr, vtable, LOG_RESULTS};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    const SEEDS: u32 = 200;

    /// The allocator mutex (`EA_Mutex` at 0x538b00), marked as held by this
    /// thread (id 0x42, which the first fake syscall also returns) so the real
    /// `__builtin_delete` takes its fast path.
    const MEM_MUTEX: u32 = 0x0053_8b00;

    fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    /// The behaviour-library singleton (a zeroed dummy, so
    /// `BELibrary_GetInstance` neither constructs one nor reads the disc) and
    /// a world whose race manager holds one rider.
    fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, DATA + 0x6_8400);
        c.bus.write32(arr + 0x7c, 1);
    }

    /// A deleting-destructor case: a random object pointer, both values of the
    /// delete flag, and a real `operator delete` (with the allocator mutex
    /// taken) that may unwind at the fake's call index.
    fn dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            alloc_setup(c, &mut r);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    // ------------------------------------------------------- sound / track slots

    #[test]
    fn ps2_sound_manager_v07_matches() {
        check(ee::s08::f_002a9288__generated, ps2_sound_manager_v07, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2a9288, 1);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x30c, r.next_u32());
        });
    }

    #[test]
    fn path_plat_remove_track_slot_matches() {
        check(ee::s08::f_002ae188__generated, path_plat_remove_track_slot, SEEDS, &|c, seed| {
            let mut r = Rng(0x002a_e188 ^ (seed as u64) << 3);
            scramble(c, &mut r);
            // The 24-entry table; some entries equal the slot to be cleared
            // (duplicates included so several matches happen per scan).
            let target = 0x7700_0000u32 | (seed % 24);
            for i in 0..24u32 {
                let v = if (i + seed) % 4 == 0 { target } else { r.next_u32() };
                c.bus.write32(0x004d_3e98 + 4 * i, v);
            }
            c.g[A0][0] = if seed % 3 == 0 { r.next_u32() as u64 } else { target as u64 };
        });
    }

    // ------------------------------------------------------- world triggers

    #[test]
    fn c_song_configuration_dtor_matches() {
        dtor_case(ee::s08::f_002b1b68__generated, c_song_configuration_dtor, 0x2b1b68);
    }

    #[test]
    fn world_trigger_manager_find_free_instance_matches() {
        check(
            ee::s08::f_002b5b90__generated,
            world_trigger_manager_find_free_instance,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b5b90, 0);
                let base = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, base);
                // Cover a free slot early, a free slot late, and a full table.
                let full = seed % 5 == 0;
                for i in 0..40u32 {
                    let active = if full { 1 } else { ((i + seed) % 7 != 0) as u32 };
                    c.bus.write32(base + 0x70 * i, active);
                    c.bus.write32(base + 0x70 * i + 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn world_trigger_manager_is_trigger_flag4_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x2b67d8, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 6) as u64;
            let refs = DATA + 0x400;
            c.bus.write32(DATA + 0x33c, refs);
            for i in 0..6u32 {
                let obj = DATA + 0x600 + 0x10 * i;
                c.bus.write32(refs + 8 * i, r.next_u32());
                c.bus.write32(refs + 8 * i + 4, obj);
                c.bus.write32(obj + 4, ((i + seed) % 3 == 0) as u32);
            }
        };
        check(ee::s08::f_002b67d8__generated, world_trigger_manager_is_trigger_flag4, SEEDS, &setup);
    }

    #[test]
    fn world_trigger_manager_is_trigger_flag8_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x2b6988, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 6) as u64;
            let refs = DATA + 0x400;
            c.bus.write32(DATA + 0x33c, refs);
            for i in 0..6u32 {
                let obj = DATA + 0x600 + 0x10 * i;
                c.bus.write32(refs + 8 * i, r.next_u32());
                c.bus.write32(refs + 8 * i + 4, obj);
                c.bus.write32(obj + 8, ((i + seed) % 3 == 0) as u32);
            }
        };
        check(ee::s08::f_002b6988__generated, world_trigger_manager_is_trigger_flag8, SEEDS, &setup);
    }

    // ------------------------------------------------------- menu

    #[test]
    fn world_painter_query_base_dtor_matches() {
        dtor_case(ee::s08::f_002c1460__generated, world_painter_query_base_dtor, 0x2c1460);
    }

    #[test]
    fn menu_sys_item_dtor_matches() {
        dtor_case(ee::s09::f_002ca280__generated, menu_sys_item_dtor, 0x2ca280);
    }

    #[test]
    fn menu_sys_page_dtor_matches() {
        dtor_case(ee::s09::f_002caa80__generated, menu_sys_page_dtor, 0x2caa80);
    }

    /// A menu page at `DATA`: item array at `+0xc`, selected index at `+0x4`,
    /// each item's vtable at `+0x10` (slots 0x18/0x1c for ValidateSelection,
    /// 0x40/0x44 for the group-item notify).
    fn menu_page_setup(c: &mut Cpu, seed: u32, salt: u64, page: u32) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        let n = 1 + (seed % 3);
        let vt = vtable(c, &mut r, &[0x18, 0x40]);
        c.bus.write32(page + 4, seed % n);
        for i in 0..3u32 {
            let item = DATA + 0x300 + 0x40 * i;
            c.bus.write32(page + 4 * i + 0xc, item);
            c.bus.write32(item + 0x10, vt);
        }
        r
    }

    #[test]
    fn menu_sys_page_get_selected_item_matches() {
        check(
            ee::s09::f_002cbf30__generated,
            menu_sys_page_get_selected_item,
            SEEDS,
            &|c, seed| {
                menu_page_setup(c, seed, 0x2cbf30, DATA);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn menu_sys_group_item_v08_matches() {
        check(ee::s09::f_002cf8d8__generated, menu_sys_group_item_v08, SEEDS, &|c, seed| {
            // The group item wraps its page at +0x18.
            menu_page_setup(c, seed, 0x2cf8d8, DATA + 0x18);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn menu_sys_group_item_v10_matches() {
        check(ee::s09::f_002cfb28__generated, menu_sys_group_item_v10, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2cfb28, 0);
            let vt = vtable(c, &mut r, &[0x20]);
            let owner = DATA + 0x100;
            c.bus.write32(DATA, owner);
            c.bus.write32(owner + 0x12c, vt);
            c.g[A0][0] = DATA as u64;
        });
    }

    // ------------------------------------------------------- script

    #[test]
    fn wscript_man_obj_trigger_hook_matches() {
        check(
            ee::s09::f_002d19b8__generated,
            wscript_man_obj_trigger_hook,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d19b8, 8);
                let manager = DATA + 0x100;
                let trigger = DATA + 0x2000;
                let vt = vtable(c, &mut r, &[0x38, 0x40]);
                c.bus.write32(GAME_GP + 0xce8, manager);
                c.bus.write32(manager, r.next_u32());
                c.bus.write32(trigger, vt);
                // The second half of WScriptMan_OnObjectTrigger must take its
                // early return: the trigger object at a3+0x480 is non-null and
                // the world's race manager reports a mode other than 4.
                c.bus.write32(trigger.wrapping_sub(0x6c0) + 0x480, 1);
                globals(c);
                c.bus.write32(DATA + 0x6_8100 + 0x214, 0);
                c.g[A0][0] = (DATA + 0x3000) as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.g[A2][0] = trigger as u64;
                c.bus.write32(DATA + 0x4000 + 0x50, r.next_u32());
            },
        );
    }

    #[test]
    fn c_transition_man_script_deactivate_matches() {
        check(
            ee::s09::f_002e4c60__generated,
            c_transition_man_script_deactivate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e4c60, 1);
                c.g[A0][0] = DATA as u64;
                // No transition in flight: cTransitionMan_Stop returns at once.
                c.bus.write32(DATA + 0x44, 0);
                c.bus.write32(DATA + 0x10 + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_process_list_find_by_id_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x30b320, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 4 == 0 {
                c.bus.write32(DATA, 0);
                return;
            }
            let n0 = DATA + 0x100;
            let n1 = DATA + 0x140;
            let n2 = DATA + 0x180;
            c.bus.write32(DATA, n0);
            c.bus.write32(n0 + 0x18, n1);
            c.bus.write32(n1 + 0x18, n2);
            c.bus.write32(n2 + 0x18, 0);
            c.bus.write32(n0 + 0x20, 10);
            c.bus.write32(n1 + 0x20, 11);
            c.bus.write32(n2 + 0x20, 12);
            c.g[A1][0] = (8 + seed % 7) as u64;
            c.bus.write32(DATA + 0x1f0, r.next_u32());
        };
        check(ee::s10::f_0030b320__generated, wscript_process_list_find_by_id, SEEDS, &setup);
    }

    #[test]
    fn wscript_process_list_contains_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x30b428, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 5 == 0 {
                c.bus.write32(DATA, 0);
                return;
            }
            let n0 = DATA + 0x100;
            let n1 = DATA + 0x140;
            c.bus.write32(DATA, n0);
            c.bus.write32(n0 + 0x18, n1);
            c.bus.write32(n1 + 0x18, 0);
            c.bus.write32(n0 + 0x20, r.next_u32());
            c.bus.write32(n1 + 0x20, r.next_u32());
            c.g[A1][0] = match seed % 4 {
                0 => n0,
                1 => n1,
                2 => DATA + 0x200,
                _ => 0,
            } as u64;
        };
        check(ee::s10::f_0030b428__generated, wscript_process_list_contains, SEEDS, &setup);
    }

    #[test]
    fn wscript_man_reset_command_queue_matches() {
        check(ee::s10::f_0030c760__generated, wscript_man_reset_command_queue, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x30c760, 0);
            c.g[A0][0] = DATA as u64;
            for i in 0..0x40u32 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn wscript_man_event_queue_reset_matches() {
        check(ee::s10::f_0030c790__generated, wscript_man_event_queue_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x30c790, 0);
            c.g[A0][0] = DATA as u64;
            for i in 0..0x40u32 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------- destructors (batch)

    #[test]
    fn wscript_man_sub_obj_dtor_d420_matches() {
        dtor_case(ee::s10::f_0030d420__generated, wscript_man_sub_obj_dtor_d420, 0x30d420);
    }

    #[test]
    fn vfx_component_dtor_matches() {
        dtor_case(ee::s09::f_002f7bf0__generated, vfx_component_dtor, 0x2f7bf0);
    }

    #[test]
    fn vfx_power_up_fx_toggles_dtor_matches() {
        dtor_case(ee::s09::f_002f7f00__generated, vfx_power_up_fx_toggles_dtor, 0x2f7f00);
    }

    #[test]
    fn c_ps2_fog_particle_man_base_dtor_matches() {
        dtor_case(ee::s09::f_002f7f30__generated, c_ps2_fog_particle_man_base_dtor, 0x2f7f30);
    }

    #[test]
    fn c_ps2_fog_particle_man_dtor_matches() {
        dtor_case(ee::s09::f_002f7f60__generated, c_ps2_fog_particle_man_dtor, 0x2f7f60);
    }

    #[test]
    fn vfx_splash_toggles_dtor_matches() {
        dtor_case(ee::s09::f_002f7f90__generated, vfx_splash_toggles_dtor, 0x2f7f90);
    }

    #[test]
    fn transition_effect_dtor_matches() {
        dtor_case(ee::s09::f_002f8000__generated, transition_effect_dtor, 0x2f8000);
    }

    #[test]
    fn transition_effect_fade_dtor_matches() {
        dtor_case(ee::s09::f_002f8030__generated, transition_effect_fade_dtor, 0x2f8030);
    }

    #[test]
    fn transition_wipe_dtor_matches() {
        dtor_case(ee::s09::f_002f80d8__generated, transition_wipe_dtor, 0x2f80d8);
    }

    #[test]
    fn transition_effect_random_dtor_matches() {
        dtor_case(ee::s09::f_002f8108__generated, transition_effect_random_dtor, 0x2f8108);
    }

    #[test]
    fn vfx_snow_fall_toggles_dtor_matches() {
        dtor_case(ee::s09::f_002f8138__generated, vfx_snow_fall_toggles_dtor, 0x2f8138);
    }

    #[test]
    fn vfx_board_trail_dtor_matches() {
        dtor_case(ee::s09::f_002f8188__generated, vfx_board_trail_dtor, 0x2f8188);
    }

    #[test]
    fn vfx_light_glow_dtor_matches() {
        dtor_case(ee::s09::f_002f81f0__generated, vfx_light_glow_dtor, 0x2f81f0);
    }

    #[test]
    fn vfx_depth_fog_dtor_matches() {
        dtor_case(ee::s09::f_002f8220__generated, vfx_depth_fog_dtor, 0x2f8220);
    }

    #[test]
    fn vfx_height_fog_dtor_matches() {
        dtor_case(ee::s09::f_002f8250__generated, vfx_height_fog_dtor, 0x2f8250);
    }

    #[test]
    fn vfx_shadow_dtor_matches() {
        dtor_case(ee::s09::f_002f8280__generated, vfx_shadow_dtor, 0x2f8280);
    }

    #[test]
    fn vfx_snow_surface_dtor_matches() {
        dtor_case(ee::s09::f_002f82b0__generated, vfx_snow_surface_dtor, 0x2f82b0);
    }

    #[test]
    fn vfx_flags_dtor_matches() {
        dtor_case(ee::s09::f_002f82e0__generated, vfx_flags_dtor, 0x2f82e0);
    }

    #[test]
    fn visual_effects_man_dtor_matches() {
        dtor_case(ee::s09::f_002f8310__generated, visual_effects_man_dtor, 0x2f8310);
    }

    #[test]
    fn ps2_exec_man_v01_matches() {
        dtor_case(ee::s10::f_00320ad8__generated, ps2_exec_man_v01, 0x320ad8);
    }

    #[test]
    fn pad_device_dtor_matches() {
        dtor_case(ee::s10::f_003277d0__generated, pad_device_dtor, 0x3277d0);
    }

    #[test]
    fn c_flag_manager_group_dtor_matches() {
        dtor_case(ee::s11::f_0034afb8__generated, c_flag_manager_group_dtor, 0x34afb8);
    }

    #[test]
    fn roller_modifier_dtor_matches() {
        dtor_case(ee::s12::f_00360b00__generated, roller_modifier_dtor, 0x360b00);
    }

    #[test]
    fn boost_modifier_base_dtor_matches() {
        dtor_case(ee::s12::f_00360c28__generated, boost_modifier_base_dtor, 0x360c28);
    }

    #[test]
    fn intersect_info_dtor_matches() {
        dtor_case(ee::s12::f_003611d8__generated, intersect_info_dtor, 0x3611d8);
    }

    #[test]
    fn intersect_info_dtor_3_matches() {
        dtor_case(ee::s12::f_00361340__generated, intersect_info_dtor_3, 0x361340);
    }

    #[test]
    fn boost_modifier_v01_matches() {
        dtor_case(ee::s12::f_00361488__generated, boost_modifier_v01, 0x361488);
    }

    // ------------------------------------------------------- strings

    #[test]
    fn cbxstring_append_str_matches() {
        check(ee::s10::f_003189a0__generated, cbxstring_append_str, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3189a0, 0);
            // Destination cBXString: `self->text` at DATA+0x200, header at
            // -0xc (>= 2 selects the in-place path), -8 (length), -4
            // (capacity, large enough for the append).
            let buffer = DATA + 0x200;
            let length = (seed % 5) as u32;
            c.bus.write32(DATA, buffer);
            c.bus.write32(buffer - 0xc, 2);
            c.bus.write32(buffer - 8, length);
            c.bus.write32(buffer - 4, 0x1000);
            let src = DATA + 0x300;
            c.bus.write32(DATA + 0x40, src);
            c.bus.write32(src - 8, 1 + (seed % 7));
            for i in 0..16u32 {
                c.bus.write8(src + i, (r.next_u32() & 0x7f) as u8);
                c.bus.write8(buffer + length + i, (r.next_u32() & 0x7f) as u8);
            }
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x40) as u64;
        });
    }

    // ------------------------------------------------------- effects / modifiers

    #[test]
    fn halo_modifier_dtor_matches() {
        check(ee::s11::f_00346228__generated, halo_modifier_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x346228, 1);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(GAME_GP + 0xf10, 1 + r.next_u32() % 3);
        });
    }

    #[test]
    fn flex_bridge_update_matches() {
        check(ee::s11::f_003475a8__generated, flex_bridge_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3475a8, 0);
            c.g[A0][0] = DATA as u64;
            // The frame-clock singleton the step length comes from.
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x300);
            c.bus.write32(DATA + 0x310, 0x42f0_0000); // 120.0f
            // A bridge with no nodes: both physics passes return immediately.
            c.bus.write32(DATA + 0x50, 0);
            c.bus.write32(DATA + 0x74, r.next_u32());
        });
    }

    #[test]
    fn modifier_block_has_rail_modifier_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x353418, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 5 == 0 {
                c.bus.write32(DATA + 0x1c, 0);
                return;
            }
            let n0 = DATA + 0x100;
            let n1 = DATA + 0x140;
            c.bus.write32(DATA + 0x1c, n0);
            c.bus.write32(n0, n1);
            c.bus.write32(n1, 0);
            c.bus.write32(n0 + 0x30, 0x21);
            c.bus.write32(n1 + 0x30, 0x22);
            c.g[A1][0] = (0x20 + seed % 4) as u64;
        };
        check(ee::s11::f_00353418__generated, modifier_block_has_rail_modifier, SEEDS, &setup);
    }

    #[test]
    fn tex_flip_modifier_set_frame_matches() {
        check(ee::s12::f_0035f378__generated, tex_flip_modifier_set_frame, SEEDS, &|c, seed| {
            machine(c, seed, 0x35f378, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 7) as u64;
            // The frame count at +0x20 and no textures in the apply loop.
            c.bus.write32(DATA + 0x20, seed % 5);
            c.bus.write32(DATA + 0x1c, 0);
        });
    }
}
